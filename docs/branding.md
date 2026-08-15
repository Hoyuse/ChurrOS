# Branding

El sistema de branding de ChurrOS define la identidad visual y textual de la distribución.

Su objetivo es reemplazar completamente la identidad de Arch Linux dentro de la imagen Live y, en el futuro, del sistema instalado.

Toda modificación relacionada con la apariencia o identidad de ChurrOS debe realizarse desde este sistema.

---

# Objetivo

El branding permite que ChurrOS tenga una identidad propia.

Esto incluye:

- Nombre de la distribución.
- Mensajes de bienvenida.
- Información del sistema.
- Logos.
- Wallpapers.
- Fastfetch.
- Splash Screen.
- Bootloader.
- Instalador.

El objetivo es que el usuario nunca vea referencias innecesarias a Arch Linux durante el uso normal de ChurrOS.

---

# Estructura

Todo el branding se encuentra dentro del directorio:

```text
branding/
```

La estructura actual es:

```text
branding
├── customize_airootfs.sh
├── files/              # os-release, issue, motd
├── grub-theme/         # tema del GRUB instalado
├── colors.md
├── typography.md
├── logo-guidelines.md
├── mascot.md
└── ui-guidelines.md
```

Los wallpapers, logos de escritorio y Fastfetch viven en `archiso/airootfs/usr/share/churros/`. Plymouth y un tema propio de greetd aún no están. El branding de Calamares está en `installer/calamares/branding/churros/` (no se edita salvo que el cambio lo pida).

---

# files/

Contiene archivos de configuración que reemplazan los originales del sistema.

Ejemplo:

```text
branding/files

issue
motd
hostname
os-release
```

Estos archivos son copiados automáticamente durante la construcción de la ISO.

---

# issue

Archivo mostrado antes del inicio de sesión.

Actualmente muestra el banner oficial de ChurrOS.

Ejemplo:

```text
██████╗██╗  ██╗██╗   ██╗██████╗ ██████╗  ██████╗ ███████╗
██╔════╝██║  ██║██║   ██║██╔══██╗██╔══██╗██╔═══██╗██╔════╝
██║     ███████║██║   ██║██████╔╝██████╔╝██║   ██║███████╗
██║     ██╔══██║██║   ██║██╔══██╗██╔══██╗██║   ██║╚════██║
╚██████╗██║  ██║╚██████╔╝██║  ██║██║  ██║╚██████╔╝███████║
 ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝

Welcome to ChurrOS
```

---

# motd

Mensaje mostrado inmediatamente después del inicio de sesión.

Su propósito es dar la bienvenida al usuario y proporcionar información útil sobre la sesión Live.

Puede incluir:

- versión
- enlaces
- comandos útiles
- estado del sistema

---

# hostname

Define el nombre del equipo durante la sesión Live.

Ejemplo:

```text
churros
```

---

# os-release

Contiene la información oficial de la distribución.

Programas como:

- fastfetch
- neofetch
- screenfetch
- systemd

obtienen información desde este archivo.

Debe contener la identidad oficial de ChurrOS.

---

# Logos

Las guías están en `branding/logo-guidelines.md`. Los SVG/PNG que usa el escritorio viven en `archiso/airootfs/usr/share/churros/` (welcome, control-center, etc.).

---

# Wallpapers

Los fondos oficiales están en:

```text
archiso/airootfs/usr/share/churros/wallpapers/
```

---

# Fastfetch

La config live está en:

```text
archiso/airootfs/usr/share/churros/defaults/fastfetch/
```

---

# Plymouth

En el futuro el tema oficial de Plymouth se almacenará en:

```text
branding/plymouth/
```

Permitirá personalizar completamente la animación de arranque.

---

# Instalador

El branding de Calamares está en `installer/calamares/branding/churros/` (slideshow, QSS). No se toca salvo que el cambio lo pida explícitamente.

---

# Integración con la compilación

Durante la construcción de la ISO, el branding es copiado automáticamente al sistema Live.

El proceso se realiza mediante:

```text
customize_airootfs.sh
```

Este script reemplaza los archivos originales del sistema por los personalizados de ChurrOS.

---

# Buenas prácticas

Las guías y el script live viven en `branding/`. Los assets que el escritorio carga en runtime viven en `archiso/airootfs/usr/share/churros/`.

No mezclar scripts de build con recursos gráficos. No editar las copias generadas en el airootfs (`root/customize_airootfs.sh`, `root/branding/`).

---

# Objetivo a largo plazo

El sistema de branding evolucionará para abarcar toda la experiencia del usuario.

Ya cubre Fastfetch, wallpapers, iconos, cursor, tema GRUB e instalador Calamares.

Sigue pendiente:

- Plymouth.
- Tema / branding de greetd.
- Sonidos del sistema.

El objetivo es que la experiencia visual sea consistente desde el arranque hasta el escritorio.