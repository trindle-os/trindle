
# machine translated (Spanish), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Guardado

# Said beside a button that worked, when nothing more is known.
settings-common-done = Hecho

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = No se hizo

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Acerca de esta página

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = Acerca de { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } ya existe

# SAFETY: needs a human review before it ships
# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Hacer lo mismo con el resto ({ $n } más)
        [many] Hacer lo mismo con el resto ({ $n } más)
       *[other] Hacer lo mismo con el resto ({ $n } más)
    }

# SAFETY: needs a human review before it ships
# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Hacer lo mismo con el resto

# SAFETY: needs a human review before it ships
# Tip of the button that keeps the one already there.
settings-common-skip = Omitir: conservar el que ya está

# SAFETY: needs a human review before it ships
# Button that replaces the one already there.
settings-common-replace = Reemplazar

# SAFETY: needs a human review before it ships
# Spoken name of the Replace button.
settings-common-replace-spoken = Reemplazar el que ya está por el que llega

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = El que llega

# Heading of the line that shows the one already there.
settings-common-already-line = El que ya está

# SAFETY: needs a human review before it ships
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = ¿Reemplazar el que ya está?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Vuelve atrás en { $left } segundo.
        [many] Vuelve atrás en { $left } segundos.
       *[other] Vuelve atrás en { $left } segundos.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Vuelve atrás en { $left } segundo. { $more }
        [many] Vuelve atrás en { $left } segundos. { $more }
       *[other] Vuelve atrás en { $left } segundos. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Volver atrás ahora

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Conservar

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } no está disponible

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Instálalo con: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = No se pudo abrir la página web: no se pudo iniciar ningún navegador.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = No se pudo abrir { $name }.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = No se pudo guardar.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = No se hizo: no se pudo acceder al bus de mensajes del sistema. Reinicia el equipo y vuelve a intentarlo.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = No se hizo: no se pudo acceder al bus de mensajes de esta sesión. Cierra sesión, vuelve a iniciarla y prueba otra vez.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = No se pudo abrir la ventana para elegir. Cierra sesión, vuelve a iniciarla y prueba otra vez.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = El disco está lleno.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Ahora mismo no se puede escribir en el disco.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = La parte del sistema que hace esto no se está ejecutando.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Tardó demasiado en responder. Inténtalo de nuevo.

# SAFETY: needs a human review before it ships
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = No se dio la contraseña.

# SAFETY: needs a human review before it ships
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = Esta cuenta no tiene permiso para hacerlo.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Falta en este equipo algo que necesita.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } y { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = El anillo se abre con un clic en el borde de la pantalla.

# Word drawn in the middle of that picture.
settings-common-ring-name = El anillo

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = No se pudo acceder al firmware del portátil: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = El firmware del portátil no aceptó el límite de carga.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Este equipo no permite detener su carga antes de tiempo.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = No se pudo establecer { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Probando Trindle en una máquina virtual

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle se está ejecutando en { $machine }. Funciona bien allí, pero una máquina virtual nunca es tan rápida ni tan fluida como un equipo real, así que juzga su velocidad en hardware real.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle se está ejecutando en { $machine } sin gráficos 3D, así que el procesador tiene que dibujar todo lo que normalmente dibujaría un chip gráfico. Las ventanas, los vídeos y las páginas web irán más lentos que en un equipo real, y Trindle omite sus animaciones para ayudar. Si el equipo donde se ejecuta puede hacerlo, activar la aceleración 3D en los ajustes de pantalla de la máquina virtual marca una gran diferencia. Instalado en un equipo real, nada de esto se aplica.

