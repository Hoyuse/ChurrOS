#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
WORK_DIR="$PROJECT_DIR/work/calamares-build"
PACKAGE_DIR="$PROJECT_DIR/archiso/packages"

echo "======================================"
echo "  Building Calamares from AUR"
echo "======================================"

mkdir -p "$PACKAGE_DIR"

if ls "$PACKAGE_DIR"/calamares-*.pkg.tar.zst 1>/dev/null 2>&1; then
    echo "[skip] Calamares already built."
    exit 0
fi

if [ ! -w "$WORK_DIR" ] && [ -d "$WORK_DIR" ]; then
    echo "[!]  $WORK_DIR is not writable (did you run this with sudo before?)"
    echo "     Run:  sudo chown -R $USER:$USER work/"
    exit 1
fi

rm -rf "$WORK_DIR"

echo "[1/3] Cloning calamares from AUR..."
git clone https://aur.archlinux.org/calamares.git "$WORK_DIR"

echo "[2/3] Building calamares (this may take a while)..."
cd "$WORK_DIR"
makepkg -sf --noconfirm

echo "[3/3] Installing package to local repo..."

cp "$WORK_DIR"/*.pkg.tar.zst "$PACKAGE_DIR/"
rm -f "$PACKAGE_DIR"/calamares-debug-*.pkg.tar.zst 2>/dev/null || true

cd "$PACKAGE_DIR"
repo-add churros.db.tar.gz *.pkg.tar.zst

echo
echo "======================================"
echo "  Calamares build complete."
echo "======================================"
echo "  Packages:"
ls -la "$PACKAGE_DIR"/*.pkg.tar.zst
echo
echo "  Now run: ./churros build"
