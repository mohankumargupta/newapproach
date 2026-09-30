//! Error types.

/// All errors returned by this driver.
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error.
    I2C(E),
}
