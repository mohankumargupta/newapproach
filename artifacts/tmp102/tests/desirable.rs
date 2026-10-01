//! Minimal integration tests for Desirable functionality (blocking + async).
//!
//! Covers plan items #3–#6: one-shot trigger/ready, conversion rate,
//! high/low temperature thresholds.

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use tmp102::{ConversionRate, SlaveAddr, Tmp102};

const ADDR: u8 = 0x48;
// Power-up default config 0x60A0.
const CFG_MSB: u8 = 0x60;
const CFG_LSB: u8 = 0xA0;
const CFG_OS: u8 = 0x60 | 0x80;

fn sensor(expectations: &[I2cTrans]) -> Tmp102<I2cMock> {
    Tmp102::new(I2cMock::new(expectations), SlaveAddr::default())
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn trigger_one_shot_writes_os_bit_uncached() {
    let expectations = [I2cTrans::write(ADDR, vec![0x01, CFG_OS, CFG_LSB])];
    let mut dev = sensor(&expectations);
    dev.trigger_one_shot_measurement().await.unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn one_shot_ready_flag() {
    // Not ready: OS bit clear.
    let expectations = [I2cTrans::write_read(
        ADDR,
        vec![0x01],
        vec![CFG_MSB, CFG_LSB],
    )];
    let mut dev = sensor(&expectations);
    assert!(!dev.is_one_shot_measurement_result_ready().await.unwrap());
    dev.destroy().done();

    // Ready: OS bit set.
    let expectations = [I2cTrans::write_read(
        ADDR,
        vec![0x01],
        vec![CFG_OS, CFG_LSB],
    )];
    let mut dev = sensor(&expectations);
    assert!(dev.is_one_shot_measurement_result_ready().await.unwrap());
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn set_conversion_rate_1hz() {
    // 4Hz default LSB 0xA0 -> 1Hz clears CR1, sets CR0: 0x60.
    let expectations = [I2cTrans::write(ADDR, vec![0x01, CFG_MSB, 0x60])];
    let mut dev = sensor(&expectations);
    dev.set_conversion_rate(ConversionRate::_1Hz).await.unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn set_conversion_rate_8hz() {
    // 8Hz sets both CR bits: 0xE0.
    let expectations = [I2cTrans::write(ADDR, vec![0x01, CFG_MSB, 0xE0])];
    let mut dev = sensor(&expectations);
    dev.set_conversion_rate(ConversionRate::_8Hz).await.unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn set_thresholds_normal_mode() {
    // 127.9375 C -> (0x7F, 0xF0) on T_HIGH; -0.25 C -> (0xFF, 0xC0) on T_LOW.
    let expectations = [
        I2cTrans::write(ADDR, vec![0x03, 0x7F, 0xF0]),
        I2cTrans::write(ADDR, vec![0x02, 0xFF, 0xC0]),
    ];
    let mut dev = sensor(&expectations);
    dev.set_high_temperature_threshold(127.9375).await.unwrap();
    dev.set_low_temperature_threshold(-0.25).await.unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn thresholds_clamp_to_datasheet_limits() {
    // Out-of-range values clamp: 200 C -> 127.9375, -300 C -> -128.0.
    let expectations = [
        I2cTrans::write(ADDR, vec![0x03, 0x7F, 0xF0]),
        I2cTrans::write(ADDR, vec![0x02, 0x80, 0x00]),
    ];
    let mut dev = sensor(&expectations);
    dev.set_high_temperature_threshold(200.0).await.unwrap();
    dev.set_low_temperature_threshold(-300.0).await.unwrap();
    dev.destroy().done();
}
