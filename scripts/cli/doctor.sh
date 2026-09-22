#!/usr/bin/env bash
set -euo pipefail

echo "Running diagnostics..."
echo

missing=0

check() {
    local cmd="$1"
    local pkg="${2:-$1}"
    if command -v "$cmd" >/dev/null 2>&1; then
        echo "✓ $cmd"
    else
        echo "✗ $cmd — missing (install package '$pkg')"
        missing=$((missing + 1))
    fi
}

check mkarchiso archiso
check git
check qemu-system-x86_64 qemu-desktop
check xorriso libisoburn
check mksquashfs squashfs-tools
check mcopy mtools
check mmd mtools
check mkfs.fat dosfstools
check grub-mkstandalone grub
check mkinitcpio
check sudo
check rustc rust
check cargo
check shellcheck
check msgfmt gettext
check pkg-config

# KVM hardware virtualization check
if [ -e /dev/kvm ]; then
    if [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
        echo "✓ /dev/kvm (hardware virtualization ready)"
    else
        echo "✗ /dev/kvm — permission denied for $USER"
        echo "  Fix: sudo usermod -aG kvm $USER (log out and back in)"
        missing=$((missing + 1))
    fi
else
    if [ -n "$(journalctl -k -b 0 -g "disabled by BIOS" --no-pager 2>/dev/null || true)" ]; then
        echo "✗ /dev/kvm — Virtualization (VT-x/AMD-V) is DISABLED in BIOS/UEFI"
        echo "  Fix: Reboot into BIOS/UEFI setup and enable 'Intel Virtualization Technology' (VT-x) or 'SVM'"
    else
        echo "! /dev/kvm — not available (QEMU will fall back to software emulation)"
    fi
fi

echo
if [ "$missing" -ne 0 ]; then
    echo "Diagnostics found missing tools."
    exit 1
fi

echo "Diagnostics complete."
