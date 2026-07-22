# Devlog

## 2026-07-16

- Added `python-psutil` to `archiso/packages.x86_64` so `churros-welcome` can import `psutil` in the live image.
- Updated `apps/churros-welcome/src/utils/system.py` and `archiso/airootfs/usr/share/churros/churros-welcome/src/utils/system.py` to import `psutil` safely and fall back to `/proc/meminfo` when `psutil` is missing.
- Verified that Niri autostart already includes `spawn-at-startup "churros-welcome"` in `/etc/skel/.config/niri/config.kdl`.
- Did not change autostart configuration itself; the fix was ensuring the welcome app can launch without `psutil` missing in the image.
- Updated ChurrOS Welcome styling in `apps/churros-welcome/assets/style.css` and `archiso/airootfs/usr/share/churros/churros-welcome/assets/style.css` for a darker branded theme, softer shadows, and consistent spacing.
- Updated Waybar styling in `archiso/airootfs/etc/skel/.config/waybar/style.css` to match ChurrOS branding with rounded panels, improved workspace button styles, and cleaner tray item appearance.
