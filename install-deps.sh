#!/usr/bin/env bash
set -euo pipefail

# ChurrOS - Development dependency installer
# Supported target: Arch Linux and Arch-based distributions

if [[ "${EUID}" -eq 0 ]]; then
    echo "No ejecutes este script como root."
    echo "El script usará sudo cuando sea necesario."
    exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
    echo "ERROR: sudo no está instalado."
    exit 1
fi

if ! command -v pacman >/dev/null 2>&1; then
    echo "ERROR: ChurrOS actualmente requiere pacman/Arch Linux para este instalador."
    exit 1
fi

echo "========================================"
echo "       ChurrOS - Install Dependencies"
echo "========================================"
echo

# Required development dependencies
REQUIRED=(
    archiso
    git
    qemu-full
    edk2-ovmf
    rust
    cargo
)

# Optional dependencies
OPTIONAL=(
    virt-manager
    swtpm
)

echo "[1/4] Actualizando la base de datos de paquetes..."
sudo pacman -Sy --needed

echo
echo "[2/4] Instalando dependencias requeridas..."
sudo pacman -S --needed "${REQUIRED[@]}"

echo
echo "[3/4] Instalando dependencias opcionales..."
sudo pacman -S --needed "${OPTIONAL[@]}" || {
    echo
    echo "Aviso: algunas dependencias opcionales no pudieron instalarse."
    echo "La instalación principal continuará."
}

echo
echo "[4/4] Verificando herramientas..."

FAILED=0

check_command() {
    local name="$1"

    if command -v "$name" >/dev/null 2>&1; then
        printf '  [OK] %s\n' "$name"
    else
        printf '  [FAIL] %s\n' "$name"
        FAILED=1
    fi
}

check_command git
check_command qemu-system-x86_64
check_command cargo
check_command rustc
check_command mkarchiso

echo

if [[ "$FAILED" -ne 0 ]]; then
    echo "Algunas herramientas requeridas no fueron encontradas."
    exit 1
fi

echo "========================================"
echo " Dependencias de ChurrOS instaladas."
echo " Ya puedes ejecutar:"
echo
echo "   ./churros build"
echo "   ./churros run"
echo "========================================"