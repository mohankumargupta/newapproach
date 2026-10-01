//! Platform-agnostic `no_std` driver for TMP102 / TMP112 temperature sensors.
//!
//! I2C only. Implements `embedded-hal` v1 (blocking) and
//! `embedded-hal-async` v1 (async) via the `async` cargo feature.
//!
//! Supported devices (shared logic, same register map for Essential):
//! - TMP102
//! - TMP112 family (TMP112A / TMP112B / TMP112N)
//!
//! I2C addresses (ADD0 pin):
//! - `0x48` ADD0 to GND
//! - `0x49` ADD0 to VDD
//! - `0x4A` ADD0 to SDA
//! - `0x4B` ADD0 to SCL
//!
//! ## Essential API (this stage)
//!
//! ```no_run
//! use tmp102::{SlaveAddr, Tmp102};
//! # #[cfg(not(feature = "async"))]
//! # {
//! # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
//! # let i2c = I2cMock::new(&[I2cTrans::write_read(0x48, vec![0x00], vec![0x19, 0x00])]);
//! let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
//! let temp: f32 = sensor.read_temperature().unwrap();
//! sensor.set_conversion_mode(tmp102::ConversionMode::Shutdown).unwrap();
//! sensor.set_conversion_mode(tmp102::ConversionMode::Continuous).unwrap();
//! # sensor.destroy().done();
//! # }
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

mod desirable;
mod errors;
mod essential;
mod rare;
mod register;
mod tmp102;
mod types;

pub use errors::Error;
pub use tmp102::{Tmp102, Tmp112, Tmp1x2};
pub use types::{
    AlertPolarity, ConversionMode, ConversionRate, FaultQueue, SlaveAddr, ThermostatMode,
};
