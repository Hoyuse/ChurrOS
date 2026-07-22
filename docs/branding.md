# Branding

El sistema de branding de ChurrOS define la identidad visual y textual de la distribución.

Su objetivo es reemplazar completamente la identidad de Arch Linux dentro de la imagen Live y, en el futuro, del sistema instalado.

Toda modificación relacionada con la apariencia o identidad de ChurrOS debe realizarse desde este sistema.

---

# Objetivo

El branding permite que ChurrOS tenga una identidad propia.

Esto incluye:

- Nombre de la distribución.
- Mensajes de bienvenida.
- Información del sistema.
- Logos.
- Wallpapers.
- Fastfetch.
- Splash Screen.
- Bootloader.
- Instalador.

El objetivo es que el usuario nunca vea referencias innecesarias a Arch Linux durante el uso normal de ChurrOS.

---

# Estructura

Todo el branding se encuentra dentro del directorio:

```text
branding/
```

La estructura recomendada es:

```text
branding
├── files/
├── logos/
├── wallpapers/
├── fastfetch/
├── plymouth/
└── installer/
```

---

# files/

Contiene archivos de configuración que reemplazan los originales del sistema.

Ejemplo:

```text
branding/files

issue
motd
hostname
os-release
```

Estos archivos son copiados automáticamente durante la construcción de la ISO.

---

# issue

Archivo mostrado antes del inicio de sesión.

Actualmente muestra el banner oficial de ChurrOS.

Ejemplo:

```text
██████╗██╗  ██╗██╗   ██╗██████╗ ██████╗  ██████╗ ███████╗
██╔════╝██║  ██║██║   ██║██╔══██╗██╔══██╗██╔═══██╗██╔════╝
██║     ███████║██║   ██║██████╔╝██████╔╝██║   ██║███████╗
██║     ██╔══██║██║   ██║██╔══██╗██╔══██╗██║   ██║╚════██║
╚██████╗██║  ██║╚██████╔╝██║  ██║██║  ██║╚██████╔╝███████║
 ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝

Welcome to ChurrOS
```

---

# motd

Mensaje mostrado inmediatamente después del inicio de sesión.

Su propósito es dar la bienvenida al usuario y proporcionar información útil sobre la sesión Live.

Puede incluir:

- versión
- enlaces
- comandos útiles
- estado del sistema

---

# hostname

Define el nombre del equipo durante la sesión Live.

Ejemplo:

```text
churros
```

---

# os-release

Contiene la información oficial de la distribución.

Programas como:

- fastfetch
- neofetch
- screenfetch
- systemd

obtienen información desde este archivo.

Debe contener la identidad oficial de ChurrOS.

---

# Logos

La carpeta:

```text
branding/logos/
```

almacenará los logotipos oficiales de la distribución.

Versiones recomendadas:

- SVG
- PNG
- Blanco
- Negro
- Monocromático

---

# Wallpapers

Todos los fondos oficiales deberán almacenarse en:

```text
branding/wallpapers/
```

Esto permitirá reutilizarlos durante la instalación del sistema.

---

# Fastfetch

Las configuraciones oficiales de Fastfetch vivirán en:

```text
branding/fastfetch/
```

Aquí se definirán:

- logo
- colores
- información mostrada
- formato

---

# Plymouth

En el futuro el tema oficial de Plymouth se almacenará en:

```text
branding/plymouth/
```

Permitirá personalizar completamente la animación de arranque.

---

# Instalador

Los recursos gráficos del instalador gráfico vivirán en:

```text
branding/installer/
```

Por ejemplo:

- iconos
- ilustraciones
- fondos
- logotipo

---

# Integración con la compilación

Durante la construcción de la ISO, el branding es copiado automáticamente al sistema Live.

El proceso se realiza mediante:

```text
customize_airootfs.sh
```

Este script reemplaza los archivos originales del sistema por los personalizados de ChurrOS.

---

# Buenas prácticas

Se recomienda mantener todos los recursos gráficos centralizados dentro de la carpeta branding.

No almacenar imágenes o logotipos dentro de otros directorios del proyecto.

Esto facilita el mantenimiento y futuras modificaciones de la identidad visual.

---

# Objetivo a largo plazo

El sistema de branding evolucionará para abarcar toda la experiencia del usuario.

En futuras versiones incluirá:

- Fastfetch personalizado.
- Plymouth.
- Tema de GRUB.
- Tema de SDDM.
- Wallpapers oficiales.
- Iconos.
- Cursor.
- Sonidos del sistema.
- Instalador gráfico.

El objetivo final es que toda la experiencia visual de ChurrOS sea consistente desde el arranque hasta el escritorio.