#!/usr/bin/env bash
#
# build-churros-release.sh — genera el bundle de utilidades de ChurrOS
# (churros-utils-<version>.tar.zst) y su updates.json para publicar en
# el servidor de actualizaciones.
#
# Uso:
#   ./scripts/build-churros-release.sh
#
# Produce en release/:
#   churros-utils-<version>.tar.zst
#   updates.json
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
RUST_DIR="$PROJECT_DIR/rust"
AIROOTFS="$PROJECT_DIR/archiso/airootfs"
VERSION="$(cat "$PROJECT_DIR/VERSION")"
OUT="$PROJECT_DIR/release"
STAGE="$(mktemp -d)"
BUNDLE="churros-utils-${VERSION}.tar.zst"

echo "==> ChurrOS release build — v${VERSION}"

# 1. Compilar las apps Rust (release)
echo "  [1/4] compilando apps Rust..."
cargo build --release --manifest-path "$RUST_DIR/Cargo.toml" --jobs "$(nproc)"

# 2. Staging: binarios Rust (solo crates con deploy = true)
echo "  [2/4] montando staging..."
mkdir -p "$STAGE/usr/bin" "$STAGE/usr/local/bin" "$STAGE/etc"
for crate_dir in "$RUST_DIR"/*/; do
    [ -f "$crate_dir/Cargo.toml" ] || continue
    grep -q '^deploy = true$' "$crate_dir/Cargo.toml" || continue
    crate_name=$(sed -n 's/^name = "\(.*\)"/\1/p' "$crate_dir/Cargo.toml" | head -1)
    [ -n "$crate_name" ] || continue
    binary="$RUST_DIR/target/release/$crate_name"
    if [ -x "$binary" ]; then
        cp "$binary" "$STAGE/usr/bin/$crate_name"
        echo "    + usr/bin/$crate_name"
    fi
done

# 3. Scripts de la ISO (usr/bin y usr/local/bin)
for s in churros-apply-wallpaper churros-pick-image churros-pkexec churros-portal-start churros-update-utils; do
    if [ -f "$AIROOTFS/usr/bin/$s" ]; then
        cp "$AIROOTFS/usr/bin/$s" "$STAGE/usr/bin/$s"
        chmod 755 "$STAGE/usr/bin/$s"
        echo "    + usr/bin/$s"
    fi
done
for s in churros-theme churros-update-auto; do
    if [ -f "$AIROOTFS/usr/local/bin/$s" ]; then
        cp "$AIROOTFS/usr/local/bin/$s" "$STAGE/usr/local/bin/$s"
        chmod 755 "$STAGE/usr/local/bin/$s"
        echo "    + usr/local/bin/$s"
    fi
done

# 4. Assets de /usr/share/churros (estilos, defaults, wallpapers, set-*)
if [ -d "$AIROOTFS/usr/share/churros" ]; then
    mkdir -p "$STAGE/usr/share"
    cp -r "$AIROOTFS/usr/share/churros" "$STAGE/usr/share/churros"
    echo "    + usr/share/churros"
fi

# 5. Versión instalada (se autoactualiza al extraer el bundle)
echo "$VERSION" > "$STAGE/etc/churros-version"

# 6. Empaquetar
echo "  [3/4] empaquetando $BUNDLE..."
mkdir -p "$OUT"
tar --zstd -cf "$OUT/$BUNDLE" -C "$STAGE" usr etc

# 7. updates.json (manifiesto con versión + sha256)
SHA=$(sha256sum "$OUT/$BUNDLE" | awk '{print $1}')
DATE=$(date +%Y-%m-%d)
cat > "$OUT/updates.json" <<EOF
{
  "version": "$VERSION",
  "date": "$DATE",
  "file": "$BUNDLE",
  "sha256": "$SHA"
}
EOF

rm -rf "$STAGE"

echo "  [4/4] listo:"
echo "    $OUT/$BUNDLE"
echo "    $OUT/updates.json"
echo
echo "  Sube ambos a: https://download.churroslinux.org/churros/"
