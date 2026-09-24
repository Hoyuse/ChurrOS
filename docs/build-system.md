# Build System

El sistema de compilación de ChurrOS está basado en **ArchISO** y automatizado mediante `./churros`.

El objetivo es generar imágenes ISO reproducibles, mantener un flujo sencillo y minimizar las tareas manuales.

---

# Arquitectura

```
               Código fuente
                     │
                     ▼
              ./churros build
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
    branding    paquetes AUR   apps Rust
        │            │            │
        └────────────┼────────────┘
                     ▼
              mkarchiso (sudo)
                     ▼
          out/ChurrOS-*.iso
```

---

# Componentes

- ArchISO (`mkarchiso`)
- CLI de ChurrOS (`scripts/cli/build.sh`)
- Perfil en `archiso/`
- Workspace Rust en `rust/`
- Paquetes locales en `archiso/packages/`
- Scripts de compilación auxiliares en `scripts/`:
  - `build-rust.sh`: Compila en release todos los crates de `rust/` con `deploy = true` y los instala en el airootfs.
  - `build-calamares.sh`: Compila el instalador Calamares desde AUR con parches locales y soporte de Python.
  - `build-aur.sh`: Compila paquetes AUR necesarios (`python-pywal`, `waypaper`, `yay`).
  - `build-bazaar.sh`: Compila la tienda de aplicaciones Bazaar resolviendo conflictos de dependencias con libdex.
  - `build-grub-theme.sh`: Genera fuentes `.pf2` y recursos gráficos para el tema de GRUB.
  - `build-i18n.sh`: Compila catálogos gettext de `po/*.po` a `.mo` en `archiso/airootfs/usr/share/locale/`.
  - `build-churros-release.sh`: Genera el bundle OTA `churros-utils-<version>.tar.zst` y `updates.json` para el servidor de actualizaciones.

---

# Flujo de compilación

`./churros build [--edition <niri|xfce>]` hace, en este orden:

## 0. Selección de edición y paquetes

- Si se especifica `--edition xfce`: selecciona `archiso/packages.xfce.x86_64`, configura `/etc/churros-edition` con `xfce` y ajusta el autologin de `greetd` a `startxfce4`.
- Si se especifica `--edition niri` (por defecto): utiliza `archiso/packages.x86_64` (Waybar, Niri, foot, Fuzzel, Mako) y el autologin a sesión Niri.

## 1. Branding y tema GRUB

Copia `branding/customize_airootfs.sh` y `branding/files/` al airootfs. Estampa `VERSION` y la edición activa en `os-release`. Regenera fuentes del tema GRUB si faltan y copia `branding/grub-theme` a `/usr/share/churros/grub-theme/`.

## 2. Paquetes locales

Si no están, construye Calamares y los extras AUR (`python-pywal`, `waypaper`, `yay`) en `archiso/packages/`. Si hay paquete de Calamares, `installer/apply-calamares.sh` despliega la config y se copian los `.pkg.tar.zst` a `airootfs/root/packages/`.

## 3. Apps Rust

`scripts/build-rust.sh` compila el workspace en release y copia los crates con `deploy = true` a `archiso/airootfs/usr/bin/`. Esos binarios no se versionan; un trap al salir del build los limpia del airootfs (junto con branding y Calamares generados).

## 4. ArchISO

```bash
sudo rm -rf work out
sudo mkarchiso -v -w work -o out archiso
```

ArchISO instala paquetes, genera initramfs y squashfs, crea los cargadores (GRUB UEFI + Syslinux BIOS) y escribe la ISO.

## 5. Resultado

La ISO queda en `out/`. Ejemplo:

```text
out/ChurrOS-2026.08.17-x86_64-v0.7.iso
```

Al terminar se borra `work/` y se devuelve `out/` al usuario.

---

# Probar la distribución

```bash
./churros run
```

Busca la ISO más reciente, crea el disco QEMU si hace falta e inicia la VM. Detalle en `docs/vm.md`.

---

# Limpiar el proyecto

```bash
./churros clean
```

Elimina `work/` y `out/`. No toca el código fuente.

---

# Directorios utilizados

| Ruta | Uso |
|------|-----|
| `archiso/` | Perfil Live |
| `rust/` | Código de las apps oficiales |
| `branding/` | Identidad y script live |
| `installer/` | Config de Calamares |
| `work/` | Temporal de ArchISO |
| `out/` | ISO generada |

---

# Branding

Durante la compilación se integran:

- hostname, issue, motd, os-release
- logos y fondos
- tema GRUB
- configuraciones del Live

Editar la copia generada en `archiso/airootfs/root/customize_airootfs.sh` no sirve: se regenera en cada build.

---

# Errores comunes

## La ISO no aparece

```bash
ls out/
./churros build
```

## Error de permisos

```bash
./churros clean
./churros build
```

`mkarchiso` necesita `sudo`.

## ArchISO no encontrado

```bash
pacman -Q archiso
```

## Falla la compilación Rust

```bash
pacman -Q rust cargo
cargo build --release --manifest-path rust/Cargo.toml
```

## La compilación falla

Revisa el registro de `mkarchiso`. Suele deberse a paquetes inexistentes, rutas incorrectas, permisos o config inválida.

---

# Buenas prácticas

```
Modificar archivos
        ↓
./churros check
        ↓
./churros build
        ↓
./churros run
        ↓
Corregir
        ↓
Commit + pull request
```

Nunca modificar la ISO generada. Todos los cambios van en el código fuente.

---

# Futuro

Mejoras previstas:

- Compilaciones incrementales.
- Generación de checksums desde la CLI.
- Comando `./churros release`.

`./churros check` y el workflow de GitHub Actions ya cubren la verificación estática. El release v1.2 se publica a mano en download.churroslinux.org (ISO + torrent).
