---
description: implement  espforge driver for device <device>
---

driver is no_std rust using 
embedded-hal v1 and embedded-hal-async v1 traits

scope of driver, want to implement 
bare minimal implementation, 
bare essentials a user would expect, no exotic use cases
please.

# Context variable

<device> : you get this by reading the file called
           processing.txt in the original pwd



1. read espforge directory. It contains source code for
   espforge. You need to implement a driver for <device>

2. read the research folder. 
   - esphome contains source code for esphome copied
     from the components directory which you can use
     to see how the yaml should look. find code relevant for
     device <device>
   - esphome.io contains esphome docs for devices, find
     the one that is relevant to device <device> 

   then read the rustcrates folder:
   - it contains rust crates. Evaluate their implementations,
     find what functions they implement, if they are good
     impls use it or if you can do it better
         

3. previous drivers in espforge/espforge-runtime/src/devices
   are implemented in single files, but I want you to 
   implement in a folder and use mod.rs along with the
   following files:
   
   implementation.rs
   error.rs
   register.rs
   
4. Guidance   
   You have a couple of options

   - multi-struct pattern 

   ```rust
   use crate::TemperaturePayload;
   use embedded_hal::i2c::I2c;
   
   pub struct MySensor<I2C> { i2c: I2C }
   
   impl<I2C: I2c> MySensor<I2C> {
       pub fn read_temp(&mut self) -> Result<f32, Error> {
           let mut buf = [0u8; 2];
           self.i2c.write_read(ADDR, &[REG], &mut buf)?;
           Ok(TemperaturePayload(buf).to_celsius())
       }
   }

   use embedded_hal_async::i2c::I2c as AsyncI2c;
   
   pub struct MySensorAsync<I2C> { i2c: I2C }
   
   impl<I2C: AsyncI2c> MySensorAsync<I2C> {
       pub fn async read_temp(&mut self) -> Result<f32, Error> {
           let mut buf = [0u8; 2];
           self.i2c.write_read(ADDR, &[REG], &mut buf).await?;
           Ok(TemperaturePayload(buf).to_celsius())
       }
   }
   
   ```
   should they be in blocking.rs and asynch.rs or 
   in implementation.rs

   - type state/marker generics

   ```rust

   // Markers
   pub struct BlockingMode;
   pub struct AsyncMode;
   
   pub struct MySensor<BUS, MODE> {
       bus: BUS,
       _mode: core::marker::PhantomData<MODE>,
   }
   
   // ----------------------------------------------------
   // Blocking Impl Block
   // ----------------------------------------------------
   impl<I2C: embedded_hal::i2c::I2c> MySensor<I2C, BlockingMode> {
       pub fn read_temp(&mut self) -> Result<f32, Error> {
           // ... standard blocking write_read
       }
   }
   
   // ----------------------------------------------------
   // Async Impl Block
   // ----------------------------------------------------
   impl<I2C: embedded_hal_async::i2c::I2c> MySensor<I2C, AsyncMode> {
       pub async fn read_temp(&mut self) -> Result<f32, Error> {
           // ... standard async write_read .await
       }
   }
   
   ```
   You will need to use judgement, if it makes sense
   to make blocking.rs and asynch.rs instead of implementation.rs

- for now, do not add tests, it just need to compile   
