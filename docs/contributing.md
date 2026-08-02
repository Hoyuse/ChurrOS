# Contributing

¡Gracias por tu interés en contribuir a ChurrOS!

Actualmente el proyecto se encuentra en una etapa temprana de desarrollo y toda ayuda es bienvenida.

El objetivo de esta guía es mantener un flujo de trabajo organizado y consistente para todos los colaboradores.

---

# Antes de comenzar

Antes de realizar cualquier cambio se recomienda:

- Leer toda la documentación ubicada en `docs/`.
- Comprender la estructura del proyecto.
- Configurar correctamente el entorno de desarrollo.
- Generar una ISO de prueba utilizando la CLI.

Consulta `getting-started.md` si es tu primera vez colaborando.

---

# Flujo de trabajo

El flujo de desarrollo recomendado es el siguiente:

```
Fork (si aplica)

↓

Crear una rama

↓

Realizar cambios

↓

Compilar

↓

Probar

↓

Documentar

↓

Commit

↓

Push

↓

Pull Request
```

---

# Crear una rama

Cada nueva funcionalidad debe desarrollarse en una rama independiente.

Ejemplos:

```bash
git checkout -b feature/fastfetch
```

```bash
git checkout -b feature/installer
```

```bash
git checkout -b fix/build-system
```

Evita trabajar directamente sobre la rama principal cuando el proyecto tenga más colaboradores.

---

# Antes de hacer un commit

Verifica siempre que:

- La ISO compila correctamente.
- El sistema inicia sin errores.
- La documentación está actualizada.
- No existen archivos temporales innecesarios.

Ejecuta:

```bash
./churros build
```

y posteriormente:

```bash
./churros run
```

---

# Convención para commits

Se recomienda utilizar mensajes descriptivos.

Ejemplos:

```text
feat: add Fastfetch branding

fix: correct build script

docs: update installation guide

refactor: simplify CLI

style: improve terminal banner
```

Evita mensajes como:

```
update

changes

fix

test
```

---

# Documentación

Toda funcionalidad nueva debe estar documentada.

Si agregas:

- un comando nuevo,
- una herramienta,
- una configuración,
- una característica,

actualiza el documento correspondiente dentro de `docs/`.

---

# Organización

Respeta la estructura del proyecto.

Ejemplo:

```
branding/
```

solo contiene recursos relacionados con la identidad visual.

```
scripts/
```

solo contiene scripts.

```
docs/
```

solo contiene documentación.

Mantener una estructura limpia facilita el mantenimiento del proyecto.

---

# Código

Al escribir código intenta mantener estas reglas:

- Código legible.
- Nombres descriptivos.
- Evitar duplicación.
- Evitar scripts innecesariamente complejos.
- Priorizar la simplicidad.

---

# Reportar errores

Si encuentras un problema, intenta incluir:

- Descripción del error.
- Pasos para reproducirlo.
- Resultado esperado.
- Resultado obtenido.
- Capturas de pantalla (si aplica).
- Registros relevantes.

Esto facilita encontrar la causa del problema.

---

# Sugerencias

Las nuevas ideas son bienvenidas.

Antes de implementar una característica grande, se recomienda abrir primero un Issue para discutirla.

De esta manera se evita desarrollar funcionalidades que puedan cambiar posteriormente.

---

# Filosofía

La prioridad de ChurrOS no es añadir la mayor cantidad posible de funciones.

La prioridad es construir una distribución estable, organizada y fácil de mantener.

Cada contribución debe seguir esta filosofía.

---

# Gracias

Cada contribución ayuda a mejorar ChurrOS.

Gracias por dedicar tu tiempo a este proyecto.