//! Configuration settings for the CC1101 radio
//!
//! The [`RXConfig`] and [`TXConfig`] structs are used to control the configuration of the CC1101.
//! 
use crate::{CC1101Error, ConfigError};

/// Radio modulation mode
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Modulation {
    /// Frequency Shift Keying (2 Frequencies)
    FSK2 = 0, 
    /// Gaussian Shaped Frequency Shift Keying
    GFSK = 1,
    /// On-Off Keying
    OOK = 3,
    /// Frequency Shift Keying (4 Frequencies)
    FSK4 = 4,
    /// Minimum Shift Keying
    MSK = 7 
}

// 
const TX_POWERS_315: &[(u8, f32); 109] = &[(0xC0,  10.6), (0xC1,  10.3), (0xC2,   9.9), (0xC3,   9.6), (0xC4,   9.2), (0xC5,   8.8), (0xC6,   8.5), (0xC7,   8.2), (0xC8,   7.9), (0xC9,   7.5), (0xCA,   7.2), (0xCB,   6.9), (0xCC,   6.6), /*(0x80,   6.6),*/ (0x81,   6.3), /*(0xCD,   6.3),*/ (0x82,   6.0), (0x83,   5.8), (0xCE,   5.6), (0x84,   5.4), (0x85,   5.0), (0x86,   4.7), (0x87,   4.3), (0x88,   3.9), (0x89,   3.5), (0x8A,   3.1), (0xCF,   2.8), (0x8B,   2.7), (0x8C,   2.2), (0x8D,   1.7), (0x50,   0.7), (0x8E,   0.6), (0x60,   0.5), (0x51,   0.1), (0x61,  -0.1), (0x40,  -0.3), (0x52,  -0.5), (0x62,  -0.7), (0x3F,  -0.8), (0x3E,  -1.0), (0x53,  -1.1), (0x3D,  -1.3), /*(0x63,  -1.3),*/ (0x3C,  -1.7), /*(0x54,  -1.7),*/ (0x64,  -1.9), (0x3B,  -2.1), (0x55,  -2.3), (0x65,  -2.5), (0x2F,  -2.6), (0x3A,  -2.7), (0x56,  -3.0), (0x2E,  -3.1), /*(0x66,  -3.1),*/ (0x39,  -3.4), (0x57,  -3.5), (0x2D,  -3.6), (0x67,  -3.7), (0x8F,  -4.2), /*(0x2C,  -4.2),*/ (0x38,  -4.3), /*(0x68,  -4.3),*/ (0x2B,  -4.9), /*(0x69,  -4.9),*/ (0x37,  -5.4), (0x6A,  -5.5), (0x2A,  -5.7), (0x6B,  -6.1), (0x29,  -6.5), (0x6C,  -6.7), /*(0x36,  -6.7),*/ (0x6D,  -7.2), (0x28,  -7.5), (0x35,  -8.1), (0x6E,  -8.4), (0x27,  -8.6), (0x26,  -9.8), (0x34,  -9.9), (0x25, -11.1), (0x33, -12.2), (0x24, -13.0), (0x6F, -13.2), (0x1F, -13.3), (0x1E, -13.9), (0x1D, -14.5), (0x1C, -15.2), (0x23, -15.4), (0x32, -15.6), (0x1B, -15.9), (0x1A, -16.6), (0x19, -17.5), (0x18, -18.5), (0x22, -18.8), /*(0x0F, -18.8),*/ (0x0E, -19.4), (0x17, -19.6), (0x0D, -20.0), (0x0C, -20.7), (0x16, -20.9), (0x31, -21.3), (0x0B, -21.4), (0x0A, -22.2), (0x15, -22.4), (0x09, -23.0), (0x08, -24.0), (0x14, -24.3), (0x21, -24.5), (0x07, -25.1), (0x06, -26.4), (0x13, -26.6), (0x05, -27.7), (0x04, -29.6), (0x12, -29.8), (0x03, -31.7), /*(0x02, -34.6),*/ (0x11, -34.6), (0x01, -38.3), (0x10, -41.2), (0x30, -41.3), /*(0x20, -41.3),*/ (0x00, -63.8)];
const TX_POWERS_433: &[(u8, f32); 103] = &[(0xc0, 9.9), (0xc1, 9.5), (0xc2, 9.2), (0xc3, 8.8), (0xc4, 8.5), (0xc5, 8.1), (0xc6, 7.8), (0xc7, 7.4), (0xc8, 7.1), (0xc9, 6.8), (0xca, 6.4), (0x80, 6.3), (0xcb, 6.1), (0x81, 6.0), (0xcc, 5.8), /*(0x82, 5.8),*/ (0xcd, 5.5), /*(0x83, 5.5),*/ (0x84, 5.1), (0xce, 4.9), (0x85, 4.8), (0x86, 4.4), (0x87, 4.0), (0x88, 3.6), (0x89, 3.2), (0x8a, 2.8), (0x8b, 2.3), (0xcf, 2.0), (0x8c, 1.9), (0x8d, 1.4), (0x8e, 0.4), /*(0x50, 0.4),*/ (0x60, 0.1), (0x51, -0.3), (0x61, -0.5), (0x40, -0.8), (0x52, -0.9), (0x62, -1.1), (0x3e, -1.4), (0x53, -1.5), (0x63, -1.7), (0x3c, -2.1), (0x54, -2.2), (0x64, -2.3), (0x3b, -2.5), (0x55, -2.8), (0x65, -2.9), (0x2f, -3.0), (0x3a, -3.1), (0x56, -3.3), (0x66, -3.5), (0x39, -3.8), (0x2d, -4.0), (0x67, -4.1), (0x8f, -4.6), (0x68, -4.7), (0x69, -5.3), (0x37, -5.6), (0x6a, -5.9), (0x2a, -6.0), (0x6b, -6.5), (0x36, -6.8), (0x29, -6.9), (0x6c, -7.1), (0x6d, -7.7), (0x28, -7.8), (0x35, -8.3), (0x27, -8.7), (0x6e, -8.9), (0x26, -9.9), (0x34, -10.1), (0x25, -11.4), (0x33, -12.3), (0x24, -13.3), (0x1f, -13.7), (0x1e, -14.3), (0x1d, -14.9), (0x1c, -15.5), (0x23, -15.6), (0x32, -15.7), (0x1b, -16.2), (0x1a, -17.0), (0x19, -17.8), (0x18, -18.8), (0x22, -19.0), (0xf, -19.3), (0xe, -19.8), (0xd, -20.4), (0x16, -21.0), (0x31, -21.3), (0xb, -21.7), (0xa, -22.5), (0x9, -23.3), (0x14, -24.3), (0x21, -24.5), (0x7, -25.3), (0x13, -26.5), (0x5, -27.9), (0x4, -29.5), (0x12, -29.6), (0x3, -31.4), (0x2, -33.8), (0x1, -36.5), (0x20, -38.3), (0x30, -38.4), (0x0, -62.7)];
const TX_POWERS_868: &[(u8, f32); 109] = &[(0xc0, 10.7), (0xc1, 10.3), (0xc2, 10.0), (0xc3, 9.6), (0xc4, 9.2), (0xc5, 8.9), (0xc6, 8.5), (0xc7, 8.2), (0xc8, 7.8), (0xc9, 7.5), (0xca, 7.2), (0xcb, 6.8), (0xcc, 6.5), (0xcd, 6.2), (0xce, 5.5), (0x80, 5.2), (0x81, 5.0), (0x82, 4.8), (0x83, 4.6), (0x84, 4.4), (0x85, 4.1), (0x86, 3.7), (0x87, 3.4), (0x88, 3.0), (0x89, 2.6), (0xcf, 2.4), (0x8a, 2.1), (0x8b, 1.7), (0x8c, 1.1), (0x8d, 0.6), (0x50, -0.3), (0x60, -0.5), /*(0x8e, -0.5),*/ (0x51, -0.9), (0x61, -1.1), (0x40, -1.5), (0x52, -1.6), (0x62, -1.8), (0x53, -2.3), (0x63, -2.4), (0x3f, -2.6), (0x3e, -2.8), (0x54, -2.9), (0x64, -3.1), (0x3d, -3.2), (0x3c, -3.5), (0x55, -3.6), (0x65, -3.7), (0x3b, -4.0), (0x56, -4.2), (0x66, -4.4), (0x2f, -4.5), /*(0x3a, -4.5),*/ (0x57, -4.8), (0x2e, -4.9), (0x67, -5.0), (0x39, -5.2), (0x2d, -5.5), (0x68, -5.7), (0x8f, -6.0), /*(0x2c, -6.0),*/ (0x38, -6.1), (0x69, -6.3), (0x2b, -6.7), (0x6a, -6.9), /*(0x37, -6.9),*/ (0x2a, -7.4), (0x6b, -7.5), (0x36, -8.1), (0x29, -8.2), (0x6c, -8.7), (0x28, -9.0), (0x35, -9.4), (0x27, -9.8), (0x26, -11.0), (0x34, -11.1), (0x25, -12.5), (0x33, -13.3), (0x24, -14.3), (0x6d, -14.5), (0x1f, -14.6), (0x1e, -15.1), (0x1d, -15.7), (0x1c, -16.4), (0x23, -16.5), /*(0x32, -16.5),*/ (0x1b, -17.0), (0x1a, -17.8), (0x19, -18.6), (0x18, -19.5), (0x22, -19.6), (0xf, -20.0), (0xe, -20.5), /*(0x17, -20.5),*/ (0xd, -21.1), (0xc, -21.7), /*(0x16, -21.7),*/ (0x31, -21.9), (0xb, -22.3), (0xa, -23.0), /*(0x15, -23.0),*/ (0x9, -23.8), (0x8, -24.6), (0x14, -24.7), (0x21, -24.8), (0x7, -25.5), (0x13, -26.5), /*(0x6, -26.5),*/ (0x5, -27.7), (0x12, -28.9), /*(0x4, -28.9),*/ (0x3, -30.2), (0x2, -31.7), /*(0x11, -31.7),*/ (0x1, -33.1), (0x10, -34.1), /*(0x20, -34.1),*/ (0x30, -34.2), (0x6e, -45.8), (0x0, -59.3), (0x6f, -69.2)];
const TX_POWERS_915: &[(u8, f32); 71] =  &[(0xc0, 9.4), (0xc1, 9.0), (0xc2, 8.6), (0xc3, 8.3), (0xc4, 7.9), (0xc5, 7.6), (0xc6, 7.2), (0xc7, 6.9), (0xc8, 6.6), (0xc9, 6.2), (0xca, 5.9), (0xcb, 5.6), (0xcc, 5.3), (0xcd, 5.0), (0x80, 4.9), (0x81, 4.7), (0x82, 4.5), (0xce, 4.3), (0x83, 4.2), (0x84, 3.9), (0x85, 3.6), (0x86, 3.3), (0x87, 2.9), (0x88, 2.5), (0x89, 2.2), (0x8a, 1.8), (0xcf, 1.6), (0x8b, 1.3), (0x8c, 0.9), (0x8d, 0.5), (0x8e, -0.6), (0x50, -0.9), (0x60, -1.1), (0x51, -1.6), (0x61, -1.8), (0x40, -2.1), (0x52, -2.2), (0x62, -2.4), (0x3f, -2.5), (0x3e, -2.7), (0x53, -2.9), (0x3d, -3.0), /*(0x63, -3.0),*/ (0x3c, -3.4), (0x22, -19.4), (0xf, -19.7), (0xe, -20.2), (0x17, -20.3), (0xd, -20.8), (0xc, -21.4), /*(0x16, -21.4),*/ (0x31, -21.7), (0xb, -22.0), (0xa, -22.7), (0x15, -22.8), (0x9, -23.5), (0x6d, -23.8), (0x8, -24.3), (0x14, -24.4), (0x21, -24.6), (0x7, -25.2), (0x13, -26.2), /*(0x6, -26.2),*/ (0x5, -27.3), (0x12, -28.6), /*(0x4, -28.6),*/ (0x3, -29.8), (0x2, -31.2), (0x11, -31.3), (0x1, -32.7), (0x10, -33.6), (0x20, -33.7), /*(0x30, -33.7),*/ (0x0, -58.2), (0x6e, -64.5), (0x6f, -69.7)];

/// Device / driver register types
#[derive(Copy, Clone)]
pub enum RegistersType {
    /// Hardware registers 
    Device,
    /// Driver transmit configuration registers
    Tx,
    /// Driver receive configuration registers
    Rx
}

/// CC1101 register values
#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Registers {
    IOCFG2: u8,      /// GDO2 Output Pin Configuration
    IOCFG1: u8,      // GDO1 Output Pin Configuration
    IOCFG0: u8,      // GDO0 Output Pin Configuration
    FIFOTHR: u8,     // RX FIFO and TX FIFO Thresholds
    SYNC1: u8,       // Sync Word, High Byte
    SYNC0: u8,       // Sync Word, Low Byte
    PKTLEN: u8,      // Packet Length
    PKTCTRL1: u8,    // Packet Automation Control
    PKTCTRL0: u8,    // Packet Automation Control
    ADDR: u8,        // Device Address
    CHANNR: u8,      // Channel Number
    FSCTRL1: u8,     // Frequency Synthesizer Control
    FSCTRL0: u8,     // Frequency Synthesizer Control
    FREQ2: u8,       // Frequency Control Word, High Byte
    FREQ1: u8,       // Frequency Control Word, Middle Byte
    FREQ0: u8,       // Frequency Control Word, Low Byte
    MDMCFG4: u8,     // Modem Configuration
    MDMCFG3: u8,     // Modem Configuration
    MDMCFG2: u8,     // Modem Configuration
    MDMCFG1: u8,     // Modem Configuration
    MDMCFG0: u8,     // Modem Configuration
    DEVIATN: u8,     // Modem Deviation Setting
    MCSM2: u8,       // Main Radio Control State Machine Configuration
    MCSM1: u8,       // Main Radio Control State Machine Configuration
    MCSM0: u8,       // Main Radio Control State Machine Configuration
    FOCCFG: u8,      // Frequency Offset Compensation Configuration
    BSCFG: u8,       // Bit Synchronization Configuration
    AGCCTRL2: u8,    // AGC Control
    AGCCTRL1: u8,    // AGC Control
    AGCCTRL0: u8,    // AGC Control
    WOREVT1: u8,     // High Byte Event0 Timeout
    WOREVT0: u8,     // Low Byte Event0 Timeout
    WORCTRL: u8,     // Wake On Radio Control
    FREND1: u8,      // Front End RX Configuration
    FREND0: u8,      // Front End TX Configuration
    FSCAL3: u8,      // Frequency Synthesizer Calibration
    FSCAL2: u8,      // Frequency Synthesizer Calibration
    FSCAL1: u8,      // Frequency Synthesizer Calibration
    FSCAL0: u8,      // Frequency Synthesizer Calibration
    RCCTRL1: u8,     // RC Oscillator Configuration
    RCCTRL0: u8,     // RC Oscillator Configuration
    FSTEST: u8,      // Frequency Synthesizer Calibration Control
    PTEST: u8,       // Production Test
    AGCTEST: u8,     // AGC Test
    TEST2: u8,       // Various Test Settings
    TEST1: u8,       // Various Test Settings
    TEST0: u8        // Various Test Settings
}

/// Configuration values shared between transmit and receive
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CommonConfig {
    frequency: u32,
    modulation: Modulation,
    baud_rate_mantissa: u8,
    baud_rate_exponent: u8,
    deviation_mantissa: u8,
    deviation_exponent: u8,
    sync_word: u32
}

impl Default for CommonConfig {
    fn default() -> CommonConfig {
        CommonConfig {
            frequency: 0xECC41E,
            modulation: Modulation::FSK2,
            baud_rate_mantissa: 0x22,
            baud_rate_exponent: 0x04,
            deviation_mantissa: 0x07,
            deviation_exponent: 0x04,
            sync_word: 0x091D3
        }
    }
}

/// Configuration values specific to receive
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct RXConfig {
    common: CommonConfig,
    bandwidth_mantissa: u8,
    bandwidth_exponent: u8,
    carrier_sense: u8,
    packet_length: u32,
}

impl Default for RXConfig {
    fn default() -> RXConfig {
        RXConfig {
            common: CommonConfig::default(),
            bandwidth_mantissa: 0x00,
            bandwidth_exponent: 0x02,
            carrier_sense: 33,
            packet_length: 1024
        }
    }
}

/// Configuration values specific to transmit
#[repr(C)]
#[derive(Debug)]
pub struct TXConfig {
    common: CommonConfig,
    tx_power: u8,
}

impl Default for TXConfig {
    fn default() -> TXConfig {
        TXConfig {
            common: CommonConfig::default(),
            tx_power: 0x00
        }
    }
}

const XTAL_FREQ: f32 = 26.0;

fn round(value: f32, precision: u8) -> f32 {
    let m = 10_f32.powi(precision as i32);
    (value * m).round() / m
}

impl CommonConfig {

    /// Create a new CommonConfig
    /// 
    /// # Example
    /// 
    /// ```
    /// CommonConfig::new(433.92, Modulation::OOK, 1.0, None, None)
    /// ```
    pub fn new(frequency: f32, modulation: Modulation, baud_rate: f32, deviation: Option<f32>, sync_word: Option<u32>) -> Result<CommonConfig, CC1101Error> {
        let mut config = CommonConfig::default();
        config.set_frequency(frequency)?;
        config.set_modulation_and_baud_rate(modulation, baud_rate)?;
        
        if let Some(sync_word) = sync_word {
            config.set_sync_word(sync_word)?;
        }
        else {
            config.set_sync_word(0x00)?;
        }

        if let Some(deviation) = deviation {
            config.set_deviation(deviation)?;
        }

        Ok(config)
    }

    /// Convert a frequency in MHz to a configuration value
    /// Uses the formula from section 21 of the CC1101 datasheet
    fn frequency_to_config(frequency: f32) -> Result<u32, CC1101Error> {

        if !(
            (299.99976..=347.99994).contains(&frequency) ||
            (386.99994..=463.9998).contains(&frequency) ||
            (778.9999..=928.000000).contains(&frequency)
        ) {
            return Err(CC1101Error::Config(ConfigError::InvalidFrequency));
        }

        let f = ((frequency * 65536_f32) / XTAL_FREQ) as u32;
        Ok(f)
    }

    /// Convert a configuration value to a frequency in MHz
    /// Uses the formula from section 21 of the CC1101 datasheet
    fn config_to_frequency(config: u32) -> f32 {
        (XTAL_FREQ / 2.0_f32.powi(16)) * config as f32
    }

    /// The frequency to receive/transmit on. 
    /// 
    /// Valid values are 300-348, 387-464 and 779-928 MHz.
    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), CC1101Error> {
        self.frequency = CommonConfig::frequency_to_config(frequency)?;
        Ok(())
    }

    /// Get the current receive/transmit frequency
    pub fn get_frequency(&self) -> f32 {
        CommonConfig::config_to_frequency(self.frequency)
    }

    /// Convert a baud rate in kBaud to a configuration value.
    /// 
    /// Uses the formula from section 12 of the datasheet
    fn baud_rate_to_config(modulation: Modulation, baud_rate: f32) -> Result<(u8, u8), CC1101Error> {
        let valid_baud_rate = match modulation {
            Modulation::GFSK | Modulation::OOK => {
                (0.599742..=249.939).contains(&baud_rate)
            },
            Modulation::FSK2 => {
                (0.599742..=500.0).contains(&baud_rate)
            },
            Modulation::FSK4 => {
                (0.599742..=299.927).contains(&baud_rate)
            }, 
            Modulation::MSK => {
                (25.9857..=499.878).contains(&baud_rate)
            }
        };

        if !valid_baud_rate {
            return Err(CC1101Error::Config(ConfigError::InvalidBaudRate));
        }

        let xtal_freq = XTAL_FREQ * 1000000.0;

        let r_data = baud_rate * 1000.0;

        let exponent = ((r_data * 2_f32.powi(20)) / xtal_freq).log(2.0).floor();
        let mantissa = ((r_data * 2_f32.powi(28) / (xtal_freq * 2_f32.powf(exponent))) - 256_f32).round();

        let mantissa = mantissa as u8;
        let exponent = exponent as u8;

        Ok((mantissa, exponent))
    }

    /// Convert a baud rate configuration value to kBaud
    fn config_to_baud_rate(mantissa: u8, exponent: u8) -> f32 {
        let xtal_freq = XTAL_FREQ * 1000000.0;

        let r_data = ((((256 + mantissa as u32) as f32) * 2_f32.powi(exponent as i32)) / 2_f32.powi(28)) * xtal_freq;

        round(r_data / 1000.0, 6)
    }

    /// Set the modulation scheme and the baud rate in kBaud
    /// 
    /// # Valid Modulation / Baud Rate Values
    /// 
    /// | Modulation           | Baud Rate  |
    /// | ---------------------| ---------- |
    /// | [`Modulation::OOK`]  | 0.6 - 250  |
    /// | [`Modulation::GFSK`] | 0.6 - 250  |
    /// | [`Modulation::FSK2`] | 0.6 - 500  |
    /// | [`Modulation::FSK4`] | 0.6 - 300  |
    /// | [`Modulation::MSK`]  | 26 - 500   |
    ///
    pub fn set_modulation_and_baud_rate(&mut self, modulation: Modulation, baud_rate: f32) -> Result<(), CC1101Error> {
        let (mantissa, exponent) = CommonConfig::baud_rate_to_config(modulation, baud_rate)?;
        self.modulation = modulation;
        self.baud_rate_mantissa = mantissa;
        self.baud_rate_exponent = exponent;
        Ok(())
    }

    /// Get the current modulation
    pub fn get_modulation(&self) -> Modulation {
        self.modulation
    }

    /// Get the current baud rate in kBaud
    pub fn get_baud_rate(&self) -> f32 {
        CommonConfig::config_to_baud_rate(self.baud_rate_mantissa, self.baud_rate_exponent)
    }

    /// Convert a deviation configuration value to kHz
    /// 
    /// Uses the formula from section 16.1 of the datasheet
    fn config_to_deviation(mantissa: u8, exponent: u8) -> f32 {
        let xtal_freq = XTAL_FREQ * 1000000.0;
        let dev = (xtal_freq / 2_f32.powi(17)) * (mantissa + 8) as f32 * 2_f32.powi(exponent as i32);
        round(dev / 1000.0, 6)
    }

    /// Convert a deviation in kHz to a configuration value
    fn deviation_to_config(deviation: f32) -> Result<(u8, u8), CC1101Error> {
        for mantissa in 0..8 {
            for exponent in 0..8 {
                #[allow(clippy::float_cmp)]
                if CommonConfig::config_to_deviation(mantissa, exponent) == deviation {
                    return Ok((mantissa, exponent))
                }
            }
        }
        Err(CC1101Error::Config(ConfigError::InvalidDeviation))
    }

    /// Set the frequency deviation in kHz
    pub fn set_deviation(&mut self, deviation: f32) -> Result<(), CC1101Error> {
        let (mantissa, exponent) = CommonConfig::deviation_to_config(deviation)?;
        self.deviation_mantissa = mantissa;
        self.deviation_exponent = exponent;
        Ok(())
    }

    /// Convert a sync word to a configuration value. 
    fn sync_word_to_config(sync_word: u32) -> Result<u32, CC1101Error> {
        if sync_word > 0xFFFF {
            let lsb = sync_word & 0x0000FFFF;
            let msb = sync_word >> 16;

            if lsb != msb {
                return Err(CC1101Error::Config(ConfigError::InvalidSyncWord));
            }
        }
        Ok(sync_word)
    }

    /// Set the sync word
    /// 
    /// Any sync word between 0x0000 and 0xFFFF is allowed. Above 0xFFFF, the high and low 16-bits must be the same (e.g `0x0f0f0f0f`).
    /// 
    /// In RX, the device searches for the specified sync word to begin reception.
    /// 
    /// In TX, the sync word is prepended to each packet.
    pub fn set_sync_word(&mut self, sync_word: u32) -> Result<(), CC1101Error> {
        self.sync_word = CommonConfig::sync_word_to_config(sync_word)?;
        Ok(())
    }

    /// Get the configured sync word
    pub fn get_sync_word(&self) -> u32 {
        self.sync_word
    }
}

impl RXConfig {

    /// Create a new receive configuration
    /// 
    /// # Example
    /// 
    /// ```
    /// RXConfig::new(433.92, Modulation::OOK, 1.0, None, None, 1024, None, None)
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(frequency: f32, modulation: Modulation, baud_rate: f32, deviation: Option<f32>, sync_word: Option<u32>, packet_length: u32, bandwidth: Option<f32>, carrier_sense: Option<u8>) -> Result<RXConfig, CC1101Error> {
        let common = CommonConfig::new(frequency, modulation, baud_rate, deviation, sync_word)?;

        let mut rx_config = RXConfig{
            common,
            packet_length,
            ..RXConfig::default()
        };

        if let Some(carrier_sense) = carrier_sense {
            rx_config.set_carrier_sense(carrier_sense)?;
        }

        if let Some(bandwidth) = bandwidth {
            rx_config.set_bandwidth(bandwidth)?;
        }

        Ok(rx_config)
    }

    /// Convert a bandwidth configuration value to kHz. 
    /// 
    /// Uses the formula from section 13 of the datasheet
    fn config_to_bandwidth(mantissa: u8, exponent: u8) -> f32 {
        let xtal_freq = XTAL_FREQ * 1000000.0;
        let bw_channel = xtal_freq / (8.0 * (mantissa as f32 + 4.0) * 2_f32.powi(exponent as i32));
        bw_channel / 1000.0
    }

    /// Convert a bandwidth in kHz to a configuration value
    fn bandwidth_to_config(bandwidth: f32) -> Result<(u8, u8), CC1101Error>{
        for mantissa in 0..4 {
            for exponent in 0..4 {
                #[allow(clippy::float_cmp)]
                if bandwidth == RXConfig::config_to_bandwidth(mantissa, exponent) {
                    return Ok((mantissa, exponent))
                }
            }
        }
        Err(CC1101Error::Config(ConfigError::InvalidBandwidth))
    }

    /// Set the configured bandwith in KHz
    /// 
    /// Valid values are `58,67,81,101,116,135,162,203,232,270,325,406,464,541,650,812`
    pub fn set_bandwidth(&mut self, bandwidth: f32) -> Result<(), CC1101Error> {
        let (mantissa, exponent) = RXConfig::bandwidth_to_config(bandwidth)?;
        self.bandwidth_mantissa = mantissa;
        self.bandwidth_exponent = exponent;
        Ok(())
    }

    /// Get the configured bandwidth
    pub fn get_bandwith(&self) -> f32 {
        RXConfig::config_to_bandwidth(self.bandwidth_mantissa, self.bandwidth_exponent)
    }

    /// Sets the carrier sense threshold in dB.
    /// 
    /// Valid values are `17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48`
    /// 
    /// When a sync word is provided, RX only begins when the carrier sense is above the threshold and the sync word has been received.
    pub fn set_carrier_sense(&mut self, carrier_sense: u8) -> Result<(), CC1101Error> {
        if !(17..=49).contains(&carrier_sense){
            return Err(CC1101Error::Config(ConfigError::InvalidCarrierSense));
        }
        Ok(())
    }

    /// Get the configured carrier sense
    pub fn get_carrier_sense(&self) -> u8 {
        self.carrier_sense
    }

    /// Set the length of packets to receive in bytes
    pub fn set_packet_length(&mut self, packet_length: u32) {
        self.packet_length = packet_length
    }

    /// Get the configured packet length
    pub fn get_packet_length(&self) -> u32 {
        self.packet_length
    }
}

impl TXConfig {
    
    /// Is a frequency close to a target frequency
    fn frequency_near(frequency: f32, target_frequency: f32) -> bool {
        (frequency - target_frequency).abs() < 1.0
    }

    /// Get the appropriate power table based on the provided frequency
    fn get_power_table(frequency: f32) -> Result<&'static [(u8, f32)], CC1101Error> {
        if Self::frequency_near(frequency, 315.0) {
            Ok(TX_POWERS_315)
        }
        else if Self::frequency_near(frequency, 433.0) {
            Ok(TX_POWERS_433)
        }
        else if Self::frequency_near(frequency, 868.0) {
            Ok(TX_POWERS_868)
        }
        else if Self::frequency_near(frequency, 915.0) {
            Ok(TX_POWERS_915)
        }
        else {
            Err(CC1101Error::Config(ConfigError::InvalidFrequency))
        }
    }

    
    /// Create a new transmit configuration using a TX power specified in dBm. 
    /// 
    /// Frequency must be close to 315/433/868/915Mhz
    /// 
    /// # Example
    /// 
    /// ```
    /// TXConfig::new(433.92, Modulation::OOK, 1.0, None, None, 0.1)
    /// ```
    pub fn new(frequency: f32, modulation: Modulation, baud_rate: f32, deviation: Option<f32>, sync_word: Option<u32>, tx_power: f32) -> Result<TXConfig, CC1101Error> {
        let common = CommonConfig::new(frequency, modulation, baud_rate, deviation, sync_word)?;

        let mut tx_config = TXConfig {
            common,
            ..TXConfig::default()
        };

        tx_config.set_tx_power_ism(tx_power)?;

        Ok(tx_config)
    }

    /// Create a new transmit configuration using a TX power specified as a raw CC1101 PATABLE byte. 
    /// 
    /// Frequency can be any valid value.
    /// 
    /// # Example
    /// 
    /// ```
    /// TXConfig::new_raw(433.92, Modulation::OOK, 1.0, None, None, 0x60)
    /// ```
    pub fn new_raw(frequency: f32, modulation: Modulation, baud_rate: f32, deviation: Option<f32>, sync_word: Option<u32>, tx_power: u8) -> Result<TXConfig, CC1101Error> {
        let common = CommonConfig::new(frequency, modulation, baud_rate, deviation, sync_word)?;
        Ok(TXConfig {common, tx_power})
    }

    /// Lookup a TX power in dBM in the appropriate power table (based on [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)). 
    /// 
    /// Frequency must be within 1MHz of 315/433/868/915Mhz
    fn tx_power_ism_to_config(frequency: f32, tx_power: f32) -> Result<u8, CC1101Error> {
        let power_table = Self::get_power_table(frequency)?;

        for (hex, dbm) in power_table {
            if (dbm-tx_power).abs() < f32::EPSILON {
                return Ok(*hex)
            }
        }

        Err(CC1101Error::Config(ConfigError::InvalidTXPower))
    }

    /// Lookup a TX power PATABLE byte in the appropriate power table (based on [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)). 
    /// 
    /// Frequency must be within 1Mhz of 315/433/868/915Mhz
    fn config_to_tx_power_ism(frequency: f32, tx_power: u8) -> Result<f32, CC1101Error> {
        let power_table = Self::get_power_table(frequency)?;

        for (hex, dbm) in power_table {
            if *hex == tx_power {
                return Ok(*dbm)
            }
        }

        Err(CC1101Error::Config(ConfigError::InvalidTXPower))
    }

    /// Set the TX power to a value in dBm. 
    /// 
    /// Based on the values in [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)
    /// 
    /// Configured frequency must be within 1Mhz of 315/433/868/915Mhz
    pub fn set_tx_power_ism(&mut self, tx_power: f32) -> Result<(), CC1101Error> {
        self.tx_power = Self::tx_power_ism_to_config(self.common.get_frequency(), tx_power)?;
        Ok(())
    }

    /// Get the TX power in dBm. 
    /// 
    /// Based on the values in [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)
    /// 
    /// Configured frequency must be within 1MHz of 315/433/868/915Mhz
    pub fn get_tx_power_ism(&mut self) -> Result<f32, CC1101Error> {
        Self::config_to_tx_power_ism(self.common.get_frequency(), self.tx_power)
    }
    
    /// Set the TX power to a raw value which will be set in the devices PATABLE
    pub fn set_tx_power_raw(&mut self, tx_power: u8) {
        self.tx_power = tx_power;
    }

    /// Get the TX power as raw value from the devices PATABLE
    pub fn get_tx_power_raw(&self) -> u8 {
        self.tx_power
    }

}

#[cfg(test)]
mod tests {
    #![allow(clippy::excessive_precision)]
    use super::*;

    #[test]
    fn test_freq() -> Result<(), CC1101Error> {

        assert_eq!(CommonConfig::frequency_to_config(315.0)?, 0x000C1D89);
        assert_eq!(CommonConfig::frequency_to_config(433.0)?, 0x0010A762);
        assert_eq!(CommonConfig::frequency_to_config(868.0)?, 0x00216276);
        assert_eq!(CommonConfig::frequency_to_config(915.0)?, 0x0023313B);

        assert_eq!(CommonConfig::frequency_to_config(299.999756)?, 0x000B89D8);
        assert_eq!(CommonConfig::frequency_to_config(347.999939)?, 0x000D6276);
        assert_eq!(CommonConfig::frequency_to_config(386.999939)?, 0x000EE276);
        assert_eq!(CommonConfig::frequency_to_config(463.999786)?, 0x0011D89D);
        assert_eq!(CommonConfig::frequency_to_config(778.999878)?, 0x001DF627);
        assert_eq!(CommonConfig::frequency_to_config(928.000000)?, 0x0023B13B);

        assert_eq!(CommonConfig::config_to_frequency(0x000B89D8),  299.999756); 
        assert_eq!(CommonConfig::config_to_frequency(0x000D6276),  347.999939); 
        assert_eq!(CommonConfig::config_to_frequency(0x000EE276),  386.999939); 
        assert_eq!(CommonConfig::config_to_frequency(0x0011D89D),  463.999786); 
        assert_eq!(CommonConfig::config_to_frequency(0x001DF627),  778.999878); 
        assert_eq!(CommonConfig::config_to_frequency(0x0023B13B),  928.000000); 

        assert_eq!(CommonConfig::config_to_frequency(0x000C1D89), 314.999664);
        assert_eq!(CommonConfig::config_to_frequency(0x0010A762), 432.999817);
        assert_eq!(CommonConfig::config_to_frequency(0x00216276), 867.999939);
        assert_eq!(CommonConfig::config_to_frequency(0x0023313B), 915.000000);

        assert!(CommonConfig::frequency_to_config(0.0).is_err());
        assert!(CommonConfig::frequency_to_config(464.0).is_err());
        assert!(CommonConfig::frequency_to_config(999.0).is_err());

        Ok(())
    }

    #[test]
    fn test_baud_rate() -> Result<(), CC1101Error> {
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 0.6)?, (0x83, 0x04));
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 0.599742)?, (0x83, 0x04));

        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 26.0)?, (0x06, 0x0A));
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 25.9857)?, (0x06, 0x0A));

        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 250.0)?, (0x3B, 0x0D));
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 249.939)?, (0x3B, 0x0D));

        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 300.0)?, (0x7A, 0x0D));
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 299.927)?, (0x7A, 0x0D));

        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 500.0)?, (0x3B, 0x0E));
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 499.878)?, (0x3B, 0x0E));
        
        assert_eq!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 115.051)?, (0x22, 0x0C));

        assert_eq!(CommonConfig::config_to_baud_rate(0x83, 0x04), 0.599742);
        assert_eq!(CommonConfig::config_to_baud_rate(0x06, 0x0A), 25.98572);
        assert_eq!(CommonConfig::config_to_baud_rate(0x3B, 0x0D), 249.93896);
        assert_eq!(CommonConfig::config_to_baud_rate(0x7A, 0x0D), 299.92676);
        assert_eq!(CommonConfig::config_to_baud_rate(0x3B, 0x0E), 499.87793);
        assert_eq!(CommonConfig::config_to_baud_rate(0x22, 0x0C), 115.05126);

        assert!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 0.0).is_err());
        assert!(CommonConfig::baud_rate_to_config(Modulation::FSK2, 999.0).is_err());

        Ok(())
    }

    #[test]
    fn test_deviation() -> Result<(), CC1101Error> {
        assert_eq!(CommonConfig::deviation_to_config(1.586914)?, (0x00, 0x00));
        assert_eq!(CommonConfig::deviation_to_config(380.85938)?, (0x07, 0x07));
        assert_eq!(CommonConfig::config_to_deviation(0x00, 0x00), 1.586914);
        assert_eq!(CommonConfig::config_to_deviation(0x07, 0x07), 380.859375);
        assert!(CommonConfig::deviation_to_config(0.0).is_err());
        assert!(CommonConfig::deviation_to_config(400.0).is_err());

        Ok(())
    }

    #[test]
    fn test_sync_word() -> Result<(), CC1101Error> {
        CommonConfig::sync_word_to_config(0x00000000)?;
        CommonConfig::sync_word_to_config(0x0000FFFF)?;
        CommonConfig::sync_word_to_config(0xFFFFFFFF)?;
        
        assert!(CommonConfig::sync_word_to_config(0xFFFF0000).is_err());
        assert!(CommonConfig::sync_word_to_config(0xAAAABBBB).is_err());
        Ok(())
    }
    
    #[test]
    fn test_bandwidth() -> Result<(), CC1101Error> {
        assert_eq!(RXConfig::bandwidth_to_config(812.500000)?, (0x00, 0x00));
        assert_eq!(RXConfig::bandwidth_to_config(58.035714)?, (0x03, 0x03));

        assert_eq!(RXConfig::config_to_bandwidth(0x00, 0x00), 812.500000);
        assert_eq!(RXConfig::config_to_bandwidth(0x03, 0x03), 58.035714);

        assert!(RXConfig::bandwidth_to_config(0.0).is_err());
        assert!(RXConfig::bandwidth_to_config(400.0).is_err());

        Ok(())
    }

    #[test]
    fn test_tx_power() -> Result<(), CC1101Error> {

        assert!(TXConfig::config_to_tx_power_ism(123.0, 0xFF).is_err());
        assert!(TXConfig::config_to_tx_power_ism(433.0, 0xFF).is_err());
        assert!(TXConfig::tx_power_ism_to_config(433.0, -1.0).is_err());

        for frequency in [315.0, 433.0, 868.0, 915.0] {
            let power_table = TXConfig::get_power_table(frequency)?;
            for (hex, dbm) in power_table {
                assert_eq!(TXConfig::config_to_tx_power_ism(frequency, *hex)?, *dbm);
                assert_eq!(TXConfig::tx_power_ism_to_config(frequency, *dbm)?, *hex);
            }
        }

        Ok(())
    }
}