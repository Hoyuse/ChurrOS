# Privileged Execution

Este documento describe cómo obtienen privilegios los procesos de ChurrOS y
qué reglas se aplican para que esa escalada sea explícita y auditable.

La regla que resume todo lo demás: **se autoriza por ruta absoluta exacta, nunca
por nombre de comando ni por comodín.**

---

# Por qué importa

`pkexec` resuelve rutas relativas (`./x`, `../x`) y busca en `$PATH` antes de
elevar. Una regla polkit que autorice por *basename* concede root sin contraseña
a cualquier fichero que el usuario pueda crear:

```bash
printf '#!/bin/sh\nid -u\n' > /tmp/churros-pwn && chmod +x /tmp/churros-pwn
pkexec /tmp/churros-pwn        # uid=0, sin contraseña
```

Por eso las reglas de ChurrOS comparan la ruta absoluta contra una lista
cerrada, todas bajo `/usr` o `/usr/local` (propiedad de root, no escribibles por
el usuario).

---

# Piezas

| Pieza | Ruta | Función |
|-------|------|---------|
| Regla polkit | `/etc/polkit-1/rules.d/50-churros-store.rules` | Autoriza a `wheel` un allowlist de binarios con validación de argumentos |
| Regla de Calamares | `/etc/polkit-1/rules.d/49-calamares.rules` | Solo durante la instalación: permite al usuario Live lanzar Calamares |
| Wrapper | `/usr/bin/churros-pkexec` | Intenta `pkexec` y, si no puede elevar (126/127), `sudo -n` |
| Helper de config | `/usr/local/bin/churros-write-root-config` | Escribe un `/etc` concreto desde stdin, con lista cerrada de destinos |
| Snapshots | `/usr/local/bin/churros-snapshot` | Snapshots btrfs de `@` y `@home` |

---

# Allowlist

| Binario | Condiciones |
|---------|-------------|
| `/usr/bin/pacman` | Solo `-S`, `-Sy`, `-Syy`, `-Syu`, `-R`, `-Sc`, `-Q`, `-Qs` y nombres de paquete planos. Se rechazan `--hookdir`, `--config`, `--dbpath`, `--cachedir`, `--gpgdir`, `--root`, `--sysroot`, `-C` y cualquier ruta absoluta |
| `/usr/bin/flatpak`, `/usr/bin/yay`, `/usr/bin/paru` | Autorizados (AUR compila y ejecuta como root: es la contrapartida de exponer `yay`) |
| `/usr/bin/timedatectl` | Solo operaciones de reloj y zona horaria |
| `/usr/bin/churros-update-utils` | Sin argumentos: el origen está fijado en el binario |
| `/usr/local/bin/churros-snapshot` | Verbos conocidos y stamps con formato `^\d{8}-\d{6}$` |
| `/usr/local/bin/churros-write-root-config` | Exactamente un destino, validado por el propio helper |
| `/usr/local/bin/churros-theme` | Sin argumentos |

**`churros-pkexec` no está autorizado.** Reenvía argumentos libres a `pkexec`, así
que permitirlo anularía la allowlist por completo.

---

# Escritura de configuración del sistema

Los ajustes que viven en `/etc` (autologin de greetd, saludo de ReGreet,
autologin de LightDM) se escriben con `churros-write-root-config`:

```bash
churros-pkexec churros-write-root-config /etc/greetd/config.toml < contenido
```

El helper:

- acepta **un solo** destino, y solo de su lista cerrada;
- lee el contenido de **stdin** (nunca de una ruta temporal compartida);
- escribe en un temporal `mktemp` **dentro del directorio de destino** y lo
  renombra de forma atómica.

Así se evita tanto la carrera de symlinks sobre un `/tmp` predecible como la
posibilidad de escribir en rutas arbitrarias.

---

# Actualización de las utilidades

`churros-update-utils` se ejecuta como root y extrae sobre `/`, así que:

- el origen (`https://download.churroslinux.org/churros/`) está fijado en el
  binario y el comando **no acepta argumentos**;
- versión, nombre de fichero y sha256 del manifiesto se validan con expresiones
  regulares;
- la lista de miembros del tarball se valida antes de extraer: sin rutas
  absolutas, sin `..`, sin enlaces simbólicos o duros, y solo sobre los prefijos
  que el bundle puede tocar;
- la extracción se hace en un directorio de stage y solo después se copia sobre
  `/`;
- si existe `/usr/share/churros/churros-release.pubkey`, el manifiesto debe
  además venir firmado por minisign.

Detalle en [live-services.md](live-services.md#churros-update-utils).

---

# Entorno Live

En el Live, el usuario `churros` tiene `NOPASSWD: ALL` en
`/etc/sudoers.d/churros` y el arranque de tty1 es un autologin de root (herencia
de archiso). Es intencional para un Live, y ese fichero se elimina en
`shellprocess@post-install` junto con los artefactos del usuario Live.

En consecuencia, en el Live la frontera real no es el privilegio sino el acceso
físico a la máquina: quien la tiene ya tiene root. Lo que sí se protege es el
sistema instalado, donde el usuario normal es `wheel` y su `sudo` **sí** pide
contraseña.

---

# Checklist al añadir un helper privilegiado

1. ¿La ruta está en el allowlist de `50-churros-store.rules`?
2. ¿El helper valida sus argumentos por sí mismo, en vez de confiar en la regla?
3. ¿El contenido llega por stdin y no por un temporal en `/tmp`?
4. ¿La escritura es atómica y en el directorio de destino?
5. ¿Hay una prueba que demuestre que un usuario sin privilegios no puede
   convertirlo en ejecución arbitraria?