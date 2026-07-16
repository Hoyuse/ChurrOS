#!/usr/bin/env bash
set -e

echo "==> Configuring desktop..."

# Copiar configuración por defecto
cp -r /etc/skel/.config /home/churros/

# Permisos
chown -R churros:churros /home/churros/.config

echo "✓ Desktop configured."