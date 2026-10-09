# shellcheck shell=bash
#
# Directorio del repositorio local [churros]. Se carga con `source`.
#
# x86_64 sigue en archiso/packages/. aarch64 escribe en
# archiso/packages/aarch64/ para no mezclar paquetes de las dos
# arquitecturas en el mismo churros.db.

# Imprime la ruta absoluta del repo local de esta arquitectura.
churros_local_repo_dir() {
    if [ -n "${CHURROS_PKG_DIR:-}" ]; then
        printf '%s\n' "$CHURROS_PKG_DIR"
        return
    fi
    local root
    root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
    if [ "$(uname -m)" = aarch64 ]; then
        printf '%s\n' "$root/archiso/packages/aarch64"
    else
        printf '%s\n' "$root/archiso/packages"
    fi
}

# makepkg rechaza el paquete si CARCH no está en arch=(). Varios PKGBUILD
# de AUR y de Arch declaran solo x86_64 aunque el código compila en aarch64
# (wlogout, calamares, bazaar). En aarch64 se añade la arquitectura; en
# x86_64 el PKGBUILD no se toca. arch=('any') tampoco.
churros_pkgbuild_allow_arch() {
    local pkgbuild="$1"
    local arch
    arch="$(uname -m)"
    [ "$arch" = aarch64 ] || return 0
    python3 - "$pkgbuild" "$arch" <<'PY'
import re
import sys

path, arch = sys.argv[1], sys.argv[2]
text = open(path, encoding="utf-8").read()

def add(match):
    body = match.group(1)
    tokens = re.findall(r"[A-Za-z0-9_]+", body)
    if arch in tokens or "any" in tokens:
        return match.group(0)
    body = body.rstrip()
    if body.endswith(","):
        body = body + f" '{arch}'"
    else:
        body = body + f" '{arch}'"
    return "arch=(" + body + ")"

new, count = re.subn(r"^arch=\(([^)]*)\)", add, text, count=1, flags=re.M)
if count != 1:
    raise SystemExit(f"PKGBUILD: no arch=() line in {path}")
if new != text:
    open(path, "w", encoding="utf-8").write(new)
    print(f"    arch+=({arch})")
PY
}
