#!/usr/bin/env bash

set -e

echo "======================================"
echo "      ChurrOS Build System"
echo "======================================"
echo

echo "[1/4] Preparing branding..."

mkdir -p archiso/airootfs/root

cp branding/customize_airootfs.sh \
    archiso/airootfs/root/customize_airootfs.sh

mkdir -p archiso/airootfs/root/branding

cp -r branding/files \
    archiso/airootfs/root/branding/

echo "[2/4] Cleaning previous build..."

sudo rm -rf work
mkdir -p out

echo "[3/4] Building ISO..."

sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

echo "[4/4] Cleaning temporary files..."

rm -f archiso/airootfs/root/customize_airootfs.sh
rm -rf archiso/airootfs/root/branding

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

find out -name "*.iso"