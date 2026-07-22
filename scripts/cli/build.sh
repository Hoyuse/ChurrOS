#!/usr/bin/env bash

set -e

cleanup_temp() {
    echo "[cleanup] Removing temporary build files..."
    rm -f archiso/airootfs/root/customize_airootfs.sh 2>/dev/null || true
    rm -rf archiso/airootfs/root/branding 2>/dev/null || true
    rm -rf archiso/airootfs/root/packages 2>/dev/null || true
    rm -rf archiso/airootfs/etc/calamares 2>/dev/null || true
    rm -f archiso/airootfs/etc/polkit-1/rules.d/49-calamares.rules 2>/dev/null || true
}

trap cleanup_temp EXIT

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

sudo rm -rf work out
mkdir -p out

echo "[4/5] Building ISO..."

sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

sudo chown -R "$USER:$USER" work out 2>/dev/null || true

echo "[5/5] Cleaning build artifacts..."

rm -rf work 2>/dev/null || true

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

find out -name "*.iso"
