
# machine translated (French), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Enregistré

# Said beside a button that worked, when nothing more is known.
settings-common-done = Terminé

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Non effectué

# Tip of the information button at the top of every Settings page.
settings-common-about-page = À propos de cette page

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = À propos de { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } existe déjà

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Faire de même pour le reste ({ $n } de plus)
        [many] Faire de même pour le reste ({ $n } de plus)
       *[other] Faire de même pour le reste ({ $n } de plus)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Faire de même pour le reste

# Tip of the button that keeps the one already there.
settings-common-skip = Ignorer : garder celui qui existe déjà

# SAFETY: needs a human review before it ships
# Button that replaces the one already there.
settings-common-replace = Remplacer

# SAFETY: needs a human review before it ships
# Spoken name of the Replace button.
settings-common-replace-spoken = Remplacer celui qui existe déjà par celui qui arrive

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Celui qui arrive

# Heading of the line that shows the one already there.
settings-common-already-line = Celui qui existe déjà

# SAFETY: needs a human review before it ships
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = Remplacer celui qui existe déjà ?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Retour en arrière dans { $left } seconde.
        [many] Retour en arrière dans { $left } secondes.
       *[other] Retour en arrière dans { $left } secondes.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Retour en arrière dans { $left } seconde. { $more }
        [many] Retour en arrière dans { $left } secondes. { $more }
       *[other] Retour en arrière dans { $left } secondes. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Revenir en arrière maintenant

# SAFETY: needs a human review before it ships
# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Garder

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } n'est pas disponible

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Installez-le avec : sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = La page web n'a pas pu être ouverte : aucun navigateur web n'a pu être lancé.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } n'a pas pu être ouvert.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = L'enregistrement a échoué.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Non effectué : le bus de messages du système est inaccessible. Redémarrez l'ordinateur, puis réessayez.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Non effectué : le bus de messages de cette session est inaccessible. Fermez la session puis rouvrez-la, et réessayez.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = La fenêtre de sélection n'a pas pu être ouverte. Fermez la session puis rouvrez-la, et réessayez.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Le disque est plein.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Il est impossible d'écrire sur le disque pour le moment.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = La partie du système qui s'en occupe n'est pas en cours d'exécution.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = La réponse a pris trop de temps. Réessayez.

# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = Le mot de passe n'a pas été fourni.

# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = Ce compte n'y est pas autorisé.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Un élément nécessaire est absent de cet ordinateur.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } et { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = L'anneau s'ouvre d'un clic au bord de l'écran.

# Word drawn in the middle of that picture.
settings-common-ring-name = L'anneau

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Impossible d'accéder au micrologiciel de l'ordinateur portable : { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Le micrologiciel de l'ordinateur portable n'a pas accepté la limite de charge.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Cet ordinateur ne permet pas d'arrêter la charge avant la fin.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Impossible de définir { $path } : { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Essayer Trindle dans une machine virtuelle

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle fonctionne dans { $machine }. Cela marche bien, mais une machine virtuelle n'est jamais tout à fait aussi rapide ni aussi fluide qu'un vrai ordinateur : jugez donc sa vitesse sur du matériel réel.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle fonctionne dans { $machine } sans graphisme 3D : le processeur doit donc dessiner tout ce qu'une puce graphique dessine normalement. Les fenêtres, les vidéos et les pages web seront plus lentes que sur un vrai ordinateur, et Trindle supprime ses animations pour aider. Si l'ordinateur qui l'héberge le permet, activer l'accélération 3D dans les réglages d'affichage de la machine virtuelle change beaucoup de choses. Installé sur un vrai ordinateur, rien de tout cela ne s'applique.
