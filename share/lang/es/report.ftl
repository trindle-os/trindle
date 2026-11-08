
# machine translated (Spanish), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Informar de un problema

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Algo salió mal. ¿Enviar un informe?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = Esto envía el error, un resumen del ordenador y las últimas líneas del registro, con los nombres, direcciones de correo, direcciones de red y nombres de Wi-Fi quitados. En las líneas del registro aún puede haber nombres de archivos y direcciones web. No envía tus archivos ni tus contraseñas.

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = ¿Qué estabas haciendo? (opcional)

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
# SAFETY: needs a human review before it ships
report-send = Enviar

# Button at the bottom of the window when the person opened it themselves.
# SAFETY: needs a human review before it ships
report-cancel = Cancelar

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
# SAFETY: needs a human review before it ships
report-dont-send = No enviar

# Button that opens the exact text of the report.
report-show-details = Mostrar detalles

# Button that closes the exact text of the report.
report-hide-details = Ocultar detalles

# Button under the shown report text that copies it.
report-copy = Copiar

# Button under the shown report text that saves it as a file in Downloads.
report-save = Guardar

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Todavía no se aceptan informes.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Comprobando.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Demasiados informes a la vez.

# Written beside the Send button once the report has been sent.
report-already-sent = Ya enviado.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Ya guardado.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Esperando una conexión.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Esperando una conexión. { $why }

# Under the typing box after the report was sent.
report-sent = Enviado. Gracias.

# Under the typing box while the report is being posted.
report-sending = Enviando.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Guardado como Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Usa tu nombre de usuario de Buildmorphic, no un correo electrónico, así que sin @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Un nombre de usuario tiene de 2 a 31 letras minúsculas, números o guiones.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Un nombre de GitHub solo puede tener letras, números y guiones.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = No se pudo conservar para más tarde: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = No se pudo guardar: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = sin permiso

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = la unidad está llena

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = falta la carpeta

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = la unidad lo rechazó

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = falta curl.

# Why a report could not be posted: the post could not be started.
report-not-posted = No se pudo enviar.

# Why a report could not be posted: the computer is offline.
report-no-connection = Sin conexión.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = La dirección respondió { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Escribe unas palabras más para decir qué pasó.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Leyendo el registro.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = No hay registro que enviar con él, así que no se puede enviar. Guárdalo en su lugar.



report-sent-as-bug = Enviado. Registrado como el error { $number }. Gracias.


# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle está en fase beta y cada informe nos ayuda a que funcione a la perfección. Un informe es anónimo y no contiene nada privado ni personal. No hace falta correo electrónico y el nombre de usuario es opcional. No se envía nada hasta que pulses Enviar.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Nombre de usuario de Buildmorphic (opcional)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = No enviado. { $why } Pulsa Enviar para intentarlo de nuevo.

