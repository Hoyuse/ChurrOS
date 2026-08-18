# Apps

Este documento describe las aplicaciones oficiales de ChurrOS.

Desde **v0.5 / v0.6** todas las apps oficiales están escritas en **Rust** con **gtk4-rs** y **libadwaita**. El código Python de `/usr/share/churros/` se eliminó del repositorio; en runtime solo quedan assets (CSS, SVG) y los binarios que despliega `scripts/build-rust.sh`.

```text
rust/
├── Cargo.toml              # workspace
├── churros-welcome/        # binario churros-welcome
├── preferences/            # binario churros-settings
├── control-center/         # binario churros-control-center
├── popups/                 # binario churros-popup
└── services/               # crate churros_services (no se despliega)
```

Cada crate con `deploy = true` en `Cargo.toml` se copia a `archiso/airootfs/usr/bin/<nombre>` al construir la ISO. Los binarios no se versionan.

En desarrollo, los crates resuelven assets a `rust/<crate>/assets/` si no existe `/usr/share/churros/<app>/`.

---

# churros-welcome

**Path:** `rust/churros-welcome/`
**Binario:** `/usr/bin/churros-welcome`
**Autostart:** `archiso/airootfs/etc/skel/.config/niri/config.kdl` (`spawn-at-startup "churros-welcome"`)
**Assets:** `archiso/airootfs/usr/share/churros/churros-welcome/assets/`

Pantalla de bienvenida al iniciar la sesión Live.

## Purpose

- Dar la bienvenida al usuario.
- Ofrecer accesos a instalación, GitHub y comunidad.

`system_card.rs` y `system_info.rs` existen (leen `/proc` y `/etc/os-release`) pero la `SystemCard` **no se monta** en la ventana actual. El footer muestra `Linux • Niri • ChurrOS` más la versión de `churros_services::version::distro()` (archivo `VERSION` del repo, embebido al compilar).

## Stack

- GTK 4 + Libadwaita (gtk4-rs / libadwaita-rs)
- Sin psutil: CPU, RAM, kernel y hostname salen de `/proc`

## Window

- Maximizada, sin decoración
- Tamaño mínimo: 640×480
- Layout vertical con scroll
- CSS: `/usr/share/churros/styles/churros.css` + `assets/style.css`

## Structure

```text
rust/churros-welcome/
├── Cargo.toml
├── assets/
└── src/
    ├── main.rs
    ├── header.rs
    ├── cards.rs            # FlowBox con 3 ActionCards
    ├── footer.rs
    ├── action_card.rs
    ├── system_card.rs      # Definida, no montada
    ├── system_info.rs
    ├── actions.rs          # URLs + calamares.desktop
    └── assets.rs
```

## Action Cards

| Icono | Título | Acción |
|-------|--------|--------|
| install.svg | Install ChurrOS | Lanza `calamares.desktop` |
| github.svg | GitHub | Abre el repositorio |
| community.svg | Comunidad | Abre el servidor de comunidad |

Máximo 3 columnas; en pantallas estrechas se apilan.

## Desktop Entry

`archiso/airootfs/usr/share/applications/churros-welcome.desktop` — `Exec=churros-welcome`.

---

# churros-control-center

**Path:** `rust/control-center/`
**Binario:** `/usr/bin/churros-control-center`
**Desktop entry:** `archiso/airootfs/usr/share/applications/churros-control-center.desktop`
**Atajo:** `Mod + C` (niri)

Centro de control con tarjetas que abren el popup correspondiente (`churros-popup <nombre>`).

## Window

- 430×650, no redimensionable, sin decoración
- Header: logo, título, botón de settings (`churros-settings`) y botón de power
- Grid 2×2 (red, bluetooth, brillo, batería) + tarjeta de audio a ancho completo
- Refresh asíncrono cada 2 s (`churros_services` en un hilo)

## Cards

| Posición | Tarjeta | Popup |
|----------|---------|-------|
| 0,0 | Network | `churros-popup network` |
| 0,1 | Bluetooth | `churros-popup bluetooth` |
| 1,0 | Brightness | `churros-popup brightness` |
| 1,1 | Battery | `churros-popup battery` |
| debajo | Audio | (controles in-place + popup de audio) |

## Services

Usa el crate `churros_services` (`rust/services/`): wifi, ethernet, bluetooth, brightness, battery, audio. Detalle en `docs/services.md`.

Logs de arranque: `/tmp/churros/churros-control-center.log`.

---

# churros-settings

**Path:** `rust/preferences/`
**Binario:** `/usr/bin/churros-settings`
**Atajo:** `Mod + P` (niri)

App de configuración principal, estilo System Settings con colores ChurrOS. Documentación dedicada en `docs/preferences.md`.

Logs: `/tmp/churros/churros-settings.log`.

---

# churros-popup

**Path:** `rust/popups/`
**Binario:** `/usr/bin/churros-popup`

Un solo binario con los seis popups y toggle nativo (pidfiles en `/tmp/churros/`). Documentación en `docs/popups.md`.

---

# fuzzel (launcher)

**Path:** paquete del sistema (`archiso/packages.x86_64`).
**Atajo:** `Mod + Space`
**Waybar:** `custom/launcher` → `fuzzel`

No es una app ChurrOS. Config: `archiso/airootfs/etc/skel/.config/fuzzel/fuzzel.ini`.

---

# churros-ui (planificado)

**Estado:** no implementado.

Hoy cada app tiene su propio CSS. A largo plazo convendría un crate o stylesheet compartido más allá de `/usr/share/churros/styles/churros.css`.

---

# Packaging

| Qué | Dónde |
|-----|--------|
| Código | `rust/<crate>/` |
| Binario en la ISO | `archiso/airootfs/usr/bin/<app>` (generado, no versionado) |
| Assets | `archiso/airootfs/usr/share/churros/<app>/` |
| Desktop entries | `archiso/airootfs/usr/share/applications/` |

---

# Development

1. Edita el crate en `rust/<app>/`.
2. Compila en el host:

```bash
cargo build --release --manifest-path rust/Cargo.toml
```

3. Para verlo en el Live:

```bash
./churros build
./churros run
```

`./churros check` no compila Rust; valida scripts, paquetes, desktop entries y que los `spawn` de Niri resuelvan a un binario, crate con `deploy = true` o paquete de la ISO.

---

# Future Work

- Empaquetar las apps como paquetes pacman propios.
- Completar i18n (los `.po` existen; las apps Rust aún no cargan gettext).
- churros-ui: widgets y CSS compartidos.
- Tests automatizados de las apps GTK.
