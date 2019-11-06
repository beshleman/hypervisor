CC := aarch64-linux-gnu-gcc

CFLAGS=-Werror -Wredundant-decls -Wno-pointer-arith -nostdinc      \
	    -nostdlib -fno-builtin -fno-common -ffreestanding -fpic -g

LINKER := qemu-virt-arm64.ld

.PHONY: all

all: src/head.o
	xargo rustc --bin hypervisor --target aarch64-unknown-linux-gnu -- -C link-arg=-nostartfiles -C panic=abort -C link-arg=-T$(LINKER) -C link-arg=src/head.o
	cp target/aarch64-unknown-linux-gnu/debug/hypervisor target/aarch64-unknown-linux-gnu/debug/hypervisor.elf
	aarch64-linux-gnu-objcopy -S target/aarch64-unknown-linux-gnu/debug/hypervisor.elf target/aarch64-unknown-linux-gnu/debug/hypervisor.bin

src/head.o: src/head.S
	$(CC) -I src/ -o $@ -c $< $(CFLAGS)


.PHONY: dump
dump:
	aarch64-linux-gnu-objdump -S target/aarch64-unknown-linux-gnu/debug/hypervisor | less
