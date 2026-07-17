# YTKernel 
im like bored and so like this is for a ysws for a custom skirt

this is a WIP kernel that can search for, find, and play audio and video. This DOES require a selfhosted instance of [invidious](https://invidious.io) for now. It will compile with whatever is in your environment with INVID_INSTANCE_URL="https://example.com" otherwise fail.

## Downloading
You can download from [releases](https://git.owo.sh/snarky/ytkernel/-/releases) to get the files to run with qemu, or you may clone and run with ./sim.sh

Run the following to run the iso
```bash
# change the bios flag depending on your distro
qemu-system-x86_64 \
    -bios /usr/share/edk2-ovmf/x64/OVMF.4m.fd \
    -cdrom ytk.iso \
    -m 256M \
    -vga std \
    -serial stdio
```
