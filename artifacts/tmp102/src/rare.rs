//! Rare functionality (plan items #7–#11).
//!
//! - extended measurement mode (up to 150 C)
//! - fault queue, alert polarity, thermostat mode
//! - comparator-mode alert status

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{BitFlagsHigh, BitFlagsLow, Config, Register};
use crate::tmp102::Tmp1x2;
use crate::types::{AlertPolarity, FaultQueue, ThermostatMode};

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
    /// Enable the extended measurement mode.
    ///
    /// Allows measuring temperatures above 128 C (up to 150 C).
    pub async fn enable_extended_measurement_mode(&mut self) -> Result<(), Error<E>> {
        self.write_config(self.config.with_high_lsb(BitFlagsLow::EXTENDED_MODE))
            .await
    }

    /// Disable the extended measurement mode (back to normal 12-bit mode).
    pub async fn disable_extended_measurement_mode(&mut self) -> Result<(), Error<E>> {
        self.write_config(self.config.with_low_lsb(BitFlagsLow::EXTENDED_MODE))
            .await
    }

    /// Enable the extended measurement mode (alias of
    /// [`enable_extended_measurement_mode`](Self::enable_extended_measurement_mode),
    /// matching the `tmp1x2` reference driver naming).
    pub async fn enable_extended_mode(&mut self) -> Result<(), Error<E>> {
        self.enable_extended_measurement_mode().await
    }

    /// Disable the extended measurement mode (alias of
    /// [`disable_extended_measurement_mode`](Self::disable_extended_measurement_mode)).
    pub async fn disable_extended_mode(&mut self) -> Result<(), Error<E>> {
        self.disable_extended_measurement_mode().await
    }

    /// Set the fault queue (consecutive faults needed to trigger an alert).
    pub async fn set_fault_queue(&mut self, fq: FaultQueue) -> Result<(), Error<E>> {
        let Config { msb, lsb } = self.config;
        match fq {
            FaultQueue::_1 => {
                self.write_config(Config {
                    msb: msb & !BitFlagsHigh::FAULT_QUEUE1 & !BitFlagsHigh::FAULT_QUEUE0,
                    lsb,
                })
                .await
            }
            FaultQueue::_2 => {
                self.write_config(Config {
                    msb: msb & !BitFlagsHigh::FAULT_QUEUE1 | BitFlagsHigh::FAULT_QUEUE0,
                    lsb,
                })
                .await
            }
            FaultQueue::_4 => {
                self.write_config(Config {
                    msb: msb | BitFlagsHigh::FAULT_QUEUE1 & !BitFlagsHigh::FAULT_QUEUE0,
                    lsb,
                })
                .await
            }
            FaultQueue::_6 => {
                self.write_config(Config {
                    msb: msb | BitFlagsHigh::FAULT_QUEUE1 | BitFlagsHigh::FAULT_QUEUE0,
                    lsb,
                })
                .await
            }
        }
    }

    /// Set the alert (ALERT pin) polarity.
    pub async fn set_alert_polarity(&mut self, polarity: AlertPolarity) -> Result<(), Error<E>> {
        match polarity {
            AlertPolarity::ActiveLow => {
                self.write_config(self.config.with_low_msb(BitFlagsHigh::ALERT_POLARITY))
                    .await
            }
            AlertPolarity::ActiveHigh => {
                self.write_config(self.config.with_high_msb(BitFlagsHigh::ALERT_POLARITY))
                    .await
            }
        }
    }

    /// Set the thermostat mode (comparator vs interrupt ALERT behaviour).
    pub async fn set_thermostat_mode(&mut self, mode: ThermostatMode) -> Result<(), Error<E>> {
        match mode {
            ThermostatMode::Comparator => {
                self.write_config(self.config.with_low_msb(BitFlagsHigh::THERMOSTAT))
                    .await
            }
            ThermostatMode::Interrupt => {
                self.write_config(self.config.with_high_msb(BitFlagsHigh::THERMOSTAT))
                    .await
            }
        }
    }

    /// Read whether an alert is active as defined by the comparator mode.
    ///
    /// Note: this ignores the thermostat mode setting and always reports
    /// the comparator-mode status. Takes the selected alert polarity into
    /// account.
    #[allow(clippy::wrong_self_convention)]
    pub async fn is_comparator_mode_alert_active(&mut self) -> Result<bool, Error<E>> {
        let config = self.read_register_u16(Register::CONFIG).await?;
        let is_polarity_high = (config.msb & BitFlagsHigh::ALERT_POLARITY) != 0;
        let alert_status = (config.lsb & BitFlagsLow::ALERT) != 0;
        Ok(is_polarity_high == alert_status)
    }
}
