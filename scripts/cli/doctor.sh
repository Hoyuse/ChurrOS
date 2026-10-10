#!/usr/bin/env bash
set -euo pipefail

# shellcheck source=scripts/lib/host.sh
source "$(dirname "$0")/../lib/host.sh"

AUTO_INSTALL=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --install|-i|--yes|-y)
            AUTO_INSTALL=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

FAMILY="$(churros_host_family)"

echo "Running diagnostics..."
echo "Host: $(churros_host_name) ($FAMILY)"
echo

missing=0
missing_pkgs=()

# check <comando> <paquete Arch> <paquete Debian/Ubuntu> <paquete Fedora>
check() {
    local cmd="$1"
    local pkg
    case "$FAMILY" in
        arch)   pkg="${2:-$cmd}" ;;
        debian) pkg="${3:-${2:-$cmd}}" ;;
        fedora) pkg="${4:-${2:-$cmd}}" ;;
        *)      pkg="" ;;
    esac
    if command -v "$cmd" >/dev/null 2>&1; then
        echo "✓ $cmd"
    else
        if [ -n "$pkg" ]; then
            echo "✗ $cmd — missing (install package '$pkg')"
            missing_pkgs+=("$pkg")
        else
            echo "✗ $cmd — missing"
        fi
        missing=$((missing + 1))
    fi
}

#     comando              Arch                Debian/Ubuntu      Fedora
check git
check qemu-system-x86_64  qemu-desktop        qemu-system-x86    qemu-system-x86
check qemu-system-i386    qemu-desktop        qemu-system-x86    qemu-system-x86
check qemu-system-aarch64 qemu-system-aarch64 qemu-system-arm    qemu-system-aarch64
check qemu-img            qemu-img            qemu-utils         qemu-img
check sudo
check python3             python              python3            python3
check node                nodejs              nodejs             nodejs
check shellcheck          shellcheck          shellcheck         ShellCheck
check msgfmt              gettext             gettext            gettext
check zstd

# El workspace usa edition 2024: rustc >= 1.85. Debian/Ubuntu traen versiones
# más viejas en apt; ./churros rust compila en el contenedor de todos modos.
if [ "$FAMILY" = arch ]; then
    check rustc rust
    check cargo rust
    check pkg-config pkgconf
else
    check rustc rust rustup rust
    check cargo rust rustup cargo
fi
if command -v rustc >/dev/null 2>&1; then
    rustc_version=$(rustc --version | awk '{print $2}')
    if [ "$(printf '%s\n%s\n' 1.85 "$rustc_version" | sort -V | head -n1)" != 1.85 ]; then
        echo "✗ rustc $rustc_version — demasiado viejo para edition 2024 (>= 1.85)"
        echo "  Usa rustup, o compila las apps en el contenedor: ./churros rust"
        missing=$((missing + 1))
    fi
fi

# mkarchiso, pacstrap y makepkg solo existen en Arch. Fuera de Arch la ISO se
# construye en el contenedor (Containerfile) y lo que hace falta es el motor.
echo
if [ "$FAMILY" = arch ]; then
    echo "Build de la ISO en el host:"
    check mkarchiso archiso
    check xorriso libisoburn
    check mksquashfs squashfs-tools
    check mcopy mtools
    check mmd mtools
    check mkfs.fat dosfstools
    check grub-mkstandalone grub
    check mkinitcpio
    if command -v podman >/dev/null 2>&1 || command -v docker >/dev/null 2>&1; then
        echo "✓ podman/docker (./churros build --container y ./churros rust disponibles)"
    else
        echo "! podman/docker — opcional: ./churros build --container y ./churros rust"
    fi
else
    echo "Build de la ISO: este host no es Arch Linux, así que mkarchiso y makepkg no"
    echo "están disponibles. Usa el contenedor Arch del proyecto:"
    echo "  ./churros build --container"
    if command -v podman >/dev/null 2>&1; then
        echo "✓ podman"
    elif command -v docker >/dev/null 2>&1; then
        echo "✓ docker"
    else
        check podman podman podman podman
    fi
fi

echo

# KVM hardware virtualization check
if [ -e /dev/kvm ]; then
    if [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
        echo "✓ /dev/kvm (hardware virtualization ready)"
    else
        echo "✗ /dev/kvm — permission denied for $USER"
        echo "  Fix: sudo usermod -aG kvm $USER (log out and back in)"
        missing=$((missing + 1))
    fi
else
    if [ -n "$(journalctl -k -b 0 -g "disabled by BIOS" --no-pager 2>/dev/null || true)" ]; then
        echo "✗ /dev/kvm — Virtualization (VT-x/AMD-V) is DISABLED in BIOS/UEFI"
        echo "  Fix: Reboot into BIOS/UEFI setup and enable 'Intel Virtualization Technology' (VT-x) or 'SVM'"
    else
        echo "! /dev/kvm — not available (QEMU will fall back to software emulation)"
    fi
fi

if [ "${#missing_pkgs[@]}" -gt 0 ]; then
    readarray -t unique_pkgs < <(printf '%s\n' "${missing_pkgs[@]}" | sort -u)

    install_cmd=()
    case "$FAMILY" in
        arch)   install_cmd=(sudo pacman -S --needed) ;;
        debian) install_cmd=(sudo apt-get install) ;;
        fedora) install_cmd=(sudo dnf install) ;;
    esac

    echo
    echo "Faltan paquetes necesarios: ${unique_pkgs[*]}"
    if [ "${#install_cmd[@]}" -gt 0 ]; then
        echo "Comando para instalar: ${install_cmd[*]} ${unique_pkgs[*]}"
        echo "(o ./install-deps.sh, que instala todo lo del proyecto para esta distro)"
    fi
    echo

    if [ "${#install_cmd[@]}" -gt 0 ] && command -v "${install_cmd[1]}" >/dev/null 2>&1; then
        do_install=false
        if [ "$AUTO_INSTALL" = true ]; then
            do_install=true
        elif [ -t 0 ]; then
            read -r -p "¿Deseas instalar los paquetes faltantes ahora con ${install_cmd[1]}? [S/n] " response
            response="${response:-s}"
            if [[ "$response" =~ ^[sSyY]$ ]]; then
                do_install=true
            fi
        fi

        if [ "$do_install" = true ]; then
            echo "Instalando paquetes faltantes..."
            if "${install_cmd[@]}" "${unique_pkgs[@]}"; then
                echo "✓ Paquetes instalados correctamente."
                missing=0
            else
                echo "✗ Error al instalar paquetes con ${install_cmd[1]}."
            fi
        fi
    fi
fi

echo
if [ "$missing" -ne 0 ]; then
    echo "Diagnostics found missing tools."
    exit 1
fi

echo "Diagnostics complete."
