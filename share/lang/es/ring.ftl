
# machine translated (Spanish), not yet reviewed by a person.

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = Cerrar

# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = Abrir

# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = Ajustes

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = Uso:

# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPCIÓN…]

# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = Opciones:

# Help for the ring's --say option, printed in a terminal.
ring-cli-say = Hacer lo que piden estas palabras, como si se le hablara al anillo (para probar los comandos de voz sin micrófono)

# Help for the ring's --background option, printed in a terminal.
ring-cli-background = Iniciar sin mostrarse, para que la primera apertura sea instantánea

# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = Guardar todas las ventanas para mostrar el escritorio, o volver a traerlas (Super y D lo piden)

# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = Abrir el anillo ya escuchando, como puede hacerlo un atajo de teclado

# Help for the ring's --all option, printed in a terminal.
ring-cli-all = Abrir el anillo con todas las aplicaciones mostradas alrededor

# Help for the ring's --power option, printed in a terminal.
ring-cli-power = Abrir el anillo al cerrar sesión, reiniciar y apagar

# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = Abrir la página de favoritos en Ajustes

# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = Adoptar de inmediato un aspecto nuevo (claro u oscuro, o un acento nuevo) desde Ajustes

# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = Volver a crear los puntos táctiles de las esquinas superiores

# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = Mover el teclado a los círculos de notificaciones de la pantalla (Super y N lo piden)

# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = opción desconocida { $option }

# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = falta el argumento de --say

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = Ninguna aplicación ni favorito se llama así

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's menu that removes the app.
ring-uninstall = Desinstalar

# SAFETY: needs a human review before it ships
# Line that replaces the uninstall button while the app is being removed.
ring-uninstalling = Desinstalando

# SAFETY: needs a human review before it ships
# Line that replaces the uninstall button once the app is removed.
ring-uninstalled = Desinstalada

# SAFETY: needs a human review before it ships
# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = Quitar del anillo

# SAFETY: needs a human review before it ships
# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = Quitando

# SAFETY: needs a human review before it ships
# Line that replaces that button once the app is off the ring.
ring-taken-off = Quitada

# Name of the ring's round button that opens Apps.
ring-tool-apps = Aplicaciones

# Name of the ring's round button that opens Activity.
ring-tool-activity = Actividad

# Name of the ring's round button that opens Drives.
ring-tool-drives = Unidades

# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = Captura de pantalla

# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = Portapapeles

# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = Notificaciones

# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = Lupa

# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = Cuentagotas

# Name of the ring's round button that opens the report a problem window.
ring-tool-report = Informar de un problema

# Line in the ring when the item that was pressed is not there any more.
ring-gone = Eso ya no está.

# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Nada pudo abrir eso.

# Line in the ring while it listens to a spoken command.
ring-listening = Escuchando... habla, y se detiene cuando tú lo haces

# Line in the ring while it works out what was said.
ring-working-out = Pensándolo...

# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = Nada que cerrar

# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = Son las { $time } del { $date }

# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = El brillo de esta pantalla no se puede cambiar desde aquí

# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = El teclado en pantalla no está instalado

# Screen reader name of the ring of open windows.
ring-open-windows = Ventanas abiertas

# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = Restaurar

# Tip of the button on a window's menu that minimises it.
ring-window-minimise = Minimizar

# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = Restaurar tamaño

# Tip of the button on a window's menu that maximises it.
ring-window-maximise = Maximizar

# SAFETY: needs a human review before it ships
# Tip of the button on a window's menu that stops a program that does not answer.
ring-window-force-close = Forzar cierre

# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = Todas las aplicaciones

# Name of the ring's round button that opens the power page.
ring-power = Energía

# Screen reader description of the ring's power button.
ring-power-spoken = Cerrar sesión, reiniciar o apagar

# Screen reader name of the ring's results while something is typed.
ring-finding = Buscando { $typed }

# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = Fecha y hora

# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = Ventana nueva

# SAFETY: needs a human review before it ships
# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
        [one] Cerrar ventana
       *[other] Cerrar las { $n } ventanas
       [many] Cerrar las { $n } ventanas
    }

# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = Quitar del anillo

# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = Quitar de favoritos

# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = Añadir a favoritos

# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = No se hizo

# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    El centro contiene { $n }.
    Quita uno para hacer sitio.

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: lock the screen.
ring-power-lock = Bloquear

# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = Salvapantallas

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = Suspender

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: hibernate.
ring-power-hibernate = Hibernar

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: log out.
ring-power-log-out = Cerrar sesión

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: restart the computer.
ring-power-restart = Reiniciar

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = Apagar

# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = Haz clic en Todas las aplicaciones y luego clic derecho en una para mantenerla aquí

# SAFETY: needs a human review before it ships
# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = Esto solo lo confirma un clic.

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = Solo tú puedes apagar este equipo.

# SAFETY: needs a human review before it ships
# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = La página de energía se cerró: un programa pulsó una tecla, no tú.

# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = Nada abierto que cerrar

ring-startup-add = Iniciar al arrancar la sesión

ring-startup-remove = Dejar de iniciar al arrancar la sesión



# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = Mostrar todos los atajos de teclado en una lista, o guardar la lista (Super y K, salvo que se cambie en Ajustes)

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = Reiniciado

# Under the notice above.
ring-restarted-why = El anillo y las partes auxiliares se han iniciado de nuevo.

