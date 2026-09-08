#!/usr/bin/env bash

echo "Running diagnostics..."
echo

check() {
    if command -v "$1" >/dev/null 2>&1; then
        echo "✓ $1"
    else
        echo "✗ $1 — missing"
    fi
}

check mkarchiso
check git
check qemu-system-x86_64
check qemu-system-aarch64
check edk2-aarch64
check systemd-binfmt
check xorriso
check mksquashfs
check mcopy
check mkinitcpio
check grub-mkstandalone
check qemu-img

echo
echo "Diagnostics complete."
