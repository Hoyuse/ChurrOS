#!/usr/bin/env bash
set -euo pipefail

PACKAGE_DIR="${1:-archiso/packages}"
REQUIRED_PACKAGES=(python-pywal yay wlogout noctalia-qs noctalia-shell)

for package in "${REQUIRED_PACKAGES[@]}"; do
    if ! compgen -G "$PACKAGE_DIR/$package-*.pkg.tar.zst" >/dev/null; then
        printf '%s\n' "$package"
    fi
done
