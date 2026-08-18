#!/usr/bin/env bash
#
# Write VERSION_ID, VERSION and PRETTY_NAME into an os-release file.
# Usage: stamp-os-release.sh <os-release-path> <version>
set -euo pipefail

file=${1:-}
ver=${2:-}

if [ ! -f "$file" ]; then
    echo "stamp-os-release: file not found: $file" >&2
    exit 1
fi

if [[ ! "$ver" =~ ^[0-9]+(\.[0-9]+)*$ ]]; then
    echo "stamp-os-release: invalid version: $ver" >&2
    exit 1
fi

set_or_insert() {
    local key=$1
    local value=$2
    local after=$3
    if grep -q "^${key}=" "$file"; then
        sed -i "s|^${key}=.*|${key}=${value}|" "$file"
    else
        sed -i "/^${after}=/a ${key}=${value}" "$file"
    fi
}

set_or_insert VERSION_ID "$ver" ID
set_or_insert VERSION "\"${ver}\"" VERSION_ID
set_or_insert PRETTY_NAME "\"ChurrOS ${ver}\"" NAME
