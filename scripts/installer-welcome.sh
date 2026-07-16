#!/bin/bash
set -e

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

WELCOME_SRC="$PROJECT_ROOT/apps/churros-welcome"
ISO_ROOT="$PROJECT_ROOT/archiso/airootfs"

echo "==> Instalando ChurrOS Welcome en ArchISO..."

# Crear directorios
mkdir -p "$ISO_ROOT/usr/share/churros"
mkdir -p "$ISO_ROOT/usr/share/applications"

# Limpiar instalación anterior
rm -rf "$ISO_ROOT/usr/share/churros/churros-welcome"

# Copiar aplicación
cp -r "$WELCOME_SRC" \
      "$ISO_ROOT/usr/share/churros/"

# Copiar launcher
cp "$WELCOME_SRC/churros-welcome.desktop" \
   "$ISO_ROOT/usr/share/applications/"

   # Crear carpeta de autostart
mkdir -p "$ISO_ROOT/etc/xdg/autostart"

# Copiar launcher para inicio automático
cp "$WELCOME_SRC/churros-welcome.desktop" \
   "$ISO_ROOT/etc/xdg/autostart/"

echo ""
echo "✅ ChurrOS Welcome instalado correctamente."

