# Release

Este documento describe el proceso oficial para generar una nueva versión de ChurrOS.

El objetivo es garantizar que todas las versiones publicadas sean reproducibles, estables y correctamente documentadas.

---

# Objetivo

Antes de publicar una nueva versión se debe comprobar que:

- La ISO se construye correctamente.
- El sistema Live inicia sin errores.
- El branding es consistente.
- La documentación está actualizada.
- No existen cambios sin confirmar.

---

# Flujo de publicación

El proceso recomendado es el siguiente:

```
Actualizar código

↓

Actualizar documentación

↓

Compilar la ISO

↓

Probar en máquina virtual

↓

Crear versión

↓

Crear Release

↓

Publicar
```

---

# 1. Actualizar el repositorio

Antes de comenzar verifica que el repositorio esté actualizado.

```bash
git pull
```

---

# 2. Verificar el estado

Comprueba que no existan cambios pendientes.

```bash
git status
```

El resultado esperado es:

```
working tree clean
```

---

# 3. Compilar

Construye una nueva ISO.

```bash
./churros build
```

La compilación debe finalizar sin errores.

---

# 4. Probar

Ejecuta la ISO.

```bash
./churros run
```

Verifica:

- Arranque correcto.
- Inicio de sesión.
- Branding.
- Fastfetch.
- Servicios principales.
- Ausencia de errores críticos.

---

# 5. Actualizar la versión

El número de versión vive en `VERSION` (lo muestran `./churros version` y `./churros info`). Debe coincidir con el tag de GitHub.

La versión actual es **0.6** (tag `v0.6`). El esquema previsto es Semantic Versioning (`MAJOR.MINOR.PATCH`); algunos tags históricos omiten el parche (`v0.6`).

Ejemplo:

```
0.6

↓

0.7.0
```

---

# 6. Crear el commit

Registrar todos los cambios.

```bash
git add .
git commit -m "release: preparar versión 0.7.0"
```

---

# 7. Crear un tag

Marcar la versión publicada.

```bash
git tag v0.7.0
```

Enviar el tag.

```bash
git push origin v0.7.0
```

---

# 8. Publicar

Subir los cambios al repositorio.

```bash
git push
```

Posteriormente publicar la ISO mediante GitHub Releases.

---

# Archivos de la versión

Cada versión oficial debería incluir:

- Imagen ISO.
- SHA256SUMS.
- Notas de la versión.
- Número de versión.
- Fecha de publicación.

---

# Versionado

ChurrOS seguirá el esquema Semantic Versioning.

```
MAJOR.MINOR.PATCH
```

Ejemplos:

```
0.4.0

0.6

0.7.0

1.0.0
```

---

# Checklist

Antes de publicar verificar:

- [ ] La ISO compila correctamente.
- [ ] La máquina virtual inicia.
- [ ] El branding es correcto.
- [ ] La documentación está actualizada.
- [ ] No existen errores conocidos críticos.
- [ ] El repositorio está limpio.
- [ ] La versión fue etiquetada correctamente.

---

# Futuro

En futuras versiones este proceso será automatizado mediante la CLI de ChurrOS.

El objetivo es que un único comando permita generar una nueva versión oficial de la distribución, incluyendo la compilación, verificación, creación de checksums y preparación del Release.