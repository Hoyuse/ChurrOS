# Getting Started

Bienvenido al entorno de desarrollo de ChurrOS.

Esta guía explica cómo preparar el sistema, obtener el código fuente del proyecto y generar la primera imagen ISO.

---

# Requisitos

ChurrOS está pensado para desarrollarse sobre Arch Linux o una distribución basada en Arch.

## Paquetes necesarios

```bash
sudo pacman -S \
    archiso \
    git \
    qemu-full \
    edk2-ovmf \
    rust \
    cargo \
    virt-manager \
    swtpm
```

`./churros build` puede instalar `rust`/`cargo` si faltan, pero conviene tenerlos de antemano.

| Paquete | Obligatorio |
|----------|-------------|
| archiso | ✅ |
| git | ✅ |
| qemu-full | ✅ |
| edk2-ovmf | ✅ |
| rust / cargo | ✅ (compila las apps oficiales) |
| virt-manager | Opcional |
| swtpm | Opcional |

Comprueba el entorno con:

```bash
./churros doctor
```

---

# Clonar el proyecto

```bash
git clone https://github.com/Hoyuse/ChurrOS.git

cd ChurrOS
```

No se trabaja directo en `main`. Crea una rama por cambio y abre un pull request.

---

# Estructura inicial

Después de clonar el proyecto encontrarás una estructura similar a la siguiente:

```text
ChurrOS
├── archiso/
├── branding/
├── docs/
├── installer/
├── po/
├── rust/
├── scripts/
├── churros
├── VERSION
└── README.md
```

`out/`, `work/` y `vm/` aparecen al construir o ejecutar la ISO.

---

# Compilar la ISO

```bash
./churros build
```

Este comando:

1. Copia branding y tema GRUB al airootfs.
2. Construye paquetes AUR locales si faltan.
3. Compila las apps Rust y las deja en `usr/bin/`.
4. Ejecuta ArchISO (`mkarchiso`).
5. Deja la ISO en `out/`.

Hace falta `sudo` para `mkarchiso`.

---

# Ejecutar la ISO

```bash
./churros run
```

Este comando:

- Construye la ISO si es necesario.
- Inicia una máquina virtual mediante QEMU.
- Arranca desde la última ISO generada.

No modifica el sistema anfitrión. Flags útiles: `--fresh` (UEFI limpia), `--nokvm`, `--clean`. Detalle en `docs/vm.md`.

---

# Limpiar archivos temporales

```bash
./churros clean
```

Elimina `work/` y `out/`.

---

# Flujo de trabajo recomendado

Modificar archivos

↓

```bash
./churros check
```

↓

```bash
./churros build
```

↓

```bash
./churros run
```

↓

Verificar cambios

↓

Commit en una rama y pull request

---

# Primeros cambios recomendados

Si acabas de comenzar a contribuir, puedes empezar por:

- Documentación
- CLI (`scripts/cli/`)
- Apps en `rust/`
- Configuración del escritorio (con cuidado: el skel temático no se toca salvo bugs)

Estos componentes permiten familiarizarse con la estructura sin tocar branding ni el instalador.

---

# Problemas frecuentes

## La ISO no se genera

Comprueba que `archiso` esté instalado.

```bash
pacman -Q archiso
```

## QEMU no inicia

```bash
qemu-system-x86_64 --version
```

## Falla la compilación Rust

```bash
pacman -Q rust cargo
cargo build --release --manifest-path rust/Cargo.toml
```

## No aparece la ISO

```bash
ls out/
```

Si está vacía, ejecuta de nuevo `./churros build`.

---

# Siguiente paso

Cuando puedas generar una ISO, continúa con **Project Structure** (`docs/project-structure.md`) y **Apps** (`docs/apps.md`).
