#!/bin/bash
set -e

make clean
make

mkdir -p esp/EFI/BOOT
cp BOOTX64.EFI esp/EFI/BOOT/BOOTX64.EFI

OVMF_PATH="/usr/share/edk2-ovmf/x64/OVMF.4m.fd"

if [ ! -f "$OVMF_PATH" ]; then
    echo "Warning: OVMF firmware not found at $OVMF_PATH"
    echo "Please check your distribution's OVMF path and update sim.sh"
fi

qemu-system-x86_64 \
    -bios "$OVMF_PATH" \
    -drive format=raw,file=fat:rw:esp \
    -net none \
    -serial stdio \
    -display default \
    -vga std


# qemu-system-x86_64 \
#     -bios "$OVMF_PATH" \
#     -drive format=raw,file=fat:rw:esp \
#     -net none \
#     -serial stdio
