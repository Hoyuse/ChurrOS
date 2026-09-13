#!/usr/bin/env bash
set -e

echo "==> Creating ChurrOS live user..."

# Crear grupos si no existen
groupadd -f autologin

# Crear usuario
useradd -m \
    -G wheel,audio,video,input,storage,network,autologin \
    -s /bin/zsh \
    churros

# Sin contraseña para la sesión Live
passwd -d churros

# Sudo sin contraseña
mkdir -p /etc/sudoers.d

echo "churros ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/churros
echo 'Defaults:churros env_keep += "WAYLAND_DISPLAY XDG_RUNTIME_DIR QT_QPA_PLATFORM QT_WAYLAND_DISABLE_WINDOWDECORATION DISPLAY XAUTHORITY XDG_SESSION_TYPE XDG_CURRENT_DESKTOP DBUS_SESSION_BUS_ADDRESS"' >> /etc/sudoers.d/churros

chmod 440 /etc/sudoers.d/churros

# Permisos para el usuario greeter de greetd si existe
if id greeter >/dev/null 2>&1; then
    usermod -a -G video,input,render,tty greeter 2>/dev/null || true
fi

echo "✓ Live user created."