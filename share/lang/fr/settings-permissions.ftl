# machine translated (French), not yet reviewed by a person.

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Autorisations

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = autorisations permissions autoriser confidentialité flatpak flathub bac à sable internet réseau fichiers son microphone caméra bluetooth notifications modules plugins deb x11 xwayland anciens programmes steam wine java écran terminal ordinateur thunderbolt usb4 station d'accueil appareils

# Help line at the foot of the Permissions page.
# SAFETY: needs a human review before it ships
settings-permissions-help = Cette page montre ce que chaque application peut utiliser. Les applications de Flathub, les programmes installés dans leur propre dossier et les modules s'exécutent chacun dans leur propre boîte, avec des limites fixées par défaut. Un changement pour une application de Flathub prend effet à son prochain démarrage. Les appareils Thunderbolt et USB4 que vous avez autorisés se connectent toujours sans demander. Oubliez un appareil si vous voulez qu'on vous le demande à nouveau.

# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = Impossible de le modifier.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Enregistré, mais Trindle n'a pas pu relancer le module. Le changement prend effet à son prochain démarrage.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Enregistré

# Help text of the AppImages card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-images = Chaque AppImage s'exécute dans sa propre boîte, avec un dossier personnel privé. Un changement prend effet au prochain démarrage de l'AppImage.

# Help text of the Started from Folders card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-programs = Chaque programme que vous lancez depuis Folders s'exécute dans sa propre boîte, avec un dossier personnel privé. Un changement prend effet au prochain démarrage du programme.

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title } : { $words }

# Under the Allow older programs switch.
# SAFETY: needs a human review before it ships
settings-permissions-older-why = Chacun s'exécute séparément. Si c'est désactivé, ils ne s'ouvriront pas.

# Heading of the card with the switch for older programs.
settings-permissions-older = Anciens programmes

# Label and tip of the switch that lets older programs open.
# SAFETY: needs a human review before it ships
settings-permissions-allow-older = Autoriser les anciens programmes

# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = Ces ports ne servent qu'aux écrans, aucun appareil ne peut donc atteindre la mémoire de cet ordinateur. Il n'y a rien à demander.

# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Ces ports ne servent qu'à l'USB et aux écrans, aucun appareil ne peut donc atteindre la mémoire de cet ordinateur. Il n'y a rien à demander.

# Help text on the Thunderbolt and USB4 card when the ports let every device in.
# SAFETY: needs a human review before it ships
settings-permissions-lets-in-all = Ces ports laissent tout appareil se connecter, il n'y a donc rien à demander. Cela se règle dans le micrologiciel de l'ordinateur.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Seuls des écrans utilisent ces ports.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Seuls l'USB et les écrans utilisent ces ports.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
# SAFETY: needs a human review before it ships
settings-permissions-short-firmware = Le micrologiciel laisse tout appareil se connecter.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Version { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Version { $v }. Il ne demande rien.

# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = Version { $v } : { $notes }.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = Impossible de l'oublier.

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt et USB4

# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
# SAFETY: needs a human review before it ships
settings-permissions-thunder-about = Les appareils Thunderbolt et USB4 que vous avez autorisés se connectent toujours sans demander. Oubliez un appareil si vous voulez qu'on vous le demande à nouveau. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Ports

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Vérification des ports…

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Appareils autorisés

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Chargement…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Protection de la mémoire

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = Aucun port Thunderbolt ou USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Il n'y a donc rien à demander.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Aucun appareil ne se connecte de cette façon.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Rien à demander

# Under Devices allowed when the ports let every device in themselves.
# SAFETY: needs a human review before it ships
settings-permissions-every-one = Tous. Ce sont les ports qui décident.

# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-iommu = Cet ordinateur empêche aussi les appareils d'atteindre une mémoire qui n'est pas la leur, le risque est donc plus faible. Si vous oubliez un appareil, il reste connecté jusqu'à ce que vous le débranchiez.

# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-no-iommu = Si vous oubliez un appareil, il reste connecté jusqu'à ce que vous le débranchiez. La prochaine fois que vous le brancherez, on vous le demandera à nouveau.

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
# SAFETY: needs a human review before it ships
settings-permissions-asked-before = On vous demande pour les nouveaux appareils

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Trindle n'a pas pu lire la liste des appareils autorisés : { $why }

# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = Aucun appareil n'a encore le droit de se connecter sans demander.

# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
# SAFETY: needs a human review before it ships
settings-permissions-forget-device = Oublier cet appareil

# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = Un appareil Thunderbolt

# Under an allowed device's name when its maker is not known.
# SAFETY: needs a human review before it ships
settings-permissions-may-always = Peut toujours se connecter

# Under an allowed device's name: the maker is { $vendor }.
# SAFETY: needs a human review before it ships
settings-permissions-made-by = Fabriqué par { $vendor }. Peut toujours se connecter.

# Headline while the list of apps is being read.
settings-permissions-reading-apps = Vérification de ce que les applications peuvent utiliser…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Vérification de ce que { $n } application peut utiliser…
        [many] Vérification de ce que { $n } d'applications peuvent utiliser…
       *[other] Vérification de ce que { $n } applications peuvent utiliser…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = Vous n'avez installé aucune application ni aucun module qui s'exécute dans sa propre boîte.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Une application s'exécute dans sa propre boîte et ne peut utiliser que ce que vous autorisez ici.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } application ou module s'exécute dans sa propre boîte et ne peut utiliser que ce que vous autorisez ici.
        [many] { $n } d'applications et de modules s'exécutent chacun dans leur propre boîte et ne peuvent utiliser que ce que vous autorisez ici.
       *[other] { $n } applications et modules s'exécutent chacun dans leur propre boîte et ne peuvent utiliser que ce que vous autorisez ici.
    }

# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Applications Flathub

# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = Dans leur propre dossier

# Heading of the card for plugins.
settings-permissions-plugins = Modules

# Heading of the card for AppImages.
settings-permissions-appimages = AppImages

# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Lancés depuis Folders

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = Aucun d'installé.

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = Aucun pour le moment.

# Help text of the Flathub apps card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-flathub-about = Chaque application de Flathub s'exécute dans sa propre boîte et ne peut utiliser que ce qui est autorisé ici. Un changement prend effet au prochain démarrage de l'application.

# Help text of the In their own folder card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-debs-about = Les programmes fournis avec Trindle, comme le Terminal, et les programmes que vous avez installés à partir de fichiers .deb. Chacun s'exécute dans son propre dossier, avec des limites fixées par défaut.

# Help text of the Plugins card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-plugins-about = Les modules démarrent avec des limites, et vous ne pouvez autoriser que ce qu'un module demande. Un module qui tourne en arrière-plan redémarre aussitôt quand vous changez quelque chose.

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Codes de saisie

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Codes courts que Trindle remplace par des mots pendant que vous tapez. Chacun indique le module dont il vient.

settings-permissions-codes-none = Aucun code n'est actif

# Button on a plugin's row: stops that plugin's codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-off = Désactiver les codes de { $name }

settings-permissions-codes-count =
    { $n ->
        [one] Un code, { $id }
        [many] { $n } de codes, { $id }
       *[other] { $n } codes, { $id }
    }

# A code's second line: what it types and who owns it.
settings-permissions-code-types = Tape « { $words } » pour { $name }

settings-permissions-codes-more =
    { $n ->
        [one] Un autre code est actif
        [many] { $n } d'autres codes sont actifs
       *[other] { $n } autres codes sont actifs
    }

# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread-line = Impossible de lire le dossier des modules

# Red words under that line.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread = Trindle n'a pas pu lire le dossier des modules, donc certains codes peuvent manquer.

settings-permissions-not-added = Ouverte depuis Folders, pas dans Apps

settings-permissions-kind-tip = Type d'application



# Heading of the card for programs that are not in a box.
settings-outside-heading = Programmes hors d'une boîte

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle regarde toutes les quelques secondes et vous dit ce qu'il trouve. Il ne peut pas arrêter ces programmes, et il peut en rater un qui ne tourne qu'un instant. Un programme dans une boîte ne peut utiliser que ce que sa boîte autorise.

# Label of the switch for the notice about the network.
settings-outside-network = Me prévenir quand des programmes utilisent le réseau

# Label of the switch for the notice about the person's files.
settings-outside-files = Me prévenir quand des programmes ouvrent mes fichiers

# Under those two switches.
settings-outside-once = Une seule fois par programme.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Ne plus ignorer { $name }

# Under a program marked fine for the network.
settings-outside-fine-network = L'accès au réseau est autorisé.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Avoir vos fichiers ouverts est autorisé.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Aucun programme n'est marqué comme autorisé.

# Heading of the card for downloaded files.
settings-downloads-heading = Fichiers téléchargés

# Help text of that card (the information button).
settings-downloads-about = Les fichiers de Téléchargements, et les fichiers que votre navigateur a marqués comme téléchargés, s'ouvrent dans une boîte. Le programme qui en ouvre un reçoit uniquement ce fichier. Il ne peut que le lire, et il ne peut utiliser ni internet ni vos dossiers. Cela ne couvre pas le bouton Ouvrir propre à un navigateur, qui lance lui-même le programme.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Ouvrir les fichiers téléchargés dans une boîte

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Ouvrir les fichiers { $kind } normalement

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Fichiers { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Ils s'ouvrent normalement, avec internet.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Aucun type de fichier pour l'instant. Un type apparaît ici dès qu'un fichier téléchargé s'est ouvert dans une boîte.
