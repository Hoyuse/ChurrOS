#!/usr/bin/env bash
# shellcheck disable=SC2034

export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(date +%s)}"

iso_name="ChurrOS"
iso_label="ChurrOS_$(date --date="@${SOURCE_DATE_EPOCH}" +%Y%m)"
iso_publisher="Hoyuse"
iso_application="ChurrOS Installer"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH}" +%Y.%m.%d)"
install_dir="churros"
buildmodes=('iso')
bootmodes=('bios.syslinux'
           'uefi.grub')
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/root"]="0:0:750"

  ["/root/scripts/greetd-config.sh"]="0:0:755"

  ["/usr/bin/churros-welcome"]="0:0:755"

  ["/usr/bin/churros-popup"]="0:0:755"

  ["/usr/local/bin/choose-mirror"]="0:0:755"
  ["/usr/local/bin/churros-theme"]="0:0:755"
["/usr/local/bin/calamares"]="0:0:755"
  ["/usr/local/bin/churros-update-auto"]="0:0:755"
  ["/usr/bin/churros-update-utils"]="0:0:755"
  ["/usr/bin/churros-settings"]="0:0:755"
  ["/usr/bin/churros-control-center"]="0:0:755"
  ["/usr/bin/churros-pick-image"]="0:0:755"
  ["/usr/bin/churros-pkexec"]="0:0:755"
  ["/usr/bin/churros-portal-start"]="0:0:755"
  ["/usr/bin/churros-apply-wallpaper"]="0:0:755"
  ["/usr/share/churros/scripts/set-accent"]="0:0:755"
  ["/usr/share/churros/scripts/set-theme"]="0:0:755"
  ["/usr/share/churros/scripts/set-wallpaper"]="0:0:755"
  ["/usr/share/churros/scripts/set-cursor"]="0:0:755"
  ["/usr/share/churros/scripts/set-icons"]="0:0:755"
  ["/usr/share/churros/scripts/make-boot-grub-readable"]="0:0:755"
  ["/usr/share/icons/hicolor/scalable/apps/churros-welcome.svg"]="0:0:644"
  ["/usr/share/icons/hicolor/scalable/apps/churros-settings.svg"]="0:0:644"
  ["/usr/share/icons/hicolor/128x128/apps/churros-welcome.png"]="0:0:644"
  ["/usr/share/icons/hicolor/128x128/apps/churros-settings.png"]="0:0:644"
)
