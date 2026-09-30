# tmp102

Platform-agnostic `no_std` driver for the TMP102 and TMP112 digital
temperature sensors, implementing `embedded-hal` v1 and
`embedded-hal-async` v1.

Scope: **Essential** functionality only — reading temperature and
switching between continuous and shutdown (one-shot) conversion modes
over I2C.

```rust
use tmp102::{ConversionMode, SlaveAddr, Tmp102};

let mut sensor = Tmp102::new(i2c, SlaveAddr::default());
let temp: f32 = sensor.read_temperature().unwrap();
sensor.set_conversion_mode(ConversionMode::Shutdown).unwrap();
sensor.set_conversion_mode(ConversionMode::Continuous).unwrap();
```

Enable the `async` feature for the `embedded-hal-async` API.
