.PHONY: all clean kernel bootloader iso

all: bootloader

kernel:
	@echo "=== Building Rust Kernel ==="
	cd kernel && cargo build --release --target x86_64-unknown-none
	objcopy -O binary kernel/target/x86_64-unknown-none/release/kernel kernel.bin

bootloader: kernel
	@echo "=== Building FASM Bootloader ==="
	fasm bootloader/main.asm BOOTX64.EFI

# AI Generated Code - I was too lazy xd
iso: bootloader
	@echo "=== Building UEFI ISO ==="
	mkdir -p IMAGE/EFI/BOOT
	mkdir -p output
	
	cp BOOTX64.EFI IMAGE/EFI/BOOT/BOOTX64.EFI
	cp kernel.bin IMAGE/kernel.bin
	
	dd if=/dev/zero of=efiboot.img bs=1M count=4
	mkfs.vfat efiboot.img
	
	mmd -i efiboot.img ::/EFI
	mmd -i efiboot.img ::/EFI/BOOT
	mcopy -i efiboot.img BOOTX64.EFI ::/EFI/BOOT/BOOTX64.EFI
	
	mv efiboot.img IMAGE/efiboot.img
	
	xorriso -as mkisofs \
		-iso-level 3 \
		-full-iso9660-filenames \
		-volid "ytk_os" \
		-eltorito-alt-boot \
		-e efiboot.img \
		-no-emul-boot \
		-isohybrid-gpt-basdat \
		-output output/ytk.iso \
		IMAGE
	
	rm -rf IMAGE


clean:
	cd kernel && cargo clean
	rm -f kernel.bin
	rm -f BOOTX64.EFI
	rm -rf esp/
	rm -rf IMAGE efiboot.img output/



release: iso
	glab release create "v$$(cargo metadata --manifest-path kernel/Cargo.toml --format-version 1 | jq -r '.packages[] | select(.name=="kernel") | .version')" \
		--name "Release $$(date +%Y-%m-%d)" \
		--notes "Release $$(date +%Y-%m-%d)" \
		kernel.bin \
		BOOTX64.EFI \
		output/ytk.iso
	rm -rf output
