ChurrOS

<p align="center">
  <strong>Una distribución Linux basada en Arch Linux, construida con ArchISO.</strong>
</p><p align="center">
  Desarrollo orientado a rendimiento, simplicidad, identidad propia y automatización.
</p><p align="center">
  <a href="https://github.com/Hoyuse/ChurrOS">
    <img src="https://img.shields.io/badge/GitHub-Hoyuse%2FChurrOS-black?logo=github" alt="GitHub">
  </a>
  <img src="https://img.shields.io/badge/Base-Arch%20Linux-1793D1?logo=archlinux&logoColor=white" alt="Arch Linux">
  <img src="https://img.shields.io/badge/Build-ArchISO-1793D1" alt="ArchISO">
  <img src="https://img.shields.io/badge/Language-Rust-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/License-GPL--3.0-blue" alt="GPL-3.0">
</p>---

¿Qué es ChurrOS?

ChurrOS es una distribución Linux basada en Arch Linux y construida mediante ArchISO.

El proyecto mantiene su propio sistema de construcción, branding, CLI de desarrollo, aplicaciones oficiales, configuración de escritorio e integración con Calamares.

El desarrollo está organizado alrededor de una infraestructura automatizada que permite preparar el sistema, construir la ISO y ejecutarla en una máquina virtual mediante QEMU.

La versión indicada actualmente por el repositorio es 1.2.

---

Características

- Basado en Arch Linux.
- Construcción mediante ArchISO.
- CLI de desarrollo "./churros".
- Edición Niri como configuración predeterminada.
- Edición XFCE disponible.
- Aplicaciones oficiales desarrolladas en Rust.
- Integración con GTK 4 y libadwaita.
- Instalador Calamares con branding de ChurrOS.
- Tema personalizado de GRUB.
- Paquetes locales utilizados durante la construcción.
- Ejecución de la ISO mediante QEMU.
- Sistema de comprobaciones estáticas mediante "./churros check".
- GitHub Actions para comprobaciones y pruebas de Rust.
- Sistema de actualización y rollback documentado en el proyecto.

---

Ediciones

ChurrOS dispone actualmente de dos perfiles de escritorio.

Edición| Escritorio| Sesión
Niri| Niri| Wayland
XFCE| XFCE| X11

La edición Niri es la utilizada por defecto.

Para seleccionar una edición durante la compilación:

./churros build --edition niri
./churros build --edition xfce

---

Requisitos de desarrollo

ChurrOS está diseñado para desarrollarse sobre Arch Linux o una distribución basada en Arch.

Los paquetes principales indicados por la documentación son:

sudo pacman -S \
    archiso \
    git \
    qemu-full \
    edk2-ovmf \
    rust \
    cargo

También pueden instalarse:

sudo pacman -S \
    virt-manager \
    swtpm

"virt-manager" y "swtpm" son opcionales.

El comando "./churros doctor" permite comprobar el entorno de desarrollo y detectar dependencias ausentes.

---

Inicio rápido

Clonar el repositorio

git clone https://github.com/Hoyuse/ChurrOS.git
cd ChurrOS

Comprobar el entorno

./churros doctor

Construir la ISO

./churros build

Para XFCE:

./churros build --edition xfce

La ISO generada queda en:

out/

Ejecutar ChurrOS

./churros run

Si no existe una ISO, el comando puede solicitar construirla.

Limpiar artefactos

./churros clean

Esto elimina los directorios de trabajo "work/" y "out/".

---

CLI

El ejecutable "./churros" es la interfaz principal para el desarrollo del proyecto.

./churros <command> [options]

Comandos disponibles:

Comando| Función
"build"| Construye la ISO
"run"| Ejecuta ChurrOS mediante QEMU
"clean"| Elimina artefactos de compilación
"check"| Ejecuta comprobaciones estáticas
"doctor"| Comprueba el entorno de desarrollo
"info"| Muestra información del proyecto
"version"| Muestra la versión
"logo"| Muestra el logo en la terminal
"apps"| Ejecuta aplicaciones de ChurrOS directamente en el host

Ejemplos:

./churros build
./churros build --edition xfce
./churros run
./churros run --nokvm
./churros run --fresh
./churros run --clean
./churros check
./churros doctor
./churros version
./churros info

---

Sistema de compilación

El sistema de compilación está basado en ArchISO y se encuentra automatizado mediante "./churros build".

El flujo general es:

Código fuente
      │
      ▼
./churros build
      │
      ├── Selección de edición
      │
      ├── Branding
      │
      ├── Tema GRUB
      │
      ├── Paquetes locales
      │
      ├── Aplicaciones Rust
      │
      ▼
   mkarchiso
      │
      ▼
     ISO
      │
      ▼
     out/

Durante la compilación se preparan, entre otros componentes:

- Branding de ChurrOS.
- Tema GRUB.
- Paquetes locales.
- Calamares.
- Paquetes AUR utilizados por el sistema.
- Aplicaciones Rust.
- Perfil de ArchISO.

Finalmente se ejecuta "mkarchiso" para generar la ISO.

---

Aplicaciones oficiales

Las aplicaciones oficiales de ChurrOS están desarrolladas en Rust utilizando gtk4-rs y libadwaita.

Actualmente el workspace contiene:

rust/
├── churros-welcome/
├── preferences/
├── control-center/
├── popups/
└── services/

ChurrOS Welcome

Binario:

churros-welcome

Proporciona una interfaz de bienvenida y muestra información básica del sistema, además de accesos relacionados con ChurrOS.

ChurrOS Settings

Binario:

churros-settings

Aplicación principal de configuración del sistema.

ChurrOS Control Center

Binario:

churros-control-center

Centro de control que proporciona acceso a diferentes funciones del sistema.

ChurrOS Popup

Binario:

churros-popup

Implementa los popups utilizados para funciones como:

- Audio.
- Bluetooth.
- Batería.
- Brillo.
- Red.
- Energía.

Services

El crate:

churros_services

proporciona servicios compartidos utilizados por las aplicaciones oficiales.

---

Desarrollo de aplicaciones

Las aplicaciones pueden ejecutarse directamente en el host para facilitar su desarrollo.

Ejemplos:

./churros apps welcome
./churros apps settings
./churros apps control-center
./churros apps popup audio

También existe una vista previa de Calamares:

./churros apps calamares

Estas herramientas permiten trabajar en las aplicaciones sin necesidad de construir una ISO para cada cambio.

---

Calamares

ChurrOS utiliza Calamares como instalador gráfico.

La configuración se encuentra en:

installer/
└── calamares/

La configuración incluye:

- "settings.conf"
- módulos propios de configuración
- branding de ChurrOS
- configuración de instalación
- integración con el sistema de paquetes local.

El proyecto mantiene scripts para construir e integrar Calamares durante el proceso de creación de la ISO.

---

Paquetes y componentes locales

El sistema de construcción utiliza un repositorio pacman local situado en:

archiso/packages/

Durante el proceso de compilación pueden construirse e integrarse componentes adicionales, entre ellos:

- Calamares.
- "python-pywal".
- "waypaper".
- "yay".
- Bazaar.

Estos paquetes se preparan durante el proceso de construcción y se incorporan al entorno de ChurrOS cuando corresponde.

---

Branding

El branding se encuentra principalmente en:

branding/

Incluye componentes como:

branding/
├── customize_airootfs.sh
├── files/
├── grub-theme/
├── colors.md
├── typography.md
├── logo-guidelines.md
├── mascot.md
└── ui-guidelines.md

Durante la compilación, estos recursos se incorporan al sistema Live.

El proyecto también genera e integra el tema GRUB utilizado por el sistema instalado.

---

Entornos de escritorio

Niri

La edición Niri utiliza:

- Niri.
- Waybar.
- foot.
- Fuzzel.
- Mako.

La configuración del escritorio se encuentra dentro del perfil de ArchISO.

XFCE

La edición XFCE proporciona una alternativa de escritorio basada en X11.

La selección se realiza durante la compilación:

./churros build --edition xfce

---

Comprobaciones

ChurrOS dispone de un sistema de comprobaciones mediante:

./churros check

Entre las comprobaciones realizadas se encuentran:

- Sintaxis de scripts Bash.
- ShellCheck cuando está disponible.
- Sintaxis de archivos Python.
- Duplicados en listas de paquetes.
- Resolución de comandos utilizados por Niri.
- Resolución de entradas ".desktop".
- Rutas absolutas utilizadas por aplicaciones.
- Secuencia de ejecución de Calamares.
- Configuraciones de módulos de Calamares.
- Traducciones gettext.
- Otros controles de integridad del repositorio.

El objetivo es detectar errores antes de iniciar una compilación completa.

---

GitHub Actions

El repositorio utiliza GitHub Actions mediante:

.github/workflows/ci.yml

Actualmente el workflow contiene dos trabajos principales.

Static checks

Ejecuta:

./churros check

e instala las herramientas necesarias para las comprobaciones de ShellCheck y gettext.

Tests Rust

Ejecuta:

cargo test -p churros-services --manifest-path rust/Cargo.toml

El workflow se ejecuta en:

- Push a "main".
- Pull requests.

---

QEMU

ChurrOS incluye una configuración de máquina virtual para probar la ISO.

El comando principal es:

./churros run

El sistema utiliza:

- QEMU.
- OVMF/UEFI.
- Un disco "qcow2".
- Virtio.
- Aceleración KVM cuando está disponible.
- Renderizado gráfico mediante virtio/virgl cuando el host lo permite.

Los archivos generados para la máquina virtual se encuentran en:

vm/

Por ejemplo:

vm/
├── ChurrOS.qcow2
└── OVMF_VARS.fd

Estos archivos son generados durante las pruebas y no forman parte del código fuente.

Opciones disponibles:

./churros run --nokvm
./churros run --fresh
./churros run --clean

---

Estructura del repositorio

ChurrOS/
├── archiso/
├── branding/
├── docs/
├── installer/
├── po/
├── rust/
├── scripts/
├── out/
├── vm/
├── work/
├── churros
├── VERSION
├── LICENSE
└── README.md

"archiso/"

Perfil utilizado para generar la ISO.

"branding/"

Identidad visual y scripts relacionados con el branding.

"docs/"

Documentación del proyecto.

"installer/"

Configuración e integración de Calamares.

"po/"

Archivos de traducción gettext.

"rust/"

Workspace de las aplicaciones oficiales.

"scripts/"

Scripts utilizados durante el proceso de construcción y desarrollo.

"out/"

Salida de las imágenes ISO generadas.

"work/"

Archivos temporales utilizados durante la construcción.

"vm/"

Archivos generados para las máquinas virtuales de desarrollo.

---

Documentación

La documentación adicional está organizada dentro de "docs/".

Documentos importantes:

- "getting-started.md"
- "project-structure.md"
- "build-system.md"
- "apps.md"
- "roadmap.md"
- "contributing.md"
- "vision.md"

La documentación debe mantenerse sincronizada con el comportamiento real del proyecto.

---

Roadmap

El roadmap oficial se encuentra en:

docs/roadmap.md

El estado documentado actualmente incluye:

Fundación

- ArchISO.
- CLI.
- Primera ISO.
- Branding.
- Documentación.
- QEMU.
- CI.
- Versión Alpha.

Identidad

- Logo.
- Mascota.
- Wallpapers.
- Fastfetch.
- Iconos.
- Cursor.
- Tema GRUB.

Pendientes documentados:

- Plymouth.
- Branding de greetd.

Escritorio

Implementados:

- Niri.
- Waybar.
- foot.
- Fuzzel.
- Mako.
- Centro de control.
- Tema oficial.

Pendiente:

- Wlogout.

Instalador

Documentado como completado mediante Calamares con:

- Particionado automático.
- Particionado manual.
- Selección de idioma.
- Zona horaria.
- Creación de usuario.
- Instalación del bootloader.
- Configuración inicial.

Ecosistema

Actualmente existen:

- CLI.
- Actualizador.
- Rollback mediante snapshots Btrfs.
- Welcome.
- Settings.
- Control Center.
- Popups.

El repositorio oficial de paquetes continúa marcado como pendiente.

Publicación

La versión indicada actualmente es:

1.2

El roadmap mantiene pendientes elementos como:

- Wiki.
- Manual de usuario.
- Comunidad.

---

Contribuir

ChurrOS se encuentra en desarrollo activo.

Antes de enviar cambios:

./churros check

Después:

./churros build

Y, cuando sea necesario, prueba la ISO:

./churros run

El flujo recomendado es trabajar en una rama independiente y enviar un pull request hacia "main".

Consulta:

docs/contributing.md

para las normas específicas del proyecto.

---

Licencia

ChurrOS se distribuye bajo:

GNU General Public License v3.0

Consulta "LICENSE" para el texto completo de la licencia.

---

Enlaces

- Sitio web: https://www.churroslinux.org/
- Repositorio: https://github.com/Hoyuse/ChurrOS

---

<p align="center">
  ChurrOS
</p>