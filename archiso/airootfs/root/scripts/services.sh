#!/usr/bin/env bash
set -e

echo "==> Enabling services..."

systemctl mask getty@tty1.service
systemctl enable NetworkManager.service
systemctl enable greetd.service

echo "✓ Services enabled."