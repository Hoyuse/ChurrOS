#!/usr/bin/env bash
set -euo pipefail

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

echo "Running diagnostics..."
echo

missing=0
missing_pkgs=()

check() {
    local cmd="$1"
    local pkg="${2:-$1}"
    if command -v "$cmd" >/dev/null 2>&1; then
        echo "✓ $cmd"
    else
        echo "✗ $cmd — missing (install package '$pkg')"
        missing=$((missing + 1))
        missing_pkgs+=("$pkg")
    fi
}

check mkarchiso archiso
check git
check qemu-system-x86_64 qemu-desktop
check xorriso libisoburn
check mksquashfs squashfs-tools
check mcopy mtools
check mmd mtools
check mkfs.fat dosfstools
check grub-mkstandalone grub
check mkinitcpio
check sudo
check rustc rust
check cargo rust
check shellcheck
check msgfmt gettext
check pkg-config pkgconf

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
    echo
    echo "Faltan paquetes necesarios: ${unique_pkgs[*]}"
    echo "Comando para instalar: sudo pacman -S --needed ${unique_pkgs[*]}"
    echo

    if command -v pacman >/dev/null 2>&1; then
        do_install=false
        if [ "$AUTO_INSTALL" = true ]; then
            do_install=true
        elif [ -t 0 ]; then
            read -r -p "¿Deseas instalar los paquetes faltantes ahora con pacman? [S/n] " response
            response="${response:-s}"
            if [[ "$response" =~ ^[sSyY]$ ]]; then
                do_install=true
            fi
        fi

        if [ "$do_install" = true ]; then
            echo "Instalando paquetes faltantes..."
            if sudo pacman -S --needed "${unique_pkgs[@]}"; then
                echo "✓ Paquetes instalados correctamente."
                missing=0
            else
                echo "✗ Error al instalar paquetes con pacman."
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
