#!/usr/bin/env bash
set -e

echo "==> Enabling services..."

systemctl enable NetworkManager.service

systemctl enable sddm.service

echo "✓ Services enabled."