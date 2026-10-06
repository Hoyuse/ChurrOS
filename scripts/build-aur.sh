#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$PROJECT_DIR/archiso/packages"

choose_work_dir() {
    local parent="$PROJECT_DIR/work"
    if [ -e "$parent" ] && [ ! -w "$parent" ]; then
        WORK_DIR="$(mktemp -d /tmp/churros-aur-build.XXXXXX)"
        return
    fi
    mkdir -p "$parent"
    WORK_DIR="$parent/aur-build"
}

choose_work_dir

echo "======================================"
echo "  Building AUR extras for ChurrOS"
echo "======================================"

mkdir -p "$PACKAGE_DIR" "$WORK_DIR"

build_aur() {
    local name="$1"
    local package_dir="$WORK_DIR/$name"

    if ls "$PACKAGE_DIR"/"$name"-*.pkg.tar.zst 1>/dev/null 2>&1; then
        echo "[skip] $name already built"
        return
    fi

    echo "[build] $name from AUR..."
    rm -rf "$package_dir"
    git clone "https://aur.archlinux.org/${name}.git" "$package_dir"
    (
        cd "$package_dir"
        makepkg -sf --noconfirm --skippgpcheck
    )
    cp "$package_dir"/*.pkg.tar.zst "$PACKAGE_DIR/"
    rm -f "$PACKAGE_DIR"/"$name"-debug-*.pkg.tar.zst 2>/dev/null || true
    echo "[done] $name built"
}

build_aur python-pywal
build_aur yay
build_aur wlogout
build_aur noctalia-qs
build_aur noctalia-shell

echo
echo "Updating churros local repo..."
(
    cd "$PACKAGE_DIR"
    repo-add churros.db.tar.gz *.pkg.tar.zst
)

rm -rf "$WORK_DIR"

echo
echo "======================================"
echo "  AUR extras built."
echo "======================================"
ls -la "$PACKAGE_DIR"/python-pywal-*.pkg.tar.zst 2>/dev/null || echo "(pywal not built)"
ls -la "$PACKAGE_DIR"/yay-*.pkg.tar.zst 2>/dev/null || echo "(yay not built)"
ls -la "$PACKAGE_DIR"/wlogout-*.pkg.tar.zst 2>/dev/null || echo "(wlogout not built)"
ls -la "$PACKAGE_DIR"/noctalia-qs-*.pkg.tar.zst 2>/dev/null || echo "(noctalia-qs not built)"
ls -la "$PACKAGE_DIR"/noctalia-shell-*.pkg.tar.zst 2>/dev/null || echo "(noctalia-shell not built)"
echo
echo "  Run: ./churros build"
