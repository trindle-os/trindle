
# machine translated (French), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = À propos de cet ordinateur

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space nom ordinateur réseau système mémoire processeur disque écran noyau activité programmes moniteur disques espace

# Text behind the info button of the About page.
settings-about-help = Le nom de cet ordinateur, tel que les autres ordinateurs du réseau le voient, de quoi il est fait, et combien de temps il met à démarrer. Ce qu'il fait en ce moment, et quels programmes utilisent la mémoire et le processeur, se trouve dans Activité, et les disques et leur espace dans Disques, chacun dans sa propre fenêtre : les flèches de Plus de détails les ouvrent.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Inconnu

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } thread
        [many] { $model }, { $cores } threads
       *[other] { $model }, { $cores } threads
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Graphisme Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Graphisme AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Graphisme NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Graphisme de machine virtuelle

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Graphisme

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } à { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } utilisés

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } libres

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = sur { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, allumé depuis { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, allumé depuis { $on }. Le disque est presque plein.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } secondes

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } minutes

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } minute
        [many] { $n } minutes
       *[other] { $n } minutes
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } heure
        [many] { $n } heures
       *[other] { $n } heures
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } jour
        [many] { $n } jours
       *[other] { $n } jours
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Lettres, chiffres et traits d'union uniquement, sans commencer ni finir par un trait d'union.

# Words under the computer name box while the password window is open.
settings-about-name-waiting = En attente du mot de passe...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = L'ordinateur n'a pas pu être renommé.

# Words under the computer name box after renaming worked.
settings-about-name-done = Les programmes déjà ouverts verront le nouveau nom à la prochaine ouverture de session.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nom de l'ordinateur

# SAFETY: needs a human review before it ships
# Tip of the save button beside the computer name box.
settings-about-rename = Renommer cet ordinateur

# Bold line of the computer name setting.
settings-about-name = Nom de l'ordinateur

# Quiet line under the computer name setting.
settings-about-name-sub = Lettres, chiffres et traits d'union.

# Heading of the card with the computer's name.
settings-about-name-group = Nom

# Text behind the info button of the Name card.
settings-about-name-about = Comment cet ordinateur apparaît sur le réseau, et à l'invite d'un Terminal. Lettres, chiffres et traits d'union, sans commencer ni finir par un trait d'union. Le modifier demande votre mot de passe.

# Heading of the card about the computer's hardware.
settings-about-hardware = L'ordinateur

# Text behind the info button of the hardware card.
settings-about-hardware-about = Le fabricant et le modèle, le processeur et le nombre de threads qu'il exécute à la fois, le graphisme, et chaque écran utilisé avec sa taille et sa fréquence de rafraîchissement.

# Quiet line under the maker and model.
settings-about-hw-computer = L'ordinateur

# Quiet line under the processor.
settings-about-hw-processor = Le processeur

# Quiet line under the graphics.
settings-about-hw-graphics = Dessine les images

# Quiet line under the screens.
settings-about-hw-screen = L'écran

# Heading of the card about the software.
settings-about-software = Le logiciel

# Text behind the info button of the software card.
settings-about-software-about = Trindle est un bureau construit sur Debian, qui s'occupe des programmes en dessous. Le noyau est la partie qui dialogue avec le matériel, et le compositeur propre à Trindle dessine le bureau, sous Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Fonctionne en dessous

# Quiet line under the kernel version.
settings-about-sw-kernel = Le noyau

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Le compositeur propre à Trindle

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Dessine le bureau, sous Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } application
        [many] { $n } applications
       *[other] { $n } applications
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Installées, prêtes dans l'anneau

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Plus de détails

# Text behind the info button of the More detail card.
settings-about-more-about = Ce qu'il fait en ce moment, et quels programmes utilisent la mémoire et le processeur, se trouve dans Activité. Chaque disque, son remplissage et son état de santé, ainsi que les clés USB, se trouvent dans Disques.

# Bold line of the setting that opens Activity.
settings-about-activity = Voir ce qui s'exécute

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Programmes utilisant la mémoire et le processeur.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Activité

# Bold line of the setting that opens Drives.
settings-about-drives = Disques et espace

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Chaque disque, son remplissage et son état de santé.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Disques

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Les temps de démarrage n'ont pas pu être enregistrés dans Downloads.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Dans Downloads, sous le nom { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Enregistrer

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Enregistrer tous les temps de démarrage dans Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Tous les démarrages, à conserver

# Quiet line under that setting.
settings-about-times-keep-sub = Enregistrés dans Downloads sous forme de feuille de calcul.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Chaque colonne est un démarrage, le plus récent à droite.

# Heading of the card with the last ten start-up times.
settings-about-times = Les dix derniers démarrages

# Text behind the info button of the start-up times card.
settings-about-times-about = Chaque démarrage est chronométré du bouton d'alimentation à l'écran de connexion, sans compter le temps passé à saisir le mot de passe du disque. Le plus récent est en haut.

# Heading of the memory tile.
settings-about-memory = Mémoire

# Heading of the disk tile.
settings-about-disk = Disque


# Heading of the card with the switch for looking for updates.
settings-about-updates = Mises à jour

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle cherche de nouvelles versions des applications, des modules, du micrologiciel et de lui-même peu après votre ouverture de session et toutes les quelques heures, et les correctifs de sécurité de Debian arrivent chaque jour. Quand c'est désactivé, il ne cherche que lorsque vous appuyez sur Rechercher les mises à jour dans Applications, ou sur le bouton des mises à jour de l'anneau. Les correctifs de sécurité de Debian ont leur propre réglage ; désactivés, ils ne s'installent plus tout seuls et les failles connues restent ouvertes jusqu'à ce que vous fassiez la mise à jour. Changer l'un ou l'autre réglage demande le mot de passe d'un administrateur, pour que les services de l'ordinateur voient aussi ce choix.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Rechercher les mises à jour

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = En attente du mot de passe...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Désactivé : rien ne cherche tant que vous ne le demandez pas.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Mises à jour de sécurité

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Les correctifs de sécurité s'installent tout seuls chaque jour.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Les correctifs de sécurité ne s'installent plus tout seuls.

