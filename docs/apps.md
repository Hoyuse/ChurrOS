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
**Autostart:** `archiso/airootfs/etc/skel/.config/hypr/autostart.conf` (`exec-once = churros-welcome`)

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
    ├── desktop.py         # Lanzar apps (kitty, firefox)
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

La pantalla principal muestra seis tarjetas de acción (más la `SystemCard`):

| Icono | Título | Callback |
|-------|--------|----------|
| documentation.svg | Documentación | Abre wiki en el navegador |
| applications.svg | Aplicaciones | Abre terminal |
| github.svg | GitHub | Abre el repositorio |
| community.svg | Comunidad | (placeholder) |
| customize.svg | Personalizar | (placeholder) |
| update.svg | Actualizar | (placeholder) |

Las tarjetas están organizadas en un `Gtk.FlowBox` con un máximo de 4 columnas. En pantallas estrechas se reorganizan automáticamente.

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
| 0 | AudioCard | BatteryCard |
| 1 | NetworkCard | BluetoothCard |
| 2 | CalendarCard (ancho 2) | — |

## Services

Cada tarjeta consulta un servicio (ver `docs/services.md`):

- `services/audio.py` + `widgets/audio.py` → AudioCard
- `services/battery.py` + `widgets/battery.py` → BatteryCard
- `services/network.py` + `widgets/network.py` → NetworkCard
- `services/bluetooth.py` + `widgets/bluetooth.py` → BluetoothCard
- `widgets/calendar.py` → CalendarCard (sin servicio)

## Style

- Fondo raíz: `#1f1f1f`
- Tarjetas: `#2b2b2b`, radio 16px
- Hover: `#333333`
- Acento: `#ff8c00` (sliders, switches activos, calendar selection)
- Texto: blanco, secundario `#bdbdbd`

---

# churros-launcher

**Path:** `archiso/airootfs/usr/share/churros/launcher/`
**Wrapper:** `/usr/bin/churros-launcher`
**Keybind:** `SUPER + SPACE` (definido en `archiso/airootfs/etc/skel/.config/hypr/keybinds.conf`)

Launcher de aplicaciones al estilo Spotlight/Rofi, pero GTK4.

## Window

- Tamaño: 700×500, no redimensionable, **sin decoración** (se ve como un popup flotante)
- Layout vertical: barra de búsqueda + lista de apps
- Margen: 20px

## Search

`widgets/search.py` extiende `Gtk.SearchEntry`:

- Placeholder: "Search applications..."
- Emite la señal `search-changed` en cada cambio de texto

## App List

`widgets/applist.py` usa `Gio.AppInfo.get_all()` para enumerar todas las aplicaciones instaladas en el sistema. Filtra con `app.should_show()` para omitir apps ocultas, y las ordena alfabéticamente.

Cada fila (`widgets/approw.py`) muestra:

- Icono (del `.desktop` o genérico si no tiene)
- Nombre
- Hover: `app-row` CSS class

Al pulsar Enter o hacer clic en una fila:

1. Se llama a `app.launch()` (vía `Gio.AppInfo.launch`)
2. Se cierra la ventana del launcher

## Filter

El filtrado es en tiempo real y por nombre. No hay coincidencia difusa (fuzzy match): se usa `text in application["name"].lower()`. Esto puede mejorarse en versiones futuras.

---

# churros-ui

**Path:** `apps/churros-ui/`
**Estado:** Planificado, en desarrollo.

Biblioteca de componentes UI compartidos que las apps oficiales usarán para mantener una identidad visual consistente.

## Roadmap

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

## Consumers

Las apps que la utilizarán:

- `churros-welcome`
- `churros-settings` (futuro)
- `churros-installer` (futuro)
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
- Sustituir los placeholders de las tarjetas de acción (Comunidad, Personalizar, Actualizar) por acciones reales.
- Internacionalización: las cadenas están en español hardcodeadas. Hace falta un sistema `gettext` o similar.
- Tests: no hay suite de tests. Las apps interactúan con el sistema, así que los tests serían de integración con un display virtual.
