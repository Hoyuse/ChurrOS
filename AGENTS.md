# AGENTS.md — ChurrOS Development Guide

## Build Commands

```bash
./churros build      # Build ISO (runs from repo root; needs sudo for mkarchiso)
./churros run        # Build (if needed) and launch QEMU
./churros run --nokvm  # Force software emulation (no /dev/kvm)
./churros run --fresh  # Reset OVMF_VARS.fd so UEFI boots from CD-ROM instead of an existing install
./churros clean      # Remove work/ and out/ (also runs sudo rm -rf)
./churros check      # Static checks: bash, python, package list, niri autostart, po files
./churros doctor     # Check for mkarchiso, qemu, xorriso, mksquashfs, mcopy, mkinitcpio
./scripts/build-calamares.sh  # Build Calamares .pkg.tar.zst from AUR into archiso/packages/
./scripts/build-aur.sh        # Build python-pywal + waypaper + yay AUR packages
./scripts/build-grub-theme.sh # Regenerate GRUB theme fonts (.pf2) + assets in branding/grub-theme/
```

The `churros` dispatcher is at repo root and `cd`s to its own dir before delegating to `scripts/cli/<cmd>.sh`.

## Build Flow (scripts/cli/build.sh)

Five ordered steps, runs from repo root:

1. Copy `branding/customize_airootfs.sh` + `branding/files/` into `archiso/airootfs/root/`.
2. Build missing local packages: `scripts/build-calamares.sh`, `scripts/build-aur.sh`. Expect `calamares-*.pkg.tar.zst`, `python-pywal-*.pkg.tar.zst`, `waypaper-*.pkg.tar.zst`, `yay-*.pkg.tar.zst` in `archiso/packages/`.
3. If Calamares pkg exists: run `installer/apply-calamares.sh` (deploys `settings.conf`, `modules/*.conf`, `modules/*.yaml`, `branding/churros/`, plus a polkit rule `49-calamares.rules` allowing user `churros` to pkexec calamares) and copy all `archiso/packages/*.pkg.tar.zst` into `airootfs/root/packages/`.
4. Run `scripts/build-rust.sh`: compiles every crate in `rust/` (release) and deploys binaries into `archiso/airootfs/usr/bin/`. Binary names match crate names (e.g. `churros-welcome`).
5. `sudo rm -rf work out` then `sudo mkarchiso -v -w work -o out archiso`.
6. `rm -rf work` and `chown` `out/` back to `$USER`.

A trap on EXIT cleans generated files out of `archiso/airootfs/` (`root/customize_airootfs.sh`, `root/branding`, `root/packages`, `etc/calamares`, `polkit-1/rules.d/49-calamares.rules`, `usr/bin/churros-welcome`). Do not edit those paths directly — they are regenerated each build.

## Testing

There are no unit tests yet. Two layers of verification exist today.

`./churros check` runs the static checks (`scripts/cli/check.sh`): bash syntax, shellcheck at error level, Python syntax, duplicate entries in `packages.x86_64`, commands spawned by niri that resolve to a binary/crate/package, desktop `Exec`/`TryExec` resolution, Calamares exec order and shellprocess configs, local AUR extras listed in `netinstall.yaml`, and `msgfmt --check` on `po/*.po`. It needs no ISO build and runs in seconds. The same script runs in CI (`.github/workflows/ci.yml`) on every push to `main` and every pull request.

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
rust/                         Rust workspace (apps portadas a gtk4-rs/libadwaita)
  churros-welcome/            Crate de la app de bienvenida (port completo)
  preferences/                Crate de ajustes (binario churros-settings)
  services/                   Crate de servicios (wpctl, nmcli, bluetoothctl, brightnessctl…)
  popups/                     Crate de los popups (binario churros-popup + toggle nativo)
  control-center/             Crate del control center (binario churros-control-center)
scripts/
  cli/                        build.sh, run.sh, clean.sh, doctor.sh, info.sh, version.sh, logo.sh
  build-calamares.sh          Produces archiso/packages/calamares-*.pkg.tar.zst
  build-aur.sh                Produces python-pywal + waypaper + yay pkgs
  build-rust.sh               Compiles rust/* crates -> archiso/airootfs/usr/bin/
archiso/                      ArchISO profile root
  profiledef.sh               iso metadata, bootmodes, file_permissions map
  packages/                   Local pacman repo (built pkgs + repo db live here)
  airootfs/                   Squashfs root overlay
    etc/skel/.config/          niri, waybar, foot, fuzzel — DO NOT MODIFY
    root/scripts/             Live-ISO runtime scripts (users, services, desktop, cleanup, greetd-config)
    usr/share/churros/        Assets runtime de las apps Rust (welcome, preferences, control-center) + i18n.py + scripts
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

- **Git workflow**: every change starts on a new branch (never on `main`). Create the branch, make and verify the changes there, and only merge back into `main` once everything works.
- Shell scripts: `#!/usr/bin/env bash`, `set -e`, shellcheck-compliant.
- Calamares modules: `.conf` (and `.yaml` for netinstall) in `installer/calamares/modules/`.
- Package lists: one package per line in `archiso/packages.x86_64`.
- File mode map (not git): declared in `archiso/profiledef.sh` `file_permissions` (e.g. `/usr/bin/churros-*` 0755).
- Bootstrap uses `pacman.conf` (declared in `profiledef.sh`); airootfs compressed squashfs xz; bootstrap tarball zstd.

## Calamares Sequence

`installer/calamares/settings.conf` defines five `shellprocess` instances and the exec order. The exec sequence IS order-sensitive — keypin requirements:

- `shellprocess@boot-nocow` runs after `mount` and **MUST** come before `unpackfs`: `chattr +C` + `compression=none` on the target `/boot` so vmlinuz is never stored as btrfs zstd (GRUB `premature end of file`).
- `shellprocess@pacman-init` (keyring init) **MUST** come before `shellprocess@fix-boot` (mkinitcpio preset rewrite + kernel modules) — both already ordered this way; do not reorder.
- `shellprocess@fix-boot` runs before `shellprocess@churros-repo`, which registers the build-time `[churros]` repo (`Server = file:///root/packages`) in the target's pacman.conf so `netinstall` can resolve yay/waypaper/python-pywal.
- `shellprocess@churros-repo` **MUST** run before `netinstall`/`packages`; the repo is removed again by `shellprocess@post-install` (unanchored `sed /churros/d` is forbidden — use the anchored `[churros]` block removal).
- `shellprocess@post-install` (cleanup: drops `[churros]`, `userdel -r churros`, removes live-only `/root` artifacts) is the last exec step before `umount`.
- `shellprocess@grub-theme` runs right after `bootloader`: copies `branding/grub-theme` (deployed to `/usr/share/churros/grub-theme/` at live boot) into `/boot/grub/themes/churros/`, appends `GRUB_THEME` to the target's `/etc/default/grub`, reruns `grub-mkconfig -o /boot/grub/grub.cfg`, then `make-boot-grub-readable` so GRUB can read `/boot` on btrfs+zstd.

Config files per instance: `shellprocess-pacman.conf`, `shellprocess-fixboot.conf`, `shellprocess-repo.conf`, `shellprocess-grub-theme.conf`, `shellprocess-cleanup.conf`, `shellprocess-boot-nocow.conf`. Module IDs in `instances:` are `pacman-init`, `fix-boot`, `churros-repo`, `grub-theme`, `post-install`, `boot-nocow`.

## Key Architecture

- **Live user**: `churros` (wheel, audio, video, input, storage, network), NOPASSWD sudo — created by `archiso/airootfs/root/scripts/users.sh`.
- **Compositor**: Niri (Wayland scrollable-tiling). Requires 3D accel in QEMU (see Testing).
- **Display Manager**: greetd with autologin to `churros` / `niri` session.
- **Panel/Launcher/Terminal**: Waybar / Fuzzel / foot.
- **Apps**: portadas a Rust (gtk4-rs + libadwaita-rs) en `rust/`: `churros-welcome`, `churros-settings` (preferences), `churros-popup` (6 popups en un binario con toggle nativo vía pidfiles en `/tmp/churros/`) y `churros-control-center`. Sus binarios se despliegan en `/usr/bin/churros-*` por `build-rust.sh` (crates con `deploy = true`); los assets runtime viven en `/usr/share/churros/<app>/` (los crates resuelven a `assets/` local en desarrollo). `usr/share/churros/i18n.py` (gettext) sigue en Python para las apps que lo usan.
- **Installer**: Calamares with custom `churros` branding (slideshow, QSS stylesheet).
- **Boot modes** (from `profiledef.sh`): `bios.syslinux` + `uefi.grub`. No systemd-boot, no Limine (mkarchiso del host no lo soporta).
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
