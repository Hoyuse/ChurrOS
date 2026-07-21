#!/usr/bin/env bash
set -e

echo "==> Creating ChurrOS live user..."

# Crear usuario
useradd -m \
    -G wheel,audio,video,input,storage,network \
    -s /bin/zsh \
    churros

# Sin contraseña para la sesión Live
passwd -d churros

# Sudo sin contraseña
mkdir -p /etc/sudoers.d

echo "churros ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/churros

chmod 440 /etc/sudoers.d/churros

echo "✓ Live user created."