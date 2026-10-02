---
name: espforge-device-driver
description: use when you are asked to add device driver for espforge
---

To learn more about where everything is in the espforge folder,
read filelayout.md in this skill

# Add a device to espforge

Reference implementations: 
`rc522` 
`ssd1306`
`ili9341`

## Crates

| Crate | Environment |
|---|---|
| `espforge-model`, `espforge`, `espforge-bindings`, `espforge-examples` | host, std |
| `espforge-runtime` | chip, `no_std` (std allowed only in `#[cfg(test)]`); not a workspace default member |

- Host crates must never depend on `espforge-runtime` in Cargo. They name it only inside emitted strings.
- `esp-hal` exists only on riscv32/xtensa. Gate every `esp_hal`/`crate::components::*` use with `#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]`. Never gate with `cfg(test)`.

## Naming

`<name>` must be identical in: runtime Cargo feature, bindings file stem, catalog `kind`, `Driver::kind()`, YAML `using:`.

## 1. espforge-runtime

1. Create `espforge-runtime/src/devices/<name>.rs`.
2. Add to `src/devices/mod.rs`:
   ```rust
   #[cfg(feature = "<name>")] pub mod <name>;
   #[cfg(feature = "<name>")] pub use <name>::<Type>;
   #[cfg(all(feature = "<name>", any(target_arch = "riscv32", target_arch = "xtensa")))] pub use <name>::Esp<Type>;  // only if alias exists
   ```
3. Add to `Cargo.toml` `[features]`: `<name> = []`, or `<name> = ["dep:<crate>"]` with the crate listed under `[dependencies]` as `optional = true`.
4. Requirements:
   - Constructor is `new`. Receive buses, pins and `Delay` by value.
   - Public methods take `&self`; use `RefCell` for mutation.
   - Blocking only (`I2cBus<Blocking>`, `SpiDevice<Blocking>`).
5. For host testing, make the struct generic over `embedded-hal` traits and pin esp-hal types in a target-gated alias (pattern from `rc522.rs`):
   ```rust
   #[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
   use crate::components::spi::SpiError;
   #[cfg(not(any(target_arch = "riscv32", target_arch = "xtensa")))]
   #[derive(Debug, Clone, Copy, PartialEq)]
   pub enum SpiError { Bus }                       // plus `impl embedded_hal::spi::Error` (kind -> Other)

   pub struct MyDev<SPI, RST, DLY> { spi: RefCell<SPI>, reset: RefCell<Option<RST>>, delay: RefCell<DLY> }

   #[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
   pub type EspMyDev = MyDev<crate::components::spi::SpiDevice<esp_hal::Blocking>, esp_hal::gpio::Output<'static>, crate::Delay>;
   ```
   Map bus/pin errors with `.map_err(|_| SpiError::Bus)`.
   A driver that uses `I2cBus`/`SpiDevice` directly (like `ssd1306`) cannot be host-compiled. Skip tests for it and state so in the report.
6. Tests (`#[cfg(test)] mod tests`, `embedded-hal-mock` 0.11 `eh1`):
   - Wrap every SPI call: `SpiTx::transaction_start()`, ops (`write_vec`, `transfer_in_place(tx, rx)`), `SpiTx::transaction_end()`.
   - Clone the mock into the driver; call `.done()` on every mock in every test.
   - SPI mock cannot inject errors: use a fake `SpiDevice` returning `Err(ErrorKind::Other)`. Digital mock: `.with_error(MockError::Io(..))`.
   - Per public method: happy path, error path, boundary (empty, max size), stops-after-failure.
   - Use `NoopDelay`. Assert bytes and call order, not timing.

## 2. espforge-bindings

1. Create `espforge-bindings/src/devices/<name>.rs`. No `mod.rs` edit: `build.rs` globs the folder and `include!`s each file into module `<stem>`.
   - Must export `pub const DRIVER: &'static dyn Driver = &MyDevDriver;`.
   - Use `//` comments. `//!` breaks the `include!`.
2. Implement `Driver` (SPI device with CS and optional reset, from `rc522.rs`):
   ```rust
   use espforge_model::codegen;
   use espforge_model::driver::{Construction, Driver, GenContext};
   use espforge_model::ir::{DepKind, ResolvedInstance, Tier};
   use espforge_model::value::{Artifact, Diag};

   #[derive(Debug)] pub struct MyDevDriver;
   pub const DRIVER: &'static dyn Driver = &MyDevDriver;

   impl Driver for MyDevDriver {
       fn kind(&self) -> &str { "<name>" }
       fn tier(&self) -> Tier { Tier::Device }
       fn type_name(&self) -> &str { "EspMyDev" }   // must be exported from espforge_runtime::devices
       fn generate(&self, _: &ResolvedInstance, _: &GenContext) -> Result<Vec<Artifact>, Diag> { Ok(vec![]) }
       fn construct(&self, inst: &ResolvedInstance, ctx: &GenContext) -> Construction {
           let spi_field = inst.deps.iter().find(|d| d.kind == DepKind::Instance)
               .map(|d| codegen::sanitize(&d.name)).unwrap_or_else(|| "unreachable!()".into());
           let cs = ctx.backend.gpio_output(&codegen::gpio_field_from_with(ctx, inst, "cs"), true);
           let spi = format!("espforge_runtime::components::SpiDevice::<esp_hal::Blocking>::new(components.{spi_field}, {cs}, delay)");
           Construction::for_instance(inst, ctx.backend.ctor(Tier::Device, "EspMyDev", &[spi, "None".into(), "delay".into()]))
       }
   }
   ```
3. Names usable inside `construct` expressions: `registry.peripherals.<FIELD>`, `components.<field>`, `delay`. Components are built before devices.
4. Helpers: `codegen::sanitize`, `codegen::gpio_field_from_with(ctx, inst, key)` (resolves `$name` only), `codegen::bool_or`, `ctx.backend.gpio_output/gpio_input/ctor`. Need a new snippet type: add a method to `espforge-model/src/backend.rs`.
5. Leave `runtime_features()` default (`[kind]`, must equal the runtime feature name). `Driver::flags()` and `required_features()` are unused; leave defaults.
6. Add to `espforge-bindings/src/catalog.rs` inside `catalog()`:
   ```rust
   DriverSpec {
       kind: "<name>".to_string(),
       tier: Tier::Device,
       deps: vec![DepSpec { key: "spi".to_string(), kind: DepKind::Instance, access: Access::Shared }],
       pins: vec!["cs".to_string()],        // `with:` keys holding pin refs
       peripherals: vec![],                 // `with:` keys claiming a bus (components only)
       flags: SpecFlags { needs_delay: true, ..Default::default() },
   },
   ```
   - Every key in `pins`, `peripherals`, `deps` is required; a missing `with:` key fails validation. Do not list optional keys. (`rc522` lists optional `rst`; existing bug, do not copy.)
   - Pin values in YAML must be `$name` refs to `esp32.gpio` entries.
   - `SpecFlags` (`is_embassy`, `has_alloc`, `has_wifi`, `needs_delay`, `needs_stack`) feed `ir.flags`. `needs_delay` is informational. `main.rs` blocking/embassy comes from YAML `runtime:`.
   - Devices may depend on components only, never on devices.
   - An SPI device's `spi` component must not set `cs:` (the device owns CS).

## 3. espforge-examples

Create `espforge-examples/examples/<NN.Category>/<name>/`:

- `<name>.yaml`: must contain `espforge:`.
  ```yaml
  espforge: { name: <name>_example, target: esp32c3, runtime: blocking }
  esp32:
    spi:  { spi2: { spi: 2, sck: 3, mosi: 4, miso: 2, mode: 0, frequency_kHz: 1000 } }
    gpio: { pin_cs: { pin: 5, direction: output } }
  components:
    main_spi: { using: spi, with: { bus: $spi2 } }
  devices:
    mydev: { using: <name>, with: { spi: $main_spi, cs: $pin_cs } }
  ```
- `app/rust/app.rs`: `use crate::{Context, device};`, `pub fn setup(ctx: &mut Context)`, `pub fn forever(ctx: &mut Context)`; access with `device!(ctx, mydev)`. Template: `06.Displays/ssd1306_example/app/rust/app.rs`.
- `diagram.json`: minijinja. Tokens: `{{ board }}`, `{{ serialMonitor }}`, `{{ <instance_id>.pin }}`. Copy serial TX/RX connections from an existing example.
- Optional custom Wokwi chip: `chip.json`, `chip.wasm`, `chip/*.zig`.

Do not test `espforge-examples`; a separate harness does.

## 4. Generated output (what your code plugs into)

- `src/generated.rs`: field `pub <id>: espforge_runtime::devices::<type_name_for()>,`
- `src/lib.rs`: `pub use espforge_runtime::devices::<type_name()>;`
- `src/bin/main.rs`: `let components = Components {..}; let devices = Devices {..};`, then `app::setup`.
- `Cargo.toml`: `espforge-runtime` features = union of every instance's `runtime_features()`.

## 5. Verification

Run from `v2/`. Record a baseline `cargo test` before editing. Known pre-existing failure: `display_requests_ssd1306_runtime_feature` (`espforge/src/main.rs`) reads a nonexistent `06.Displays/display/display.yaml`. Report baseline failures separately.

```sh
cargo build
cargo test
cargo test -p espforge-runtime --features <name>
cargo test -p espforge-runtime --features rc522        # regression
cargo clippy --all-targets
cargo clippy -p espforge-runtime --features <name> --tests
cargo fmt --check
```

Add `espforge/tests/<name>_policy.rs` (copy helpers from `manifest_policy.rs`). Use `parse::parse_str`, `pipeline::validate`, `pipeline::resolve`, `emit::rust::emit` against a temp dir. Assert:

1. A valid spec validates.
2. `Cargo.toml` artifact contains the `<name>` runtime feature.
3. `src/bin/main.rs` contains `espforge_runtime::devices::<Type>::new(`; `src/generated.rs` contains `pub <id>: espforge_runtime::devices::<Type>,`.
4. A spec missing a required `with:` key, or with an unresolved `$ref`, returns `Err` with a `Diag`.

Not verifiable in this workspace: esp-hal/target compilation. If an esp toolchain is available, run `espforge build` on your example and build the output. Otherwise report "target build not verified".

## Failure checklist

- Name mismatch across feature/stem/kind/`using:`.
- Missing catalog entry or `devices/mod.rs` lines.
- `//!` in a bindings file.
- `cfg(test)` used instead of `target_arch` gating.
- Host test with a feature that pulls esp-hal types (`ssd1306`, `spi`, ...).
- Integer pin values on a device.
- Optional key listed in catalog `pins`/`peripherals`/`deps`.
- `runtime: embassy` in a device example.
- `&mut self` public methods.
- Mock without `.done()` or without `transaction_start`/`transaction_end`.


