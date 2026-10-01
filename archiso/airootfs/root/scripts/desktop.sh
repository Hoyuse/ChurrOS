#!/usr/bin/env bash
set -e

echo "==> Configuring desktop..."

# Copiar configuración por defecto
cp -r /etc/skel/.config /home/churros/

# Permisos
chown -R churros:churros /home/churros/.config

# Setear variables de sesion segun la edicion (Niri / XFCE)
EDITION="niri"
if [ -f /etc/churros-edition ]; then
    EDITION="$(tr -d '[:space:]' < /etc/churros-edition | tr '[:upper:]' '[:lower:]')"
fi

SESSION_FILE="/home/churros/.config/environment.d/churros-session.conf"
mkdir -p "/home/churros/.config/environment.d"

if [ "$EDITION" = "xfce" ] || [ "$EDITION" = "server" ]; then
    cat > "$SESSION_FILE" << 'EOF'
XDG_CURRENT_DESKTOP=XFCE
XDG_SESSION_DESKTOP=xfce
XDG_SESSION_TYPE=x11
DESKTOP_SESSION=xfce
EOF

    # Asegurar que la sesión X11 ejecute el wrapper para iniciar Xorg desde greetd
    if [ -f /usr/share/xsessions/xfce.desktop ]; then
        sed -i 's|^Exec=.*|Exec=churros-xfce-session|' /usr/share/xsessions/xfce.desktop
    fi

    # Asegurar que el fondo de ChurrOS se aplique incluso en monitores no preconfigurados por XFCE
    mkdir -p /usr/share/backgrounds/xfce
    ln -sf /usr/share/churros/wallpapers/default.png /usr/share/backgrounds/xfce/xfce-shapes.svg
    ln -sf /usr/share/churros/wallpapers/default.png /usr/share/backgrounds/xfce/xfce-verticals.png
    ln -sf /usr/share/churros/wallpapers/default.png /usr/share/backgrounds/xfce/xfce-teal.jpg
    ln -sf /usr/share/churros/wallpapers/default.png /usr/share/backgrounds/xfce/xfce-stripes.png
elif [ "$EDITION" = "kde" ]; then
    cat > "$SESSION_FILE" << 'EOF'
XDG_CURRENT_DESKTOP=KDE
XDG_SESSION_DESKTOP=plasma
XDG_SESSION_TYPE=wayland
DESKTOP_SESSION=plasma
EOF

    # Plasma arranca con su propio lanzador (startplasma-wayland); lo unico que
    # hace falta es dejar el fondo de ChurrOS entre los predeterminados para
    # que aparezca en la primera sesion.
    mkdir -p /usr/share/backgrounds/xdg
    cp -f /usr/share/churros/wallpapers/default.png /usr/share/backgrounds/xdg/churros.png
else
    cat > "$SESSION_FILE" << 'EOF'
XDG_CURRENT_DESKTOP=niri
XDG_SESSION_DESKTOP=niri
XDG_SESSION_TYPE=wayland
XCURSOR_THEME=Adwaita
XCURSOR_SIZE=24
EOF

    # Asegurar que la sesión Wayland ejecute directamente churros-niri-session en el scope de LightDM
    if [ -f /usr/share/wayland-sessions/niri.desktop ]; then
        sed -i 's|^Exec=.*|Exec=/usr/bin/churros-niri-session|' /usr/share/wayland-sessions/niri.desktop
    fi
fi
chown -R churros:churros "/home/churros/.config/environment.d"

# Regenerar cache de iconos GTK para que encuentre los iconos Churros
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    for theme_dir in /usr/share/icons/hicolor /usr/share/icons/Adwaita; do
        if [ -d "$theme_dir" ]; then
            gtk-update-icon-cache -f -t "$theme_dir" 2>/dev/null || true
        fi
    done
fi

echo "✓ Desktop configured."