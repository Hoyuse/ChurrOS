#!/usr/bin/env bash

set -e

echo "======================================"
echo "        ChurrOS Build System"
echo "======================================"
echo

echo "[1/6] Cleaning previous build..."
sudo rm -rf work out
mkdir -p out

echo "[2/6] Preparing branding..."
rm -rf archiso/airootfs/root/branding
mkdir -p archiso/airootfs/root/branding

cp -r branding/files archiso/airootfs/root/branding/
cp branding/customize_airootfs.sh archiso/airootfs/root/

echo "[3/6] Building ChurrOS ISO..."
sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

echo "[4/6] Cleaning temporary branding..."
rm -rf archiso/airootfs/root/branding
rm -f archiso/airootfs/root/customize_airootfs.sh

echo
echo "======================================"
echo " Build completed!"
echo "======================================"
echo

find out -name "*.iso"
