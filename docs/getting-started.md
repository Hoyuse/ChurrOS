# Getting Started

Bienvenido al entorno de desarrollo de ChurrOS.

Esta guía explica cómo preparar el sistema, obtener el código fuente del proyecto y generar la primera imagen ISO.

---

# Requisitos

Actualmente ChurrOS está pensado para desarrollarse sobre Arch Linux o una distribución basada en Arch.

## Paquetes necesarios

Instala los siguientes paquetes:

```bash
sudo pacman -S \
    archiso \
    git \
    qemu-full \
    edk2-ovmf \
    virt-manager \
    swtpm
```

Algunos paquetes son opcionales dependiendo del flujo de trabajo.

| Paquete | Obligatorio |
|----------|-------------|
| archiso | ✅ |
| git | ✅ |
| qemu-full | ✅ |
| edk2-ovmf | ✅ |
| virt-manager | Opcional |
| swtpm | Opcional |

---

# Clonar el proyecto

```bash
git clone https://github.com/Hoyuse/ChurrOS.git

cd ChurrOS
```

---

# Estructura inicial

Después de clonar el proyecto encontrarás una estructura similar a la siguiente:

```text
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

---

# Compilar la ISO

Para construir una nueva imagen de ChurrOS simplemente ejecuta:

```bash
./churros build
```

Este comando realizará automáticamente las siguientes tareas:

1. Limpiar compilaciones anteriores.
2. Preparar el entorno de trabajo.
3. Ejecutar ArchISO.
4. Generar la imagen ISO.
5. Guardarla dentro de la carpeta `out/`.

---

# Ejecutar la ISO

Para probar la distribución rápidamente:

```bash
./churros run
```

Este comando:

- Construye la ISO si es necesario.
- Inicia una máquina virtual mediante QEMU.
- Arranca directamente desde la última ISO generada.

No modifica tu sistema anfitrión.

---

# Limpiar archivos temporales

Para eliminar todos los archivos generados durante la compilación:

```bash
./churros clean
```

Este comando elimina:

- work/
- out/

---

# Flujo de trabajo recomendado

Durante el desarrollo se recomienda seguir siempre este flujo:

Modificar archivos

↓

Compilar

```bash
./churros build
```

↓

Probar

```bash
./churros run
```

↓

Verificar cambios

↓

Realizar commit

↓

Push al repositorio

---

# Primeros cambios recomendados

Si acabas de comenzar a contribuir al proyecto, puedes empezar modificando:

- Branding
- Documentación
- Configuración del escritorio
- Fastfetch
- Wallpapers

Estos componentes permiten familiarizarse con la estructura del proyecto sin afectar partes críticas del sistema.

---

# Problemas frecuentes

## La ISO no se genera

Comprueba que `archiso` esté instalado correctamente.

```bash
pacman -Q archiso
```

---

## QEMU no inicia

Verifica que los paquetes necesarios estén instalados.

```bash
qemu-system-x86_64 --version
```

---

## No aparece la ISO

Comprueba el contenido de la carpeta:

```bash
out/
```

Si está vacía, ejecuta nuevamente:

```bash
./churros build
```

---

# Siguiente paso

Una vez que puedas generar correctamente una ISO de ChurrOS, continúa con la documentación de **Project Structure**, donde se explica en detalle la función de cada directorio del proyecto.
