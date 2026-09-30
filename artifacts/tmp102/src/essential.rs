//! Essential functionality: read temperature, set conversion mode.
//!
//! This is the complete **Essential** category from the device plan — the
//! core daily operations. Everything else (one-shot triggering, conversion
//! rate, thresholds, extended mode, fault queue, alert polarity,
//! thermostat mode) is Desirable / Rare and out of scope.

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{BitFlagsHigh, Register};
use crate::tmp102::{Tmp102, Tmp112};
use crate::types::ConversionMode;

/// Convert a raw temperature-register reading to Celsius.
///
/// Handles both normal (12-bit) and extended (13-bit) measurement modes;
/// the mode is signalled by bit 0 of the LSB. Values are two's complement
/// with an LSB weight of 0.0625°C.
pub(crate) fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    let sign = (u16::from(msb & 0b1000_0000)) << 8;
    let extended_mode = (lsb & 1) != 0;
    if extended_mode {
        let sign = if sign != 0 {
            sign | 0b1111_0000 << 8
        } else {
            0
        };
        let value = sign | (u16::from(msb & 0b0111_1111) << 5) | u16::from(lsb >> 3);
        f32::from(value as i16) * 0.0625
    } else {
        let sign = if sign != 0 {
            sign | 0b1111_1000 << 8
        } else {
            0
        };
        let value = sign | (u16::from(msb & 0b0111_1111) << 4) | u16::from(lsb >> 4);
        f32::from(value as i16) * 0.0625
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp102",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Tmp102<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Read the current temperature in degrees Celsius.
    ///
    /// In continuous mode this returns the latest conversion; in shutdown
    /// mode it returns the result of the most recent conversion.
    pub async fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let [msb, lsb] = self.core.read_register_u16(Register::TEMPERATURE).await?;
        Ok(convert_temp_from_register(msb, lsb))
    }

    /// Switch between continuous and shutdown (one-shot) conversion mode.
    ///
    /// This performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn set_conversion_mode(&mut self, mode: ConversionMode) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match mode {
            ConversionMode::Shutdown => msb | BitFlagsHigh::SHUTDOWN,
            ConversionMode::Continuous => msb & !BitFlagsHigh::SHUTDOWN,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp112",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Tmp112<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Read the current temperature in degrees Celsius.
    ///
    /// Same behaviour as [`Tmp102::read_temperature`].
    pub async fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let [msb, lsb] = self.core.read_register_u16(Register::TEMPERATURE).await?;
        Ok(convert_temp_from_register(msb, lsb))
    }

    /// Switch between continuous and shutdown (one-shot) conversion mode.
    ///
    /// Same behaviour as [`Tmp102::set_conversion_mode`].
    pub async fn set_conversion_mode(&mut self, mode: ConversionMode) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match mode {
            ConversionMode::Shutdown => msb | BitFlagsHigh::SHUTDOWN,
            ConversionMode::Continuous => msb & !BitFlagsHigh::SHUTDOWN,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }
}

#[cfg(test)]
mod tests {
    use super::convert_temp_from_register as convert;

    fn assert_near(left: f32, right: f32) {
        assert!((left - right).abs() < f32::EPSILON, "{left} != {right}");
    }

    #[test]
    fn converts_normal_mode_values() {
        assert_near(25.0, convert(0b0001_1001, 0b0000_0000));
        assert_near(100.0, convert(0b0110_0100, 0b0000_0000));
        assert_near(0.0, convert(0b0000_0000, 0b0000_0000));
        assert_near(-25.0, convert(0b1110_0111, 0b0000_0000));
        assert_near(-55.0, convert(0b1100_1001, 0b0000_0000));
    }

    #[test]
    fn converts_extended_mode_values() {
        assert_near(150.0, convert(0b0100_1011, 0b0000_0001));
        assert_near(25.0, convert(0b0000_1100, 0b1000_0001));
    }
}
