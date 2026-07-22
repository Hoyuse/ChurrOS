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
cp /root/branding/files/issue /etc/issue
cp /root/branding/files/motd /etc/motd

chmod 644 /etc/os-release
chmod 644 /etc/issue
chmod 644 /etc/motd

echo "✓ Branding applied."

#
# Live environment
#

echo "Creating live user..."
bash /root/scripts/users.sh

echo "Enabling services..."
bash /root/scripts/services.sh

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
Exec=sh -c "sudo -E calamares -d"
Icon=calamares
Terminal=false
StartupNotify=true
Categories=Qt;System;
DESKTOP

    echo "✓ Calamares installed."
else
    echo "  (not available — installer skipped)"
fi

echo "Cleaning..."
bash /root/scripts/cleanup.sh

echo ""
echo "======================================="
echo " ChurrOS customization complete."
echo "======================================="
