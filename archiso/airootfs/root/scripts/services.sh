#!/usr/bin/env bash
set -e

echo "==> Enabling services..."

systemctl mask getty@tty1.service
systemctl mask plymouth-start.service 2>/dev/null || true
systemctl mask plymouth-quit.service 2>/dev/null || true
systemctl mask plymouth-quit-wait.service 2>/dev/null || true
systemctl mask systemd-time-wait-sync.service 2>/dev/null || true
systemctl mask NetworkManager-wait-online.service 2>/dev/null || true
systemctl mask systemd-networkd-wait-online.service 2>/dev/null || true

systemctl enable NetworkManager.service
systemctl enable greetd.service
systemctl enable ufw.service

echo "✓ Services enabled."