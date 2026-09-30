#!/usr/bin/env bash

set -e
HOST_REPO_SYMLINK=0
EDITION="niri"
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

PACKAGES_BACKED_UP=0
unmount_work_submounts() {
    local target_dir="${1:-work}"
    if [ -d "$target_dir" ]; then
        local abs_target
        abs_target=$(cd "$target_dir" 2>/dev/null && pwd)
        if [ -n "$abs_target" ]; then
            local mounts
            if command -v findmnt >/dev/null 2>&1; then
                mounts=$(findmnt -lno TARGET 2>/dev/null | grep "^$abs_target/" | sort -r || true)
            else
                mounts=$(awk -v p="$abs_target" '$2 ~ "^"p"/" {print $2}' /proc/mounts 2>/dev/null | sort -r || true)
            fi
            if [ -n "$mounts" ]; then
                echo "  [cleanup] Desmontando sistemas de archivos residuales en $target_dir..."
                while IFS= read -r mnt; do
                    if [ -n "$mnt" ]; then
                        sudo umount -l "$mnt" 2>/dev/null || true
                    fi
                done <<< "$mounts"
            fi
        fi
    fi
}

cleanup_temp() {
    echo "[cleanup] Removing temporary build files..."
    unmount_work_submounts work
    if [ "$HOST_REPO_SYMLINK" -eq 1 ]; then
        echo "[cleanup] Removing host /root/packages symlink..."
        sudo rm -f /root/packages 2>/dev/null || true
    fi
    if [ "$PACKAGES_BACKED_UP" -eq 1 ] && [ -f archiso/packages.x86_64.orig ]; then
        mv archiso/packages.x86_64.orig archiso/packages.x86_64
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

# Pre-flight: validar dependencias esenciales del host antes de compilar
missing_deps=()
for tool in mkarchiso mksquashfs xorriso; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        missing_deps+=("$tool")
    fi
done

if ! command -v grub-mkstandalone >/dev/null 2>&1; then
    missing_deps+=("grub (comando grub-mkstandalone requerido para uefi.grub)")
fi

if ! command -v mkfs.fat >/dev/null 2>&1; then
    missing_deps+=("dosfstools (comando mkfs.fat)")
fi

if ! command -v mcopy >/dev/null 2>&1 || ! command -v mmd >/dev/null 2>&1; then
    missing_deps+=("mtools (comandos mcopy y mmd)")
fi

if [ "${#missing_deps[@]}" -gt 0 ]; then
    echo "Error: Faltan dependencias en el host para compilar la ISO con mkarchiso:" >&2
    for dep in "${missing_deps[@]}"; do
        echo "  - $dep" >&2
    done
    echo >&2
    echo "Instálalas con:" >&2
    echo "  sudo pacman -S --needed archiso grub dosfstools mtools squashfs-tools libisoburn" >&2
    exit 1
fi

# 0. Configurar paquetes según la edición
if [ "$EDITION" = "xfce" ]; then
    echo "[0/5] Selecting XFCE packages..."
    if [ -f archiso/packages.xfce.x86_64 ]; then
        cp archiso/packages.x86_64 archiso/packages.x86_64.orig
        PACKAGES_BACKED_UP=1
        cp archiso/packages.xfce.x86_64 archiso/packages.x86_64
    else
        echo "Error: archiso/packages.xfce.x86_64 not found!" >&2
        exit 1
    fi
fi

# Guardar la edición activa en el airootfs
mkdir -p archiso/airootfs/etc
echo "$EDITION" > archiso/airootfs/etc/churros-edition

# Configurar greetd autologin para la sesión Live
mkdir -p archiso/airootfs/etc/greetd
if [ "$EDITION" = "xfce" ]; then
    cat > archiso/airootfs/etc/greetd/config.toml << 'EOF'
[terminal]
vt = 7

[default_session]
command = "env WLR_NO_HARDWARE_CURSORS=1 XCURSOR_THEME=Adwaita XCURSOR_SIZE=24 cage -s -- regreet"
user = "greeter"

[initial_session]
command = "startxfce4"
user = "churros"
EOF
else
    cat > archiso/airootfs/etc/greetd/config.toml << 'EOF'
[terminal]
vt = 7

[default_session]
command = "env WLR_NO_HARDWARE_CURSORS=1 XCURSOR_THEME=Adwaita XCURSOR_SIZE=24 cage -s -- regreet"
user = "greeter"

[initial_session]
command = "niri"
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

unmount_work_submounts work
if mountpoint -q work 2>/dev/null; then
    echo "  work is mounted (tmpfs) — cleaning contents..."
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

unmount_work_submounts work
if mountpoint -q work 2>/dev/null; then
    sudo find work -mindepth 1 -delete 2>/dev/null || sudo rm -rf work/* 2>/dev/null || true
else
    sudo rm -rf work 2>/dev/null || true
fi

echo
echo "======================================"
echo " Build completed!"
echo "======================================"

find out -name "*.iso"
