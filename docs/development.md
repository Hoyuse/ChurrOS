# Development

Este documento describe el flujo de trabajo recomendado para desarrollar ChurrOS.

Su objetivo es mantener un proceso de desarrollo consistente, organizado y fácil de seguir.

---

# Filosofía

El desarrollo de ChurrOS se basa en cuatro principios fundamentales:

- Simplicidad.
- Organización.
- Automatización.
- Documentación.

Todo cambio importante debe estar documentado y ser fácilmente reproducible.

---

# Flujo de trabajo

El ciclo de desarrollo recomendado es el siguiente:

Modificar código

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

Verificar funcionamiento

↓

Realizar commit

↓

Enviar cambios

```bash
git push
```

---

# Organización del proyecto

Antes de agregar nuevos archivos, verifica que pertenezcan al directorio correcto.

Ejemplos:

Configuraciones

```
configs/
```

Documentación

```
docs/
```

Recursos gráficos

```
branding/
```

Scripts

```
scripts/
```

Perfil del sistema

```
archiso/
```

---

# Commits

Los commits deben ser pequeños y representar una única modificación lógica.

Ejemplos:

```
feat: add Fastfetch branding

fix: correct build script

docs: update branding guide

refactor: simplify CLI
```

Evita realizar commits con múltiples cambios no relacionados.

---

# Documentación

Toda nueva funcionalidad debe incluir su documentación correspondiente.

Ejemplos:

Nuevo comando

↓

Actualizar

```
docs/cli.md
```

Nuevo proceso de compilación

↓

Actualizar

```
docs/build-system.md
```

Nueva identidad visual

↓

Actualizar

```
docs/branding.md
```

---

# Pruebas

Antes de realizar un commit se recomienda ejecutar:

```bash
./churros build
```

y posteriormente:

```bash
./churros run
```

La ISO debe iniciar correctamente y los cambios deben verificarse manualmente.

---

# Buenas prácticas

- Utilizar nombres descriptivos.
- Evitar duplicar código.
- Mantener la estructura del proyecto organizada.
- Escribir comentarios únicamente cuando aporten valor.
- Mantener la documentación actualizada.

---

# Ramas

Actualmente el proyecto utiliza las siguientes ramas:

main

Versión estable.

archiso

Desarrollo activo.

En el futuro podrán añadirse ramas específicas para nuevas funcionalidades.

---

# Objetivo

El objetivo del flujo de desarrollo es mantener un proyecto limpio, organizado y fácil de mantener conforme ChurrOS continúe creciendo.