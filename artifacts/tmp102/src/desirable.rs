//! Desirable functionality (plan items #3–#6).
//!
//! - trigger one-shot measurement
//! - one-shot ready flag
//! - conversion rate
//! - high/low temperature thresholds

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{BitFlagsHigh, BitFlagsLow, Config, Register, RegisterU16};
use crate::tmp102::{convert_temp_to_register_extended, convert_temp_to_register_normal, Tmp1x2};
use crate::types::ConversionRate;

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
    /// Trigger a one-shot measurement.
    ///
    /// Use in shutdown mode for on-demand conversions (low power).
    /// The OS bit is sent on the wire but not cached in `self.config`,
    /// matching the reference `tmp1x2` driver.
    pub async fn trigger_one_shot_measurement(&mut self) -> Result<(), Error<E>> {
        self.write_register(
            Register::CONFIG,
            self.config.with_high_msb(BitFlagsHigh::ONE_SHOT),
        )
        .await
    }

    /// Read whether the one-shot measurement result is ready.
    ///
    /// Returns `true` once the OS flag reads back set, meaning the
    /// conversion result can be fetched with [`read_temperature`](crate::Tmp1x2::read_temperature).
    pub async fn is_one_shot_measurement_result_ready(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register_u16(Register::CONFIG).await?;
        Ok((config.msb & BitFlagsHigh::ONE_SHOT) != 0)
    }

    /// Set the conversion rate for continuous conversion mode.
    pub async fn set_conversion_rate(&mut self, rate: ConversionRate) -> Result<(), Error<E>> {
        let Config { msb, lsb } = self.config;
        match rate {
            ConversionRate::_0_25Hz => {
                self.write_config(Config {
                    msb,
                    lsb: lsb & !BitFlagsLow::CONV_RATE1 & !BitFlagsLow::CONV_RATE0,
                })
                .await
            }
            ConversionRate::_1Hz => {
                self.write_config(Config {
                    msb,
                    lsb: lsb & !BitFlagsLow::CONV_RATE1 | BitFlagsLow::CONV_RATE0,
                })
                .await
            }
            ConversionRate::_4Hz => {
                self.write_config(Config {
                    msb,
                    lsb: lsb | BitFlagsLow::CONV_RATE1 & !BitFlagsLow::CONV_RATE0,
                })
                .await
            }
            ConversionRate::_8Hz => {
                self.write_config(Config {
                    msb,
                    lsb: lsb | BitFlagsLow::CONV_RATE1 | BitFlagsLow::CONV_RATE0,
                })
                .await
            }
        }
    }

    /// Set the high temperature threshold in degrees Celsius.
    ///
    /// Clamped to `[-128.0, 127.9375]` in normal mode and
    /// `[-256.0, 255.875]` in extended mode (per cached config).
    pub async fn set_high_temperature_threshold(
        &mut self,
        temperature: f32,
    ) -> Result<(), Error<E>> {
        self.set_temperature_threshold(temperature, Register::T_HIGH)
            .await
    }

    /// Set the low temperature threshold in degrees Celsius.
    ///
    /// Clamped to `[-128.0, 127.9375]` in normal mode and
    /// `[-256.0, 255.875]` in extended mode (per cached config).
    pub async fn set_low_temperature_threshold(
        &mut self,
        temperature: f32,
    ) -> Result<(), Error<E>> {
        self.set_temperature_threshold(temperature, Register::T_LOW)
            .await
    }

    async fn set_temperature_threshold(
        &mut self,
        temperature: f32,
        register: u8,
    ) -> Result<(), Error<E>> {
        let (msb, lsb) = if (self.config.lsb & BitFlagsLow::EXTENDED_MODE) != 0 {
            convert_temp_to_register_extended(temperature)
        } else {
            convert_temp_to_register_normal(temperature)
        };
        self.write_register(register, RegisterU16 { msb, lsb })
            .await
    }
}
