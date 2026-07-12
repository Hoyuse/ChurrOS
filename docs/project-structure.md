# Project Structure

Este documento describe la organización del repositorio de ChurrOS y el propósito de cada directorio.

Mantener una estructura clara facilita el mantenimiento, el desarrollo y la incorporación de nuevos colaboradores.

---

# Estructura general

```text
ChurrOS
├── archiso/
├── branding/
├── configs/
├── docs/
├── installer/
├── out/
├── scripts/
├── work/
├── churros
├── LICENSE
└── README.md
```

---

# Directorios

## archiso/

Contiene el perfil de ArchISO utilizado para construir la distribución.

Aquí se encuentra todo lo relacionado con la imagen Live de ChurrOS.

Ejemplos:

- paquetes
- configuración del sistema
- archivos del Live ISO
- servicios
- hooks

Es el corazón de la distribución.

---

## branding/

Contiene todos los recursos relacionados con la identidad de ChurrOS.

Ejemplos:

```text
branding
├── files/
├── logos/
├── wallpapers/
└── fastfetch/
```

Aquí se almacenan elementos como:

- issue
- motd
- os-release
- hostname
- logos
- fondos de pantalla
- fastfetch
- futuras imágenes del instalador

Todo lo relacionado con la identidad visual debe vivir aquí.

---

## configs/

Almacena configuraciones reutilizables.

Ejemplos futuros:

```text
configs
├── hyprland/
├── kitty/
├── waybar/
├── rofi/
└── sddm/
```

La idea es mantener separadas las configuraciones del sistema y poder reutilizarlas fácilmente.

---

## docs/

Documentación oficial del proyecto.

Incluye:

- instalación
- desarrollo
- branding
- estructura
- roadmap
- contribución

Toda funcionalidad importante debe estar documentada.

---

## installer/

Reservado para el futuro instalador gráfico de ChurrOS.

Actualmente la distribución utiliza ArchISO, pero este directorio contendrá el código fuente del instalador propio.

Ejemplo futuro:

```text
installer
├── backend/
├── frontend/
├── assets/
└── translations/
```

---

## scripts/

Scripts auxiliares utilizados durante el desarrollo.

Ejemplos:

- automatización
- compilación
- pruebas
- generación de archivos

Los scripts no forman parte del sistema instalado.

---

## out/

Directorio donde se generan las imágenes ISO.

Ejemplo:

```text
out/

ChurrOS-2026.07-x86_64.iso
```

No debe modificarse manualmente.

Su contenido puede eliminarse sin afectar el proyecto.

---

## work/

Directorio temporal utilizado por ArchISO.

Contiene archivos generados durante la compilación.

No forma parte del repositorio.

Puede eliminarse en cualquier momento.

---

# Archivos principales

## churros

CLI oficial de desarrollo.

Permite ejecutar comandos como:

```bash
./churros build
./churros run
./churros clean
```

En el futuro incorporará nuevas funciones para facilitar el desarrollo.

---

## README.md

Página principal del proyecto.

Es la primera documentación que verá cualquier usuario o colaborador.

---

## LICENSE

Licencia oficial del proyecto.

---

# Flujo del proyecto

El flujo general de desarrollo es el siguiente:

```text
Modificar código

↓

Compilar

↓

Generar ISO

↓

Probar en máquina virtual

↓

Realizar cambios

↓

Commit

↓

Push
```

---

# Organización del repositorio

Cada carpeta tiene una única responsabilidad.

No deben mezclarse archivos de distinta naturaleza.

Ejemplo:

❌ Incorrecto

```
branding/
    wallpaper.png
    build.sh
```

✅ Correcto

```
branding/
    wallpaper.png

scripts/
    build.sh
```

---

# Convenciones

Durante el desarrollo se siguen las siguientes reglas:

- Mantener una estructura simple.
- Utilizar nombres descriptivos.
- Evitar duplicar archivos.
- Mantener separados el código, la documentación y los recursos gráficos.
- Documentar cualquier cambio importante.

---

# Objetivo

La estructura del proyecto debe permanecer organizada incluso cuando ChurrOS crezca considerablemente.

Una buena organización facilita el mantenimiento, reduce errores y mejora la colaboración entre desarrolladores.