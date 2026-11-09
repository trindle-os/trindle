
# machine translated (Spanish), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = No se restauró nada, porque { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Una parte de la copia de seguridad no se restauró, porque { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = la copia contiene un enlace u otra cosa que llegaría fuera de la carpeta donde se restaura

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = algo de este equipo ya ocupa el lugar donde debe ir una parte de la copia

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = no hay dónde descomprimir la copia

# The reason when the backup file cannot be read.
store-because-unreadable = no se pudo leer el archivo de la copia

# The reason when the file is not a Trindle backup.
store-because-not-trindle = ese archivo no es una copia de seguridad de Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = la copia está dañada y no se pudo leer su lista

# The reason when the file is not a Flat backup.
store-because-not-flat = ese archivo no parece una copia de seguridad de Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = ese archivo no contiene una lista de aplicaciones de Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = esta copia la hizo una versión de Flat más nueva de la que Trindle puede leer

# The reason when the file holds no apps.
store-because-no-apps = no hay aplicaciones en ese archivo

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = no se pudo leer la copia: { $why }

# The reason when the program gave none.
store-because-none = no dio ningún motivo

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = este equipo no dejó que Trindle escribiera ahí

# The reason when the disk is full.
store-because-space = no queda espacio en este equipo

# The reason when a part of the computer is read-only.
store-because-read-only = esa parte de este equipo es de solo lectura

# The reason when the person's disk quota is used up.
store-because-quota = se agotó el espacio permitido para esta cuenta

# The reason when a file or program is missing.
store-because-missing = falta un archivo o programa que necesita

# The reason when no reason could be read from the program's last line.
store-because-failed = falló

# The reason when the backup has no name to be written under.
store-because-no-name = no tiene nombre

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = sus ajustes en la copia llegan fuera de su propia carpeta

# The reason when something is too big to be read whole.
store-because-too-big = es demasiado grande para leerlo entero

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = sus ajustes en la copia no coinciden con su suma de comprobación

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Restaurado, salvo la imagen de tu cuenta. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Tampoco se restauró la imagen de tu cuenta. { $why }

# Said when the pkexec program could not be started.
# SAFETY: needs a human review before it ships
store-refusal-pkexec = No se pudo iniciar pkexec: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = No se pudo leer de la copia: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = No se hizo la copia de seguridad, porque { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = todavía nada

# Beside the size of a part of the backup: about how big it is.
store-backup-about = unos { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Restaurando los ajustes y los AppImages de Trindle...

# Step shown while the settings and data are put back.
store-backup-step-data = Restaurando ajustes y datos...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Instalando { $count } aplicación desde Flathub... puede tardar un rato.
       *[other] Instalando { $count } aplicaciones desde Flathub... puede tardar un rato.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Se restauraron los ajustes y los datos, pero estas aplicaciones no se instalaron, porque Flatpak las rechazó: { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Se restauraron los ajustes y los datos, pero estas aplicaciones no se instalaron, porque Flatpak las rechazó: { $failed }. Tampoco se instalaron: { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Se restauraron los ajustes y los datos, pero estas aplicaciones no se instalaron: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] 1 aplicación omitida: su nombre no era válido.
       *[other] { $n } aplicaciones omitidas: sus nombres no eran válidos.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = el origen { $name } no está cifrado y no se aceptó, así que no se usó

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = el origen { $name } no está en este equipo y la copia no indica dónde está

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = los ajustes de { $name } (porque su copia en la copia de seguridad no pasó las comprobaciones)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (porque el nombre de su origen no es válido)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (porque { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (porque Flatpak la rechazó: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = todo (porque no hay dónde descomprimir la copia)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = lo que se permite a todas las aplicaciones

# Said among what could not be done: one app's settings.
store-flat-app-settings = los ajustes de { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Lo que había antes se conserva como { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Instalando { $name } ({ $at } de { $total })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Restaurando los ajustes de { $name } ({ $at } de { $total })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] Se instaló { $n } aplicación.
       *[other] Se instalaron { $n } aplicaciones.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Ajustes restaurados para { $n }.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } ya estaba aquí y se dejó como estaba.
       *[other] { $n } ya estaban aquí y se dejaron como estaban.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = No restaurado: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

