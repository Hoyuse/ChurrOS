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

echo "Cleaning..."
bash /root/scripts/cleanup.sh

echo ""
echo "======================================="
echo " ChurrOS customization complete."
echo "======================================="