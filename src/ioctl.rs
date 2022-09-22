use ioctl_sys::ioctl;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use crate::config::{RXConfig, Registers, RegistersType, TXConfig};
use crate::{CC1101Error, DeviceError};

const DEVICE_CHARACTER: u8 = b'c';

enum Ioctl {
    GetVersion = 0,
    Reset = 1,
    SetTXConf = 2,
    SetRXConf = 3,
    GetTXConf = 4,
    GetTXRawConf = 5,
    GetRXConf = 6,
    GetRXRawConf = 7,
    GetDevRawConf = 8,
    GetRSSI = 9,
    GetMaxPacketSize = 10,
}

ioctl!(read ioctl_get_version with DEVICE_CHARACTER, Ioctl::GetVersion; u32);
ioctl!(none ioctl_reset with DEVICE_CHARACTER, Ioctl::Reset);
ioctl!(write ioctl_set_tx_conf with DEVICE_CHARACTER, Ioctl::SetTXConf; TXConfig);
ioctl!(write ioctl_set_rx_conf with DEVICE_CHARACTER, Ioctl::SetRXConf; RXConfig);
ioctl!(read ioctl_get_tx_conf with DEVICE_CHARACTER, Ioctl::GetTXConf; TXConfig);
ioctl!(read ioctl_get_rx_conf with DEVICE_CHARACTER, Ioctl::GetRXConf; RXConfig);
ioctl!(read ioctl_get_tx_raw_conf with DEVICE_CHARACTER, Ioctl::GetTXRawConf; Registers);
ioctl!(read ioctl_get_rx_raw_conf with DEVICE_CHARACTER, Ioctl::GetRXRawConf; Registers);
ioctl!(read ioctl_get_dev_raw_conf with DEVICE_CHARACTER, Ioctl::GetDevRawConf; Registers);
ioctl!(read ioctl_get_rssi with DEVICE_CHARACTER, Ioctl::GetRSSI; u8);
ioctl!(read ioctl_get_max_packet_size with DEVICE_CHARACTER, Ioctl::GetMaxPacketSize; u32);

pub fn get_version(cc1101: &File) -> Result<u32, CC1101Error> {
    let mut version = 0;

    let status = unsafe { ioctl_get_version(cc1101.as_raw_fd(), &mut version) };

    match status {
        0 => Ok(version),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn reset(cc1101: &File) -> Result<(), CC1101Error> {
    let status = unsafe { ioctl_reset(cc1101.as_raw_fd()) };

    match status {
        0 => Ok(()),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn get_registers(cc1101: &File, config_type: RegistersType) -> Result<Registers, CC1101Error> {
    let mut config = Registers::default();

    let status = match config_type {
        RegistersType::Device => unsafe { ioctl_get_dev_raw_conf(cc1101.as_raw_fd(), &mut config) },
        RegistersType::Tx => unsafe { ioctl_get_tx_raw_conf(cc1101.as_raw_fd(), &mut config) },
        RegistersType::Rx => unsafe { ioctl_get_rx_raw_conf(cc1101.as_raw_fd(), &mut config) },
    };

    match status {
        0 => Ok(config),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn get_tx_conf(cc1101: &File) -> Result<TXConfig, CC1101Error> {
    let mut tx_config = TXConfig::default();

    let status = unsafe { ioctl_get_tx_conf(cc1101.as_raw_fd(), &mut tx_config) };

    match status {
        0 => Ok(tx_config),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn get_rx_conf(cc1101: &File) -> Result<RXConfig, CC1101Error> {
    let mut rx_config = RXConfig::default();

    let status = unsafe { ioctl_get_rx_conf(cc1101.as_raw_fd(), &mut rx_config) };

    match status {
        0 => Ok(rx_config),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn set_rx_conf(cc1101: &File, rx_config: &RXConfig) -> Result<(), CC1101Error> {
    let status = unsafe { ioctl_set_rx_conf(cc1101.as_raw_fd(), rx_config) };

    match status {
        0 => Ok(()),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        libc::EFAULT => Err(CC1101Error::Device(DeviceError::Copy)),
        libc::EINVAL => Err(CC1101Error::Device(DeviceError::InvalidConfig)),
        libc::ENOMEM => Err(CC1101Error::Device(DeviceError::OutOfMemory)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn set_tx_conf(cc1101: &File, tx_config: &TXConfig) -> Result<(), CC1101Error> {
    let status = unsafe { ioctl_set_tx_conf(cc1101.as_raw_fd(), tx_config) };

    match status {
        0 => Ok(()),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        libc::EFAULT => Err(CC1101Error::Device(DeviceError::Copy)),
        libc::EINVAL => Err(CC1101Error::Device(DeviceError::InvalidConfig)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn get_rssi(cc1101: &File) -> Result<u8, CC1101Error> {
    let mut rssi = 0;

    let status = unsafe { ioctl_get_rssi(cc1101.as_raw_fd(), &mut rssi) };

    match status {
        0 => Ok(rssi),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}

pub fn get_max_packet_size(cc1101: &File) -> Result<u32, CC1101Error> {
    let mut max_packet_size = 0;

    let status = unsafe { ioctl_get_max_packet_size(cc1101.as_raw_fd(), &mut max_packet_size) };

    match status {
        0 => Ok(max_packet_size),
        libc::EIO => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown)),
    }
}
