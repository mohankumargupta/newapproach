//! I2C register map and bit flags needed for the Essential functionality.
//!
//! Full register details are in the TMP102 / TMP112x datasheets.
//! Registers used only by Desirable / Rare functionality (temperature
//! thresholds, extended-mode / conversion-rate / fault-queue / polarity /
//! thermostat configuration bits) are intentionally omitted here.

/// Device base I2C address (`ADD0` to GND).
pub(crate) const DEVICE_BASE_ADDRESS: u8 = 0x48;

/// Register pointers.
pub(crate) struct Register;

impl Register {
    /// Temperature result register (read-only, 2 bytes).
    pub(crate) const TEMPERATURE: u8 = 0x00;
    /// Configuration register (read/write, 2 bytes: MSB first).
    pub(crate) const CONFIG: u8 = 0x01;
}

/// Configuration-register MSB flags.
pub(crate) struct BitFlagsHigh;

impl BitFlagsHigh {
    /// Shutdown bit: `1` = shutdown (one-shot) mode, `0` = continuous mode.
    pub(crate) const SHUTDOWN: u8 = 0b0000_0001;
}
