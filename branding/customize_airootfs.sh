#!/usr/bin/env bash
set -e

echo "Applying ChurrOS branding..."

cp /root/branding/files/os-release /etc/os-release
cp /root/branding/files/issue /etc/issue
cp /root/branding/files/motd /etc/motd

chmod 644 /etc/os-release
chmod 644 /etc/issue
chmod 644 /etc/motd

echo "Branding applied successfully."

