#!/usr/bin/env bash

set -e
HOST_REPO_SYMLINK=0
EDITION="niri"
TARGET_ARCH="aarch64"
PACKAGE_LIST="archiso/packages.${TARGET_ARCH}"
while [[ $# -gt 0 ]]; do
    case "$1" in
        --edition|-e)
            EDITION="$2"
            shift 2
            ;;
        --edition=*)
            EDITION="${1#*=}"
            shift
            ;;
        *)
            shift
            ;;
    esac
done

EDITION=$(echo "$EDITION" | tr '[:upper:]' '[:lower:]')
if [ "$EDITION" != "niri" ] && [ "$EDITION" != "xfce" ]; then
    echo "Error: unsupported edition '$EDITION' (supported: niri, xfce)" >&2
    exit 1
fi

if [ ! -f "$PACKAGE_LIST" ]; then
    echo "Error: package list not found: $PACKAGE_LIST" >&2
    exit 1
fi

PACKAGES_BACKED_UP=0
GREETD_BACKED_UP=0
cleanup_temp() {
    echo "[cleanup] Removing temporary build files..."
    if [ "$HOST_REPO_SYMLINK" -eq 1 ]; then
        echo "[cleanup] Removing host /root/packages symlink..."
        sudo rm -f /root/packages 2>/dev/null || true
    fi
    if [ "$PACKAGES_BACKED_UP" -eq 1 ] && [ -f "$PACKAGE_LIST.orig" ]; then
        mv "$PACKAGE_LIST.orig" "$PACKAGE_LIST"
    fi
    if [ "$GREETD_BACKED_UP" -eq 1 ] && [ -f archiso/airootfs/etc/greetd/config.toml.orig ]; then
        mv archiso/airootfs/etc/greetd/config.toml.orig archiso/airootfs/etc/greetd/config.toml
    fi
    rm -f archiso/airootfs/etc/churros-edition 2>/dev/null || true
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
echo "      Edition: ${EDITION^^}"
echo "======================================"
echo

# 0. Configurar paquetes según la edición
if [ "$EDITION" = "xfce" ]; then
    echo "[0/5] Selecting XFCE packages..."
    EDITION_PACKAGE_LIST="archiso/packages.xfce.${TARGET_ARCH}"
    if [ -f "$EDITION_PACKAGE_LIST" ]; then
        cp "$PACKAGE_LIST" "$PACKAGE_LIST.orig"
        PACKAGES_BACKED_UP=1
        cp "$EDITION_PACKAGE_LIST" "$PACKAGE_LIST"
    else
        echo "Error: $EDITION_PACKAGE_LIST not found!" >&2
        exit 1
    fi
fi

# Guardar la edición activa en el airootfs
mkdir -p archiso/airootfs/etc
echo "$EDITION" > archiso/airootfs/etc/churros-edition

# Configurar greetd para la sesión correspondiente
mkdir -p archiso/airootfs/etc/greetd
if [ -f archiso/airootfs/etc/greetd/config.toml ]; then
    cp archiso/airootfs/etc/greetd/config.toml archiso/airootfs/etc/greetd/config.toml.orig
    GREETD_BACKED_UP=1
fi

if [ "$EDITION" = "xfce" ]; then
    cat > archiso/airootfs/etc/greetd/config.toml << 'EOF'
[terminal]
vt = 1

[default_session]
command = "/usr/bin/startxfce4"
user = "churros"
EOF
else
    cat > archiso/airootfs/etc/greetd/config.toml << 'EOF'
[terminal]
vt = 1

[default_session]
command = "/usr/bin/niri"
user = "churros"
EOF
fi

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
    "$CHURROS_VERSION" \
    "$EDITION"

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
YAY_PKG=$(ls archiso/packages/yay-*.pkg.tar.zst 2>/dev/null | head -1 || true)
BAZAAR_PKG=$(ls archiso/packages/bazaar-*.pkg.tar.zst 2>/dev/null | head -1 || true)

if [ -z "$PYWAL_PKG" ] || [ -z "$YAY_PKG" ]; then
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

if mountpoint -q work 2>/dev/null; then
    sudo find work -mindepth 1 -delete 2>/dev/null || sudo rm -rf work/* 2>/dev/null || true
else
    sudo rm -rf work
fi
sudo rm -rf out
mkdir -p out

echo "[5/5] Building ISO...";

# El repo local [churros] usa Server = file:///root/packages. Durante pacstrap
# file:// se resuelve contra el root del HOST (no el chroot), así que exponemos
# el repo local en /root/packages del host para que el build lo encuentre.
if sudo test -L /root/packages && [ "$(sudo readlink /root/packages)" = "$PWD/archiso/packages" ]; then
    echo "  /root/packages symlink already in place."
    HOST_REPO_SYMLINK=1
elif sudo test -e /root/packages || sudo test -L /root/packages; then
    echo "  WARNING: /root/packages exists but is not our symlink — leaving as is."
else
    echo "  Exposing local repo at host /root/packages..."
    sudo ln -sfn "$PWD/archiso/packages" /root/packages
    HOST_REPO_SYMLINK=1
fi

sudo mkarchiso -v \
    -w work \
    -o out \
    archiso

sudo chown -R "$USER:$USER" work out 2>/dev/null || true

echo "[5/5] Cleaning build artifacts..."

if mountpoint -q work 2>/dev/null; then
    sudo find work -mindepth 1 -delete 2>/dev/null || sudo rm -rf work/* 2>/dev/null || true
else
    rm -rf work 2>/dev/null || true
fi

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

find out -name "*.iso"
