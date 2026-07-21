# Desktop Config

Este documento describe la configuración del escritorio Live de ChurrOS: Niri, Waybar, SDDM y el usuario live.

La configuración se aplica a todo usuario nuevo del sistema gracias a que vive en `/etc/skel/.config/`, que el script `desktop.sh` (ver `docs/live-services.md`) copia a `/home/churros/` durante la inicialización del Live.

---

# Niri

**Path:** `archiso/airootfs/etc/skel/.config/niri/config.kdl`

Niri es el compositor Wayland usado por ChurrOS. Es un compositor desplazable (scrollable-tiling) escrito en Rust. Toda la configuración vive en un solo archivo `config.kdl` en formato KDL.

## Structure

```text
niri/
└── config.kdl           # Único archivo de configuración
```

## Environment

Las variables de entorno se definen en `config.kdl` dentro del bloque `environment`:

```kdl
environment {
    QT_QPA_PLATFORM "wayland"
    GDK_BACKEND "wayland"
    MOZ_ENABLE_WAYLAND "1"
}

cursor {
    xcursor-size 24
}
```

Fuerza a las apps Qt y GTK a usar Wayland en lugar de XWayland cuando es posible. Firefox usará Wayland nativo gracias a `MOZ_ENABLE_WAYLAND`.

## Input

```kdl
input {
    keyboard {
        xkb {
            layout "us"
        }
    }
}
```

Layout de teclado US.

## Layout

```kdl
layout {
    gaps 8
    border {
        on
        width 2
        active-color "#f97316"
        inactive-color "#4a4a4a"
    }
}
```

- Gaps: 8px entre ventanas y bordes
- Borde de 2px con color naranja ChurrOS en la ventana activa

## Keybinds

| Atajo | Acción |
|-------|--------|
| `SUPER + Return` | Abre Kitty (terminal) |
| `SUPER + Q` | Cierra la ventana activa |
| `SUPER + M` | Sale de Niri |
| `SUPER + F` | Maximiza columna |
| `SUPER + SPACE` | Abre el launcher (churros-launcher) |
| `SUPER + C` | Abre el centro de control |
| `SUPER + 1-9` | Cambia al workspace N (1-9) |
| `SUPER + SHIFT + 1-9` | Mueve la ventana activa al workspace N |
| `SUPER + ←/→` | Mueve el foco entre columnas |
| `SUPER + ↑/↓` | Mueve el foco entre ventanas |
| `SUPER + SHIFT + ←/→` | Mueve la columna |
| `SUPER + SHIFT + ↑/↓` | Mueve la ventana |

## Autostart

```kdl
spawn-at-startup "awww-daemon"
spawn-at-startup "awww" "img" "/usr/share/churros/wallpapers/default.jpeg" "--transition-type" "none"
spawn-at-startup "waybar" "--config" "/home/churros/.config/waybar/config.jsonc"
spawn-at-startup "mako"
spawn-at-startup "churros-welcome"
```

`awww-daemon` carga el wallpaper. `waybar` arranca la barra superior. `mako` es el daemon de notificaciones. `churros-welcome` muestra la pantalla de bienvenida.

---

# Waybar

**Path:** `archiso/airootfs/etc/skel/.config/waybar/`

Waybar es la barra superior de ChurrOS. Configurada con estilo dark y acento naranja.

## Config

`config.jsonc`:

```jsonc
{
    "layer": "top",
    "position": "top",
    "height": 40,
    "margin-top": 12,
    "margin-left": 16,
    "margin-right": 16
}
```

### Modules

| Posición | Módulos |
|----------|---------|
| Izquierda | `image#logo`, `niri/workspaces` |
| Centro | `niri/window` |
| Derecha | `tray`, `network`, `custom/bluetooth`, `pulseaudio`, `custom/brightness`, `battery`, `clock`, `custom/power` |

### Module Actions

| Módulo | Acción |
|--------|--------|
| `network` | Clic → popup de red. Tooltip con info. |
| `custom/bluetooth` | Clic → popup de bluetooth |
| `pulseaudio` | Clic → popup de audio. Clic derecho → toggle mute. Scroll → ±5% volumen. |
| `custom/brightness` | Clic → popup de brillo |
| `battery` | Clic → popup de batería |
| `clock` | Tooltip con calendario |
| `custom/power` | Clic → popup de power |

Los iconos usados son glyphs Nerd Font (`󰈀 󰖩 󰖪 󰂯 󰕾 󰃠 󰁹` etc.).

## Style

`style.css`:

- Fondo: `rgba(31,31,31,0.96)` (gris casi negro, 96% opacidad)
- Borde: 1px sólido `rgba(249,115,22,0.25)` (naranja al 25%)
- Radio: 16px (esquinas redondeadas)
- Padding: 6px
- Margen exterior: 10px (para que se vea "flotante")

Los workspaces usan fondo `rgba(255,255,255,0.05)` y al activarse se vuelven naranja sólido (`#f97316`).

Los módulos individuales comparten estilo:

```css
background: rgba(255,255,255,0.05);
color: #f5f5f5;
border-radius: 10px;
margin: 4px;
padding: 6px 14px;
```

Hover:

```css
background: rgba(249,115,22,0.15);
```

Tipografía: `JetBrainsMono Nerd Font`, 14px en todo.

---

# SDDM Autologin

**Path:** `archiso/airootfs/etc/sddm.conf.d/autologin.conf`

```ini
[Autologin]
User=churros
Session=niri

[General]
InputMethod=
```

SDDM arranca, autologin con el usuario `churros`, y carga la sesión `niri`. Esto permite que el Live entre directamente al escritorio sin pedir credenciales.

El input method está vacío por defecto. Si añades soporte para otro idioma con caracteres especiales, edita esta línea.

---

# Live User

**Script:** `archiso/airootfs/root/scripts/users.sh`

```bash
useradd -m \
    -G wheel,audio,video,input,storage,network \
    -s /bin/bash \
    churros

passwd -d churros

echo "churros ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/churros
chmod 440 /etc/sudoers.d/churros
```

El usuario `churros`:

- Pertenece a `wheel`, `audio`, `video`, `input`, `storage`, `network` (todos los grupos necesarios para usar el hardware)
- Shell: bash
- Sin contraseña (`passwd -d` la vacía)
- `sudo NOPASSWD` para que las acciones administrativas no pidan credencial
- Home: `/home/churros/`

El script es seguro para el Live porque el sistema corre en RAM: cualquier cambio se pierde al apagar. En el sistema instalado (futuro), se deberá crear un usuario con contraseña.

---

# Init Order

Durante el arranque del Live, los servicios y la configuración se aplican en este orden:

1. systemd arranca y carga los servicios base (NetworkManager, etc).
2. `getty@tty1.service` hace autologin como `root` en tty1.
3. `.zlogin` ejecuta `/root/.automated_script.sh`.
4. `customize_airootfs.sh` se ejecuta (ver `docs/live-services.md`):
   - Crea el usuario `churros` (`users.sh`)
   - Habilita NetworkManager y SDDM (`services.sh`)
   - Copia la configuración de `/etc/skel/` a `/home/churros/` (`desktop.sh`)
   - Limpia la cache de pacman (`cleanup.sh`)
5. SDDM arranca, autologin como `churros`, carga `niri`.
6. Niri lee `config.kdl` y ejecuta los `spawn-at-startup` (awww-daemon, waybar, churros-welcome).
7. Waybar arranca y carga los popups de los módulos.

---

# Customization

## Cambiar el layout de teclado

Edita `archiso/airootfs/etc/skel/.config/niri/config.kdl`:

```kdl
input {
    keyboard {
        xkb {
            layout "es"    # o "us,es" para alternar
        }
    }
}
```

## Cambiar el wallpaper

Reemplaza `archiso/airootfs/usr/share/churros/wallpapers/default.jpeg` con tu imagen. `awww-daemon` la carga automáticamente al inicio.

## Cambiar los gaps

Edita `archiso/airootfs/etc/skel/.config/niri/config.kdl`:

```kdl
layout {
    gaps 8   // valor único (todos los lados)
}
```

## Añadir un módulo a Waybar

Edita `archiso/airootfs/etc/skel/.config/waybar/config.jsonc` y añade el módulo en `modules-right`. Los módulos nativos están listados en la documentación oficial de Waybar.

## Cambiar la posición de la barra

En `config.jsonc`:

```jsonc
"position": "top"    // top, bottom, left, right
```

## Cambiar el tema

`style.css` define todo el estilo. Los colores principales están centralizados en las primeras líneas: cambia `rgba(31,31,31,...)` para el fondo y `#f97316` para el acento.

---

# Future Work

- Soporte multi-monitor: configuración de outputs pendiente.
- Perfiles de energía: cambiar decoraciones automáticamente según si el equipo está en batería.
- Configuración por usuario: hoy `/etc/skel/` aplica a todos. En el sistema instalado se deberá copiar a `~/.config/niri/` para permitir personalizaciones.
- Integración con `wlogout`: el script de salida que se muestra desde el popup de power.
