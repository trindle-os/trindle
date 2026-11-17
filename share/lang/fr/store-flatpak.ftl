# machine translated (French), not yet reviewed by a person.

# The page shown before a Flatpak program is installed, with what it will be able to reach.
# Headline over the page; the source of the program is given.
store-fp-headline-from = Installer { $name } depuis { $host } ?

# Headline over the page for a program that has been downloaded.
store-fp-headline = Installer { $name } ?

# Name of the card that lists what the program will be able to reach.
store-fp-card = Ce à quoi elle pourra accéder

# Tooltip of the round button that leaves without installing.
store-fp-cancel = Ne pas installer

# The main button.
store-fp-install = Installer

# Said while Flatpak is being asked what the program wants.
store-fp-reading = Lecture de ce qu'elle demande…

# Red words when what the program asks for could not be read.
store-fp-unread = Trindle n'a pas pu lire ce qu'elle demande.

# Red words after Install was pressed once for a program whose list could not be read.
store-fp-unread-again = Appuyez de nouveau sur Installer pour continuer.

# Said when the program is already on this computer; the name is given.
store-fp-have = { $name } est déjà installée.

# Said when the program asks to reach nothing outside its own box.
store-fp-none = Rien en dehors de sa propre boîte.

# Each line below is one thing the program will be able to reach.
store-fp-network = Internet et votre réseau local

store-fp-x11 = Les fenêtres des autres applications, et ce que vous y tapez

store-fp-sound = Le son et votre microphone

store-fp-devices = Votre caméra et tous les autres appareils branchés

store-fp-input = Les manettes de jeu et autres périphériques de saisie

store-fp-kvm = Les machines virtuelles rapides

store-fp-shm = La mémoire partagée avec d'autres applications

store-fp-usb = Les appareils USB

store-fp-canbus = Les appareils sur bus CAN

store-fp-cups = Vos imprimantes

store-fp-multiarch = L'exécution de programmes 32 bits

store-fp-escape = Lancer n'importe quel programme sur cet ordinateur

# A reference that also adds a second place to get apps from, named by its address.
store-fp-adds-source = Ajouter { $host } à vos sources d'applis

# Last line, when the program asks for more than the lines above say.
store-fp-other = Et d'autres réglages

store-fp-bluetooth = Les appareils Bluetooth

store-fp-smartcard = Les cartes à puce

store-fp-ssh = Vos clés SSH

store-fp-gpg = Vos clés GPG

store-fp-devel = Les outils de débogage, qui lui permettent d'observer et de contrôler les programmes qu'elle lance

store-fp-session-bus = Tous les programmes de votre session

store-fp-system-bus = Tous les services de cet ordinateur

# A place the program can open and change; the place is given, as below.
store-fp-files = Ouvrir et modifier { $what }

# The same for a place it can only look at.
store-fp-files-read = Consulter { $what }

store-fp-where-host = tous les fichiers de cet ordinateur

store-fp-where-system = les fichiers système de cet ordinateur

store-fp-where-home = tout votre dossier personnel, fichiers cachés compris

store-fp-where-path = le dossier { $path }

# A program on your session that it can talk to; the name is given.
store-fp-talk = Communiquer avec { $name }

# A service the program runs on your session.
store-fp-own = Lancer le service { $name }

store-fp-talk-system = Communiquer avec le service système { $name }

store-fp-own-system = Lancer le service système { $name }

# Said in red when Install failed and taking the program out again failed too.
store-fp-undo-failed = Elle est peut-être installée en partie.

# Said in red when a bundle could not be copied for a safe read.
store-fp-copy-failed = Impossible de copier le fichier pour le lire en toute sécurité.
