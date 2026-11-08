# machine translated (French), not yet reviewed by a person.

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Autorisations

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = autorisations permissions autoriser confidentialité flatpak flathub bac à sable internet réseau fichiers son microphone caméra bluetooth notifications modules plugins deb x11 xwayland anciens programmes steam wine java écran terminal ordinateur thunderbolt usb4 station d'accueil appareils

# Help line at the foot of the Permissions page.
# SAFETY: needs a human review before it ships
settings-permissions-help = Ce que chaque application peut utiliser. Les applications de Flathub, les programmes installés dans leur propre dossier et les modules sont chacun installés dans leur propre dossier, avec des restrictions par défaut. Une icône pleine signifie autorisé ; cliquez dessus pour changer. Un changement pour une application de Flathub prend effet à son prochain démarrage. Les appareils Thunderbolt et USB4 autorisés se connectent toujours sans demander ; oubliez-en un pour qu'on vous le demande à nouveau.

# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = Cela n'a pas pu être modifié.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Enregistré, mais le module n'a pas pu être relancé. Le changement prend effet à son prochain démarrage.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Enregistré

# Help text of the AppImages card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-images = Chaque AppImage s'exécute dans une boîte à part, avec un dossier personnel privé. Une icône pleine signifie autorisé ; cliquez dessus pour changer. Un changement prend effet au prochain démarrage de l'AppImage.

# Help text of the Started from Folders card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-programs = Les programmes lancés depuis Dossiers s'exécutent chacun dans une boîte à part, avec un dossier personnel privé. Une icône pleine signifie autorisé ; cliquez dessus pour changer. Un changement prend effet au prochain démarrage du programme.

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title } : { $words }

# Under the Allow older programs switch.
# SAFETY: needs a human review before it ships
settings-permissions-older-why = Chacun est tenu à l'écart des autres. Désactivé, ils ne s'ouvrent pas.

# Heading of the card with the switch for older programs.
settings-permissions-older = Anciens programmes

# Label and tip of the switch that lets older programs open.
# SAFETY: needs a human review before it ships
settings-permissions-allow-older = Autoriser les anciens programmes

# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = Ces ports ne transportent que des écrans, aucun appareil ne peut donc atteindre la mémoire de cet ordinateur et il n'y a rien à demander.

# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Ces ports ne transportent que de l'USB et des écrans, aucun appareil ne peut donc atteindre la mémoire de cet ordinateur et il n'y a rien à demander.

# Help text on the Thunderbolt and USB4 card when the ports let every device in.
# SAFETY: needs a human review before it ships
settings-permissions-lets-in-all = Ces ports laissent tout appareil se connecter de lui-même, il n'y a donc rien à demander. Cela se règle dans le micrologiciel de l'ordinateur.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Ces ports ne transportent que des écrans.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Ces ports ne transportent que de l'USB et des écrans.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
# SAFETY: needs a human review before it ships
settings-permissions-short-firmware = Le micrologiciel laisse tout appareil se connecter.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Version { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Version { $v }. Il ne demande rien.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = Il n'a pas pu être oublié.

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt et USB4

# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
# SAFETY: needs a human review before it ships
settings-permissions-thunder-about = Les appareils Thunderbolt et USB4 autorisés se connectent toujours sans demander ; oubliez-en un pour qu'on vous le demande à nouveau. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Les ports

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Lecture de ce qu'ils laissent entrer…

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Appareils autorisés

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Lecture…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Mémoire tenue à l'écart

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = Aucun port Thunderbolt ou USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Il n'y a donc rien à demander.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Aucun : rien ne se branche de cette façon.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Rien à demander

# Under Devices allowed when the ports let every device in themselves.
# SAFETY: needs a human review before it ships
settings-permissions-every-one = Tous, par les ports eux-mêmes.

# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-iommu = Cet ordinateur tient aussi les appareils à l'écart de la mémoire qui ne leur a pas été donnée, le risque est donc plus faible. Oublier un appareil le laisse connecté jusqu'à ce qu'il soit débranché.

# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
# SAFETY: needs a human review before it ships
settings-permissions-no-iommu = Oublier un appareil le laisse connecté jusqu'à ce qu'il soit débranché ; la fois suivante, on vous le demandera à nouveau.

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
# SAFETY: needs a human review before it ships
settings-permissions-asked-before = Demandé avant qu'un nouvel appareil se connecte

# Under the heading above.
# SAFETY: needs a human review before it ships
settings-permissions-forget-one = Oubliez-en un pour qu'on vous le demande à nouveau.

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Les appareils autorisés n'ont pas pu être lus : { $why }

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
settings-permissions-reading-apps = Lecture de ce que chaque application peut utiliser…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Lecture de ce que chacune des { $n } peut utiliser…
        [many] Lecture de ce que chacune des { $n } peut utiliser…
       *[other] Lecture de ce que chacune des { $n } peut utiliser…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = Aucune application ni aucun module tenu dans une boîte à part n'est installé.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Une application est tenue dans une boîte à part et n'utilise que ce qui est autorisé ici.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } application ou module est tenu dans une boîte à part, et n'utilise que ce qui est autorisé ici.
        [many] { $n } d'applications et de modules sont chacun tenus dans une boîte à part, et n'utilisent que ce qui est autorisé ici.
       *[other] { $n } applications et modules sont chacun tenus dans une boîte à part, et n'utilisent que ce qui est autorisé ici.
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
settings-permissions-from-folders = Lancés depuis Dossiers

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = Aucun d'installé.

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = Aucun lancé pour le moment.

# Help text of the Flathub apps card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-flathub-about = Les applications de Flathub sont chacune tenues dans une boîte à part et ne peuvent utiliser que ce qui est autorisé ici. Une icône pleine signifie autorisé ; cliquez dessus pour changer. Un changement prend effet au prochain démarrage de l'application.

# Help text of the In their own folder card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-debs-about = Les programmes installés dans leur propre dossier sont restreints par défaut. Une icône pleine signifie autorisé ; cliquez dessus pour changer.

# Help text of the Plugins card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-plugins-about = Les modules sont restreints par défaut et ne peuvent recevoir que ce qu'ils demandent. Un module qui tourne en arrière-plan redémarre aussitôt avec le changement.

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Codes de saisie

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Codes courts que Trindle remplace par des mots pendant que vous tapez, avec le module qui les gère. Désactivez les codes d'un module avec son icône ronde.

settings-permissions-codes-none = Aucun code n'est actif

# SAFETY: needs a human review before it ships
settings-permissions-codes-none-sub = Un module autorisé à développer des codes les liste ici.

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
settings-permissions-codes-unread-line = Le dossier des extensions n'a pas pu être lu

# Red words under that line.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread = Le dossier des extensions n'a pas pu être lu, donc certains codes peuvent manquer.

settings-permissions-not-added = Lancée depuis Folders, non ajoutée à Apps

settings-permissions-kind-tip = Type d'application



# Heading of the card for programs that are not in a box.
settings-outside-heading = Programmes hors d'une boîte

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle regarde toutes les quelques secondes et vous prévient. Il ne peut pas arrêter ces programmes, et il peut en rater un très rapide. Un programme dans une boîte est retenu par sa boîte.

# Label of the switch for the notice about the network.
settings-outside-network = Prévenir quand un programme accède au réseau

# Label of the switch for the notice about the person's files.
settings-outside-files = Prévenir quand un programme a vos fichiers ouverts

# Under those two switches.
settings-outside-once = Une seule fois par programme.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Prévenir à nouveau pour { $name }

# Under a program marked fine for the network.
settings-outside-fine-network = L'accès au réseau est autorisé.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Avoir vos fichiers ouverts est autorisé.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Aucun programme n'est marqué comme autorisé.

# Heading of the card for downloaded files.
settings-downloads-heading = Fichiers téléchargés

# Help text of that card (the information button).
settings-downloads-about = Un fichier venu d'internet est un fichier de Téléchargements, ou un fichier marqué par le navigateur. Le programme qui l'ouvre reçoit ce fichier seulement, en lecture seule, sans internet et sans aucun de vos dossiers. Cela ne couvre pas le bouton Ouvrir propre à un navigateur, qui lance lui-même le programme.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Ouvrir les fichiers téléchargés dans une boîte

# Under that switch.
settings-downloads-box-sub = Sans internet, et seulement ce fichier, en lecture seule.

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Ouvrir les fichiers { $kind } normalement

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Fichiers { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Activé : s'ouvre normalement, avec internet.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Aucun type pour l'instant. Un type apparaît ici quand un fichier téléchargé s'ouvre dans une boîte.

