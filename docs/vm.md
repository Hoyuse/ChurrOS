# VM (Virtual Machine)

Este documento describe la máquina virtual de desarrollo usada para probar ChurrOS sin instalarlo en hardware real.

ChurrOS utiliza **QEMU** con KVM para ejecutar la ISO en una VM con UEFI. El objetivo es ofrecer un entorno de pruebas reproducible y rápido.

---

# Overview

El comando principal es:

```bash
./churros run
```

Este script vive en `scripts/cli/run.sh` y se encarga de:

1. Buscar la última ISO en `out/`.
2. Si no existe, ejecutar `./churros build` automáticamente.
3. Crear un disco persistente si es la primera vez.
4. Lanzar QEMU con la configuración adecuada.

Los archivos de la VM se guardan en `vm/`:

```text
vm/
├── ChurrOS.qcow2    # Disco persistente (64 GB)
└── OVMF_VARS.fd     # Variables UEFI (no se commitea)
```

Estos archivos están listados en `.gitignore` (junto con `*.qcow2`) para evitar que se suban al repositorio. Cada desarrollador genera los suyos localmente.

---

# QEMU Configuration

El script `scripts/cli/run.sh` lanza QEMU con los siguientes parámetros:

| Parámetro | Valor | Significado |
|-----------|-------|-------------|
| `-enable-kvm` | — | Aceleración por hardware (KVM) |
| `-machine` | `q35,accel=kvm` | Chipset moderno con virtualización |
| `-cpu` | `host` | Pasa todas las instrucciones del CPU al guest |
| `-smp` | `4` | 4 cores |
| `-m` | `4096` | 4 GB de RAM |
| `-drive if=pflash,...readonly=on,file=...` | `/usr/share/edk2/x64/OVMF_CODE.4m.fd` | Firmware UEFI (código) |
| `-drive if=pflash,file=...` | `vm/OVMF_VARS.fd` | Variables UEFI (modificable) |
| `-drive file=...format=qcow2,if=virtio` | `vm/ChurrOS.qcow2` | Disco persistente |
| `-cdrom` | la última ISO en `out/` | CD Live |
| `-boot` | `order=d` | Arranca desde disco |

## UEFI

Se usa OVMF (Open Virtual Machine Firmware) para que la VM arranque en modo UEFI, igual que la mayoría de PCs modernos. Esto es importante porque la ISO de ChurrOS incluye entradas systemd-boot que solo funcionan en UEFI.

Los dos archivos de firmware:

- `OVMF_CODE.4m.fd` (de `/usr/share/edk2/x64/`) — código del firmware, solo lectura.
- `vm/OVMF_VARS.fd` — variables UEFI (boot order, secure boot, etc). Se copia del CODE en el primer arranque y se modifica por el firmware en runtime.

## Disk

El disco persistente es un `qcow2` de 64 GB. Se crea con `qemu-img create -f qcow2` la primera vez que se ejecuta `./churros run`. Permite:

- Instalar paquetes en la VM sin perderlos al apagar.
- Probar el instalador de ChurrOS.
- Guardar configuraciones de prueba.

Para resetear la VM, basta con borrar `vm/ChurrOS.qcow2` y `vm/OVMF_VARS.fd`. La próxima ejecución los regenerará.

## ISO Detection

```bash
ISO=$(find out -name "*.iso" | head -n1)
```

El script toma la primera ISO que encuentra en `out/` (orden alfabético). Si la ISO tiene fecha en el nombre (`ChurrOS-2026.07-x86_64.iso`), la última será la más reciente.

---

# Requirements

Para que `./churros run` funcione, el host necesita:

| Paquete | Obligatorio | Notas |
|---------|-------------|-------|
| `qemu-full` | ✅ | QEMU con todos los backends |
| `edk2-ovmf` | ✅ | Firmware UEFI (`/usr/share/edk2/x64/OVMF_CODE.4m.fd`) |
| KVM habilitado en el kernel | ✅ | `lsmod \| grep kvm` debe mostrar `kvm_intel` o `kvm_amd` |
| `/dev/kvm` accesible | ✅ | El usuario debe pertenecer al grupo `kvm` |

Ver `docs/getting-started.md` para las instrucciones de instalación.

---

# Troubleshooting

## "Could not access KVM kernel module"

KVM no está disponible. Soluciones:

```bash
# Cargar el módulo
sudo modprobe kvm_intel   # o kvm_amd para AMD

# Comprobar que el dispositivo existe
ls -la /dev/kvm

# Añadir tu usuario al grupo kvm
sudo usermod -aG kvm $USER
# Cierra sesión y vuelve a entrar
```

## "OVMF_CODE.4m.fd not found"

El paquete `edk2-ovmf` no está instalado:

```bash
sudo pacman -S edk2-ovmf
```

## "No ISO found"

No hay ISOs en `out/`. Ejecuta primero:

```bash
./churros build
```

## La VM arranca pero no muestra nada

Si la pantalla queda en negro, prueba a quitar `-cpu host` y usar `-cpu kvm64` o un modelo genérico. Algunos CPUs exponen instrucciones que el firmware de OVMF no soporta bien.

También puedes añadir `-vga qxl` para forzar una VGA compatible.

## Quiero reiniciar la VM desde cero

```bash
./churros clean
rm -rf vm
./churros run
```

Esto borra la ISO, el workdir de ArchISO y el disco persistente. La próxima ejecución regenera todo.

---

# Alternatives

Si no puedes usar KVM, la VM funcionará pero mucho más lenta. En ese caso:

- Reduce `-smp` a 2 cores.
- Reduce `-m` a 2048 MB.
- Usa `-cpu kvm64` en vez de `-cpu host`.
- No actives `-enable-kvm`.

Para una alternativa con interfaz gráfica, puedes usar **virt-manager** con la misma ISO. Solo tienes que crear una VM nueva, asignar 4GB RAM, 4 cores, y montar la ISO como CD-ROM.

---

# Future Work

- Script `./churros vm create` para generar la VM desde cero con parámetros personalizados.
- Snapshot automático antes de cada cambio importante.
- Compartir portapapeles entre host y guest (requiere `spice-vdagent` y `-spice port=...`).
- Red NAT para que la VM tenga acceso a internet.
- Carpetas compartidas vía virtio-9p.
- Script `./churros vm reset` para borrar solo la VM sin tocar `out/`.
