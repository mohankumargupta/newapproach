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

/// Conversion rate of the sensor.
///
/// - [`Continuous`](ConversionMode::Continuous): the device converts
///   temperature continuously (power-up default).
/// - [`Shutdown`](ConversionMode::Shutdown): the device shuts down and
///   stops converting, cutting current consumption to minimise power.
///   A single conversion can then be triggered on demand (see
///   Desirable one-shot support).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionMode {
    /// Continuous conversion mode (default).
    #[default]
    Continuous,
    /// Shutdown (one-shot) mode.
    Shutdown,
}

/// Conversion rate for continuous conversion mode.
///
/// Controls how often the device converts in continuous mode,
/// trading wake time / power against responsiveness.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionRate {
    /// 0.25 Hz (slowest, lowest power).
    Hz0_25,
    /// 1 Hz.
    Hz1,
    /// 4 Hz (power-up default).
    #[default]
    Hz4,
    /// 8 Hz (fastest, most responsive).
    Hz8,
}

/// Fault queue: number of consecutive faults needed to trigger an alert.
///
/// Used to debounce false alarms from transient temperature spikes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultQueue {
    /// 1 consecutive fault triggers an alert (power-up default).
    #[default]
    Consecutive1,
    /// 2 consecutive faults trigger an alert.
    Consecutive2,
    /// 4 consecutive faults trigger an alert.
    Consecutive4,
    /// 6 consecutive faults trigger an alert.
    Consecutive6,
}

/// Alert polarity for the ALERT pin and alert flag.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertPolarity {
    /// Active low (power-up default).
    #[default]
    ActiveLow,
    /// Active high.
    ActiveHigh,
}

/// Thermostat mode controlling the ALERT pin behaviour.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermostatMode {
    /// Comparator mode (power-up default): alert stays active until the
    /// temperature falls below the low threshold.
    #[default]
    Comparator,
    /// Interrupt mode: alert is generated when the temperature exceeds the
    /// high threshold or goes below the low threshold.
    Interrupt,
}
