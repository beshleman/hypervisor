#!/bin/bash

set -e

sudo cp target/aarch64-unknown-linux-gnu/debug/hypervisor /mnt/hypervisor.elf

set +e
killall -q -s 9 $(which qemu-system-aarch64)
set -e

qemu-system-aarch64 -M virt -cpu cortex-a57 -nographic -smp 1 -bios ../u-boot/u-boot.bin \
    -drive if=none,id=hd0,file=../disk.img \
    -device virtio-blk-device,drive=hd0 -S -s

