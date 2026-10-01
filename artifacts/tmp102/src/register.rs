//! Register map (device description, no driver policy).

/// Register pointers.
pub(crate) struct Register;

impl Register {
    pub(crate) const TEMPERATURE: u8 = 0x00;
    pub(crate) const CONFIG: u8 = 0x01;
    pub(crate) const T_LOW: u8 = 0x02;
    pub(crate) const T_HIGH: u8 = 0x03;
}

/// Config register MSB (first byte) bit flags.
pub(crate) struct BitFlagsHigh;

impl BitFlagsHigh {
    pub(crate) const SHUTDOWN: u8 = 0b0000_0001;
    pub(crate) const THERMOSTAT: u8 = 0b0000_0010;
    pub(crate) const ALERT_POLARITY: u8 = 0b0000_0100;
    pub(crate) const FAULT_QUEUE0: u8 = 0b0000_1000;
    pub(crate) const FAULT_QUEUE1: u8 = 0b0001_0000;
    pub(crate) const RESOLUTION: u8 = 0b0110_0000;
    pub(crate) const ONE_SHOT: u8 = 0b1000_0000;
}

/// Config register LSB (second byte) bit flags.
pub(crate) struct BitFlagsLow;

impl BitFlagsLow {
    pub(crate) const EXTENDED_MODE: u8 = 0b0001_0000;
    pub(crate) const ALERT: u8 = 0b0010_0000;
    pub(crate) const CONV_RATE0: u8 = 0b0100_0000;
    pub(crate) const CONV_RATE1: u8 = 0b1000_0000;
}

/// Raw 16-bit register value (msb first on the wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RegisterU16 {
    pub(crate) msb: u8,
    pub(crate) lsb: u8,
}

pub(crate) type Config = RegisterU16;

impl Default for Config {
    /// Power-up default: 0x60A0.
    fn default() -> Self {
        Config {
            msb: BitFlagsHigh::RESOLUTION,
            lsb: BitFlagsLow::CONV_RATE1 | BitFlagsLow::ALERT,
        }
    }
}

impl RegisterU16 {
    pub(crate) fn with_high_msb(&self, mask: u8) -> Self {
        Self {
            msb: self.msb | mask,
            lsb: self.lsb,
        }
    }

    pub(crate) fn with_low_msb(&self, mask: u8) -> Self {
        Self {
            msb: self.msb & !mask,
            lsb: self.lsb,
        }
    }

    pub(crate) fn with_high_lsb(&self, mask: u8) -> Self {
        Self {
            msb: self.msb,
            lsb: self.lsb | mask,
        }
    }

    pub(crate) fn with_low_lsb(&self, mask: u8) -> Self {
        Self {
            msb: self.msb,
            lsb: self.lsb & !mask,
        }
    }
}
