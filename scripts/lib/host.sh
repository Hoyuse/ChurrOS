# shellcheck shell=bash
#
# Detección de la distro del host. Se carga con `source`; no ejecuta nada.
#
# Se lee /etc/os-release en vez de buscar pacman en el PATH: Debian empaqueta
# un juego llamado pacman (/usr/games/pacman) y pacman-package-manager.

# Imprime la familia del host: arch, debian, fedora u other.
# ID_LIKE cubre las derivadas (Manjaro, EndeavourOS, CachyOS, Arch Linux ARM;
# Ubuntu, Mint, Pop!_OS; Nobara, RHEL, Rocky).
churros_host_family() {
    local ids=""
    if [ -r /etc/os-release ]; then
        # shellcheck disable=SC1091
        ids=$(. /etc/os-release && printf '%s %s' "${ID:-}" "${ID_LIKE:-}")
    fi
    case " $ids " in
        *" arch "*|*" archarm "*) echo arch ;;
        *" debian "*|*" ubuntu "*) echo debian ;;
        *" fedora "*|*" rhel "*) echo fedora ;;
        *) echo other ;;
    esac
}

churros_host_is_arch() {
    [ "$(churros_host_family)" = arch ]
}

# Nombre legible de la distro, para los mensajes.
churros_host_name() {
    local name=""
    if [ -r /etc/os-release ]; then
        # shellcheck disable=SC1091
        name=$(. /etc/os-release && printf '%s' "${PRETTY_NAME:-${NAME:-}}")
    fi
    printf '%s\n' "${name:-$(uname -s)}"
}

# Dentro del contenedor de build (scripts/container-entrypoint.sh).
churros_in_container() {
    [ "${CHURROS_IN_CONTAINER:-0}" = 1 ]
}

# Un host que no es aarch64 necesita qemu-user + binfmt para correr el
# contenedor de la ISO ARM. En un host aarch64 el mismo contenedor es nativo.
churros_need_aarch64_emulation() {
    [ "$(uname -m)" != aarch64 ]
}

# Cierto si binfmt tiene registrada la interpretación de binarios aarch64 y
# el intérprete existe. Acepta qemu-aarch64 y qemu-aarch64-static.
churros_aarch64_emulation_ready() {
    local entry interp
    for entry in /proc/sys/fs/binfmt_misc/qemu-aarch64 \
                 /proc/sys/fs/binfmt_misc/qemu-aarch64-static; do
        [ -r "$entry" ] || continue
        grep -q '^enabled$' "$entry" || continue
        interp=$(awk '/^interpreter / { print $2; exit }' "$entry")
        if [ -n "$interp" ] && [ -x "$interp" ]; then
            return 0
        fi
    done
    return 1
}
