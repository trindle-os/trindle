
# machine translated (German), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Noch keine Drucker. Alle, die in deinem Netzwerk gefunden werden, stehen unten.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } ist angehalten, darum wird vorerst nichts darauf gedruckt.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } druckt.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Bereit zum Drucken auf { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } ist { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } ist eingerichtet; es ist aber keiner zum bevorzugten Drucken gewählt.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } Drucker sind eingerichtet; keiner ist zum bevorzugten Drucken gewählt.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Diesen Auftrag kennt CUPS nicht.

# Shown beside a print job after it was cancelled.
# SAFETY: needs a human review before it ships
settings-printers-cancelled = Abgebrochen

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat ist fehlgeschlagen

# Words in the ring while the network is searched for printers.
settings-printers-searching = Suche läuft

# Words in the ring when the search is finished.
settings-printers-search-done = Fertig

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Jetzt Standard

# Shown beside a printer after a test page was sent.
settings-printers-sent = Gesendet

# Shown beside a printer after it was removed.
# SAFETY: needs a human review before it ships
settings-printers-removed = Entfernt

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Gib zuerst die Adresse des Druckers ein.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Wird hinzugefügt ...

# Shown beside a printer or the address line after it was added.
settings-printers-added = Hinzugefügt

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Der Treiber wird gesucht ...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Auftrag { $number } auf { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = Diesen Auftrag abbrechen

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Druckaufträge

# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Jeder Auftrag, der auf irgendeinem Drucker auf das Drucken wartet, so wie es jetzt ist. Die Liste wird alle paar Sekunden neu gelesen, solange diese Seite offen ist. Das Kreuz bricht einen Auftrag ab; schon gedrucktes Papier bleibt gedruckt.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS antwortet nicht

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Die Warteschlange kann nicht gelesen werden.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Suche läuft …

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = Die Warteschlange.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Nichts wartet

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = Aufträge erscheinen hier, solange sie drucken.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Angehalten

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Untätig

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Druckt

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Standard, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Zuerst auf diesem drucken

# Tip of the button that prints a test page.
settings-printers-test-page = Eine Testseite drucken

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = Diesen Drucker entfernen

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Die Suche braucht cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Installiere es mit: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Diesen Drucker hinzufügen

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Nichts Neues gefunden

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = Ist er an und im selben Netzwerk?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = Zuerst die, die sich melden, dann jedes Gerät nacheinander.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Hinzufügen

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Über seine Adresse

# Small line under "By its address".
settings-printers-by-address-sub = Zum Beispiel 192.168.1.50, auf seinem eigenen Display.

# Heading of the card for adding printers.
settings-printers-add-card = Einen Drucker hinzufügen

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Zuerst werden Drucker gesucht, die sich selbst melden, dann wird jedes Gerät im Netzwerk nacheinander gefragt. Der Drucker muss an und im selben Netzwerk sein. Manche Drucker melden sich nicht selbst, und manche Netzwerke leiten die Meldung nicht weiter: Füge einen dann über seine Adresse hinzu, zum Beispiel 192.168.1.50. Du findest sie auf dem Display des Druckers unter den Netzwerkeinstellungen.

# Name of the page in the list of settings pages.
settings-printers-title = Drucker

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = drucker drucken scanner

# Help text at the top of the page.
settings-printers-help = Drucker in deinem Netzwerk werden von selbst gefunden. Drücke neben einem auf Hinzufügen, um ihn zu nutzen.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = Das Drucken ist auf diesem Computer nicht installiert.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Suche läuft …

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Der Druckdienst antwortet nicht, darum kann gerade nichts gedruckt werden.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Drucken

# Help text (i button) of that card.
settings-printers-printing-about = Zum Drucken wird CUPS gebraucht, das auf diesem Computer nicht vorhanden ist. Sobald es installiert ist, werden Drucker in deinem Netzwerk von selbst gefunden.

# Line in that card.
settings-printers-unavailable = Drucken ist nicht verfügbar

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Installiere es mit: sudo apt install cups

# Line in that card.
settings-printers-then-found = Dann werden Drucker gefunden

# Small line under it.
settings-printers-then-found-sub = Die in deinem Netzwerk, von selbst.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Der Druckdienst.

# Line in the Printers card when there are none.
settings-printers-none = Noch keine Drucker

# Small line under "No printers yet".
settings-printers-none-sub = Füge daneben einen hinzu.

# Heading of the card that lists the printers.
settings-printers-card = Drucker

# Help text (i button) of the Printers card.
settings-printers-card-about = Jeder Drucker, der auf diesem Computer eingerichtet ist. Der Stern macht einen zum zuerst genutzten Drucker; das Seitensymbol druckt eine Testseite darauf; der Papierkorb entfernt ihn und fragt zweimal. Für Änderungen muss dein Konto in der Gruppe lpadmin sein.

# Red words under a page range box that cannot be used.
printers-bad-range = Seiten wie 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = Kopien: 1 bis 99

# Name of the choice to print on one side of the paper.
printers-sides-one = Eine Seite

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Beide, lange Kante

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Beide, kurze Kante

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Hochformat

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Querformat

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = Nicht erlaubt. Dein Konto muss in der Gruppe lpadmin sein; melde dich ab und wieder an, sobald es das ist.

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS hat abgelehnt.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = Dieser Druckername ist bei CUPS nicht erlaubt.

# Red words when printing is not installed on the computer.
printers-not-installed = Das Drucken ist auf diesem Computer nicht installiert.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } ist nicht mehr eingerichtet.

# Red words when the printer chosen is paused.
printers-paused = { $name } ist angehalten.

# Red words when sending the document to CUPS took too long.
printers-too-slow = Das Senden des Dokuments hat zu lange gedauert.

