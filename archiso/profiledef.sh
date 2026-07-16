#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="ChurrOS"
iso_label="ChurrOS_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="Hoyuse"
iso_application="ChurrOS Installer"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"
install_dir="churros"
buildmodes=('iso')
bootmodes=('bios.syslinux'
           'uefi.systemd-boot')
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/root"]="0:0:750"

  ["/usr/bin/churros-welcome"]="0:0:755"
  ["/usr/share/churros/churros-welcome/churros-welcome.sh"]="0:0:755"

  ["/usr/local/bin/choose-mirror"]="0:0:755"
  
)