# Desktop Config

Este documento describe la configuración del escritorio Live de ChurrOS: Niri, Noctalia, greetd y el usuario live.

La configuración se aplica a todo usuario nuevo del sistema gracias a que vive en `/etc/skel/.config/`, que el script `desktop.sh` (ver `docs/live-services.md`) copia a `/home/churros/` durante la inicialización del Live.

---

# Niri

**Path:** `archiso/airootfs/etc/skel/.config/niri/config.kdl`

Niri es el compositor Wayland usado por ChurrOS. Es un compositor desplazable (scrollable-tiling) escrito en Rust. Toda la configuración vive en un solo archivo `config.kdl` en formato KDL. El panel, las notificaciones y el fondo los pinta **Noctalia** (shell, no compositor).

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
    background-color "transparent"
    border {
        on
        width 2
        active-color "#f97316"
        inactive-color "#4a4a4a"
    }
}
```

- Gaps: 8px entre ventanas y bordes
- Fondo de workspace transparente para que se vea el wallpaper de Noctalia
- Borde de 2px con color naranja ChurrOS en la ventana activa

## Keybinds

| Atajo | Acción |
|-------|--------|
| `SUPER + Return` | Abre foot (terminal) |
| `SUPER + Q` | Cierra la ventana activa |
| `SUPER + M` | Sale de Niri |
| `SUPER + F` | Maximiza columna |
| `SUPER + SHIFT + F` | Pantalla completa |
| `SUPER + SPACE` | Abre el launcher (fuzzel) |
| `SUPER + C` | Abre el centro de control |
| `SUPER + P` | Abre preferencias (churros-settings) |
| `SUPER + W` | Abre churros-welcome |
| `SUPER + S` | Abre Bazaar (tienda Flatpak) |
| `SUPER + V` | Toggle ventana flotante |
| `SUPER + SHIFT + V` | Cambiar foco entre floating y tiling |
| `SUPER + O` | Toggle overview |
| `SUPER + R` | Cambiar preset de ancho de columna |
| `SUPER + SHIFT + N` | Popup de red |
| `SUPER + SHIFT + A` | Popup de audio |
| `SUPER + SHIFT + B` | Popup de bluetooth |
| `SUPER + SHIFT + L` | Popup de brillo |
| `SUPER + SHIFT + T` | Popup de batería |
| `SUPER + SHIFT + E` | Popup de energía |
| `Print` | Screenshot interactivo |
| `Ctrl + Print` | Screenshot de pantalla |
| `Alt + Print` | Screenshot de ventana |
| `SUPER + 1-9` | Cambia al workspace N (1-9) |
| `SUPER + SHIFT + 1-9` | Mueve la ventana activa al workspace N |
| `SUPER + ←/→` | Mueve el foco entre columnas |
| `SUPER + ↑/↓` | Mueve el foco entre ventanas |
| `SUPER + SHIFT + ←/→` | Mueve la columna |
| `SUPER + SHIFT + ↑/↓` | Mueve la ventana |
| `XF86AudioRaise/Lower/Mute` | Volumen (con `allow-when-locked`) |
| `XF86AudioPlay/Next/Prev` | Control multimedia (con `allow-when-locked`) |
| `XF86MonBrightnessUp/Down` | Brillo (con `allow-when-locked`) |

## Autostart

```kdl
spawn-at-startup "noctalia"
spawn-at-startup "churros-portal-start"
spawn-at-startup "/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1"
spawn-at-startup "churros-welcome"
```

`noctalia` arranca el shell (barra, notificaciones y wallpaper). `churros-portal-start` arranca los xdg-desktop-portals. `churros-welcome` muestra la pantalla de bienvenida. Waybar, Mako y swaybg siguen en la ISO pero ya no se lanzan al entrar a la sesión.

Niri también incluye la regla de ventana de ajustes de Noctalia (`dev.noctalia.Noctalia`), `honor-xdg-activation-with-invalid-serial` para acciones de notificación, y un `layer-rule` que coloca el wallpaper de Noctalia en el backdrop del overview.

---

# Noctalia

**Path:** `archiso/airootfs/etc/skel/.config/noctalia/config.toml`

Noctalia es el shell de escritorio sobre Niri. El paquete es `noctalia` de `[extra]` (v5). El wallpaper oficial se declara aquí:

```toml
[wallpaper]
enabled = true
fill_mode = "crop"
directory = "/usr/share/churros/wallpapers"

[wallpaper.default]
path = "/usr/share/churros/wallpapers/default.png"
```

Los atajos de Niri todavía abren Fuzzel y los popups de ChurrOS; el IPC `noctalia msg` queda para un corte posterior.

---

# Waybar

**Path:** `archiso/airootfs/etc/skel/.config/waybar/`

Waybar queda en el skel y en `churros-settings`, pero el live ya no la arranca. Configurada con estilo dark y acento naranja.

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
| Izquierda | `custom/launcher`, `custom/sep`, `niri/workspaces`, `custom/sep`, `mpris` |
| Centro | (vacío) |
| Derecha | `tray` (group), `custom/sep`, `memory`, `disk`, `cpu`, `battery`, `backlight`, `custom/sep`, `custom/screenrecording-indicator`, `idle_inhibitor`, `custom/dnd`, `custom/sep`, `bluetooth`, `network`, `pulseaudio`, `custom/sep`, `custom/control-center`, `custom/settings`, `custom/sep`, `clock` |

### Module Actions

| Módulo | Acción |
|--------|--------|
| `custom/launcher` | Clic → fuzzel. Clic derecho → foot. |
| `custom/control-center` | Clic → `churros-control-center`. Tooltip "Control Center". |
| `custom/settings` | Clic → `churros-settings`. Tooltip "Settings". |
| `network` | Clic → popup de red. Tooltip con info. |
| `bluetooth` | Clic → popup de bluetooth. Tooltip con nº devices. |
| `pulseaudio` | Clic → popup de audio. Clic derecho → toggle mute. Scroll → ±5%. |
| `backlight` | Clic → popup de brillo. Scroll → ±10% brillo. |
| `battery` | Clic → popup de batería. |
| `cpu` | Clic → `foot btop`. Clic derecho → foot. |
| `clock` | Tooltip con calendario |
| `custom/dnd` | Clic → toggle "No molestar" (mako mode). |
| `idle_inhibitor` | Inhibe el idle. |
| `custom/screenrecording-indicator` | Indicador cuando wf-recorder está activo. |
| `mpris` | Playerctl. Clic izq → prev, centro → play/pause, der → next. Scroll → ±5%. |

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

# greetd Autologin

**Path:** `archiso/airootfs/etc/greetd/config.toml`

```toml
[terminal]
vt = 1

[default_session]
command = "/usr/bin/niri"
user = "churros"
```

greetd arranca en el VT1, autologin con el usuario `churros`, y lanza Niri directamente. Esto permite que el Live entre al escritorio sin pedir credenciales.

Tras la instalación, `greetd-config.sh` reescribe este archivo con el usuario creado por Calamares.

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

# XFCE Edition (X11)

**Path:** `archiso/airootfs/etc/skel/.config/xfce4/`
**Session command:** `/usr/bin/startxfce4`

ChurrOS XFCE Edition ofrece una experiencia de escritorio clásica, ligera y basada en ventanas flotantes tradicionales gestionadas por `xfwm4`.

## Estructura de configuración (xfconf y GTK)

```text
.config/
├── gtk-3.0/
│   └── gtk.css           # Tema oscuro con acento naranja (#F97316), panel y Whisker Menu
├── gtk-4.0/
│   └── gtk.css           # Tokens de color y acento ChurrOS para apps GTK 4
├── xfce4/
│   ├── panel/
│   │   └── whiskermenu-1.rc  # Configuración del menú de inicio (icono ChurrOS, favoritos)
│   ├── terminal/
│   │   └── terminalrc        # Paleta de colores ChurrOS, fuente JetBrainsMono y transparencia
│   └── xfconf/xfce-perchannel-xml/
│       ├── xfce4-desktop.xml # Wallpaper (/usr/share/churros/wallpapers/default.png) e iconos
│       ├── xfce4-panel.xml   # Panel superior: Whisker Menu, Tasklist, Systray, Pulseaudio, Reloj
│       ├── xfwm4.xml         # Gestor de ventanas: sombras de composición y botones |HMC
│       └── xsettings.xml     # Tema GTK (Adwaita-dark), iconos (Papirus-Dark) y cursor (Adwaita)
```

## Panel de ChurrOS en XFCE

El panel superior de 34px está estilizado con efecto cristal oscuro (`rgba(16, 16, 18, 0.94)`) y borde inferior naranja:
1. **Botón de Inicio ChurrOS**: Con el logo oficial de ChurrOS (`churros-logo.svg`) y menú **Whisker Menu** con barra de búsqueda superior y accesos directos a apps principales.
2. **Separador espaciador**.
3. **Lista de ventanas / tareas**: Botones planos con indicador inferior de color naranja ChurrOS (`#F97316`) en la ventana activa.
4. **Separador expandible**.
5. **Bandeja del sistema (Systray)**: Indicadores de red (`nm-applet`), Bluetooth y notificaciones.
6. **Plugin de Pulseaudio**: Control deslizante de volumen de audio.
7. **Plugin de Energía / Batería**: Monitor de batería y perfiles de energía.
8. **Reloj digital**: Formato legible `%I:%M %p`.
9. **Botones de sesión / Power**: Bloqueo, reinicio y apagado.

## Autostart en XFCE

Los programas de inicio automático para la sesión de XFCE se definen en `archiso/airootfs/etc/skel/.config/autostart/`:
- `churros-welcome.desktop`: Inicia automáticamente la pantalla de bienvenida al entrar al escritorio XFCE (`OnlyShowIn=XFCE;`).

## Integración con utilidades de ChurrOS

- **Fondo de pantalla:** `churros-apply-wallpaper` detecta XFCE y aplica el fondo automáticamente mediante `xfconf-query -c xfce4-desktop ...`.
- **Apps Rust:** Las aplicaciones como `churros-settings` y `churros-welcome` detectan que la sesión es XFCE y adaptan sus subpáginas y estilos (ocultando subpáginas exclusivas de Wayland/Niri).

---

# Init Order

Durante el arranque del Live, los servicios y la configuración se aplican en este orden:

1. systemd arranca y carga los servicios base (NetworkManager, etc).
2. `getty@tty1.service` hace autologin como `root` en tty1.
3. `.zlogin` ejecuta `/root/.automated_script.sh`.
4. `customize_airootfs.sh` se ejecuta (ver `docs/live-services.md`):
   - Crea el usuario `churros` (`users.sh`)
   - Habilita NetworkManager y greetd (`services.sh`)
   - Copia la configuración de `/etc/skel/` a `/home/churros/` (`desktop.sh`)
   - Limpia la cache de pacman (`cleanup.sh`)
5. greetd arranca, autologin como `churros`, carga `niri`.
6. Niri lee `config.kdl` y ejecuta los `spawn-at-startup` (noctalia, churros-welcome, …).
7. Noctalia pinta barra, notificaciones y fondo.

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

Reemplaza `archiso/airootfs/usr/share/churros/wallpapers/default.png` con tu imagen. Noctalia la carga desde `~/.config/noctalia/config.toml`.

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
- Perfiles de energía: cambiar decoraciones automáticamente según si el equipo está en batería. La infraestructura (PowerService + power-profile popup + battery subpágina) ya está lista.
- Integración con `wlogout`: script de salida que se muestra desde el popup de power.
- Traducciones reales (gettext): hoy `i18n._()` simplemente devuelve la string original (no carga PO files). Definir `po/churros.po` y compilar a `/usr/share/locale/es/LC_MESSAGES/churros.mo`.
