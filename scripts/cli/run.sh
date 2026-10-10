#!/usr/bin/env bash

set -e

VM_DIR="vm"
# Sin --arch se usa la arquitectura del equipo, igual que ./churros build.
TARGET_ARCH=""
QEMU_BIN="qemu-system-aarch64"
DISK="$VM_DIR/ChurrOS-arm64.qcow2"
VARS="$VM_DIR/OVMF_VARS_arm64.fd"
# i686 puede arrancar por BIOS (SeaBIOS) sin OVMF; el resto lo exige.
OVMF_REQUIRED=true

for ((arg_index = 1; arg_index <= $#; arg_index++)); do
    arg="${!arg_index}"
    case "$arg" in
        --arch=*) TARGET_ARCH="${arg#*=}" ;;
        --arch)
            arg_index=$((arg_index + 1))
            TARGET_ARCH="${!arg_index}"
            ;;
    esac
done
[ -n "$TARGET_ARCH" ] || TARGET_ARCH="$(uname -m)"
case "$TARGET_ARCH" in
    arm64|aarch64)
        TARGET_ARCH="aarch64"
        ;;
    x86_64|amd64)
        TARGET_ARCH="x86_64"
        QEMU_BIN="qemu-system-x86_64"
        DISK="$VM_DIR/ChurrOS.qcow2"
        VARS="$VM_DIR/OVMF_VARS.fd"
        ;;
    i386|i486|i586|i686)
        TARGET_ARCH="i686"
        QEMU_BIN="qemu-system-i386"
        DISK="$VM_DIR/ChurrOS-i686.qcow2"
        VARS="$VM_DIR/OVMF_VARS_ia32.fd"
        ;;
    *)
        echo "Error: unsupported architecture '$TARGET_ARCH' (use --arch x86_64, --arch arm64 or --arch i686)." >&2
        exit 1
        ;;
esac

if [ "$TARGET_ARCH" = "aarch64" ]; then
    OVMF_CODE_CANDIDATES=(
        /usr/share/edk2/aarch64/QEMU_EFI.fd
        /usr/share/edk2/aarch64/QEMU_EFI-pflash.raw
        /usr/share/qemu-efi-aarch64/QEMU_EFI.fd
        /usr/share/AAVMF/AAVMF_CODE.fd
    )
    OVMF_VARS_CANDIDATES=(
        /usr/share/edk2/aarch64/QEMU_VARS.fd
        /usr/share/AAVMF/AAVMF_VARS.fd
    )
elif [ "$TARGET_ARCH" = "i686" ]; then
    # Firmware IA32 (32-bit UEFI). Si no está, el arranque por BIOS (SeaBIOS)
    # sigue funcionando: OVMF_REQUIRED=false.
    OVMF_REQUIRED=false
    OVMF_CODE_CANDIDATES=(
        /usr/share/OVMF/OVMF32_CODE_4M.fd
        /usr/share/OVMF/OVMF32_CODE.fd
        /usr/share/OVMF32/OVMF32_CODE_4M.fd
        /usr/share/edk2/ia32/OVMF_CODE.fd
        /usr/share/ovmf/OVMF32_CODE_4M.fd
    )
    OVMF_VARS_CANDIDATES=(
        /usr/share/OVMF/OVMF32_VARS_4M.fd
        /usr/share/OVMF/OVMF32_VARS.fd
        /usr/share/OVMF32/OVMF32_VARS_4M.fd
        /usr/share/edk2/ia32/OVMF_VARS.fd
        /usr/share/ovmf/OVMF32_VARS_4M.fd
    )
else
    OVMF_CODE_CANDIDATES=(
        /usr/share/edk2/x64/OVMF_CODE.4m.fd
        /usr/share/edk2-ovmf/x64/OVMF_CODE.4m.fd
        /usr/share/edk2/x64/OVMF_CODE.fd
        /usr/share/OVMF/OVMF_CODE.fd
        /usr/share/ovmf/x64/OVMF_CODE.4m.fd
        /usr/share/ovmf/OVMF_CODE.fd
    )
    OVMF_VARS_CANDIDATES=(
        /usr/share/edk2/x64/OVMF_VARS.4m.fd
        /usr/share/edk2-ovmf/x64/OVMF_VARS.4m.fd
        /usr/share/edk2/x64/OVMF_VARS.fd
        /usr/share/OVMF/OVMF_VARS.fd
        /usr/share/ovmf/x64/OVMF_VARS.4m.fd
        /usr/share/ovmf/OVMF_VARS.fd
    )
fi

# Search OVMF firmware files in standard locations
OVMF_CODE=""
for path in "${OVMF_CODE_CANDIDATES[@]}"; do
    if [ -f "$path" ]; then
        OVMF_CODE="$path"
        break
    fi
done
if [ -z "$OVMF_CODE" ]; then
    if [ "$TARGET_ARCH" = "i686" ]; then
        OVMF_CODE=$(find /usr/share/edk2 /usr/share/ovmf /usr/share/OVMF /usr/share/OVMF32 /usr/share/qemu -type f \( -iname 'OVMF32_CODE*.fd' -o -iname 'OVMF_CODE*ia32*.fd' \) ! -name '*secboot*' -print -quit 2>/dev/null || true)
    else
        OVMF_CODE=$(find /usr/share/edk2 /usr/share/ovmf /usr/share/OVMF /usr/share/AAVMF /usr/share/qemu /usr/share/qemu-efi-aarch64 -type f \( -iname 'OVMF_CODE*.4m.fd' -o -iname 'OVMF_CODE*.fd' -o -iname 'QEMU_EFI*.fd' -o -iname 'AAVMF_CODE*.fd' \) ! -name '*secboot*' -print -quit 2>/dev/null || true)
    fi
fi

OVMF_VARS=""
for path in "${OVMF_VARS_CANDIDATES[@]}"; do
    if [ -f "$path" ]; then
        OVMF_VARS="$path"
        break
    fi
done
if [ -z "$OVMF_VARS" ]; then
    if [ "$TARGET_ARCH" = "i686" ]; then
        OVMF_VARS=$(find /usr/share/edk2 /usr/share/ovmf /usr/share/OVMF /usr/share/OVMF32 /usr/share/qemu -type f \( -iname 'OVMF32_VARS*.fd' -o -iname 'OVMF_VARS*ia32*.fd' \) -print -quit 2>/dev/null || true)
    else
        OVMF_VARS=$(find /usr/share/edk2 /usr/share/ovmf /usr/share/OVMF /usr/share/AAVMF /usr/share/qemu -type f \( -iname 'OVMF_VARS*.4m.fd' -o -iname 'OVMF_VARS*.fd' -o -iname 'QEMU_VARS*.fd' -o -iname 'AAVMF_VARS*.fd' \) -print -quit 2>/dev/null || true)
    fi
fi

# Solo la ISO de la arquitectura pedida: si en out/ conviven varias (x86_64,
# i686, aarch64), head -n1 arrancaría la equivocada con este QEMU.
ISO=$(find out \( -name "*-${TARGET_ARCH}.iso" -o -name "*-${TARGET_ARCH}-*.iso" \) -print 2>/dev/null | sort | tail -n1)

FORCE_NOKVM=false
FORCE_FRESH=false
FORCE_CLEAN=false
for arg in "$@"; do
    case "$arg" in
        --nokvm) FORCE_NOKVM=true ;;
        --fresh) FORCE_FRESH=true ;;
        --clean) FORCE_CLEAN=true ;;
    esac
done

# If OVMF firmware files couldn't be found, exit 1 (salvo i686, que arranca
# por BIOS sin OVMF).
if [ -z "$OVMF_CODE" ] || [ -z "$OVMF_VARS" ]; then
    if [ "$OVMF_REQUIRED" = true ]; then
        echo "Error: OVMF firmware not found, do you have QEMU installed?"
        echo "Please configure the OVMF firmware paths manually otherwaise."
        exit 1
    fi
    echo "OVMF IA32 firmware not found: arrancando por BIOS (SeaBIOS)."
    OVMF_CODE=""
    OVMF_VARS=""
else
    echo "OVMF firmware found:"
    echo "  OVMF_CODE: $OVMF_CODE"
    echo "  OVMF_VARS: $OVMF_VARS"
fi

# If no ISO was found, prompt the user to build ChurrOS
if [ -z "$ISO" ]; then
    echo "No ISO found."
    read -r -p "Do you want to build ChurrOS? [y/N] " answer

    case "$answer" in
        [yY]|[yY][eE][sS])
            echo "Building..."
            ./churros build "$@"

            ISO=$(find out \( -name "*-${TARGET_ARCH}.iso" -o -name "*-${TARGET_ARCH}-*.iso" \) -print 2>/dev/null | sort | tail -n1)

            if [ -z "$ISO" ]; then
                echo "Error: Build completed, but no ISO was found."
                exit 1
            fi

            echo "ISO found: $ISO"
            ;;
        *)
            echo "Please specify the path to the ISO file."
            exit 1
            ;;
    esac
fi

mkdir -p "$VM_DIR"

if [ "$FORCE_CLEAN" = true ]; then
    echo "Full clean (--clean): removing disk and EFI vars..."
    rm -f "$DISK" "$VARS"
fi

if [ "$FORCE_FRESH" = true ] && [ -f "$VARS" ]; then
    echo "Resetting EFI vars (--fresh)..."
    rm -f "$VARS"
fi

if [ ! -f "$DISK" ]; then
    echo
    echo "Creating development virtual machine..."
    echo

    qemu-img create -f qcow2 "$DISK" 64G
fi

# If the OVMF vars file doesn't exist, copy the default one to the VM directory.
# En i686 puede no haber OVMF (arranque BIOS): en ese caso no hay nada que copiar.
if [ -n "$OVMF_VARS" ] && [ ! -f "$VARS" ]; then
    cp "$OVMF_VARS" "$VARS"
fi

echo
echo "Launching ChurrOS Development VM..."
echo

KVM_ARGS=""
GPU_ARGS=""
CPU_ARGS=""
MACHINE_ARGS=""
AUDIO_ARGS=""

if [ "$TARGET_ARCH" = "aarch64" ]; then
    echo "  ARM64 QEMU: TCG software emulation"
    KVM_ARGS="-cpu cortex-a72"
    CPU_ARGS="-smp 4"
    MACHINE_ARGS="-machine virt"
    GPU_ARGS="-device virtio-gpu-gl-pci -display gtk,gl=on,show-cursor=on"
elif [ "$TARGET_ARCH" = "i686" ]; then
    if [ "$FORCE_NOKVM" = false ] && [ -e /dev/kvm ] && [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
        echo "  KVM acceleration: enabled (i686)"
        KVM_ARGS="-cpu host"
        CPU_ARGS="-smp 4"
        MACHINE_ARGS="-machine q35,accel=kvm"
    else
        echo "  KVM acceleration: not available (using software emulation)"
        KVM_ARGS="-cpu qemu32"
        CPU_ARGS="-smp 2"
        MACHINE_ARGS="-machine q35"
    fi
elif [ "$FORCE_NOKVM" = false ] && [ -e /dev/kvm ] && [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
    echo "  KVM acceleration: enabled"
    KVM_ARGS="-cpu host"
    CPU_ARGS="-smp 4"
    MACHINE_ARGS="-machine q35,accel=kvm"
else
    echo "  KVM acceleration: not available (using software emulation)"
    KVM_ARGS=""
    CPU_ARGS="-smp 2"
    MACHINE_ARGS="-machine q35"
fi

# niri requires hardware-accelerated 3D (OpenGL via virgl).
# Always attempt virtio-gpu-gl with GL; fall back to virtio-gpu (no GL) only if
# the host lacks /dev/dri entirely — in that case niri will try llvmpipe.
if [ -e /dev/dri ]; then
    [ "$TARGET_ARCH" != "aarch64" ] && GPU_ARGS="-device virtio-vga-gl -display gtk,gl=on,show-cursor=on"
    echo "  GPU: virtio-vga-gl + virgl (3D)"
else
    [ "$TARGET_ARCH" != "aarch64" ] && GPU_ARGS="-device virtio-gpu -display gtk,gl=off,show-cursor=on"
    echo "  GPU: virtio-gpu (no 3D — niri may fall back to software rendering)"
fi

if [ "$TARGET_ARCH" != "aarch64" ]; then
    AUDIO_ARGS="-device intel-hda -device hda-duplex"
fi

# En i686 sin OVMF se arranca por BIOS (SeaBIOS), sin pflash.
FIRMWARE_ARGS=()
if [ -n "$OVMF_CODE" ] && [ -n "$OVMF_VARS" ]; then
    FIRMWARE_ARGS=(
        -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE"
        -drive if=pflash,format=raw,file="$VARS"
    )
fi

"$QEMU_BIN" \
    $MACHINE_ARGS \
    $KVM_ARGS \
    $CPU_ARGS \
    -m 4096 \
    $GPU_ARGS \
    -device qemu-xhci \
    -device usb-tablet \
    $AUDIO_ARGS \
    -device virtio-serial-pci \
    -chardev qemu-vdagent,id=vdagent,name=vdagent,clipboard=on \
    -device virtserialport,chardev=vdagent,name=com.redhat.spice.0 \
    ${FIRMWARE_ARGS[@]+"${FIRMWARE_ARGS[@]}"} \
    -drive file="$DISK",format=qcow2,if=virtio \
    -cdrom "$ISO" \
    -boot order=c \
    -serial file:vm_serial.log
