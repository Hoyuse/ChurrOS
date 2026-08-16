#!/usr/bin/env bash
#
# Static checks over the repository. Runs locally (./churros check) and in CI.
# Exits 1 if any check fails. Hygiene notices are reported but never fail.

# No -e here: every check must run so the report is complete.
set -uo pipefail

cd "$(dirname "$0")/../.." || exit 1

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

duplicates=$(grep -v '^#' archiso/packages.x86_64 | grep -v '^$' | sort | uniq -d)
if [ -n "$duplicates" ]; then
    fail "duplicate entries in archiso/packages.x86_64:"
    # shellcheck disable=SC2086
    printf '      %s\n' $duplicates
else
    pass "no duplicates"
fi

# ------------------------------------------------------- Shared resolvers

mapfile -t PACKAGES < <(grep -v '^#' archiso/packages.x86_64 | grep -v '^$')

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
        fail "'$command' is spawned by Niri but is neither in usr/bin nor in packages.x86_64"
        missing=$((missing + 1))
    fi
done
[ "$missing" -eq 0 ] && pass "${#COMMANDS[@]} commands resolve"

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
        fail "$base: '$cmd' does not resolve (usr/bin, deployable crate, packages.x86_64, or local build)"
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
    repo_i=$(step_index 'shellprocess@churros-repo' || true)
    netinstall_i=$(step_index 'netinstall' || true)
    post_i=$(step_index 'shellprocess@post-install' || true)
    umount_i=$(step_index 'umount' || true)

    order_ok=1
    for pair in \
        "shellprocess@pacman-init:$pacman_i" \
        "shellprocess@fix-boot:$fixboot_i" \
        "shellprocess@churros-repo:$repo_i" \
        "netinstall:$netinstall_i" \
        "shellprocess@post-install:$post_i" \
        "umount:$umount_i"
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
        if [ "$fixboot_i" -ge "$repo_i" ]; then
            fail "shellprocess@fix-boot must run before shellprocess@churros-repo"
            order_ok=0
        fi
        if [ "$repo_i" -ge "$netinstall_i" ]; then
            fail "shellprocess@churros-repo must run before netinstall"
            order_ok=0
        fi
        if [ "$((post_i + 1))" -ne "$umount_i" ]; then
            fail "shellprocess@post-install must be the last step before umount"
            order_ok=0
        fi
    fi

    [ "$order_ok" -eq 1 ] && pass "pacman-init → fix-boot → churros-repo → netinstall; post-install before umount"
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

# ------------------------------------------- Local AUR extras ↔ netinstall

section "Local AUR extras in netinstall"

NETINSTALL=installer/calamares/modules/netinstall.yaml

if [ ! -f "$NETINSTALL" ]; then
    fail "$NETINSTALL missing"
elif [ "${#LOCAL_AUR[@]}" -eq 0 ]; then
    fail "no build_aur calls found in scripts/build-aur.sh"
else
    aur_ok=1
    aur_checked=0
    for pkg in "${LOCAL_AUR[@]}"; do
        # Si ya está en la lista base (packages.x86_64) se instala por defecto,
        # así que no necesita aparecer en netinstall.
        if printf '%s\n' "${PACKAGES[@]}" | grep -qx "$pkg"; then
            continue
        fi
        aur_checked=$((aur_checked + 1))
        if ! grep -qE "^[[:space:]]+- name:[[:space:]]+${pkg}[[:space:]]*$" "$NETINSTALL"; then
            fail "'$pkg' is built by build-aur.sh but missing from netinstall.yaml"
            aur_ok=0
        fi
    done
    [ "$aur_ok" -eq 1 ] && pass "${aur_checked} local AUR packages listed in netinstall"
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
