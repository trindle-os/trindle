# machine translated (French), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Aucune imprimante pour le moment. Celles que Trindle trouve sur votre réseau sont listées ci-dessous.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } est en pause, rien n'y sera donc imprimé pour le moment.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } imprime.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Prêt à imprimer sur { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } est { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } est configurée, mais vous n'avez pas choisi d'imprimante par défaut.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } imprimantes sont configurées, mais vous n'avez pas choisi d'imprimante par défaut.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Ce travail n'existe plus.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Annulé

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = La vérification a échoué.

# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = Le service d'impression ne fonctionne pas ou est inaccessible.

# Words in the ring while the network is searched for printers.
settings-printers-searching = Recherche

# Words in the ring when the search is finished.
settings-printers-search-done = Terminé

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Par défaut maintenant

# Shown beside a printer after a test page was sent.
settings-printers-sent = Envoyée

# Shown beside a printer after it was removed.
settings-printers-removed = Retirée

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Écrivez d'abord l'adresse de l'imprimante.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Ajout…

# Shown beside a printer or the address line after it was added.
settings-printers-added = Ajoutée

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Recherche de son pilote…

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Travail { $number } sur { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = Annuler ce travail

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Travaux d'impression

# Help text (i button) of the Print jobs card.
# SAFETY: needs a human review before it ships
settings-printers-jobs-about = Tous les travaux en attente d'impression, sur n'importe quelle imprimante. Trindle met la liste à jour toutes les quelques secondes tant que cette page est ouverte. Les pages déjà imprimées ne peuvent pas être annulées.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS ne répond pas

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Impossible de lire la file d'attente.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Recherche…

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Rien en attente

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = En pause

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Inactive

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Impression

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Par défaut, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Définir comme imprimante par défaut

# Tip of the button that prints a test page.
settings-printers-test-page = Imprimer une page de test

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = Retirer cette imprimante

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = La recherche demande cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Pour l'installer, exécutez : sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Ajouter cette imprimante

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Rien de nouveau trouvé

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = L'ajouter

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Ajouter par adresse

# Heading of the card for adding printers.
settings-printers-add-card = Ajouter une imprimante

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle cherche d'abord les imprimantes qui s'annoncent d'elles-mêmes, puis interroge chaque appareil du réseau tour à tour. L'imprimante doit être allumée et sur le même réseau. Certaines imprimantes ne s'annoncent pas, et certains réseaux ne transmettent pas l'annonce. Dans ce cas, ajoutez l'imprimante par son adresse, par exemple 192.168.1.50. Vous trouverez l'adresse sur l'écran de l'imprimante elle-même, dans les réglages réseau.

# Name of the page in the list of settings pages.
settings-printers-title = Imprimantes

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = imprimante imprimer scanner numériseur

# Help text at the top of the page.
settings-printers-help = Trindle trouve tout seul les imprimantes de votre réseau. Pour en utiliser une, appuyez sur Ajouter cette imprimante à côté d'elle.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = L'impression n'est pas installée sur cet ordinateur.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Recherche…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle ne peut pas joindre le service d'impression, rien ne peut donc s'imprimer pour le moment.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Impression

# Help text (i button) of that card.
settings-printers-printing-about = L'impression demande CUPS, qui n'est pas sur cet ordinateur. Une fois installé, Trindle trouve tout seul les imprimantes de votre réseau.

# Line in that card.
settings-printers-unavailable = L'impression n'est pas disponible

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Installez-le avec : sudo apt install cups

# Line in that card.
settings-printers-then-found = Recherche des imprimantes

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Service d'impression

# Line in the Printers card when there are none.
settings-printers-none = Aucune imprimante pour le moment

# Heading of the card that lists the printers.
settings-printers-card = Imprimantes

# Help text (i button) of the Printers card.
# SAFETY: needs a human review before it ships
settings-printers-card-about = Toutes les imprimantes configurées sur cet ordinateur. Votre compte doit être dans le groupe lpadmin pour faire des changements.

# Red words under a page range box that cannot be used.
printers-bad-range = Par exemple : 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = De 1 à 99 copies.

# Name of the choice to print on one side of the paper.
printers-sides-one = Un côté

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Recto verso, bord long

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Recto verso, bord court

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
printers-orientation-portrait = Portrait

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Paysage

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = Action non autorisée. Votre compte doit être dans le groupe lpadmin. Ensuite, déconnectez-vous puis reconnectez-vous.

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS a refusé.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = Ce nom d'imprimante n'est pas autorisé.

# Red words when printing is not installed on the computer.
printers-not-installed = L'impression n'est pas installée sur cet ordinateur.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } n'est plus configurée.

# Red words when the printer chosen is paused.
printers-paused = { $name } est en pause.

# Red words when sending the document to CUPS took too long.
printers-too-slow = L'envoi du document a pris trop de temps.

# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle n'a pas pu annuler le travail.

# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle n'a pas pu en faire l'imprimante par défaut.

# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle n'a pas pu envoyer la page de test.

# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle n'a pas pu supprimer l'imprimante.

# Red words when a printer could not be added.
settings-printers-not-added = Trindle n'a pas pu ajouter l'imprimante.

# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle n'a pas pu lire la liste des imprimantes.
