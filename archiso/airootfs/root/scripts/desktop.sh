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

if [ "$EDITION" = "xfce" ]; then
    cat > "$SESSION_FILE" << 'EOF'
XDG_CURRENT_DESKTOP=XFCE
XDG_SESSION_DESKTOP=xfce
XDG_SESSION_TYPE=x11
DESKTOP_SESSION=xfce
EOF
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
        sed -i 's|^Exec=.*|Exec=churros-niri-session|' /usr/share/wayland-sessions/niri.desktop
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