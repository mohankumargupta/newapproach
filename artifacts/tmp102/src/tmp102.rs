//! Core device struct, I2C plumbing and shared temperature conversion.
//!
//! Shared logic for TMP102 / TMP112 lives here. Device-specific aliases
//! (`Tmp102`, `Tmp112`) are thin wrappers over the same [`Tmp1x2`] core
//! because the Essential register map is identical.

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{Config, Register, RegisterU16};
use crate::types::SlaveAddr;

/// Base I2C address (ADD0 to GND).
pub const DEVICE_BASE_ADDRESS: u8 = 0x48;

/// Generic TMP1x2 driver over any `embedded-hal` I2C implementation.
#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
#[derive(Debug)]
pub struct Tmp1x2<I2C> {
    /// Concrete I2C bus.
    i2c: I2C,
    /// 7-bit I2C address.
    address: u8,
    /// Cached copy of the configuration register.
    pub(crate) config: Config,
}

/// TMP102 device (shared logic, see [`Tmp1x2`]).
pub type Tmp102<I2C> = Tmp1x2<I2C>;

/// TMP112 device / family (TMP112A/B/N share the Essential register map).
pub type Tmp112<I2C> = Tmp1x2<I2C>;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Tmp1x2<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Create a new driver instance. Device powers up in continuous mode.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp1x2 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: Config::default(),
        }
    }

    /// Destroy the driver and return the underlying I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Return the resolved 7-bit I2C address.
    pub fn address(&self) -> u8 {
        self.address
    }

    pub(crate) async fn write_config(&mut self, data: Config) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, data).await?;
        self.config = data;
        Ok(())
    }

    pub(crate) async fn write_register(
        &mut self,
        register: u8,
        data: RegisterU16,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, data.msb, data.lsb])
            .await
            .map_err(Error::I2c)
    }

    pub(crate) async fn read_register_u16(
        &mut self,
        register: u8,
    ) -> Result<RegisterU16, Error<E>> {
        let mut data = [0u8; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .await
            .map_err(Error::I2c)?;
        Ok(RegisterU16 {
            msb: data[0],
            lsb: data[1],
        })
    }
}

/// Convert raw temperature register bytes to Celsius.
///
/// Handles both normal (12-bit) and extended (13-bit) modes; the mode is
/// encoded in bit0 of the low byte as returned by the device.
pub(crate) fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    let mut sign = (u16::from(msb & 0b1000_0000)) << 8;
    let extended_mode = (lsb & 1) != 0;
    if extended_mode {
        if sign != 0 {
            sign |= 0b1111_0000 << 8;
        }
        let msb = u16::from(msb & 0b0111_1111);
        let value = sign | (msb << 5) | u16::from(lsb >> 3);
        f32::from(value as i16) * 0.0625
    } else {
        if sign != 0 {
            sign |= 0b1111_1000 << 8;
        }
        let msb = u16::from(msb & 0b0111_1111);
        let value = sign | (msb << 4) | u16::from(lsb >> 4);
        f32::from(value as i16) * 0.0625
    }
}

/// Convert Celsius to raw threshold register bytes (normal 12-bit mode).
///
/// Clamps to `[-128.0, 127.9375]`.
#[allow(clippy::manual_clamp)]
pub(crate) fn convert_temp_to_register_normal(mut t: f32) -> (u8, u8) {
    if t > 127.9375 {
        t = 127.9375;
    }
    if t < -128.0 {
        t = -128.0;
    }
    let value = t / 0.0625;
    let value = (value as i16) << 4;
    ((value >> 8) as u8, (value as u8 & 0b1111_0000))
}

/// Convert Celsius to raw threshold register bytes (extended 13-bit mode).
///
/// Clamps to `[-256.0, 255.875]`.
#[allow(clippy::manual_clamp)]
pub(crate) fn convert_temp_to_register_extended(mut t: f32) -> (u8, u8) {
    if t > 255.875 {
        t = 255.875;
    }
    if t < -256.0 {
        t = -256.0;
    }
    let value = t / 0.0625;
    let value = (value as i16) << 3;
    ((value >> 8) as u8, (value as u8 & 0b1111_1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_resolve() {
        assert_eq!(0x48, SlaveAddr::Default.addr(DEVICE_BASE_ADDRESS));
        assert_eq!(
            0x49,
            SlaveAddr::Alternative(false, true).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0x4A,
            SlaveAddr::Alternative(true, false).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0x4B,
            SlaveAddr::Alternative(true, true).addr(DEVICE_BASE_ADDRESS)
        );
    }

    #[test]
    fn converts_known_temperatures() {
        // Normal mode vectors from datasheet / reference driver.
        assert!((convert_temp_from_register(0b0110_0100, 0) - 100.0).abs() < f32::EPSILON);
        assert!((convert_temp_from_register(0, 0) - 0.0).abs() < f32::EPSILON);
        assert!((convert_temp_from_register(0b1000_0000, 0) + 128.0).abs() < f32::EPSILON);
    }
}
