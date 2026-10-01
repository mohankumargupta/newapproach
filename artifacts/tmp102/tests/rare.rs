//! Minimal integration tests for Rare functionality (blocking + async).
//!
//! Covers plan items #7–#11: extended mode, fault queue, alert polarity,
//! thermostat mode, comparator-mode alert status.

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use tmp102::{AlertPolarity, FaultQueue, SlaveAddr, ThermostatMode, Tmp102};

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
async fn extended_mode_enable_disable() {
    let expectations = [
        I2cTrans::write(ADDR, vec![0x01, CFG_MSB, CFG_LSB | 0x10]),
        I2cTrans::write(ADDR, vec![0x01, CFG_MSB, CFG_LSB]),
    ];
    let mut dev = sensor(&expectations);
    dev.enable_extended_measurement_mode().await.unwrap();
    dev.disable_extended_measurement_mode().await.unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn fault_queue_4_sets_bits() {
    // 0x60 | FQ1(0x10) = 0x70.
    let expectations = [I2cTrans::write(ADDR, vec![0x01, 0x70, CFG_LSB])];
    let mut dev = sensor(&expectations);
    dev.set_fault_queue(FaultQueue::Consecutive4).await.unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn alert_polarity_and_thermostat_mode() {
    let expectations = [
        I2cTrans::write(ADDR, vec![0x01, CFG_MSB | 0x04, CFG_LSB]),
        I2cTrans::write(ADDR, vec![0x01, (CFG_MSB | 0x04) | 0x02, CFG_LSB]),
    ];
    let mut dev = sensor(&expectations);
    dev.set_alert_polarity(AlertPolarity::ActiveHigh)
        .await
        .unwrap();
    dev.set_thermostat_mode(ThermostatMode::Interrupt)
        .await
        .unwrap();
    dev.destroy().done();
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), test),
    async(feature = "async", tokio::test)
)]
async fn comparator_alert_respects_polarity() {
    // Polarity high + ALERT bit set => active.
    let expectations = [I2cTrans::write_read(
        ADDR,
        vec![0x01],
        vec![CFG_MSB | 0x04, CFG_LSB],
    )];
    let mut dev = sensor(&expectations);
    assert!(dev.is_comparator_mode_alert_active().await.unwrap());
    dev.destroy().done();

    // Polarity low + ALERT bit set => inactive.
    let expectations = [I2cTrans::write_read(
        ADDR,
        vec![0x01],
        vec![CFG_MSB, CFG_LSB],
    )];
    let mut dev = sensor(&expectations);
    assert!(!dev.is_comparator_mode_alert_active().await.unwrap());
    dev.destroy().done();
}
