#!/usr/bin/env bash
#
# Static checks over the repository. Runs locally (./churros check) and in CI.
# Exits 1 if any check fails. Hygiene notices are reported but never fail.

# No -e here: every check must run so the report is complete.
set -uo pipefail

cd "$(dirname "$0")/../.." || exit 1

# shellcheck source=scripts/lib/host.sh
source scripts/lib/host.sh

# Arquitectura objetivo del port (x86_64 por defecto solo si el perfil arm64 no existe)
TARGET_ARCH="${TARGET_ARCH:-aarch64}"

FAILURES=0
NOTICES=0

section() { printf '\n== %s\n' "$1"; }
pass()    { printf '  ✓ %s\n' "$1"; }
fail()    { printf '  ✗ %s\n' "$1"; FAILURES=$((FAILURES + 1)); }
notice()  { printf '  ! %s\n' "$1"; NOTICES=$((NOTICES + 1)); }

mapfile -t SCRIPTS < <(git ls-files '*.sh' 'churros')

# ------------------------------------------------------------- Bash syntax

section "Bash syntax"

syntax_errors=0
for script in "${SCRIPTS[@]}"; do
    if ! err=$(bash -n "$script" 2>&1); then
        fail "$script: $err"
        syntax_errors=$((syntax_errors + 1))
    fi
done
[ "$syntax_errors" -eq 0 ] && pass "${#SCRIPTS[@]} scripts parse"

# --------------------------------------------------------------- ShellCheck

section "ShellCheck"

if command -v shellcheck >/dev/null 2>&1; then
    if shellcheck -S error "${SCRIPTS[@]}"; then
        pass "no error-level findings"
    else
        fail "shellcheck reported errors"
    fi
    warnings=$(shellcheck -S warning -f gcc "${SCRIPTS[@]}" 2>/dev/null | grep -c warning || true)
    [ "$warnings" -gt 0 ] && notice "$warnings style warnings (non-blocking)"
else
    notice "shellcheck not installed"
fi

# ----------------------------------------------------------- Python syntax

section "Python syntax"

# ast.parse only reads the source: it neither runs it nor writes .pyc files.
if git ls-files '*.py' | python3 -c '
import ast, sys

files = [line.strip() for line in sys.stdin if line.strip()]
broken = 0
for path in files:
    try:
        ast.parse(open(path, encoding="utf-8").read(), filename=path)
    except SyntaxError as exc:
        print(f"    {path}:{exc.lineno}: {exc.msg}")
        broken += 1
print(f"    {len(files) - broken}/{len(files)} files parse")
sys.exit(1 if broken else 0)
'; then
    pass "all Python files parse"
else
    fail "some Python files have syntax errors"
fi

# ---------------------------------------------------------- Package list

section "ISO package list"

# Una lista por edicion (packages.<edicion>.x86_64). Se recorren todas para
# que anadir una edicion no obligue a tocar este script.
for pkg_list in archiso/packages*.x86_64; do
    [ -f "$pkg_list" ] || continue

    dups=$(grep -v '^#' "$pkg_list" | grep -v '^$' | sort | uniq -d)
    if [ -n "$dups" ]; then
        fail "duplicate entries in $pkg_list:"
        # shellcheck disable=SC2086
        printf '      %s\n' $dups
    else
        pass "no duplicates ($pkg_list)"
    fi
done

# -------------------------------------------- Defaults vs skel (coherencia)

# /usr/share/churros/defaults lo usa churros-settings para "restaurar valores
# por defecto": copia esos ficheros sobre ~/.config. Si divergen de
# /etc/skel/.config, que es lo que recibe una instalacion nueva, el usuario
# recibe otra configuracion distinta cada vez que restaura. Pasa a ser fallo de
# CI en vez de sorpresa.
DEFAULTS_DIR="archiso/airootfs/usr/share/churros/defaults"
SKEL_DIR="archiso/airootfs/etc/skel/.config"
defaults_drift=0

if [ -d "$DEFAULTS_DIR" ]; then
    while IFS= read -r def_file; do
        rel="${def_file#"$DEFAULTS_DIR"/}"
        skel_file="$SKEL_DIR/$rel"
        if [ ! -f "$skel_file" ]; then
            fail "defaults/$rel no tiene equivalente en el skel"
            defaults_drift=$((defaults_drift + 1))
        elif ! cmp -s "$def_file" "$skel_file"; then
            fail "defaults/$rel difiere del skel (restaurar valores por defecto daria otra config)"
            defaults_drift=$((defaults_drift + 1))
        fi
    done < <(find "$DEFAULTS_DIR" -type f | sort)

    if [ "$defaults_drift" -eq 0 ]; then
        pass "defaults sincronizado con el skel ($(find "$DEFAULTS_DIR" -type f | wc -l) ficheros)"
    fi
fi

# ------------------------------------------------------- Installer por edicion

# Anadir una edicion a medias (lista de paquetes pero sin lanzador de sesion,
# o al reves) deja instalaciones que no arrancan. Se comprueba que cada lista
# de paquetes tenga su edicion cableada en los cuatro sitios que la necesitan:
# build.sh, stamp-os-release.sh, edition-session.sh (sesion de greetd del
# sistema instalado) y el dispatcher.

EDITION_SESSION=archiso/airootfs/usr/share/churros/scripts/edition-session.sh

# Brazo de un case. Se admiten los brazos combinados (xfce|server).
arm_pattern() {
    printf '^[[:space:]][^#]*\\b%s\\b[|)]' "$1"
}

list_editions() {
    for list in archiso/packages*.x86_64; do
        [ -f "$list" ] || continue
        base="${list##*/}"
        base="${base#packages.}"
        # packages.${TARGET_ARCH} es el perfil por defecto y es la edicion niri.
        if [ "$base" = "x86_64" ]; then
            echo "niri"
        else
            echo "${base%.x86_64}"
        fi
    done
}

for ed in $(list_editions); do
    problems=""

    # En build.sh el brazo que importa es el que asigna el comando de sesion;
    # el case del nombre que ve el slideshow tambien menciona la edicion y no
    # cuenta.
    if ! grep -qE "$(arm_pattern "$ed")[^#]*SESSION_CMD=" scripts/cli/build.sh; then
        problems="$problems build.sh-sin-brazo-de-sesion"
    fi

    if [ "$ed" = "niri" ]; then
        # niri es el valor por defecto: en estos dos scripts no aparece con su
        # nombre, sino como el brazo * del case o como valor inicial.
        if ! grep -qE '^[[:space:]]*\*\)' branding/stamp-os-release.sh; then
            problems="$problems os-release-sin-variante"
        fi
        if ! grep -qE '\*\)|churros-niri-session' "$EDITION_SESSION"; then
            problems="$problems greetd-sin-edicion"
        fi
    else
        if ! grep -qE "$(arm_pattern "$ed")" branding/stamp-os-release.sh; then
            problems="$problems os-release-sin-variante"
        fi
        if ! grep -qE "$(arm_pattern "$ed")" "$EDITION_SESSION"; then
            problems="$problems greetd-sin-edicion"
        fi
    fi

    if [ -z "$problems" ]; then
        pass "edicion $ed cableada en el instalador"
    else
        fail "edicion $ed incompleta:$problems"
    fi
done

# La tabla edicion -> sesion solo vive en edition-session.sh: la instalacion
# (configure-greetd-session) y el autologin de Ajustes
# (churros-write-root-config) la cargan en vez de llevar una copia propia.
if grep -q 'edition-session.sh' archiso/airootfs/usr/share/churros/scripts/configure-greetd-session &&
   grep -q 'edition-session.sh' archiso/airootfs/usr/local/bin/churros-write-root-config; then
    pass "configure-greetd-session y churros-write-root-config comparten edition-session.sh"
else
    fail "configure-greetd-session o churros-write-root-config no cargan edition-session.sh"
fi

# El instalador no puede quedarse en una sola edicion: el ejecutable que
# declara Calamares tiene que resolver la edicion en runtime.
if grep -q 'executable: "churros-xsession"' installer/calamares/modules/displaymanager.conf &&
   [ -x archiso/airootfs/usr/local/bin/churros-xsession ]; then
    pass "el instalador resuelve la edicion en runtime (churros-xsession)"
else
    fail "displaymanager.conf sigue con un ejecutable de una sola edicion, o falta churros-xsession"
fi

# ------------------------------------------------------- Shared resolvers

mapfile -t PACKAGES < <(grep -v '^#' archiso/packages.${TARGET_ARCH} | grep -v '^$')

# AUR extras built into archiso/packages/ by scripts/build-aur.sh
mapfile -t LOCAL_AUR < <(
    grep -E '^[[:space:]]*build_aur[[:space:]]+' scripts/build-aur.sh |
        awk '{print $2}'
)

# Binary names do not always match package names: awww-daemon ships in 'awww',
# so a substring match counts as a hit.
# Rust apps (churros-*) are compiled at build time by scripts/build-rust.sh and
# are not present in a clean checkout, so a crate with deploy = true resolves.
# Calamares and AUR extras live in archiso/packages/ after the ISO build scripts.
command_exists() {
    local command=$1 package crate_toml
    [ -e "archiso/airootfs/usr/bin/$command" ] && return 0

    for crate_toml in rust/*/Cargo.toml; do
        [ -f "$crate_toml" ] || continue
        grep -q '^deploy = true$' "$crate_toml" || continue
        grep -q "^name = \"$command\"$" "$crate_toml" && return 0
    done

    if [ "$command" = calamares ]; then
        return 0
    fi
    for package in "${LOCAL_AUR[@]}"; do
        [ "$command" = "$package" ] && return 0
    done

    for package in "${PACKAGES[@]}"; do
        # Exact name always counts (short packages like mpv).
        [ "$command" = "$package" ] && return 0
        # Substring only for longer names (awww-daemon ⊆ awww) to avoid
        # false hits from tiny tokens.
        [ "${#package}" -ge 4 ] || continue
        [[ $command == *"$package"* ]] && return 0
    done
    return 1
}

# ------------------------------------------------------------ Niri autostart

section "Commands referenced by Niri"

NIRI_CONFIG=archiso/airootfs/etc/skel/.config/niri/config.kdl

mapfile -t COMMANDS < <(
    grep -oE '(spawn|spawn-at-startup) "[^"]+"' "$NIRI_CONFIG" |
        sed -E 's/.*"([^"]+)"/\1/' | sort -u
)

missing=0
for command in "${COMMANDS[@]}"; do
    if ! command_exists "$command"; then
        fail "'$command' is spawned by Niri but is neither in usr/bin nor in packages.${TARGET_ARCH}"
        missing=$((missing + 1))
    fi
done
[ "$missing" -eq 0 ] && pass "${#COMMANDS[@]} commands resolve"

# -------------------------------------------------- Niri Xwayland integration

section "Niri Xwayland integration"

# packages.${TARGET_ARCH} is the default Niri profile; build.sh substitutes an edition-
# specific list for XFCE, KDE, and Server builds. Keep the satellite scoped to Niri.
XWAYLAND_SATELLITE="xwayland-satellite"
if grep -Fxq "$XWAYLAND_SATELLITE" archiso/packages.${TARGET_ARCH}; then
    pass "xwayland-satellite is included in the Niri profile"
else
    fail "xwayland-satellite is missing from archiso/packages.${TARGET_ARCH} (Niri)"
fi

other_profile_satellite=0
for pkg_list in archiso/packages.*.x86_64; do
    [ -f "$pkg_list" ] || continue
    if grep -Fxq "$XWAYLAND_SATELLITE" "$pkg_list"; then
        fail "$XWAYLAND_SATELLITE must stay out of alternate profile $pkg_list"
        other_profile_satellite=$((other_profile_satellite + 1))
    fi
done
[ "$other_profile_satellite" -eq 0 ] && pass "xwayland-satellite is excluded from alternate profiles"

# Niri owns DISPLAY when its automatic Xwayland integration is enabled. The
# portal startup helper propagates that value to systemd and D-Bus activation.
NIRI_SESSION_WRAPPER="archiso/airootfs/usr/bin/churros-niri-session"
NIRI_PORTAL_HELPER="archiso/airootfs/usr/bin/churros-portal-start"
if grep -Eq '^[[:space:]]*(DISPLAY[[:space:]]+|spawn(-at-startup)?[[:space:]].*xwayland-satellite)' "$NIRI_CONFIG"; then
    fail "Niri config overrides DISPLAY or starts xwayland-satellite manually"
else
    pass "Niri config leaves DISPLAY and satellite startup to Niri"
fi

if grep -Eq '(^|[[:space:]])(unset[[:space:]]+DISPLAY|export[[:space:]]+DISPLAY=|DISPLAY=)' "$NIRI_SESSION_WRAPPER"; then
    fail "Niri session wrapper overrides DISPLAY"
else
    pass "Niri session wrapper preserves Niri's DISPLAY"
fi

if grep -Eq '^[[:space:]]*spawn-at-startup[[:space:]]+"churros-portal-start"[[:space:]]*$' "$NIRI_CONFIG" &&
   grep -A8 -F 'systemctl --user import-environment' "$NIRI_PORTAL_HELPER" |
       grep -qE '^[[:space:]]+DISPLAY([[:space:]]|$)' &&
   grep -A8 -F 'dbus-update-activation-environment --systemd' "$NIRI_PORTAL_HELPER" |
       grep -qE '^[[:space:]]+DISPLAY([[:space:]]|$)'; then
    pass "Niri startup imports DISPLAY into systemd and D-Bus environments"
else
    fail "Niri startup does not propagate DISPLAY to systemd and D-Bus"
fi

# ------------------------------------------------------- Desktop entries

section "Desktop Exec / TryExec"

DESKTOP_DIR=archiso/airootfs/usr/share/applications
desktop_missing=0
desktop_path_missing=0
desktop_count=0

# Prefer TryExec when present; otherwise first token of Exec (field codes stripped).
desktop_command() {
    local file=$1 line
    line=$(grep -E '^TryExec=' "$file" | head -1 | cut -d= -f2- || true)
    if [ -n "$line" ]; then
        printf '%s\n' "$line"
        return
    fi
    line=$(grep -E '^Exec=' "$file" | head -1 | cut -d= -f2- || true)
    # Desktop Entry field codes: %f %F %u %U %i %c %k …
    line=$(printf '%s' "$line" | sed -E 's/ %[[:alpha:]]//g')
    printf '%s\n' "${line%% *}"
}

for desktop in "$DESKTOP_DIR"/*.desktop; do
    [ -f "$desktop" ] || continue
    desktop_count=$((desktop_count + 1))
    base=$(basename "$desktop")

    cmd=$(desktop_command "$desktop")
    if [ -z "$cmd" ]; then
        fail "$base: no Exec= or TryExec="
        desktop_missing=$((desktop_missing + 1))
        continue
    fi
    if ! command_exists "$cmd"; then
        fail "$base: '$cmd' does not resolve (usr/bin, deployable crate, packages.${TARGET_ARCH}, or local build)"
        desktop_missing=$((desktop_missing + 1))
    fi

    # Absolute paths in Exec must exist under airootfs (catches stale python main.py paths).
    exec_line=$(grep -E '^Exec=' "$desktop" | head -1 | cut -d= -f2- || true)
    # shellcheck disable=SC2086
    for token in $exec_line; do
        [[ $token == /* ]] || continue
        token=${token#\"}
        token=${token%\"}
        if [ ! -e "archiso/airootfs$token" ]; then
            fail "$base: Exec path '$token' missing under archiso/airootfs"
            desktop_path_missing=$((desktop_path_missing + 1))
        fi
    done
done

if [ "$desktop_missing" -eq 0 ] && [ "$desktop_path_missing" -eq 0 ]; then
    pass "$desktop_count desktop entries resolve"
fi

# ---------------------------------------------------- Calamares sequence

section "Calamares exec order"

SETTINGS=installer/calamares/settings.conf
MODULES_DIR=installer/calamares/modules

if [ ! -f "$SETTINGS" ]; then
    fail "$SETTINGS missing"
else
    mapfile -t EXEC_STEPS < <(
        awk '
            /^  - exec:/{ in_exec=1; next }
            in_exec && /^  - /{ exit }
            in_exec && /^      - /{
                sub(/^[[:space:]]+-[[:space:]]+/, "")
                print
            }
        ' "$SETTINGS"
    )

    step_index() {
        local needle=$1 i
        for i in "${!EXEC_STEPS[@]}"; do
            if [ "${EXEC_STEPS[$i]}" = "$needle" ]; then
                printf '%s\n' "$i"
                return 0
            fi
        done
        printf '%s\n' '-1'
        return 1
    }

    pacman_i=$(step_index 'shellprocess@pacman-init' || true)
    fixboot_i=$(step_index 'shellprocess@fix-boot' || true)
    post_i=$(step_index 'shellprocess@post-install' || true)
    umount_i=$(step_index 'umount' || true)
    mount_i=$(step_index 'mount' || true)
    bootnocow_i=$(step_index 'shellprocess@boot-nocow' || true)
    unpackfs_i=$(step_index 'unpackfs' || true)

    order_ok=1
    for pair in \
        "shellprocess@pacman-init:$pacman_i" \
        "shellprocess@fix-boot:$fixboot_i" \
        "shellprocess@post-install:$post_i" \
        "umount:$umount_i" \
        "mount:$mount_i" \
        "shellprocess@boot-nocow:$bootnocow_i" \
        "unpackfs:$unpackfs_i"
    do
        name=${pair%%:*}
        idx=${pair##*:}
        if [ "$idx" -lt 0 ]; then
            fail "exec sequence missing '$name'"
            order_ok=0
        fi
    done

    if [ "$order_ok" -eq 1 ]; then
        if [ "$pacman_i" -ge "$fixboot_i" ]; then
            fail "shellprocess@pacman-init must run before shellprocess@fix-boot"
            order_ok=0
        fi
        if [ "$((post_i + 1))" -ne "$umount_i" ]; then
            fail "shellprocess@post-install must be the last step before umount"
            order_ok=0
        fi
        if [ "$mount_i" -ge "$bootnocow_i" ]; then
            fail "shellprocess@boot-nocow must run after mount"
            order_ok=0
        fi
        if [ "$bootnocow_i" -ge "$unpackfs_i" ]; then
            fail "shellprocess@boot-nocow must run before unpackfs"
            order_ok=0
        fi
    fi

    [ "$order_ok" -eq 1 ] && pass "boot-nocow after mount; pacman-init → fix-boot; post-install before umount"
fi

# --------------------------------------------- Calamares shellprocess confs

section "Calamares shellprocess configs"

if [ ! -f "$SETTINGS" ]; then
    fail "cannot check instances without $SETTINGS"
else
    mapfile -t INSTANCE_IDS < <(
        awk '
            /^instances:/{ in_i=1; next }
            in_i && /^[a-zA-Z]/{ exit }
            in_i && /^[[:space:]]+- id:/{
                sub(/^[[:space:]]+- id:[[:space:]]*/, "")
                print
            }
        ' "$SETTINGS"
    )
    mapfile -t INSTANCE_CONFIGS < <(
        awk '
            /^instances:/{ in_i=1; next }
            in_i && /^[a-zA-Z]/{ exit }
            in_i && /^[[:space:]]+config:/{
                sub(/^[[:space:]]+config:[[:space:]]*/, "")
                print
            }
        ' "$SETTINGS"
    )

    conf_ok=1
    if [ "${#INSTANCE_IDS[@]}" -eq 0 ]; then
        fail "no shellprocess instances declared"
        conf_ok=0
    fi
    if [ "${#INSTANCE_IDS[@]}" -ne "${#INSTANCE_CONFIGS[@]}" ]; then
        fail "instances: id/config count mismatch (${#INSTANCE_IDS[@]} ids, ${#INSTANCE_CONFIGS[@]} configs)"
        conf_ok=0
    fi

    i=0
    while [ "$i" -lt "${#INSTANCE_CONFIGS[@]}" ]; do
        conf=${INSTANCE_CONFIGS[$i]}
        if [ ! -f "$MODULES_DIR/$conf" ]; then
            fail "instance '${INSTANCE_IDS[$i]:-?}' config missing: $MODULES_DIR/$conf"
            conf_ok=0
        fi
        i=$((i + 1))
    done

    # Every shellprocess@id in the exec sequence must have a matching instance id.
    for step in "${EXEC_STEPS[@]+"${EXEC_STEPS[@]}"}"; do
        case "$step" in
            shellprocess@*)
                id=${step#shellprocess@}
                found=0
                for known in "${INSTANCE_IDS[@]}"; do
                    if [ "$known" = "$id" ]; then
                        found=1
                        break
                    fi
                done
                if [ "$found" -eq 0 ]; then
                    fail "exec references shellprocess@$id but no matching instances id"
                    conf_ok=0
                fi
                ;;
        esac
    done

    [ "$conf_ok" -eq 1 ] && pass "${#INSTANCE_CONFIGS[@]} shellprocess configs present and referenced"
fi

# Calamares reads defaultFileSystemType (capital S). The other spelling is ignored and ext4 is used.
PARTITION_CONF=installer/calamares/modules/partition.conf
if [ -f "$PARTITION_CONF" ]; then
    if grep -qE '^[[:space:]]*defaultFilesystemType:' "$PARTITION_CONF"; then
        fail "$PARTITION_CONF uses defaultFilesystemType (ignored); need defaultFileSystemType"
    elif grep -qE '^[[:space:]]*defaultFileSystemType:' "$PARTITION_CONF"; then
        pass "partition.conf defaultFileSystemType is the key Calamares reads"
    else
        notice "$PARTITION_CONF has no defaultFileSystemType (Calamares falls back to ext4)"
    fi
fi

# GRUB gfxmenu rejects unknown global properties and unknown + components
# (install then fails to boot the kernel *and* prints theme.txt errors).
# Styled-box pixmaps are a filename pattern with '*', not a + pixmap_style block.
# /boot on btrfs+zstd is unreadable unless images are rewritten into +C inodes.
GRUB_THEME_TXT=branding/grub-theme/theme.txt
if [ -f "$GRUB_THEME_TXT" ]; then
    if grep -qE '^[[:space:]]*title-align:' "$GRUB_THEME_TXT"; then
        fail "$GRUB_THEME_TXT: title-align is not a GRUB gfxmenu property"
    elif grep -qE 'selected_item_pixmap_style_(left|right)' "$GRUB_THEME_TXT"; then
        fail "$GRUB_THEME_TXT: selected_item_pixmap_style_left/right are not GRUB properties"
    elif grep -qE '^\+[[:space:]]*pixmap_style\b' "$GRUB_THEME_TXT"; then
        fail "$GRUB_THEME_TXT: + pixmap_style is not a GRUB gfxmenu component"
    else
        pass "GRUB theme.txt uses only gfxmenu global properties"
    fi

    unknown_comp=0
    while IFS= read -r comp; do
        case "$comp" in
            boot_menu|label|image|hbox|vbox|canvas|circular_progress|progress_bar) ;;
            *)
                fail "$GRUB_THEME_TXT: unknown GRUB component + $comp"
                unknown_comp=$((unknown_comp + 1))
                ;;
        esac
    done < <(grep -E '^\+[[:space:]]+[A-Za-z0-9_]+' "$GRUB_THEME_TXT" \
        | sed -E 's/^\+[[:space:]]+([A-Za-z0-9_]+).*/\1/')
    [ "$unknown_comp" -eq 0 ] && pass "GRUB theme.txt components are gfxmenu types"

    style=$(grep -E '^[[:space:]]*selected_item_pixmap_style[[:space:]]*=' "$GRUB_THEME_TXT" \
        | sed -E 's/.*=[[:space:]]*"([^"]+)".*/\1/' | tail -n1)
    if [ -z "$style" ]; then
        pass "GRUB theme.txt has no selected_item_pixmap_style"
    elif [[ "$style" != *'*'* ]]; then
        fail "$GRUB_THEME_TXT: selected_item_pixmap_style must be a styled-box pattern (e.g. select_*.png), not '$style'"
    else
        center=${style/\*/c}
        if [ ! -f "branding/grub-theme/$center" ]; then
            fail "branding/grub-theme/$center missing (GRUB styled box center slice)"
        else
            pass "GRUB selected_item_pixmap_style uses $style ($center present)"
        fi
    fi

    spaced=0
    for f in branding/grub-theme/*; do
        [ -e "$f" ] || continue
        base=$(basename "$f")
        if [[ "$base" == *' '* ]]; then
            fail "branding/grub-theme/$base: GRUB cannot open filenames with spaces"
            spaced=$((spaced + 1))
        fi
    done
    [ "$spaced" -eq 0 ] && pass "GRUB theme filenames have no spaces"

    if grep -qE '(title-font|terminal-font|[[:space:]]font[[:space:]]*=|[[:space:]]item_font|[[:space:]]selected_item_font).*\.pf2' "$GRUB_THEME_TXT"; then
        fail "$GRUB_THEME_TXT: font properties must use the PFF2 name, not the .pf2 filename"
    else
        pass "GRUB theme.txt font properties use font names, not filenames"
    fi

    if grep -qE '%[[:space:]]+[+-]|[+-][[:space:]]+[0-9]' "$GRUB_THEME_TXT"; then
        fail "$GRUB_THEME_TXT: position expressions must be 50%-260 (no spaces; grub_strtoull fails)"
    else
        pass "GRUB theme.txt position expressions have no spaces"
    fi
fi

BOOT_GRUB_SCRIPT=archiso/airootfs/usr/share/churros/scripts/make-boot-grub-readable
GRUB_THEME_CONF=installer/calamares/modules/shellprocess-grub-theme.conf
BOOT_GRUB_HOOK=archiso/airootfs/etc/pacman.d/hooks/91-churros-boot-grub-readable.hook
if [ ! -f "$BOOT_GRUB_SCRIPT" ]; then
    fail "$BOOT_GRUB_SCRIPT missing (GRUB premature EOF on btrfs zstd /boot)"
elif [ ! -f "$GRUB_THEME_CONF" ] || ! grep -q 'make-boot-grub-readable' "$GRUB_THEME_CONF"; then
    fail "$GRUB_THEME_CONF does not run make-boot-grub-readable after grub-mkconfig"
elif ! grep -q 'conv=fsync' "$BOOT_GRUB_SCRIPT" || grep -qE 'cp -a --' "$BOOT_GRUB_SCRIPT"; then
    fail "$BOOT_GRUB_SCRIPT must full-copy into a +C inode (cp -a reflinks zstd extents on btrfs)"
elif [ ! -f "$BOOT_GRUB_HOOK" ]; then
    fail "$BOOT_GRUB_HOOK missing (kernel updates would rewrite compressed /boot images)"
elif grep -q 'remove from airootfs' "$BOOT_GRUB_HOOK"; then
    fail "$BOOT_GRUB_HOOK would be deleted by the ISO-only hook cleaner"
elif grep -qE 'Type[[:space:]]*=[[:space:]]*Package' "$BOOT_GRUB_HOOK" \
    && grep -qE 'Type[[:space:]]*=[[:space:]]*Path' "$BOOT_GRUB_HOOK" \
    && [ "$(grep -c '^\[Trigger\]' "$BOOT_GRUB_HOOK")" -lt 2 ]; then
    fail "$BOOT_GRUB_HOOK mixes Path and Package Type in one Trigger (pacman applies the last Type to every Target)"
else
    pass "GRUB btrfs /boot rewrite is wired (install + pacman hook)"
fi

# ------------------------------------------- Rollback (churros-snapshot)

section "Rollback snapshots btrfs"

SNAP_SCRIPT=archiso/airootfs/usr/local/bin/churros-snapshot
SNAP_HOOK=archiso/airootfs/etc/pacman.d/hooks/50-churros-snapshot.hook

snap_ok=1
if [ ! -f "$SNAP_SCRIPT" ]; then
    fail "$SNAP_SCRIPT missing (rollback btrfs)"
    snap_ok=0
else
    if ! grep -q 'subvolid=5' "$SNAP_SCRIPT"; then
        fail "$SNAP_SCRIPT must mount the top-level subvol (subvolid=5) for snapshots"
        snap_ok=0
    fi
    if ! grep -q 'btrfs subvol snapshot' "$SNAP_SCRIPT"; then
        fail "$SNAP_SCRIPT must create snapshots with btrfs subvol snapshot"
        snap_ok=0
    fi
    if ! grep -q 'meta.txt' "$SNAP_SCRIPT"; then
        fail "$SNAP_SCRIPT must write per-snapshot metadata (meta.txt)"
        snap_ok=0
    fi
fi

if [ ! -f "$SNAP_HOOK" ]; then
    fail "$SNAP_HOOK missing (no snapshot before pacman transactions)"
    snap_ok=0
else
    if ! grep -q 'When[[:space:]]*=[[:space:]]*PreTransaction' "$SNAP_HOOK"; then
        fail "$SNAP_HOOK must be PreTransaction (snapshot BEFORE the upgrade)"
        snap_ok=0
    fi
    if ! grep -q 'churros-snapshot' "$SNAP_HOOK"; then
        fail "$SNAP_HOOK does not call churros-snapshot"
        snap_ok=0
    fi
    if ! grep -q '||' "$SNAP_HOOK"; then
        fail "$SNAP_HOOK must tolerate snapshot failure (a snapshot error must not block pacman)"
        snap_ok=0
    fi
    if grep -q 'remove from airootfs' "$SNAP_HOOK"; then
        fail "$SNAP_HOOK would be deleted by the ISO-only hook cleaner"
        snap_ok=0
    fi
fi

if ! grep -q 'churros-snapshot' archiso/profiledef.sh; then
    fail "archiso/profiledef.sh must declare the file_permissions entry for /usr/local/bin/churros-snapshot"
    snap_ok=0
fi

if ! grep -q 'churros-snapshot' rust/preferences/src/services/update.rs; then
    fail "UpdateService must expose snapshot management (create/list/delete)"
    snap_ok=0
fi

[ "$snap_ok" -eq 1 ] && pass "rollback btrfs wired (script + hook + profiledef + UI)"

# PartitionLabelsView fills palette().window() and upstream paints Qt::black / Qt::gray.
LABELS_PATCH=installer/patches/calamares-partition-labels.patch
if [ ! -f "$LABELS_PATCH" ]; then
    fail "$LABELS_PATCH missing (partition size/fs text stays Qt::gray on the legend)"
elif ! grep -q 'bg.lightness()' "$LABELS_PATCH"; then
    fail "$LABELS_PATCH does not pick label pens from the view background"
else
    pass "partition labels secondary-text patch present"
fi

# ----------------------------------------------- Calamares branding files

section "Calamares branding"

# Stdlib only (CI has no PyYAML). Mirrors Branding.cpp bail() checks:
# componentName == directory, slideshow exists, image paths exist and
# are non-empty, slideshowAPI 2 requires onActivate/onLeave.
if python3 - installer/calamares/branding <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
errors = 0


def fail(msg: str) -> None:
    global errors
    errors += 1
    print(f"    {msg}")


def unquote(value: str) -> str:
    value = value.strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in {'"', "'"}:
        return value[1:-1]
    return value


def parse_desc(path: Path) -> dict[str, object]:
    text = path.read_text(encoding="utf-8")
    data: dict[str, object] = {"images": {}}
    section = None
    for raw in text.splitlines():
        line = raw.split("#", 1)[0].rstrip()
        if not line.strip() or line.strip() == "---":
            continue
        if re.match(r"^[A-Za-z][A-Za-z0-9_]*:\s*$", line):
            section = line.split(":", 1)[0]
            if section == "images":
                data["images"] = {}
            continue
        m = re.match(r"^([A-Za-z][A-Za-z0-9_]*):\s*(.*)$", line)
        if m:
            section = None
            data[m.group(1)] = unquote(m.group(2))
            continue
        if section == "images":
            m = re.match(r"^\s+([A-Za-z][A-Za-z0-9_]*):\s*(.*)$", line)
            if m:
                data["images"][m.group(1)] = unquote(m.group(2))  # type: ignore[index]
    return data


if not root.is_dir():
    fail(f"{root} missing")
    sys.exit(1)

for component in sorted(p for p in root.iterdir() if p.is_dir()):
    desc = component / "branding.desc"
    if not desc.is_file():
        fail(f"{desc} missing")
        continue
    data = parse_desc(desc)
    name = str(data.get("componentName") or "")
    if name != component.name:
        fail(f"{desc}: componentName '{name}' != directory '{component.name}'")

    sidebar = str(data.get("sidebar") or "widget")
    if sidebar.split(",")[0].strip() == "qml":
        qml_sidebar = component / "calamares-sidebar.qml"
        if not qml_sidebar.is_file():
            fail(f"{desc}: sidebar: qml requires {qml_sidebar}")

    slideshow = str(data.get("slideshow") or "")
    if not slideshow:
        fail(f"{desc}: slideshow is missing")
    else:
        show = component / slideshow
        if not show.is_file():
            fail(f"{desc}: slideshow file {show} does not exist")
        else:
            api = str(data.get("slideshowAPI") or "")
            if api == "2":
                qml = show.read_text(encoding="utf-8")
                if not re.search(r"function\s+onActivate\s*\(", qml):
                    fail(f"{show}: slideshowAPI 2 requires onActivate()")
                if not re.search(r"function\s+onLeave\s*\(", qml):
                    fail(f"{show}: slideshowAPI 2 requires onLeave()")
                if "#0F0F10" not in qml:
                    fail(f"{show}: slideshow has no dark fill (Install page stays Fusion-white)")

    images = data.get("images") or {}
    if not isinstance(images, dict) or not images:
        fail(f"{desc}: images: must list productLogo/productIcon files")
    else:
        for key, value in images.items():
            if value == "":
                fail(f"{desc}: images.{key} is empty (Calamares exits)")
                continue
            image_path = component / value
            if not image_path.is_file():
                fail(f"{desc}: images.{key} file {image_path} does not exist")
            elif image_path.stat().st_size == 0:
                fail(f"{desc}: images.{key} file {image_path} is empty")

    qss = component / "stylesheet.qss"
    if qss.is_file():
        qss_text = qss.read_text(encoding="utf-8")
        if re.search(r"^QWidget\s*\{", qss_text, re.M):
            fail(f"{qss}: QWidget {{ }} paints PartitionLabelsView unreadable")
        if "PrettyRadioButton" not in qss_text or "ChoicePage" not in qss_text:
            fail(f"{qss}: partition ChoicePage/PrettyRadioButton styles missing (white-on-white)")
        if "#summaryStep QWidget" not in qss_text:
            fail(f"{qss}: summary page QWidget styles missing (white-on-white)")
        if "PartitionLabelsView" not in qss_text or "QQuickWidget" not in qss_text:
            fail(f"{qss}: PartitionLabelsView/QQuickWidget styles missing (Fusion-white panels)")
        if "combo-arrow.svg" in qss_text and not (component / "combo-arrow.svg").is_file():
            fail(f"{qss}: combo-arrow.svg is referenced but missing")

sys.exit(1 if errors else 0)
PY
then
    pass "branding component would load"
else
    fail "branding component would make Calamares exit"
fi

# ------------------------------------------- Calamares host preview (no ISO)

section "Calamares host preview"

PREVIEW=installer/calamares/preview
if [ ! -f "$PREVIEW/settings.conf" ]; then
    fail "$PREVIEW/settings.conf missing"
else
    preview_ok=1
    if grep -E '^[[:space:]]+-[[:space:]]*partition[[:space:]]*$' "$PREVIEW/settings.conf" >/dev/null; then
        fail "preview settings.conf must not load partition (polkit / real disks)"
        preview_ok=0
    fi
    if [ -f "$PREVIEW/finished.conf" ] && ! grep -q '^restartNowMode:[[:space:]]*never' "$PREVIEW/finished.conf"; then
        fail "preview finished.conf must set restartNowMode: never"
        preview_ok=0
    fi
    if [ -f "$PREVIEW/locale.conf" ] && ! grep -q '^adjustLiveTimezone:[[:space:]]*false' "$PREVIEW/locale.conf"; then
        fail "preview locale.conf must set adjustLiveTimezone: false"
        preview_ok=0
    fi
    if [ -f "$PREVIEW/keyboard.conf" ] && ! grep -q '^useLocale1:[[:space:]]*false' "$PREVIEW/keyboard.conf"; then
        fail "preview keyboard.conf must set useLocale1: false"
        preview_ok=0
    fi
    [ "$preview_ok" -eq 1 ] && pass "preview does not partition, reboot, or push layout/timezone"
fi

# ------------------------------------------- Calamares finished (producción)

# restartNowMode solo admite never|user-unchecked|user-checked|always en
# Calamares 3.4.x; un valor inválido oculta el botón de reinicio de la
# pantalla final (regresión: "no aparece la opción de reiniciar").
FINISHED_CONF=installer/calamares/modules/finished.conf
if [ ! -f "$FINISHED_CONF" ]; then
    fail "$FINISHED_CONF missing (finished page defaults; restart option may vanish)"
else
    mode=$(grep -E '^restartNowMode:' "$FINISHED_CONF" | head -1 | awk '{print $2}')
    case "$mode" in
        never|user-unchecked|user-checked|always)
            pass "finished.conf restartNowMode=$mode is a valid Calamares 3.4 value"
            ;;
        *)
            fail "$FINISHED_CONF restartNowMode='$mode' is invalid (hide the restart checkbox); use never|user-unchecked|user-checked|always"
            ;;
    esac
    if ! grep -q '^restartNowCommand:' "$FINISHED_CONF"; then
        fail "$FINISHED_CONF should set restartNowCommand (systemctl -i reboot)"
    fi
fi

# Local AUR extras check removed as netinstall module was deprecated in favor of churros-tour

# ------------------------------------------- Calamares Python ABI

section "Calamares libpython"

# El python del host solo representa al de la ISO en Arch. En otra distro
# (paquete construido con ./churros build --container) la comparación daría un
# fallo falso: build-calamares.sh ya recompila dentro del contenedor si la
# versión de python de Arch cambia.
CALAMARES_LOCAL=$(ls archiso/packages/calamares-[0-9]*.pkg.tar.zst 2>/dev/null | head -1 || true)
if [ -z "$CALAMARES_LOCAL" ]; then
    notice "no local calamares package (ISO build will compile it)"
elif ! churros_host_is_arch; then
    notice "host is not Arch: libpython of $(basename "$CALAMARES_LOCAL") is checked by ./churros build --container"
elif ! command -v readelf >/dev/null 2>&1; then
    notice "readelf not available; skip libpython check"
else
    host_python=$(/usr/bin/python3 -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}")')
    abi_tmp=$(mktemp -d)
    if command -v bsdtar >/dev/null 2>&1; then
        bsdtar -xf "$CALAMARES_LOCAL" -C "$abi_tmp" usr/lib/libcalamares.so.3.4.2 2>/dev/null || \
            bsdtar -xf "$CALAMARES_LOCAL" -C "$abi_tmp" usr/lib/libcalamares.so 2>/dev/null || true
    else
        tar --zstd -xf "$CALAMARES_LOCAL" -C "$abi_tmp" usr/lib/libcalamares.so.3.4.2 2>/dev/null || \
            tar --zstd -xf "$CALAMARES_LOCAL" -C "$abi_tmp" usr/lib/libcalamares.so 2>/dev/null || true
    fi
    abi_so=$(find "$abi_tmp" -name 'libcalamares.so*' -type f | head -1 || true)
    pkg_python=""
    if [ -n "$abi_so" ]; then
        pkg_python=$(readelf -d "$abi_so" | sed -n 's/.*libpython\([0-9.]*\)\.so.*/\1/p' | head -1)
    fi
    rm -rf "$abi_tmp"
    if [ -z "$pkg_python" ]; then
        fail "$(basename "$CALAMARES_LOCAL"): could not read libpython NEEDED"
    elif [ "$pkg_python" != "$host_python" ]; then
        fail "$(basename "$CALAMARES_LOCAL") links libpython${pkg_python} but ISO python is ${host_python} (Calamares will not start). Run ./scripts/build-calamares.sh"
    else
        pass "Calamares links libpython${pkg_python} (matches host)"
    fi
    want_stamp=$(
        (
            cd installer/patches || exit 1
            ls calamares-*.patch | sort | xargs sha256sum
            echo "python=$host_python"
        ) | sha256sum | awk '{print $1}'
    )
    have_stamp=$(cat archiso/packages/.calamares-build.stamp 2>/dev/null || true)
    if [ "$have_stamp" != "$want_stamp" ]; then
        fail "local calamares package is stale vs installer/patches (run ./scripts/build-calamares.sh)"
    else
        pass "local calamares package matches installer/patches stamp"
    fi
fi

# El config de greetd versionado debe arrancar la sesión niri en el Live;
# build.sh lo reescribe por edición y lo restaura en el trap.
section "greetd initial_session"
if grep -A3 '\[initial_session\]' archiso/airootfs/etc/greetd/config.toml | grep -q 'niri'; then
    pass "greetd initial_session es niri"
else
    fail "greetd initial_session no es niri"
fi

# ------------------------------------------------------- Live overlay size

section "Live overlay size"

BOOT_CMDLINE_FILES=(
    archiso/grub/grub.cfg
    archiso/grub/loopback.cfg
    archiso/syslinux/archiso_sys-linux.cfg
    archiso/syslinux/archiso_pxe-linux.cfg
    archiso/efiboot/loader/entries/01-archiso-linux.conf
    archiso/efiboot/loader/entries/02-archiso-speech-linux.conf
)

cow_ok=1
for boot_file in "${BOOT_CMDLINE_FILES[@]}"; do
    if [ ! -f "$boot_file" ]; then
        fail "$boot_file missing"
        cow_ok=0
        continue
    fi
    if ! grep -qE '(^|[[:space:]])cow_spacesize=' "$boot_file"; then
        fail "$boot_file missing cow_spacesize= (Flatpak/Bazaar needs >500M overlay)"
        cow_ok=0
    fi
done
[ "$cow_ok" -eq 1 ] && pass "live boot entries set cow_spacesize"

# ------------------------------------------------------------ Translations

section "Translations"

# Aviso honesto: el catalogo sigue apuntando a la interfaz en Python
# (preferences/pages/*.py, popups/*/widgets/*.py), que ya no existe; las apps
# Rust no usan gettext y escriben los textos en castellano directamente. El
# chequeo de abajo es de sintaxis del .po, no de que las traducciones se
# apliquen a la interfaz actual.
notice "po/*.po cubren la UI en Python antigua; las apps Rust no usan gettext"

if command -v msgfmt >/dev/null 2>&1; then
    for po in po/*.po; do
        [ -e "$po" ] || continue
        if msgfmt --check -o /dev/null "$po" 2>/dev/null; then
            pass "$po"
        else
            fail "$po has errors"
        fi
    done
else
    notice "msgfmt not available (gettext package)"
fi

# ---------------------------------------------------------- Distro version

section "Distro version"

if [ ! -f VERSION ]; then
    fail "VERSION missing at repo root"
else
    ver=$(tr -d '[:space:]' < VERSION)
    if [[ ! "$ver" =~ ^[0-9]+(\.[0-9]+)*$ ]]; then
        fail "VERSION must be dotted numeric (got '$ver')"
    else
        pass "VERSION=$ver"
    fi
fi

if grep -q 'include_str!("../../../VERSION")' rust/services/src/version.rs 2>/dev/null; then
    pass "churros-services bakes VERSION at compile time"
else
    fail "rust/services/src/version.rs must include_str the repo VERSION file"
fi

if grep -qE 'const VERSION:[[:space:]]*&str[[:space:]]*=[[:space:]]*"[0-9]' rust/churros-welcome/src/footer.rs; then
    fail "welcome footer has a hardcoded version; use churros_services::version::distro()"
elif grep -q 'churros_services::version::distro' rust/churros-welcome/src/footer.rs; then
    pass "welcome footer reads churros_services::version::distro"
else
    fail "welcome footer must call churros_services::version::distro()"
fi

if grep -qE 'fn version\(\) -> &' rust/preferences/src/services/about.rs \
    && grep -q 'churros_services::version::distro' rust/preferences/src/services/about.rs; then
    pass "Settings About reads churros_services::version::distro"
else
    fail "preferences AboutService::version must call churros_services::version::distro()"
fi

if [ -f branding/stamp-os-release.sh ]; then
    pass "branding/stamp-os-release.sh present"
else
    fail "branding/stamp-os-release.sh missing"
fi

if grep -q 'stamp-os-release.sh' branding/customize_airootfs.sh \
    && grep -q 'stamp-os-release.sh' scripts/cli/build.sh; then
    pass "ISO build stamps os-release from VERSION"
else
    fail "build.sh and customize_airootfs.sh must stamp os-release from VERSION"
fi

# --------------------------------------------- Ejecución privilegiada (polkit)

section "Privileged execution"

POLKIT_RULE=archiso/airootfs/etc/polkit-1/rules.d/50-churros-store.rules

# pkexec publica `program` y `command_line`. `command` no existe y una regla que
# lo lea no autoriza nunca nada: así pasó en #152 sin que nadie lo notara. Se
# mira solo el código (las líneas `//` lo explican y lo nombran).
rule_code=$(grep -nvE '^[[:space:]]*//' "$POLKIT_RULE" || true)
if printf '%s\n' "$rule_code" |
    grep -E "lookup\([[:space:]]*[\"']command[\"'][[:space:]]*\)|getDetails"; then
    fail "$POLKIT_RULE lee lookup(\"command\") o getDetails(): pkexec solo publica program y command_line"
else
    pass "la regla polkit lee las claves que publica pkexec (program, command_line)"
fi

if command -v node >/dev/null 2>&1; then
    if polkit_out=$(node scripts/test-polkit-rules.js 2>&1); then
        pass "decisiones de la regla polkit ($polkit_out)"
    else
        printf '%s\n' "$polkit_out"
        fail "scripts/test-polkit-rules.js"
    fi
else
    notice "node no está instalado: no se prueban las decisiones de la regla polkit"
fi

# churros-update-utils sobre un root falso, churros-write-root-config y la
# tabla edición -> sesión. Sin root y sin tocar /.
if helpers_out=$(python3 scripts/test-privileged-helpers.py 2>&1); then
    pass "helpers privilegiados en un root falso (scripts/test-privileged-helpers.py)"
else
    printf '%s\n' "$helpers_out" | tail -n 40
    fail "scripts/test-privileged-helpers.py"
fi

if network_out=$(python3 scripts/test-network-services.py 2>&1); then
    pass "network service ownership (scripts/test-network-services.py)"
else
    printf '%s\n' "$network_out" | tail -n 40
    fail "scripts/test-network-services.py"
fi

# --------------------------------------------------------------- Hygiene

section "Repository hygiene"

tracked_artifacts=$(git ls-files | grep -cE '\.pyc$|\.pkg\.tar\.zst$|churros\.(db|files)|OVMF_VARS|^\.vscode/|^session-' || true)
if [ "$tracked_artifacts" -gt 0 ]; then
    notice "$tracked_artifacts generated files are tracked in git"
else
    pass "no generated files tracked"
fi

# ---------------------------------------------------------------- Summary

printf '\n----------------------------------------\n'
if [ "$FAILURES" -eq 0 ]; then
    printf 'All checks passed (%s notices)\n' "$NOTICES"
    exit 0
else
    printf '%s checks failed (%s notices)\n' "$FAILURES" "$NOTICES"
    exit 1
fi
