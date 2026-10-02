#!/usr/bin/env bash
set -e

echo "======================================="
echo " Configuring ChurrOS Live ISO"
echo "======================================="

#
# Branding
#

echo "Applying ChurrOS branding..."

cp /root/branding/files/os-release /etc/os-release
cp /root/branding/files/os-release /usr/lib/os-release
cp /root/branding/files/issue /etc/issue
cp /root/branding/files/motd /etc/motd

if [ -f /root/branding/VERSION ] && [ -f /root/branding/stamp-os-release.sh ]; then
    ver=$(tr -d '[:space:]' < /root/branding/VERSION)
    bash /root/branding/stamp-os-release.sh /etc/os-release "$ver"
    bash /root/branding/stamp-os-release.sh /usr/lib/os-release "$ver"
fi

chmod 644 /etc/os-release
chmod 644 /usr/lib/os-release
chmod 644 /etc/issue
chmod 644 /etc/motd

echo "✓ Branding applied."

#
# GRUB theme (installed system)
#

echo "Deploying GRUB theme..."
if [ -d /root/branding/grub-theme ]; then
    mkdir -p /usr/share/churros/grub-theme
    cp -r /root/branding/grub-theme/. /usr/share/churros/grub-theme/
    echo "✓ GRUB theme deployed."
else
    echo "  (grub-theme not found — skipped)"
fi

#
# Live environment
#

echo "Creating live user..."
bash /root/scripts/users.sh

echo "Enabling services..."
bash /root/scripts/services.sh

# Ni keyring ni bases de datos: los paquetes ya están instalados por
# pacstrap antes de este script, y Calamares se instala con bsdtar más abajo.
#
# El keyring que se generase aquí se descartaría: en el Live, /etc/pacman.d/gnupg
# se monta como tmpfs al arrancar (etc-pacman.d-gnupg.mount) y lo vuelve a
# crear pacman-init.service. En el sistema instalado lo inicializa el paso
# shellprocess@pacman-init de Calamares. Y un `pacman -Sy` en pleno build solo
# descarga bases de datos que nadie consulta durante la construcción.

# Ensure multilib is enabled in live environment pacman.conf
if [ -f /etc/pacman.conf ] && grep -q '^#\[multilib\]' /etc/pacman.conf; then
    sed -i '/^#\[multilib\]/,/^#Include/ s/^#//' /etc/pacman.conf
fi

echo "Configuring desktop..."
bash /root/scripts/desktop.sh

echo "Installing Calamares..."
if ls /root/packages/calamares-[0-9]*.pkg.tar.zst 1>/dev/null 2>&1; then
    pacman -Scc --noconfirm 2>/dev/null || true

    bsdtar -xf /root/packages/calamares-*.pkg.tar.zst -C /

    rm -f /root/packages/calamares-*.pkg.tar.zst

    cat > /usr/share/applications/calamares.desktop << 'DESKTOP'
[Desktop Entry]
Type=Application
Name=Install ChurrOS
Name[es]=Instalar ChurrOS
GenericName=System Installer
GenericName[es]=Instalador del Sistema
Comment=Install ChurrOS on your computer
Comment[es]=Instala ChurrOS en tu computadora
TryExec=calamares
Exec=/usr/local/bin/calamares
Icon=calamares
Terminal=false
StartupNotify=true
Categories=Qt;System;
DESKTOP

    # Parchear DMgreetd en Calamares para soportar regreet como default_session
    if [ -f /usr/lib/calamares/modules/displaymanager/main.py ]; then
        sed -i 's|elif os.path.exists(self.os_path("usr/bin/tuigreet")):|elif os.path.exists(self.os_path("usr/bin/regreet")) and os.path.exists(self.os_path("usr/bin/cage")):\n            self.config_data["default_session"]["command"] = "env WLR_NO_HARDWARE_CURSORS=1 XCURSOR_THEME=Adwaita XCURSOR_SIZE=24 cage -s -- regreet"\n        elif os.path.exists(self.os_path("usr/bin/tuigreet")):|' /usr/lib/calamares/modules/displaymanager/main.py 2>/dev/null || true
    fi

    echo "✓ Calamares installed."
else
    echo "  (not available — installer skipped)"
fi

echo "Installing Bazaar..."
# Bazaar se instala desde packages.x86_64 via pacstrap (repo local [churros],
# patcheado para fix de libdex). Ya no se usa bsdtar.

if ls /root/packages/*.pkg.tar.zst 1>/dev/null 2>&1; then
    echo "  (paquetes del repo local quedan en /root/packages para Calamares/netinstall)"
fi

echo "Cleaning..."
bash /root/scripts/cleanup.sh

echo ""
echo "======================================="
echo " ChurrOS customization complete."
echo "======================================="
