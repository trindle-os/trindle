
# machine translated (French), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Bienvenue dans Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Ouvrir l'anneau

# Title of the third page, where the first apps are picked.
welcome-page-apps = Choisissez vos applications

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Encore du choix

# Title of the fifth page, about plugins.
welcome-page-plugins = Les extensions ajoutent plus

# Title of the sixth page, about crash reports.
welcome-page-reports = Aidez à corriger Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Les points

# Title of the last page, with a few things worth knowing.
welcome-page-know = Bon à savoir

# Sentence under the title of the first page.
welcome-sentence-hello = Tout ce dont vous avez besoin tient dans un cercle, l'anneau.

# Sentence under the title of the second page.
welcome-sentence-ring = Double-cliquez tout au bord de l'écran, ou appuyez sur la touche Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Presque rien n'est encore installé, choisissez donc les applications que vous voulez.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Choisissez-en d'autres, puis installez tout ce que vous avez choisi en un clic.

# Sentence under the title of the fifth page.
welcome-sentence-plugins = Les extensions sont de petits extras venant d'Apps, et chacune ne fait que ce que vous autorisez.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Envoyez un rapport quand quelque chose ne va pas, si vous le souhaitez.

# Sentence under the title of the dots page.
welcome-sentence-dots = Un point signifie qu'un programme utilise quelque chose qui vous appartient.

# Sentence under the title of the last page.
welcome-sentence-know = Quelques points qui rendent Trindle différent.

# Top right of every page: where the person is in the tour.
welcome-step = Étape { $n } sur { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Étape { $n } sur { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = La visite de bienvenue a été vue. Elle est dans Toutes les applications sous le nom Welcome ; supprimez ce fichier pour la revoir à la prochaine connexion.

# Name of the round button at the foot that closes the tour.
welcome-close = Fermer

# Name of the round button at the foot that goes to the page before.
welcome-back = Retour

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Suivant

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Installer les applications choisies

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Commencer à utiliser Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Connectez-vous à Internet pour installer des applications.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Réessayer

# Foot while apps are going in: their names.
welcome-installing = Installation de { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Installation de { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } est maintenant dans l'anneau.
       *[other] { $names } sont maintenant dans l'anneau.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Les applications n'ont pas pu être installées. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak a refusé.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Choisies : { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } et { $more } de plus

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = L'anneau

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Ce qu'il contient

# Name of the first thing the ring holds.
welcome-held-programs = Vos programmes

# Line under it.
welcome-held-programs-line = Sont placés tout autour.

# Name of the second thing the ring holds.
welcome-held-time = L'heure et la date

# Line under it.
welcome-held-time-line = Sont placées au milieu.

# Name of the third thing the ring holds.
welcome-held-switches = Les interrupteurs

# Line under it.
welcome-held-switches-line = Son, écran, maintien en éveil et alimentation.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Avec le pointeur

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Avec le clavier

# Note under the picture of the Super key.
welcome-ring-super-note = La touche Super porte souvent un logo Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Navigateur web

# Head of a card of apps: email programs.
welcome-kind-email = Courriel

# Head of a card of apps: chat programs.
welcome-kind-chat = Messagerie

# Head of a card of apps: office programs.
welcome-kind-office = Bureautique

# Head of a card of apps: music and video players.
welcome-kind-media = Musique et vidéo

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Images et dessin

# Head of a card of apps: notes and writing.
welcome-kind-notes = Notes et écriture

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF et documents

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Aucune de celles-ci n'est sur Flathub pour le moment.

# Beside an app that has been installed.
welcome-app-installed = Installée

# Beside an app that is going in now.
welcome-app-installing = Installation

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, choisie

# Name of the round button on an app's row that picks it.
welcome-app-pick = Choisir { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = Dans l'anneau

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Les extensions

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = Dans Apps, sous Extensions.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Ouvrir les extensions dans Apps

# Said in the plugins card while there are none to show.
welcome-plugins-none = Les extensions proposées sont listées dans Apps.

# SAFETY: needs a human review before it ships
# Head of the card on the crash reports page.
welcome-reports-card = Rapports de plantage

# SAFETY: needs a human review before it ships
# The one question about crash reports.
welcome-reports-question = Aider à corriger Trindle en envoyant un rapport quand quelque chose ne va pas ? Vous le voyez toujours d'abord.

# SAFETY: needs a human review before it ships
# Note under the question about crash reports.
welcome-reports-note = Rien n'est envoyé sans que vous appuyiez. Vous voyez tout le rapport d'abord, et il ne contient ni fichiers, ni mots de passe, ni noms de Wi-Fi.

# SAFETY: needs a human review before it ships
# Second note under the question about crash reports.
welcome-reports-settings = Vous pouvez changer cela à tout moment dans Réglages, Rapports de plantage.

# SAFETY: needs a human review before it ships
# The answer that turns crash reports off.
welcome-reports-no = Non

# SAFETY: needs a human review before it ships
# The answer that turns crash reports on.
welcome-reports-yes = Oui, me demander

# SAFETY: needs a human review before it ships
# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Envoyer un rapport quand quelque chose ne va pas

# Head of a card on the last page.
welcome-know-boxes = Les applications vivent dans leurs propres boîtes

# SAFETY: needs a human review before it ships
# Line in that card.
welcome-know-boxes-line = Une application demande avant d'accéder à vos fichiers, à la caméra ou au microphone.

# Head of a card on the last page.
welcome-know-updates = Les mises à jour de sécurité s'installent toutes seules

# Line in that card.
welcome-know-updates-line = Chaque jour, sans rien demander. Les autres mises à jour vous attendent.

# Head of a card on the last page.
welcome-know-ring = Réglages et Apps sont dans l'anneau

# Line in that card.
welcome-know-ring-line = En son milieu, au-dessus de l'heure.

# Head of a card on the last page.
welcome-know-help = L'aide est dans Réglages

# Line in that card.
welcome-know-help-line = Cette visite est dans Toutes les applications sous le nom Welcome, quand vous voulez la revoir.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Un petit écran sans rien dessus que l'anneau au milieu : l'heure en son centre, les applications favorites autour.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = L'anneau ouvert sur un petit écran, avec les icônes de quelques extensions alignées en son milieu, sous l'heure, là où l'icône d'une extension se place une fois installée.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Deux petits dessins de l'endroit où les points s'affichent : sur l'onglet d'une fenêtre dans l'anneau, et en haut au centre de l'écran.

welcome-dots-where-tab = Sur l'onglet d'une fenêtre

welcome-dots-where-corner = En haut au centre de l'écran

welcome-picture-ring = Un petit écran. Le pointeur se déplace tout en haut au bord et double-clique, et l'anneau s'ouvre au milieu.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = La touche Super est pressée, ce qui ouvre l'anneau.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Que savez-vous déjà ?

# Sentence under that title.
welcome-sentence-level = Choisissez le plus proche, et la visite laisse de côté ce que vous savez.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Nouveau sous Linux

# Line in that answer's card.
welcome-level-new-line = Chaque page, y compris ce que fait tout bureau Linux.

# Name of the answer for someone who has used Linux.
welcome-level-linux = J'ai déjà utilisé Linux

# Line in that answer's card.
welcome-level-linux-line = Seulement ce qui est propre à Trindle. Les bases de Linux sont passées.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = J'ai déjà utilisé Trindle

# Line in that answer's card.
welcome-level-trindle-line = Ferme la visite maintenant et ne la montre plus.

# Title of the page about files and folders.
welcome-page-files = Vos fichiers

# Sentence under that title.
welcome-sentence-files = Tout ce qui est à vous se trouve dans votre dossier personnel.

# Title of the page about getting software.
welcome-page-software = Obtenir des logiciels

# Sentence under that title.
welcome-sentence-software = Les applications viennent d'Applications, pas de fichiers d'installation à télécharger.

# Title of the page about the administrator password.
welcome-page-admin = Le mot de passe de l'administrateur

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Les changements touchant tout l'ordinateur demandent d'abord un mot de passe.

# Title of the page about snapshots.
welcome-page-snapshots = Instantanés

# Sentence under that title.
welcome-sentence-snapshots = Trindle garde des copies du système, pour qu'une mise à jour puisse être annulée.

# Title of the page about the person's own words.
welcome-page-words = Vos propres mots

# Sentence under that title.
welcome-sentence-words = Changez n'importe quel mot que Trindle affiche, ou ajoutez une langue à vous.

# Head of a card on the files page.
welcome-files-home = Un dossier personnel

# Line in that card.
welcome-files-home-line = Documents, Téléchargements et Images s'y trouvent. Dossiers s'ouvre à cet endroit.

# Head of a card on the files page.
welcome-files-drives = Pas de lettres de lecteur

# Line in that card.
welcome-files-drives-line = Une clé USB ou un autre disque apparaît dans Dossiers dès que vous le branchez.

# Head of a card on the files page.
welcome-files-hidden = Noms cachés

# Line in that card.
welcome-files-hidden-line = Un nom commençant par un point, comme .config, est caché. Il contient les réglages d'une application.

# Head of a card on the files page.
# SAFETY: needs a human review before it ships
welcome-files-bin = La corbeille

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Un fichier supprimé attend dans la corbeille jusqu'à ce que vous la vidiez.

# Head of a card on the software page.
welcome-software-apps = Un seul endroit pour les applications

# Line in that card.
welcome-software-apps-line = Ouvrez Applications depuis l'anneau pour trouver, installer et retirer des programmes.

# Head of a card on the software page.
welcome-software-exe = Pas de fichiers d'installation

# Line in that card.
welcome-software-exe-line = Un fichier d'installation Windows, finissant par .exe, ne s'exécute pas ici. Cherchez l'application dans Applications.

# Head of a card on the software page.
welcome-software-updates = Les mises à jour au même endroit

# Line in that card.
welcome-software-updates-line = Applications et Réglages maintiennent tout à jour, donc aucune application n'a besoin de son propre outil de mise à jour.

# Head of a card on the software page.
# SAFETY: needs a human review before it ships
welcome-software-remove = Un retrait propre

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Retirer une application l'enlève entièrement et ne laisse rien derrière.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Pourquoi il le demande

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Installer pour tout le monde ou modifier un disque touche chaque utilisateur, donc l'ordinateur demande d'abord.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = Le mot de passe de qui

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = Celui d'un administrateur, en général celui avec lequel vous ouvrez votre session.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Quand il ne le demande pas

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Les applications que vous installez pour vous seul et vos propres fichiers n'en ont jamais besoin.

# Head of a card on the snapshots page.
welcome-snap-taken = Faits pour vous

# Line in that card.
welcome-snap-taken-line = Sur un système Btrfs, un instantané est fait avant et après chaque changement de logiciel.

# Head of a card on the snapshots page.
welcome-snap-list = Listés dans les réglages

# Line in that card.
welcome-snap-list-line = Réglages, Instantanés les liste. L'appareil photo en fait un tout de suite.

# Head of a card on the snapshots page.
welcome-snap-boot = Utilisés si le système ne démarre pas

# Line in that card.
welcome-snap-boot-line = Choisissez Trindle snapshots dans le menu de démarrage, puis un instantané.

# Head of a card on the your own words page.
welcome-words-change = Changer un mot

# Line in that card.
welcome-words-change-line = Trouvez n'importe quel mot que Trindle affiche et tapez le vôtre. « Ouvrez-le, hop » peut remplacer « Cliquez ici pour ouvrir le fichier ».

# Head of a card on the your own words page.
welcome-words-share = Partager une feuille

# Line in that card.
welcome-words-share-line = Exportez vos mots dans un fichier, ou importez celui d'un ami. Il est vérifié avant d'être utilisé.

# Head of a card on the your own words page.
welcome-words-all = Tous les programmes à la fois

# Line in that card.
welcome-words-all-line = Tout Trindle lit ses mots au même endroit, donc un seul changement se voit partout.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = C'est dans Réglages, Région et langue. L'aide l'explique pas à pas.

# Name of the round button that opens that Settings page.
welcome-words-open = Ouvrir Région et langue
