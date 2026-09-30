//! Desirable functionality: one-shot control, conversion rate, thresholds.
//!
//! This is the complete **Desirable** category from the device plan — the
//! common configurations and optimisations:
//!
//! - Triggering a one-shot measurement in shutdown mode.
//! - Polling one-shot readiness.
//! - Setting the conversion rate.
//! - Setting the high/low temperature thresholds.

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{BitFlagsHigh, BitFlagsLow, Register};
use crate::tmp102::{Tmp102, Tmp112};
use crate::types::ConversionRate;

/// Convert Celsius to threshold-register bytes in normal (12-bit) mode.
///
/// Clamped to `[-128.0, 127.9375]`; LSB weight 0.0625 °C.
pub(crate) fn convert_temp_to_register_normal(mut temp: f32) -> (u8, u8) {
    if temp > 127.9375 {
        temp = 127.9375;
    }
    if temp < -128.0 {
        temp = -128.0;
    }
    let value = (temp / 0.0625) as i16;
    let value = value << 4;
    ((value >> 8) as u8, (value as u8 & 0b1111_0000))
}

/// Convert Celsius to threshold-register bytes in extended (13-bit) mode.
///
/// Clamped to `[-256.0, 255.875]`; LSB weight 0.0625 °C.
pub(crate) fn convert_temp_to_register_extended(mut temp: f32) -> (u8, u8) {
    if temp > 255.875 {
        temp = 255.875;
    }
    if temp < -256.0 {
        temp = -256.0;
    }
    let value = (temp / 0.0625) as i16;
    let value = value << 3;
    ((value >> 8) as u8, (value as u8 & 0b1111_1000))
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
    /// Trigger a single temperature conversion while in shutdown mode.
    ///
    /// The device returns to shutdown once the conversion completes.
    /// Poll [`is_one_shot_measurement_result_ready`](Self::is_one_shot_measurement_result_ready)
    /// before reading the result with `read_temperature`.
    pub async fn trigger_one_shot_measurement(&mut self) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        self.core
            .write_register(Register::CONFIG, msb | BitFlagsHigh::ONE_SHOT, lsb)
            .await
    }

    /// Read whether the one-shot measurement result is ready.
    ///
    /// Returns `true` once the conversion triggered by
    /// [`trigger_one_shot_measurement`](Self::trigger_one_shot_measurement)
    /// has completed.
    pub async fn is_one_shot_measurement_result_ready(&mut self) -> Result<bool, Error<E>> {
        let [msb, _] = self.core.read_config_raw().await?;
        Ok((msb & BitFlagsHigh::ONE_SHOT) != 0)
    }

    /// Set the conversion rate used in continuous conversion mode.
    ///
    /// Performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn set_conversion_rate(&mut self, rate: ConversionRate) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let lsb = match rate {
            ConversionRate::Hz0_25 => lsb & !BitFlagsLow::CONV_RATE1 & !BitFlagsLow::CONV_RATE0,
            ConversionRate::Hz1 => lsb & !BitFlagsLow::CONV_RATE1 | BitFlagsLow::CONV_RATE0,
            ConversionRate::Hz4 => lsb | BitFlagsLow::CONV_RATE1 & !BitFlagsLow::CONV_RATE0,
            ConversionRate::Hz8 => lsb | BitFlagsLow::CONV_RATE1 | BitFlagsLow::CONV_RATE0,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Set the low temperature threshold in degrees Celsius.
    ///
    /// Uses 12-bit encoding normally, or 13-bit encoding when extended
    /// measurement mode is currently enabled (see Rare API). Values are
    /// clamped to the active range.
    pub async fn set_low_temperature_threshold(&mut self, temp_c: f32) -> Result<(), Error<E>> {
        self.write_temperature_threshold(temp_c, Register::T_LOW)
            .await
    }

    /// Set the high temperature threshold in degrees Celsius.
    ///
    /// Encoding and clamping behave as in
    /// [`set_low_temperature_threshold`](Self::set_low_temperature_threshold).
    pub async fn set_high_temperature_threshold(&mut self, temp_c: f32) -> Result<(), Error<E>> {
        self.write_temperature_threshold(temp_c, Register::T_HIGH)
            .await
    }

    async fn write_temperature_threshold(
        &mut self,
        temp_c: f32,
        register: u8,
    ) -> Result<(), Error<E>> {
        let [_, lsb_cfg] = self.core.read_config_raw().await?;
        let extended = (lsb_cfg & BitFlagsLow::EXTENDED_MODE) != 0;
        let (msb, lsb) = if extended {
            convert_temp_to_register_extended(temp_c)
        } else {
            convert_temp_to_register_normal(temp_c)
        };
        self.core.write_register(register, msb, lsb).await
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
    /// Trigger a single temperature conversion while in shutdown mode.
    ///
    /// Same behaviour as [`Tmp102::trigger_one_shot_measurement`].
    pub async fn trigger_one_shot_measurement(&mut self) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        self.core
            .write_register(Register::CONFIG, msb | BitFlagsHigh::ONE_SHOT, lsb)
            .await
    }

    /// Read whether the one-shot measurement result is ready.
    ///
    /// Same behaviour as [`Tmp102::is_one_shot_measurement_result_ready`].
    pub async fn is_one_shot_measurement_result_ready(&mut self) -> Result<bool, Error<E>> {
        let [msb, _] = self.core.read_config_raw().await?;
        Ok((msb & BitFlagsHigh::ONE_SHOT) != 0)
    }

    /// Set the conversion rate used in continuous conversion mode.
    ///
    /// Same behaviour as [`Tmp102::set_conversion_rate`].
    pub async fn set_conversion_rate(&mut self, rate: ConversionRate) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let lsb = match rate {
            ConversionRate::Hz0_25 => lsb & !BitFlagsLow::CONV_RATE1 & !BitFlagsLow::CONV_RATE0,
            ConversionRate::Hz1 => lsb & !BitFlagsLow::CONV_RATE1 | BitFlagsLow::CONV_RATE0,
            ConversionRate::Hz4 => lsb | BitFlagsLow::CONV_RATE1 & !BitFlagsLow::CONV_RATE0,
            ConversionRate::Hz8 => lsb | BitFlagsLow::CONV_RATE1 | BitFlagsLow::CONV_RATE0,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Set the low temperature threshold in degrees Celsius.
    ///
    /// Same behaviour as [`Tmp102::set_low_temperature_threshold`].
    pub async fn set_low_temperature_threshold(&mut self, temp_c: f32) -> Result<(), Error<E>> {
        self.write_temperature_threshold(temp_c, Register::T_LOW)
            .await
    }

    /// Set the high temperature threshold in degrees Celsius.
    ///
    /// Same behaviour as [`Tmp102::set_high_temperature_threshold`].
    pub async fn set_high_temperature_threshold(&mut self, temp_c: f32) -> Result<(), Error<E>> {
        self.write_temperature_threshold(temp_c, Register::T_HIGH)
            .await
    }

    async fn write_temperature_threshold(
        &mut self,
        temp_c: f32,
        register: u8,
    ) -> Result<(), Error<E>> {
        let [_, lsb_cfg] = self.core.read_config_raw().await?;
        let extended = (lsb_cfg & BitFlagsLow::EXTENDED_MODE) != 0;
        let (msb, lsb) = if extended {
            convert_temp_to_register_extended(temp_c)
        } else {
            convert_temp_to_register_normal(temp_c)
        };
        self.core.write_register(register, msb, lsb).await
    }
}

#[cfg(test)]
mod tests {
    use super::{
        convert_temp_to_register_extended as to_ext, convert_temp_to_register_normal as to_reg,
    };

    #[test]
    fn threshold_conversion_normal_is_clamped() {
        assert_eq!((0b0111_1111, 0b1111_0000), to_reg(129.0));
        assert_eq!((0b1000_0000, 0b0000_0000), to_reg(-129.0));
    }

    #[test]
    fn threshold_conversion_extended_is_clamped() {
        assert_eq!((0b0111_1111, 0b1111_0000), to_ext(256.0));
        assert_eq!((0b1000_0000, 0b0000_0000), to_ext(-257.0));
    }

    #[test]
    fn threshold_conversion_normal_values() {
        assert_eq!((0b0001_1001, 0b0000_0000), to_reg(25.0));
        assert_eq!((0b0101_0000, 0b0000_0000), to_reg(80.0));
        assert_eq!((0b0000_0000, 0b0000_0000), to_reg(0.0));
        assert_eq!((0b1110_0111, 0b0000_0000), to_reg(-25.0));
    }
}
