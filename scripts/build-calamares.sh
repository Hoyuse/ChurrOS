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

HOST_PYTHON="$(python3 -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}")')"

if ls "$PACKAGE_DIR"/calamares-*.pkg.tar.zst 1>/dev/null 2>&1; then
    PKG="$(ls "$PACKAGE_DIR"/calamares-*.pkg.tar.zst | head -1)"
    TMP_DIR="$(mktemp -d)"
    bsdtar -xf "$PKG" -C "$TMP_DIR" usr/lib/libcalamares.so.3.4.2 2>/dev/null || \
        bsdtar -xf "$PKG" -C "$TMP_DIR" usr/lib/libcalamares.so 2>/dev/null || true
    PKG_PYTHON="$(ldd "$TMP_DIR"/usr/lib/libcalamares.so* 2>/dev/null \
        | sed -n 's/.*libpython\([0-9.]*\)\.so.*/\1/p' | head -1)"
    rm -rf "$TMP_DIR"
    if [ "$PKG_PYTHON" != "$HOST_PYTHON" ]; then
        echo "[rebuild] Calamares links Python ${PKG_PYTHON:-<none>} but host has $HOST_PYTHON."
        echo "          Rebuilding to link the matching libpython."
    else
        echo "[skip] Calamares already built (Python $PKG_PYTHON matches host)."
        exit 0
    fi
fi

rm -rf "$WORK_DIR" 2>/dev/null || true

echo "[1/4] Cloning calamares from AUR..."
git clone https://aur.archlinux.org/calamares.git "$WORK_DIR"

echo "[2/4] Applying netinstall UI patch..."

PATCH_FILE="$PROJECT_DIR/installer/patches/calamares-netinstall-columns.patch"
cp "$PATCH_FILE" "$WORK_DIR/calamares-netinstall-columns.patch"

PATCH_SHA="$(sha256sum "$PATCH_FILE" | cut -d' ' -f1)"

python3 - "$WORK_DIR/PKGBUILD" "$PATCH_SHA" <<'PY'
import sys

path, sha = sys.argv[1], sys.argv[2]

lines = open(path, encoding="utf-8").read().splitlines()
out = []
inserted = False

for line in lines:
    if line.startswith("source=("):
        line = line.rstrip(")") + " 'calamares-netinstall-columns.patch')"
    elif line.startswith("sha256sums=("):
        line = line.rstrip(")") + " '" + sha + "')"
    elif line.startswith("build() {") and not inserted:
        out.append("prepare() {")
        out.append("    patch -Np1 -d \"$_pkgname-$pkgver\" -i \"$srcdir/calamares-netinstall-columns.patch\"")
        out.append("}")
        out.append("")
        inserted = True
    out.append(line)

open(path, "w", encoding="utf-8").write("\n".join(out) + "\n")
PY

echo "    PKGBUILD patched (source + prepare + checksum)"

echo "[3/4] Pre-downloading calamares source (codeberg can be flaky)..."

PKGVER="$(sed -n 's/^pkgver=//p' "$WORK_DIR/PKGBUILD")"
SRC_URL="https://codeberg.org/Calamares/calamares/releases/download/v$PKGVER/calamares-$PKGVER.tar.gz"
SRC_FILE="$WORK_DIR/calamares-$PKGVER.tar.gz"
EXPECTED_SHA="$(sed -n 's/.*sha256sums=('"'"'\([0-9a-f]*\)'"'"'.*/\1/p' "$WORK_DIR/PKGBUILD")"

if [[ -n "${SRCDEST:-}" ]] && [[ -s "$SRCDEST/calamares-$PKGVER.tar.gz" ]] \
    && [[ "$(sha256sum "$SRCDEST/calamares-$PKGVER.tar.gz" | cut -d' ' -f1)" == "$EXPECTED_SHA" ]]; then
    echo "    Using cached source from SRCDEST..."
    cp "$SRCDEST/calamares-$PKGVER.tar.gz" "$SRC_FILE"
fi

for attempt in 1 2 3 4 5; do
    if [[ -s "$SRC_FILE" ]] && [[ "$(sha256sum "$SRC_FILE" | cut -d' ' -f1)" == "$EXPECTED_SHA" ]]; then
        echo "    Source already present and valid (attempt $attempt)."
        break
    fi
    echo "    Downloading source (attempt $attempt)..."
    rm -f "$SRC_FILE"
    curl -fsSL --http1.1 --connect-timeout 30 --max-time 900 -o "$SRC_FILE" "$SRC_URL"
    if [[ "$(sha256sum "$SRC_FILE" | cut -d' ' -f1)" != "$EXPECTED_SHA" ]]; then
        echo "    WARNING: checksum mismatch, retrying..."
        rm -f "$SRC_FILE"
        continue
    fi
    break
done

if [[ "$(sha256sum "$SRC_FILE" | cut -d' ' -f1)" != "$EXPECTED_SHA" ]]; then
    echo "ERROR: could not download a valid calamares source tarball." >&2
    exit 1
fi
echo "    Source ready: $SRC_FILE ($(du -h "$SRC_FILE" | cut -f1))"

echo "[4/4] Building calamares (this may take a while)..."

(
    cd "$WORK_DIR"
    makepkg -sf --noconfirm
)

echo "    makepkg done."

echo "[5/5] Installing package to local repo..."

cp "$WORK_DIR"/*.pkg.tar.zst "$PACKAGE_DIR/"
rm -f "$PACKAGE_DIR"/calamares-debug-*.pkg.tar.zst 2>/dev/null || true

(
    cd "$PACKAGE_DIR"
    repo-add churros.db.tar.gz *.pkg.tar.zst
)

rm -rf "$WORK_DIR"

echo
echo "======================================"
echo "  Calamares build complete."
echo "======================================"
echo "  Packages:"
ls -la "$PACKAGE_DIR"/*.pkg.tar.zst
echo
echo "  Now run: ./churros build"
