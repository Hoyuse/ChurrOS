#!/usr/bin/env bash
set -euo pipefail

echo "Running diagnostics..."
echo

missing=0

check() {
    if command -v "$1" >/dev/null 2>&1; then
        echo "✓ $1"
    else
        echo "✗ $1 — missing"
        missing=$((missing + 1))
    fi
}

check mkarchiso
check git
check qemu-system-x86_64
check xorriso
check mksquashfs
check mcopy
check mkinitcpio
check sudo
check rustc
check cargo
check shellcheck
check msgfmt
check pkg-config

echo
if [ "$missing" -ne 0 ]; then
    echo "Diagnostics found missing tools."
    exit 1
fi

echo "Diagnostics complete."
