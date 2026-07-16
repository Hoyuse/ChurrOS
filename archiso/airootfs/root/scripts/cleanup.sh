#!/usr/bin/env bash
set -e

echo "==> Cleanup..."

rm -rf /var/cache/pacman/pkg/*

echo "✓ Cleanup complete."