//! Essential functionality: read temperature + conversion mode.
//!
//! Covers plan items #1 (read temperature) and #2 (shutdown vs continuous).

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{BitFlagsHigh, Register};
use crate::tmp102::{convert_temp_from_register, Tmp1x2};
use crate::types::ConversionMode;

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
    /// Read the current temperature in degrees Celsius.
    ///
    /// Works in both continuous and shutdown modes; in shutdown mode it
    /// returns the last converted value (see Desirable one-shot API for
    /// on-demand conversions).
    pub async fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let data = self.read_register_u16(Register::TEMPERATURE).await?;
        Ok(convert_temp_from_register(data.msb, data.lsb))
    }

    /// Set the conversion mode: continuous (active) or shutdown (sleep).
    ///
    /// ```no_run
    /// # #[cfg(not(feature = "async"))]
    /// # {
    /// # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
    /// # use tmp102::{ConversionMode, SlaveAddr, Tmp102};
    /// # let i2c = I2cMock::new(&[I2cTrans::write(0x48, vec![0x01, 0x60 | 0x01, 0xA0])]);
    /// # let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
    /// sensor.set_conversion_mode(ConversionMode::Shutdown).unwrap(); // Sleep
    /// # sensor.destroy().done();
    /// # }
    /// ```
    pub async fn set_conversion_mode(&mut self, mode: ConversionMode) -> Result<(), Error<E>> {
        match mode {
            ConversionMode::Shutdown => {
                self.write_config(self.config.with_high_msb(BitFlagsHigh::SHUTDOWN))
                    .await
            }
            ConversionMode::Continuous => {
                self.write_config(self.config.with_low_msb(BitFlagsHigh::SHUTDOWN))
                    .await
            }
        }
    }
}
