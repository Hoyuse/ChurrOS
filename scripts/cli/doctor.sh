#!/usr/bin/env bash

echo "Running diagnostics..."
echo

check() {
    if command -v "$1" >/dev/null 2>&1; then
        echo "✓ $1"
    else
        echo "✗ $1"
    fi
}

check mkarchiso
check git
check qemu-system-x86_64
check xorriso
check mksquashfs
check mcopy

echo
echo "Diagnostics complete."