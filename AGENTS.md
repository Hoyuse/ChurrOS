# AGENTS.md — ChurrOS Development Guide

## Build Commands

```bash
./churros build      # Build ISO (runs from repo root; needs sudo for mkarchiso)
./churros run        # Build (if needed) and launch QEMU
./churros run --nokvm  # Force software emulation (no /dev/kvm)
./churros clean      # Remove work/ and out/ (also runs sudo rm -rf)
./churros check      # Static checks: bash, python, package list, niri autostart, po files
./churros doctor     # Check for mkarchiso, qemu, xorriso, mksquashfs, mcopy, mkinitcpio
./scripts/build-calamares.sh  # Build Calamares .pkg.tar.zst from AUR into archiso/packages/
./scripts/build-aur.sh        # Build python-pywal + waypaper + yay AUR packages
```

The `churros` dispatcher is at repo root and `cd`s to its own dir before delegating to `scripts/cli/<cmd>.sh`.

## Build Flow (scripts/cli/build.sh)

Five ordered steps, runs from repo root:

1. Copy `branding/customize_airootfs.sh` + `branding/files/` into `archiso/airootfs/root/`.
2. Build missing local packages: `scripts/build-calamares.sh`, `scripts/build-aur.sh`. Expect `calamares-*.pkg.tar.zst`, `python-pywal-*.pkg.tar.zst`, `waypaper-*.pkg.tar.zst`, `yay-*.pkg.tar.zst` in `archiso/packages/`.
3. If Calamares pkg exists: run `installer/apply-calamares.sh` (deploys `settings.conf`, `modules/*.conf`, `modules/*.yaml`, `branding/churros/`, plus a polkit rule `49-calamares.rules` allowing user `churros` to pkexec calamares) and copy all `archiso/packages/*.pkg.tar.zst` into `airootfs/root/packages/`.
4. `sudo rm -rf work out` then `sudo mkarchiso -v -w work -o out archiso`.
5. `rm -rf work` and `chown` `out/` back to `$USER`.

A trap on EXIT cleans generated files out of `archiso/airootfs/` (`root/customize_airootfs.sh`, `root/branding`, `root/packages`, `etc/calamares`, `polkit-1/rules.d/49-calamares.rules`). Do not edit those paths directly — they are regenerated each build.

## Testing

There are no unit tests yet. Two layers of verification exist today.

`./churros check` runs the static checks (`scripts/cli/check.sh`): bash syntax, shellcheck at error level, Python syntax, duplicate entries in `packages.x86_64`, commands spawned by niri that resolve to a binary or package, and `msgfmt --check` on `po/*.po`. It needs no ISO build and runs in seconds. The same script runs in CI (`.github/workflows/ci.yml`) on every push to `main` and every pull request.

Behaviour on the live system is verified in QEMU:

```bash
./churros run
```

- ISO output: `out/*.iso`
- VM disk: `vm/ChurrOS.qcow2` (64G qcow2, created on first run)
- EFI vars: `vm/OVMF_VARS.fd` (copied from `/usr/share/edk2/x64/OVMF_VARS.4m.fd`)
- Serial log: `vm_serial.log` (in root, gitignored)
- 4 GB RAM, 4 cores + `-cpu host` with KVM (2 cores, plain q35 without). niri needs 3D: build.sh picks `virtio-vga-gl` if `/dev/dri` exists, else `virtio-gpu` (niri falls back to llvmpipe).

## Project Layout

```
churros                       Bash dispatcher -> scripts/cli/<cmd>.sh
scripts/
  cli/                        build.sh, run.sh, clean.sh, doctor.sh, info.sh, version.sh, logo.sh
  build-calamares.sh          Produces archiso/packages/calamares-*.pkg.tar.zst
  build-aur.sh                Produces python-pywal + waypaper pkgs
archiso/                      ArchISO profile root
  profiledef.sh               iso metadata, bootmodes, file_permissions map
  packages/                   Local pacman repo (built pkgs + repo db live here)
  airootfs/                   Squashfs root overlay
    etc/skel/.config/          niri, waybar, foot, fuzzel — DO NOT MODIFY
    root/scripts/             Live-ISO runtime scripts (users, services, desktop, cleanup, greetd-config)
    usr/share/churros/        Python GTK4/Libadwaita apps + scripts
branding/                     Visual identity
  customize_airootfs.sh       Runs at live boot: applies os-release/issue/motd, creates live user, installs Calamares via bsdtar, configures local [churros] pacman repo
  files/                      os-release, issue, motd, logos, wallpapers
installer/
  calamares/settings.conf     Instance + sequence definition (see below)
  calamares/modules/*.conf    One .conf per Calamares module
  calamares/modules/*.yaml    netinstall package groups
  apply-calamares.sh          Copies config + polkit rule into airootfs
docs/                         Project documentation
```

## Conventions

- Shell scripts: `#!/usr/bin/env bash`, `set -e`, shellcheck-compliant.
- Calamares modules: `.conf` (and `.yaml` for netinstall) in `installer/calamares/modules/`.
- Package lists: one package per line in `archiso/packages.x86_64`.
- File mode map (not git): declared in `archiso/profiledef.sh` `file_permissions` (e.g. `/usr/bin/churros-*` 0755).
- Bootstrap uses `pacman.conf` (declared in `profiledef.sh`); airootfs compressed squashfs xz; bootstrap tarball zstd.

## Calamares Sequence

`installer/calamares/settings.conf` defines three `shellprocess` instances and the exec order. The exec sequence IS order-sensitive — keypin requirements:

- `shellprocess@pacman-init` (keyring init) **MUST** come before `shellprocess@fix-boot` (kernel reinstall via pacman) — both already ordered this way; do not reorder.
- `shellprocess@fix-boot` runs before `netinstall`/`packages` install additional packages.
- `shellprocess@post-install` (cleanup) is the last exec step before `umount`.

Config files per instance: `shellprocess-pacman.conf`, `shellprocess-fixboot.conf`, `shellprocess-cleanup.conf`. Module IDs in `instances:` are `pacman-init`, `fix-boot`, `post-install`.

## Key Architecture

- **Live user**: `churros` (wheel, audio, video, input, storage, network), NOPASSWD sudo — created by `archiso/airootfs/root/scripts/users.sh`.
- **Compositor**: Niri (Wayland scrollable-tiling). Requires 3D accel in QEMU (see Testing).
- **Display Manager**: greetd with autologin to `churros` / `niri` session.
- **Panel/Launcher/Terminal**: Waybar / Fuzzel / foot.
- **Apps**: Python GTK4 + Libadwaita in `archiso/airootfs/usr/share/churros/` (`churros-welcome`, `control-center`, `popups`, `preferences`, `services`), installed into `/usr/bin/churros-*` via `profiledef.sh` perms.
- **Installer**: Calamares with custom `churros` branding (slideshow, QSS stylesheet).
- **Boot modes** (from `profiledef.sh`): `bios.syslinux` + `uefi.systemd-boot`. No GRUB.
- **Audio**: PipeWire + WirePlumber.
- **Build system**: archiso (`mkarchiso`).

## What NOT to Modify

- `installer/calamares/branding/churros/` — branding, slideshow, QSS.
- `branding/` — colors, typography, logo guidelines, mascot.
- `archiso/airootfs/etc/skel/.config/` — niri, waybar, foot, fuzzel themes.
- Bootloader graphics and splash images.

## Notes

- `customize_airootfs.sh` lives in `branding/`, not in `archiso/`. It is copied into `airootfs/root/` on every build; editing the copy has no effect.
- `scripts/build-calamares.sh` and `scripts/build-aur.sh` run on the host (Arch Linux assumed) and produce pacman packages in `archiso/packages/`. They are skipped by `build.sh` if matching pkgs already exist.
- README and most `docs/*.md` are in Spanish; code and shell scripts are in English.
