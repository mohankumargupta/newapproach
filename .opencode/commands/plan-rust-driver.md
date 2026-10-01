---
description: create a plan for writing espforge device driver
---


# Context variable

<device> : you get this by reading the file called
           processing.txt in the original pwd

# Create plan 
create plan called outputs/<device>/plan.md using
the template below as an example.
 
# Template example for example device tmp102

```markdown
# <device>

* **no_std Rust Crates Implementing `embedded-hal v1 and embedded-hal-async v1`
* **Supporting Devices:** TMP102, TMP112 (including TMP112A, TMP112B, TMP112N)
* **Interfaces Used:** I2C
* **Addresses: 
              0x48(ADD0 to GND)
              0x49(ADD0 to VDD)
              0x4A(ADD0 to SDA)
              0x4B(ADD0 to SCL)  

### **Essential (Core Daily Operations)**

#### **1. Read the temperature**

```rust
let temp = sensor.read_temperature().unwrap();

```

#### **2. Change into one-shot or continuous conversion mode**

```rust
sensor.set_conversion_mode(tmp1x2::ConversionMode::Shutdown).unwrap(); // Sleep
sensor.set_conversion_mode(tmp1x2::ConversionMode::Continuous).unwrap(); // Active

```

---

### **Desirable (Common Configurations & Optimizations)**

#### **3. Trigger a one-shot measurement**

```rust
sensor.trigger_one_shot_measurement().unwrap();

```

#### **4. Read whether the one-shot measurement result is ready**

```rust
if sensor.is_one_shot_measurement_result_ready().unwrap() {
    let temp = sensor.read_temperature().unwrap();
}

```

#### **5. Set the conversion rate**

```rust
sensor.set_conversion_rate(tmp1x2::ConversionRate::Hz1).unwrap();

```

#### **6. Set the high/low temperature threshold**

```rust
sensor.set_low_temperature_threshold(60.0).unwrap();
sensor.set_high_temperature_threshold(80.0).unwrap();

```

---

### **Rare (Specialized Hardware Tweaks & Hardware Status)**

#### **7. Enable/disable the extended measurement mode**

```rust
sensor.enable_extended_measurement_mode().unwrap();
sensor.disable_extended_measurement_mode().unwrap();

```

#### **8. Set the fault queue**

```rust
sensor.set_fault_queue(tmp1x2::FaultQueue::Consecutive4).unwrap();

```

#### **9. Set the alert polarity**

```rust
sensor.set_alert_polarity(tmp1x2::AlertPolarity::ActiveHigh).unwrap();

```

#### **10. Set the thermostat mode**

```rust
sensor.set_thermostat_mode(tmp1x2::ThermostatMode::Interrupt).unwrap();

```

#### **11. Read whether a comparator mode alert is active**

```rust
let is_active = sensor.is_comparator_mode_alert_active().unwrap();

```

---

### **Summary Table**

| # | Feature | Category | Use Case |
| --- | --- | --- | --- |
| **1** | Read temperature | Essential | Normal operational loop |
| **2** | Change conversion mode | Essential | Toggle sleep vs continuous measurement |
| **3** | Trigger one-shot measurement | Desirable | On-demand wake-up for low-power battery systems |
| **4** | Read one-shot measurement ready status | Desirable | Polling sensor readiness before reading data |
| **5** | Set conversion rate | Desirable | Balancing wake time vs responsiveness (0.25Hz–8Hz) |
| **6** | Set high/low temperature threshold | Desirable | Thermal cutoffs, thermostat trigger bounds |
| **7** | Enable/disable extended measurement mode | Rare | High-temperature applications (>128°C up to 150°C) |
| **8** | Set fault queue | Rare | Debouncing false alarms from transient temp spikes |
| **9** | Set alert polarity | Rare | Matching microcontroller interrupt line specs (Active High/Low) |
| **10** | Set thermostat mode | Rare | Switching between Comparator and Interrupt pin behaviors |
| **11** | Read comparator mode alert status | Rare | Checking hardware alert flag status directly via I2C |

``` 


