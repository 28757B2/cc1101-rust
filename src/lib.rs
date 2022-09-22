//! This project provides an interface to the [CC1101 Linux Driver](https://github.com/28757B2/cc1101-driver) to allow receiving and transmitting packets from Rust.
//!
//! The CC1101 is a general purpose packet radio that operates in the Sub-GHz Industrial, Scientific and Medical (ISM) bands (315/433/868/915 MHz).
//!
//! The driver supports a subset of the CC1101 hardware's features and provides a high-level interface to the device that does not require setting of the individual hardware registers.
//!
//! * Frequencies - 300-348/387-464/778-928 MHz
//! * Modulation - OOK/2FSK/4FSK/GFSK/MSK
//! * Data Rate - 0.6 - 500 kBaud
//! * RX Bandwidth - 58 - 812 kHz
//! * Arbitrary packet length RX/TX
//! * Sync word or carrier sense triggered RX
//! * 16/32 bit configurable sync word

pub mod config;
mod ioctl;
mod patable;

use config::{RXConfig, Registers, RegistersType, TXConfig};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

// Driver version
const VERSION: u32 = 4;

/// Errors encountered during communication with the CC1101 driver
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
    Unknown,
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
    InvalidSyncWord,
    InvalidMaxLNAGain,
    InvalidMaxDVGAGain,
    InvalidMagnTarget,
}

/// Generic type for errors thrown by the module
#[derive(Debug)]
pub enum CC1101Error {
    Device(DeviceError),
    Config(ConfigError),
}

/// CC1101 radio device
///
/// This struct provides a handle to a CC1101 device, presented by the [Linux Driver](https://github.com/28757B2/cc1101-driver) as a character device (e.g `/dev/cc1101.0.0`).
/// It is used to configure the driver to receive and transmit packets.
///
/// # Receive
///
/// As all packet reception and interaction with the CC1101 hardware occurs within the kernel driver, receiving packets is an asynchronous process.
///
/// The driver begins RX when an IOCTL is sent to the character device with a receive configuration. Packets received by the radio are buffered within a FIFO
/// until being read into userspace via a `read()` call. Packet reception stops when a reset IOCTL is sent.
///
/// Calling [`CC1101::new`] or [`CC1101::set_rx_config`] with an [`RXConfig`] causes the driver to begin packet reception.
///
/// [`CC1101::receive`] can then later be used to read out the contents of the driver packet receive FIFO.
///
/// [`CC1101::reset`] is used to stop packet reception by the driver.
///
/// # Transmit
///
/// Transmission of packets is a synchronous process.
///
/// TX occurs when an IOCTL is sent to the character device with a transmit configuration and a `write()` call made with the bytes to transmit.
/// When a `write()` occurs, RX is paused, the device is reconfigured for TX and the provided packet is transmitted. Once TX completes, the receive config is restored and RX continues.
/// The `write()` call blocks until completion of the transmission.
///
/// [`CC1101::transmit`] is used to transmit packets using a [`TXConfig`]. This call will block until TX is complete.
///
/// # Device Sharing
///
/// It is possible to share a CC1101 character device between multiple receiving and transmitting process.
///
/// For example, this allows a long-running receiving process to share a device with a process that occasionally transmits.
///
/// The receiving process periodically polls the character device to check for new packets. Another process can acquire the character device while the receving process is sleeping and request a transmit.
/// This will cause the driver to reconfigure the hardware, transmit, then return to the receive configuration and continue listening for new packets.
///
///
/// This behaviour is controlled by the `blocking` argument to [`CC1101::new`]. Specifying `false` will release the file handle to the character device after every [`CC1101::receive`] and [`CC1101::transmit`] call.
/// This enables another process to aquire a handle to use the radio between events. The `open()` call on the character device will block until the radio becomes available again.
///
/// Specifying `true` will hold the file handle open while the [`CC1101`] struct is kept in scope. This prevents another process from using the device between events.
///
/// Note - sharing a device between two receiving processes will cause packet loss, as the driver's internal packet buffer is reset each time a new receive configuration is set.
///
pub struct CC1101 {
    device: String,
    handle: Option<File>,
    rx_config: Option<RXConfig>,
}

impl CC1101 {
    /// Create a new handle to a CC1101 device
    ///
    /// Providing an `rx_config` will configure the driver for RX with the provided configuration and begin packet reception. Received packets can be read using [`CC1101::receive`].
    ///
    /// `blocking` determines if the file handle to the device should be kept open. This prevents another process from using the radio (and reconfiguring it), but prevents sharing of the device with another process.
    ///
    /// # Example
    ///
    /// ```
    /// # use cc1101_rust::{CC1101, config::{RXConfig, Modulation}};
    /// let rx_config = RXConfig::new(433.92, Modulation::OOK, 1.0, 64, None, None, None, None, None, None, None)?;
    /// let cc1101 = CC1101::new("/dev/cc1101.0.0", Some(rx_config), false)?;
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    pub fn new(
        device: &str,
        rx_config: Option<RXConfig>,
        blocking: bool,
    ) -> Result<CC1101, CC1101Error> {
        let handle = Self::open(device)?;

        if let Some(rx_config) = &rx_config {
            Self::set_rx_config_on_device(&handle, &None, rx_config, blocking)?;
        }

        match blocking {
            true => Ok(CC1101 {
                device: device.to_string(),
                handle: Some(handle),
                rx_config,
            }),
            false => Ok(CC1101 {
                device: device.to_string(),
                handle: None,
                rx_config,
            }),
        }
    }

    /// Get the current RSSI value from the radio
    pub fn get_rssi(&self) -> Result<u8, CC1101Error> {
        let handle = self.get_handle()?;
        ioctl::get_rssi(&handle)
    }

    /// Get the maximum packet size configured in the driver
    pub fn get_max_packet_size(&self) -> Result<u32, CC1101Error> {
        let handle = self.get_handle()?;
        ioctl::get_max_packet_size(&handle)
    }

    /// Receive packets from the radio
    ///
    /// This will read the content of the driver's received packet buffer if the driver is already in RX.
    ///
    /// If the driver is not in RX (i.e [`CC1101::reset`] has been called), calling this will configure the driver for RX and begin packet reception.
    ///
    /// Individual packets are a [`Vec<u8>`] of the size specified in the `packet_length` argument to [`RXConfig::new`].
    ///
    /// The return type is [`Vec<Vec<u8>>`], as multiple packets can be returned in one receive call. This will be empty if no packets have been received.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::{thread, time};
    /// # use cc1101_rust::{CC1101, config::{RXConfig, Modulation}};
    /// let rx_config = RXConfig::new(433.92, Modulation::OOK, 1.0, 64, None, None, None, None, None, None, None)?;
    /// let cc1101 = CC1101::new("/dev/cc1101.0.0", Some(rx_config), false)?;
    ///
    /// loop {
    ///     let packets = cc1101.receive()?;
    ///     for packet in packets {
    ///         println!("Received - {:x?}", packet);
    ///     }
    ///     thread::sleep(time::Duration::from_millis(100));
    /// }
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    pub fn receive(&self) -> Result<Vec<Vec<u8>>, CC1101Error> {
        if let Some(rx_config) = &self.rx_config {
            let mut handle = self.get_handle()?;
            Self::set_rx_config_on_device(
                &handle,
                &self.rx_config,
                rx_config,
                self.handle.is_some(),
            )?;

            let mut packets = vec![];
            loop {
                let mut packet = vec![0; rx_config.get_packet_length() as usize];
                match handle.read(&mut packet) {
                    Ok(_) => {
                        packets.push(packet);
                    }
                    Err(e) => match e.raw_os_error() {
                        Some(libc::ENOMSG) => break,
                        Some(libc::EMSGSIZE) => {
                            return Err(CC1101Error::Device(DeviceError::PacketSize))
                        }
                        Some(libc::EBUSY) => return Err(CC1101Error::Device(DeviceError::Busy)),
                        Some(libc::EINVAL) => {
                            return Err(CC1101Error::Device(DeviceError::InvalidConfig))
                        }
                        Some(libc::EFAULT) => return Err(CC1101Error::Device(DeviceError::Copy)),
                        _ => {
                            return Err(CC1101Error::Device(DeviceError::Unknown));
                        }
                    },
                }
            }

            Ok(packets)
        } else {
            Err(CC1101Error::Device(DeviceError::NoRXConfig))
        }
    }

    /// Transmit a packet via the radio using the provided configuration
    ///
    /// # Example
    /// ```no_run
    /// # use cc1101_rust::{CC1101, config::{TXConfig, Modulation}};
    /// const PACKET: [u8; 11] = [0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f];       
    ///
    /// let tx_config = TXConfig::new(433.92, Modulation::OOK, 1.0, 0.1, None, None)?;
    /// let cc1101 = CC1101::new("/dev/cc1101.0.0", None, false)?;
    ///
    /// cc1101.transmit(&tx_config, &PACKET)?;
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    ///
    pub fn transmit(&self, tx_config: &TXConfig, data: &[u8]) -> Result<(), CC1101Error> {
        let mut handle = self.get_handle()?;

        Self::set_tx_config_on_device(&handle, tx_config)?;

        match handle.write(data) {
            Ok(_) => Ok(()),
            Err(e) => match e.raw_os_error() {
                Some(libc::EINVAL) => Err(CC1101Error::Device(DeviceError::PacketSize)),
                Some(libc::ENOMEM) => Err(CC1101Error::Device(DeviceError::OutOfMemory)),
                Some(libc::EFAULT) => Err(CC1101Error::Device(DeviceError::Copy)),
                _ => Err(CC1101Error::Device(DeviceError::Unknown)),
            },
        }
    }

    /// Open a file handle to the device
    fn open(device: &str) -> Result<File, CC1101Error> {
        let handle = match OpenOptions::new().read(true).write(true).open(device) {
            Ok(file) => file,
            Err(e) => match e.raw_os_error() {
                Some(libc::EBUSY) => return Err(CC1101Error::Device(DeviceError::Busy)),
                _ => return Err(CC1101Error::Device(DeviceError::Unknown)),
            },
        };

        let version = ioctl::get_version(&handle)?;

        if version != VERSION {
            return Err(CC1101Error::Device(DeviceError::VersionMismatch));
        }

        Ok(handle)
    }

    /// Get a handle to the device.
    ///
    /// Either re-use the existing handle if in blocking mode, or create a new one.
    fn get_handle(&self) -> Result<File, CC1101Error> {
        if let Some(handle) = &self.handle {
            match handle.try_clone() {
                Ok(h) => Ok(h),
                Err(_) => Err(CC1101Error::Device(DeviceError::FileHandleClone)),
            }
        } else {
            Ok(Self::open(&self.device)?)
        }
    }

    /// Issue a reset command to the device.
    ///
    /// This will clear the received packet buffer and stop receiving. Packet reception can be resumed by calling [`CC1101::receive`].
    pub fn reset(&mut self) -> Result<(), CC1101Error> {
        ioctl::reset(&self.get_handle()?)
    }

    fn set_tx_config_on_device(handle: &File, tx_config: &TXConfig) -> Result<(), CC1101Error> {
        ioctl::set_tx_conf(handle, tx_config)
    }

    /// Set the receive configuration.
    ///
    /// This will configure the driver for RX with the provided configuration and begin packet reception. Received packets can be read using [`CC1101::receive`].
    ///
    pub fn set_rx_config(&mut self, rx_config: &RXConfig) -> Result<(), CC1101Error> {
        Self::set_rx_config_on_device(
            &self.get_handle()?,
            &self.rx_config,
            rx_config,
            self.handle.is_some(),
        )?;
        self.rx_config = Some(rx_config.clone());
        Ok(())
    }

    fn set_rx_config_on_device(
        handle: &File,
        old_config: &Option<RXConfig>,
        new_config: &RXConfig,
        blocking: bool,
    ) -> Result<(), CC1101Error> {
        // Does the new config match the saved config
        let configs_match = match old_config {
            Some(old_config) => old_config == new_config,
            None => false,
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
        } else {
            ioctl::set_rx_conf(handle, new_config)?;
        }
        Ok(())
    }

    /// Get the configured receive config
    pub fn get_rx_config(&self) -> &Option<RXConfig> {
        &self.rx_config
    }

    /// Get the transmit configuration currently set in the driver
    pub fn get_device_tx_config(&mut self) -> Result<TXConfig, CC1101Error> {
        ioctl::get_tx_conf(&self.get_handle()?)
    }

    /// Get the receive configuration currently set in the driver
    ///
    /// In non-blocking mode, this may differ from the value returned by [`CC1101::get_rx_config`] if another process has reconfigured the device.
    pub fn get_device_rx_config(&mut self) -> Result<RXConfig, CC1101Error> {
        ioctl::get_rx_conf(&self.get_handle()?)
    }

    /// Get the set of hardware registers for RX/TX currently configured in the driver, or currently configured on the CC1101
    pub fn get_device_registers(
        &self,
        registers_type: RegistersType,
    ) -> Result<Registers, CC1101Error> {
        ioctl::get_registers(&self.get_handle()?, registers_type)
    }
}
