#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$PROJECT_DIR/archiso/packages"

choose_work_dir() {
    local parent="$PROJECT_DIR/work"
    if [ -e "$parent" ] && [ ! -w "$parent" ]; then
        WORK_DIR="$(mktemp -d /tmp/churros-bazaar-build.XXXXXX)"
        return
    fi
    mkdir -p "$parent"
    WORK_DIR="$parent/bazaar-build"
}

choose_work_dir

echo "======================================"
echo "  Building Bazaar app store from Arch"
echo "======================================"

mkdir -p "$PACKAGE_DIR"

if ls "$PACKAGE_DIR"/bazaar-*.pkg.tar.zst 1>/dev/null 2>&1; then
    echo "[skip] Bazaar already built."
    exit 0
fi

rm -rf "$WORK_DIR" 2>/dev/null || true
mkdir -p "$WORK_DIR"

ARCH_TARGET="${ARCH:-$(uname -m)}"
case "$ARCH_TARGET" in
    arm64|aarch64)
        PKGBUILD_URLS=(
            "https://raw.githubusercontent.com/ArchLinuxARM/PKGBUILDs/master/community/bazaar/PKGBUILD"
            "https://gitlab.archlinux.org/archlinux/packaging/packages/bazaar/-/raw/main/PKGBUILD"
        )
        ;;
    *)
        PKGBUILD_URLS=(
            "https://gitlab.archlinux.org/archlinux/packaging/packages/bazaar/-/raw/main/PKGBUILD"
        )
        ;;
esac

echo "[1/4] Fetching Arch PKGBUILD for bazaar (${ARCH_TARGET})..."
for f in PKGBUILD bazaar.install; do
    for url in "${PKGBUILD_URLS[@]}"; do
        if curl -fsSL "${url%/PKGBUILD}/$f" -o "$WORK_DIR/$f" 2>/dev/null; then
            break
        fi
    done
    if [ ! -f "$WORK_DIR/$f" ]; then
        echo "[warn] Could not download $f from the chosen Arch mirror(s)."
        curl -fsSL "https://gitlab.archlinux.org/archlinux/packaging/packages/bazaar/-/raw/main/$f" \
            -o "$WORK_DIR/$f" 2>/dev/null || true
    fi
done

echo "[2/4] Patching PKGBUILD (libdex conflict)..."

python3 - "$WORK_DIR/PKGBUILD" <<'PY'
import sys, re

path = sys.argv[1]
text = open(path, encoding="utf-8").read()

if "provides=" in text and "libdex" in text:
    print("    already patched, skipping")
    sys.exit(0)

# bazaar bundles its own libdex (>= 1.2.beta) via the subprojects wrap file;
# the standalone libdex package in the repos is 1.1.0 and conflicts at file level.
# Drop the dependency and advertise the bundled libdex instead.
text = re.sub(r"\n\s*libdex\b\n", "\n", text)

# bump pkgrel so this patched build always beats the unpatched repo package
text = re.sub(r"^pkgrel=\d+$", "pkgrel=2", text, flags=re.MULTILINE)

text = text.replace(
    "optdepends=('krunner-bazaar: krunner integration')",
    "optdepends=('krunner-bazaar: krunner integration')\n"
    "provides=('libdex')\n"
    "conflicts=('libdex')"
)

open(path, "w", encoding="utf-8").write(text)
print("    libdex moved from depends to provides/conflicts; pkgrel bumped to 2")
PY

echo "[3/4] Building bazaar (this may take a while)..."

(
    cd "$WORK_DIR"
    makepkg -sf --noconfirm --skippgpcheck
)

echo "[4/4] Installing package to local repo..."

cp "$WORK_DIR"/*.pkg.tar.zst "$PACKAGE_DIR/"
rm -f "$PACKAGE_DIR"/bazaar-debug-*.pkg.tar.zst 2>/dev/null || true

(
    cd "$PACKAGE_DIR"
    repo-add churros.db.tar.gz *.pkg.tar.zst
)

rm -rf "$WORK_DIR"

echo
echo "======================================"
echo "  Bazaar build complete."
echo "======================================"
ls -la "$PACKAGE_DIR"/bazaar-*.pkg.tar.zst
echo
echo "  Now run: ./churros build"