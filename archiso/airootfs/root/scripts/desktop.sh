#!/usr/bin/env bash
set -e

echo "==> Configuring desktop..."

# Copiar configuración por defecto
cp -r /etc/skel/.config /home/churros/

# Permisos
chown -R churros:churros /home/churros/.config



# Setear XDG_CURRENT_DESKTOP para que los servicios de preferences detecten Niri
SESSION_FILE="/home/churros/.config/environment.d/churros-session.conf"
mkdir -p "/home/churros/.config/environment.d"
cat > "$SESSION_FILE" << 'EOF'
XDG_CURRENT_DESKTOP=niri
XDG_SESSION_DESKTOP=niri
XDG_SESSION_TYPE=wayland
EOF
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