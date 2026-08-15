# churros-welcome

Pantalla de bienvenida de ChurrOS, mostrada al iniciar la sesión Live.

**Portada a Rust** (gtk4-rs + libadwaita-rs). El código fuente vive en
`rust/churros-welcome/` (crate Cargo) y se compila a un binario ELF que se
despliega en `/usr/bin/churros-welcome` durante el build (`scripts/build-rust.sh`).

Los assets (SVG + style.css) siguen aquí para el runtime de la ISO; el crate
tiene su propia copia en `rust/churros-welcome/assets/` para desarrollo.

Muestra accesos directos a:

- Instalar ChurrOS (Calamares)
- Repositorio en GitHub
- Comunidad

La tarjeta de información del sistema (`system_card.rs`) está en el crate pero no se monta en la ventana.

Más detalles en `docs/apps.md`.
