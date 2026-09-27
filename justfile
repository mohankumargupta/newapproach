set shell:= ["bash", "-c"]
set windows-shell := ["powershell", "-c"]

[no-cd]
run num:
	#!/usr/bin/env bash
	set -x
	device=$(cat processing.txt) 	
	opencode run $(cat {{ num }}-*) | tee outputs/${device}/plan.md

		
