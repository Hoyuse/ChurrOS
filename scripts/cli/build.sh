#!/usr/bin/env bash

set -e

echo "======================================"
echo "      ChurrOS Build System"
echo "======================================"
echo

echo "[1/2] Cleaning previous build..."
sudo rm -rf work
mkdir -p out

echo "[2/2] Building ISO..."

sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

find out -name "*.iso"