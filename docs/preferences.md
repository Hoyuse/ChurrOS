# Preferences

`churros-settings` es la app de configuración principal de ChurrOS. Está escrita en **Rust** (gtk4-rs) con un estilo visual inspirado en System Settings de macOS y los colores naranja de ChurrOS.

---

# Overview

**Path:** `rust/preferences/`
**Binario:** `/usr/bin/churros-settings` (desplegado por `build-rust.sh`)
**Atajo:** `Mod + P` (niri)
**Assets:** `/usr/share/churros/churros-settings/` en la ISO; `rust/preferences/assets/` en desarrollo
**Log:** `/tmp/churros/churros-settings.log`

Sidebar + stack de páginas (con subpáginas). Ventana por defecto 1280×760.

---

# Estructura

```text
rust/preferences/
├── Cargo.toml                 # name = "churros-settings", deploy = true
├── assets/
└── src/
    ├── main.rs                # Gtk.Application + carga de CSS
    ├── window.rs              # PreferencesWindow (sidebar + stack + history)
    ├── logging.rs
    ├── assets.rs
    ├── widgets/               # Page, Sidebar, Row, Group, ComboRow, …
    ├── services/              # Settings, tema, acento, niri, mako, …
    └── pages/                 # Una página por archivo
```

Los servicios de display aún entienden un backend Hyprland (`hyprctl`) además de Niri; el escritorio oficial es Niri. Pywal está en la UI de Apariencia como interruptor, pero la integración real sigue en TODO.

---

# Settings persistence

`SettingsService` (`services/settings.rs`):

- Ruta: `~/.config/churros/settings.json`
- JSON anidado, acceso con dot keys (`theme.dark`, `accent.color`).
- Defaults en código; crea el archivo la primera vez.

También se espeja en gsettings:

| Clave local | gsettings |
|-------------|-----------|
| `theme.dark` | `org.gnome.desktop.interface color-scheme` |
| `cursor.theme` / tamaño | `cursor-theme` / `cursor-size` |
| `fonts.family` / escala | `font-name`, `document-font-name`, `monospace-font-name`, `text-scaling-factor` |
| `icons.theme` | `icon-theme` |

---

# Tema (dark/light)

`ThemeService`:

1. Lee `theme.dark` del JSON; si no está, consulta gsettings y cachea.
2. `set(dark)` persiste y escribe `prefer-dark` / `default`.
3. La ventana añade o quita la clase `.light`. El CSS usa variables (`--bg-window`, `--text-primary`, …) definidas en `window` (oscuro) y `window.light`.

En maximizado/fullscreen se añade la clase `maximized` para el CSS glass (menos blur).

---

# Accent color

`AccentService` — 8 colores (Blue, Purple, Pink, Red, Orange, Yellow, Green, Teal).

`set(color)` escribe `~/.config/churros/accent.css` con `--accent` y variantes. El CSS usa `window { … }` (GTK4 no soporta `:root`). Se recarga en caliente con un `CssProvider` a prioridad USER.

---

# Fuentes, cursor, iconos, wallpaper

- **FontService** — `fc-list`, gsettings + `Gtk.Settings` (`gtk-font-name`, `gtk-xft-dpi`) para que la propia app reaccione sin reiniciar.
- **CursorService** — busca temas en `/usr/share/icons` y `~/.icons`; aplica `cursor-theme` / `cursor-size`.
- **IconsService** — `icon-theme` en gsettings.
- **WallpaperService** — `swaybg` (con fallback). El hook pywal está pendiente.

---

# Display

`DisplayService` lista monitores y modos. Backend principal: `niri msg outputs` / `niri msg action`. Queda un backend Hyprland (`hyprctl monitors -j`) por si `XDG_CURRENT_DESKTOP` lo indica; no implementa VRR ni cambio de resolución (la página omite esos controles en ese caso).

---

# Navegación

- `gtk::Stack` + historial en `window.rs`.
- `Ctrl+F` enfoca la búsqueda global; salta a subpáginas y resalta el row.
- `Alt+Left` o el botón atrás vuelven en el historial.
- En pantallas estrechas el sidebar se colapsa (`Revealer`).
- El item del sidebar sigue al padre si estás en una subpágina.

---

# Árbol de páginas

`power` es padre de power-profile / battery / display-timeout / sleep. `appearance` agrupa acento, iconos, cursor, fuentes, wallpaper y waybar.

| Página | Sección | Función |
|--------|---------|---------|
| `system.rs` | Sistema | Hostname, kernel, OS |
| `appearance.rs` | Apariencia | Padre (tema, pywal stub, 9 grupos) |
| ↳ `accent.rs` | (subpágina) | 8 colores + custom |
| ↳ `icons.rs` | (subpágina) | Tema de iconos |
| ↳ `cursor.rs` | (subpágina) | Tema y tamaño de cursor |
| ↳ `fonts.rs` | (subpágina) | Familia, escala, preview |
| ↳ `wallpaper.rs` | (subpágina) | Selector de fondo |
| ↳ `waybar.rs` | (subpágina) | Posición, módulos, colores |
| `display.rs` | Pantalla | Resolución, brillo, scale, VRR |
| `audio.rs` | Sonido | Volumen, dispositivo |
| `mako.rs` | Notificaciones | DND, tipografía, colores, bordes |
| `night_light.rs` | Pantalla | `wlsunset` |
| `lock_screen.rs` | Pantalla | `swaylock` / `swayidle` |
| `connectivity.rs` | Red | Wi-Fi, ethernet, bluetooth |
| `power.rs` | Energía | Padre |
| ↳ `power_profile.rs` | (subpágina) | `powerprofilesctl` |
| ↳ `battery.rs` | (subpágina) | Estado y umbrales |
| ↳ `display_timeout.rs` | (subpágina) | Apagado de pantalla |
| ↳ `sleep.rs` | (subpágina) | Suspensión y tapa |
| `datetime.rs` | Sistema | Reloj, zona horaria, NTP |
| `niri.rs` | Sistema | Animaciones y duraciones |
| `window_rules.rs` | Sistema | Bloques `window-rule {}` |
| `keyboard.rs` | Entrada | Atajos de Niri |
| `input.rs` | Entrada | Teclado / ratón |
| `foot.rs` | Entrada | `foot.ini` |
| `fuzzel.rs` | Entrada | `fuzzel.ini` |
| `applications.rs` | Sistema | Apps instaladas |
| `users.rs` | Sistema | Cuentas |
| `privacy.rs` | Sistema | Permisos |
| `backup.rs` | Sistema | Export / import / reset de `~/.config/churros/` |
| `logs.rs` | Sistema | Visor de logs |
| `about.rs` | Sistema | Versión, créditos, licencia |

---

# Servicios extra

- **LockScreenService** — `swaylock` + `swayidle` (timeout, lock al suspender).
- **NightLightService** — `wlsunset` (temperatura, gamma, automático).
- **MakoService** (`mako_config.rs`) — escribe `~/.config/mako/config`, `makoctl reload`, DND.
- **NiriConfig** — parser KDL y `reload()` (`pkill -HUP niri`).
- **WaybarService** — `config.jsonc` + `style.css`; recarga con `SIGUSR1`.
- **DatetimeService** — `timedatectl` vía `churros-pkexec`.
- **BackupService** — tar de `~/.config/churros/`.

---

# Robustez

- `logging.rs` escribe a `/tmp/churros/churros-settings.log` y captura panics.
- Recargas sin reiniciar la app: Niri (HUP), Mako (`makoctl reload`), Waybar (`SIGUSR1`), tema y acento (CSS en caliente).
- `ComboRow` es un `Gtk.Box` (no hereda de `Button`, para no comerse los clics del dropdown).

---

# Atajos globales

| Atajo | Acción |
|-------|--------|
| `Ctrl+F` | Búsqueda |
| `Alt+Left` | Atrás |
| `Mod+P` | Abrir la app (niri) |

---

# Known limitations

- El cambio de fuente se aplica al momento en esta app; otras GTK ya abiertas pueden necesitar reinicio.
- i18n: los `.po` existen, pero el binario Rust aún no carga gettext.
- Display asume un monitor principal; multi-monitor incompleto.
- Backend Hyprland: sin VRR ni `set_resolution`.
- Pywal: interruptor visible, integración pendiente.
- Sin tests automatizados GTK. Verificación: `cargo build` + probar en niri / `./churros run`.

---

# Future work

- Multi-monitor en Display.
- Terminar pywal (`theme.dynamic_colors`).
- Gestos de trackpad y scrolling natural.
- Aplicar fuente/cursor a apps Qt.
