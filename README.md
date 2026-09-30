<div align="center">ChurrOS

Una distribución Linux basada en Arch Linux, diseñada con identidad propia.

""License" (https://img.shields.io/badge/license-GPL--3.0-blue.svg)" (LICENSE)
""Base" (https://img.shields.io/badge/base-Arch%20Linux-1793D1.svg)" (https://archlinux.org/)
""Build" (https://img.shields.io/badge/build-ArchISO-1793D1.svg)" (https://wiki.archlinux.org/title/Archiso)
""Rust" (https://img.shields.io/badge/apps-Rust-orange.svg)" (https://www.rust-lang.org/)
""Wayland" (https://img.shields.io/badge/desktop-Wayland-FFBC00.svg)" (https://wayland.freedesktop.org/)

</div>---

¿Qué es ChurrOS?

ChurrOS es una distribución Linux basada en Arch Linux y construida mediante ArchISO.

El proyecto busca ofrecer una experiencia moderna, organizada y coherente, combinando:

- Una base Arch Linux.
- Un sistema de construcción automatizado.
- Una identidad visual propia.
- Escritorios configurados por edición.
- Aplicaciones oficiales desarrolladas en Rust.
- Integración con Calamares.
- Herramientas propias de desarrollo y administración.
- Documentación orientada al mantenimiento del proyecto.

ChurrOS utiliza Arch Linux como base actualmente, mientras desarrolla progresivamente su propio ecosistema.

---

✨ Características

Característica| Estado
Base Arch Linux| ✅
ArchISO personalizado| ✅
CLI "./churros"| ✅
Construcción automatizada de ISO| ✅
QEMU para pruebas| ✅
Edición Niri| ✅
Edición XFCE| ✅
Aplicaciones oficiales en Rust| ✅
Calamares personalizado| ✅
Tema GRUB| ✅
Actualizador de ChurrOS| ✅
Rollback mediante snapshots Btrfs| ✅
Repositorio oficial de paquetes| 🚧
Instalador completamente propio| 🚧
Wiki oficial| 🚧
Manual de usuario| 🚧

---

🖥️ Ediciones

ChurrOS dispone actualmente de dos ediciones.

Niri

La edición predeterminada.

Utiliza:

- Niri
- Waybar
- foot
- Fuzzel
- Mako
- Aplicaciones oficiales de ChurrOS

Construcción:

./churros build

---

XFCE

Una edición basada en XFCE orientada a una experiencia de escritorio tradicional y ligera.

Construcción:

./churros build --edition xfce

---

🛠️ Sistema de construcción

ChurrOS utiliza una cadena de construcción basada en ArchISO.

                    Código fuente
                         │
                         ▼
                  ./churros build
                         │
          ┌──────────────┼──────────────┐
          │              │              │
          ▼              ▼              ▼
      Branding      Paquetes        Apps Rust
          │          locales            │
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                     mkarchiso
                         │
                         ▼
                    ISO de ChurrOS

El proceso de construcción incluye, entre otras tareas:

1. Selección de la edición.
2. Preparación del branding.
3. Generación del tema GRUB.
4. Construcción de Calamares.
5. Construcción de paquetes AUR locales.
6. Construcción de Bazaar.
7. Compilación de las aplicaciones Rust.
8. Preparación del perfil ArchISO.
9. Generación de la ISO.
10. Limpieza de artefactos temporales.

La ISO generada queda en:

out/

---

🚀 Inicio rápido

Requisitos

ChurrOS está diseñado para desarrollarse desde Arch Linux o una distribución basada en Arch.

Las dependencias principales incluyen:

archiso
git
qemu
edk2-ovmf
rust
cargo
xorriso
squashfs-tools
mtools
dosfstools
grub
mkinitcpio
shellcheck
gettext
pkgconf
sudo

También existen herramientas opcionales para determinados flujos de trabajo, como:

virt-manager
swtpm

La forma recomendada de comprobar el entorno es:

./churros doctor

El comando puede detectar dependencias faltantes y, cuando corresponde, ofrecer instalarlas mediante "pacman".

---

Clonar

git clone https://github.com/Hoyuse/ChurrOS.git
cd ChurrOS

Se recomienda trabajar mediante ramas y pull requests en lugar de realizar cambios directamente sobre "main".

---

🔨 Construir

Niri

./churros build

XFCE

./churros build --edition xfce

La ISO resultante se genera dentro de:

out/

La construcción utiliza "sudo" para ejecutar las partes de ArchISO que requieren privilegios.

---

🧪 Probar en QEMU

./churros run

Si no existe una ISO, "./churros run" puede solicitar al desarrollador que la construya.

Opciones disponibles:

./churros run --nokvm
./churros run --fresh
./churros run --clean

"--nokvm"

Ejecuta QEMU sin aceleración KVM.

"--fresh"

Regenera las variables UEFI de la máquina virtual.

"--clean"

Elimina el disco virtual y las variables UEFI antes de iniciar.

La máquina virtual utiliza un disco QCOW2 de desarrollo de 64 GiB.

Los archivos de la VM se almacenan en:

vm/

---

🧹 Limpiar

./churros clean

Esto elimina los artefactos generados de construcción, como:

work/
out/

No elimina el código fuente del proyecto.

---

🧰 CLI de ChurrOS

La herramienta principal de desarrollo es:

./churros

Comandos disponibles:

build
run
clean
check
doctor
info
version
logo
apps

Ejemplos:

./churros check
./churros doctor
./churros info
./churros version
./churros apps

Para obtener ayuda:

./churros --help

---

🦀 Aplicaciones oficiales

Las aplicaciones oficiales de ChurrOS están desarrolladas en Rust, utilizando principalmente:

- GTK 4
- libadwaita
- gtk4-rs

Actualmente existen:

Aplicación| Binario| Función
Welcome| "churros-welcome"| Pantalla de bienvenida
Settings| "churros-settings"| Configuración del sistema
Control Center| "churros-control-center"| Centro de control
Popups| "churros-popup"| Controles rápidos
Services| "churros_services"| Servicios compartidos

Los binarios con "deploy = true" se compilan durante el proceso de construcción y se despliegan en la ISO.

---

👋 ChurrOS Welcome

"churros-welcome" proporciona la pantalla inicial de ChurrOS.

Muestra información como:

- CPU
- RAM
- Kernel
- Sistema operativo
- Arquitectura
- Hostname

También proporciona accesos rápidos al:

- Instalador
- GitHub
- Comunidad

---

⚙️ ChurrOS Settings

"churros-settings" es la aplicación principal de configuración.

Está desarrollada con GTK4 y libadwaita y proporciona distintas páginas de configuración adaptadas al sistema.

En Niri puede abrirse mediante:

Mod + P

---

🎛️ ChurrOS Control Center

"churros-control-center" centraliza funciones del sistema como:

- Red
- Bluetooth
- Brillo
- Batería
- Audio

En Niri puede abrirse mediante:

Mod + C

---

🔔 ChurrOS Popups

"churros-popup" proporciona controles rápidos para:

audio
bluetooth
battery
brightness
network
power

Ejemplo:

churros-popup audio

---

📦 Paquetes y ecosistema

ChurrOS utiliza un repositorio local de paquetes durante la construcción.

El sistema puede construir e integrar componentes adicionales como:

- Calamares
- yay
- waypaper
- python-pywal
- Bazaar

El objetivo a largo plazo es disponer de un repositorio oficial de paquetes de ChurrOS.

---

💿 Instalador

ChurrOS utiliza actualmente Calamares como instalador gráfico.

El instalador incluye branding propio y configuración específica de ChurrOS.

Actualmente soporta, entre otras funciones:

- Particionado automático.
- Particionado manual.
- Selección de idioma.
- Selección de zona horaria.
- Creación de usuarios.
- Instalación del bootloader.
- Configuración posterior a la instalación.

El proyecto contempla desarrollar un instalador propio en el futuro, pero Calamares es el instalador utilizado actualmente.

---

🎨 Identidad visual

ChurrOS mantiene sus recursos visuales dentro de una estructura organizada.

Actualmente incluye:

- Logo oficial.
- Mascota.
- Wallpapers.
- Fastfetch personalizado.
- Iconos.
- Cursor.
- Tema GRUB.
- Configuración visual de las aplicaciones.
- Branding del sistema Live.

El objetivo es que ChurrOS tenga una identidad reconocible independientemente de su base Arch Linux.

---

🔄 Actualizaciones y rollback

ChurrOS incluye herramientas relacionadas con:

- Actualizaciones mediante pacman.
- Flatpak.
- Utilidades propias de ChurrOS.
- Snapshots Btrfs.
- Rollback mediante "churros-snapshot".

Estas funciones forman parte del ecosistema de administración de la distribución.

---

🧪 Sistema de comprobaciones

ChurrOS dispone de:

./churros check

El sistema de comprobaciones valida diferentes partes del repositorio, incluyendo:

- Sintaxis Bash.
- ShellCheck.
- Sintaxis Python.
- Listas de paquetes.
- Comandos utilizados por Niri.
- Desktop entries.
- Configuración de Calamares.
- Configuración de GRUB.
- Traducciones.
- Versionado.
- Higiene del repositorio.
- Integración de las aplicaciones Rust.

La comprobación termina con código de salida distinto de cero cuando encuentra fallos.

---

🩺 Diagnóstico del entorno

Para comprobar las herramientas disponibles en el equipo de desarrollo:

./churros doctor

El diagnóstico comprueba herramientas necesarias para:

- ArchISO.
- QEMU.
- Rust.
- Construcción de la ISO.
- GRUB.
- SquashFS.
- FAT.
- MTools.
- gettext.
- ShellCheck.
- pkg-config.
- Virtualización KVM.

También puede detectar si "/dev/kvm" está disponible.

---

🤖 Integración continua

ChurrOS utiliza GitHub Actions.

Actualmente la CI ejecuta dos trabajos principales:

GitHub Actions
      │
      ├── Static checks
      │      └── ./churros check
      │
      └── Rust tests
             └── cargo test

Los workflows se ejecutan para cambios en "main" y para pull requests.

La CI actual no construye la ISO completa.

---

📁 Estructura del proyecto

ChurrOS/
├── archiso/       # Perfil ArchISO
├── branding/      # Identidad y branding
├── docs/          # Documentación
├── installer/     # Configuración de Calamares
├── po/            # Traducciones
├── rust/          # Aplicaciones oficiales
├── scripts/       # Scripts de desarrollo
├── vm/            # Máquina virtual generada
├── out/           # ISO generada
├── work/          # ArchISO temporal
├── churros        # CLI principal
├── VERSION        # Versión del proyecto
├── LICENSE        # Licencia
└── README.md

Los directorios "out/", "work/" y "vm/" contienen artefactos generados y no forman parte del código fuente principal.

---

📚 Documentación

La documentación oficial se encuentra en:

docs/

Documentos principales:

- "getting-started.md"
- "project-structure.md"
- "build-system.md"
- "apps.md"
- "roadmap.md"
- "contributing.md"
- "vision.md"
- "services.md"
- "preferences.md"
- "popups.md"

Antes de modificar componentes importantes del proyecto, consulta la documentación correspondiente.

---

🗺️ Roadmap

La hoja de ruta oficial se encuentra en:

docs/roadmap.md

Fundación

- [x] Repositorio
- [x] ArchISO
- [x] Primera ISO
- [x] CLI
- [x] Branding
- [x] Documentación
- [x] QEMU
- [x] CI
- [x] Versión Alpha pública

Identidad

- [x] Logo
- [x] Mascota
- [x] Wallpapers
- [x] Fastfetch
- [x] Iconos
- [x] Cursor
- [x] Tema GRUB
- [ ] Plymouth
- [ ] Branding de greetd

Escritorio

- [x] Niri
- [x] Waybar
- [x] foot
- [x] Fuzzel
- [x] Mako
- [x] Centro de control
- [x] Tema oficial
- [x] Wlogout

Instalador

- [x] Calamares
- [x] Particionado automático
- [x] Particionado manual
- [x] Idiomas
- [x] Zona horaria
- [x] Usuarios
- [x] Bootloader

Ecosistema

- [x] CLI
- [x] Actualizador
- [x] Rollback Btrfs
- [x] Welcome
- [x] Settings
- [x] Control Center
- [x] Popups
- [ ] Repositorio oficial
- [ ] Ecosistema de paquetes propio completo

Publicación

- [x] v1.2
- [x] Publicación de ISO actual
- [ ] Wiki
- [ ] Manual de usuario
- [ ] Comunidad oficial

---

🤝 Contribuir

Las contribuciones son bienvenidas.

Antes de comenzar:

./churros doctor

Después de realizar cambios:

./churros check

Y, cuando corresponda:

./churros build
./churros run

El flujo recomendado es:

Crear rama
    ↓
Modificar código
    ↓
./churros check
    ↓
./churros build
    ↓
./churros run
    ↓
Verificar
    ↓
Commit
    ↓
Pull Request

Consulta:

docs/contributing.md

para conocer las convenciones del proyecto.

---

🧭 Filosofía

ChurrOS prioriza:

- Calidad sobre cantidad.
- Estabilidad.
- Organización.
- Automatización.
- Una experiencia coherente.
- Software libre.
- Mantenimiento a largo plazo.

El objetivo no es simplemente modificar Arch Linux visualmente.

El proyecto busca construir progresivamente una distribución con:

- identidad propia,
- herramientas propias,
- aplicaciones propias,
- documentación,
- infraestructura,
- comunidad,
- y un ecosistema mantenible.

---

📌 Estado actual

Versión del proyecto: "1.2"

ChurrOS cuenta actualmente con:

- ISO personalizada.
- Dos ediciones de escritorio.
- Sistema de construcción basado en ArchISO.
- CLI de desarrollo.
- Calamares.
- Aplicaciones oficiales en Rust.
- Centro de control.
- Herramientas de configuración.
- Popups del sistema.
- Actualizador.
- Rollback Btrfs.
- Branding propio.
- Tema GRUB.
- CI mediante GitHub Actions.
- Sistema de comprobaciones y diagnóstico.

El proyecto continúa en desarrollo activo.

---

📜 Licencia

ChurrOS se distribuye bajo:

GNU General Public License v3.0 (GPL-3.0).

Consulta ""LICENSE"" (LICENSE) para obtener el texto completo de la licencia.

---

<div align="center">ChurrOS

Una distribución construida con software libre, dedicación y una buena cantidad de café.

</div>