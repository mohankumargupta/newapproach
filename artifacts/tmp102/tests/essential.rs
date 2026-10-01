//! Minimal integration tests for Essential functionality (blocking + async).
//!
//! Run with `maybe-async-cfg`: sync by default, async under `--features async`.

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use tmp102::{ConversionMode, SlaveAddr, Tmp102};

const ADDR: u8 = 0x48;
// Power-up default config 0x60A0.
const CFG_MSB: u8 = 0x60;
const CFG_LSB: u8 = 0xA0;

fn sensor(expectations: &[I2cTrans]) -> Tmp102<I2cMock> {
    Tmp102::new(I2cMock::new(expectations), SlaveAddr::default())
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn reads_100_celsius() {
    let expectations = [I2cTrans::write_read(
        ADDR,
        vec![0x00],
        vec![0b0110_0100, 0x00],
    )];
    let mut dev = sensor(&expectations);
    let temp = dev.read_temperature().await.unwrap();
    assert!((temp - 100.0).abs() < f32::EPSILON);
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn reads_negative_temperature() {
    // -25 C per datasheet vectors.
    let expectations = [I2cTrans::write_read(
        ADDR,
        vec![0x00],
        vec![0b1110_0111, 0x00],
    )];
    let mut dev = sensor(&expectations);
    let temp = dev.read_temperature().await.unwrap();
    assert!((temp + 25.0).abs() < f32::EPSILON);
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn shutdown_sets_sd_bit() {
    let expectations = [I2cTrans::write(ADDR, vec![0x01, CFG_MSB | 0x01, CFG_LSB])];
    let mut dev = sensor(&expectations);
    dev.set_conversion_mode(ConversionMode::Shutdown)
        .await
        .unwrap();
    assert_eq!(0x48, dev.address());
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn continuous_clears_sd_bit() {
    let expectations = [
        I2cTrans::write(ADDR, vec![0x01, CFG_MSB | 0x01, CFG_LSB]),
        I2cTrans::write(ADDR, vec![0x01, CFG_MSB & !0x01, CFG_LSB]),
    ];
    let mut dev = sensor(&expectations);
    dev.set_conversion_mode(ConversionMode::Shutdown)
        .await
        .unwrap();
    dev.set_conversion_mode(ConversionMode::Continuous)
        .await
        .unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn alternative_address_used() {
    let expectations = [I2cTrans::write_read(0x49, vec![0x00], vec![0x19, 0x00])];
    let i2c = I2cMock::new(&expectations);
    let mut dev = Tmp102::new(i2c, SlaveAddr::Alternative(false, true));
    assert_eq!(0x49, dev.address());
    let _ = dev.read_temperature().await.unwrap();
    dev.destroy().done();
}
