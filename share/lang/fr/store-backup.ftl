
# machine translated (French), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Rien n'a été remis en place, car { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Une partie de la sauvegarde n'a pas été remise en place, car { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = la sauvegarde contient un lien ou autre chose qui sortirait du dossier où elle va

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = quelque chose sur cet ordinateur se trouve déjà là où une partie de la sauvegarde doit aller

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = il n'y a nulle part où décompresser la sauvegarde

# The reason when the backup file cannot be read.
store-because-unreadable = le fichier de sauvegarde n'a pas pu être lu

# The reason when the file is not a Trindle backup.
store-because-not-trindle = ce fichier n'est pas une sauvegarde de Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = la sauvegarde est abîmée et sa liste n'a pas pu être lue

# The reason when the file is not a Flat backup.
store-because-not-flat = ce fichier ne ressemble pas à une sauvegarde de Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = ce fichier ne contient pas de liste d'applications de Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = elle a été faite par une version de Flat plus récente que ce que Trindle sait lire

# The reason when the file holds no apps.
store-because-no-apps = il n'y a aucune application dans ce fichier

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = la sauvegarde n'a pas pu être lue : { $why }

# The reason when the program gave none.
store-because-none = aucune raison n'a été donnée

# The reason when the system said permission was denied.
store-because-permission = cet ordinateur n'a pas laissé Trindle écrire à cet endroit

# The reason when the disk is full.
store-because-space = il ne reste plus de place sur cet ordinateur

# The reason when a part of the computer is read-only.
store-because-read-only = cette partie de l'ordinateur est en lecture seule

# The reason when the person's disk quota is used up.
store-because-quota = l'espace autorisé pour ce compte est épuisé

# The reason when a file or program is missing.
store-because-missing = il manque un fichier ou un programme dont elle a besoin

# The reason when no reason could be read from the program's last line.
store-because-failed = échec

# The reason when the backup has no name to be written under.
store-because-no-name = elle n'a pas de nom

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = ses réglages dans la sauvegarde sortent de son propre dossier

# The reason when something is too big to be read whole.
store-because-too-big = elle est trop grosse pour être lue en entier

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = ses réglages dans la sauvegarde ne correspondent pas à leur somme de contrôle

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Remis en place, sauf l'image de votre compte. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } L'image de votre compte n'a pas non plus été remise en place. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec n'a pas pu être lancé : { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Elle n'a pas pu être lue dans la sauvegarde : { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = La sauvegarde n'a pas été faite, car { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = rien pour l'instant

# Beside the size of a part of the backup: about how big it is.
store-backup-about = environ { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Remise en place des réglages et des AppImage de Trindle...

# Step shown while the settings and data are put back.
store-backup-step-data = Remise en place des réglages et des données...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Installation de { $count } application depuis Flathub... cela peut prendre du temps.
       *[other] Installation de { $count } applications depuis Flathub... cela peut prendre du temps.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Les réglages et les données ont été remis en place, mais ces applications n'ont pas été installées, car Flatpak les a refusées : { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Les réglages et les données ont été remis en place, mais ces applications n'ont pas été installées, car Flatpak les a refusées : { $failed }. Aussi non installées : { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Les réglages et les données ont été remis en place, mais ces applications n'ont pas été installées : { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] 1 application ignorée : son nom n'était pas valide.
       *[other] { $n } applications ignorées : leurs noms n'étaient pas valides.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = la source distante { $name } n'est pas chiffrée et n'a pas été acceptée, donc elle n'a pas été utilisée

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = la source distante { $name } n'est pas sur cet ordinateur et la sauvegarde ne dit pas où elle se trouve

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = les réglages de { $name } (car la copie de la sauvegarde n'a pas passé ses vérifications)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (car le nom de sa source n'est pas valide)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (car { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (car Flatpak l'a refusée : { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = tout (car il n'y a nulle part où décompresser la sauvegarde)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = ce qui est autorisé pour chaque application

# Said among what could not be done: one app's settings.
store-flat-app-settings = les réglages de { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Ce qui s'y trouvait avant est gardé sous { $path } : { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Installation de { $name } ({ $at } sur { $total })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Remise en place des réglages de { $name } ({ $at } sur { $total })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } application installée.
       *[other] { $n } applications installées.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Réglages remis en place pour { $n }.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } déjà présente, laissée telle quelle.
       *[other] { $n } déjà présentes, laissées telles quelles.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Non remis en place : { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

