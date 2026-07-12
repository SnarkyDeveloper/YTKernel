.PHONY: all clean kernel bootloader

all: bootloader

kernel:
	@echo "=== Building Rust Kernel ==="
	cd kernel && cargo build --release --target x86_64-unknown-none
	objcopy -O binary kernel/target/x86_64-unknown-none/release/kernel kernel.bin


bootloader: kernel
	@echo "=== Building FASM Bootloader ==="
	fasm bootloader/main.fasm BOOTX64.EFI

clean:
	cd kernel && cargo clean
	rm -f kernel.bin
	rm -f BOOTX64.EFI
	rm -rf esp/
