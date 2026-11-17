
# machine translated (Spanish), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Informar de un problema

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Algo salió mal. ¿Enviar un informe?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = Esto envía el error, un resumen de tu ordenador y las últimas líneas del registro. Se quitan los nombres, las direcciones de correo, las direcciones de red y los nombres de Wi-Fi. En las líneas del registro aún pueden aparecer nombres de archivos y direcciones web. No envía tus archivos ni tus contraseñas.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = ¿Qué pasó?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = ¿Qué aplicación?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = No estoy seguro

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Otro programa

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = El escritorio

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
report-not-taken = Todavía no se pueden enviar informes.

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
report-no-at = Escribe tu nombre de usuario de Buildmorphic, no un correo electrónico. No uses @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = El nombre de usuario debe tener de 2 a 31 caracteres, con letras minúsculas, números o guiones.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Un nombre de GitHub solo puede tener letras, números y guiones.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle no pudo guardar el informe para más tarde: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = No se pudo guardar: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = sin permiso

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = la unidad está llena

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = falta la carpeta

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = un error de la unidad

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = falta curl.

# Why a report could not be posted: the post could not be started.
report-not-posted = No se pudo enviar.

# Why a report could not be posted: the computer is offline.
report-no-connection = Sin conexión.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = El servidor respondió { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Escribe unas palabras más sobre lo que pasó.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Leyendo el registro.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = No hay registro que enviar con el informe, así que no se puede enviar. Guárdalo en su lugar.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Imagen opcional. Solo la ve el responsable del proyecto.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Elegir una imagen PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Hacer una captura de pantalla

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Quitar la imagen

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Tu imagen, { $width } por { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Elegir una imagen PNG

report-picture-filter = Imágenes PNG

# Under the picture while it is being read.
report-picture-reading = Leyendo.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = No se pudo hacer la captura.

report-picture-no-chooser = Trindle no pudo abrir el selector de archivos.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = No es una imagen PNG.

report-picture-too-big = Supera { $megabytes } MB.

report-picture-too-large = La imagen es demasiado grande. Puede tener hasta { $side } píxeles de ancho y { $side } de alto.

report-picture-damaged = La imagen está dañada.

report-picture-unreadable = No se pudo leer.



report-sent-as-bug = Enviado. El número de tu informe de error es { $number }. Gracias.


# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle está en fase beta. Los informes son anónimos: se quitan los nombres, las direcciones de correo y las carpetas personales. El nombre de usuario es opcional.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Nombre de usuario de Buildmorphic (opcional)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle no pudo enviar el informe. { $why } Pulsa Enviar para intentarlo de nuevo.
