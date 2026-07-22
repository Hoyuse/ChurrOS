# ChurrOS Architecture

Este documento describe la arquitectura general del proyecto.

Su objetivo es mantener una estructura limpia, organizada y fácil de mantener a medida que ChurrOS evolucione.

---

# Organización

Cada directorio tiene una única responsabilidad.

```
archiso/
```

Contiene el perfil de ArchISO utilizado para generar la imagen.

---

```
branding/
```

Contiene toda la identidad visual del proyecto.

- Logos
- Wallpapers
- Iconos
- Colores
- Tipografía

---

```
docs/
```

Documentación oficial.

---

```
scripts/
```

Scripts auxiliares.

Nunca deberán contener recursos del sistema.

---

```
installer/
```

Código del futuro instalador gráfico.

---

```
configs/
```

Configuraciones oficiales del sistema.

---

# Filosofía

Cada componente del proyecto debe ser independiente.

Esto facilita:

- mantenimiento
- pruebas
- reutilización
- colaboración

---

# Evolución

La arquitectura podrá cambiar cuando sea necesario.

Sin embargo, cualquier modificación importante deberá mantener la simplicidad del proyecto.

---

# Principios

- Una responsabilidad por carpeta.
- Código reutilizable.
- Documentación primero.
- Automatización siempre que sea posible.
- Evitar duplicación.