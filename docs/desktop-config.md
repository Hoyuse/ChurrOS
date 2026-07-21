# Desktop Config

Este documento describe la configuración del escritorio Live de ChurrOS: Hyprland, Waybar, SDDM y el usuario live.

La configuración se aplica a todo usuario nuevo del sistema gracias a que vive en `/etc/skel/.config/`, que el script `desktop.sh` (ver `docs/live-services.md`) copia a `/home/churros/` durante la inicialización del Live.

---

# Hyprland

**Path:** `archiso/airootfs/etc/skel/.config/hypr/`

Hyprland es el compositor Wayland usado por ChurrOS. La configuración está dividida en varios archivos modulares que se incluyen desde `hyprland.conf`.

## Structure

```text
hypr/
├── hyprland.conf         # Entry point
├── monitors.conf         # Configuración de monitores
├── environment.conf      # Variables de entorno
├── input.conf            # Teclado, ratón, touchpad
├── decorations.conf      # Gaps, bordes, redondeo
├── animations.conf       # Animaciones
├── keybinds.conf         # Atajos de teclado
└── autostart.conf        # Programas al iniciar
```

## Entry Point

`hyprland.conf` solo incluye los demás archivos con `source =`:

```text
source = ~/.config/hypr/monitors.conf
source = ~/.config/hypr/environment.conf
source = ~/.config/hypr/input.conf
source = ~/.config/hypr/decorations.conf
source = ~/.config/hypr/animations.conf
source = ~/.config/hypr/keybinds.conf
source = ~/.config/hypr/autostart.conf
```

## Monitors

`monitors.conf`:

```text
monitor = ,preferred,auto,1
```

Resolución preferida, tasa de refresco automática, escala 1. Se puede sobreescribir por monitor añadiendo su nombre antes de la coma.

## Environment

`environment.conf`:

```text
env = XCURSOR_SIZE,24
env = QT_QPA_PLATFORM,wayland
env = GDK_BACKEND,wayland
env = MOZ_ENABLE_WAYLAND,1
```

Fuerza a las apps Qt y GTK a usar Wayland en lugar de XWayland cuando es posible. Firefox usará Wayland nativo gracias a `MOZ_ENABLE_WAYLAND`.

## Input

`input.conf`:

```text
input {
    kb_layout = us
    follow_mouse = 1
    touchpad {
        natural_scroll = false
    }
    sensitivity = 0
}
```

Layout de teclado US. Scroll natural desactivado en touchpad. Sigue al ratón con el foco.

## Decorations

`decorations.conf`:

```text
general {
    gaps_in = 5
    gaps_out = 10
    border_size = 2
    resize_on_border = true
}

decoration {
    rounding = 8
}
```

- Gaps internos (entre ventanas) de 5px
- Gaps externos (al borde de la pantalla) de 10px
- Borde de 2px
- Redimensionar arrastrando el borde habilitado
- Esquinas redondeadas a 8px

## Animations

`animations.conf`:

```text
animations {
    enabled = yes
}
```

Animaciones activadas. Los valores por defecto de Hyprland son suficientes.

## Keybinds

`keybinds.conf`:

| Atajo | Acción |
|-------|--------|
| `SUPER + Return` | Abre Kitty (terminal) |
| `SUPER + Q` | Cierra la ventana activa |
| `SUPER + M` | Sale de Hyprland |
| `SUPER + F` | Pantalla completa |
| `SUPER + SPACE` | Abre el launcher (churros-launcher) |
| `SUPER + C` | Abre el centro de control |
| `SUPER + 1-0` | Cambia al workspace N (1-10) |
| `SUPER + SHIFT + 1-0` | Mueve la ventana activa al workspace N |
| `SUPER + ←/→/↑/↓` | Mueve el foco entre ventanas |
| `SUPER + SHIFT + ←/→/↑/↓` | Mueve la ventana activa |

`$mainMod` está definido como `SUPER` (la tecla Windows) al principio del archivo.

## Autostart

`autostart.conf`:

```text
# Wallpaper daemon
exec-once = awww-daemon

# Panel
exec-once = /usr/bin/waybar --config /home/churros/.config/waybar/config.jsonc

# Welcome
exec-once = churros-welcome
```

`awww-daemon` carga y anima el wallpaper. `waybar` arranca la barra superior. `churros-welcome` muestra la pantalla de bienvenida.

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
| Izquierda | `image#logo`, `hyprland/workspaces` |
| Centro | `hyprland/window` |
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
Session=hyprland

[General]
InputMethod=
```

SDDM arranca, autologin con el usuario `churros`, y carga la sesión `hyprland`. Esto permite que el Live entre directamente al escritorio sin pedir credenciales.

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
5. SDDM arranca, autologin como `churros`, carga `hyprland`.
6. Hyprland lee su config y carga los `exec-once` (waybar, awww, churros-welcome).
7. Waybar arranca y carga los popups de los módulos.

---

# Customization

## Cambiar el layout de teclado

Edita `archiso/airootfs/etc/skel/.config/hypr/input.conf`:

```text
input {
    kb_layout = es    # o "us,es" para alternar
}
```

## Cambiar el wallpaper

Reemplaza `archiso/airootfs/usr/share/churros/wallpapers/default.png` con tu imagen. `awww-daemon` la carga automáticamente al inicio.

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

- Soporte multi-monitor: `monitors.conf` está pensado para un solo display.
- Perfiles de energía: cambiar `animations.conf` y decoraciones automáticamente según si el equipo está en batería.
- Configuración por usuario: hoy `/etc/skel/` aplica a todos. En el sistema instalado se deberá copiar a `~/.config/hypr/` para permitir personalizaciones.
- Integración con `wlogout`: el script de salida que se muestra desde el popup de power.
- Soporte para otros WMs: XMonad, Sway, etc. Como alternativa a Hyprland.
