//! This project provides an interface to the [CC1101 Linux Driver](https://github.com/28757B2/cc1101-driver) to allow receiving and transmitting packets from Rust.

pub mod config;
mod ioctl;

use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use config::{TXConfig, RXConfig, Registers, RegistersType};

// Driver version
const VERSION: u32 = 2;

/// Errors encountered while communicating with the CC1101 driver
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

/// Errors caused by device configuration
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

/// Generic error type for errors thrown by the module
#[derive(Debug)]
pub enum CC1101Error {
    Device(DeviceError),
    Config(ConfigError)
}

/// CC1101 radio device
pub struct CC1101 {
    device: String,
    handle: Option<File>,
    rx_config: Option<RXConfig>
}

impl CC1101 {

    /// Create a new handle to a CC1101 device
    /// 
    /// Providing an `rx_config` immediately configures the device driver to receive with the provided configuration. Received packets can be read using [`CC1101::receive`].
    /// 
    /// `blocking` determines if the file handle to the device should be kept open. This prevents another process from using the radio (and reconfiguring it), but prevents multiplexing of transmit/recieve between two processes on the same device.
    /// 
    /// # Example
    /// 
    /// ```
    /// let rx_config = RXConfig::new(433.92, Modulation::OOK, 1.0, None, None, 64, None, None)?;
    /// let cc1101 = CC1101::new("/dev/cc1101.0.0", Some(rx_config), false)?;    
    /// ```
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

    /// Open a file handle to the device
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

    /// Get a handle to the device.
    /// 
    /// Etiher re-use the existing handle if in blocking mode, or create a new one.
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

    /// Issue a reset command to the device, setting it to idle
    pub fn reset(&mut self) -> Result<(), CC1101Error> {
        ioctl::reset(&self.get_handle()?)
    }

    /// Set the transmit configuration
    pub fn set_tx_config(&mut self, tx_config: &TXConfig) -> Result<(), CC1101Error> {
        Self::set_tx_config_on_device(&self.get_handle()?, tx_config)
    }

    fn set_tx_config_on_device(handle: &File, tx_config: &TXConfig) -> Result<(), CC1101Error> {
        ioctl::set_tx_conf(handle, tx_config)
    }

    /// Set the receive configuration
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

    /// Get the configured receive config
    pub fn get_rx_config(&self) -> &Option<RXConfig> {
        &self.rx_config
    }

    /// Get the transmit config currently configured on the device
    pub fn get_device_tx_config(&mut self) -> Result<TXConfig, CC1101Error> {
        ioctl::get_tx_conf(&self.get_handle()?)
    }

    /// Get the receive config currently configured on the device
    /// 
    /// In non-blocking mode, this may differ from the value returned by [`CC1101::get_rx_config`] if another process has reconfigured the device
    pub fn get_device_rx_config(&mut self) -> Result<RXConfig, CC1101Error> {
        ioctl::get_rx_conf(&self.get_handle()?)
    }

    /// Get the set of CC1101 registers currently configured on the device
    pub fn get_device_registers(&self, registers_type: RegistersType) -> Result<Registers, CC1101Error> {
        ioctl::get_registers(&self.get_handle()?, registers_type)
    }

    /// Transmit a packet using the radio
    /// 
    /// # Example
    /// ```
    /// const PACKET: [u8; 11] = [0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f];       
    /// let tx_config = TXConfig::new(433.92, Modulation::OOK, 1.0, None, None, 0.1)?;
    /// 
    /// let cc1101 = CC1101::new("/dev/cc1101.0.0", None, false)?;
    /// 
    /// cc1101.transmit(&tx_config, &PACKET)?;
    /// ```
    /// 
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

    /// Configure and receive packets from the radio
    /// 
    /// # Example
    /// 
    /// ```
    /// let rx_config = RXConfig::new(433.92, Modulation::OOK, 1.0, None, None, 64, None, None)?;
    /// let cc1101 = CC1101::new("/dev/cc1101.0.0", Some(rx_config), false)?;
    /// 
    /// loop {
    ///     let packets = cc1101.receive()?;
    ///     for packet in packets {
    ///         println!("Received - {:x?}", packet);
    ///     }
    ///     thread::sleep(time::Duration::from_millis(100));
    /// }
    /// ```
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