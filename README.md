# ChurrOS


<h1 align="center">ChurrOS</h1>

<p align="center">
A modern Arch Linux based distribution focused on performance, simplicity and aesthetics.
</p>

---

# ¿Qué es ChurrOS?

ChurrOS es una distribución Linux basada en Arch Linux desarrollada desde cero utilizando ArchISO como sistema de construcción.

El objetivo del proyecto es crear una distribución moderna, rápida y elegante con una identidad propia, un entorno de escritorio cuidadosamente diseñado y herramientas de desarrollo que faciliten su mantenimiento.

Aunque actualmente utiliza Arch Linux como base, el objetivo a largo plazo es que ChurrOS tenga su propio ecosistema de herramientas, branding e instalador gráfico.

---

# Objetivos

- Crear una distribución Linux moderna.
- Mantener una experiencia limpia y consistente.
- Automatizar completamente el proceso de construcción.
- Tener un escritorio elegante y funcional.
- Desarrollar un instalador gráfico propio.
- Mantener una documentación clara y completa.

---

# Estado del proyecto

Actualmente ChurrOS se encuentra en una etapa temprana de desarrollo. La versión actual es **v0.6**.

Características implementadas:

- Perfil personalizado de ArchISO.
- Sistema de branding propio.
- CLI de desarrollo (`./churros`).
- Construcción automática de la ISO (incluye apps Rust y paquetes AUR locales).
- Ejecución automática en QEMU.
- Personalización del sistema Live.
- Documentación oficial.
- **Escritorio completo** — Niri + Waybar + foot + Fuzzel + Mako.
- **Apps oficiales en Rust** — `churros-welcome`, `churros-settings`, `churros-control-center` y `churros-popup` (gtk4-rs + libadwaita).
- **Panel de preferencias** (`churros-settings`) — 30+ páginas GTK4.
- **Popups integrados** — audio, bluetooth, batería, brillo, red, power.
- **Instalador gráfico** — Calamares con branding ChurrOS.
- **Tema GRUB** — menú centrado aplicado al sistema instalado.
- **CI** — `./churros check` corre en GitHub Actions.

---

# Filosofía

ChurrOS prioriza la calidad sobre la velocidad.

Cada componente del sistema se desarrolla cuidadosamente con el objetivo de mantener una distribución organizada, fácil de mantener y agradable de utilizar.

No se busca crear una simple personalización de Arch Linux, sino una distribución con identidad propia.

---

# Requisitos

Para desarrollar ChurrOS se recomienda utilizar Arch Linux.

Paquetes necesarios:

- archiso
- git
- qemu-full
- edk2-ovmf
- rust y cargo (para las apps oficiales)
- virt-manager (opcional)
- swtpm (opcional)

---

# Inicio rápido

Clonar el proyecto

```bash
git clone https://github.com/Hoyuse/ChurrOS.git
cd ChurrOS
```

Construir la ISO

```bash
./churros build
```

Ejecutar la ISO

```bash
./churros run
```

Limpiar archivos temporales

```bash
./churros clean
```

---

# Estructura del proyecto

```
ChurrOS
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
└── README.md
```

Una explicación detallada de cada carpeta se encuentra dentro de `docs/project-structure.md`.

---

# Documentación

Toda la documentación oficial se encuentra en la carpeta `docs/`.

- Getting Started
- Project Structure
- Build System
- Branding
- CLI
- Development
- Release
- Roadmap
- Contributing
- Vision

---

# Roadmap

El roadmap detallado con el progreso de cada fase está en `docs/roadmap.md`.

- **Fase 1 — Fundación**: completada (CI integrada).
- **Fase 2 — Identidad**: en curso (logo, mascota, wallpapers, fastfetch, iconos, cursor y tema GRUB; faltan Plymouth y branding de greetd).
- **Fase 3 — Escritorio**: en curso (Niri, Waybar, foot, Fuzzel, Mako, centro de control; falta Wlogout).
- **Fase 4 — Instalador**: completada (Calamares con branding ChurrOS; GRUB UEFI + Syslinux BIOS).
- **Fase 5 — Ecosistema**: en curso (apps oficiales en Rust; faltan actualizador y repositorio oficial).
- **Fase 6 — Publicación**: en curso (release **v0.6** publicada; faltan 1.0, sitio y wiki).

---

# Contribuir

Actualmente el proyecto está en desarrollo activo.

Toda contribución es bienvenida.

Consulta `docs/contributing.md`.

---

# Licencia

Este proyecto está distribuido bajo la licencia GNU General Public License v3.0 (GPL-3.0).

---

<p align="center">
Desarrollado con ❤️ para la comunidad Linux.
</p>

