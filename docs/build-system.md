# Build System

El sistema de compilación de ChurrOS está basado en **ArchISO** y automatizado mediante la herramienta de desarrollo `./churros`.

El objetivo del sistema de compilación es generar imágenes ISO reproducibles, mantener un flujo de trabajo sencillo y minimizar las tareas manuales.

---

# Arquitectura

El proceso de compilación puede resumirse de la siguiente manera:

```
               Código fuente
                     │
                     ▼
              ./churros build
                     │
                     ▼
          Preparación del entorno
                     │
                     ▼
              Ejecución de ArchISO
                     │
                     ▼
             Construcción de la ISO
                     │
                     ▼
          out/ChurrOS-*.iso
```

---

# Componentes

El sistema de compilación está compuesto por tres elementos principales:

- ArchISO
- CLI de ChurrOS
- Perfil personalizado ubicado en `archiso/`

---

# CLI

Toda la compilación se realiza mediante:

```bash
./churros build
```

Este comando automatiza completamente el proceso.

No es necesario ejecutar `mkarchiso` manualmente.

---

# Flujo de compilación

Cuando se ejecuta:

```bash
./churros build
```

ocurre lo siguiente:

## 1. Limpieza

Se elimina el directorio temporal.

```text
work/
```

Esto evita reutilizar archivos de compilaciones anteriores.

---

## 2. Preparación

Se crea la carpeta de salida si no existe.

```text
out/
```

---

## 3. Construcción

Se ejecuta:

```bash
mkarchiso
```

utilizando el perfil ubicado en:

```text
archiso/
```

ArchISO realiza:

- instalación de paquetes
- generación del initramfs
- construcción del sistema Live
- generación del sistema squashfs
- creación del cargador UEFI
- generación de la imagen ISO

---

## 4. Resultado

La ISO final se almacena en:

```text
out/
```

Ejemplo:

```text
out/

ChurrOS-2026.07-x86_64.iso
```

---

# Probar la distribución

Para iniciar la ISO en una máquina virtual:

```bash
./churros run
```

Este comando:

- busca la ISO más reciente
- inicia QEMU
- arranca automáticamente la distribución

No modifica el sistema anfitrión.

---

# Limpiar el proyecto

Para eliminar todos los archivos temporales:

```bash
./churros clean
```

Este comando elimina:

```
work/
out/
```

No elimina ningún archivo del proyecto.

---

# Directorios utilizados

## archiso/

Perfil de ArchISO.

Contiene todo el sistema Live.

---

## work/

Archivos temporales generados durante la compilación.

Puede eliminarse sin problemas.

---

## out/

Imágenes ISO generadas.

---

# Branding

Durante la compilación también se aplican las personalizaciones de ChurrOS.

Entre ellas:

- hostname
- issue
- motd
- os-release
- logos
- fondos de pantalla
- configuraciones del sistema

El branding forma parte del proceso de construcción y queda integrado dentro de la imagen Live.

---

# Errores comunes

## La ISO no aparece

Comprueba que exista:

```text
out/
```

Si está vacía:

```bash
./churros build
```

---

## Error de permisos

Si aparecen errores relacionados con permisos, elimina el directorio temporal:

```bash
./churros clean
```

y vuelve a compilar.

---

## ArchISO no encontrado

Comprueba que esté instalado.

```bash
pacman -Q archiso
```

---

## La compilación falla

Revisa el registro mostrado por `mkarchiso`.

La mayoría de los errores se producen por:

- paquetes inexistentes
- rutas incorrectas
- permisos
- configuraciones inválidas

---

# Buenas prácticas

Se recomienda seguir siempre el siguiente flujo:

```
Modificar archivos

↓

Compilar

↓

Probar en QEMU

↓

Corregir errores

↓

Commit

↓

Push
```

Nunca realizar cambios directamente sobre la ISO generada.

Todas las modificaciones deben hacerse sobre el código fuente del proyecto.

---

# Futuro

El sistema de compilación evolucionará conforme avance el desarrollo de ChurrOS.

Entre las mejoras planificadas se encuentran:

- Compilaciones incrementales.
- Verificación automática de errores.
- Generación de checksums.
- Generación automática de versiones.
- Integración con GitHub Actions.
- Creación automática de Releases.

El objetivo es que generar una nueva versión de ChurrOS sea un proceso completamente automatizado y reproducible.