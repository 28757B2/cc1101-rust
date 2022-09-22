//! Configuration settings for the CC1101 radio
//!
//! The [`RXConfig`] and [`TXConfig`] structs are used to control the receive and transmit configuration of the CC1101.
//!
use crate::patable::{TX_POWERS_315, TX_POWERS_433, TX_POWERS_868, TX_POWERS_915};
use crate::{CC1101Error, ConfigError};
use std::fmt;

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
    MSK = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CarrierSense {
    Relative(i8),
    Absolute(i8),
}

impl fmt::Display for CarrierSense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CarrierSense::Relative(v) => write!(f, "Relative(+{} dB)", v),
            CarrierSense::Absolute(v) => write!(f, "Absolute({} dB)", v),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum CarrierSenseMode {
    Disabled = 0,
    Relative = 1,
    Absolute = 2,
}

/// Device / driver register types
#[derive(Copy, Clone)]
pub enum RegistersType {
    /// Hardware registers
    Device,
    /// Driver transmit configuration registers
    Tx,
    /// Driver receive configuration registers
    Rx,
}

/// CC1101 register values
#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Registers {
    /// GDO2 Output Pin Configuration
    pub IOCFG2: u8,
    /// GDO1 Output Pin Configuration
    pub IOCFG1: u8,
    /// GDO0 Output Pin Configuration
    pub IOCFG0: u8,
    /// RX FIFO and TX FIFO Thresholds
    pub FIFOTHR: u8,
    /// Sync Word, High Byte
    pub SYNC1: u8,
    /// Sync Word, Low Byte
    pub SYNC0: u8,
    /// Packet Length
    pub PKTLEN: u8,
    /// Packet Automation Control
    pub PKTCTRL1: u8,
    /// Packet Automation Control
    pub PKTCTRL0: u8,
    /// Device Address
    pub ADDR: u8,
    /// Channel Number
    pub CHANNR: u8,
    /// Frequency Synthesizer Control
    pub FSCTRL1: u8,
    /// Frequency Synthesizer Control
    pub FSCTRL0: u8,
    /// Frequency Control Word, High Byte
    pub FREQ2: u8,
    /// Frequency Control Word, Middle Byte
    pub FREQ1: u8,
    /// Frequency Control Word, Low Byte
    pub FREQ0: u8,
    /// Modem Configuration
    pub MDMCFG4: u8,
    /// Modem Configuration
    pub MDMCFG3: u8,
    /// Modem Configuration
    pub MDMCFG2: u8,
    /// Modem Configuration
    pub MDMCFG1: u8,
    /// Modem Configuration
    pub MDMCFG0: u8,
    /// Modem Deviation Setting
    pub DEVIATN: u8,
    /// Main Radio Control State Machine Configuration
    pub MCSM2: u8,
    /// Main Radio Control State Machine Configuration
    pub MCSM1: u8,
    /// Main Radio Control State Machine Configuration
    pub MCSM0: u8,
    /// Frequency Offset Compensation Configuration
    pub FOCCFG: u8,
    /// Bit Synchronization Configuration
    pub BSCFG: u8,
    /// AGC Control
    pub AGCCTRL2: u8,
    /// AGC Control
    pub AGCCTRL1: u8,
    /// AGC Control
    pub AGCCTRL0: u8,
    /// High Byte Event0 Timeout
    pub WOREVT1: u8,
    /// Low Byte Event0 Timeout
    pub WOREVT0: u8,
    /// Wake On Radio Control
    pub WORCTRL: u8,
    /// Front End RX Configuration
    pub FREND1: u8,
    /// Front End TX Configuration
    pub FREND0: u8,
    /// Frequency Synthesizer Calibration
    pub FSCAL3: u8,
    /// Frequency Synthesizer Calibration
    pub FSCAL2: u8,
    /// Frequency Synthesizer Calibration
    pub FSCAL1: u8,
    /// Frequency Synthesizer Calibration
    pub FSCAL0: u8,
    /// RC Oscillator Configuration
    pub RCCTRL1: u8,
    /// RC Oscillator Configuration
    pub RCCTRL0: u8,
    /// Frequency Synthesizer Calibration Control
    pub FSTEST: u8,
    /// Production Test
    pub PTEST: u8,
    /// AGC Test
    pub AGCTEST: u8,
    /// Various Test Settings
    pub TEST2: u8,
    /// Various Test Settings
    pub TEST1: u8,
    /// Various Test Settings
    pub TEST0: u8,
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
    sync_word: u32,
}

impl Default for CommonConfig {
    fn default() -> CommonConfig {
        CommonConfig {
            frequency: 0x10B071, // 433.92
            modulation: Modulation::OOK,
            baud_rate_mantissa: 0x43, // 1.0 kBaud
            baud_rate_exponent: 0x05,
            deviation_mantissa: 0x07, // 47.607422
            deviation_exponent: 0x04,
            sync_word: 0x0,
        }
    }
}

impl fmt::Display for CommonConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommonConfig: {{Frequency: {} MHz, Modulation: {:?}, Baud Rate: {} kBaud, Deviation: {} kHz, Sync Word: 0x{:08x}}}", Self::get_frequency(self), self.modulation, Self::get_baud_rate(self), Self::get_deviation(self), self.sync_word)
    }
}

/// Configuration values specific to receive
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct RXConfig {
    common: CommonConfig,
    bandwidth_mantissa: u8,
    bandwidth_exponent: u8,
    max_lna_gain: u8,
    max_dvga_gain: u8,
    magn_target: u8,
    carrier_sense_mode: CarrierSenseMode,
    carrier_sense: i8,
    packet_length: u32,
}

impl Default for RXConfig {
    fn default() -> RXConfig {
        RXConfig {
            common: CommonConfig::default(),
            bandwidth_mantissa: 0x00, // 203
            bandwidth_exponent: 0x02,
            max_lna_gain: 0,
            max_dvga_gain: 0,
            magn_target: 33,
            carrier_sense_mode: CarrierSenseMode::Relative,
            carrier_sense: 6,
            packet_length: 1024,
        }
    }
}

impl fmt::Display for RXConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let carrier_sense = match Self::get_carrier_sense(self) {
            Some(v) => format!("{}", v),
            None => "Disabled".to_owned(),
        };

        write!(f, "RXConfig: {{{}, Bandwidth: {} kHz, Max LNA Gain: {} dB, Max DVGA Gain: {} dB, Magn Target: {} dB, Carrier Sense: {}, Packet Length: {}}}", self.common, Self::get_bandwith(self), self.max_lna_gain, self.max_dvga_gain, self.magn_target, carrier_sense, self.packet_length)
    }
}

/// Configuration values specific to transmit
#[repr(C)]
#[derive(Debug, Default)]
pub struct TXConfig {
    common: CommonConfig,
    tx_power: u8,
}

impl fmt::Display for TXConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tx_power = match Self::get_tx_power(self) {
            Ok(tx_power) => format!("{} dBm", tx_power),
            Err(_) => format!("{:02x}", self.tx_power),
        };

        write!(f, "TXConfig: {{{}, TX Power: {}}}", self.common, tx_power)
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
    /// # use cc1101_rust::config::{CommonConfig, Modulation};
    /// let config = CommonConfig::new(433.92, Modulation::OOK, 1.0, None, None)?;
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    pub fn new(
        frequency: f32,
        modulation: Modulation,
        baud_rate: f32,
        deviation: Option<f32>,
        sync_word: Option<u32>,
    ) -> Result<CommonConfig, CC1101Error> {
        let mut config = CommonConfig::default();
        config.set_frequency(frequency)?;
        config.set_modulation_and_baud_rate(modulation, baud_rate)?;

        if let Some(sync_word) = sync_word {
            config.set_sync_word(sync_word)?;
        } else {
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
        if !((299.99976..=347.99994).contains(&frequency)
            || (386.99994..=463.9998).contains(&frequency)
            || (778.9999..=928.000000).contains(&frequency))
        {
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
    fn baud_rate_to_config(
        modulation: Modulation,
        baud_rate: f32,
    ) -> Result<(u8, u8), CC1101Error> {
        let valid_baud_rate = match modulation {
            Modulation::GFSK | Modulation::OOK => (0.599742..=249.939).contains(&baud_rate),
            Modulation::FSK2 => (0.599742..=500.0).contains(&baud_rate),
            Modulation::FSK4 => (0.599742..=299.927).contains(&baud_rate),
            Modulation::MSK => (25.9857..=499.878).contains(&baud_rate),
        };

        if !valid_baud_rate {
            return Err(CC1101Error::Config(ConfigError::InvalidBaudRate));
        }

        let xtal_freq = XTAL_FREQ * 1000000.0;

        let r_data = baud_rate * 1000.0;

        let exponent = ((r_data * 2_f32.powi(20)) / xtal_freq).log(2.0).floor();
        let mantissa =
            ((r_data * 2_f32.powi(28) / (xtal_freq * 2_f32.powf(exponent))) - 256_f32).round();

        let mantissa = mantissa as u8;
        let exponent = exponent as u8;

        Ok((mantissa, exponent))
    }

    /// Convert a baud rate configuration value to kBaud
    fn config_to_baud_rate(mantissa: u8, exponent: u8) -> f32 {
        let xtal_freq = XTAL_FREQ * 1000000.0;

        let r_data = ((((256 + mantissa as u32) as f32) * 2_f32.powi(exponent as i32))
            / 2_f32.powi(28))
            * xtal_freq;

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
    pub fn set_modulation_and_baud_rate(
        &mut self,
        modulation: Modulation,
        baud_rate: f32,
    ) -> Result<(), CC1101Error> {
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
        let dev =
            (xtal_freq / 2_f32.powi(17)) * (mantissa + 8) as f32 * 2_f32.powi(exponent as i32);
        round(dev / 1000.0, 6)
    }

    /// Convert a deviation in kHz to a configuration value
    fn deviation_to_config(deviation: f32) -> Result<(u8, u8), CC1101Error> {
        for mantissa in 0..8 {
            for exponent in 0..8 {
                #[allow(clippy::float_cmp)]
                if CommonConfig::config_to_deviation(mantissa, exponent) == deviation {
                    return Ok((mantissa, exponent));
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

    /// Get the frequency deviation in kHz
    pub fn get_deviation(&self) -> f32 {
        CommonConfig::config_to_deviation(self.deviation_mantissa, self.deviation_exponent)
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
    /// See [`CommonConfig`] for valid argument values.
    ///
    /// # Example
    ///
    /// ```
    /// # use cc1101_rust::config::{RXConfig, Modulation};
    /// let config = RXConfig::new(433.92, Modulation::OOK, 1.0, 1024, None, None, None, None, None, None, None)?;
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        frequency: f32,
        modulation: Modulation,
        baud_rate: f32,
        packet_length: u32,
        deviation: Option<f32>,
        sync_word: Option<u32>,
        bandwidth: Option<u32>,
        carrier_sense: Option<CarrierSense>,
        max_lna_gain: Option<u8>,
        max_dvga_gain: Option<u8>,
        magn_target: Option<u8>,
    ) -> Result<RXConfig, CC1101Error> {
        let common = CommonConfig::new(frequency, modulation, baud_rate, deviation, sync_word)?;

        let mut rx_config = RXConfig {
            common,
            packet_length,
            ..RXConfig::default()
        };

        rx_config.set_carrier_sense(carrier_sense)?;

        if let Some(bandwidth) = bandwidth {
            rx_config.set_bandwidth(bandwidth)?;
        }

        if let Some(max_lna_gain) = max_lna_gain {
            rx_config.set_max_lna_gain(max_lna_gain)?;
        }

        if let Some(max_dvga_gain) = max_dvga_gain {
            rx_config.set_max_dvga_gain(max_dvga_gain)?;
        }

        if let Some(magn_target) = magn_target {
            rx_config.set_magn_target(magn_target)?;
        }

        Ok(rx_config)
    }

    /// Get the common configuration elements
    pub fn get_common_config(&self) -> &CommonConfig {
        &self.common
    }

    /// Get a mutable reference to the common configuration elements
    pub fn get_common_config_mut(&mut self) -> &mut CommonConfig {
        &mut self.common
    }

    /// Convert a bandwidth configuration value to kHz.
    ///
    /// Uses the formula from section 13 of the datasheet
    fn config_to_bandwidth(mantissa: u8, exponent: u8) -> u32 {
        let xtal_freq = XTAL_FREQ * 1000000.0;
        let bw_channel = xtal_freq / (8.0 * (mantissa as f32 + 4.0) * 2_f32.powi(exponent as i32));
        (bw_channel / 1000.0) as u32
    }

    /// Convert a bandwidth in kHz to a configuration value
    fn bandwidth_to_config(bandwidth: u32) -> Result<(u8, u8), CC1101Error> {
        for mantissa in 0..4 {
            for exponent in 0..4 {
                #[allow(clippy::float_cmp)]
                if bandwidth == RXConfig::config_to_bandwidth(mantissa, exponent) {
                    return Ok((mantissa, exponent));
                }
            }
        }
        Err(CC1101Error::Config(ConfigError::InvalidBandwidth))
    }

    /// Set the configured bandwith in KHz
    ///
    /// Valid values are `58,67,81,101,116,135,162,203,232,270,325,406,464,541,650,812`
    pub fn set_bandwidth(&mut self, bandwidth: u32) -> Result<(), CC1101Error> {
        let (mantissa, exponent) = RXConfig::bandwidth_to_config(bandwidth)?;
        self.bandwidth_mantissa = mantissa;
        self.bandwidth_exponent = exponent;
        Ok(())
    }

    /// Get the configured bandwidth
    pub fn get_bandwith(&self) -> u32 {
        RXConfig::config_to_bandwidth(self.bandwidth_mantissa, self.bandwidth_exponent)
    }

    /// Sets the carrier sense threshold in dB.
    ///
    /// For [`CarrierSense::Relative`] an increase of 6, 10 or 14 dB can be specified. This will begin RX on a sudden increase in RSSI greather than or equal to this value.
    ///
    /// For [`CarrierSense::Absolute`] a value between -7 dB and 7 dB can be set. When RSSI exceeds `magn_target` +/- this value, packet RX will begin.
    /// `max_lna_gain` and `max_dvga_gain` will also require configuring to set the correct absolute RSSI value.
    ///
    /// [`None`] disables carrier sense.
    pub fn set_carrier_sense(
        &mut self,
        carrier_sense: Option<CarrierSense>,
    ) -> Result<(), CC1101Error> {
        match carrier_sense {
            Some(CarrierSense::Relative(carrier_sense)) => match carrier_sense {
                6 | 10 | 14 => {
                    self.carrier_sense_mode = CarrierSenseMode::Relative;
                    self.carrier_sense = carrier_sense;
                }
                _ => return Err(CC1101Error::Config(ConfigError::InvalidCarrierSense)),
            },
            Some(CarrierSense::Absolute(carrier_sense)) => match carrier_sense {
                -7..=7 => {
                    self.carrier_sense_mode = CarrierSenseMode::Absolute;
                    self.carrier_sense = carrier_sense;
                }
                _ => return Err(CC1101Error::Config(ConfigError::InvalidCarrierSense)),
            },
            None => {
                self.carrier_sense_mode = CarrierSenseMode::Disabled;
                self.carrier_sense = 0;
            }
        }
        Ok(())
    }

    /// Get the configured carrier sense
    pub fn get_carrier_sense(&self) -> Option<CarrierSense> {
        match self.carrier_sense_mode {
            CarrierSenseMode::Disabled => None,
            CarrierSenseMode::Relative => Some(CarrierSense::Relative(self.carrier_sense)),
            CarrierSenseMode::Absolute => Some(CarrierSense::Absolute(self.carrier_sense)),
        }
    }

    /// Sets the amount to decrease the maximum LNA gain by approximately the specified amount in dB.
    /// Valid values are `0, 3, 6, 7, 9, 12, 15, 17`
    pub fn set_max_lna_gain(&mut self, max_lna_gain: u8) -> Result<(), CC1101Error> {
        match max_lna_gain {
            0 | 3 | 6 | 7 | 9 | 12 | 15 | 17 => self.max_lna_gain = max_lna_gain,
            _ => return Err(CC1101Error::Config(ConfigError::InvalidMaxLNAGain)),
        }
        Ok(())
    }

    /// Get the configured maximum LNA gain
    pub fn get_max_lna_gain(&self) -> u8 {
        self.max_lna_gain
    }

    /// Sets the amount to decrease the maximum DVGA gain by approximately the specified amount in dB.
    /// Valid values are `0, 6, 12, 18`
    pub fn set_max_dvga_gain(&mut self, max_dvga_gain: u8) -> Result<(), CC1101Error> {
        match max_dvga_gain {
            0 | 6 | 12 | 18 => self.max_dvga_gain = max_dvga_gain,
            _ => return Err(CC1101Error::Config(ConfigError::InvalidMaxDVGAGain)),
        }
        Ok(())
    }

    /// Get the configured maximum DVGA gain
    pub fn get_max_dvga_gain(&self) -> u8 {
        self.max_dvga_gain
    }

    /// Sets the target channel filter amplitude in dB
    /// Valid values are `24, 27, 30, 33, 36, 38, 40, 42`
    pub fn set_magn_target(&mut self, magn_target: u8) -> Result<(), CC1101Error> {
        match magn_target {
            24 | 27 | 30 | 33 | 36 | 38 | 40 | 42 => self.magn_target = magn_target,
            _ => return Err(CC1101Error::Config(ConfigError::InvalidMagnTarget)),
        }
        Ok(())
    }

    /// Get the configured maximum DVGA gain
    pub fn get_magn_target(&self) -> u8 {
        self.magn_target
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
        } else if Self::frequency_near(frequency, 433.0) {
            Ok(TX_POWERS_433)
        } else if Self::frequency_near(frequency, 868.0) {
            Ok(TX_POWERS_868)
        } else if Self::frequency_near(frequency, 915.0) {
            Ok(TX_POWERS_915)
        } else {
            Err(CC1101Error::Config(ConfigError::InvalidFrequency))
        }
    }

    /// Create a new transmit configuration
    ///
    /// See [`CommonConfig`] for valid argument values.
    ///
    /// TX power is specified in dBm. Valid values can be found in [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)
    ///
    /// Frequency must be close to 315/433/868/915Mhz
    ///
    /// # Example
    ///
    /// ```
    /// # use cc1101_rust::config::{TXConfig, Modulation};
    /// let config = TXConfig::new(433.92, Modulation::OOK, 1.0, 0.1, None, None)?;
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    pub fn new(
        frequency: f32,
        modulation: Modulation,
        baud_rate: f32,
        tx_power: f32,
        deviation: Option<f32>,
        sync_word: Option<u32>,
    ) -> Result<TXConfig, CC1101Error> {
        let common = CommonConfig::new(frequency, modulation, baud_rate, deviation, sync_word)?;

        let mut tx_config = TXConfig {
            common,
            ..TXConfig::default()
        };

        tx_config.set_tx_power(tx_power)?;

        Ok(tx_config)
    }

    /// Get the common configuration elements
    pub fn get_common_config(&self) -> &CommonConfig {
        &self.common
    }

    /// Get a mutable reference to the common configuration elements
    pub fn get_common_config_mut(&mut self) -> &mut CommonConfig {
        &mut self.common
    }

    /// Create a new transmit configuration using a TX power specified as a raw CC1101 PATABLE byte.
    ///
    /// Frequency can be any valid value.
    ///
    /// See [`CommonConfig`] for valid argument values.
    ///
    /// # Example
    ///
    /// ```
    /// # use cc1101_rust::config::{TXConfig, Modulation};
    /// let mut config = TXConfig::new_raw(433.92, Modulation::OOK, 1.0, 0x60, None, None)?;
    /// # Ok::<(), cc1101_rust::CC1101Error>(())
    /// ```
    pub fn new_raw(
        frequency: f32,
        modulation: Modulation,
        baud_rate: f32,
        tx_power: u8,
        deviation: Option<f32>,
        sync_word: Option<u32>,
    ) -> Result<TXConfig, CC1101Error> {
        let common = CommonConfig::new(frequency, modulation, baud_rate, deviation, sync_word)?;
        Ok(TXConfig { common, tx_power })
    }

    /// Lookup a TX power in dBM in the appropriate power table (based on [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)).
    ///
    /// Frequency must be within 1MHz of 315/433/868/915Mhz
    fn tx_power_to_config(frequency: f32, tx_power: f32) -> Result<u8, CC1101Error> {
        let power_table = Self::get_power_table(frequency)?;

        for (hex, dbm) in power_table {
            if (dbm - tx_power).abs() < f32::EPSILON {
                return Ok(*hex);
            }
        }

        Err(CC1101Error::Config(ConfigError::InvalidTXPower))
    }

    /// Lookup a TX power PATABLE byte in the appropriate power table (based on [TI DN013](https://www.ti.com/lit/an/swra151a/swra151a.pdf)).
    ///
    /// Frequency must be within 1Mhz of 315/433/868/915Mhz
    fn config_to_tx_power(frequency: f32, tx_power: u8) -> Result<f32, CC1101Error> {
        let power_table = Self::get_power_table(frequency)?;

        for (hex, dbm) in power_table {
            if *hex == tx_power {
                return Ok(*dbm);
            }
        }

        Err(CC1101Error::Config(ConfigError::InvalidTXPower))
    }

    /// Set the TX power to a value in dBm.
    ///
    /// Configured frequency must be within 1Mhz of 315/433/868/915Mhz
    pub fn set_tx_power(&mut self, tx_power: f32) -> Result<(), CC1101Error> {
        self.tx_power = Self::tx_power_to_config(self.common.get_frequency(), tx_power)?;
        Ok(())
    }

    /// Get the TX power in dBm.
    ///
    /// Configured frequency must be within 1MHz of 315/433/868/915Mhz
    pub fn get_tx_power(&self) -> Result<f32, CC1101Error> {
        Self::config_to_tx_power(self.common.get_frequency(), self.tx_power)
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

        assert_eq!(CommonConfig::config_to_frequency(0x000B89D8), 299.999756);
        assert_eq!(CommonConfig::config_to_frequency(0x000D6276), 347.999939);
        assert_eq!(CommonConfig::config_to_frequency(0x000EE276), 386.999939);
        assert_eq!(CommonConfig::config_to_frequency(0x0011D89D), 463.999786);
        assert_eq!(CommonConfig::config_to_frequency(0x001DF627), 778.999878);
        assert_eq!(CommonConfig::config_to_frequency(0x0023B13B), 928.000000);

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
        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 0.6)?,
            (0x83, 0x04)
        );
        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 0.599742)?,
            (0x83, 0x04)
        );

        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 26.0)?,
            (0x06, 0x0A)
        );
        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 25.9857)?,
            (0x06, 0x0A)
        );

        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 250.0)?,
            (0x3B, 0x0D)
        );
        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 249.939)?,
            (0x3B, 0x0D)
        );

        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 300.0)?,
            (0x7A, 0x0D)
        );
        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 299.927)?,
            (0x7A, 0x0D)
        );

        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 500.0)?,
            (0x3B, 0x0E)
        );
        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 499.878)?,
            (0x3B, 0x0E)
        );

        assert_eq!(
            CommonConfig::baud_rate_to_config(Modulation::FSK2, 115.051)?,
            (0x22, 0x0C)
        );

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
        assert_eq!(RXConfig::bandwidth_to_config(812)?, (0x00, 0x00));
        assert_eq!(RXConfig::bandwidth_to_config(58)?, (0x03, 0x03));

        assert_eq!(RXConfig::config_to_bandwidth(0x00, 0x00), 812);
        assert_eq!(RXConfig::config_to_bandwidth(0x03, 0x03), 58);

        assert!(RXConfig::bandwidth_to_config(0).is_err());
        assert!(RXConfig::bandwidth_to_config(400).is_err());

        Ok(())
    }

    #[test]
    fn test_tx_power() -> Result<(), CC1101Error> {
        assert!(TXConfig::config_to_tx_power(123.0, 0xFF).is_err());
        assert!(TXConfig::config_to_tx_power(433.0, 0xFF).is_err());
        assert!(TXConfig::tx_power_to_config(433.0, -1.0).is_err());

        for frequency in [315.0, 433.0, 868.0, 915.0] {
            let power_table = TXConfig::get_power_table(frequency)?;
            for (hex, dbm) in power_table {
                assert_eq!(TXConfig::config_to_tx_power(frequency, *hex)?, *dbm);
                assert_eq!(TXConfig::tx_power_to_config(frequency, *dbm)?, *hex);
            }
        }

        Ok(())
    }
}
