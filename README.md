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

Actualmente ChurrOS se encuentra en una etapa temprana de desarrollo.

Características implementadas:

- Perfil personalizado de ArchISO.
- Sistema de branding propio.
- CLI de desarrollo (`./churros`).
- Construcción automática de la ISO.
- Ejecución automática en QEMU.
- Personalización del sistema Live.
- Documentación oficial.

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
├── configs/
├── docs/
├── installer/
├── scripts/
├── out/
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

## Fase 1

- Sistema de compilación
- Branding
- CLI
- Máquina virtual

## Fase 2

- Fastfetch personalizado
- Wallpapers
- Plymouth
- Personalización del bootloader

## Fase 3

- Hyprland
- Waybar
- Kitty
- Rofi
- Temas

## Fase 4

- Instalador gráfico

## Fase 5

- Primera versión estable

---

# Contribuir

Actualmente el proyecto está en desarrollo activo.

Toda contribución es bienvenida.

Consulta `docs/contributing.md`.

---

# Licencia

Este proyecto está distribuido bajo la licencia GPL-3.0.

---

<p align="center">
Desarrollado con ❤️ para la comunidad Linux.
</p>
