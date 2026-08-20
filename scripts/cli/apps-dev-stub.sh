#!/usr/bin/env bash
# Symlinked as nmcli/gsettings/systemctl/... by ./churros apps.
# Reads exec the real binary; writes print a log line and exit 0.
set -euo pipefail

NAME="$(basename "$0")"
REAL="$(PATH="/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/bin" command -v "$NAME" || true)"

is_mutation() {
    case "$NAME" in
        nmcli)
            for a in "$@"; do
                case "$a" in
                    -f|--fields|-g|--get-values) ;;
                    -*) ;;
                    connect|disconnect|delete|modify|add|edit|up|down|rescan|reload|on|off)
                        return 0
                        ;;
                esac
            done
            return 1
            ;;
        wpctl)
            for a in "$@"; do
                case "$a" in set-*) return 0 ;; esac
            done
            return 1
            ;;
        brightnessctl)
            for a in "$@"; do
                case "$a" in set|s) return 0 ;; esac
            done
            return 1
            ;;
        bluetoothctl)
            skip=0
            for a in "$@"; do
                if [ "$skip" -eq 1 ]; then skip=0; continue; fi
                case "$a" in
                    --timeout) skip=1 ;;
                    -*) ;;
                    show|devices|info|list|paired-devices|help) return 1 ;;
                    *) return 0 ;;
                esac
            done
            return 1
            ;;
        rfkill)
            for a in "$@"; do
                case "$a" in block|unblock) return 0 ;; esac
            done
            return 1
            ;;
        systemctl)
            for a in "$@"; do
                case "$a" in
                    is-active|is-enabled|is-failed|status|show|cat|list-units|list-unit-files|list-timers|show-environment|--version|help)
                        return 1
                        ;;
                esac
            done
            return 0
            ;;
        loginctl)
            for a in "$@"; do
                case "$a" in lock*|terminate*|kill*) return 0 ;; esac
            done
            return 1
            ;;
        gsettings)
            case "${1-}" in set|reset) return 0 ;; *) return 1 ;; esac
            ;;
        dconf)
            case "${1-}" in write|reset) return 0 ;; *) return 1 ;; esac
            ;;
        timedatectl)
            for a in "$@"; do
                case "$a" in set-*) return 0 ;; esac
            done
            return 1
            ;;
        localectl)
            for a in "$@"; do
                case "$a" in list-*|status) return 1 ;; esac
            done
            return 0
            ;;
        ufw)
            for a in "$@"; do
                [ "$a" = status ] && return 1
            done
            return 0
            ;;
        pacman)
            for a in "$@"; do
                case "$a" in -Q|-Q*) return 1 ;; esac
            done
            return 0
            ;;
        flatpak)
            for a in "$@"; do
                case "$a" in update|install|remove) return 0 ;; esac
            done
            return 1
            ;;
        pkill|kill|killall|pkexec|sudo|churros-pkexec|calamares|wal|makoctl|swaymsg|swaybg|swaylock|swayidle|wlsunset|waybar|install|setxkbmap|loadkeys)
            return 0
            ;;
        niri)
            for a in "$@"; do
                case "$a" in action|quit) return 0 ;; esac
            done
            return 1
            ;;
        hyprctl)
            for a in "$@"; do
                [ "$a" = dispatch ] && return 0
            done
            return 1
            ;;
        sh|bash)
            # lock_screen which() uses `sh -c 'command -v …'`; allow that read.
            for a in "$@"; do
                case "$a" in
                    *'command -v'*) return 1 ;;
                esac
            done
            return 0
            ;;
        *)
            return 0
            ;;
    esac
}

if is_mutation "$@"; then
    printf '[churros-dev] blocked: %s' "$NAME" >&2
    if [ "$#" -gt 0 ]; then
        printf ' %s' "$@" >&2
    fi
    printf '\n' >&2
    exit 0
fi

if [ -z "$REAL" ]; then
    printf '[churros-dev] no real binary for %s\n' "$NAME" >&2
    exit 0
fi

exec "$REAL" "$@"
