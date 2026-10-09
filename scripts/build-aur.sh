#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
# shellcheck source=scripts/lib/local-repo.sh
source "$SCRIPT_DIR/lib/local-repo.sh"
PACKAGE_DIR="$(churros_local_repo_dir)"

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

    if [ -n "$(churros_first_pkg "$PACKAGE_DIR" "$name-*" || true)" ]; then
        echo "[skip] $name already built"
        return
    fi

    echo "[build] $name from AUR..."
    rm -rf "$package_dir"
    git clone "https://aur.archlinux.org/${name}.git" "$package_dir"
    (
        cd "$package_dir"
        churros_pkgbuild_allow_arch PKGBUILD
        makepkg -sf --noconfirm --skippgpcheck
    )
    churros_copy_pkgs "$package_dir" "$PACKAGE_DIR" "$name-*"
    churros_remove_pkgs "$PACKAGE_DIR" "$name-debug-*"
    echo "[done] $name built"
}

build_aur python-pywal
build_aur yay
build_aur wlogout

echo
echo "Updating churros local repo..."
(
    cd "$PACKAGE_DIR"
    churros_repo_add "$PACKAGE_DIR"
)

rm -rf "$WORK_DIR"

echo
echo "======================================"
echo "  AUR extras built."
echo "======================================"
ls -la "$PACKAGE_DIR"/python-pywal-*.pkg.tar.zst 2>/dev/null || echo "(pywal not built)"
ls -la "$PACKAGE_DIR"/yay-*.pkg.tar.zst 2>/dev/null || echo "(yay not built)"
ls -la "$PACKAGE_DIR"/wlogout-*.pkg.tar.zst 2>/dev/null || echo "(wlogout not built)"
echo
echo "  Run: ./churros build"
