set shell:= ["bash", "-c"]
set windows-shell := ["powershell", "-c"]

[no-cd]
run num:
	#!/usr/bin/env bash
	device=$(cat processing.txt)
	mkdir -p outputs/$device 	
	opencode run $(cat {{ num }}-*) | tee outputs/${device}/{{ num }}.log
	if [ {{ num }} -eq 01 ]; then
	  cp cp outputs/${device}/{{ num }}.log outputs/research.log 
	fi 

setup:
	# sudo apt update
	# sudo apt install libssl-dev
	cargo install cargo-clone-crate

clean:
	rm -rf research outputs artifacts

