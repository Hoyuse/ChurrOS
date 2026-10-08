# Instalador Gráfico — Calamares para ChurrOS

Este directorio contiene toda la configuración, personalización, branding y scripts de despliegue del instalador gráfico de ChurrOS, basado en **Calamares 3.4**.

---

## 📁 Estructura del Directorio

```text
installer/
├── apply-calamares.sh         # Despliega módulos, branding y reglas polkit al airootfs
├── calamares/
│   ├── settings.conf          # Secuencia oficial de ejecución y módulos
│   ├── branding/churros/      # Identidad visual, slideshow QML y estilos QSS
│   ├── modules/               # Configuraciones .conf y .yaml de cada módulo
│   └── preview/               # Overlay aislado para previsualizar en el host
└── patches/                   # Parches aplicados sobre Calamares al compilar
```

---

## ⚙️ Secuencia de Instalación (`settings.conf`)

La secuencia separa las pantallas de `show` de las acciones de `exec`. La opción `netinstall` aparece después de los datos de usuario y antes del resumen; durante `exec`, instala los paquetes seleccionados tras configurar la red. Luego `packages` retira paquetes del Live que no deben quedar en el destino.

```text
1. shellprocess@boot-nocow      -> Evita que GRUB use zstd en /boot.
2. unpackfs                     -> Extrae la imagen squashfs del Live al destino.
3. shellprocess@pacman-init     -> Inicializa el keyring de pacman.
4. shellprocess@fix-boot        -> Regenera presets de mkinitcpio y módulos del kernel.
5. users / displaymanager / networkcfg / hwclock -> Configuración del sistema instalado.
6. netinstall                   -> Instala las opciones de paquetes elegidas por el usuario.
7. packages                     -> Retira paquetes innecesarios heredados del Live.
8. services-systemd / grubcfg / bootloader -> Configura servicios y el arranque.
9. shellprocess@grub-theme      -> Aplica el tema GRUB y el hook de lectura Btrfs.
10. shellprocess@post-install   -> Limpia configuración temporal del instalador y rastros del usuario Live.
11. umount                      -> Desmonta las particiones instaladas.
```

---

## 🔒 Regla Polkit (`49-calamares.rules`)

`apply-calamares.sh` instala automáticamente una regla en `/etc/polkit-1/rules.d/49-calamares.rules` que autoriza al usuario de la sesión Live (`churros`) a ejecutar Calamares como superusuario sin solicitud interactiva de contraseña:

```javascript
polkit.addRule(function(action, subject) {
    if ((action.id == "org.freedesktop.policykit.exec" &&
         action.lookup("program") == "/usr/bin/calamares") &&
        subject.isInGroup("wheel")) {
        return polkit.Result.YES;
    }
});
```

---

## 🖼️ Vista Previa en el Host

Puedes previsualizar el diseño del instalador y el carrusel de diapositivas en el sistema anfitrión sin riesgo de modificar discos reales:

```bash
./churros apps calamares
```

Este comando utiliza el overlay en `calamares/preview/`:
- No toca `/etc/calamares` del host.
- Omite el módulo de particionado real para proteger los discos.
- Simula la fase de instalación con un temporizador para apreciar el slideshow.

---

## 📖 Documentación Relacionada

- [Roadmap de la Fase 4: Instalador](../docs/roadmap.md)
- [Documentación del Sistema de Arranque](../docs/boot.md)
- [Documentación de Snapshots y Rollback Btrfs](../docs/rollback.md)
