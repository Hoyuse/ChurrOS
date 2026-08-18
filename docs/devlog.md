# Devlog

## 2026-08-17 — ChurrOS 0.7

Release pública **v0.7**. ISO `ChurrOS-2026.08.17-x86_64-v0.7.iso` y torrent en download.churroslinux.org.

Incluye actualizador (pacman, Flatpak y utilidades de ChurrOS), tienda Bazaar, rediseño de popups y personalización de Waybar.

---

## 2026-08-13 — ChurrOS 0.6

Release pública **v0.6** (tag `v0.6`). Las apps oficiales corren como binarios Rust (`churros-welcome`, `churros-settings`, `churros-control-center`, `churros-popup`). El tema GRUB se despliega al airootfs para que Calamares lo aplique en el sistema instalado. `build-rust.sh` instala cargo/rust en el host si faltan.

La ISO supera el límite de 2 GiB de GitHub: se publica partida en dos `.7z` más `SHA256SUMS`.

`VERSION` y el resto de la documentación se alinean a esta versión en una tanda posterior (dejan de describir v0.4.0 / Python como el runtime actual).

---

## 2026-07-25 — Popups, Control Center, Welcome, Preferencias

### Resumen

Sesión grande: terminadas las fases de personalización faltantes (5c, 5d), arreglados los bugs que rompían el arranque del live ISO y los que impedían abrir los popups, el centro de control y aplicar las opciones de apariencia.

### Bug crítico: `niri validate` fallaba y rompía el arranque del live ISO

- `archiso/airootfs/etc/skel/.config/niri/config.kdl` tenía una cláusula `match app-id=r#"firefox$"# title="^Picture-in-Picture$"` que no es KDL válido (raw string literal). Cambiado a dos sentencias `match` separadas que actúan con AND semántico (cada `window-rule` aplica si todos sus `match` coinciden).
- Conflictos de atajos: `Mod+V` estaba asignado tanto a `churros-settings` como a `toggle-window-floating`. Reorganizados:

| Atajo | Acción |
|-------|--------|
| `Mod+Return` | Terminal (`foot`) |
| `Mod+Space` | Launcher (`fuzzel`) |
| `Mod+C` | Control Center |
| `Mod+P` | Preferencias |
| `Mod+W` | Welcome |
| `Mod+V` | Toggle ventana flotante |
| `Mod+Shift+N` | Popup network |
| `Mod+Shift+A` | Popup audio |
| `Mod+Shift+B` | Popup bluetooth |
| `Mod+Shift+L` | Popup brightness |
| `Mod+Shift+T` | Popup battery |
| `Mod+Shift+E` | Popup power |

### Config niri

- Terminal oficial: **foot** (`packages.x86_64`, welcome/waybar/niri `Mod+Return`). Las docs antiguas hablaban de Kitty; se corrigen para reflejar el stack real.
- Terminal atajo: `Mod+Return` → foot (sin cambios).

### Waybar nuevos módulos

`archiso/airootfs/etc/skel/.config/waybar/config.jsonc`:

- Añadido `custom/control-center` (icono `󰂗`) que lanza `churros-control-center`.
- Añadido `custom/settings` (icono `󰒋`) que lanza `churros-settings`.
- Ambos colocados en `modules-right`, justo antes del reloj, precedidos por un `custom/sep`.

### Bug crítico: popups y control-center no abrían

Síntoma: al hacer clic en un módulo de waybar o usar un atajo, no pasaba nada. El script `churros-popup` redirige stdout/stderr a `/dev/null`, así los errores quedaban ocultos.

Causa raíz:

- `control-center/window.py` y los `popups/*/widgets/*.py` importaban `from i18n import _`.
- El módulo `i18n` solo existía en `preferences/i18n.py`.
- `control-center/window.py` intentaba añadir `preferences/` al `sys.path` con `Path(__file__).resolve().parents[2] / "preferences"`, pero `parents[2]` desde `control-center/window.py` es `usr/share/`, no `churros/`. El path calculado era `/usr/share/preferences` (inexistente).
- Resultado: `ModuleNotFoundError: No module named 'i18n'` → la app moría antes de crear la ventana.

Fix aplicado:

- Copié `preferences/i18n.py` a `archiso/airootfs/usr/share/churros/i18n.py` — módulo central accesible desde todas las apps ChurrOS (popups, control-center, preferences) mediante una sola entrada en el `sys.path` (root `churros/`).
- Reescribí `control-center/main.py` para insertar paths en el orden correcto:
  1. `APP_DIR` (control-center/) primero, para que `from window import ...` resuelva al `window.py` propio y no al de preferences.
  2. `PREFS_DIR` (preferences/) segundo, por si todavía queda alguna referencia a `i18n` no centralizada.
  3. `CHURROS_ROOT` tercero, para el namespace package `services.*` (wifi, ethernet, bluetooth...).
- Quité el path-hack en `control-center/window.py` (ya lo hace `main.py`).

Verificación: los 6 popups (audio, battery, bluetooth, brightness, network, power) y `control-center` cargan sin error al ejecutar `python3 main.py` (en host sin Wayland solo reportan un Gtk-WARNING inofensivo de theme).

### SelectRow: clics muertos en accent/cursor/icons

Síntoma reportado: "se queda clavado en la opción que está y no deja poner otro".

Causa:

- `SelectRow` hereda de `Row` (`Gtk.Button`) y añade un `Gtk.CheckButton` como suffix.
- En GTK4 el `Gtk.Button` captura el clic y el `CheckButton` hijo nunca recibe el evento → el callback conectado al `clicked` del row solo se ejecutaba a veces (cuando el clic caía fuera del CheckButton).
- Además el callback solo se invocaba si `not self.check.get_active()` (ignoraba re-clicks).
- No había group entre CheckButtons, así que varios items podían quedar activos a la vez.

Fix reescribiendo `archiso/airootfs/usr/share/churros/preferences/widgets/select_row.py`:

- Uso de `Gtk.CheckButton.set_group()` con un group compartido (atributo de clase `SelectRow.group`) → selección single mutual.
- Callback conectado a `notify::active` del CheckButton (se emite SIEMPRE al togglear, no solo cuando pasa a True).
- `on_row_clicked`: si el check está inactivo, fuerzo `set_active(True)` para que el grupo se re sincronice y emita toggled en todos los afectados.
- Nuevo class method `SelectRow.reset_group()` para que cada página (Accent, Cursor, Icons) arranque con un grupo limpio y no compita con otra página que use SelectRow.

Aplicado `SelectRow.reset_group()` al inicio del `__init__` de:

- `pages/accent.py`
- `pages/cursor.py`
- `pages/icons.py`

### Accent no se aplicaba en runtime

Bug 1: el CSS autogenerado usaba `:root` (no soportado en GTK4 CSS):

```css
/* Antes */
:root {
    --churros-accent: #DE8636;
    ...
}
```

GtkCssProvider ignora `:root`. Fix en `services/accent.py._generate_accent_css`:

```css
/* Ahora */
window {
    --accent: #DE8636;
    --accent-strong: #B86A24;
    --accent-soft: #E6A56A;
    --accent-text: #ffffff;
    --accent-bg-hover: <color más claro>;
}
```

Bug 2: `AccentService.set()` escribía el archivo CSS en disco, pero GTK no lo recargaba hasta el próximo arranque de la app.

Fix en `pages/accent.py.SelectRow`: añadido `_reload_accent_css()` que crea un `Gtk.CssProvider` nuevo, lo carga desde `accent.css` y lo registra en la display con `Gtk.STYLE_PROVIDER_PRIORITY_USER + 1` (prioridad mayor que el provider principal para que las variables nuevas ganen).

Resultado: clic en "Green" → se escribe `~/.config/churros/accent.css` → se cargan las nuevas vars en la display → todos los `var(--accent)` reaccionan inmediatamente.

### Modo oscuro no se aplicaba visualmente

`appearance.py.on_dark_changed` ya llamaba `window.refresh_theme()` vía `GLib.idle_add`, y `window._apply_theme_class()` añade/quita la clase `.light` en el window. Eso estaba bien — pero solo afectaba al `window` root, no era inválido de por sí.

La causa real del "no aplica" era el CSS: antes tenía reglas hardcoded (`.row { color:white; }`) que ignoraban completamente el `.light` y siempre pintaban blanco sobre fondo claro → "mitad negra mitad blanca". El fix fue reescribir todo el CSS (ver sección CSS).

### Fonts no se aplicaban en runtime

`FontService.set()` solo escribía `gsettings set org.gnome.desktop.interface font-name ...`, pero GTK4 solo lee `font-name` al arranque. Lo mismo con `text-scaling-factor`.

Fix en `services/fonts.py`:

- `set()` además hace `Gtk.Settings.get_default().set_property("gtk-font-name", ...)` (afecta a la app actual inmediatamente).
- `set()` también resetea `gtk-fontconfig-timestamp` para que fontconfig reléa la configuración.
- `set_scale()` además hace `Gtk.Settings.get_default().set_property("gtk-xft-dpi", int(1024 * scale))` — escalar DPI en runtime.
- Aplica también `document-font-name` y `monospace-font-name` (no solo `font-name`).

### Wallpaper — explorar archivos y copiarlos a la carpeta de wallpapers

Nuevo en `services/wallpaper.py`:

- `USER_DIR = ~/.local/share/churros/wallpapers/` — carpeta personal de fondos del usuario (añadida también a `WALLPAPER_DIRS` para que aparezcan en el grid).
- `import_image(source_path)` — copia una imagen externa a `USER_DIR`. Si el nombre ya existe, añade sufijo `_1`, `_2`, etc. Devuelve ruta destino o None.
- Fallback en `set()`: si `awww` falla, mata `swaybg` existente y lo relanza con `-i path -m fill` (start_new_session=True para que no muera con la app).

Nuevo en `pages/wallpaper.py`:

- Botón "Importar desde archivos..." usando `Gtk.FileDialog` (GTK4 nativo, no el deprecated `FileChooserDialog`).
- Filtro de mime types: `image/jpeg`, `image/png`, `image/webp`, `image/gif`.
- Ruta inicial: `~` (home del usuario).
- Tras elegir archivo: copia → aplica → recarga todo el grid con `_rebuild_grid()` para mostrar el nuevo thumbnail.

### CSS Preferences reescrito — estilo macOS con colores churros

`archiso/airootfs/usr/share/churros/preferences/style.css` (576 líneas → ~460 líneas consolidadas):

Antes había 3 definiciones de `.sidebar` background, 2 de `.row-title`, hardcoded `color:white` en todos lados. Resultado: al activar `.light`, solo los 4 elementos con `var()` cambiaban → mitad negra mitad blanca.

Ahora **todo** el CSS usa variables CSS. Variables definidas en `window { ... }` (dark default) y sobrescritas en `window.light { ... }`:

- `--bg-window` (fondo principal)
- `--bg-sidebar` (sidebar translúcido)
- `--bg-group` (card de grupo)
- `--row-bg`, `--row-bg-hover`, `--row-bg-active`
- `--text-primary`, `--text-secondary`, `--text-tertiary`
- `--border-soft`, `--border-strong`
- `--accent`, `--accent-soft`, `--accent-strong`, `--accent-text`, `--accent-bg-hover`
- `--scroller-thumb`

Colores churros:

- Dark: `--accent: #DE8636` (naranja cálido)
- Light: `--accent: #C5651C` (más oscuro para contraste)

Estilo macOS aplicado:

- Sidebar translúcido (rgba con alpha < 1) + bordes redondeados (16px) + border subtle.
- Items de sidebar con border-radius 9px, hover en `--accent-bg-hover`, item activo con fondo `--accent` y texto `--accent-text`.
- Grupos tipo "card" con padding 4px, border radius 12px y border subtle.
- Rows con radius 8px, hover en `--row-bg-hover`.
- Switches estilo iOS/macOS: 42x24 con border-radius 14px, slider 18x18 que se desplaza 22px al activar (en lugar del switch nativo GTK).
- Sliders con trough 5px, slider 14px redondo con sombra.
- DropDowns con radius 8px y hover en accent.
- Popover list con radius 6px por item.
- Scrollbar fina (8px) con color `--scroller-thumb` y hover en accent.
- About card con border radius 16px.
- Wallpaper thumbnails con radius 10px y selected border 3px accent.
- Back button de subpáginas con radius 8px.

Sin reglas duplicadas, sin hardcodes blancos fuera de los tokens.

### Subpáginas con botón "Atrás"

`widgets/page.py` extendido:

- Nuevo kwarg `parent_page=None`. Si se pasa, añade un botón "Atrás" al inicio de la página usando el icono `go-previous-symbolic`.
- `on_back()` llama `self.navigator.show_page(self.parent_page)`.

Aplicado a:

- `pages/accent.py` → `parent_page="appearance"`
- `pages/icons.py` → `parent_page="appearance"`
- `pages/cursor.py` → `parent_page="appearance"`
- `pages/fonts.py` → `parent_page="appearance"`
- `pages/wallpaper.py` → `parent_page="appearance"`
- `pages/power_profile.py` → `parent_page="power"`
- `pages/battery.py` → `parent_page="power"`
- `pages/display_timeout.py` → `parent_page="power"`
- `pages/sleep.py` → `parent_page="power"`

### Página de entrada (input) válida

`pages/input.py` antes era un stub de 12 líneas con constructor roto (`__init__(self)` en vez de `__init__(self, navigator)` y llamada `super().__init__("Entrada")` con un solo arg). Reescrito:

- Group "Teclado" con `ComboRow` de layouts (`es`, `us`, `latam`, `fr`, `de`, `it`).
- Group "Ratón" con `SwitchRow` "Tocar para clic" (vía `gsettings org.gnome.desktop.peripherals.mouse tap-to-click`).
- Group "Velocidad" con `SliderRow` (-100..100) para `gsettings org.gnome.desktop.peripherals.mouse speed`.

Añadido al sidebar como "Entrada" (icon `input.svg`) entre Pantalla y Audio. Registrada en `window.py` como `"input"` page.

### PowerService completado

`services/power.py` añadido:

- `set_power_profile(profile)` — `powerprofilesctl set <profile>`.
- `set_screen_timeout(seconds)` — `gsettings set org.gnome.desktop.session idle-delay <n>`.
- `set_sleep_timeout(seconds)` — `gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-ac-timeout <n>`.
- `set_lid_close_action(action)` — `gsettings set org.gnome.settings-daemon.plugins.power lid-close-ac-action <action>`.

### Páginas de power sub-functions ahora funcionan

Antes `pages/power.py` referenciaba subpáginas `power-profile`, `battery`, `display-timeout`, `sleep` vía `NavigationRow`, pero NO existían como navigator pages registradas en `window.py`. Al pulsar no pasaba nada.

Creadas y registradas:

- `pages/battery.py` — estado real con upower (present/percentage/state/profile).
- `pages/power_profile.py` — combo performance/balanced/power-saver con labels traducidos y callback `on_profile_changed` (invoca `PowerService.set_power_profile`).
- `pages/display_timeout.py` — combo "1m/2m/5m/10m/15m/30m/Nunca".
- `pages/sleep.py` — combo timeout + combo acción al cerrar tapa.

Todas registradas en `window.py` con `add_page("power-profile", ...)`, etc.

### WallpaperPage reescrito con thumbnails reales

Antes mostraba cada wallpaper como un `Row` con título y subtítulo. Reescrito con:

- `Gtk.FlowBox` con `set_max_children_per_line(4)`, `set_min_children_per_line(2)`.
- Cada thumbnail es un `Gtk.Box` vertical con `Gtk.Image.new_from_paintable(Gdk.Texture.new_from_filename(path))` a 120px.
- Imagen actual destacada con clase `wallpaper-selected` (borde accent).
- Fallback a `image-missing` si no puede cargar.
- Label con el nombre del archivo, ellipsize END, tooltip.
- Preview del fondo actual a 160px en un Group separado arriba.

### i18n module

Antes `preferences/i18n.py` era el único que existía; los popups lo referenciaban pero no estaba en el path. Ahora:

- Copia:`archiso/airootfs/usr/share/churros/i18n.py` (idéntico a `preferences/i18n.py`).
- Las apps solo necesitan `churros/` en el `sys.path` para importar `i18n`.
- `preferences/i18n.py` se mantiene para no romper imports antiguos dentro de preferences (que tienen `.` en el path).

### profiledef.sh

- Quitada la referencia a `churros-welcome/churros-welcome.sh` (eliminado en sesión anterior, pero la línea en profiledef seguía).
- Permisos declarados para `/usr/bin/churros-popup` (0755), `/usr/bin/churros-control-center` (0755), `/usr/bin/churros-settings` (0755), `/usr/bin/churros-welcome` (0755).

---

## 2026-08-07 — Preferences Brain (release v0.4.0)

Sesión de 60 commits centrada en madurar el panel de preferencias, eliminar bugs latentes y ampliar la cobertura de la GUI.

### Selector de zona horaria integrado

- `services/datetime.py` nuevo: `DatetimeService` con `get_timezone()`, `get_ntp()`, `list_timezones()` (598 zonas), `set_timezone()`/`set_ntp()` vía `churros-pkexec`.
- `pages/datetime.py` reescrito: reloj vivo (segundo a segundo), switch NTP integrado, **selector de zona con `Gtk.SearchEntry` que filtra en vivo** y abre un `Popover` con `ListBox` de resultados (max 200 visibles). Al hacer clic, aplica via `churros-pkexec` (polkit).
- `50-churros-store.rules` añadida autorización para `timedatectl` junto a `pacman/flatpak/yay/paru`.
- Eliminado el flujo anterior con `fzf`+`foot`; ahora todo se hace dentro de la GUI.

### Nuevas páginas

- **Mako** (`pages/mako.py`) — Tipografía, colores, bordes, disposición, padding, comportamiento + reload + **toggle DND**.
- **Backup** (`pages/backup.py`) — Exportar/importar/resetear la config de `~/.config/churros/`.
- **Night Light** (`pages/night_light.py`) — `wlsunset` (temperatura, gamma, automático).
- **Lock Screen** (`pages/lock_screen.py`) — `swaylock` + `swayidle`.
- **Window Rules** (`pages/window_rules.py`) — Editor visual de `window-rule {}` blocks en KDL.
- **Logs** (`pages/logs.py`) — Ver logs del sistema.
- **Niri** (`pages/niri.py`) — Toggle animaciones + sliders de duración de `window-open` y `workspace-switch`.

### Bugs críticos resueltos

- `NiriConfig.reload()` añadido (`f19966e`). Antes 11 sitios llamaban `NiriConfig.reload()` que no existía → `AttributeError`. Ahora envía `pkill -HUP niri`.
- `DisplayService.set_vrr()` añadido (`f19966e`). Faltaba wrapper; backend base ya lo tenía.
- `ComboRow` reescrito como `Gtk.Box` directo (`e1c2477`). Antes heredaba de `Gtk.Button` → el button capturaba el click del dropdown. Bug manifiesto: pulsar la flecha no abría el menú.
- `Row.set_title()` / `Row.set_subtitle()` añadidos como API pública (`f19966e`).
- `ColorPickerRow` acepta `subtitle` kwarg (`e517f4f`). Bug rompía `do_activate()` al construir `LockScreenPage`.
- `display_timeout.py` — `parent_page` corregido de `"power"` a `"display"` (`18ac16a`).
- `pages/shortcuts.py` borrada (`0b9bcd5`) — duplicaba `keyboard.py` (la única página de atajos).
- `archiso/packages/` — 9 artefactos de pacman quitados del índice (`ed7ab08`).
- `churros-settings` sin permisos de ejecución (`65c5f95`).
- Screen negro al cambiar wallpaper + modo oscuro (`ecd4adc`).
- OVMF_VARS.fd copia independiente (`dea9249`).

### Robustez y logs

- `churros-settings` (launcher bash) blindado (`677789f`):
  - Output redirigido a `$XDG_RUNTIME_DIR/churros-settings.log` (fallback `/tmp/churros-settings.log`).
  - Autodetección de `WAYLAND_DISPLAY` si no exportado.
  - Si la app falla, no aborta la sesión del usuario.
- `main.py` blindado (`677789f`):
  - Cada bloque crítico envuelto en try/except.
  - Errores se loguean con stack trace completo.
  - `_build_wallpaper` ahora no muere si el wallpaper no existe.

### Apariencia reorganizada

`pages/appearance.py` reorganizada en 9 grupos temáticos (`053872d`):

1. Tema (claro/oscuro)
2. Color de acento
3. Tipos de letra
4. Cursor
5. Iconos
6. Fondo de pantalla
7. Esquinas / forma
8. Pywal (paleta dinámica)
9. Waybar

ThemeService y AccentService tienen **hooks pywal** — cuando pywal está activo, regenera paleta desde el wallpaper.

### Búsqueda mejorada

- Search global con subpáginas (`36d9b7e`) — buscar "modo oscuro" salta a `Apariencia → Tema` y resalta el row correspondiente.
- Layout responsive — sidebar narrow en pantallas estrechas.
- Color picker entry editable — pegar hex manualmente funciona.

### Atajos globales

`PreferencesWindow` registra (`2d7f7bf`):

| Atajo | Acción |
|-------|--------|
| `Ctrl+F` | Foco al search del sidebar |
| `Ctrl+B` | Toggle sidebar narrow |
| `Shift+Ctrl+N` | Toggle sidebar narrow (alias) |

### Pulido de páginas existentes

- **Power profile** — descripciones + advertencias sobre rendimiento (`6b1c8a6`).
- **Sleep** — estado batería integrado (`6b1c8a6`).
- **Fonts** — preview en vivo + debounce (`6b1c8a6`).

### CI

- `./churros check` corre en GitHub Actions (`.github/workflows/ci.yml`) en cada push a `main` y PR.
- Verifica: bash syntax, shellcheck error level, Python syntax, duplicate entries en `packages.x86_64`, Niri autostart commands resolvibles, `msgfmt --check` en `po/*.po`, repo hygiene (no generated files tracked).

---

## 2026-07-16

- Added `python-psutil` to `archiso/packages.x86_64` so `churros-welcome` can import `psutil` in the live image.
- Updated `apps/churros-welcome/src/utils/system.py` and `archiso/airootfs/usr/share/churros/churros-welcome/src/utils/system.py` to import `psutil` safely and fall back to `/proc/meminfo` when `psutil` is missing.
- Verified that Niri autostart already includes `spawn-at-startup "churros-welcome"` in `/etc/skel/.config/niri/config.kdl`.
- Did not change autostart configuration itself; the fix was ensuring the welcome app can launch without `psutil` missing in the image.
- Updated ChurrOS Welcome styling in `apps/churros-welcome/assets/style.css` and `archiso/airootfs/usr/share/churros/churros-welcome/assets/style.css` for a darker branded theme, softer shadows, and consistent spacing.
- Updated Waybar styling in `archiso/airootfs/etc/skel/.config/waybar/style.css` to match ChurrOS branding with rounded panels, improved workspace button styles, and cleaner tray item appearance.
