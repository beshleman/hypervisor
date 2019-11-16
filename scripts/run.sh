#!/bin/bash

set -e
set -x

#sudo cp target/aarch64-unknown-linux-gnu/debug/hypervisor /mnt/hypervisor.elf

set +e
killall -q -s 9 $(which qemu-system-aarch64)
set -e

#	-machine virt,gic_version=3 \
#qemu-system-aarch64 -M virt -cpu cortex-a53 -nographic -smp 1 -bios ../u-boot/u-boot.bin \
#    -machine virtualization=true	\
#    -drive if=none,id=hd0,file=../disk.img \
#    -device virtio-blk-device,drive=hd0 -S -s

#    -machine dumpdtb=qemu-aarch64.dtb	\

#    -drive if=none,id=hd0,file=/home/bobbye/projects/disk.img 	\
#    -device virtio-blk-device,drive=hd0 -S -s			\

qemu-system-aarch64 -M virt -cpu cortex-a53 -nographic -smp 1 \
    -kernel /home/bobbye/projects/hypervisor/target/aarch64-unknown-linux-gnu/debug/hypervisor.bin	\
    -machine virt,gic_version=2					\
    -machine virtualization=true				\
    -device loader,file=../u-boot/u-boot.bin,addr=0x40400000	\
    -monitor telnet::45454,server,nowait # -s -S
