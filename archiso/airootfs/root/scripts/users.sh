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
echo 'Defaults:churros env_keep += "WAYLAND_DISPLAY XDG_RUNTIME_DIR QT_QPA_PLATFORM QT_WAYLAND_DISABLE_WINDOWDECORATION DISPLAY XAUTHORITY XDG_SESSION_TYPE XDG_CURRENT_DESKTOP DBUS_SESSION_BUS_ADDRESS"' >> /etc/sudoers.d/churros

chmod 440 /etc/sudoers.d/churros

echo "✓ Live user created."