
# machine translated (Spanish), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Informes de fallos

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = informes de fallos cierres inesperados errores bugs fallos enviar enviado privacidad telemetría buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = Cuando uno de los programas de Trindle falla, el módulo de informes de fallos guarda una nota breve en este ordenador. No envía nada hasta que ejecutes su comando de envío y digas que sí a cada informe, que antes te muestra el informe completo. Tu nombre y tu nombre de GitHub solo se incluyen si los has escrito aquí. Nunca un mensaje, un nombre de archivo, dónde estás ni nada sobre otros programas. Quitar el módulo lo detiene todo.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Fallos

# Under the Crashes switch.
settings-crash-crashes-short = En qué parte del código de Trindle falló.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Fallos envía el programa, su versión, el tipo de fallo y en qué parte del código de Trindle ocurrió.

# Heading and label of the switch for sending errors.
settings-crash-errors = Errores

# Under the Errors switch.
settings-crash-errors-short = El nombre fijo del error y dónde ocurrió.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Errores envía el programa, su versión, el nombre fijo que Trindle da al error y en qué parte de su código.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = Sobre este equipo

# Under the About this computer switch.
settings-crash-computer-short = Linux, procesador, memoria, gráficos, pantallas.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = Sobre este equipo añade a cada informe: el Linux que usa, el tipo de procesador, la memoria redondeada a la baja, Wayland o X11, quién fabricó los gráficos y cuántas pantallas hay.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Qué se envía

# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Los informes no llevan nombre a menos que escribas tu nombre de usuario de Buildmorphic en el cuadro de abajo, lo que solo te da reconocimiento.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = No se cambió nada, porque { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = No se cambió nada, porque el archivo que lo guarda no está como lo escribió Trindle.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = No hay carpeta de ajustes.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = No se pudo crear la carpeta de ajustes.

# Red words when a setting could not be written to its file.
settings-crash-not-written = No se pudo escribir el ajuste.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Fallo ({ $signal }) en { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Error { $code } en { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Recibido

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = No enviado: seguimiento cerrado

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = No enviado: demasiado grande

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = No enviado: rechazado

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = No enviado: demasiados a la vez

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = No enviado

# Said beside the link line once it is saved.
settings-crash-saved = Guardado

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Vaciar la lista

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = No se pudo vaciar la lista.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Eliminado

# Heading of the line that holds the clear button.
settings-crash-kept-here = Se guarda solo en este equipo

# Under the Kept on this computer only heading.
settings-crash-newest-first = Los más recientes primero, hasta cincuenta.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Todavía nada

# Under the Nothing yet heading.
settings-crash-listed-here = Cada informe aparece aquí, exactamente como se envió.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Mostrar exactamente lo que se envió

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }, { $n } veces: { $answer }
       *[other] { $when }, { $n } veces: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Enviado hasta ahora

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Cada informe aparece exactamente como se envió, los más recientes primero, y se guarda solo en este equipo. Recibido significa que Buildmorphic lo tiene. No enviado: seguimiento cerrado significa que el seguimiento de errores de Trindle no acepta informes; rechazado significa que se rechazó por estar mal formado; demasiados significa que llegaron demasiados a la vez.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Después de un fallo se te pregunta, mostrando el informe completo, si quieres enviarlo.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = No se envía nada y no se te pregunta después de un fallo.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = No se envía nada: los fallos y los errores están desactivados.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Los errores de los programas de Trindle se informan de forma anónima.
        [both] Los fallos y errores de los programas de Trindle se informan de forma anónima.
       *[crashes] Los fallos de los programas de Trindle se informan de forma anónima.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Preguntar sobre problemas y compartir informes

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Desactivado lo oculta todo y elimina los informes en espera.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Desactivado. No había informes en espera.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [one] Desactivado. Se eliminó { $n } informe en espera.
        [many] Desactivado. Se eliminaron { $n } informes en espera.
       *[other] Desactivado. Se eliminaron { $n } informes en espera.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Sobre la comunidad

# Heading of the line with the community button.
settings-crash-go-to-bm = Los informes van a Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = La comunidad que ayuda a construir Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = nombre de usuario

# Heading of the line with the username box.
settings-crash-your-username = Tu nombre de usuario de Buildmorphic

# Under the username heading.
settings-crash-username-sub = Opcional. Nunca una dirección de correo.

# Heading of a line about joining with an email address.
settings-crash-join = Unirse solo con una dirección de correo

# Under the Join heading.
settings-crash-join-sub = Gana créditos y niveles por los problemas que informes primero.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Un informe contiene solo una lista fija

# Under the fixed list heading.
settings-crash-fixed-sub = Lo ves primero, y no se envía nada sin que pulses.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Informes que envías

# Help text of the Reports you send card (the information button).
# SAFETY: needs a human review before it ships
settings-crash-reports-about = Cuando falla uno de los programas de Trindle, puede abrirse una ventana que pregunta si enviar un informe. Muestra antes el texto exacto: tus palabras, el modelo del equipo y sus gráficos, el error y el registro justo anterior. Nunca se leen archivos, contraseñas ni nombres de Wi-Fi, y los nombres y direcciones se tachan. También puedes abrir la misma ventana en cualquier momento con el botón Informar de un problema del anillo.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = nombre de github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Tu nombre de GitHub

# Small words under that heading.
settings-crash-github-about = Opcional. Se muestra como Informado por.

