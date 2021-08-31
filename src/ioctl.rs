use ioctl_sys::ioctl;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use crate::{CC1101Error, DeviceError};
use crate::config::{RXConfig, TXConfig, RawConfig, RawConfigType};

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
}

ioctl!(read ioctl_get_version with DEVICE_CHARACTER, Ioctl::GetVersion; u32);
ioctl!(none ioctl_reset with DEVICE_CHARACTER, Ioctl::Reset);
ioctl!(write ioctl_set_tx_conf with DEVICE_CHARACTER, Ioctl::SetTXConf; TXConfig);
ioctl!(write ioctl_set_rx_conf with DEVICE_CHARACTER, Ioctl::SetRXConf; RXConfig);
ioctl!(read ioctl_get_tx_conf with DEVICE_CHARACTER, Ioctl::GetTXConf; TXConfig);
ioctl!(read ioctl_get_rx_conf with DEVICE_CHARACTER, Ioctl::GetRXConf; RXConfig);
ioctl!(read ioctl_get_tx_raw_conf with DEVICE_CHARACTER, Ioctl::GetTXRawConf; RawConfig);
ioctl!(read ioctl_get_rx_raw_conf with DEVICE_CHARACTER, Ioctl::GetRXRawConf; RawConfig);
ioctl!(read ioctl_get_dev_raw_conf with DEVICE_CHARACTER, Ioctl::GetDevRawConf; RawConfig);

const EINVAL: i32 = - libc::EINVAL;
const ENOMEM: i32 = - libc::ENOMEM;
const EFAULT: i32 = - libc::EFAULT;

pub fn get_version(cc1101: &File) -> Result<u32, CC1101Error> {
    let mut version = 0;

    let status = unsafe { ioctl_get_version(cc1101.as_raw_fd(), &mut version) };

    match status {
        0 => Ok(version),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}

pub fn reset(cc1101: &File) -> Result<(), CC1101Error> {

    let status = unsafe { ioctl_reset(cc1101.as_raw_fd()) };

    match status {
        0 => Ok(()),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}

pub fn get_raw_conf(cc1101: &File, config_type: RawConfigType) -> Result<RawConfig, CC1101Error> {
    let mut config = RawConfig::default();

    let status = match config_type {
        RawConfigType::Device => unsafe { ioctl_get_dev_raw_conf(cc1101.as_raw_fd(), &mut config) },
        RawConfigType::Tx => unsafe { ioctl_get_tx_raw_conf(cc1101.as_raw_fd(), &mut config) },
        RawConfigType::Rx => unsafe { ioctl_get_rx_raw_conf(cc1101.as_raw_fd(), &mut config) }
    };
    
    match status {
        0 => Ok(config),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}

pub fn get_tx_conf(cc1101: &File) -> Result<TXConfig, CC1101Error> {
    let mut tx_config = TXConfig::default();

    let status = unsafe { ioctl_get_tx_conf(cc1101.as_raw_fd(), &mut tx_config) };

    match status {
        0 => Ok(tx_config),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}

pub fn get_rx_conf(cc1101: &File) -> Result<RXConfig, CC1101Error> {
    let mut rx_config = RXConfig::default();

    let status = unsafe { ioctl_get_rx_conf(cc1101.as_raw_fd(), &mut rx_config) };

    match status {
        0 => Ok(rx_config),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}

pub fn set_rx_conf(cc1101: &File, rx_config: &RXConfig) -> Result<(), CC1101Error> {
    
    let status = unsafe { ioctl_set_rx_conf(cc1101.as_raw_fd(), rx_config) };

    match status {
        0 => Ok(()),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        EFAULT => Err(CC1101Error::Device(DeviceError::Copy)),
        //EINVAL => Err(CC1101Error::Device(DeviceError::InvalidConfig)),
        ENOMEM => Err(CC1101Error::Device(DeviceError::OutOfMemory)),

        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}

pub fn set_tx_conf(cc1101: &File, tx_config: &TXConfig) -> Result<(), CC1101Error> {
    
    let status = unsafe { ioctl_set_tx_conf(cc1101.as_raw_fd(), tx_config) };

    match status {
        0 => Ok(()),
        EINVAL => Err(CC1101Error::Device(DeviceError::InvalidIOCTL)),
        EFAULT => Err(CC1101Error::Device(DeviceError::Copy)),
        //EINVAL => Err(CC1101Error::Device(DeviceError::InvalidConfig)),
        _ => Err(CC1101Error::Device(DeviceError::Unknown))
    }
}