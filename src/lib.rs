pub mod config;
mod ioctl;

use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use config::{TXConfig, RXConfig, RawConfig, RawConfigType};

const VERSION: u32 = 2;

#[derive(Debug)]
pub enum DeviceError {
    NoDevice,
    FileHandleClone,
    InvalidIOCTL,
    VersionMismatch,
    NoRXConfig,
    Busy,
    Copy,
    InvalidConfig,
    OutOfMemory,
    BufferEmpty,
    PacketSize,
    Unknown
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidFrequency,
    InvalidBandwidth,
    InvalidCarrierSense,
    InvalidTXPower,
    InvalidBaudRate,
    InvalidDeviation,
    InvalidSyncWord
}

#[derive(Debug)]
pub enum CC1101Error {
    Device(DeviceError),
    Config(ConfigError)
}

pub struct CC1101 {
    device: String,
    handle: Option<File>,
    rx_config: Option<RXConfig>
}

impl CC1101 {
    pub fn new(device: &str, rx_config: Option<RXConfig>, blocking: bool) -> Result<CC1101, CC1101Error> {

        let handle = Self::open(device)?;

        if let Some(rx_config) = &rx_config {
            Self::set_rx_config_on_device(&handle, &None, rx_config, blocking)?;
        }

        match blocking {
            true => Ok(CC1101{device: device.to_string(), handle: Some(handle), rx_config}),
            false => Ok(CC1101{device: device.to_string(), handle: None, rx_config})
        }
    }

    fn open(device: &str) -> Result<File, CC1101Error> {

        let handle = match OpenOptions::new().read(true).write(true).open(device) {
            Ok(file) => file,
            Err(e) => {
                match e.raw_os_error() {
                    Some(libc::EBUSY) => return Err(CC1101Error::Device(DeviceError::Busy)), 
                    _ => {
                        return Err(CC1101Error::Device(DeviceError::Unknown))
                    }
                }
            }
        };

        let version = ioctl::get_version(&handle)?;

        if version != VERSION {
            return Err(CC1101Error::Device(DeviceError::VersionMismatch))
        }

        Ok(handle)
    }

    fn get_handle(&self) -> Result<File, CC1101Error> {
        if let Some(handle) = &self.handle {
            match handle.try_clone() {
                Ok(h) => Ok(h),
                Err(_) => Err(CC1101Error::Device(DeviceError::FileHandleClone))
            }
        }
        else {
            Ok(Self::open(&self.device)?)
        }
    }

    pub fn reset(&mut self) -> Result<(), CC1101Error> {
        ioctl::reset(&self.get_handle()?)
    }

    pub fn set_tx_config(&mut self, tx_config: &TXConfig) -> Result<(), CC1101Error> {
        Self::set_tx_config_on_device(&self.get_handle()?, tx_config)
    }

    fn set_tx_config_on_device(handle: &File, tx_config: &TXConfig) -> Result<(), CC1101Error> {
        ioctl::set_tx_conf(handle, tx_config)
    }

    pub fn set_rx_config(&mut self, rx_config: &RXConfig) -> Result<(), CC1101Error>{
        Self::set_rx_config_on_device(&self.get_handle()?, &self.rx_config, rx_config, self.handle.is_some())?;
        self.rx_config = Some(rx_config.clone());
        Ok(())
    }

    fn set_rx_config_on_device(handle: &File, old_config: &Option<RXConfig>, new_config: &RXConfig, blocking: bool) -> Result<(), CC1101Error> {

        // Does the new config match the saved config
        let configs_match = match old_config {
            Some(old_config) => old_config == new_config,
            None => false
        };

        if configs_match {
            // In non-blocking mode, the RX config on the device may become of out sync with the saved config
            if !blocking {
                // Get the current config on the device
                let current_device_config = ioctl::get_rx_conf(handle)?;

                // Update the device if the config on the device and the saved config differ
                if current_device_config != *new_config {
                    ioctl::set_rx_conf(handle, new_config)?;
                }  
            }
        }
        else {
            ioctl::set_rx_conf(handle, new_config)?;
        }
        Ok(())
    }

    pub fn get_rx_config(&self) -> &Option<RXConfig> {
        &self.rx_config
    }

    pub fn get_device_tx_config(&mut self) -> Result<TXConfig, CC1101Error> {
        ioctl::get_tx_conf(&self.get_handle()?)
    }

    pub fn get_device_rx_config(&mut self) -> Result<RXConfig, CC1101Error> {
        ioctl::get_rx_conf(&self.get_handle()?)
    }

    pub fn get_device_raw_config(&self, config_type: RawConfigType) -> Result<RawConfig, CC1101Error> {
        ioctl::get_raw_conf(&self.get_handle()?, config_type)
    }

    pub fn transmit(&self, tx_config: &TXConfig, data: &[u8]) -> Result<(), CC1101Error> {
        
        let mut handle = self.get_handle()?;

        Self::set_tx_config_on_device(&handle, tx_config)?;

        match handle.write(data) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.raw_os_error() {
                    Some(libc::EINVAL) => Err(CC1101Error::Device(DeviceError::PacketSize)),
                    Some(libc::ENOMEM) => Err(CC1101Error::Device(DeviceError::OutOfMemory)),
                    Some(libc::EFAULT) => Err(CC1101Error::Device(DeviceError::Copy)),
                    _ => Err(CC1101Error::Device(DeviceError::Unknown))
                }
            }
        }
    }

    pub fn receive(&self) -> Result<Vec<Vec<u8>>, CC1101Error> {

        if let Some(rx_config) = &self.rx_config {
            let mut handle = self.get_handle()?;
            Self::set_rx_config_on_device(&handle, &self.rx_config, rx_config, self.handle.is_some())?;
            
            let mut packets = vec![];
            loop {
                let mut packet = vec![0; rx_config.get_packet_length() as usize];
                match handle.read(&mut packet) {
                    Ok(_) => {
                        packets.push(packet);
                    },
                    Err(e) => {
                        match e.raw_os_error() {
                            Some(libc::ENOMSG) => break,
                            Some(libc::EMSGSIZE) => return Err(CC1101Error::Device(DeviceError::PacketSize)),
                            Some(libc::EBUSY) => return Err(CC1101Error::Device(DeviceError::Busy)),
                            Some(libc::EINVAL) => return Err(CC1101Error::Device(DeviceError::InvalidConfig)),
                            Some(libc::EFAULT) => return Err(CC1101Error::Device(DeviceError::Copy)),
                            _ => {
                                return Err(CC1101Error::Device(DeviceError::Unknown));
                            }
                        }
                    }
                }
            }
    
            Ok(packets)
        }
        else {
            Err(CC1101Error::Device(DeviceError::NoRXConfig))
        }
    }
}