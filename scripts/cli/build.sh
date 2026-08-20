#!/usr/bin/env bash

set -e

HOST_REPO_SYMLINK=0

cleanup_temp() {
    echo "[cleanup] Removing temporary build files..."
    if [ "$HOST_REPO_SYMLINK" -eq 1 ]; then
        echo "[cleanup] Removing host /root/packages symlink..."
        sudo rm -f /root/packages 2>/dev/null || true
    fi
    rm -f archiso/airootfs/root/customize_airootfs.sh 2>/dev/null || true
    rm -rf archiso/airootfs/root/branding 2>/dev/null || true
    rm -rf archiso/airootfs/root/packages 2>/dev/null || true
    rm -rf archiso/airootfs/etc/calamares 2>/dev/null || true
    rm -f archiso/airootfs/etc/polkit-1/rules.d/49-calamares.rules 2>/dev/null || true
    # Binarios Rust desplegados por build-rust.sh (no se versionan en git)
    rm -f archiso/airootfs/usr/bin/churros-welcome 2>/dev/null || true
    rm -f archiso/airootfs/usr/bin/churros-settings 2>/dev/null || true
    rm -f archiso/airootfs/usr/bin/churros-popup 2>/dev/null || true
    rm -f archiso/airootfs/usr/bin/churros-control-center 2>/dev/null || true
    # GRUB theme copiado al airootfs para que esté disponible en el sistema instalado
    rm -rf archiso/airootfs/usr/share/churros/grub-theme 2>/dev/null || true
}

trap cleanup_temp EXIT

echo "======================================"
echo "      ChurrOS Build System"
echo "======================================"
echo

echo "[1/5] Preparing branding..."

bash scripts/build-grub-theme.sh

mkdir -p archiso/airootfs/root

cp branding/customize_airootfs.sh \
    archiso/airootfs/root/customize_airootfs.sh

mkdir -p archiso/airootfs/root/branding

cp -r branding/files \
    archiso/airootfs/root/branding/

cp VERSION archiso/airootfs/root/branding/VERSION
cp branding/stamp-os-release.sh archiso/airootfs/root/branding/stamp-os-release.sh
chmod +x archiso/airootfs/root/branding/stamp-os-release.sh
CHURROS_VERSION=$(tr -d '[:space:]' < VERSION)
bash branding/stamp-os-release.sh \
    archiso/airootfs/root/branding/files/os-release \
    "$CHURROS_VERSION"

if [ -d branding/grub-theme ]; then
    cp -r branding/grub-theme \
        archiso/airootfs/root/branding/grub-theme

    mkdir -p archiso/airootfs/usr/share/churros
    cp -r branding/grub-theme \
        archiso/airootfs/usr/share/churros/grub-theme
fi

echo "[2/5] Checking packages..."

# Always invoke: rebuilds if the package is missing or linked against a
# different libpython than the ISO's `python` package (pacstrap).
bash scripts/build-calamares.sh
CALAMARES_PKG=$(ls archiso/packages/calamares-[0-9]*.pkg.tar.zst 2>/dev/null | head -1 || true)
PYWAL_PKG=$(ls archiso/packages/python-pywal-*.pkg.tar.zst 2>/dev/null | head -1 || true)
WAYPAPER_PKG=$(ls archiso/packages/waypaper-*.pkg.tar.zst 2>/dev/null | head -1 || true)
BAZAAR_PKG=$(ls archiso/packages/bazaar-*.pkg.tar.zst 2>/dev/null | head -1 || true)

if [ -z "$PYWAL_PKG" ] || [ -z "$WAYPAPER_PKG" ]; then
    echo "  AUR extras not found — building..."
    bash scripts/build-aur.sh
fi

if [ -z "$BAZAAR_PKG" ]; then
    echo "  Bazaar not found — building (patched to fix libdex conflict)..."
    bash scripts/build-bazaar.sh
fi

if [ -n "$CALAMARES_PKG" ]; then
    echo "  Integrating Calamares installer..."

    bash installer/apply-calamares.sh

    mkdir -p archiso/airootfs/root/packages
    cp archiso/packages/*.pkg.tar.zst archiso/airootfs/root/packages/
    cp archiso/packages/churros.db* archiso/airootfs/root/packages/ 2>/dev/null || true
    cp archiso/packages/churros.files* archiso/airootfs/root/packages/ 2>/dev/null || true
else
    echo "  Calamares not available — building without installer."
fi

echo "[3/5] Building Rust apps...";

bash scripts/build-rust.sh;

echo "[4/5] Cleaning previous build...";

sudo rm -rf work out
mkdir -p out

echo "[5/5] Building ISO...";

# El repo local [churros] usa Server = file:///root/packages. Durante pacstrap
# file:// se resuelve contra el root del HOST (no el chroot), así que exponemos
# el repo local en /root/packages del host para que el build lo encuentre.
if [ -L /root/packages ] && [ "$(readlink /root/packages)" = "$PWD/archiso/packages" ]; then
    echo "  /root/packages symlink already in place."
    HOST_REPO_SYMLINK=1
elif [ -e /root/packages ]; then
    echo "  WARNING: /root/packages exists but is not our symlink — leaving as is."
else
    echo "  Exposing local repo at host /root/packages..."
    sudo ln -s "$PWD/archiso/packages" /root/packages
    HOST_REPO_SYMLINK=1
fi

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
