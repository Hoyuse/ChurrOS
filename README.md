<div align="center">🍪 ChurrOS

Linux ligero, moderno y con identidad propia.

Construido sobre Arch Linux · Niri · XFCE · Calamares · Rust · ArchISO

""License" (https://img.shields.io/badge/license-GPL--3.0-blue.svg)" (LICENSE)
""Base" (https://img.shields.io/badge/base-Arch%20Linux-1793D1.svg)" (https://archlinux.org/)
""Build" (https://img.shields.io/badge/build-ArchISO-1793D1.svg)" (https://wiki.archlinux.org/title/Archiso)
""Language" (https://img.shields.io/badge/apps-Rust-orange.svg)" (https://www.rust-lang.org/)
""Wayland" (https://img.shields.io/badge/display-Wayland-FFBC00.svg)" (https://wayland.freedesktop.org/)

Rendimiento primero · Diseño después · Software libre siempre

</div>---

🧭 ¿Qué es ChurrOS?

ChurrOS es una distribución Linux basada en Arch Linux, diseñada para ofrecer una experiencia moderna, ligera y consistente incluso en equipos con recursos limitados.

El proyecto combina una base Arch con una experiencia cuidadosamente integrada:

- 🪶 Configuración orientada a hardware modesto
- 🖥️ Ediciones Niri y XFCE
- 🎨 Identidad visual propia
- 🚀 CLI de desarrollo ("./churros")
- 📦 Construcción mediante ArchISO
- 🦀 Aplicaciones oficiales escritas en Rust
- 🛠️ Instalador gráfico basado en Calamares
- 🔄 Actualizador y sistema de rollback
- 🧪 Verificaciones automáticas mediante GitHub Actions

ChurrOS no pretende ser solamente una personalización de Arch Linux. El objetivo es construir una distribución con herramientas, identidad y experiencia propias.

---

✨ Características

Característica| Estado
Base Arch Linux| ✅
ArchISO personalizado| ✅
CLI "./churros"| ✅
Construcción automática de ISO| ✅
Niri Edition| ✅
XFCE Edition| ✅
Calamares| ✅
GRUB personalizado| ✅
Syslinux / BIOS| ✅
UEFI / GRUB| ✅
Apps oficiales en Rust| ✅
Centro de control| ✅
Aplicación de configuración| ✅
Popups del sistema| ✅
Aplicación de bienvenida| ✅
Actualizador| ✅
Rollback Btrfs| ✅
CI con GitHub Actions| ✅
Repositorio oficial de paquetes| 🚧
Instalador propio| 🗺️
Wiki oficial| 🗺️

---

🖥️ Ediciones

ChurrOS ofrece diferentes experiencias de escritorio para adaptarse a distintos tipos de hardware y preferencias.

🌀 Niri Edition

La edición principal utiliza Niri, un compositor Wayland de mosaico desplazable.

Incluye:

- Niri
- Waybar
- Fuzzel
- foot
- Mako
- Centro de control ChurrOS
- Popups integrados
- Configuración personalizada

Es la experiencia moderna de ChurrOS.

---

🪶 XFCE Edition

Una alternativa clásica y ligera basada en XFCE.

Pensada para usuarios que prefieren:

- Un escritorio tradicional
- Menor complejidad
- Compatibilidad amplia
- Una experiencia familiar

---

🧰 CLI de ChurrOS

La herramienta principal de desarrollo es:

./churros

Algunos comandos disponibles:

./churros build
./churros run
./churros check
./churros doctor
./churros apps
./churros clean
./churros info
./churros version

Construir

./churros build

Construye la edición Niri por defecto.

Para XFCE:

./churros build --edition xfce

---

Ejecutar en QEMU

./churros run

Opciones disponibles según el entorno:

./churros run --fresh
./churros run --nokvm
./churros run --clean

---

Verificar el proyecto

./churros check

Comprueba automáticamente diferentes partes del proyecto:

- Sintaxis Bash
- ShellCheck
- Sintaxis Python
- Listas de paquetes
- Configuración de Niri
- Archivos ".desktop"
- Configuración de Calamares
- Branding de Calamares
- Paquetes AUR locales
- Traducciones gettext

Estas comprobaciones también forman parte del CI.

---

Diagnóstico del entorno

./churros doctor

Comprueba herramientas necesarias como:

- "mkarchiso"
- QEMU
- "xorriso"
- "mksquashfs"
- "mcopy"
- "mkinitcpio"
- Rust
- Cargo
- ShellCheck
- gettext

También puede detectar problemas con KVM y dependencias faltantes.

---

🏗️ Sistema de construcción

El sistema de compilación está basado en ArchISO.

                         ChurrOS
                            │
                            ▼
                     ./churros build
                            │
             ┌──────────────┼──────────────┐
             │              │              │
             ▼              ▼              ▼
          Branding     Paquetes locales   Apps Rust
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                       mkarchiso
                            │
                            ▼
                    ┌───────────────┐
                    │   ChurrOS ISO │
                    └───────────────┘
                            │
                            ▼
                           QEMU

El proceso integra:

1. Branding
2. Configuración del sistema
3. Paquetes locales
4. Calamares
5. Aplicaciones Rust
6. Traducciones
7. Perfil ArchISO
8. GRUB
9. Syslinux
10. Generación de la ISO

---

🦀 Aplicaciones oficiales

Las aplicaciones oficiales están desarrolladas en Rust, utilizando GTK4 y Libadwaita.

rust/
├── churros-welcome/
├── preferences/
├── control-center/
├── popups/
└── services/

"churros-welcome"

Aplicación de bienvenida.

Muestra información como:

- CPU
- RAM
- Kernel
- Sistema operativo
- Arquitectura
- Hostname

También proporciona accesos rápidos a funciones importantes del sistema.

---

"churros-settings"

Aplicación principal de configuración de ChurrOS.

Proporciona una interfaz integrada con la identidad visual de la distribución.

---

"churros-control-center"

Centro de control rápido.

Incluye acceso a:

- 🌐 Red
- 🔵 Bluetooth
- 🔆 Brillo
- 🔋 Batería
- 🔊 Audio
- ⚙️ Configuración
- ⏻ Opciones de energía

---

"churros-popup"

Sistema unificado de popups.

Incluye:

audio
bluetooth
battery
brightness
network
power

Todos se gestionan mediante un único binario.

---

📦 Sistema de paquetes

Durante la construcción, ChurrOS puede generar paquetes locales para componentes que todavía no forman parte de un repositorio oficial.

El repositorio local se encuentra en:

archiso/packages/

Puede contener componentes como:

calamares
yay
waypaper
python-pywal

Estos paquetes pueden utilizarse durante la construcción y la instalación.

---

💿 Instalador

ChurrOS utiliza Calamares como instalador gráfico.

La configuración propia del proyecto se encuentra en:

installer/
└── calamares/
    ├── settings.conf
    ├── branding/
    ├── modules/
    └── preview/

El instalador soporta:

- Particionado automático
- Particionado manual
- Selección de idioma
- Zona horaria
- Creación de usuario
- Instalación del bootloader
- Configuración inicial
- Branding de ChurrOS

Actualmente utiliza:

- GRUB para UEFI
- Syslinux para BIOS

---

🎨 Identidad visual

ChurrOS mantiene una identidad visual propia.

branding/
├── customize_airootfs.sh
├── files/
├── grub-theme/
├── colors.md
├── typography.md
├── logo-guidelines.md
├── mascot.md
└── ui-guidelines.md

La identidad incluye:

- Logo
- Mascota
- Wallpapers
- Iconos
- Cursor
- Fastfetch
- Tema GRUB
- Colores
- Tipografía
- Configuración visual del escritorio

---

🔄 Actualizaciones y rollback

ChurrOS incluye herramientas para gestionar actualizaciones.

El sistema contempla:

- Pacman
- Flatpak
- Utilidades propias de ChurrOS

También incluye rollback mediante snapshots Btrfs:

churros-snapshot
        │
        ▼
   Btrfs snapshot
        │
        ▼
      Rollback

---

🧪 CI

ChurrOS utiliza GitHub Actions para comprobar automáticamente el proyecto.

Push / Pull Request
        │
        ├── Static checks
        │       └── ./churros check
        │
        └── Rust tests
                └── cargo test

El workflow comprueba el código automáticamente en Pull Requests y cambios dirigidos a "main".

---

🛠️ Requisitos de desarrollo

ChurrOS está diseñado para desarrollarse sobre Arch Linux o distribuciones basadas en Arch.

Instala los paquetes principales:

sudo pacman -S \
    archiso \
    git \
    qemu-full \
    edk2-ovmf \
    rust \
    cargo

Opcionales:

sudo pacman -S \
    virt-manager \
    swtpm

Después comprueba el entorno:

./churros doctor

---

🚀 Inicio rápido

1. Clonar

git clone https://github.com/Hoyuse/ChurrOS.git
cd ChurrOS

2. Comprobar el entorno

./churros doctor

3. Ejecutar las comprobaciones

./churros check

4. Construir la ISO

./churros build

5. Ejecutarla

./churros run

La ISO generada aparecerá en:

out/

---

📁 Estructura del proyecto

ChurrOS/
│
├── archiso/                 # Perfil ArchISO
│   ├── airootfs/            # Sistema Live
│   ├── grub/                # Configuración GRUB
│   ├── syslinux/            # Configuración BIOS
│   ├── packages/            # Repositorio local
│   ├── packages.x86_64
│   ├── packages.xfce.x86_64
│   └── profiledef.sh
│
├── branding/                # Identidad visual
│
├── docs/                    # Documentación
│
├── installer/               # Configuración Calamares
│
├── po/                      # Traducciones
│
├── rust/                    # Aplicaciones oficiales
│
├── scripts/                 # Scripts y CLI
│   └── cli/
│
├── vm/                      # Datos generados de QEMU
│
├── out/                     # ISO generada
│
├── work/                    # ArchISO temporal
│
├── churros                  # CLI principal
├── VERSION                  # Versión
├── LICENSE                  # Licencia
└── README.md

«"out/", "work/" y los datos de "vm/" son generados durante desarrollo y no forman parte del código fuente principal.»

---

🔨 Flujo de desarrollo

                 Modificar código
                       │
                       ▼
                ./churros check
                       │
                       ▼
                ./churros build
                       │
                       ▼
                  ./churros run
                       │
                       ▼
                     Probar
                       │
                       ▼
                  Crear rama
                       │
                       ▼
                    Commit
                       │
                       ▼
                 Pull Request

No se recomienda trabajar directamente sobre "main".

---

🤝 Contribuir

Las contribuciones son bienvenidas.

Antes de comenzar:

./churros doctor

Después de realizar cambios:

./churros check

Si el cambio afecta al escritorio, instalador o sistema de construcción:

./churros build
./churros run

Ramas

Ejemplos:

feature/nuevo-componente
feature/installer
fix/build-system
fix/calamares
docs/readme
ci/docker-build

Commits

Se recomiendan mensajes descriptivos:

feat: añadir nueva herramienta

fix: corregir el sistema de build

docs: actualizar documentación

ci: mejorar workflow

refactor: simplificar la CLI

---

📚 Documentación

La documentación completa se encuentra en ""docs/"" (docs/).

Documento| Contenido
""getting-started.md"" (docs/getting-started.md)| Preparar el entorno
""project-structure.md"" (docs/project-structure.md)| Estructura del repositorio
""build-system.md"" (docs/build-system.md)| Sistema de compilación
""apps.md"" (docs/apps.md)| Aplicaciones oficiales
""contributing.md"" (docs/contributing.md)| Guía para contribuidores
""roadmap.md"" (docs/roadmap.md)| Hoja de ruta
""vision.md"" (docs/vision.md)| Visión del proyecto

---

🗺️ Roadmap

Fundación

- [x] Repositorio
- [x] ArchISO
- [x] Primera ISO
- [x] CLI
- [x] Branding
- [x] Documentación
- [x] QEMU
- [x] CI

Identidad

- [x] Logo
- [x] Mascota
- [x] Wallpapers
- [x] Iconos
- [x] Cursor
- [x] Fastfetch
- [x] GRUB
- [ ] Plymouth
- [ ] Branding completo de greetd

Escritorio

- [x] Niri
- [x] XFCE
- [x] Waybar
- [x] foot
- [x] Fuzzel
- [x] Mako
- [x] Centro de control
- [ ] Wlogout

Instalador

- [x] Calamares
- [x] Particionado automático
- [x] Particionado manual
- [x] Localización
- [x] Zona horaria
- [x] Usuarios
- [x] GRUB UEFI
- [x] Syslinux BIOS

Ecosistema

- [x] CLI
- [x] Actualizador
- [x] Rollback Btrfs
- [x] Welcome
- [x] Settings
- [x] Control Center
- [x] Popups
- [ ] Repositorio oficial
- [ ] Ecosistema completo de paquetes

Publicación

- [x] Versión 1.2
- [x] Publicaciones de ISO
- [ ] Sitio web completo
- [ ] Wiki
- [ ] Manual de usuario
- [ ] Comunidad

---

🎯 Filosofía

Calidad antes que cantidad

ChurrOS no busca añadir características simplemente para aumentar una lista.

Cada componente debe aportar valor y mantener una experiencia consistente.

Automatización

Las tareas repetitivas deben poder automatizarse.

Por eso ChurrOS cuenta con una CLI que centraliza:

build
run
check
doctor
apps
clean
info
version

Identidad

Aunque ChurrOS utiliza Arch Linux como base, el objetivo es desarrollar una experiencia reconocible y propia.

Software libre

ChurrOS busca fomentar:

- Colaboración
- Aprendizaje
- Transparencia
- Compartir conocimiento
- Desarrollo comunitario

---

📊 Estado actual

Componente| Estado
Versión| "1.2"
Base| Arch Linux
Build system| ArchISO
Desktop principal| Niri
Desktop alternativo| XFCE
Display manager| greetd
Installer| Calamares
Display stack| Wayland / X11 según edición
Apps oficiales| Rust + GTK4 + Libadwaita
Virtualización| QEMU
CI| GitHub Actions
Licencia| GPL-3.0

---

📜 Licencia

ChurrOS se distribuye bajo los términos de la:

GNU General Public License v3.0

Consulta ""LICENSE"" (LICENSE) para obtener el texto completo.

---

<div align="center">🍪 ChurrOS

Linux ligero. Identidad propia. Software libre.

Hecho con ❤️ para la comunidad Linux.

</div>