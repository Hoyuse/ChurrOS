#!/usr/bin/env bash

set -e

VM_DIR="vm"
DISK="$VM_DIR/ChurrOS.qcow2"
VARS="$VM_DIR/OVMF_VARS.fd"

ISO=$(find out -name "*.iso" | head -n1)

if [ -z "$ISO" ]; then
    echo "No ISO found."
    echo "Building..."

    ./churros build

    ISO=$(find out -name "*.iso" | head -n1)
fi

mkdir -p "$VM_DIR"

if [ ! -f "$DISK" ]; then

    echo
    echo "Creating development virtual machine..."
    echo

    qemu-img create -f qcow2 "$DISK" 64G

    cp /usr/share/edk2/x64/OVMF_VARS.4m.fd "$VARS"

fi

echo
echo "Launching ChurrOS Development VM..."
echo

qemu-system-x86_64 \
    -enable-kvm \
    -machine q35,accel=kvm \
    -cpu host \
    -smp 4 \
    -m 4096 \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/x64/OVMF_CODE.4m.fd \
    -drive if=pflash,format=raw,file="$VARS" \
    -drive file="$DISK",format=qcow2,if=virtio \
    -cdrom "$ISO" \
    -boot order=d