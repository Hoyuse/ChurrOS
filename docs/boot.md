# Boot

Este documento describe el sistema de arranque de la ISO Live de ChurrOS.

ChurrOS utiliza ArchISO, que genera automáticamente cargadores de arranque para BIOS y UEFI. Los tres cargadores soportados son **GRUB** (BIOS y UEFI), **systemd-boot** (UEFI) y **Syslinux** (BIOS legacy).

Todas las entradas de menú de los cargadores están renombradas de "Arch Linux" a "ChurrOS Live" para mantener la identidad de la distribución.

---

# Overview

El sistema de arranque se distribuye en tres carpetas bajo `archiso/`:

```text
archiso/
├── grub/
│   ├── grub.cfg
│   └── loopback.cfg
├── efiboot/
│   └── loader/
│       ├── loader.conf
│       └── entries/
│           ├── 01-archiso-linux.conf
│           ├── 02-archiso-speech-linux.conf
│           └── 03-archiso-memtest86+x64.conf
└── syslinux/
    ├── syslinux.cfg
    ├── archiso_head.cfg
    ├── archiso_sys.cfg
    ├── archiso_sys-linux.cfg
    ├── archiso_pxe.cfg
    ├── archiso_pxe-linux.cfg
    ├── archiso_tail.cfg
    └── splash.png
```

El modo de arranque se define en `archiso/profiledef.sh`:

```bash
bootmodes=('bios.syslinux' 'uefi.systemd-boot')
```

Eso significa que en BIOS se usa Syslinux y en UEFI se usa systemd-boot. El archivo `grub/grub.cfg` lo usa ArchISO para crear una ISO híbrida que también puede arrancar en BIOS/UEFI legacy.

---

# GRUB

**Archivos:** `archiso/grub/grub.cfg`, `archiso/grub/loopback.cfg`

El menú de GRUB ofrece las siguientes entradas:

| Entrada | Hotkey | Descripción |
|---------|--------|-------------|
| ChurrOS Live | — | Arranque normal |
| ChurrOS Live Accessibility | `s` | Arranque con `accessibility=on` (activa espeakup) |
| Run Memtest86+ (RAM test) | — | Test de memoria |
| UEFI Shell | — | Solo en UEFI |
| UEFI Firmware Settings | — | Solo en UEFI (vuelve a la BIOS) |
| System shutdown | — | Apaga el equipo desde GRUB |
| System restart | — | Reinicia el equipo desde GRUB |

La entrada principal está etiquetada como `id 'archlinux'` (mantiene el id interno de ArchISO para compatibilidad con scripts) pero el texto visible dice "ChurrOS Live".

El parámetro `archisobasedir=%INSTALL_DIR%` se sustituye en tiempo de compilación por `churros` (definido en `profiledef.sh` como `install_dir="churros"`).

La entrada de accesibilidad añade `accessibility=on` a la línea del kernel, lo que activa los servicios `livecd-alsa-unmuter.service` y `livecd-talk.service` descritos en `docs/live-services.md`.

GRUB también carga módulos para soportar distintos sistemas de archivos y modos de consola (serial, USB-serial).

---

# systemd-boot

**Archivos:** `archiso/efiboot/loader/loader.conf`, `archiso/efiboot/loader/entries/*.conf`

systemd-boot es el cargador usado en sistemas UEFI modernos. Es más rápido y sencillo que GRUB.

Configuración del cargador (`loader.conf`):

```text
timeout 15
default 01-archiso-linux.conf
beep on
```

Entradas:

| Archivo | Título | Descripción |
|---------|--------|-------------|
| `01-archiso-linux.conf` | ChurrOS Live (x86_64, UEFI) | Arranque principal |
| `02-archiso-speech-linux.conf` | (Accesibilidad) | Añade `accessibility=on` |
| `03-archiso-memtest86+x64.conf` | Memtest86+ | Test de memoria |

Todas las entradas usan `archisobasedir=%INSTALL_DIR%` y `archisosearchuuid=%ARCHISO_UUID%`, igual que GRUB.

---

# Syslinux

**Archivos:** `archiso/syslinux/syslinux.cfg` + varios `archiso_*.cfg`

Syslinux se usa como cargador de BIOS legacy. La estructura está dividida en varios archivos:

| Archivo | Función |
|---------|---------|
| `syslinux.cfg` | Punto de entrada; redirige según el modo de arranque |
| `archiso_head.cfg` | Cabecera común (UI, splash) |
| `archiso_sys.cfg` | Menú para arranque desde disco/ISO local |
| `archiso_sys-linux.cfg` | Entradas de kernel para arranque local |
| `archiso_pxe.cfg` | Menú para arranque por red (PXE) |
| `archiso_pxe-linux.cfg` | Entradas de kernel para arranque PXE |
| `archiso_tail.cfg` | Pie común (submenu, utilidades) |
| `splash.png` | Imagen de fondo del menú |

El punto de entrada decide entre arranque PXE o local mediante `whichsys.c32`:

```text
DEFAULT select

LABEL select
COM32 whichsys.c32
APPEND -pxe- pxe -sys- sys -iso- sys

LABEL pxe
CONFIG archiso_pxe.cfg

LABEL sys
CONFIG archiso_sys.cfg
```

Si el medio es arrancable por red (PXE), se carga `archiso_pxe.cfg`. En cualquier otro caso (disco o ISO local), se carga `archiso_sys.cfg`.

---

# Boot Process

El proceso de arranque es el siguiente:

1. La BIOS/UEFI carga el cargador correspondiente (GRUB, systemd-boot o Syslinux).
2. El cargador lee `archisobasedir` y `archisosearchuuid` para localizar la partición Live.
3. Carga el kernel (`vmlinuz-linux`) y el initramfs (`initramfs-linux.img`).
4. El initramfs monta el sistema squashfs raíz.
5. systemd arranca y los servicios live (ver `docs/live-services.md`) se inicializan.
6. Si `accessibility=on` está presente, se activan los servicios de accesibilidad.
7. SDDM arranca, autologin como `churros` y se carga Hyprland.

---

# Building

No necesitas editar manualmente los archivos de los cargadores para regenerar la ISO: ArchISO los usa tal cual desde `archiso/`. Si modificas `grub.cfg`, por ejemplo, simplemente ejecuta:

```bash
./churros build
```

La nueva ISO incluirá los cambios.

---

# Customization

## Cambiar el nombre del menú

Edita los archivos correspondientes:

- GRUB: `archiso/grub/grub.cfg` (cadenas dentro de `menuentry`)
- systemd-boot: `archiso/efiboot/loader/entries/*.conf` (campo `title`)
- Syslinux: `archiso/syslinux/archiso_sys-linux.cfg` (campo `LABEL` y `MENU LABEL`)

## Cambiar el splash

Para Syslinux/GRUB, reemplaza `archiso/syslinux/splash.png` con tu imagen (recomendado 640×480 o 800×600, formato PNG).

## Cambiar el timeout

- GRUB: `archiso/grub/grub.cfg` → `timeout=15`
- systemd-boot: `archiso/efiboot/loader/loader.conf` → `timeout 15`

---

# Troubleshooting

## La ISO no arranca en UEFI

Comprueba que tu firmware tenga activado el modo UEFI (no Legacy/CSM). Si el firmware solo soporta Legacy, la ISO arrancará con Syslinux.

## No aparece la entrada de accesibilidad

La entrada solo se activa si el paquete `espeakup` está en `archiso/packages.x86_64`. Ya está incluido por defecto.

## GRUB muestra "Arch Linux" en vez de "ChurrOS"

Verifica que `archiso/grub/grub.cfg` tenga las cadenas actualizadas. El id interno (`archlinux`) debe permanecer por compatibilidad, pero el texto visible debe decir "ChurrOS Live".

## Cambios en grub.cfg no aparecen

Recuerda que `archiso/airootfs/` es la fuente para `mkarchiso`. Si modificas archivos fuera de `archiso/`, no se incluirán.
