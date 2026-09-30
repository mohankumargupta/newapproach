//! Minimal integration tests for the Essential API, using
//! `embedded-hal-mock`. Covers the core use cases only:
//! reading temperature and switching conversion mode, on both
//! device types.

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use tmp102::{ConversionMode, SlaveAddr, Tmp102, Tmp112};

const DEVICE_ADDRESS: u8 = 0x48;
const REG_TEMP: u8 = 0x00;
const REG_CONFIG: u8 = 0x01;
// Power-up default configuration register content.
const DEFAULT_MSB: u8 = 0x60;
const DEFAULT_LSB: u8 = 0xA0;

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
async fn tmp102_reads_temperature() {
    // 0x19 0x00 = 25.0 °C in normal 12-bit mode.
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![REG_TEMP],
        vec![0x19, 0x00],
    )];
    let mut sensor = setup_tmp102(&expectations);
    let temp = sensor.read_temperature().await.unwrap();
    assert!((temp - 25.0).abs() < f32::EPSILON, "got {temp}");
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn tmp102_enters_shutdown_mode_preserving_other_bits() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB | 0x01, DEFAULT_LSB],
        ),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor
        .set_conversion_mode(ConversionMode::Shutdown)
        .await
        .unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn tmp102_returns_to_continuous_mode() {
    // Device currently shut down (SD bit set); driver must clear only it.
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB | 0x01, DEFAULT_LSB],
        ),
        I2cTransaction::write(DEVICE_ADDRESS, vec![REG_CONFIG, DEFAULT_MSB, DEFAULT_LSB]),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor
        .set_conversion_mode(ConversionMode::Continuous)
        .await
        .unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn tmp112_reads_temperature() {
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![REG_TEMP],
        vec![0x19, 0x00],
    )];
    let mut sensor = setup_tmp112(&expectations);
    let temp = sensor.read_temperature().await.unwrap();
    assert!((temp - 25.0).abs() < f32::EPSILON, "got {temp}");
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn alternative_address_is_used_on_bus() {
    // ADD0 to SCL -> 0x4B; the transaction must target that address.
    let expectations = [I2cTransaction::write_read(
        0x4B,
        vec![REG_TEMP],
        vec![0x19, 0x00],
    )];
    let mut sensor = Tmp102::new(
        I2cMock::new(&expectations),
        SlaveAddr::Alternative(true, true),
    );
    let temp = sensor.read_temperature().await.unwrap();
    assert!((temp - 25.0).abs() < f32::EPSILON, "got {temp}");
    sensor.destroy().done();
}
