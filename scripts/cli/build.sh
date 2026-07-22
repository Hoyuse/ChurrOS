#!/usr/bin/env bash

set -e

echo "======================================"
echo "      ChurrOS Build System"
echo "======================================"
echo

echo "[1/5] Preparing branding..."

mkdir -p archiso/airootfs/root

cp branding/customize_airootfs.sh \
    archiso/airootfs/root/customize_airootfs.sh

mkdir -p archiso/airootfs/root/branding

cp -r branding/files \
    archiso/airootfs/root/branding/

CALAMARES_PKG=$(ls archiso/packages/calamares-[0-9]*.pkg.tar.zst 2>/dev/null | head -1 || true)

if [ -n "$CALAMARES_PKG" ]; then
    echo "[2/5] Integrating Calamares installer..."

    bash installer/apply-calamares.sh

    mkdir -p archiso/airootfs/root/packages
    cp archiso/packages/calamares-*.pkg.tar.zst archiso/airootfs/root/packages/
    cp archiso/packages/churros.db* archiso/airootfs/root/packages/ 2>/dev/null || true
    cp archiso/packages/churros.files* archiso/airootfs/root/packages/ 2>/dev/null || true
else
    echo "[2/5] Calamares not found — building without installer."
    echo "      Run ./scripts/build-calamares.sh first to include it."
fi

echo "[3/5] Cleaning previous build..."

sudo rm -rf work
mkdir -p out

echo "[4/5] Building ISO..."

sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

echo "[5/5] Cleaning temporary files..."

rm -f archiso/airootfs/root/customize_airootfs.sh
rm -rf archiso/airootfs/root/branding
rm -rf archiso/airootfs/root/packages
rm -rf archiso/airootfs/etc/calamares
rm -f archiso/airootfs/etc/polkit-1/rules.d/49-calamares.rules

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

find out -name "*.iso"
