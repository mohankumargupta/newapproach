---
name: guidance-rust-crate-embedded
description: Guidance for writing rust embedded crates properly
---

# File layout

instead of everything under src/lib.rs, prefer:

under src directory:

<device>.rs
desirable.rs
essential.rs
rare.rs
register.rs
types.rs
errors.rs
lib.rs

under tests
embedded-hal-mock tests go here(
just add the bare minimum tests as possible, tests the main 
core use cases only)

when you are asked to implement essential, desirable, or rare 
functionality, you put them in the appropriate file and then 
import it in <device>.rs 

You will need to implement the following:

- blocking and async support
- if driver supports multiple devices, implement shared logic
  and distinct logic
- if a driver supports multiple interfaces you will need 
  to implement all interfaces


# Addition to README.md

Just one thing, add a Developer test section on how to run 
both unit and functional tests

