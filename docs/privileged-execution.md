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
| Helper de paquetes | `/usr/local/bin/churros-pkg` | Instala o desinstala un paquete de Arch, validando el nombre |

---

# Allowlist

| Binario | Condiciones |
|---------|-------------|
| `/usr/bin/pacman` | Solo `-S`, `-Sy`, `-Syy`, `-Syu`, `-R`, `-Sc`, `-Q`, `-Qs` y nombres de paquete planos. Se rechazan `--hookdir`, `--config`, `--dbpath`, `--cachedir`, `--gpgdir`, `--root`, `--sysroot`, `-C` y cualquier ruta absoluta |
| `/usr/bin/flatpak` | Solo `install`, `remove`, `uninstall`, `update`, `repair`, `list`, `search`, `info`, `remotes` y `flathub`. Autorizar el binario entero permitía escalar a root con `flatpak run` o `flatpak override` |
| `/usr/bin/yay`, `/usr/bin/paru` | **No autorizados.** Compilan el `PKGBUILD` con `makepkg`, así que root con `yay` es root sin más. La instalación de AUR va en una terminal como el usuario, que es donde `yay` pide el sudo |
| `/usr/bin/timedatectl` | Solo operaciones de reloj y zona horaria |
| `/usr/bin/churros-update-utils` | Sin argumentos: el origen está fijado en el binario |
| `/usr/local/bin/churros-snapshot` | Verbos conocidos y stamps con formato `^\d{8}-\d{6}$` |
| `/usr/local/bin/churros-write-root-config` | Exactamente un destino, validado por el propio helper |
| `/usr/local/bin/churros-theme` | Sin argumentos |
| `/usr/local/bin/churros-pkg` | Exactamente `<install\|remove> <paquete>`, con el nombre contra `^[a-zA-Z0-9][a-zA-Z0-9@._+:-]*$` |

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

# Instalación de paquetes

`churros-software` no llama a `pacman` directamente: lo hace a través de
`/usr/local/bin/churros-pkg`, que es un helper estrecho.

```bash
churros-pkexec churros-pkg install firefox
churros-pkexec churros-pkg remove firefox
```

El helper:

- acepta **una sola** acción (`install` o `remove`) y **un solo** nombre;
- valida el nombre contra `^[a-zA-Z0-9][a-zA-Z0-9@._+:-]*$`, lo que descarta de
  entrada `--hookdir=/tmp/evil`, `-S`, `con espacio` o `../../etc/passwd`;
- comprueba que el paquete **existe** con `pacman -Si` antes de tocar nada, para
  que un nombre mal escrito no deje el registro de pacman a medias;
- llama a pacman con `--` delante del nombre, que es lo que impide que nada se
  interprete como opción;
- deja rastro en `/var/log/churros-software.log` (si el log se puede escribir).

## AUR no pasa por aquí

El AUR se instala en una terminal como el usuario, con `yay`, que pide el sudo
él mismo. Compilar un `PKGBUILD` con `makepkg` como root sería root sin más, así
que `yay` y `paru` **no** están en el allowlist.

---

# Checklist al añadir un helper privilegiado

1. ¿La ruta está en el allowlist de `50-churros-store.rules`?
2. ¿El helper valida sus argumentos por sí mismo, en vez de confiar en la regla?
3. ¿El contenido llega por stdin y no por un temporal en `/tmp`?
4. ¿La escritura es atómica y en el directorio de destino?
5. ¿Hay una prueba que demuestre que un usuario sin privilegios no puede
   convertirlo en ejecución arbitraria?
6. ¿La regla comprueba **los argumentos**, no solo el binario? Autorizar
   `/usr/bin/flatpak` entero era root: `flatpak run` y `flatpak override`
   permiten escribir donde quieras.
7. ¿El comando que acaba en root usa `--` antes de los datos que vienen de la
   aplicación?
8. Para herramientas que precompilan código (`yay`, `makepkg`, `configure`),
   ¿se ejecutan como el usuario y no como root?