#!/usr/bin/env bash

set -e

ISO=$(find out -name "*.iso" | head -n1)

if [ -z "$ISO" ]; then
    echo "No ISO found."
    echo "Building..."

    ./churros build

    ISO=$(find out -name "*.iso" | head -n1)
fi

echo "Launching QEMU..."

qemu-system-x86_64 \
    -enable-kvm \
    -m 4096 \
    -cpu host \
    -smp 4 \
    -bios /usr/share/edk2/x64/OVMF.4m.fd \
    -cdrom "$ISO"