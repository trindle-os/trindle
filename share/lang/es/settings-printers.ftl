
# machine translated (Spanish), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Aún no hay impresoras. Las que se encuentren en tu red aparecen abajo.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } está en pausa, así que por ahora no se imprimirá nada en ella.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } está imprimiendo.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Lista para imprimir en { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } está { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } está configurada; no se ha elegido nada para imprimir primero.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = Hay { $n } impresoras configuradas; ninguna elegida para imprimir primero.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Ese no es un trabajo que CUPS conozca.

# SAFETY: needs a human review before it ships
# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Cancelado

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat falló

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
settings-printers-adding = Añadiendo...

# Shown beside a printer or the address line after it was added.
settings-printers-added = Añadida

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Buscando su controlador...

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
settings-printers-jobs-about = Todos los trabajos que esperan para imprimir, en cualquier impresora, tal como están ahora. La lista se vuelve a leer cada pocos segundos mientras esta página está abierta. La cruz cancela un trabajo; el papel ya impreso sigue impreso.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS no responde

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = No se puede leer la cola.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Buscando…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = La cola.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Nada en espera

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = Los trabajos aparecen aquí mientras se imprimen.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = En pausa

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Inactiva

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Imprimiendo

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Predeterminada, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Imprimir primero en esta

# Tip of the button that prints a test page.
settings-printers-test-page = Imprimir una página de prueba

# SAFETY: needs a human review before it ships
# Tip of the bin button that removes a printer.
settings-printers-remove = Eliminar esta impresora

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Para buscar hace falta cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Instálalo con: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Añadir esta impresora

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = No se encontró nada nuevo

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = ¿Está encendida y en la misma red?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = Primero las que se anuncian, luego cada equipo por turno.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Añadirla

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Por su dirección

# Small line under "By its address".
settings-printers-by-address-sub = Como 192.168.1.50, en su propia pantalla.

# Heading of the card for adding printers.
settings-printers-add-card = Añadir una impresora

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Primero se buscan las impresoras que se anuncian solas, y luego se pregunta a cada equipo de la red por turno. La impresora debe estar encendida y en la misma red. Algunas impresoras no se anuncian, y algunas redes no transmiten el anuncio: añade una por su dirección, como 192.168.1.50. Puedes verla en la propia pantalla de la impresora, en los ajustes de red.

# Name of the page in the list of settings pages.
settings-printers-title = Impresoras

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = impresora imprimir escáner

# Help text at the top of the page.
settings-printers-help = Las impresoras de tu red se encuentran solas. Pulsa Añadir junto a una para usarla.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = La impresión no está instalada en este equipo.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Buscando…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = El servicio de impresión no responde, así que ahora no se puede imprimir nada.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Impresión

# Help text (i button) of that card.
settings-printers-printing-about = La impresión necesita CUPS, que no está en este equipo. Cuando se instale, las impresoras de tu red se encontrarán solas.

# Line in that card.
settings-printers-unavailable = La impresión no está disponible

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Instálalo con: sudo apt install cups

# Line in that card.
settings-printers-then-found = Entonces se encuentran las impresoras

# Small line under it.
settings-printers-then-found-sub = Las de tu red, solas.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = El servicio de impresión.

# Line in the Printers card when there are none.
settings-printers-none = Aún no hay impresoras

# Small line under "No printers yet".
settings-printers-none-sub = Añade una junto a esta.

# Heading of the card that lists the printers.
settings-printers-card = Impresoras

# SAFETY: needs a human review before it ships
# Help text (i button) of the Printers card.
settings-printers-card-about = Todas las impresoras configuradas en este equipo. La estrella hace que una sea la que se usa primero; el icono de página imprime una página de prueba en ella; la papelera la elimina, preguntando dos veces. Los cambios requieren que tu cuenta esté en el grupo lpadmin.

# Red words under a page range box that cannot be used.
printers-bad-range = Páginas como 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = Copias: de 1 a 99

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
printers-not-allowed = No permitido. Tu cuenta debe estar en el grupo lpadmin; cierra sesión y vuelve a entrar cuando lo esté.

# SAFETY: needs a human review before it ships
# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS lo rechazó.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = El nombre de esa impresora no es uno que CUPS permita.

# Red words when printing is not installed on the computer.
printers-not-installed = La impresión no está instalada en este equipo.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } ya no está configurada.

# Red words when the printer chosen is paused.
printers-paused = { $name } está en pausa.

# Red words when sending the document to CUPS took too long.
printers-too-slow = El documento tardó demasiado en enviarse.

