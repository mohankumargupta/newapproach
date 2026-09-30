//! I2C register map and bit flags for Essential + Desirable + Rare functionality.
//!
//! Full register details are in the TMP102 / TMP112x datasheets.

/// Device base I2C address (`ADD0` to GND).
pub(crate) const DEVICE_BASE_ADDRESS: u8 = 0x48;

/// Register pointers.
pub(crate) struct Register;

impl Register {
    /// Temperature result register (read-only, 2 bytes).
    pub(crate) const TEMPERATURE: u8 = 0x00;
    /// Configuration register (read/write, 2 bytes: MSB first).
    pub(crate) const CONFIG: u8 = 0x01;
    /// Low temperature threshold (read/write, 2 bytes).
    pub(crate) const T_LOW: u8 = 0x02;
    /// High temperature threshold (read/write, 2 bytes).
    pub(crate) const T_HIGH: u8 = 0x03;
}

/// Configuration-register MSB flags.
pub(crate) struct BitFlagsHigh;

impl BitFlagsHigh {
    /// Shutdown bit: `1` = shutdown (one-shot) mode, `0` = continuous mode.
    pub(crate) const SHUTDOWN: u8 = 0b0000_0001;
    /// Thermostat mode bit: `1` = interrupt mode, `0` = comparator mode.
    pub(crate) const THERMOSTAT: u8 = 0b0000_0010;
    /// Alert polarity bit: `1` = active high, `0` = active low.
    pub(crate) const ALERT_POLARITY: u8 = 0b0000_0100;
    /// Fault-queue bit 0.
    pub(crate) const FAULT_QUEUE0: u8 = 0b0000_1000;
    /// Fault-queue bit 1.
    pub(crate) const FAULT_QUEUE1: u8 = 0b0001_0000;
    /// One-shot bit: write `1` in shutdown mode to start a conversion;
    /// reads as `1` when the one-shot result is ready.
    pub(crate) const ONE_SHOT: u8 = 0b1000_0000;
}

/// Configuration-register LSB flags.
pub(crate) struct BitFlagsLow;

impl BitFlagsLow {
    /// Extended-measurement-mode bit (Rare API; read by Desirable code so
    /// threshold conversion uses the active resolution).
    pub(crate) const EXTENDED_MODE: u8 = 0b0001_0000;
    /// Alert flag (read-only status bit).
    pub(crate) const ALERT: u8 = 0b0010_0000;
    /// Conversion-rate bit 0 (CR0).
    pub(crate) const CONV_RATE0: u8 = 0b0100_0000;
    /// Conversion-rate bit 1 (CR1).
    pub(crate) const CONV_RATE1: u8 = 0b1000_0000;
}
