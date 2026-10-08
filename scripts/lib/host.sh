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
