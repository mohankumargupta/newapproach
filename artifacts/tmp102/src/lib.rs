//! Platform-agnostic `no_std` driver for the TMP102 and TMP112
//! digital temperature sensors, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! Supported devices: TMP102, TMP112 (including TMP112A, TMP112B, TMP112N).
//! Interface: I2C (addresses `0x48`–`0x4B`, selected via [`SlaveAddr`]).
//!
//! ## Scope
//!
//! This crate implements the **Essential**, **Desirable**, and **Rare**
//! functionality from the device plan:
//!
//! - Reading the temperature ([`Tmp102::read_temperature`]).
//! - Switching between continuous and shutdown (one-shot) conversion
//!   modes ([`Tmp102::set_conversion_mode`]).
//! - Triggering a one-shot measurement
//!   ([`Tmp102::trigger_one_shot_measurement`]).
//! - Polling one-shot readiness
//!   ([`Tmp102::is_one_shot_measurement_result_ready`]).
//! - Setting the conversion rate ([`Tmp102::set_conversion_rate`]).
//! - Setting the high/low temperature thresholds
//!   ([`Tmp102::set_low_temperature_threshold`],
//!   [`Tmp102::set_high_temperature_threshold`]).
//! - Enabling/disabling extended measurement mode
//!   ([`Tmp102::enable_extended_measurement_mode`],
//!   [`Tmp102::disable_extended_measurement_mode`]).
//! - Setting the fault queue ([`Tmp102::set_fault_queue`]).
//! - Setting the alert polarity ([`Tmp102::set_alert_polarity`]).
//! - Setting the thermostat mode ([`Tmp102::set_thermostat_mode`]).
//! - Reading comparator-mode alert status
//!   ([`Tmp102::is_comparator_mode_alert_active`]).
//!
//! See [`rare`] for the Rare category details.
//!
//! ## Blocking vs async
//!
//! By default the driver uses the blocking [`embedded-hal`] I2C traits.
//! Enable the `async` cargo feature to use the `embedded-hal-async` I2C
//! traits instead:
//!
//! ```toml
//! tmp102 = { version = "0.1", features = ["async"] }
//! ```
//!
//! ## Example (blocking)
//!
//! ```ignore
//! use tmp102::{ConversionMode, ConversionRate, SlaveAddr, Tmp102};
//!
//! let i2c = /* embedded-hal I2C implementation */;
//! let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
//! let temp_celsius: f32 = sensor.read_temperature().unwrap();
//! // Sleep to save power, then resume continuous measurement.
//! sensor.set_conversion_mode(ConversionMode::Shutdown).unwrap();
//! sensor.set_conversion_mode(ConversionMode::Continuous).unwrap();
//! // Desirable: on-demand measurement, conversion rate, thresholds.
//! sensor.trigger_one_shot_measurement().unwrap();
//! let ready: bool = sensor.is_one_shot_measurement_result_ready().unwrap();
//! sensor.set_conversion_rate(ConversionRate::Hz1).unwrap();
//! sensor.set_low_temperature_threshold(60.0).unwrap();
//! sensor.set_high_temperature_threshold(80.0).unwrap();
//! // Rare: extended mode, fault queue, alert polarity, thermostat mode.
//! sensor.enable_extended_measurement_mode().unwrap();
//! sensor.disable_extended_measurement_mode().unwrap();
//! sensor.set_fault_queue(tmp102::FaultQueue::Consecutive4).unwrap();
//! sensor.set_alert_polarity(tmp102::AlertPolarity::ActiveHigh).unwrap();
//! sensor.set_thermostat_mode(tmp102::ThermostatMode::Interrupt).unwrap();
//! let alert: bool = sensor.is_comparator_mode_alert_active().unwrap();
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

mod errors;
mod essential;
mod register;
mod tmp102;
mod types;

// Category modules holding the Essential / Desirable / Rare impl blocks.
pub mod desirable;
pub mod rare;

pub use errors::Error;
pub use tmp102::{Tmp102, Tmp112};
pub use types::{
    AlertPolarity, ConversionMode, ConversionRate, FaultQueue, SlaveAddr, ThermostatMode,
};
