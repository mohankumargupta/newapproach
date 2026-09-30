//! Minimal integration tests for the Rare API, using
//! `embedded-hal-mock`. Covers the core use cases only:
//! extended mode, fault queue, alert polarity, thermostat mode,
//! and comparator-mode alert status.

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use tmp102::{AlertPolarity, FaultQueue, SlaveAddr, ThermostatMode, Tmp102, Tmp112};

const DEVICE_ADDRESS: u8 = 0x48;
const REG_CONFIG: u8 = 0x01;
// Power-up default configuration register content.
const DEFAULT_MSB: u8 = 0x60;
const DEFAULT_LSB: u8 = 0xA0;
// Rare bit positions.
const EXTENDED_MODE: u8 = 0x10;
const FAULT_QUEUE1: u8 = 0x10;
const ALERT_POLARITY: u8 = 0x04;
const THERMOSTAT: u8 = 0x02;
const ALERT_FLAG: u8 = 0x20;

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
async fn enables_extended_measurement_mode() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB, DEFAULT_LSB | EXTENDED_MODE],
        ),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor.enable_extended_measurement_mode().await.unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn disables_extended_measurement_mode() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB | EXTENDED_MODE],
        ),
        I2cTransaction::write(DEVICE_ADDRESS, vec![REG_CONFIG, DEFAULT_MSB, DEFAULT_LSB]),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor.disable_extended_measurement_mode().await.unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn sets_fault_queue_preserving_other_bits() {
    // Default (Consecutive1, FQ bits clear) -> Consecutive4 (FQ1 set, FQ0 clear).
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB | FAULT_QUEUE1, DEFAULT_LSB],
        ),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor
        .set_fault_queue(FaultQueue::Consecutive4)
        .await
        .unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn sets_alert_polarity_and_thermostat_mode() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB | ALERT_POLARITY, DEFAULT_LSB],
        ),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB | ALERT_POLARITY, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![
                REG_CONFIG,
                DEFAULT_MSB | ALERT_POLARITY | THERMOSTAT,
                DEFAULT_LSB,
            ],
        ),
    ];
    let mut sensor = setup_tmp102(&expectations);
    sensor
        .set_alert_polarity(AlertPolarity::ActiveHigh)
        .await
        .unwrap();
    sensor
        .set_thermostat_mode(ThermostatMode::Interrupt)
        .await
        .unwrap();
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn reports_comparator_alert_status() {
    // Active-low polarity + alert flag set -> polarity_high(false) != alert(true) -> inactive.
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![REG_CONFIG],
        vec![DEFAULT_MSB, DEFAULT_LSB | ALERT_FLAG],
    )];
    let mut sensor = setup_tmp102(&expectations);
    assert!(!sensor.is_comparator_mode_alert_active().await.unwrap());
    sensor.destroy().done();

    // Active-high polarity + alert flag set -> active.
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![REG_CONFIG],
        vec![DEFAULT_MSB | ALERT_POLARITY, DEFAULT_LSB | ALERT_FLAG],
    )];
    let mut sensor = setup_tmp102(&expectations);
    assert!(sensor.is_comparator_mode_alert_active().await.unwrap());
    sensor.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn tmp112_supports_rare_api() {
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB, DEFAULT_LSB | EXTENDED_MODE],
        ),
        I2cTransaction::write_read(
            DEVICE_ADDRESS,
            vec![REG_CONFIG],
            vec![DEFAULT_MSB, DEFAULT_LSB],
        ),
        I2cTransaction::write(
            DEVICE_ADDRESS,
            vec![REG_CONFIG, DEFAULT_MSB | ALERT_POLARITY, DEFAULT_LSB],
        ),
    ];
    let mut sensor = setup_tmp112(&expectations);
    sensor.enable_extended_measurement_mode().await.unwrap();
    sensor
        .set_alert_polarity(AlertPolarity::ActiveHigh)
        .await
        .unwrap();
    sensor.destroy().done();
}
