//! Minimal integration tests for the Desirable API, using
//! `embedded-hal-mock`. Covers the core use cases only:
//! one-shot triggering + readiness, conversion rate, and
//! high/low temperature thresholds.

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use tmp102::{ConversionRate, SlaveAddr, Tmp102, Tmp112};

const DEVICE_ADDRESS: u8 = 0x48;
const REG_CONFIG: u8 = 0x01;
const REG_T_LOW: u8 = 0x02;
const REG_T_HIGH: u8 = 0x03;
// Power-up default configuration register content.
const DEFAULT_MSB: u8 = 0x60;
const DEFAULT_LSB: u8 = 0xA0;
// One-shot flag lives in config MSB bit 7.
const ONE_SHOT: u8 = 0x80;

fn setup_tmp102(expectations: &[I2cTransaction]) -> Tmp102<I2cMock> {
    Tmp102::new(I2cMock::new(expectations), SlaveAddr::default())
}

fn setup_tmp112(expectations: &[I2cTransaction]) -> Tmp112<I2cMock> {
    Tmp112::new(I2cMock::new(expectations), SlaveAddr::default())
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn triggers_one_shot_measurement() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB | ONE_SHOT, DEFAULT_LSB],
        ),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor.trigger_one_shot_measurement().await.unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn reports_one_shot_ready_when_os_bit_set() {
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![REG_CONFIG],
        vec![DEFAULT_MSB | ONE_SHOT, DEFAULT_LSB],
    )];
    let mut sensor = setup_tmp102(&expectations);
    assert!(sensor.is_one_shot_measurement_result_ready().await.unwrap());
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn reports_one_shot_busy_when_os_bit_clear() {
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![REG_CONFIG],
        vec![DEFAULT_MSB, DEFAULT_LSB],
    )];
    let mut sensor = setup_tmp102(&expectations);
    assert!(!sensor.is_one_shot_measurement_result_ready().await.unwrap());
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn sets_conversion_rate_preserving_other_bits() {
    // Default 4Hz (CR1=1, CR0=0) -> 1Hz (CR1=0, CR0=1): 0xA0 -> 0x60.
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(DEVICE_ADDRESS, vec![REG_CONFIG, DEFAULT_MSB, 0x60]),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor
        .set_conversion_rate(ConversionRate::Hz1)
        .await
        .unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn sets_temperature_thresholds() {
    // 60.0 C -> 0x3C00, 80.0 C -> 0x5000 in normal 12-bit mode.
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(DEVICE_ADDRESS, vec![REG_T_LOW, 0x3C, 0x00]),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(DEVICE_ADDRESS, vec![REG_T_HIGH, 0x50, 0x00]),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor.set_low_temperature_threshold(60.0).await.unwrap();
    sensor.set_high_temperature_threshold(80.0).await.unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn tmp112_supports_desirable_api() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB | ONE_SHOT, DEFAULT_LSB],
        ),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(DEVICE_ADDRESS, vec![REG_T_HIGH, 0x50, 0x00]),
    ];
    let mut sensor = setup_tmp112(&expectations);
    sensor.trigger_one_shot_measurement().await.unwrap();
    sensor.set_high_temperature_threshold(80.0).await.unwrap();
    sensor.destroy().done();
}
