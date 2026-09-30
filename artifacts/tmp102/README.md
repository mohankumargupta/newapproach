# tmp102

Platform-agnostic `no_std` driver for the TMP102 and TMP112 digital
temperature sensors, implementing `embedded-hal` v1 and
`embedded-hal-async` v1.

Scope: **Essential + Desirable + Rare** functionality — reading temperature,
switching between continuous and shutdown conversion modes, triggering
and polling one-shot measurements, setting the conversion rate, setting
the high/low temperature thresholds, extended measurement mode, fault
queue, alert polarity, thermostat mode, and comparator-mode alert status
over I2C.

```rust
use tmp102::{ConversionMode, ConversionRate, SlaveAddr, Tmp102};

let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
let temp: f32 = sensor.read_temperature().unwrap();
sensor.set_conversion_mode(ConversionMode::Shutdown).unwrap();
sensor.set_conversion_mode(ConversionMode::Continuous).unwrap();
// Desirable
sensor.trigger_one_shot_measurement().unwrap();
if sensor.is_one_shot_measurement_result_ready().unwrap() {
    let temp: f32 = sensor.read_temperature().unwrap();
}
sensor.set_conversion_rate(ConversionRate::Hz1).unwrap();
sensor.set_low_temperature_threshold(60.0).unwrap();
sensor.set_high_temperature_threshold(80.0).unwrap();
// Rare
sensor.enable_extended_measurement_mode().unwrap();
sensor.disable_extended_measurement_mode().unwrap();
sensor.set_fault_queue(tmp102::FaultQueue::Consecutive4).unwrap();
sensor.set_alert_polarity(tmp102::AlertPolarity::ActiveHigh).unwrap();
sensor.set_thermostat_mode(tmp102::ThermostatMode::Interrupt).unwrap();
let alert: bool = sensor.is_comparator_mode_alert_active().unwrap();
```

Enable the `async` feature for the `embedded-hal-async` API.

## Developer test

Run unit + integration tests (blocking):

```sh
cargo test
```

Run unit + integration tests (async):

```sh
cargo test --features async
```
