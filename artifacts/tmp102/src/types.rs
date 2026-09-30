//! Public configuration types.

// NB: `SlaveAddr::addr` needs the device base address, which lives in
// `register.rs` to keep the whole register map in one place.
use crate::register::DEVICE_BASE_ADDRESS;

/// Possible I2C slave addresses.
///
/// The ADD0 pin selects one of four addresses:
///
/// | ADD0 connection | Address |
/// | --------------- | ------- |
/// | GND             | `0x48`  |
/// | VDD             | `0x49`  |
/// | SDA             | `0x4A`  |
/// | SCL             | `0x4B`  |
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlaveAddr {
    /// `0x48` (ADD0 connected to GND).
    #[default]
    Default,
    /// Alternative address, providing bit values for `(A1, A0)`.
    ///
    /// - `(false, false)` → `0x48`
    /// - `(false, true)` → `0x49`
    /// - `(true, false)` → `0x4A`
    /// - `(true, true)` → `0x4B`
    Alternative(bool, bool),
}

impl SlaveAddr {
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => DEVICE_BASE_ADDRESS,
            SlaveAddr::Alternative(a1, a0) => {
                DEVICE_BASE_ADDRESS | ((u8::from(a1)) << 1) | u8::from(a0)
            }
        }
    }
}

/// Conversion mode of the sensor.
///
/// - [`Continuous`](ConversionMode::Continuous): the device converts
///   temperature continuously (power-up default).
/// - [`Shutdown`](ConversionMode::Shutdown): the device shuts down and
///   stops converting, cutting current consumption to minimise power.
///   A single conversion can then be triggered on demand (one-shot
///   triggering is Desirable functionality and out of scope here).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionMode {
    /// Continuous conversion mode (default).
    #[default]
    Continuous,
    /// Shutdown (one-shot) mode.
    Shutdown,
}
