//! Rare functionality: extended mode, fault queue, alert polarity,
//! thermostat mode, comparator-mode alert status.
//!
//! This is the complete **Rare** category from the device plan —
//! specialized hardware tweaks and hardware status:
//!
//! - Enabling/disabling extended measurement mode (up to 150 °C).
//! - Setting the fault queue (debouncing transient spikes).
//! - Setting the alert polarity (matching MCU interrupt specs).
//! - Setting the thermostat mode (comparator vs interrupt).
//! - Reading comparator-mode alert status.

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::{BitFlagsHigh, BitFlagsLow, Register};
use crate::tmp102::{Tmp102, Tmp112};
use crate::types::{AlertPolarity, FaultQueue, ThermostatMode};

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
    /// Enable extended measurement mode (13-bit, up to 150 °C).
    ///
    /// Performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn enable_extended_measurement_mode(&mut self) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        self.core
            .write_register(Register::CONFIG, msb, lsb | BitFlagsLow::EXTENDED_MODE)
            .await
    }

    /// Disable extended measurement mode (back to 12-bit normal mode).
    ///
    /// Performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn disable_extended_measurement_mode(&mut self) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        self.core
            .write_register(Register::CONFIG, msb, lsb & !BitFlagsLow::EXTENDED_MODE)
            .await
    }

    /// Set the fault queue (consecutive faults needed to trigger an alert).
    ///
    /// Performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn set_fault_queue(&mut self, queue: FaultQueue) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match queue {
            FaultQueue::Consecutive1 => {
                msb & !BitFlagsHigh::FAULT_QUEUE1 & !BitFlagsHigh::FAULT_QUEUE0
            }
            FaultQueue::Consecutive2 => {
                (msb & !BitFlagsHigh::FAULT_QUEUE1) | BitFlagsHigh::FAULT_QUEUE0
            }
            FaultQueue::Consecutive4 => {
                (msb | BitFlagsHigh::FAULT_QUEUE1) & !BitFlagsHigh::FAULT_QUEUE0
            }
            FaultQueue::Consecutive6 => {
                msb | BitFlagsHigh::FAULT_QUEUE1 | BitFlagsHigh::FAULT_QUEUE0
            }
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Set the alert polarity (active high vs active low).
    ///
    /// Performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn set_alert_polarity(&mut self, polarity: AlertPolarity) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match polarity {
            AlertPolarity::ActiveLow => msb & !BitFlagsHigh::ALERT_POLARITY,
            AlertPolarity::ActiveHigh => msb | BitFlagsHigh::ALERT_POLARITY,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Set the thermostat mode (comparator vs interrupt).
    ///
    /// Performs a read-modify-write of the configuration register so
    /// all other device settings are preserved.
    pub async fn set_thermostat_mode(&mut self, mode: ThermostatMode) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match mode {
            ThermostatMode::Comparator => msb & !BitFlagsHigh::THERMOSTAT,
            ThermostatMode::Interrupt => msb | BitFlagsHigh::THERMOSTAT,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Read whether a comparator-mode alert is active.
    ///
    /// This ignores the thermostat-mode setting and always reports the
    /// comparator-mode status, taking the selected alert polarity into
    /// account (polarity-high == alert-flag means active).
    pub async fn is_comparator_mode_alert_active(&mut self) -> Result<bool, Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let polarity_high = (msb & BitFlagsHigh::ALERT_POLARITY) != 0;
        let alert_flag = (lsb & BitFlagsLow::ALERT) != 0;
        Ok(polarity_high == alert_flag)
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
    /// Enable extended measurement mode (13-bit, up to 150 °C).
    ///
    /// Same behaviour as [`Tmp102::enable_extended_measurement_mode`].
    pub async fn enable_extended_measurement_mode(&mut self) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        self.core
            .write_register(Register::CONFIG, msb, lsb | BitFlagsLow::EXTENDED_MODE)
            .await
    }

    /// Disable extended measurement mode (back to 12-bit normal mode).
    ///
    /// Same behaviour as [`Tmp102::disable_extended_measurement_mode`].
    pub async fn disable_extended_measurement_mode(&mut self) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        self.core
            .write_register(Register::CONFIG, msb, lsb & !BitFlagsLow::EXTENDED_MODE)
            .await
    }

    /// Set the fault queue (consecutive faults needed to trigger an alert).
    ///
    /// Same behaviour as [`Tmp102::set_fault_queue`].
    pub async fn set_fault_queue(&mut self, queue: FaultQueue) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match queue {
            FaultQueue::Consecutive1 => {
                msb & !BitFlagsHigh::FAULT_QUEUE1 & !BitFlagsHigh::FAULT_QUEUE0
            }
            FaultQueue::Consecutive2 => {
                (msb & !BitFlagsHigh::FAULT_QUEUE1) | BitFlagsHigh::FAULT_QUEUE0
            }
            FaultQueue::Consecutive4 => {
                (msb | BitFlagsHigh::FAULT_QUEUE1) & !BitFlagsHigh::FAULT_QUEUE0
            }
            FaultQueue::Consecutive6 => {
                msb | BitFlagsHigh::FAULT_QUEUE1 | BitFlagsHigh::FAULT_QUEUE0
            }
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Set the alert polarity (active high vs active low).
    ///
    /// Same behaviour as [`Tmp102::set_alert_polarity`].
    pub async fn set_alert_polarity(&mut self, polarity: AlertPolarity) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match polarity {
            AlertPolarity::ActiveLow => msb & !BitFlagsHigh::ALERT_POLARITY,
            AlertPolarity::ActiveHigh => msb | BitFlagsHigh::ALERT_POLARITY,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Set the thermostat mode (comparator vs interrupt).
    ///
    /// Same behaviour as [`Tmp102::set_thermostat_mode`].
    pub async fn set_thermostat_mode(&mut self, mode: ThermostatMode) -> Result<(), Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let msb = match mode {
            ThermostatMode::Comparator => msb & !BitFlagsHigh::THERMOSTAT,
            ThermostatMode::Interrupt => msb | BitFlagsHigh::THERMOSTAT,
        };
        self.core.write_register(Register::CONFIG, msb, lsb).await
    }

    /// Read whether a comparator-mode alert is active.
    ///
    /// Same behaviour as [`Tmp102::is_comparator_mode_alert_active`].
    pub async fn is_comparator_mode_alert_active(&mut self) -> Result<bool, Error<E>> {
        let [msb, lsb] = self.core.read_config_raw().await?;
        let polarity_high = (msb & BitFlagsHigh::ALERT_POLARITY) != 0;
        let alert_flag = (lsb & BitFlagsLow::ALERT) != 0;
        Ok(polarity_high == alert_flag)
    }
}
