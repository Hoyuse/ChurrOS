#!/usr/bin/env bash

set -e

VM_DIR="vm"
ARCH="x86_64"
DISK="$VM_DIR/ChurrOS.qcow2"
VARS="$VM_DIR/OVMF_VARS.fd"

case "${ARCH:-x86_64}" in
    x86_64|amd64)
        ARCH="x86_64"
        DISK="$VM_DIR/ChurrOS.qcow2"
        VARS="$VM_DIR/OVMF_VARS.fd"
        QEMU_BIN="qemu-system-x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        DISK="$VM_DIR/ChurrOS-arm64.qcow2"
        VARS="$VM_DIR/OVMF_VARS_arm64.fd"
        QEMU_BIN="qemu-system-aarch64"
        ;;
    *)
        echo "Error: unsupported arch '$ARCH' (supported: x86_64, aarch64)." >&2
        exit 1
        ;;
esac

# Search OVMF firmware files in standard locations
OVMF_CODE=""
for path in \
    /usr/share/edk2/x64/OVMF_CODE.4m.fd \
    /usr/share/edk2-ovmf/x64/OVMF_CODE.4m.fd \
    /usr/share/edk2/x64/OVMF_CODE.fd \
    /usr/share/OVMF/OVMF_CODE.fd \
    /usr/share/ovmf/x64/OVMF_CODE.4m.fd \
    /usr/share/ovmf/OVMF_CODE.fd \
    /usr/share/edk2/aarch64/QEMU_EFI.fd \
    /usr/share/edk2-ovmf/aarch64/QEMU_EFI.fd \
    /usr/share/OVMF/AARCH64/QEMU_EFI.fd; do
    if [ -f "$path" ]; then
        OVMF_CODE="$path"
        break
    fi
done
if [ -z "$OVMF_CODE" ]; then
    OVMF_CODE=$(find /usr/share/edk2 /usr/share/ovmf /usr/share/OVMF /usr/share/qemu -type f \( -iname 'OVMF_CODE*.4m.fd' -o -iname 'OVMF_CODE*.fd' -o -iname 'QEMU_EFI.fd' \) ! -name '*secboot*' -print -quit 2>/dev/null || true)
fi

OVMF_VARS=""
for path in \
    /usr/share/edk2/x64/OVMF_VARS.4m.fd \
    /usr/share/edk2-ovmf/x64/OVMF_VARS.4m.fd \
    /usr/share/edk2/x64/OVMF_VARS.fd \
    /usr/share/OVMF/OVMF_VARS.fd \
    /usr/share/ovmf/x64/OVMF_VARS.4m.fd \
    /usr/share/ovmf/OVMF_VARS.fd \
    /usr/share/edk2/aarch64/OVMF_VARS.fd \
    /usr/share/edk2-ovmf/aarch64/OVMF_VARS.fd \
    /usr/share/OVMF/AARCH64/OVMF_VARS.fd; do
    if [ -f "$path" ]; then
        OVMF_VARS="$path"
        break
    fi
done
if [ -z "$OVMF_VARS" ]; then
    OVMF_VARS=$(find /usr/share/edk2 /usr/share/ovmf /usr/share/OVMF /usr/share/qemu -type f \( -iname 'OVMF_VARS*.4m.fd' -o -iname 'OVMF_VARS*.fd' -o -iname 'OVMF_VARS.fd' \) -print -quit 2>/dev/null || true)
fi

ISO=$(find out -name "*.iso" 2>/dev/null | head -n1)

FORCE_NOKVM=false
FORCE_FRESH=false
FORCE_CLEAN=false
for arg in "$@"; do
    case "$arg" in
        --arch)
            shift
            if [ "$#" -gt 0 ]; then
                case "$1" in
                    x86_64|amd64) ARCH="x86_64" ;;
                    arm64|aarch64) ARCH="aarch64" ;;
                    *) echo "Unsupported arch: $1" >&2; exit 1 ;;
                esac
            fi
            ;;
        --arch=*)
            case "${arg#*=}" in
                x86_64|amd64) ARCH="x86_64" ;;
                arm64|aarch64) ARCH="aarch64" ;;
                *) echo "Unsupported arch: ${arg#*=}" >&2; exit 1 ;;
            esac
            ;;
        --nokvm) FORCE_NOKVM=true ;;
        --fresh) FORCE_FRESH=true ;;
        --clean) FORCE_CLEAN=true ;;
    esac
done

case "$ARCH" in
    x86_64)
        DISK="$VM_DIR/ChurrOS.qcow2"
        VARS="$VM_DIR/OVMF_VARS.fd"
        QEMU_BIN="qemu-system-x86_64"
        ;;
    aarch64)
        DISK="$VM_DIR/ChurrOS-arm64.qcow2"
        VARS="$VM_DIR/OVMF_VARS_arm64.fd"
        QEMU_BIN="qemu-system-aarch64"
        ;;
esac

# If OVMF firmware files couldn't be found, exit 1
if [ -z "$OVMF_CODE" ] || [ -z "$OVMF_VARS" ]; then
    echo "Error: OVMF firmware not found, do you have QEMU installed?"
    echo "Please configure the OVMF firmware paths manually otherwaise."
    exit 1
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

            ISO=$(find out -name "*.iso" -print -quit)

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
if [ ! -f "$VARS" ]; then
    cp "$OVMF_VARS" "$VARS"
fi

echo
echo "Launching ChurrOS Development VM..."
echo

KVM_ARGS=""
GPU_ARGS=""
CPU_ARGS=""

if [ "$ARCH" = "x86_64" ]; then
    if [ "$FORCE_NOKVM" = false ] && [ -e /dev/kvm ] && [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
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
        GPU_ARGS="-device virtio-vga-gl -display gtk,gl=on,show-cursor=on"
        echo "  GPU: virtio-vga-gl + virgl (3D)"
    else
        GPU_ARGS="-device virtio-gpu -display gtk,gl=off,show-cursor=on"
        echo "  GPU: virtio-gpu (no 3D — niri may fall back to software rendering)"
    fi

    "$QEMU_BIN" \
        $MACHINE_ARGS \
        $KVM_ARGS \
        $CPU_ARGS \
        -m 4096 \
        $GPU_ARGS \
        -device qemu-xhci \
        -device usb-tablet \
        -device intel-hda \
        -device hda-duplex \
        -device virtio-serial-pci \
        -chardev qemu-vdagent,id=vdagent,name=vdagent,clipboard=on \
        -device virtserialport,chardev=vdagent,name=com.redhat.spice.0 \
        -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
        -drive if=pflash,format=raw,file="$VARS" \
        -drive file="$DISK",format=qcow2,if=virtio \
        -cdrom "$ISO" \
        -boot order=c \
        -serial file:vm_serial.log
else
    echo "  ARM64 QEMU target: qemu-system-aarch64"
    "$QEMU_BIN" \
        -machine virt \
        -cpu cortex-a72 \
        -smp 4 \
        -m 4096 \
        -device virtio-gpu-gl-pci \
        -display gtk,gl=on \
        -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/aarch64/QEMU_EFI.fd \
        -drive if=pflash,format=raw,file="$VARS" \
        -drive file="$DISK",format=qcow2,if=virtio \
        -cdrom "$ISO" \
        -boot order=c \
        -serial file:vm_serial.log
fi
