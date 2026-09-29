set shell:= ["bash", "-c"]
set windows-shell := ["powershell", "-c"]

[no-cd]
run num:
	#!/usr/bin/env bash
	set -x
	device=$(cat processing.txt) 	
	opencode run $(cat {{ num }}-*) | tee outputs/${device}/{{ num }}.log

setup:
    # sudo apt update
	# sudo apt install libssl-dev
    cargo install cargo-clone-crate
