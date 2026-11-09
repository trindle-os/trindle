# machine translated (Spanish), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Teclado y ratón

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = distribución idioma escritura bloq mayús teclado en pantalla botón anillo ratón panel táctil touchpad puntero velocidad tocar clic desplazamiento rueda más lento más rápido natural zurdo complemento atajo teclas reasignar cambiar intercambiar ctrl escape super alt mac botones laterales pulsado mantenido creado por programas keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Todo lo que hay aquí cambia al instante. Para cambiar a una segunda distribución, pulsa Alt y Mayús a la vez.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Escribiendo en { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt y Mayús cambian a { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Bloq Mayús no hace nada.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Bloq Mayús actúa como { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = otra tecla Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Retroceso

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = La lista de distribuciones de teclado no está en este equipo.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Teclado

# Heading of the line in that card.
settings-keyboard-no-list-heading = La lista de distribuciones de teclado no está disponible

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Instálala con: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Se ha cambiado una tecla.
       *[other] Se han cambiado { $n } teclas y botones.
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

# Small words under the second layout line.
settings-keyboard-switch-to-about = Alt y Mayús a la vez alternan entre las dos.

# Words behind the i of the layout card.
settings-keyboard-layout-about = Lo que escribe cada tecla: la distribución y una variante, por ejemplo una sin teclas muertas. Una segunda distribución se activa con Alt y Mayús a la vez, y se vuelve a la anterior del mismo modo. Todo cambia al instante.

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
settings-keyboard-caps-acts-as = La tecla Bloq Mayús actúa como

# Small words under that line.
settings-keyboard-caps-acts-as-about = Otra Ctrl, Escape, etc.

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Botón de teclado en el anillo

# Small words under that line.
settings-keyboard-ring-button-about = Muestra un teclado en la pantalla.

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Otras teclas

# Words behind the i of that card.
settings-keyboard-other-about = Bloq Mayús puede actuar como otra tecla, por ejemplo otra Ctrl o Escape. El botón de teclado es un teclado en el centro del anillo que muestra un teclado en la pantalla, para escribir con el ratón o con una pantalla táctil. Aparece la próxima vez que se abra el anillo.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Solo una pulsación tuya puede cambiar esto.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Pulsa la tecla o el botón del ratón que quieras cambiar.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = No se pulsó nada, así que no se añadió nada.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Escribe en qué se convierte, por ejemplo Escape, Ctrl+C o Nada.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = No se pudo distinguir esa pulsación de otra. Elige la tecla de la lista.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Todos los teclados y ratones

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = También se ha cambiado abajo; se usa ese cambio.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = En qué teclado o ratón se cambia { $key }

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = En qué se convierte { $key }

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Quitar este cambio

# Small words under a changed key's line when it changes on every keyboard and mouse.
settings-keyboard-every-device-about = En todos los teclados y ratones.

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = La línea { $n } no se usa: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = No se pudieron leer las teclas cambiadas

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Todas las teclas funcionan como están impresas hasta que se puedan leer: { $why }.

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Elige la tecla o el botón que quieres cambiar

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Pulsa la tecla o el botón que quieres cambiar

# Small words under the change a key line.
settings-keyboard-keys-add-about = Púlsala o elígela de la lista.

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Cambiar una tecla o un botón

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } ya está cambiada, más abajo.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Teclas y botones

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Cualquier tecla o botón del ratón puede hacer otra cosa: otra tecla u otro botón, teclas pulsadas juntas como Ctrl+C, una serie de teclas una tras otra como H, I, nada en absoluto, o una cosa al pulsar y otra al mantener, por ejemplo Escape al pulsar y Ctrl al mantener. Funciona en todas partes, desde la pantalla de inicio de sesión. Ctrl, Alt y una tecla F siempre llegan a una consola de texto, y si escribes allí trindle-settings --keys default, todas las teclas vuelven a su estado original.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Intercambiar Ctrl y Bloq Mayús

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Bloq Mayús es Escape al pulsar y Ctrl al mantener

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Intercambiar Escape y Bloq Mayús

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Intercambiar Alt y Super, como en un teclado de Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Los botones laterales del ratón copian y pegan

# Small words under a keyboard or mouse a program made, when it counts as the person.
settings-keyboard-made-yes = Cuenta como tú, igual que un teclado conectado.

# Small words under a keyboard or mouse a program made, when it does not count as the person.
settings-keyboard-made-no = Escribe y juega. No puede pulsar botones protegidos.

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = { $name } cuenta como tú

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Ninguno por ahora

# Small words under that line.
settings-keyboard-made-none-about = Aquí aparecerá el que cree un programa, para que puedas hacer que cuente como tú.

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Teclados y ratones creados por programas

# Words behind the i of that card.
settings-keyboard-made-about = Las herramientas de reasignación y los lanzadores de juegos pueden crear su propio teclado o ratón. Sus teclas y clics siguen escribiendo y jugando, pero no pueden pulsar botones protegidos, como Apagar o Permitir, ni escribir en una pregunta dirigida a ti. Activa uno aquí para que cuente como tú.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Mando { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } en { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = Qué hace { $title }

# Small words under a controller's changed button; the words Button and what it becomes stand for the two sides of the line typed.
settings-keyboard-pad-type = Escríbelo como Botón = en qué se convierte.

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Intercambiar A y B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Intercambiar X e Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Invertir arriba y abajo del joystick derecho

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Atajos de complementos

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }. No se usa: { $why }

# Small words under a plugin shortcut; { $plugin } is the plugin. The example is keys a person types.
settings-keyboard-plugin-about = { $plugin }. Escribe las teclas, por ejemplo Ctrl+Alt+S, o nada para ninguna.

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Teclas para { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Guardado

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = También el panel táctil.

# Title of the mouse card.
settings-keyboard-mouse-title = Ratón

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = La velocidad del puntero es cuánto se mueve el puntero por cada movimiento del ratón, y también la del panel táctil. La velocidad de desplazamiento es cuánto se mueve una página por cada giro de la rueda: Más lento, Normal o Más rápido, o cualquier punto del control deslizante. Con el desplazamiento natural la página se mueve en el sentido en que gira la rueda, como en un teléfono. Para zurdos intercambia los botones izquierdo y derecho.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Velocidad del puntero

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Velocidad de desplazamiento

# Small words under the wheel's scroll speed.
settings-keyboard-scroll-wheel-about = Cuánto se mueve una página por cada giro de la rueda.

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Desplazamiento natural

# Small words under natural scrolling for the mouse wheel.
settings-keyboard-natural-wheel-about = La página sigue a la rueda, como en un teléfono.

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Para zurdos

# Small words under that switch.
settings-keyboard-left-handed-about = Intercambia los botones izquierdo y derecho.

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Ayuda para hacer clic

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Los clics de un dispositivo de clic por permanencia o por pulsador cuentan como tuyos.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Panel táctil

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = La velocidad de desplazamiento es cuánto se mueve una página con dos dedos. Tocar para hacer clic hace que un toque suave cuente como un clic. Con el desplazamiento natural la página se mueve en el sentido en que van tus dedos, como en un teléfono. Desactivar al escribir evita que la palma de la mano mueva el puntero al rozar el panel táctil mientras escribes.

# Small words under the touchpad's scroll speed.
settings-keyboard-scroll-touchpad-about = Cuánto se mueve una página con dos dedos.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tocar para hacer clic

# Small words under that switch.
settings-keyboard-tap-about = Un toque suave cuenta como un clic.

# Small words under natural scrolling for the touchpad.
settings-keyboard-natural-touchpad-about = La página sigue a tus dedos, como en un teléfono.

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Desactivar al escribir

# Small words under that switch.
settings-keyboard-quiet-about = Si la palma roza el panel, el puntero no se mueve.

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Más lento

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Más rápido

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = Puertos USB

# Words behind the i of that card.
settings-keyboard-usb-about = Cuando un puerto USB no arranca, Trindle lo apaga y lo vuelve a encender una vez para intentarlo de nuevo, y nunca más de una vez por puerto.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error }; y no se pudo quitar su marca: { $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Captura de pantalla y lista de atajos

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Hacer una captura de pantalla

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Mostrar la lista de atajos

# Small words under each of the two shortcuts.
settings-keyboard-shortcuts-about = Escribe las teclas, o nada para ninguna.

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Hacer una captura de pantalla ahora

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Mostrar la lista ahora

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Reiniciar el escritorio

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Reiniciar el anillo y las partes auxiliares

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Reiniciar todo

# Small words under each of the two restart shortcuts. The example is keys a person types.
settings-keyboard-restart-about = Escribe las teclas, como Ctrl+Alt+Escape, o nada para ninguna.

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Teclas para { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Reiniciar ahora. Las ventanas abiertas se quedan como están.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Reiniciar todo. Pregunta antes.

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Reiniciado

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } ya lo usa otro atajo.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = No se guardó: { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Informar de un problema

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Informar de un problema ahora

