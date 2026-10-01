# tmp102

`no_std` Rust driver for TMP102 / TMP112 (TMP112A/B/N) digital temperature sensors over I2C.
Implements `embedded-hal` v1 (blocking) and `embedded-hal-async` v1 (async, via `async` feature).

## Status

Essential stage implemented (plan items #1–#2):

- `read_temperature()` → `f32` Celsius
- `set_conversion_mode(ConversionMode::Shutdown | Continuous)`

Desirable stage implemented (plan items #3–#6):

- `trigger_one_shot_measurement()`
- `is_one_shot_measurement_result_ready() -> bool`
- `set_conversion_rate(ConversionRate::{_0_25Hz,_1Hz,_4Hz,_8Hz})`
- `set_high/low_temperature_threshold(f32)` (clamped, extended-mode aware)

Rare (#7–#11) implemented (plan items #7–#11):

- `enable/disable_extended_measurement_mode()` (aliases `enable/disable_extended_mode()`)
- `set_fault_queue(FaultQueue::{_1,_2,_4,_6})` (`FaultQueue::Consecutive4` alias works)
- `set_alert_polarity(AlertPolarity::{ActiveLow,ActiveHigh})`
- `set_thermostat_mode(ThermostatMode::{Comparator,Interrupt})`
- `is_comparator_mode_alert_active() -> bool`

## Addresses

| ADD0 pin | Address |
|---|---|
| GND | `0x48` |
| VDD | `0x49` |
| SDA | `0x4A` |
| SCL | `0x4B` |

## Example

```rust
use tmp102::{ConversionMode, SlaveAddr, Tmp102};

let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
let temp: f32 = sensor.read_temperature().unwrap();
sensor.set_conversion_mode(ConversionMode::Shutdown).unwrap();
sensor.set_conversion_mode(ConversionMode::Continuous).unwrap();
```

Async (feature `async`, same API with `.await`):

```toml
tmp102 = { version = "0.1.0", features = ["async"] }
```

## Developer test

```bash
# blocking (embedded-hal) incl. unit + integration tests
cargo test
# async (embedded-hal-async)
cargo test --features async
# lints
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
