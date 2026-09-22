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

echo
if [ "$missing" -ne 0 ]; then
    echo "Diagnostics found missing tools."
    exit 1
fi

echo "Diagnostics complete."
