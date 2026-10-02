set shell := ["bash", "-c"]

_default:
	@just --choose --chooser "fzf --layout=reverse"

# [no-cd]
# run num:
# 	#!/usr/bin/env bash
# 	device=$(cat processing.txt)
# 	mkdir -p outputs/$device 	
# 	opencode run $(cat {{ num }}-*) | tee outputs/${device}/{{ num }}.log
# 	if [ {{ num }} -eq 01 ]; then
# 	  cp cp outputs/${device}/{{ num }}.log outputs/research.log 
# 	fi 

[no-cd]
task01-groundwork:
  #!/usr/bin/env bash
  mkdir -p research artifacts outputs
  [ -d research/esphome ]    || npx tiged esphome/esphome/esphome/components#dev research/esphome
  [ -d research/esphome.io ] || npx tiged esphome/esphome.io/src/content/docs/components#current research/esphome.io
  [ -d research/Periph ]     || npx tiged tuhde/Periph#main research/Periph
  [ -d research/devices ]    || npx tiged periph/devices#main research/devices
  mkdir -p research/goldstandardcrates
  cd research/goldstandardcrates && for c in scd4x sht4x bme280 ds323x ads1x1x mcp251xfd tcs3472 eeprom24x lis3dh lis3dh-async; do \
  [ -d $c ] || cargo clone $c; done
  
[no-cd]
task02-espforge-github:
  rm -rf espforge
  npx tiged mohankumargupta/espforge/v2#espforgev2 espforge
  cd espforge && git init -b main && git add -A && git commit -qm "initial commit"

_stage cmd num args='':
  #!/usr/bin/env bash
  dev=$(cat processing.txt)
  mkdir -p outputs/$dev
  opencode run "/{{ cmd }} $dev" | tee -a outputs/$dev/{{num}}.log


task03-discover-rust-crates:
  @just _stage discover-rust-crates task03-discover-rust-crates

task04-plan-rust-driver:
  @just _stage plan-rust-driver task04-plan-rust-driver

task05-rust-driver-essential:
  @just _stage rust-driver task05-rust-driver-essential Essential

task06-rust-driver-desirable:
  @just _stage rust-driver task06-rust-driver-desirable Desirable

task07-rust-driver-rare:
  @just _stage rust-driver task07-rust-driver-rare Rare

task08-espforge-add-device:
  @just _stage espforge-add-device task-08-espforge-add-device

task10-setup:
	# sudo apt update
	# sudo apt install libssl-dev
	cargo install cargo-clone-crate cargo-mutants

task11-clean:
	rm -rf research outputs artifacts espforge rustcrates


