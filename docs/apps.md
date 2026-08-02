# Apps

Este documento describe las aplicaciones oficiales de ChurrOS.

Todas las apps están escritas en **Python 3** usando **GTK4** y **Libadwaita**. La estructura común es:

```text
app/
├── main.py             # Entry point (Gtk.Application)
├── window.py           # Ventana principal
├── widgets/            # Componentes reutilizables
├── services/           # (opcional) Lógica de negocio
├── assets/             # Iconos, CSS
└── README.md
```

Las apps se instalan en `/usr/share/churros/<app>/` dentro de la ISO Live y se ejecutan mediante binarios en `/usr/bin/` (wrappers que hacen `cd` al directorio de la app y llaman a `python3 main.py`).

---

# churros-welcome

**Path:** `apps/churros-welcome/`
**Instalada en:** `/usr/share/churros/churros-welcome/`
**Wrapper:** `/usr/bin/churros-welcome`
**Autostart:** `archiso/airootfs/etc/skel/.config/niri/config.kdl` (`spawn-at-startup "churros-welcome"`)

La pantalla de bienvenida que se muestra al iniciar la sesión Live.

## Purpose

- Dar la bienvenida al usuario.
- Mostrar información básica del sistema (CPU, RAM, kernel, SO, arquitectura, hostname).
- Ofrecer accesos rápidos a documentación, GitHub, comunidad, personalización y actualización.

## Stack

- **GTK 4.0** + **Libadwaita 1** (a través de PyGObject)
- **psutil** (opcional): con fallback a `/proc/meminfo` si no está disponible
- **Python 3.14+**

## Window

- Tamaño por defecto: 1100×720 (definido en `src/config/constants.py`)
- Tamaño mínimo: 640×480 (para pantallas pequeñas)
- Layout: vertical con scroll (`Gtk.ScrolledWindow`)
- CSS cargado desde `assets/style.css` con prioridad `APPLICATION`

## Structure

```text
src/
├── main.py                # Entry point
├── window.py              # ChurrOSWelcome (Adw.Application)
├── pages/
│   └── home.py            # Página principal
├── config/
│   ├── constants.py       # WINDOW_WIDTH, CARD_WIDTH, etc.
│   ├── metadata.py        # APP_NAME, VERSION, REPOSITORY, etc.
│   └── paths.py           # Rutas a assets
├── ui/
│   ├── header.py          # Logo, título, subtítulo
│   ├── cards.py           # Grid de tarjetas
│   └── footer.py          # Pie de página
├── widgets/
│   ├── action_card.py     # Tarjeta de acción (botón)
│   └── system_card.py     # Tarjeta con info del sistema
├── service/
│   ├── welcome.py
│   ├── applications.py
│   ├── package_manager.py
│   └── updater.py
└── utils/
    ├── browser.py         # Abrir URLs
    ├── commands.py
    ├── desktop.py         # Lanzar apps (foot, firefox, calamares)
    └── system.py          # get_cpu, get_memory, etc.
```

## System Card

La `SystemCard` muestra información en tiempo real del sistema:

| Campo | Fuente | Fallback |
|-------|--------|----------|
| CPU | `/proc/cpuinfo` (`model name`) | "Desconocido" |
| RAM | `psutil.virtual_memory()` o `/proc/meminfo` | "Desconocido" |
| Kernel | `platform.release()` | — |
| SO | `/etc/os-release` (`PRETTY_NAME`) | `platform.system()` |
| Arquitectura | `platform.machine()` | — |
| Hostname | `platform.node()` | — |

La memoria RAM usa `psutil` si está disponible. Si no, lee `/proc/meminfo` directamente y formatea en GiB. Esto permite que la app funcione incluso si `psutil` no se instaló en la ISO.

## Action Cards

La pantalla principal muestra la `SystemCard` más 6 tarjetas de acción:

| Icono | Título | Callback |
|-------|--------|----------|
| install.svg | Install ChurrOS | Lanza `calamares.desktop` vía `Gio.DesktopAppInfo.new("calamares.desktop").launch()`. Si no existe muestra un `Gtk.AlertDialog` informativo. |
| github.svg | GitHub | Abre el repositorio en el navegador. |
| community.svg | Discord | Abre la comunidad. |
| documentation.svg | Documentation | Abre la wiki. |
| terminal.svg | Terminal | Lanza `foot`. |
| browser.svg | Browser | Lanza `firefox`. |

La `SystemCard` no es clickable, solo informativa (CPU, RAM, kernel, SO, arquitectura, hostname). Se muestra la primera en el FlowBox.

Las tarjetas están organizadas en un `Gtk.FlowBox` con un máximo de 4 columnas y mínimo de 1. En pantallas estrechas se reorganizan automáticamente.

## Desktop Entry

`archiso/airootfs/usr/share/applications/churros-welcome.desktop`:

```ini
[Desktop Entry]
Name=ChurrOS Welcome
Exec=churros-welcome
Terminal=false
Categories=System;
X-GNOME-Autostart-enabled=true
```

---

# churros-control-center

**Path:** `archiso/airootfs/usr/share/churros/control-center/`
**Wrapper:** `/usr/bin/churros-control-center` (o `python /usr/share/churros/control-center/main.py` desde desktop entry)
**Desktop entry:** `archiso/airootfs/usr/share/applications/churros-control-center.desktop`

Centro de control con tarjetas para los componentes principales del sistema.

## Window

- Tamaño: 520×570, no redimensionable, sin decoración de ventana
- Layout: `Gtk.Grid` con 2 columnas y 3 filas
- Espaciado: 12px entre celdas, 16px de margen
- CSS: estilo dark con acento naranja (`#ff8c00`)

## Cards

| Fila | Columna 0 | Columna 1 |
|------|-----------|-----------|
| 0 | NetworkCard | BluetoothCard |
| 1 | BrightnessCard | BatteryCard |
| 2 | AudioCard (ancho completo) | — |

Cada card es un `Card` (Gtk.Button) que al hacer clic llama a `popup_launcher.open_<name>(window)`, que lanza el popup correspondiente en un proceso separado (vía `subprocess.Popen([sys.executable, popup_main])`).

Header del control center tiene un botón de "Settings" (icon `preferences-system`) que lanza `churros-settings` y cierra el control center.

## Services

Cada tarjeta consulta un servicio compartido en `/usr/share/churros/services/`:

- `services/wifi.py` + `services/ethernet.py` → `widgets/network.py` → NetworkCard
- `services/bluetooth.py` → `widgets/bluetooth.py` → BluetoothCard
- `services/audio.py` → `widgets/audio.py` → AudioCard
- `services/brightness.py` → `widgets/brightness.py` → BrightnessCard
- `services/battery.py` → `widgets/battery.py` → BatteryCard

i18n: el control center importa `from i18n import _` desde `/usr/share/churros/i18n.py` (módulo central añadido para que popups + control center + preferences lo compartan).

## Style

- Fondo: variables CSS del `style.css` del propio control center.
- Acento: `#DE8636` (ChurrOS naranja).
- Polling: cada 2s hace refresh de todas las cards (vía `GLib.timeout_add_seconds(2, self.refresh)`).

---

# churros-settings

**Path:** `archiso/airootfs/usr/share/churros/preferences/`
**Wrapper:** `/usr/bin/churros-settings` → `python3 /usr/share/churros/preferences/main.py`
**Keybind:** `SUPER + P` (definido en `archiso/airootfs/etc/skel/.config/niri/config.kdl`)

App de configuración principal, estilo System Settings de macOS con colores ChurrOS. Documentación dedicada en `docs/preferences.md`.

---

# fuzzel (launcher)

**Path:** binario del sistema (instalado vía `archiso/packages.x86_64`).
**Keybind:** `SUPER + SPACE` (definido en `archiso/airootfs/etc/skel/.config/niri/config.kdl`)
**Waybar:** `custom/launcher` → `on-click: "fuzzel"` (clic derecho → foot).

El launcher de aplicaciones es **fuzzel** ( Wayland-native, ligero), no una app ChurrOS propia. Muestra la lista de apps instaladas (leídas desde los `.desktop` del sistema), filtrado incremental por teclado, y lanza la seleccionada vía la DBus launcheable del .desktop.

Config: `archiso/airootfs/etc/skel/.config/fuzzel/fuzzel.ini` (tipografía, colores, atajos).

Razón: fuzzel cumple el rol de "Spotlight" sin necesitar write + mantain una app GTK4 custom para ello. Si en el futuro se quiere hacer un launcher con previews / acciones extra (estilo Raycast), sería momento de sustituirlo por una app propia.

---

# churros-ui (planificado)

**Estado:** Aún no implementado. Idea original pendiente.

Biblioteca de componentes UI compartidos que las apps oficiales usarían para mantener una identidad visual consistente. Hoy cada app (welcome, control-center, preferences, popups) tiene su propio `style.css` con variables CSS similares pero duplicadas.

Roadmap potencial:

### v0.1
- `ActionCard`
- `InfoCard`
- `Header`
- `Footer`

### v0.2
- `Sidebar`
- `Dialogs`
- `Buttons`
- `Navigation`

### v0.3
- Animaciones
- Temas
- Componentes avanzados

## Consumers esperados

- `churros-welcome`
- `churros-settings`
- `churros-control-center`
- Popups
- Cualquier herramienta oficial nueva

---

# Packaging

Las apps se incluyen en la ISO copiando su contenido a `archiso/airootfs/usr/share/churros/<app>/`. La estructura ya está sincronizada: `apps/<app>/` es la fuente y `archiso/airootfs/usr/share/churros/<app>/` es la copia que se incluye en la imagen.

> **Nota:** El `build.sh` no copia automáticamente las apps. La sincronización es manual. Revisa siempre que ambas copias estén alineadas antes de hacer un commit.

---

# Development

Para modificar una app:

1. Edita el código en `apps/<app>/` (o en `archiso/airootfs/usr/share/churros/<app>/` si el archivo solo existe ahí).
2. Si solo existe en `archiso/`, cópialo a `apps/` para mantener la paridad.
3. Compila y prueba:

```bash
./churros build
./churros run
```

4. Confirma que la app arranca y se ve correctamente.

---

# Future Work

- Mover las apps a un repositorio separado: hoy viven dentro del repo de la distro. A largo plazo deberían empaquetarse e instalarse vía pacman.
- Sustituir placeholders de las tarjetas de_action antiguas — hecho: hoy Welcome tiene Install/GitHub/Discord/Documentation/Terminal/Browser todas funcionales.
- Internacionalización: hoy `i18n._()` (`/usr/share/churros/preferences/i18n.py` y la copia en `/usr/share/churros/i18n.py`) simplemente devuelve la string original; falta compilar `po/churros.po` a `/usr/share/locale/es/LC_MESSAGES/churros.mo`.
- churros-ui: centralizar el CSS + widgets compartidos (hoy hay code duplication entre welcome/control-center/preferences/popups).
- Tests: no hay suite de tests. Las apps interactúan con el sistema; verificación es `python3 -c "import ast; ast.parse(open(f).read())"` + lanzar manualmente cada app en niri.
