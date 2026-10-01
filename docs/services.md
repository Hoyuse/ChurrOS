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
├── dev.rs          # Aislamiento dry-run para ./churros apps
├── ethernet.rs     # nmcli
├── jsonc.rs        # Parser y normalizador JSONC (comentarios y trailing commas)
├── power.rs        # loginctl, niri msg, systemctl
├── version.rs      # Versión embebida en tiempo de compilación (VERSION)
├── waybar_style.rs # Manipulador de estilos y selectores CSS para Waybar
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

Si no hay batería, `available` es `false`. Iconos de `Symbols Nerd Font Mono` según porcentaje y carga.

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

## dev

Módulo de seguridad para desarrollo (`CHURROS_DEV=1`). Permite ejecutar `./churros apps` sobre la máquina del desarrollador en modo vista previa sin riesgo de alterar el sistema host.

- **Comportamiento:** Las operaciones de sólo lectura (obtener volumen, listar redes, consultar batería) se ejecutan normalmente.
- **Aislamiento:** Cualquier comando que altere el estado (`wpctl set-volume`, `nmcli con up/down`, `bluetoothctl connect`, `systemctl reboot/poweroff`, `pkill`) es interceptado, cancelado y registrado en stderr como `[churros-dev] blocked: <comando>`.

---

## jsonc

Parser y normalizador minimalista de JSON con comentarios (`//`) y trailing commas (`,}` o `,]`).

- `strip_line_comments(raw)`: Elimina comentarios de línea preservando URLs y cadenas con barras (`//`).
- `remove_trailing_commas(text)`: Limpia comas sobrantes para permitir parseo estricto con `serde_json`.
- `parse(raw)`: Combina ambas funciones para convertir texto JSONC a `serde_json::Value`.

Utilizado principalmente para la manipulación de configuraciones como `config.jsonc` de Waybar y archivos de preferencias de ChurrOS.

---

## version

Manejo y lectura de la versión de la distribución.

- `distro() -> &'static str`: Retorna el número de versión (`VERSION`) embebido en tiempo de compilación mediante `include_str!("../../../VERSION")`.
- `from_os_release() -> String`: Intenta leer `VERSION_ID` desde `/etc/os-release` en runtime con fallback a `distro()`.

Garantiza que apps como `churros-welcome`, `churros-settings` y los servicios del sistema reporten siempre la versión oficial sincronizada.

---

## waybar_style

Manipulación de CSS y normalización de sintaxis para temas de Waybar.

- `css_color(hex)`: Normaliza colores `#rrggbbaa` a formato de 6 dígitos `#rrggbb` compatible con los analizadores CSS de GTK en Waybar.
- `sanitize_selectors(css)`: Corrige automáticamente selectores mal formados (ej. convierte `#custom/sep` a `#custom-sep`).
- `parse_define_colors(css)`: Extrae definiciones `@define-color nombre valor;` a estructuras JSON para inspección y edición interactiva desde `churros-settings`.
- `patch_color(css, name, value)`: Modifica o inyecta variables de color en stylesheets sin destruir comentarios ni reglas existentes.

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
