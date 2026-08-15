# Services

Este documento describe la capa de servicios que usan las apps oficiales.

`churros_services` es un crate Rust (`rust/services/`, `deploy = false`) que envuelve comandos del sistema y expone una API uniforme. Lo consumen `churros-popup` y `churros-control-center`. Preferencias tiene además sus propios servicios de dotfiles y settings en `rust/preferences/src/services/`.

El código Python de `usr/share/churros/services/` ya no está en el repositorio.

---

# Overview

```text
rust/services/src/
├── lib.rs          # run / spawn / which
├── audio.rs        # wpctl
├── battery.rs      # upower
├── bluetooth.rs    # bluetoothctl / rfkill
├── brightness.rs   # brightnessctl + /sys/class/backlight
├── ethernet.rs     # nmcli
├── power.rs        # loginctl, niri msg, systemctl
└── wifi.rs         # nmcli
```

Patrón:

- Funciones libres (no hay clases estáticas).
- `get()` / `available()` leen el sistema en el momento.
- `run(cmd, timeout_ms)` captura stdout/stderr; `spawn` es fire-and-forget.
- Sin estado interno: cada llamada refleja el hardware actual.

---

# Common helpers

```rust
pub type RunOut = (i32, String, String);

pub fn run(cmd: &[&str], timeout_ms: u64) -> Option<RunOut>;
pub fn spawn(cmd: &[&str]);
pub fn which(bin: &str) -> bool;
```

`run` devuelve `None` si falla el spawn, hay timeout o la salida no es UTF-8.

---

# Services

## audio

Wrapper sobre `wpctl` (PipeWire). Opera sobre `@DEFAULT_AUDIO_SINK@` / `@DEFAULT_AUDIO_SOURCE@`.

| Función | Acción |
|---------|--------|
| `get_volume()` / `set_volume(value)` | Volumen de salida 0–100 |
| `is_muted()` / `set_mute(muted)` | Mute de salida |
| `get_input_volume()` / `set_input_volume` | Entrada |
| `list_sinks()` / `list_sources()` | Dispositivos (`AudioDevice { id, name, default }`) |
| `set_default_sink(node_id)` | Cambiar sink |

Usado por el popup de audio, Waybar (`pulseaudio`) y el control center.

---

## battery

Wrapper sobre `upower`.

`get()` → `BatteryInfo`:

```text
available, percentage, state, time_to_full, time_to_empty, icon
```

Si no hay batería, `available` es `false`. Iconos Nerd Font según porcentaje y carga.

---

## wifi

Wrapper sobre `nmcli`.

`get()` → `WifiInfo`: `available`, `enabled`, `connected` (SSID o `None`), `networks` (`ssid`, `signal`, `security`, `connected`, `saved`).

También: `enable` / `disable` / `toggle`, `connect`, `connect_hidden`, `disconnect`, `forget`, `scan`.

---

## ethernet

`get()` → `EthernetInfo`: `available`, `connected`, `interface`, `connection`.

También: `speed(device)`, `ip(device)`, `connect`, `disconnect`.

---

## brightness

`available()` mira `/sys/class/backlight`. `get()` → `{ available, brightness }` (0–100). `set(value)` usa `brightnessctl`.

Si no hay backlight (GPU externa, escritorio), `available` es `false` y el slider se desactiva.

---

## bluetooth

Wrapper real sobre `bluetoothctl` (ya no es una lista hardcodeada).

| Función | Acción |
|---------|--------|
| `available()` / `is_enabled()` / `is_blocked()` | Estado del adaptador |
| `enable()` / `disable()` | Power |
| `scan_start()` / `scan_stop()` | Escaneo |
| `list_devices()` | `BtDevice { address, name, connected }` |
| `connect` / `disconnect` / `pair` / `remove` | Por dirección |

---

## power

| Función | Comando |
|---------|---------|
| `lock()` | `loginctl lock-session` |
| `logout()` | `niri msg action quit` (si el desktop es Hyprland, `hyprctl dispatch exit`) |
| `suspend()` | `systemctl suspend` |
| `hibernate()` | `systemctl hibernate` (`can_hibernate()` primero) |
| `restart()` | `systemctl reboot` |
| `shutdown()` | `systemctl poweroff` |

---

# Preferencias

Los servicios de `churros-settings` (tema, acento, niri, mako, wallpaper, …) no están en este crate: viven en `rust/preferences/src/services/`. Ver `docs/preferences.md`.

---

# Best Practices

- No guardar estado: cada llamada lee el sistema.
- Comprobar `available` antes de pintar widgets.
- Timeouts cortos (`run`) para no bloquear el hilo de UI; el control center ya refresca en un hilo aparte.
- Presentación (iconos, porcentajes) en el widget, no en el servicio, salvo iconos que el servicio ya calcula (batería).

---

# Future Work

- Audio: elegir sink desde el control center con la lista que ya expone `list_sinks`.
- Notificaciones: API sobre mako para que las apps manden avisos.
- Tema dinámico ante batería baja o red perdida.
