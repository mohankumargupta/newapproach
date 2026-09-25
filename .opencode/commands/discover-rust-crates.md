---
description: find suitable crates on crates.io for device based on criteria
---

# Context variable

<device> : you get this by reading the file called
           processing.txt in the original pwd

if a folder called rustcrates is not created in 
current folder, create it.

inside that folder, create folder called <device>



i have some criteria for suitable crates for device <device>:

must either have dependency on embedded-hal v1 or
embedded-hal-async v1

if either conditions are satisfied, git clone
repos to rustcrates/<device> directory.

  IMPORTANT - you MUST actually git clone. Do not stop after finding
  candidates. For each crate that satisfies the criteria:
  1. Find its repository URL (from crates.io / docs.rs / lib.rs).
  2. Clone into rustcrates/$1/<crate-name>, e.g.
       git clone https://github.com/<org>/<crate-repo> rustcrates/$1/<crate-name>
  3. Verify each clone: the directory must contain a .git folder and
     a Cargo.toml. If a clone fails, report it, do not skip silently.
  4. When done, list the cloned repos with
       ls rustcrates/<device>
     and finish only after at least one repo is cloned.

  Do not create summary/report files; cloning is the deliverable.



