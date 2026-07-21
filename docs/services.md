# Services

Este documento describe los servicios del sistema que utilizan las aplicaciones oficiales de ChurrOS.

Los servicios son módulos Python que envuelven comandos del sistema y exponen una API uniforme a las apps GTK4 (control center, popups, widgets). Toda la interacción con el hardware y los servicios del sistema (audio, batería, red, brillo, energía) pasa por aquí.

---

# Overview

Todos los servicios viven en:

```text
archiso/airootfs/usr/share/churros/services/
```

Cada servicio es una clase con métodos estáticos. El patrón común es:

- `get()` — devuelve un diccionario con el estado actual
- `set(value)` — aplica un cambio
- Métodos auxiliares (`enable`, `disable`, `toggle`, `lock`, `logout`, etc.) según el servicio

Ningún servicio tiene estado interno: cada llamada lee del sistema en tiempo real. Esto simplifica el código y permite que múltiples widgets o popups consulten el mismo dato sin coordinarse.

Los servicios no se importan directamente desde las apps: se accede a ellos desde los widgets de cada app. Por ejemplo, el popup de audio importa `services/audio.py` desde `archiso/airootfs/usr/share/churros/services/`.

---

# Common Pattern

```python
import subprocess


class AudioService:

    @staticmethod
    def get_volume():

        output = subprocess.check_output(
            ["wpctl", "get-volume", "@DEFAULT_AUDIO_SINK@"],
            text=True
        )

        volume = float(output.split()[1])

        return int(volume * 100)

    @staticmethod
    def set_volume(value):

        subprocess.run(
            [
                "wpctl",
                "set-volume",
                "@DEFAULT_AUDIO_SINK@",
                f"{value}%"
            ]
        )
```

Todas las clases siguen esta forma. La salida de `get()` siempre es un dict plano con campos predecibles; `set()` siempre es síncrono y no devuelve nada.

---

# Services

## audio

**Archivo:** `archiso/airootfs/usr/share/churros/services/audio.py`

Wrapper sobre `wpctl` (PipeWire).

| Método | Comando | Devuelve |
|--------|---------|----------|
| `get_volume()` | `wpctl get-volume @DEFAULT_AUDIO_SINK@` | int (0–100) |
| `set_volume(value)` | `wpctl set-volume @DEFAULT_AUDIO_SINK@ <value>%` | — |

No usa `sinks` por nombre: siempre opera sobre `@DEFAULT_AUDIO_SINK@`, que PipeWire resuelve en tiempo de ejecución. Esto evita problemas cuando el usuario cambia de dispositivo de salida.

Usado por:

- Popup de audio (volumen + mute + dispositivo)
- Waybar (módulo `pulseaudio` con scroll y click derecho)
- Control center (tarjeta de audio)

---

## battery

**Archivo:** `archiso/airootfs/usr/share/churros/services/battery.py`

Wrapper sobre `upower`.

| Método | Comando | Devuelve |
|--------|---------|----------|
| `get()` | `upower -e` + `upower -i <device>` | dict |

El método `get()` devuelve un dict con esta forma:

```python
{
    "available": bool,
    "percentage": int,
    "state": str,         # charging, discharging, full, unknown
    "time": str,          # "1:23" o "1:23:45" según upower
    "icon": str           # glifo Nerd Font (󰁹 󰂂 󰂀 󰁾 󰁼 󰂎)
}
```

Los iconos se eligen según el porcentaje:

| Porcentaje | Icono |
|------------|-------|
| ≥ 95% | 󰁹 |
| ≥ 80% | 󰂂 |
| ≥ 60% | 󰂀 |
| ≥ 40% | 󰁾 |
| ≥ 20% | 󰁼 |
| < 20% | 󰂎 |

Si no se detecta ninguna batería, devuelve `{"available": False}`. Los widgets deben comprobar esta clave antes de leer el resto.

Usado por:

- Popup de batería
- Waybar (módulo `battery`)
- Control center (tarjeta de batería)

---

## wifi

**Archivo:** `archiso/airootfs/usr/share/churros/services/wifi.py`

Wrapper sobre `nmcli` (NetworkManager).

| Método | Comando | Devuelve |
|--------|---------|----------|
| `get()` | `nmcli device` + `nmcli device wifi list` | dict |
| `enable()` | `nmcli radio wifi on` | — |
| `disable()` | `nmcli radio wifi off` | — |
| `toggle()` | según estado actual | — |

El método `get()` devuelve:

```python
{
    "available": bool,    # ¿hay adaptador Wi-Fi?
    "enabled": bool,      # ¿está encendido?
    "connected": str,     # SSID activo, "" si ninguno
    "networks": [
        {
            "ssid": str,
            "signal": int,    # 0–100
            "connected": bool
        },
        ...
    ]
}
```

Si no hay adaptador, devuelve solo `{"available": False, ...}` y la lista `networks` queda vacía.

Usado por:

- Popup de red
- Waybar (módulo `network` cuando el dispositivo es Wi-Fi)
- Control center (tarjeta de red)

---

## ethernet

**Archivo:** `archiso/airootfs/usr/share/churros/services/ethernet.py`

Wrapper sobre `nmcli` para interfaces cableadas.

| Método | Comando | Devuelve |
|--------|---------|----------|
| `get()` | `nmcli device` | dict |

El método `get()` devuelve:

```python
{
    "available": bool,      # ¿hay adaptador Ethernet?
    "connected": bool,
    "interface": str,       # nombre del dispositivo (eth0, enp3s0, ...)
    "connection": str       # nombre del perfil de conexión
}
```

Acepta tanto el estado `connected` como `conectado` (NetworkManager i18n).

Usado por:

- Popup de red (sección Ethernet)
- Control center (tarjeta de red)

---

## brightness

**Archivo:** `archiso/airootfs/usr/share/churros/services/brightness.py`

Wrapper sobre `brightnessctl` y `/sys/class/backlight`.

| Método | Comando | Devuelve |
|--------|---------|----------|
| `available()` | (lectura de `/sys/class/backlight`) | bool |
| `get()` | `brightnessctl g` + `brightnessctl m` | dict |
| `set(value)` | `brightnessctl set <value>%` | — |

El método `get()` devuelve:

```python
{
    "available": bool,
    "brightness": int   # 0–100
}
```

Si no hay soporte de brillo por software (por ejemplo, algunas GPUs externas), `available()` devuelve `False` y `set()` se vuelve no-op. El widget de brillo desactiva su slider en ese caso y muestra un mensaje informativo.

Usado por:

- Popup de brillo
- Waybar (módulo `custom/brightness`)

---

## power

**Archivo:** `archiso/airootfs/usr/share/churros/services/power.py`

Wrapper sobre `loginctl`, `hyprctl` y `systemctl`.

| Método | Comando | Acción |
|--------|---------|--------|
| `lock()` | `loginctl lock-session` | bloquea la sesión |
| `logout()` | `hyprctl dispatch exit` | sale de Hyprland |
| `suspend()` | `systemctl suspend` | suspende |
| `hibernate()` | `systemctl hibernate` | hiberna |
| `restart()` | `systemctl reboot` | reinicia |
| `shutdown()` | `systemctl poweroff` | apaga |

No hay método `get()`: las acciones de energía son one-shot y no devuelven estado. Cada acción del popup invoca directamente el método correspondiente.

Usado por:

- Popup de power
- Waybar (módulo `custom/power`)

---

# Best Practices

- No guardar estado en los servicios: cada llamada es independiente y refleja la realidad del sistema.
- Devolver siempre dicts desde `get()` para que los widgets puedan usar `data["key"]` sin manejar tuplas o clases.
- Comprobar `available` antes de leer otros campos: muchos servicios exponen esta clave para entornos donde el hardware no está presente (por ejemplo, batería en un escritorio).
- Usar `subprocess.check_output` con `text=True` para evitar decode manual; envolver en `try/except` cuando la llamada pueda fallar (red, hardware ausente).
- Mantener el servicio delgado: la lógica de presentación (formatear iconos, calcular porcentajes) pertenece al widget, no al servicio.

---

# Future Work

- Servicio de Bluetooth: hoy el popup de Bluetooth usa una lista hardcodeada. Hace falta un wrapper sobre `bluetoothctl` o `bluez` DBus para enumerar dispositivos emparejados.
- Servicio de audio por dispositivo: hoy `wpctl @DEFAULT_AUDIO_SINK@` siempre opera sobre el sink activo. Para un control center avanzado habría que listar sinks y permitir elegir.
- Notificaciones: integrar `dunst` o `mako` como servicio y exponer API para que las apps manden notificaciones.
- Tema dinámico: un servicio que observe el estado del sistema (batería baja, red perdida) y dispare notificaciones automáticamente.
