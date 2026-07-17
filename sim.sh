#!/bin/bash
set -e

make clean
export INVID_INSTANCE_URL="${INVID_INSTANCE_URL:-example.com}"
make

mkdir -p esp/EFI/BOOT
cp BOOTX64.EFI esp/EFI/BOOT/BOOTX64.EFI
cp kernel.bin esp/

OVMF_PATH="/usr/share/edk2-ovmf/x64/OVMF.4m.fd"

if [ ! -f "$OVMF_PATH" ]; then
	echo "Warning: OVMF firmware not found at $OVMF_PATH"
	echo "Please check your distribution's OVMF path and update sim.sh"
fi

qemu-system-x86_64 \
	-m 2G \
	-bios "$OVMF_PATH" \
	-drive format=raw,file=fat:rw:esp \
	-net none \
	-serial stdio \
	-display default \
	-vga std \
	-d int \
	-no-reboot \
	# -s -S

# qemu-system-x86_64 \
#     -bios "$OVMF_PATH" \
#     -drive format=raw,file=fat:rw:esp \
#     -net none \
#     -serial stdio
