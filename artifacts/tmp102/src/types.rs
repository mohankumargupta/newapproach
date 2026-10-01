//! I2C slave addresses and shared configuration types.

/// Possible I2C slave addresses.
///
/// Base address `0x48`, lower two bits set by ADD0 pin strapping:
/// - `0x48` ADD0 to GND
/// - `0x49` ADD0 to VDD
/// - `0x4A` ADD0 to SDA
/// - `0x4B` ADD0 to SCL
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlaveAddr {
    /// Default address `0x48` (ADD0 to GND).
    #[default]
    Default,
    /// Alternative address from ADD0 strapping bits `(a1, a0)`.
    ///
    /// Maps to `0x48 | (a1 << 1) | a0`, covering `0x48..=0x4B`.
    Alternative(bool, bool),
}

impl SlaveAddr {
    /// Resolve to a 7-bit address given the device base address.
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a1, a0) => default | ((a1 as u8) << 1) | a0 as u8,
        }
    }
}

/// Conversion mode (shutdown vs continuous).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionMode {
    /// Continuous conversion (active, default after power-up).
    #[default]
    Continuous,
    /// Shutdown / one-shot mode (sleep, low power).
    Shutdown,
}

/// Conversion rate for continuous conversion mode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionRate {
    /// 0.25 Hz
    _0_25Hz,
    /// 1 Hz
    _1Hz,
    /// 4 Hz (default)
    #[default]
    _4Hz,
    /// 8 Hz
    _8Hz,
}

/// Fault queue: consecutive faults needed to trigger an alert.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultQueue {
    /// 1 fault triggers an alert (default).
    #[default]
    _1,
    /// 2 consecutive faults trigger an alert.
    _2,
    /// 4 consecutive faults trigger an alert.
    _4,
    /// 6 consecutive faults trigger an alert.
    _6,
}

impl FaultQueue {
    /// Alias matching `plan.md` snippet (`FaultQueue::Consecutive4`).
    #[allow(non_upper_case_globals)]
    pub const Consecutive1: Self = Self::_1;
    /// Alias matching `plan.md` snippet.
    #[allow(non_upper_case_globals)]
    pub const Consecutive2: Self = Self::_2;
    /// Alias matching `plan.md` snippet (`FaultQueue::Consecutive4`).
    #[allow(non_upper_case_globals)]
    pub const Consecutive4: Self = Self::_4;
    /// Alias matching `plan.md` snippet.
    #[allow(non_upper_case_globals)]
    pub const Consecutive6: Self = Self::_6;
}

/// Alert (ALERT pin) polarity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertPolarity {
    /// Active low (default).
    #[default]
    ActiveLow,
    /// Active high.
    ActiveHigh,
}

/// Thermostat mode for the ALERT pin.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermostatMode {
    /// Comparator mode (default): alert stays active until temperature
    /// falls below the low threshold.
    #[default]
    Comparator,
    /// Interrupt mode: alert on crossing either threshold.
    Interrupt,
}
