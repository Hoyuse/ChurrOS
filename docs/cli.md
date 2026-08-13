# ChurrOS CLI

La CLI de ChurrOS es la herramienta oficial utilizada para desarrollar, construir y probar la distribución.

Su objetivo es simplificar el flujo de trabajo del desarrollador y evitar ejecutar múltiples comandos manualmente.

Toda tarea repetitiva debe integrarse en esta herramienta.

---

# Filosofía

La CLI busca que desarrollar ChurrOS sea tan sencillo como ejecutar un único comando.

En lugar de recordar comandos largos de ArchISO o QEMU, todo se centraliza en:

```bash
./churros
```

---

# Uso

```bash
./churros <comando> [opciones]
```

Ejemplo:

```bash
./churros build
./churros run --fresh
```

Los argumentos posteriores al subcomando se reenvían al script `scripts/cli/<comando>.sh`. Un archivo `.sh` en ese directorio no es un comando público hasta que se añade a la lista explícita del dispatcher `churros`.

---

# Comandos disponibles

## build

Construye una nueva imagen ISO de ChurrOS.

```bash
./churros build
```

Este comando realiza automáticamente:

- Limpieza del directorio temporal.
- Preparación del entorno.
- Ejecución de ArchISO.
- Construcción de la imagen ISO.
- Almacenamiento de la ISO en `out/`.

---

## run

Construye la ISO (si es necesario) y la inicia en una máquina virtual utilizando QEMU.

```bash
./churros run
./churros run --nokvm
./churros run --fresh
./churros run --clean
```

Este comando permite probar rápidamente los cambios realizados sin necesidad de crear una máquina virtual manualmente.

Flags opcionales (detalle en `docs/vm.md`):

- `--nokvm` — emulación por software, sin KVM.
- `--fresh` — resetea `vm/OVMF_VARS.fd` para arrancar desde el CD-ROM.
- `--clean` — borra el disco de la VM y las variables EFI antes de arrancar.

---

## clean

Elimina todos los archivos temporales generados durante la compilación.

```bash
./churros clean
```

Directorios eliminados:

```text
work/
out/
```

No elimina ningún archivo del código fuente.

---

## check

Ejecuta las comprobaciones estáticas del repositorio.

```bash
./churros check
```

Revisa:

- Sintaxis de los scripts Bash y ShellCheck a nivel de error.
- Sintaxis de todos los archivos Python.
- Paquetes duplicados en `archiso/packages.x86_64`.
- Que los comandos del autostart de Niri existan como binario, crate Rust desplegable o paquete de la ISO.
- Que `Exec=` / `TryExec=` de los `.desktop` resuelvan, y que las rutas absolutas existan en airootfs.
- Orden crítico de Calamares y que cada instancia `shellprocess` tenga su `.conf`.
- Que los paquetes de `scripts/build-aur.sh` figuren en `netinstall.yaml`.
- Que los archivos de traducción `po/*.po` compilen.

Termina con código 0 si todo pasa y 1 si algo falla. Los avisos de higiene del repositorio se informan pero no bloquean. No necesita construir la ISO ni permisos de root, y tarda unos segundos.

Es el mismo comando que ejecuta el CI en cada Pull Request (ver `.github/workflows/ci.yml`).

---

## doctor

Comprueba que las herramientas del entorno de desarrollo estén instaladas (`mkarchiso`, `qemu-system-x86_64`, `xorriso`, etc.).

```bash
./churros doctor
```

---

## info

Muestra información del proyecto y del entorno (versión, rama, arquitectura y directorios).

```bash
./churros info
```

---

## version

Muestra la versión actual de la CLI.

```bash
./churros version
```

---

## logo

Muestra el logotipo oficial de ChurrOS en la terminal.

```bash
./churros logo
```

---

# Flujo recomendado

Durante el desarrollo se recomienda utilizar la siguiente secuencia:

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

Realizar commit

↓

Push

---

# Diseño

La CLI está diseñada para crecer junto con el proyecto.

Cada nueva funcionalidad de desarrollo debe añadirse como un nuevo comando: un script `scripts/cli/<comando>.sh` y una entrada en la lista explícita de comandos públicos del dispatcher `churros`. Un `.sh` extra en ese directorio no queda expuesto solo por existir.

Esto evita depender de múltiples scripts independientes.

---

# Comandos planificados

Las siguientes funciones están previstas para futuras versiones.

## release

```bash
./churros release
```

Permitirá generar una versión oficial de ChurrOS.

Automáticamente:

- Construirá la ISO.
- Generará checksums.
- Creará la versión.
- Preparará el Release.

---

## package

```bash
./churros package
```

Permitirá construir paquetes propios de ChurrOS.

---

## update

```bash
./churros update
```

Actualizará las dependencias del proyecto.

---

## docs

```bash
./churros docs
```

Abrirá la documentación oficial.

---

# Futuro

La CLI evolucionará hasta convertirse en la herramienta central del desarrollo de ChurrOS.

El objetivo es que prácticamente todas las tareas relacionadas con la distribución puedan ejecutarse desde un único comando.

Con el tiempo se añadirán nuevas funciones para automatizar procesos de compilación, pruebas, publicación y mantenimiento del proyecto.