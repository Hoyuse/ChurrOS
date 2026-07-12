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
./churros <comando>
```

Ejemplo:

```bash
./churros build
```

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
```

Este comando permite probar rápidamente los cambios realizados sin necesidad de crear una máquina virtual manualmente.

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

# Flujo recomendado

Durante el desarrollo se recomienda utilizar la siguiente secuencia:

Modificar archivos

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

Cada nueva funcionalidad de desarrollo debe añadirse como un nuevo comando.

Esto evita depender de múltiples scripts independientes.

---

# Comandos planificados

Las siguientes funciones están previstas para futuras versiones.

## doctor

```bash
./churros doctor
```

Verificará automáticamente:

- Dependencias instaladas.
- Estado del entorno.
- Versiones requeridas.
- Configuración de virtualización.

---

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

## version

```bash
./churros version
```

Mostrará la versión actual del proyecto.

---

## logo

```bash
./churros logo
```

Mostrará el logotipo oficial de ChurrOS en la terminal.

---

# Futuro

La CLI evolucionará hasta convertirse en la herramienta central del desarrollo de ChurrOS.

El objetivo es que prácticamente todas las tareas relacionadas con la distribución puedan ejecutarse desde un único comando.

Con el tiempo se añadirán nuevas funciones para automatizar procesos de compilación, pruebas, publicación y mantenimiento del proyecto.