//! Transport error wrapper.

/// Possible errors returned by this driver.
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error (preserves underlying HAL error).
    I2c(E),
}

impl<E: core::fmt::Debug> core::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::I2c(_) => write!(f, "I2C bus error"),
        }
    }
}
