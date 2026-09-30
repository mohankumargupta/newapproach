//! TMP102 / TMP112 device structs and shared low-level I2C logic.
//!
//! The TMP102 and the TMP112 family (TMP112A/B/N) are register-compatible:
//! all bus interaction lives in the shared [`Core`] below, while the
//! public [`Tmp102`] / [`Tmp112`] types carry the per-device identity
//! (naming, datasheet accuracy specs, constructors).
//!
//! High-level behaviour (the Essential API) is implemented in
//! [`crate::essential`] for each device type.

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

use crate::errors::Error;
use crate::register::Register;
use crate::types::SlaveAddr;

/// Shared bus logic for all supported devices.
///
/// Holds the I2C bus handle and the resolved 7-bit address, plus the
/// read/write primitives every device type builds on.
#[derive(Debug)]
pub(crate) struct Core<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Core<I2C> {
    pub(crate) fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Core {
            i2c,
            address: address.addr(),
        }
    }

    pub(crate) fn destroy(self) -> I2C {
        self.i2c
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Core",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Core<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Read two bytes (MSB first) from a register.
    pub(crate) async fn read_register_u16(&mut self, register: u8) -> Result<[u8; 2], Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .await
            .map_err(Error::I2C)?;
        Ok(data)
    }

    /// Write two bytes (MSB first) to a register.
    pub(crate) async fn write_register(
        &mut self,
        register: u8,
        msb: u8,
        lsb: u8,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, msb, lsb])
            .await
            .map_err(Error::I2C)
    }

    /// Read the raw configuration register without interpreting it.
    pub(crate) async fn read_config_raw(&mut self) -> Result<[u8; 2], Error<E>> {
        self.read_register_u16(Register::CONFIG).await
    }
}

/// Driver for the TMP102 digital temperature sensor.
///
/// Accuracy: ±0.5°C (max) over the operating range without calibration.
/// See the TMP102 datasheet for details.
#[derive(Debug)]
pub struct Tmp102<I2C> {
    pub(crate) core: Core<I2C>,
}

impl<I2C> Tmp102<I2C> {
    /// Create a new driver instance in continuous conversion mode.
    ///
    /// The driver does not touch the device configuration; it performs a
    /// read-modify-write of the configuration register whenever the mode
    /// is changed, so pre-existing device state is preserved.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp102 {
            core: Core::new(i2c, address),
        }
    }

    /// Destroy the driver and return the I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.core.destroy()
    }
}

/// Driver for the TMP112 family of digital temperature sensors
/// (TMP112A, TMP112B, TMP112N).
///
/// Register-compatible with the TMP102; the family members differ mainly
/// in accuracy and supply-voltage optimisation:
/// TMP112A/B offer 0.5°C accuracy, TMP112N offers 1°C accuracy.
/// See the TMP112x datasheet for details.
#[derive(Debug)]
pub struct Tmp112<I2C> {
    pub(crate) core: Core<I2C>,
}

impl<I2C> Tmp112<I2C> {
    /// Create a new driver instance in continuous conversion mode.
    ///
    /// Behaviour is identical to [`Tmp102::new`]; only the device identity
    /// differs.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Tmp112 {
            core: Core::new(i2c, address),
        }
    }

    /// Destroy the driver and return the I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.core.destroy()
    }
}
