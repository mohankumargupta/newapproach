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
//! This crate implements only the **Essential** functionality from the
//! device plan:
//!
//! - Reading the temperature ([`Tmp102::read_temperature`]).
//! - Switching between continuous and shutdown (one-shot) conversion
//!   modes ([`Tmp102::set_conversion_mode`]).
//!
//! Desirable and Rare functionality (one-shot triggering, conversion rate,
//! temperature thresholds, extended mode, fault queue, alert polarity,
//! thermostat mode) is intentionally out of scope; see
//! [`desirable`] and [`rare`].
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
//! use tmp102::{ConversionMode, SlaveAddr, Tmp102};
//!
//! let i2c = /* embedded-hal I2C implementation */;
//! let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
//! let temp_celsius: f32 = sensor.read_temperature().unwrap();
//! // Sleep to save power, then resume continuous measurement.
//! sensor.set_conversion_mode(ConversionMode::Shutdown).unwrap();
//! sensor.set_conversion_mode(ConversionMode::Continuous).unwrap();
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

mod errors;
mod essential;
mod register;
mod tmp102;
mod types;

// Placeholders documenting the out-of-scope plan categories.
pub mod desirable;
pub mod rare;

pub use errors::Error;
pub use tmp102::{Tmp102, Tmp112};
pub use types::{ConversionMode, SlaveAddr};
