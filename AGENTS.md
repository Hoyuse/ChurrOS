# AGENTS.md — ChurrOS Development Guide

## Build Commands

```bash
./churros build      # Build ISO
./churros run        # Build (if needed) and launch QEMU
./churros clean      # Remove build artifacts (work/, out/)
./churros doctor     # Check build dependencies
./scripts/build-calamares.sh  # Build Calamares from AUR
```

## Testing

```bash
./churros run        # Launch QEMU VM with KVM, 4 cores, 4 GB RAM, UEFI
```

- ISO output: `out/*.iso`
- VM disk: `vm/ChurrOS.qcow2`
- EFI vars: `vm/OVMF_VARS.fd`

## Project Structure

```
archiso/              ArchISO profile (airootfs, packages, boot configs)
  airootfs/           Live system root overlay
    etc/skel/         Skeleton home (niri, waybar, kitty, fuzzel, starship)
    root/scripts/     Live user creation, services, desktop setup
    usr/share/churros/ Control center, launcher, popups, services (Python GTK4)
branding/             Visual identity, customize_airootfs.sh
installer/
  calamares/          Calamares config (settings, modules, branding)
  apply-calamares.sh  Deploys calamares config into airootfs
scripts/
  cli/                CLI subcommands (build, run, clean, doctor, info, version, logo)
  build-calamares.sh  Builds Calamares package from AUR
apps/                 Python GTK4 applications (churros-welcome, churros-ui)
docs/                 Project documentation
```

## Conventions

- Shell scripts: `#!/usr/bin/env bash`, `set -e`, shellcheck-compliant
- Calamares configs: YAML with `.conf` extension in `installer/calamares/modules/`
- Package lists: one per line in `archiso/packages.x86_64`
- Branding assets: `branding/` directory, deployed by `branding/customize_airootfs.sh`
- File permissions: declared in `archiso/profiledef.sh`

## Key Architecture

- **Live user**: `churros` (wheel, audio, video, input, storage, network), NOPASSWD sudo
- **Compositor**: Niri (Wayland scrollable-tiling)
- **Display Manager**: SDDM with autologin to churros/session niri
- **Panel**: Waybar
- **Launcher**: Fuzzel
- **Terminal**: Kitty
- **Apps**: Python GTK4 + Libadwaita
- **Installer**: Calamares 3.4.2 with custom branding (slideshow, QSS stylesheet)
- **Filesystem**: Btrfs with @, @home, @swap subvolumes, compress=zstd:1
- **Boot**: GRUB (UEFI), systemd-boot (UEFI), Syslinux (BIOS)
- **Audio**: PipeWire + WirePlumber
- **Build system**: archiso (mkarchiso)

## Calamares Installer Flow

1. `settings.conf` defines module instances and execution sequence
2. `apply-calamares.sh` copies config from `installer/calamares/` to `archiso/airootfs/etc/calamares/`
3. `build.sh` calls `apply-calamares.sh` before running `mkarchiso`
4. `customize_airootfs.sh` installs calamares package via `pacman -U` on live boot

**Critical**: `shellprocess@pacman-init` (keyring init) MUST run before `shellprocess@fix-boot` (kernel reinstall via pacman) in the exec sequence.

## What NOT to Modify

- `installer/calamares/branding/churros/` (branding, slideshow, stylesheet)
- `branding/` (colors, typography, logo guidelines)
- `archiso/airootfs/etc/skel/.config/` (niri, waybar, kitty, fuzzel, starship themes)
- Bootloader graphics and splash images
