# Live Services

Este documento describe los servicios systemd y los hooks de pacman que configuran el entorno Live de ChurrOS durante el arranque.

Estos servicios se incluyen en la ISO mediante `archiso/airootfs/etc/systemd/system/` y `archiso/airootfs/etc/pacman.d/hooks/`. No se copian al sistema instalado: viven solo en el Live.

---

# Overview

```text
archiso/airootfs/etc/
├── systemd/
│   ├── journald.conf.d/
│   │   └── volatile-storage.conf
│   ├── logind.conf.d/
│   │   └── do-not-suspend.conf
│   ├── network/
│   │   ├── 20-ethernet.network
│   │   ├── 20-wlan.network
│   │   ├── 20-wwan.network
│   │   └── networkd.conf.d/
│   │       └── ipv6-privacy-extensions.conf
│   ├── resolved.conf.d/
│   │   └── archiso.conf
│   └── system/
│       ├── pacman-init.service
│       ├── etc-pacman.d-gnupg.mount
│       ├── choose-mirror.service
│       ├── livecd-alsa-unmuter.service
│       ├── livecd-talk.service
│       ├── getty@tty1.service.d/
│       │   └── autologin.conf
│       └── systemd-networkd-wait-online.service.d/
│           └── wait-for-only-one-interface.conf
└── pacman.d/
    └── hooks/
        ├── uncomment-mirrors.hook
        └── zzzz99-remove-custom-hooks-from-airootfs.hook
```

Adicionalmente, el script `customize_airootfs.sh` (en `branding/`, copiado a `airootfs/root/` durante el build) orquesta la creación del usuario, habilitación de servicios y configuración del escritorio.

---

# Customization Script

**Path:** `archiso/airootfs/root/customize_airootfs.sh` (copiado desde `branding/customize_airootfs.sh`)

Este script es ejecutado por ArchISO al final del build, dentro del chroot. No se ejecuta en cada arranque.

```bash
cp /root/branding/files/os-release /etc/os-release
cp /root/branding/files/issue /etc/issue
cp /root/branding/files/motd /etc/motd

chmod 644 /etc/os-release
chmod 644 /etc/issue
chmod 644 /etc/motd

bash /root/scripts/users.sh
bash /root/scripts/services.sh
bash /root/scripts/desktop.sh
bash /root/scripts/cleanup.sh
```

Pasos:

1. Aplica branding (issue, motd, os-release).
2. Crea el usuario `churros` (`users.sh`).
3. Habilita servicios (`services.sh`).
4. Configura el escritorio (`desktop.sh`).
5. Limpia la cache de pacman (`cleanup.sh`).

Ver `docs/desktop-config.md` para más detalle sobre el usuario y la configuración del escritorio.

---

# Services

## pacman-init.service

**Path:** `archiso/airootfs/etc/systemd/system/pacman-init.service`

Inicializa el keyring de pacman al arrancar el Live.

```ini
[Unit]
Description=Initializes Pacman keyring
Requires=etc-pacman.d-gnupg.mount
After=etc-pacman.d-gnupg.mount time-sync.target
BindsTo=etc-pacman.d-gnupg.mount
Before=archlinux-keyring-wkd-sync.service

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/bin/pacman-key --init
ExecStart=/usr/bin/pacman-key --populate

[Install]
WantedBy=multi-user.target
```

- `pacman-key --init` genera las claves locales.
- `pacman-key --populate` importa las claves oficiales de Arch Linux.
- Necesario para que `pacman -Sy` funcione dentro del Live (por ejemplo, antes de instalar con `archinstall`).

## etc-pacman.d-gnupg.mount

**Path:** `archiso/airootfs/etc/systemd/system/etc-pacman.d-gnupg.mount`

```ini
[Unit]
Description=Temporary /etc/pacman.d/gnupg directory

[Mount]
What=tmpfs
Where=/etc/pacman.d/gnupg
Type=tmpfs
Options=mode=0755,noswap
```

Monta `/etc/pacman.d/gnupg` como tmpfs. Esto evita que se generen claves persistentes dentro de la ISO: cada arranque genera claves nuevas, y al apagar se pierde todo (el sistema corre en RAM).

`pacman-init.service` requiere este mount y se bindea a él.

## choose-mirror.service

**Path:** `archiso/airootfs/etc/systemd/system/choose-mirror.service`

Permite seleccionar un mirror de pacman desde la línea de comandos del kernel (al arrancar la ISO).

```ini
[Unit]
Description=Choose mirror from the kernel command line
ConditionKernelCommandLine=mirror

[Service]
Type=oneshot
ExecStart=/usr/local/bin/choose-mirror

[Install]
WantedBy=multi-user.target
```

Solo se activa si el kernel cmdline incluye `mirror=...`. El script `choose-mirror` lee el parámetro y regenera `/etc/pacman.d/mirrorlist`.

Uso típico:

```bash
# Al arrancar la ISO desde GRUB, edita la entrada y añade:
mirror=https://mirror.example.com/archlinux
```

Útil para entornos de prueba, redes restringidas o mirrors corporativos.

## livecd-alsa-unmuter.service

**Path:** `archiso/airootfs/etc/systemd/system/livecd-alsa-unmuter.service`

Desilencia las tarjetas de sonido al iniciar, solo si `accessibility=on`.

```ini
[Unit]
Description=Unmute All Sound Card Controls For Use With The Live Arch Environment
Wants=systemd-udev-settle.service
After=systemd-udev-settle.service sound.target
ConditionKernelCommandLine=accessibility=on

[Service]
Type=oneshot
ExecStart=/usr/local/bin/livecd-sound -u

[Install]
WantedBy=sound.target
```

`livecd-sound -u` itera sobre todas las tarjetas de sonido y desilencia los controles. Sin esto, los lectores de pantalla (espeakup) no tendrían audio.

## livecd-talk.service

**Path:** `archiso/airootfs/etc/systemd/system/livecd-talk.service`

Activa el lector de pantalla espeakup, solo si `accessibility=on`.

```ini
[Unit]
Description=Screen reader service
After=livecd-alsa-unmuter.service
Before=getty@tty1.service
ConditionKernelCommandLine=accessibility=on

[Service]
Type=oneshot
TTYPath=/dev/tty13
ExecStartPre=/usr/bin/chvt 13
ExecStart=/usr/local/bin/livecd-sound -p
ExecStartPost=/usr/bin/chvt 1
ExecStartPost=systemctl start espeakup.service
StandardInput=tty
TTYVHangup=yes
TTYVTDisallocate=yes
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
```

Pasos:

1. Cambia a tty13 (consola virtual auxiliar).
2. `livecd-sound -p` permite al usuario elegir tarjeta de sonido con feedback auditivo.
3. Vuelve a tty1.
4. Inicia `espeakup.service`.

## getty@tty1.service.d/autologin.conf

**Path:** `archiso/airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf`

```ini
[Service]
ExecStart=
ExecStart=-/usr/bin/agetty --noreset --noclear --autologin root - ${TERM}
```

Hace que el login en tty1 sea automático como `root`. La línea vacía `ExecStart=` es necesaria para sobrescribir el valor por defecto de Arch.

Al loguearse, root ejecuta `/root/.zlogin` (que a su vez ejecuta `/root/.automated_script.sh`).

---

# Systemd Drop-ins

## do-not-suspend.conf

**Path:** `archiso/airootfs/etc/systemd/logind.conf.d/do-not-suspend.conf`

Impide que el sistema se suspenda automáticamente en el Live. El usuario debe apagar manualmente cuando termine.

## volatile-storage.conf

**Path:** `archiso/airootfs/etc/systemd/journald.conf.d/volatile-storage.conf`

Configura journald para almacenar logs en RAM (tmpfs). Como el sistema es Live, no tiene sentido escribir logs al disco.

## 20-*.network

**Path:** `archiso/airootfs/etc/systemd/network/`

Configuración DHCP automática para Ethernet, Wi-Fi y WWAN. Estos archivos usan systemd-networkd, no NetworkManager. NetworkManager toma el relevo cuando arranca (ver `services.sh`).

## wait-for-only-one-interface.conf

**Path:** `archiso/airootfs/etc/systemd/system/systemd-networkd-wait-online.service.d/wait-for-only-one-interface.conf`

Reduce el timeout de "esperar a que la red esté online" a la primera interfaz que se levante. Evita esperas largas si hay varios adaptadores.

---

# Pacman Hooks

## uncomment-mirrors.hook

**Path:** `archiso/airootfs/etc/pacman.d/hooks/uncomment-mirrors.hook`

```ini
[Trigger]
Operation = Install
Operation = Upgrade
Type = Package
Target = pacman-mirrorlist

[Action]
Description = Uncommenting HTTPS mirrors in /etc/pacman.d/mirrorlist...
When = PostTransaction
Depends = pacman-mirrorlist
Depends = sed
Exec = /usr/bin/sed -E -i 's/#(Server = https:)/\1/g' /etc/pacman.d/mirrorlist
```

Tras instalar o actualizar `pacman-mirrorlist`, descomenta automáticamente las líneas `Server = https://...`. Sin esto, el mirrorlist queda con todos los servidores comentados y pacman no puede descargar nada.

## zzzz99-remove-custom-hooks-from-airootfs.hook

**Path:** `archiso/airootfs/etc/pacman.d/hooks/zzzz99-remove-custom-hooks-from-airootfs.hook`

```ini
[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = *

[Action]
Description = Work around FS#49347 by removing custom pacman hooks that are only required during ISO build...
When = PostTransaction
Depends = sh
Depends = coreutils
Depends = grep
Exec = /bin/sh -c "rm -- $(grep -Frl 'remove from airootfs' /etc/pacman.d/hooks/)"
```

Workaround para el bug FS#49347: los hooks que contienen el comentario `# remove from airootfs!` se eliminan automáticamente al instalar o actualizar cualquier paquete. Esto evita que los hooks específicos del build se ejecuten en el Live cuando el usuario instale software.

El prefijo `zzzz99` en el nombre asegura que este hook se ejecute al final, después de los demás.

---

# Scripts

## livecd-sound

**Path:** `archiso/airootfs/usr/local/bin/livecd-sound`

Script de gestión de audio. Opciones:

- `-u` / `--unmute` — desilencia todas las tarjetas
- `-p` / `--pick` — permite elegir tarjeta con feedback auditivo
- `-h` / `--help` — ayuda

Usa `amixer` para controlar ALSA y genera `/etc/asound.conf` a partir de `/usr/local/share/livecd-sound/asound.conf.in`.

## choose-mirror

**Path:** `archiso/airootfs/usr/local/bin/choose-mirror`

Lee `mirror=` del kernel cmdline y regenera `/etc/pacman.d/mirrorlist`. El archivo original se guarda como `mirrorlist.orig`.

## Installation_guide

**Path:** `archiso/airootfs/usr/local/bin/Installation_guide`

```sh
exec xdg-open 'https://wiki.archlinux.org/title/Installation_guide'
```

Atajo para abrir la guía de instalación de Arch en el navegador. Disponible en el PATH para que se pueda invocar desde la terminal o desde menús.

---

# Init Order

Resumen del orden de arranque del Live:

1. systemd monta `/etc/pacman.d/gnupg` (tmpfs).
2. `pacman-init.service` inicializa el keyring.
3. `choose-mirror.service` (si `mirror=` está en cmdline) regenera el mirrorlist.
4. NetworkManager arranca.
5. `livecd-alsa-unmuter.service` (si `accessibility=on`) desilencia audio.
6. `livecd-talk.service` (si `accessibility=on`) activa espeakup.
7. `getty@tty1` hace autologin como root.
8. `.zlogin` ejecuta `.automated_script.sh`.
9. SDDM arranca.
10. Autologin como `churros`, sesión Niri.
11. Niri carga autostart (waybar, swaybg, churros-welcome).

---

# Future Work

- Mover la lógica de `services.sh` (que habilita NetworkManager y SDDM) a unidades nativas de systemd.
- Eliminar la dependencia de root autologin: usar `systemd-user-sessions` o un PAM module.
- Documentar el orden de dependencias entre servicios (hoy está implícito en los `After=` y `Before=`).
- Internacionalizar los mensajes de los hooks.
