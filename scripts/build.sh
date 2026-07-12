#!/usr/bin/env bash

set -e

echo "======================================"
echo "        ChurrOS Build System"
echo "======================================"
echo

echo "[1/5] Cleaning previous build..."
sudo rm -rf work
mkdir -p out

echo "[2/5] Building ChurrOS ISO..."
sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

echo

find out -name "*.iso"
