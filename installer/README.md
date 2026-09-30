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

La instalación en disco sigue una secuencia validada y crítica que incluye 6 hooks `shellprocess`:

```text
1. shellprocess@boot-nocow      -> Aplica chattr +C en /boot para evitar zstd en GRUB.
2. unpackfs                     -> Extrae la imagen squashfs del Live al disco destino.
3. shellprocess@pacman-init     -> Inicializa y puebla el keyring de pacman en el sistema instalado.
4. shellprocess@fix-boot        -> Regenera presets de mkinitcpio y módulos del kernel.
5. shellprocess@churros-repo    -> Registra el repositorio local [churros] temporalmente.
6. netinstall / packages        -> Instala paquetes adicionales y extras AUR locales; sin Internet, Calamares omite este paso y conserva la instalación base de la ISO.
7. bootloader                   -> Instala GRUB (UEFI) / Syslinux (BIOS).
8. shellprocess@grub-theme      -> Aplica el tema GRUB centrado y hook de lectura Btrfs.
9. shellprocess@post-install    -> Elimina el repo local y limpia rastros del usuario Live.
10. umount                      -> Desmonta las particiones instaladas.
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
