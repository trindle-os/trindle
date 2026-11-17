# machine translated (Spanish), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Teclado y ratón

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = distribución idioma escritura bloq mayús teclado en pantalla botón anillo ratón panel táctil touchpad puntero velocidad tocar clic desplazamiento rueda más lento más rápido natural zurdo complemento atajo teclas reasignar cambiar intercambiar ctrl escape super alt mac botones laterales pulsado mantenido creado por programas keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Todo lo que hay aquí cambia al instante. Puedes cambiar a una segunda distribución pulsando Alt y Mayús a la vez.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Escribiendo en { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt y Mayús cambian a { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Bloq Mayús no hace nada.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Bloq Mayús funciona como { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = otra tecla Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Retroceso

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = Este equipo no tiene la lista de distribuciones de teclado.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Teclado

# Heading of the line in that card.
settings-keyboard-no-list-heading = La lista de distribuciones de teclado no está disponible

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Para instalarla, ejecuta: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Has cambiado una tecla o un botón.
       *[other] Has cambiado { $n } teclas y botones.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Distribución

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Distribución

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Variante

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Estándar

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Ninguna

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Cambiar a

# Words behind the i of the layout card.
settings-keyboard-layout-about = Esto define lo que escribe cada tecla. Puedes elegir una distribución y una variante de ella. Pulsa Alt y Mayús a la vez para cambiar a una segunda distribución, y otra vez para volver. Los cambios se aplican al instante.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = En uso ahora

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Bloq Mayús

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Otra tecla Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Retroceso

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Nada

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Usar Bloq Mayús como

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Botón de teclado en el anillo

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Otras teclas

# Words behind the i of that card.
settings-keyboard-other-about = Bloq Mayús puede funcionar como otra tecla, por ejemplo una segunda Ctrl o Escape. El botón de teclado está en el centro del anillo y abre un teclado en la pantalla. Puedes usarlo para escribir con el ratón o con una pantalla táctil. Aparece la próxima vez que se abra el anillo.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Un programa no puede cambiar esto.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Pulsa la tecla o el botón del ratón que quieras cambiar.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = No pulsaste nada, así que no se añadió nada.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Escribe la tecla que se usará en su lugar, por ejemplo Escape o Ctrl+C, o escribe Nada.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle no pudo saber qué tecla era. Elige la tecla de la lista.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Todos los teclados y ratones

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Tú mismo has cambiado esta tecla abajo, y se usa ese cambio.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Cambiar { $key } en

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Cambiar { $key } por

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Quitar este cambio

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = Se ignoró la línea { $n }: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = No se pudieron leer las teclas cambiadas

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Tus teclas funcionan con normalidad hasta que Trindle pueda leer los cambios. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Elige la tecla o el botón que quieres cambiar

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Pulsa la tecla o el botón que quieres cambiar

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Cambiar una tecla o un botón

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = Ya has cambiado { $key }.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Teclas y botones

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Cualquier tecla o botón del ratón puede hacer otra cosa. Puede convertirse en otra tecla u otro botón, en teclas pulsadas juntas como Ctrl+C, o en teclas pulsadas una tras otra como H, I. También puede no hacer nada, o hacer una cosa al pulsar y otra al mantener, por ejemplo Escape al pulsar y Ctrl al mantener. Funciona en todas partes, desde la pantalla de inicio de sesión. Ctrl, Alt y una tecla F siempre abren una consola de texto, y si escribes allí trindle-settings --keys default, todas las teclas vuelven a su estado original.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Intercambiar Ctrl y Bloq Mayús

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Bloq Mayús: pulsar para Escape, mantener para Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Intercambiar Escape y Bloq Mayús

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Intercambiar Alt y Super, como en un teclado de Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Los botones laterales del ratón copian y pegan

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = Confiar en { $name }

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Ninguno por ahora

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Teclados y ratones creados por programas

# Words behind the i of that card.
settings-keyboard-made-about = Las herramientas de reasignación y los lanzadores de juegos pueden crear su propio teclado o ratón. Sus teclas y clics siguen pudiendo escribir y jugar, pero no pueden pulsar botones protegidos, como Apagar o Permitir, ni responder a una pregunta dirigida a ti. Activa uno aquí para que cuente como tú.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Mando { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } en { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = Qué hace { $title }

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Intercambiar A y B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Intercambiar X e Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Invertir arriba y abajo del joystick derecho

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Atajos de complementos

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } no se usa: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Teclas para { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Guardado

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = También para el panel táctil.

# Title of the mouse card.
settings-keyboard-mouse-title = Ratón

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = La velocidad del puntero es cuánto se mueve el puntero cuando mueves el ratón. También se aplica al panel táctil. La velocidad de desplazamiento es cuánto se mueve una página por cada giro de la rueda. Elige Más lento, Normal o Más rápido, o ajústala en cualquier punto del control deslizante. Con el desplazamiento natural, la página se mueve en el sentido en que gira la rueda, como en un teléfono. Para zurdos intercambia los botones izquierdo y derecho.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Velocidad del puntero

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Velocidad de desplazamiento

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Desplazamiento natural

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Para zurdos

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Ayuda para hacer clic

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Los clics de un dispositivo de clic por permanencia o por pulsador cuentan como tuyos.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Panel táctil

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = La velocidad de desplazamiento es cuánto se mueve una página cuando te desplazas con dos dedos. Tocar para hacer clic hace que un toque suave cuente como un clic. Con el desplazamiento natural, la página se mueve en el sentido en que se mueven tus dedos, como en un teléfono. Desactivar al escribir evita que la palma de la mano mueva el puntero mientras escribes.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tocar para hacer clic

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Desactivar al escribir

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Más lento

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Más rápido

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = Puertos USB

# Words behind the i of that card.
settings-keyboard-usb-about = Si un puerto USB no arranca, Trindle lo apaga y lo vuelve a encender una vez para intentarlo de nuevo. Nunca lo hace más de una vez por puerto.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle tampoco pudo deshacer el cambio: { $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Capturas de pantalla, informes y atajos

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Hacer una captura de pantalla

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Mostrar la lista de atajos

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Hacer una captura de pantalla ahora

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Mostrar la lista ahora

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Reiniciar el escritorio

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Reiniciar el anillo y los programas auxiliares

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Reiniciar todo

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Teclas para { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Reiniciar ahora. Las ventanas abiertas se quedan como están.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Preguntar y reiniciar todo

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Reiniciado

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = Otro atajo ya usa { $keys }.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = No se guardó. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Informar de un problema

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Informar de un problema ahora

# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = No se guardó. No se pudo leer el archivo de teclas cambiadas, así que se dejó como estaba.
