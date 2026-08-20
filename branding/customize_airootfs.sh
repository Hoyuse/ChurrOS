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

echo "Initializing pacman keyring..."
pacman-key --init
pacman-key --populate archlinux

echo "Populating package databases (incl. multilib para Steam)..."
pacman -Sy --noconfirm

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
