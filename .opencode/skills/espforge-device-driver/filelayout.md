# espforge v2: where everything lives

Read the top table first. Stop reading when you found your file. Paths are relative to `v2/`.
Companion doc for adding hardware: `add-device.md`.

## 0. The 10-second picture

```
YAML spec --parse--> Project --validate--> --resolve--> DeviceTree (IR)
   --emit--> generated Rust project (uses espforge-runtime on the chip)
```

Two worlds, never mixed:

| World | Crates | Runs on | std? |
|---|---|---|---|
| Host (the tool) | `espforge-model`, `espforge`, `espforge-bindings`, `espforge-examples` | your PC | std |
| Chip (the firmware) | `espforge-runtime` | ESP32 | **no_std**, esp-hal only on riscv32/xtensa |

Hard rule: host crates never depend on `espforge-runtime` in Cargo. They only write its name into generated text.

## 1. "I want to..." lookup (start here)

| I want to... | Go to |
|---|---|
| Add a device (sensor, display, reader) | `add-device.md`, then the 3 places below |
| ...write the code that runs on the chip | `espforge-runtime/src/devices/<name>.rs` + `devices/mod.rs` + `Cargo.toml` `[features]` |
| ...describe it for validation | `espforge-bindings/src/catalog.rs` |
| ...say how to construct it in generated code | `espforge-bindings/src/devices/<name>.rs` |
| ...give users a sample | `espforge-examples/examples/<NN.Category>/<name>/` |
| Find where `component!` / `device!` / `signal!` are defined | `espforge/src/emit/rust.rs`, constant `MACROS` inside `emit_lib` |
| Find `Context` (logger, delay, components, devices) | same file, `emit_lib` (the `pub struct Context`) |
| Find `Logger` / `Delay` types | `espforge-runtime/src/lib.rs` |
| Change the generated `main.rs` | `espforge/src/emit/rust.rs`: `emit_main_blocking`, `emit_main_embassy`, `emit_stack_setup` |
| Change the generated `generated.rs` (Components/Devices structs) | `espforge/src/emit/rust.rs`: `emit_generated` |
| Change how Cargo.toml is produced | `espforge/src/emit/rust.rs`: `emit_cargo_toml`, `merge_runtime_dep`, `espforge_dep` |
| Add a YAML field / change YAML shape | `espforge-model/src/project.rs` |
| Add or change a validation rule | `espforge/src/pipeline.rs`: `validate` |
| Change what the IR carries | `espforge-model/src/ir.rs` + `pipeline.rs`: `resolve` |
| Change how a GPIO/I2C/SPI/UART is built in generated code | `espforge-model/src/backend.rs` (`Blocking`) |
| Find shared helpers for `construct()` | `espforge-model/src/codegen.rs` |
| Change CLI commands (`create`, `build`, `validate`) | `espforge/src/main.rs` |
| Change the `justfile` / `answers.yaml` that `create` writes | `espforge/src/main.rs`: `justfile_content`, `write_settings`, `read_answers` |
| Change Wokwi `diagram.json` templating or `wokwi.toml` | `espforge/src/emit/wokwi.rs` |
| Change how `esp-generate` is called | `espforge/src/emit/scaffold.rs` |
| Change "don't overwrite edited files" behavior | `espforge/src/emit/mod.rs` (`write`) + `emit/manifest.rs` |
| Add a runtime Cargo feature | `espforge-runtime/Cargo.toml` `[features]` |
| Add a test for emitted output | `espforge/tests/manifest_policy.rs` (copy the helper) |

## 2. Crate by crate

### espforge-model (host, std). The vocabulary. Leaf crate.

| File | What is in it |
|---|---|
| `src/project.rs` | Typed YAML: `Project`, `Esp32Section`, bus configs, `Instance`, list-or-map parsing |
| `src/ir.rs` | `DeviceTree`, `ResolvedInstance`, `Peripheral`, `Tier` (Component/Device), `Dependency`, `Flags` |
| `src/driver.rs` | `Driver` trait, `Construction`, `GenContext`, `Registry` |
| `src/catalog.rs` | `DriverSpec`, `DepSpec`, `SpecFlags` (validation metadata, no codegen) |
| `src/backend.rs` | `Backend` trait + `Blocking`: renders `Output::new(...)`, `I2cBus::build(...)`, `ctor(...)` strings |
| `src/codegen.rs` | `sanitize`, `peripheral_field`, `gpio_field_from_with`, `bool_or` |
| `src/value.rs` | `Span`, `ResourceRef`, `PinRef`, `Artifact` (+`Ownership`), `Diag` |

### espforge (host, std). The tool. CLI + pipeline.

| File | What is in it |
|---|---|
| `src/main.rs` | CLI, `create`/`build`/`validate`, `answers.yaml`, `justfile`, copies `app.rs`/chip/diagram |
| `src/parse.rs` | `parse_file`, `parse_str` (YAML -> `Project`) |
| `src/pipeline.rs` | `validate` (errors as `Diag`), `resolve` (-> `DeviceTree`) |
| `src/emit/mod.rs` | `generate` (all artifacts), `write` (ownership + drift check) |
| `src/emit/rust.rs` | **The big one**: Cargo.toml merge, `generated.rs`, `lib.rs` (+macros, `Context`), `main.rs` |
| `src/emit/scaffold.rs` | Runs `esp-generate`, merges its output without overwriting |
| `src/emit/wokwi.rs` | `diagram.json` (minijinja) and `wokwi.toml` |
| `src/emit/manifest.rs` | `README.txt` and `.espforge/manifest.json` checksums |
| `tests/manifest_policy.rs` | Integration tests on emitted `Cargo.toml` |

### espforge-bindings (host, std). Per-driver host side.

| File | What is in it |
|---|---|
| `src/catalog.rs` | `catalog()`: one `DriverSpec` per driver (kind, tier, pins, peripherals, deps, flags) |
| `src/components/<kind>.rs` | `led`, `button`, `i2c`, `spi`, `uart`, `http`: `Driver` impls |
| `src/devices/<kind>.rs` | `ssd1306`, `ili9341`, `rc522`: `Driver` impls |
| `src/components/mod.rs`, `src/devices/mod.rs` | One line: `include!` of the file `build.rs` generates |
| `build.rs` | Globs the two folders, writes the module list + `Registry` (so no `mod.rs` edits) |
| `src/lib.rs` | `registry()` merges component + device registries |

Every driver file ends up with `pub const DRIVER: &'static dyn Driver = &...;`. Use `//` comments, never `//!` (the file is `include!`d).

### espforge-runtime (chip, no_std). The code that actually runs.

| File | What is in it |
|---|---|
| `src/lib.rs` | `Logger`, `Delay`, re-exports (`embedded_hal`, ...); module declarations |
| `src/components/{led,button,i2c,spi,uart}.rs` | Hardware components (esp-hal types) |
| `src/components/mod.rs` | Feature-gated `pub mod` and `pub use` |
| `src/devices/{ssd1306,ili9341,rc522}.rs` | Terminal devices. `rc522.rs` is generic + host-tested |
| `src/devices/mod.rs` | Feature-gated `pub mod`/`pub use` (+ target-gated `EspRc522` alias) |
| `src/services/http.rs` | Software service (no pins): `Http` |
| `Cargo.toml` | **Feature list** (one per driver) and optional deps; esp-hal only on esp targets |

### espforge-examples (host, std). Samples baked into the binary.

| Path | What is in it |
|---|---|
| `examples/<NN.Category>/<name>/<name>.yaml` | The spec (must contain `espforge:`) |
| `.../app/rust/app.rs` | User code: `setup` and `forever` |
| `.../diagram.json` | Wokwi diagram template (`{{ board }}`, `{{ serialMonitor }}`, `{{ id.pin }}`) |
| `.../chip.json`, `chip.wasm`, `chip/*.zig` | Optional custom Wokwi chip |
| `src/lib.rs` | `include_dir` lookups: `example_names`, `find_example`, `asset`, `example_spec` |
| `build.rs` | Makes cargo rebuild when example files change |

Categories today: `01.Basics`, `02.Digital`, `04.Communication`, `05.Networking`, `06.Displays`.

### v2 root

| File | What |
|---|---|
| `Cargo.toml` | Workspace. `default-members` **excludes** `espforge-runtime` |
| `README.md` | Crate table + dependency rule |
| `testplanrc522.md` | The rc522 host-test plan (a worked example of host testing) |

## 3. What a generated project looks like

`espforge build` writes into the `--out` dir (usually `build/`):

| File | Who owns it |
|---|---|
| `Cargo.toml` | Machine. esp-generate's base + merged `espforge-runtime` (+features), `static_cell`, `esp-alloc` |
| `src/generated.rs` | Machine. `PeripheralRegistry`, `Components`, `Devices` |
| `src/lib.rs` | Machine. Re-exports of every used type, **macros**, `Context`, `CTX` |
| `src/bin/main.rs` | Machine. Entry point; builds components, then devices; calls `app::setup/forever` |
| `src/app.rs` | **You.** Copied from the project's `src/app.rs` every build |
| `diagram.json`, `wokwi.toml`, `chip/` | Generated/copied for Wokwi |
| `README.txt`, `.espforge/manifest.json` | Ownership docs + checksums for drift detection |
| everything else (`.cargo/`, toolchain, ...) | `esp-generate` scaffold, never overwritten |

## 4. Where do these specific things live?

| Thing | Defined in | Notes |
|---|---|---|
| `component!(ctx, id)` | `espforge/src/emit/rust.rs` (`MACROS` in `emit_lib`) | Emitted into the generated `src/lib.rs`. Expands to `&ctx.components.id`. Defined before `pub mod app;`, so `app.rs` sees it with no import |
| `device!(ctx, id)` | same | Expands to `&ctx.devices.id` |
| `signal!(NAME)` | same | Embassy `Signal`; needs `signal` runtime feature (auto for embassy) |
| `Context` struct | same (`emit_lib`) | `logger`, `delay`, `components`, `devices` |
| `Components` / `Devices` structs | `emit_generated` | One field per instance |
| Instance constructor expression | `espforge-bindings/src/{components,devices}/<kind>.rs`, `construct()` | Strings that reference `registry`, `components`, `delay` |
| Pin/bus constructor snippets | `espforge-model/src/backend.rs` | `gpio_output`, `i2c_master`, ... |
| `Logger`, `Delay` | `espforge-runtime/src/lib.rs` | |
| Which runtime features a project gets | `emit()` in `rust.rs` | Union of each driver's `runtime_features()` (+ embassy/signal/alloc) |
| `ESPFORGE_USE_LOCAL` / path deps | `espforge_dep`, `v2_root` in `rust.rs` | Flips crates.io vs local path |
| WiFi/network stack (`NET_STACK`) | `emit_stack_setup` in `rust.rs` | Only when a driver sets `needs_stack` |
| Heap setup (`heap_allocator!`) | `emit_main_*` in `rust.rs` | Only when `has_alloc` |
| Unknown-driver / unresolved-ref errors | `pipeline.rs` `validate` | |
| Registry of drivers | generated by `espforge-bindings/build.rs` | Lives in `target/.../out/` |

## 5. Pipeline in function names (trace a bug top to bottom)

1. `espforge/src/main.rs` `run()` -> `Command::Build`
2. `parse::parse_str`
3. `pipeline::validate` (stops here on errors)
4. `pipeline::resolve`
5. `emit::scaffold::scaffold` (esp-generate)
6. `emit::generate` -> `emit::rust::emit` -> for each instance `Driver::construct()` (bindings)
7. `emit::write` (ownership/drift)
8. `emit::wokwi::resolve_diagram`, `write_wokwi_toml`; `copy_chip_to_out`, `copy_app_to_out`

## 6. Tripwires (things that bite)

- Feature name must match across: runtime `Cargo.toml`, bindings file name, catalog `kind`, `Driver::kind()`, YAML `using:`.
- Every key in a catalog spec's `pins`/`peripherals`/`deps` is **required** by validation.
- Device pin keys in YAML must be `$name` refs to `esp32.gpio` entries, not bare integers.
- Runtime code that names `esp_hal` must be gated with `target_arch = "riscv32"/"xtensa"`, not `cfg(test)`.
- `espforge-runtime` is not a default workspace member: test with `cargo test -p espforge-runtime --features <name>`.
- `Driver::flags()`/`required_features()` are unused; catalog `SpecFlags` are what count.
- Devices are blocking-only; use `runtime: blocking` in device examples.
- Known stale test: `display_requests_ssd1306_runtime_feature` in `espforge/src/main.rs` points at a non-existent example path.

