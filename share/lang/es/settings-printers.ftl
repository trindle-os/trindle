
# machine translated (Spanish), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Aún no hay impresoras. Las que Trindle encuentre en tu red aparecen abajo.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } está en pausa, así que por ahora no se imprimirá nada en ella.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } está imprimiendo.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Lista para imprimir en { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } está { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } está configurada, pero no has elegido una impresora predeterminada.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = Hay { $n } impresoras configuradas, pero no has elegido una impresora predeterminada.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Ese trabajo ya no existe.

# SAFETY: needs a human review before it ships
# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Cancelado

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = Falló la comprobación.

# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = El servicio de impresión no está en marcha o no se puede alcanzar.

# Words in the ring while the network is searched for printers.
settings-printers-searching = Buscando

# Words in the ring when the search is finished.
settings-printers-search-done = Hecho

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Ahora predeterminada

# Shown beside a printer after a test page was sent.
settings-printers-sent = Enviado

# SAFETY: needs a human review before it ships
# Shown beside a printer after it was removed.
settings-printers-removed = Eliminada

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Escribe primero la dirección de la impresora.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Añadiendo…

# Shown beside a printer or the address line after it was added.
settings-printers-added = Añadida

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Buscando su controlador…

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Trabajo { $number } en { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# SAFETY: needs a human review before it ships
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = Cancelar este trabajo

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Trabajos de impresión

# SAFETY: needs a human review before it ships
# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Todos los trabajos que esperan para imprimir, en cualquier impresora. Trindle actualiza la lista cada pocos segundos mientras esta página está abierta. Las páginas que ya se han impreso no se pueden cancelar.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS no responde

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = No se pudo leer la cola.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Buscando…

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Nada en espera

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = En pausa

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Inactiva

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Imprimiendo

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Predeterminada, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Usar como predeterminada

# Tip of the button that prints a test page.
settings-printers-test-page = Imprimir una página de prueba

# SAFETY: needs a human review before it ships
# Tip of the bin button that removes a printer.
settings-printers-remove = Eliminar esta impresora

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Para buscar hace falta cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Para instalarlo, ejecuta: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Añadir esta impresora

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = No se encontró nada nuevo

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Añadirla

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Añadir por dirección

# Heading of the card for adding printers.
settings-printers-add-card = Añadir una impresora

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle busca primero las impresoras que se anuncian solas y luego comprueba cada equipo de la red por turno. La impresora debe estar encendida y en la misma red. Algunas impresoras no se anuncian solas, y algunas redes no transmiten el anuncio. En ese caso, añade la impresora por su dirección, como 192.168.1.50. La encontrarás en la propia pantalla de la impresora, en los ajustes de red.

# Name of the page in the list of settings pages.
settings-printers-title = Impresoras

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = impresora imprimir escáner

# Help text at the top of the page.
settings-printers-help = Trindle encuentra solo las impresoras de tu red. Para usar una, pulsa Añadir esta impresora junto a ella.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = La impresión no está instalada en este equipo.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Buscando…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle no puede conectar con el servicio de impresión, así que ahora no se puede imprimir nada.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Impresión

# Help text (i button) of that card.
settings-printers-printing-about = La impresión necesita CUPS, que no está en este equipo. Cuando se instale, Trindle encontrará solo las impresoras de tu red.

# Line in that card.
settings-printers-unavailable = La impresión no está disponible

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Instálalo con: sudo apt install cups

# Line in that card.
settings-printers-then-found = Encontrar impresoras

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Servicio de impresión

# Line in the Printers card when there are none.
settings-printers-none = Aún no hay impresoras

# Heading of the card that lists the printers.
settings-printers-card = Impresoras

# SAFETY: needs a human review before it ships
# Help text (i button) of the Printers card.
settings-printers-card-about = Todas las impresoras configuradas en este equipo. Tu cuenta debe estar en el grupo lpadmin para hacer cambios.

# Red words under a page range box that cannot be used.
printers-bad-range = Por ejemplo: 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = De 1 a 99 copias.

# Name of the choice to print on one side of the paper.
printers-sides-one = Una cara

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Ambas, borde largo

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Ambas, borde corto

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Carta

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Vertical

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Horizontal

# SAFETY: needs a human review before it ships
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = No tienes permiso para hacer esto. Tu cuenta debe estar en el grupo lpadmin. Después, cierra sesión y vuelve a entrar.

# SAFETY: needs a human review before it ships
# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS lo rechazó.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = Ese nombre de impresora no está permitido.

# Red words when printing is not installed on the computer.
printers-not-installed = La impresión no está instalada en este equipo.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } ya no está configurada.

# Red words when the printer chosen is paused.
printers-paused = { $name } está en pausa.

# Red words when sending the document to CUPS took too long.
printers-too-slow = El documento tardó demasiado en enviarse.

# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle no pudo cancelar el trabajo.

# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle no pudo ponerla como impresora predeterminada.

# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle no pudo enviar la página de prueba.

# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle no pudo quitar la impresora.

# Red words when a printer could not be added.
settings-printers-not-added = Trindle no pudo añadir la impresora.

# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle no pudo leer la lista de impresoras.
