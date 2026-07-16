# Devlog

## 2026-07-16

- Added `python-psutil` to `archiso/packages.x86_64` so `churros-welcome` can import `psutil` in the live image.
- Updated `apps/churros-welcome/src/utils/system.py` and `archiso/airootfs/usr/share/churros/churros-welcome/src/utils/system.py` to import `psutil` safely and fall back to `/proc/meminfo` when `psutil` is missing.
- Verified that Hyprland autostart already includes `exec-once = churros-welcome` in `/etc/skel/.config/hypr/autostart.conf` and that `hyprland.conf` sources that autostart file.
- Did not change autostart configuration itself; the fix was ensuring the welcome app can launch without `psutil` missing in the image.
