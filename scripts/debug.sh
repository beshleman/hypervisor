#!/bin/bash

aarch64-linux-gnu-gdb target/aarch64-unknown-linux-gnu/debug/hypervisor.elf -ex "target remote localhost:1234" -ex "hb start_hypervisor" -ex "c" -ex "layout src"
