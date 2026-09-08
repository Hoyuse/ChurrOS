#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ARCH_TARGET="${ARCH:-$(uname -m)}"
case "$ARCH_TARGET" in
    arm64|aarch64)
        ARCH_TARGET="aarch64"
        ;;
    *)
        ARCH_TARGET="x86_64"
        ;;
esac

CALAMARES_SRC="$SCRIPT_DIR/calamares"
CALAMARES_DST="$PROJECT_DIR/archiso/airootfs/etc/calamares"

if [ ! -f "$CALAMARES_SRC/settings.conf" ]; then
    echo "Error: $CALAMARES_SRC/settings.conf not found." >&2
    exit 1
fi

echo "======================================"
echo "  Applying Calamares config to ISO (${ARCH_TARGET})"
echo "======================================"

mkdir -p "$CALAMARES_DST"

cp "$CALAMARES_SRC/settings.conf" "$CALAMARES_DST/"

mkdir -p "$CALAMARES_DST/branding/churros"
cp -r "$CALAMARES_SRC/branding/churros/"* "$CALAMARES_DST/branding/churros/"

mkdir -p "$CALAMARES_DST/modules"
if [ "$ARCH_TARGET" = "aarch64" ]; then
    for file in "$CALAMARES_SRC/modules/"*.conf "$CALAMARES_SRC/modules/"*.yaml; do
        [ -e "$file" ] || continue
        base="$(basename "$file")"
        case "$base" in
            unpackfs.conf)
                cp "$CALAMARES_SRC/modules/unpackfs-aarch64.conf" "$CALAMARES_DST/modules/unpackfs.conf"
                ;;
            shellprocess-fixboot.conf)
                cp "$CALAMARES_SRC/modules/shellprocess-fixboot-aarch64.conf" "$CALAMARES_DST/modules/shellprocess-fixboot.conf"
                ;;
            shellprocess-pacman.conf)
                cp "$CALAMARES_SRC/modules/shellprocess-pacman-aarch64.conf" "$CALAMARES_DST/modules/shellprocess-pacman.conf"
                ;;
            *)
                cp "$file" "$CALAMARES_DST/modules/"
                ;;
        esac
    done
else
    cp "$CALAMARES_SRC/modules/"*.conf "$CALAMARES_DST/modules/"
    cp "$CALAMARES_SRC/modules/"*.yaml "$CALAMARES_DST/modules/"
fi

echo "  Calamares config applied."
echo
echo "======================================"
echo "  Deploying PolicyKit rules"
echo "======================================"

POLKIT_DST="$PROJECT_DIR/archiso/airootfs/etc/polkit-1/rules.d"
mkdir -p "$POLKIT_DST"

cat > "$POLKIT_DST/49-calamares.rules" << 'POLKIT'
polkit.addRule(function(action, subject) {
    if (action.id == "io.calamares.calamares.pkexec.run" &&
        subject.user == "churros") {
        return polkit.Result.YES;
    }
});
POLKIT

echo "  PolicyKit rules deployed."

echo
echo "======================================"
echo "  Calamares integration complete."
echo "======================================"
