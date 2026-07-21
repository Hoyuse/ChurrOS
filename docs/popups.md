# Popups

Este documento describe el sistema de popups de ChurrOS: pequeñas ventanas que se muestran al interactuar con los iconos de Waybar o con atajos de teclado.

Los popups son componentes individuales del escritorio: audio, batería, bluetooth, brillo, red y energía. Cada uno es un proceso GTK4 independiente que se abre, muestra su contenido y se cierra solo.

---

# Overview

Los popups viven en:

```text
archiso/airootfs/usr/share/churros/popups/
```

Estructura:

```text
popups/
├── popup_manager/        # Lanzador + gestión de procesos
│   ├── main.py           # Entry point
│   ├── manager.py        # PopupManager (lógica principal)
│   ├── popup.py          # Popup (lanzador de procesos)
│   └── process.py        # PopupProcess (estado en /tmp)
├── common/               # Código compartido
│   ├── popup.py          # PopupWindow (clase base)
│   ├── main.py           # App de prueba
│   ├── style.css
│   └── widgets/
│       ├── button.py
│       ├── card.py
│       ├── header.py
│       ├── icon_button.py
│       └── separator.py
├── audio/                # Popup de audio (volumen, mute, dispositivo)
├── battery/              # Popup de batería
├── bluetooth/            # Popup de Bluetooth
├── brightness/           # Popup de brillo
├── network/              # Popup de red (Wi-Fi + Ethernet)
└── power/                # Popup de energía
```

Cada popup individual tiene esta forma:

```text
<popup>/
├── main.py               # Entry point (Gtk.Application)
├── window.py             # <Popup>Window (extiende PopupWindow)
├── style.css             # Estilos específicos
└── widgets/              # Widgets del popup
```

---

# How It Works

## Architecture

Cada popup es un proceso Python GTK4 independiente. El popup manager (`popups/popup_manager/main.py`) es el punto de entrada único desde Waybar o desde el teclado.

Flujo:

1. Waybar ejecuta `python /usr/share/churros/popups/popup_manager/main.py <nombre>` cuando se hace clic en un módulo.
2. `main.py` recibe el nombre y llama a `PopupManager.show(name)`.
3. `PopupManager` consulta si ya hay un popup abierto (lee `/tmp/churros/popup.pid`).
4. Si no hay popup, abre el solicitado. Si hay otro popup abierto, lo mata y abre el nuevo. Si es el mismo popup, lo cierra (toggle).

## PopupManager

`popups/popup_manager/manager.py`:

```python
from popup import Popup
from process import PopupProcess


class PopupManager:

    @staticmethod
    def show(name):

        if not PopupProcess.running():
            Popup.open(name)
            return

        if PopupProcess.name() == name:
            PopupProcess.kill()
            return

        PopupProcess.kill()
        Popup.open(name)
```

Reglas:

- No hay popup → abre el solicitado
- Mismo popup abierto → lo cierra (toggle)
- Otro popup abierto → lo reemplaza

## PopupProcess

`popups/popup_manager/process.py` gestiona el estado del popup activo en disco:

```text
/tmp/churros/popup.pid     # PID del proceso del popup
/tmp/churros/popup.name    # Nombre del popup activo
```

El estado se crea al abrir un popup y se borra al cerrarlo. Esto permite que el manager consulte si hay un popup activo y cuál es, sin mantener estado en memoria.

`PopupProcess.running()` valida además que el PID siga vivo (haciendo `os.kill(pid, 0)`); si el proceso murió pero los archivos quedaron en `/tmp`, los limpia automáticamente.

## Popup

`popups/popup_manager/popup.py` es solo un wrapper que lanza el proceso:

```python
ROOT = Path(__file__).resolve().parent.parent


class Popup:

    @staticmethod
    def open(name):

        popup = ROOT / name / "main.py"

        process = subprocess.Popen(
            ["python3", str(popup)]
        )

        PopupProcess.save(process.pid, name)
```

El proceso se lanza detached. El manager no espera a que termine: el popup vive por su cuenta y cuando se cierra (al hacer clic fuera, presionar Escape, o matar el proceso), el archivo de estado se actualiza la próxima vez que se abra otro popup.

---

# Available Popups

| Nombre | Descripción | Servicio |
|--------|-------------|----------|
| `audio` | Volumen, mute, dispositivo de salida | `services/audio.py` |
| `battery` | Porcentaje, estado, tiempo restante | `services/battery.py` |
| `bluetooth` | Toggle y lista de dispositivos | (hardcodeado) |
| `brightness` | Slider de brillo | `services/brightness.py` |
| `network` | Wi-Fi (toggle + redes) + Ethernet | `services/wifi.py` + `services/ethernet.py` |
| `power` | Lock, logout, suspend, hibernate, restart, shutdown | `services/power.py` |

---

# Base Class: PopupWindow

`popups/common/popup.py` define la clase base que todos los popups extienden:

```python
class PopupWindow(Gtk.ApplicationWindow):

    def __init__(self, app, title="Popup", icon="🧪"):

        super().__init__(application=app)

        self.set_title(title)
        self.set_default_size(320, 400)
        self.set_resizable(False)
        self.set_decorated(False)
        self.add_css_class("popup")

        # Header con icono + título
        self.header = Header(icon, title)
        self.main_box.append(self.header)

        # Contenido (lo añade cada popup con self.add(widget))
        self.content = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
        self.content.set_vexpand(True)
        self.main_box.append(self.content)
```

Características comunes a todos los popups:

- Tamaño fijo 320×400 (puede ser mayor si el contenido lo requiere)
- Sin decoración de ventana
- CSS class `popup` (permite estilo global desde `common/style.css`)
- Header con icono (Nerd Font glyph) + título
- Método `add(widget)` para añadir widgets al cuerpo

---

# Integration with Waybar

Los popups se invocan desde Waybar mediante `on-click` en cada módulo. Ejemplos de `archiso/airootfs/etc/skel/.config/waybar/config.jsonc`:

```jsonc
"network": {
    "on-click": "/usr/bin/python /usr/share/churros/popups/popup_manager/main.py network"
},
"custom/bluetooth": {
    "on-click": "/usr/bin/python /usr/share/churros/popups/popup_manager/main.py bluetooth"
},
"pulseaudio": {
    "on-click": "/usr/bin/python /usr/share/churros/popups/popup_manager/main.py audio"
},
"custom/brightness": {
    "on-click": "/usr/bin/python /usr/share/churros/popups/popup_manager/main.py brightness"
},
"battery": {
    "on-click": "/usr/bin/python /usr/share/churros/popups/popup_manager/main.py battery"
},
"custom/power": {
    "on-click": "/usr/bin/python /usr/share/churros/popups/popup_manager/main.py power"
}
```

Algunos módulos también tienen acciones secundarias:

- `pulseaudio` → `on-click-right` silencia (`wpctl set-mute toggle`)
- `pulseaudio` → `on-scroll-up/down` ajusta volumen en 5%

---

# Adding a New Popup

1. Crea la carpeta del popup:

   ```text
   popups/<nombre>/
   ```

2. Implementa `main.py`:

   ```python
   from window import MyWindow

   class MyApp(Gtk.Application):
       def do_activate(self):
           window = MyWindow(self)
           window.present()

   app = MyApp()
   app.run()
   ```

3. Implementa `window.py` extendiendo `PopupWindow`:

   ```python
   from common.popup import PopupWindow
   from widgets.my_widget import MyWidget

   class MyWindow(PopupWindow):
       def __init__(self, app):
           super().__init__(app, title="My Popup", icon="🧪")
           self.add(MyWidget())
   ```

4. Crea `style.css` con los estilos del popup.

5. Añade la entrada en Waybar o en un keybind de Hyprland.

---

# Limitations

- **Un solo popup a la vez**: el sistema está pensado para un popup activo. Intentar abrir dos da un comportamiento indefinido.
- **Estado en `/tmp`**: al reiniciar se pierde. Esto es intencional: el popup debe reflejar el estado real del sistema, no cachear.
- **Cierre manual**: los popups no se cierran automáticamente al perder foco. Hay que hacer clic fuera o matarlos con `pkill`.
- **Sin tests**: la interacción con GTK4 hace difícil testear sin un display virtual. Los popups se prueban manualmente.

---

# Future Work

- Cierre automático al perder foco (con `focus-out` event o `wayland-popup`).
- Animaciones de entrada/salida.
- Soporte para múltiples popups simultáneos (sidebar con widgets apilados).
- Integración con la barra de notificaciones del sistema.
- API común para que las apps externas puedan mostrar popups propios.
