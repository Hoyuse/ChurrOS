# Preferences

`churros-settings` es la app de configuración principal de ChurrOS. Está escrita en Python + GTK4 con un estilo visual inspirado en System Settings de macOS pero con los colores naranja de ChurrOS.

---

# Overview

**Path:** `archiso/airootfs/usr/share/churros/preferences/`
**Launcher:** `/usr/bin/churros-settings` → `python3 /usr/share/churros/preferences/main.py`
**Atajo:** `Mod + P` (niri)

La app tiene sidebar (páginas principales) + stack de páginas (con subpáginas navegables).

---

# Estructura

```text
preferences/
├── main.py                  # PreferencesApplication
├── window.py                # PreferencesWindow (Gtk.ApplicationWindow)
├── i18n.py                  # gettext wrapper (copia central)
├── style.css                # CSS con variables (dark/light + accent)
├── assets/
│   └── icons/               # SVGs del sidebar
├── widgets/                 # Widgets base
│   ├── page.py              # Page (ScrolledWindow + botón atrás opcional)
│   ├── sidebar.py           # Sidebar con search + lista
│   ├── navigator.py         # Navigator (Gtk.Stack con history)
│   ├── row.py               # Row (Gtk.Button base)
│   ├── group.py             # Group (card con separadores)
│   ├── combo_row.py         # ComboRow (DropDown)
│   ├── slider_row.py        # SliderRow (Gtk.Scale)
│   ├── switch_row.py        # SwitchRow (Gtk.Switch)
│   ├── select_row.py        # SelectRow (CheckButton group)
│   └── navigation_row.py    # NavigationRow (row con flecha a subpágina)
├── services/                # Services (lógica del sistema)
│   ├── settings.py          # SettingsService (JSON en ~/.config/churros/settings.json)
│   ├── theme.py             # ThemeService (dark/light + wallpaper連動)
│   ├── accent.py            # AccentService (genera accent.css con --accent-glow)
│   ├── fonts.py             # FontService (gsettings + Gtk.Settings)
│   ├── cursor.py            # CursorService (gsettings cursor-theme/size en vivo)
│   ├── icons.py             # IconsService (gsettings icon-theme en vivo)
│   ├── wallpaper.py         # WallpaperService (swaybg primero, awww fallback)
│   ├── waybar.py            # WallpaperService (genera config.jsonc + style.css liquid glass)
│   ├── keyboard.py          # KeyboardService (parsea y edita binds de niri)
│   ├── audio.py             # AudioService (wpctl/PipeWire)
│   ├── power.py             # PowerService (powerprofilesctl, upower, gsettings)
│   ├── display.py           # DisplayService + backends (niri/hyprland)
│   ├── datetime.py          # DateTime helpers (timedatectl wrapper)
│   ├── connectivity.py
│   ├── about.py
│   ├── system.py
│   ├── users.py
│   ├── applications.py
│   ├── privacy.py
│   └── backends/            # Backends de display
│       ├── base.py
│       ├── niri.py          # niri msg outputs/action
│       └── hyprland.py      # hyprctl
└── pages/                   # Páginas de la UI
    ├── system.py
    ├── appearance.py         # Padre de accent/icons/cursor/fonts/wallpaper/waybar
    ├── audio.py
    ├── display.py
    ├── connectivity.py
    ├── power.py              # Padre de power-profile/battery/display-timeout/sleep
    ├── applications.py
    ├── users.py
    ├── privacy.py
    ├── about.py
    ├── input.py              # Teclado/ratón
    ├── datetime.py          # Fecha y hora (timedatectl)
    ├── keyboard.py          # Editor visual de atajos de Niri
    # Subpáginas de appearance
    ├── accent.py
    ├── icons.py
    ├── cursor.py
    ├── fonts.py
    ├── wallpaper.py
    └── waybar.py             # Editor de barra superior
    # Subpáginas de power
    ├── power_profile.py
    ├── battery.py
    ├── display_timeout.py
    └── sleep.py
```

---

# Settings persistence

`SettingsService` (`services/settings.py`):

- Ruta: `~/.config/churros/settings.json`
- Formato JSON anidado, acceso con dot keys: `SettingsService.get("theme.dark")`, `set("accent.color", "Orange")`.
- Defaults definidos en `DEFAULTS` dict.
- `_ensure()` crea el archivo con defaults la primera vez.

outsourced a gsettings:

- `theme.dark` ↔ `org.gnome.desktop.interface color-scheme` (`prefer-dark` / `default`)
- `cursor.theme` ↔ `org.gnome.desktop.interface cursor-theme`
- `cursor theme size` ↔ `org.gnome.desktop.interface cursor-size`
- `fonts.family` ↔ `org.gnome.desktop.interface font-name`, `document-font-name`, `monospace-font-name`
- `fonts.scale` ↔ `org.gnome.desktop.interface text-scaling-factor`
- `icons.theme` ↔ `org.gnome.desktop.interface icon-theme`

`SettingsService` guarda su propia copia además para queries rápidas y para defaults offline.

---

# Tema (dark/light)

`ThemeService.is_dark()`:

1. Lee `theme.dark` del JSON local.
2. Si no está, hace `gsettings get org.gnome.desktop.interface color-scheme` y cachea el resultado.

`ThemeService.set(dark)`:

1. Persiste en JSON local.
2. Llama `gsettings set org.gnome.desktop.interface color-scheme prefer-dark|default`.
3. La propia app Preferences detecta el cambio vía `Gio.Settings` `changed::color-scheme` signal y llama `_apply_theme_class()` (añade/quita `.light` en la ventana).

El CSS se basa en variables (`--bg-window`, `--text-primary`, etc.) definidas en `window { ... }` (dark default) y sobrescritas en `window.light { ... }`. Cuando `.light` se añade al root, todas las reglas con `var(...)` reaccionan automáticamente.

Importante: el CSS viejo tenía hardcodes `color:white` y 3 reglas conflictivas para `.sidebar`. El actual está limpio y todo usa `var(--*)`.

---

# Accent color

`AccentService` (`services/accent.py`):

- 8 colores predefinidos: Blue, Purple, Pink, Red, Orange, Yellow, Green, Teal.
- `current()` lee `accent.color` del JSON.
- `set(color)`:
  1. Persiste en JSON.
  2. Genera `~/.config/churros/accent.css` con variables `--accent`, `--accent-strong`, `--accent-soft`, `--accent-bg-hover`, `--accent-text`.
  3. Usa `colorsys` para derivar variantes (strong = darker, soft = lighter).

Importante GTK4: el CSS generado usa `window { ... }` (NO `:root` — GTK4 no lo soporta).

`AccentPage._reload_accent_css()` recarga el archivo en runtime:

```python
provider = Gtk.CssProvider()
provider.load_from_path(accent_css)
Gtk.StyleContext.add_provider_for_display(
    Gdk.Display.get_default(),
    provider,
    Gtk.STYLE_PROVIDER_PRIORITY_USER + 1   # prioridad mayor al style.css principal
)
```

Resultado: clic en "Green" → recarga CSS → todos los `var(--accent)` se actualizan.

---

# Fuentes

`FontService` (`services/fonts.py`):

- `available()` — lista familias vía `fc-list : family`.
- `current()` — JSON local (`fonts.family`).
- `set(family)` — además de gsettings (`font-name`, `document-font-name`, `monospace-font-name`), llama `Gtk.Settings.get_default().set_property("gtk-font-name", ...)` para que la app actual reaccione sin reiniciar. También resetea `gtk-fontconfig-timestamp` para forzar reload de fontconfig.
- `scale()` / `set_scale(scale)` — `text-scaling-factor` en gsettings + `gtk-xft-dpi` en Gtk.Settings.

---

# Cursor

`CursorService` (`services/cursor.py`):

- `available()` — busca en `/usr/share/icons`, `~/.icons`, `~/.local/share/icons` las carpetas con subdirectorio `cursors/`.
- `current()` — gsettings `cursor-theme`.
- `set(theme)` — gsettings + JSON local.
- `size()` / `set_size(px)` — gsettings `cursor-size`.

`CursorPage` además incluye un SliderRow (8–64 px) para el tamaño.

---

# Wallpaper

`WallpaperService` (`services/wallpaper.py`):

Dirs buscadas (en orden):

- `~/.local/share/churros/wallpapers/` (USER_DIR, donde se copian las importadas)
- `/usr/share/churros/wallpapers/`
- `/usr/share/backgrounds/`
- `~/Pictures/Wallpapers/`
- `~/Pictures/`

Extensões: `.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`.

`set(path)`:

1. Persiste `wallpaper.path` en JSON.
2. Intenta con `awww img <path>` (transiciones suaves). Si `awww-daemon` no está corriendo, lo arranca.
3. Si `awww` no existe o falla → fallback a `pkill swaybg` + `swaybg -i path -m fill` (start_new_session=True para que sobreviva a la app).

`import_image(source_path)`:

- Copia cualquier imagen del disco del usuario a `~/.local/share/churros/wallpapers/`.
- Maneja renombres si el nombre ya existe (`foto_1.png`, `foto_2.png`, …).
- Devuelve la ruta destino, o None si falla.

`WallpaperPage` UI:

- Botón "Importar desde archivos..." usando `Gtk.FileDialog` (GTK4 nativo, no el deprecated FileChooserDialog).
- Filtro de mime types: image/jpeg, image/png, image/webp, image/gif.
- Ruta inicial: `~` (Gio.File.new_for_path).
- Miniatura del fondo actual (160px, con clase `wallpaper-preview`).
- `Gtk.FlowBox` con thumbnails (120px cada uno) de todos los disponibles. El actual tiene clase `wallpaper-selected` (border accent).
- Tras importar: copia → aplica → recarga todo el grid (`_rebuild_grid()`).

CSS:

- `.wallpaper-thumb` — thumbnail base (radius 10px).
- `.wallpaper-selected` — 3px border accent.
- `.wallpaper-button:hover` — border accent al hover.

## Theme連動 (Modo oscuro/claro)

`ThemeService.set(dark)` (en `services/theme.py`):

1. Persiste `theme.dark` en JSON local.
2. Llama `_write_gtk_settings(dark)` que escribe `~/.config/gtk-{3,4}.0/settings.ini` con `gtk-theme-name` y `gtk-application-prefer-dark-theme`.
3. **Cambia el wallpaper automáticamente**:
   - Dark → `/usr/share/churros/wallpapers/fondo1.png`
   - Light → `/usr/share/churros/wallpapers/default.jpeg`
4. Envía `SIGUSR2` a waybar (recarga) y a foot `SIGUSR1`/`SIGUSR2` según el modo (oscuro/claro).

`WallpaperService.apply(path)`:

1. Llama `churros-apply-wallpaper` (wrapper bash).
2. Wrapper prueba en orden: `swaybg` → `awww` (si falla).
3. Al final del apply, ejecuta `niri msg action do-screen-transition` para forzar repintura en Niri.
4. Fallback inline en Python si el wrapper no existe.

## Waybar (subpágina de Apariencia)

`WaybarService` (`services/waybar.py`):

- `DEFAULTS`: layer=top, position=top, height=30, spacing=0, font-size=14, font-family="JetBrainsMono Nerd Font", background=#2a1612, foreground=#c9c4c3, accent=#DE8636, background-alpha=0.9.

- `get()` — lee config.jsonc + colors-waybar.css.

- `set(values)`:
  1. Escribe `~/.config/waybar/config.jsonc` (preserva definiciones de módulos del skel).
  2. Escribe `~/.config/waybar/colors-waybar.css` con `@define-color` para `background`, `foreground`, `color4`, `color1`. **Nunca** `@define-color background-alpha` con número — waybar crashea con `'0' is not a valid color name`.
  3. Escribe `~/.config/waybar/style.css` con liquid glass: ventana transparente, workspaces con pill translúcido + items activos en naranja, hover states.
  4. Llama `reload()` que mata waybar (`pkill -x waybar`), espera 1s, verifica que murió (`pgrep`), lanza nuevo `waybar`.

- `reload()` loguea a `/tmp/waybar.log` para debugging.

`WaybarPage` UI:

- Sliders para altura (20-80), espaciado (0-16), font-size (10-24).
- Combo para capa (top/overlay/bottom) y posición (top/bottom/left/right).
- Color pickers para fondo, texto, acento.
- **Sistema de rotación de módulos** (sin popovers — GTK4 Popover no funciona en Niri):
  - Click izquierdo en un módulo → rota posición left → center → right → left.
  - Click derecho → quita el módulo.
- Botones "Recargar waybar" y "Restablecer defaults".

## Keyboard (Atajos de teclado)

`KeyboardService` (`services/keyboard.py`):

- Lee/escribe `~/.config/niri/config.kdl` directamente.
- `get_keybinds()` — parsea el bloque `binds { ... }` con regex.
- `set_keybind(key, action_type, command, args)` — reemplaza un binding existente.
- `add_keybind(key, action_type, command, args)` — añade un nuevo binding antes del `}` de cierre.
- `restore_backup()` — restaura desde `config.kdl.bak`.

Backup automático antes de cada escritura en `NIRI_CONFIG_BACKUP`.

`KeyboardPage` UI:

- Agrupa atajos en categorías: Aplicaciones, Ventanas, Workspaces, Movimiento, Capturas, Overlays, Multimedia, Niri.
- **Click en un atajo** abre diálogo de edición que muestra:
  - Atajo actual y acción actual (resaltados).
  - Campo "Nuevo atajo" (placeholder: `Ej: Mod+Shift+X`).
  - Campo "Nuevo comando" (pre-rellenado).
  - Campo "Argumentos" (pre-rellenado).
- Botón **"Agregar nuevo atajo"** al inicio para crear atajos nuevos con campos vacíos.
- Al guardar, rebuild automático de la lista.

## DateTime (Fecha y hora)

`DateTimePage` (sin service dedicado, usa `timedatectl` directamente):

- Estado actual: hora, fecha, zona horaria, RTC.
- "Sincronizar hora con internet" → `sudo timedatectl set-ntp true` en foot.
- "Cambiar zona horaria" → selector interactivo con `fzf` sobre `timedatectl list-timezones` o fallback `less`.

---

# Power

Métodos de lectura:

- `battery_present()` — `upower -e` busca un dispositivo que termine en `battery`.
- `battery_percentage()` — `upower -i /org/freedesktop/UPower/devices/battery_BAT0` (fallback BAT1).
- `battery_state()` — `charging`, `discharging`, `full`, etc.
- `power_profile()` — `powerprofilesctl get` (performance/balanced/power-saver).
- `power_profiles_available()` — `powerprofilesctl list`.
- `screen_timeout()` / `sleep_timeout()` — gsettings `idle-delay` / `sleep-inactive-ac-timeout`.
- `lid_close_action()` — gsettings `lid-close-ac-action`.

Setters:

- `set_power_profile(profile)`.
- `set_screen_timeout(seconds)`.
- `set_sleep_timeout(seconds)`.
- `set_lid_close_action(action)`.

Páginas:

- `power.py` — menú con NavigationRows a las 4 subpáginas.
- `power_profile.py` — combo performance/balanced/power-saver con labels localizados.
- `battery.py` — estado real (upower) o mensaje "No se detecta ninguna batería".
- `display_timeout.py` — combo "1m/2m/5m/10m/15m/30m/Nunca".
- `sleep.py` — combo timeout + combo acción al cerrar tapa.

Todas las subpáginas llevan `parent_page="power"` para mostrar el botón "Atrás".

---

# Display backends

`services/backends/` define `DisplayBackend` (clase abstracta) con dos implementaciones:

- `niri.py` — vía `niri msg action ...` y `niri msg output ...`. Soporta `set_resolution`, `set_vrr`, `set_scale`, `set_rotation`, `set_brightness`.
- `hyprland.py` — vía `hyprctl keyword monitor ...`. NO soporta `set_resolution` ni `set_vrr` (stubs que devuelven False), solo `set_scale`, `set_rotation` y brightness.

`DisplayService.backend()` autodetecta el compositor activo.

`DisplayPage` comprueba `backend.supports_resolution()` y `backend.supports_vrr()` antes de mostrar los combos correspondientes. Si el backend no soporta VRR, no se muestra el switch "Frecuencia variable (VRR)".

Bug histórico: `on_vrr(self, switch, active)` recibía 2 args cuando `SwitchRow` solo pasa 1 (el bool). Corregido a `on_vrr(self, active)`.

---

# Subpáginas y navegación

`Navigator` (`widgets/navigator.py`) es un `Gtk.Stack` con:

- `add_page(name, widget)` — añade una página al stack (si no existe ya).
- `show_page(name)` — muestra la página, metiendo la anterior en `self.history`.
- `back()` — pop del historial.
- `clear_history()`.

`Page` (`widgets/page.py`) es un `Gtk.ScrolledWindow` que acepta `parent_page=None`:

- Si se pasa `parent_page`, añade un botón "Atrás" (icon `go-previous-symbolic`) al inicio.
- `on_back()` llama `navigator.show_page(self.parent_page)`.

Subpáginas registradas en `window.py`:

```
appearance (padre) → accent, icons, cursor, fonts, wallpaper
power     (padre) → power-profile, battery, display-timeout, sleep
```

Cada subpágina declara `parent_page="appearance"` o `parent_page="power"` en su `super().__init__(...)`.

---

# SelectRow — selección single mutual

`SelectRow` (`widgets/select_row.py`) hereda de `Row` (Gtk.Button) y añade un `Gtk.CheckButton` como suffix.

Problemas que tuvo:

- En GTK4 el `Gtk.Button` captura el click y el `CheckButton` hijo no recibe el evento → el callback no se ejecutaba.
- Multiple rows activos sin group → varios items quedaban seleccionados.

Fix actual:

- `Gtk.CheckButton.set_group()` con group compartido (atributo de clase `SelectRow.group`).
- Callback conectado a `notify::active` del CheckButton (se emite SIEMPRE al togglear).
- `on_row_clicked` fuerza `set_active(True)` para que el grupo se re sincronice desde el click del row.
- `SelectRow.reset_group()` class method — llamarlo al inicio del `__init__` de cada página que use SelectRow (accent, cursor, icons) para que cada página tenga su propio grupo.

---

# CSS — Estilo macOS con colores ChurrOS

`style.css` (~460 líneas):

Tokens en `window { ... }` (dark default):

```css
--bg-window:        rgba(30,30,32,.96);
--bg-sidebar:       rgba(42,42,46,.78);
--bg-group:         rgba(56,56,60,.55);
--text-primary:     rgba(255,255,255,.96);
--text-secondary:   rgba(255,255,255,.55);
--border-soft:      rgba(255,255,255,.08);
--accent:           #DE8636;        /* ChurrOS naranja */
--accent-soft:      #E6A56A;
--accent-strong:    #B86A24;
--accent-text:      #ffffff;
--accent-bg-hover:  rgba(222,134,54,.18);
--scroller-thumb:  rgba(255,255,255,.12);
```

Override `window.light { ... }`:

```css
--bg-window:        rgba(242,242,245,.98);
--bg-sidebar:       rgba(232,232,236,.85);
--bg-group:         rgba(255,255,255,.92);
--text-primary:     rgba(28,28,30,.96);
--text-secondary:   rgba(28,28,30,.60);
--border-soft:      rgba(0,0,0,.08);
--accent:           #C5651C;        /* más oscuro para contraste en fondo claro */
```

Estilo macOS:

- Sidebar translúcido (alpha < 1), radius 16px, border subtle.
- Items sidebar: radius 9px, hover en accent-bg-hover, activo con fondo accent sólido + texto accent-text.
- Search box con radius 9px, focus border en accent.
- Grupos tipo card: radius 12px, border soft.
- Rows: radius 8px, hover en row-bg-hover. Son Gtk.Button sin frame.
- Group title: small caps, accent o text-secondary.
- Switches estilo iOS: 42x24 con border-radius 14px, slider 18x18 que se desplaza 22px al activar.
- Sliders con trough 5px y slider 14px redondo con sombra.
- DropDowns (combos) con radius 8px y hover en accent.
- Popover list rows con radius 6px.
- Scrollbar fina 8px, color scroller-thumb, hover en accent.
- About card radius 16px.
- Wallpaper thumbs radius 10px, selected 3px border accent.
- Back button radius 8px.

Sin reglas duplicadas, sin hardcodes fuera de los tokens.

---

# Build pipeline

`preederences/main.py` al arrancar:

1. `AccentService.ensure()` — si `~/.config/churros/accent.css` no existe, lo genera.
2. Carga `style.css` con `Gtk.CssProvider.load_from_path` y registra con `Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION`.
3. Si existe `~/.config/churros/accent.css`, lo carga con prioridad `Gtk.STYLE_PROVIDER_PRIORITY_USER` (mayor que el style.css principal, así `--accent` del override gana).
4. Crea `PreferencesWindow(self)` y la presenta.

CSS reload tras cambio de accent:

- `AccentPage._reload_accent_css()` crea un provider nuevo con prioridad `Gtk.STYLE_PROVIDER_PRIORITY_USER + 1` (aún mayor) para que las nuevas variables ganen.

---

# Known limitations

- **font-name y gtk**: el cambio de fuente se refleja inmediatamente solo en la app Preferences actual. Otras apps GTK respawn lo aplican automáticamente (gsettings func); para apps ya corriendo hay que esperar reload o reiniciarlas.
- **i18n gettext**: actualmente `i18n._()` solo devuelve la string original (no carga PO/MO files). La infraestructura está lista; solo falta compilar `po/churros.po` a `/usr/share/locale/es/LC_MESSAGES/churros.mo`.
- **Multi-monitor display page**: DisplayPage asume un solo monitor; multi-monitor requiere refactor del `Monitor` model.
- **Hyprland backend supports_* stubs**: hyprland backend no implementa VRR ni set_resolution (devuelven False). DisplayPage omite esos combos cuando está activo.
- **Tests**: sin tests automatizados GTK4; verificación es `python3 -c "import ast; ast.parse(open(f).read())"` para todos los .py + lanzar manualmente cada app en niri.

---

# Future work

- Soporte multi-monitor en DisplayPage.
- Dynamic colors: `theme.dynamic_colors` flag existe en settings.json pero no tiene efecto todavía. Idea: usar `python-pywal` para generar `~/.cache/wal/colors.css` desde el wallpaper actual y aplicar como `--accent` derivado.
- Per-user cursor/wallpaper persistence entre sesiones (gsettings es per-user, settings.json también).
- Página de input avanzada: gesture del trackpad, natural scrolling, tap-and-drag.
- Aplicar font/cursor a Qt apps (env vars QT_QPA_FONTDIR, etc.).
