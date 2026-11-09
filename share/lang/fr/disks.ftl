
# machine translated (French), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Disques

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = À propos de cette page

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Chaque disque de cet ordinateur ou branché dessus, dessiné sous forme d'une barre de ses partitions. Chaque partition est aussi large que sa taille et remplie selon son utilisation, si bien qu'une partition pleine se repère facilement.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Sous l'image, chaque disque a sa carte. Ouvrez une partition, montez-la ou démontez-la, retirez un disque en toute sécurité ou éteignez-le. Le crayon sur une partition ouvre ses modifications : son nom, son propre dossier et si elle est prête au démarrage, sa taille, sa vérification et sa réparation, son formatage ou sa suppression. Le plus dans l'espace libre crée une nouvelle partition.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = Le disque d'où le système démarre est affiché mais n'est jamais proposé pour une modification qui efface ou déplace des données. Toute modification d'un disque demande à chaque fois le mot de passe d'un administrateur. L'œil masque un disque ou une partition pour Folders et l'anneau : il reste ici, grisé, et n'est pas verrouillé.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Chaque partition de chaque disque, remplie selon son utilisation.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Vos disques

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Les disques ne peuvent pas être lus pour le moment.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Réessayer

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Chercher de nouveau

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Aucun disque n'a été trouvé. Branchez-en un et il apparaîtra ici.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Le service de disques

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Le bus système

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } n'est pas disponible. Installez-le avec : sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Observation des disques…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
# SAFETY: needs a human review before it ships
disks-headline-failing = { $name } est en train de lâcher. Copiez-en maintenant tout ce que vous voulez garder.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } demande un coup d'œil : { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Les disques et l'espace qu'ils contiennent.
        [one] Les disques et l'espace qu'ils contiennent, et un disque branché.
       *[other] Les disques et l'espace qu'ils contiennent, et { $plugged } disques branchés.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Presque plein : seulement { $free } libres sur cet ordinateur. Supprimez ou déplacez bientôt des fichiers.
        [one] Presque plein : seulement { $free } libres sur cet ordinateur, et un disque branché. Supprimez ou déplacez bientôt des fichiers.
       *[other] Presque plein : seulement { $free } libres sur cet ordinateur, et { $plugged } disques branchés. Supprimez ou déplacez bientôt des fichiers.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Il se remplit : { $free } libres sur cet ordinateur.
        [one] Il se remplit : { $free } libres sur cet ordinateur, et un disque branché.
       *[other] Il se remplit : { $free } libres sur cet ordinateur, et { $plugged } disques branchés.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Beaucoup de place : { $free } libres sur cet ordinateur.
        [one] Beaucoup de place : { $free } libres sur cet ordinateur, et un disque branché.
       *[other] Beaucoup de place : { $free } libres sur cet ordinateur, et { $plugged } disques branchés.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Cet ordinateur

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Vos fichiers

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } encore libres, là où se trouve le système.
       *[files] { $free } encore libres, pour vos propres fichiers.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name } : { $full } plein, { $free } libres sur { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = sur { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Aucun relevé, les disques ne disent pas comment ils vont

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = En bonne santé, selon le disque lui-même

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] En bonne santé, selon les { $n } disques
       *[other] En bonne santé, selon les { $n } disques
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } demande un coup d'œil, voir sa carte
       *[other] { $n } demandent un coup d'œil, voir leurs cartes
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 disque, { $total } au total. { $health }.
       *[other] { $n } disques, { $total } au total. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } libres

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Non monté

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Espace libre

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Aucun disque ni carte dedans

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Encore vide

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB ou amovible

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, à l'intérieur de cet ordinateur

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB ou amovible, masqué

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, à l'intérieur de cet ordinateur, masqué

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } plein, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words } : { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Partition

# Name given to a drive that gives no maker or model.
disks-drive-name = Disque

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Échange

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Chiffrée, déverrouillée

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Chiffrée, verrouillée

# Kind of a part that holds other parts.
disks-kind-container = Contient d'autres partitions

# Kind of a part that has no filesystem on it.
disks-kind-none = Aucun système de fichiers

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, la partition d'où démarre un ordinateur
        [legacy] { $kind }, la partition d'où démarre un ordinateur plus ancien
       *[loader] { $kind }, la partition propre à un chargeur de démarrage
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name } : { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = tout ordinateur

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = Linux seulement

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = anciens appareils

# Trouble a drive reports about its spare space.
disks-trouble-spare = manque bientôt d'espace de réserve

# Trouble a drive reports about its heat.
disks-trouble-temperature = trop chaud

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = s'use

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = s'est mis en lecture seule

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = la sauvegarde de sa mémoire a échoué

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = une partie s'est mise en lecture seule

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = usé au-delà de ce que prévoit son fabricant

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } zone endommagée
       *[other] { $n } zones endommagées
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] utilisé depuis { $hours } heure
       *[other] utilisé depuis { $hours } heures
    }

# How a drive says it is when it is failing, on its card.
# SAFETY: needs a human review before it ships
disks-health-failing = En défaillance : copiez-en maintenant tout ce que vous voulez garder

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Demande attention : { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = En bonne santé

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Aucun disque ne répond

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Le boîtier répond, mais pas le disque qu'il contient. Débranchez-le, attendez dix secondes et rebranchez-le sur un port bleu ou USB-C avec son propre câble ; si cela ne change rien, le disque à l'intérieur s'est peut-être détaché ou est en panne.

# Tip of the round button on a drive's card that takes the drive out safely.
# SAFETY: needs a human review before it ships
disks-remove-safely = Retirer en toute sécurité

# Tip of the round button on a drive's card that powers the drive off.
# SAFETY: needs a human review before it ships
disks-power-off-tip = Éteindre

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Effacer tout le disque

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Afficher ce disque
       *[part] Afficher cette partition
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Masquer ce disque. { $lock }
       *[part] Masquer cette partition. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Masqué pour Folders et l'anneau. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = Le système démarre d'ici : rien ne peut y être effacé.

# Note on the card of a drive that has no partition table.
disks-no-table = Aucune table de partitions : tout le disque est libre.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = non monté

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = se monte au démarrage dans { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = apparaît dans { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = dans { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Ouvrir

# Tip of the round button that mounts a part.
disks-mount = Monter

# Tip of the round button that unmounts a part.
# SAFETY: needs a human review before it ships
disks-unmount = Démonter

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Modifier

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Nom

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Nom (facultatif)

# Tip of the button that saves a part's new name.
disks-rename-tip = Renommer

# Note under the Name row of a part.
disks-name-note = Son nom ici et sur les autres ordinateurs.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Nom du dossier

# Tip of the button that saves a part's folder.
disks-use-folder = Utiliser ce dossier

# Label of the row where a part's folder is chosen.
disks-folder = Dossier

# Note under the Folder row of a part.
disks-folder-note = Où apparaissent ses fichiers. Laissez vide pour que l'ordinateur choisisse.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Monter au démarrage

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Prête dès que l'ordinateur démarre. Si elle est absente, l'ordinateur démarre quand même.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Taille en Go

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
# SAFETY: needs a human review before it ships
disks-resize-range = De { $least } à { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
# SAFETY: needs a human review before it ships
disks-resize-range-mount = De { $least } à { $most }. Montez-la pour voir jusqu'où elle peut diminuer.

# Tip of the button that resizes a part.
# SAFETY: needs a human review before it ships
disks-resize-tip = Redimensionner

# Label of the row where a part is resized.
disks-size = Taille

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Cela effacera tout ce qui se trouve sur { $words }. Impossible de revenir en arrière.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Cliquez de nouveau : { $name } sur { $drive } passe de { $from } à { $to }

# Label of the row where room is made for a new part under a part that is in use.
# SAFETY: needs a human review before it ships
disks-room-label = Faire de la place pour une nouvelle partition

# Spoken name of the slider that chooses how much room to make.
# SAFETY: needs a human review before it ships
disks-room-gb = Place en Go

# Tip of the button that makes room under a part that is in use.
# SAFETY: needs a human review before it ships
disks-room-tip = Faire de la place

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Cliquez de nouveau : { $name } sur { $drive } perd { $size }, après un instantané

# Note under the row that makes room.
# SAFETY: needs a human review before it ships
disks-room-note = Réduite pendant son utilisation, après un instantané, en laissant de l'espace libre.

# Tip of the button that checks a part for faults.
disks-check-tip = Rechercher les erreurs

# Tip of the button that repairs a part.
# SAFETY: needs a human review before it ships
disks-repair-tip = Réparer

# Label of the row where a part is checked and repaired.
disks-faults = Erreurs

# Note under the Faults row.
disks-faults-note = Recherche les erreurs dans la façon dont ses fichiers sont rangés, et les corrige. Elle est démontée pendant ce temps.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Cliquez de nouveau pour réparer { $name }, sur { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Type

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = Formater

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Formater

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = Repart de zéro avec une seule partition, remplissant le disque.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = Efface tout sur cette partition seulement.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Cliquez de nouveau : tout ce qui se trouve sur { $words } est effacé

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, sur { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Supprimer { $name }, { $size }, sur { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Supprimer cette partition

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = Son espace devient libre, prêt pour une nouvelle partition ou pour que la précédente s'y étende.

# Tip of the round button on free space that opens the box for making a new part.
# SAFETY: needs a human review before it ships
disks-new-part-tip = Créer une nouvelle partition ici

# Label of the row where a new part is made.
# SAFETY: needs a human review before it ships
disks-new-part = Nouvelle partition

# Note under the row where a new part is made.
# SAFETY: needs a human review before it ships
disks-new-part-note = Créée dans cet espace libre, et formatée prête à l'emploi.

# Tip of the button that makes a new part.
# SAFETY: needs a human review before it ships
disks-make-tip = La créer

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-make-warning = Cliquez de nouveau pour créer une partition de { $size } sur { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Effacer { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, sur { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Une nouvelle partition

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } libres, sur { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Fermer

# Words on a row while a part is being mounted.
# SAFETY: needs a human review before it ships
disks-mounting = Montage

# Words on a row when a part has been mounted.
disks-mounted = Monté

# Words on a row while a part is being unmounted.
# SAFETY: needs a human review before it ships
disks-unmounting = Démontage

# Words on a row when a part has been unmounted.
# SAFETY: needs a human review before it ships
disks-unmounted = Démonté

# Words on a drive's row while it is being closed so it can be removed.
# SAFETY: needs a human review before it ships
disks-closing-it = Fermeture

# Words on a drive's row when it is safe to remove.
# SAFETY: needs a human review before it ships
disks-safe-remove = Peut être retiré

# Words on a drive's row while it is being powered off.
# SAFETY: needs a human review before it ships
disks-powering-off = Extinction

# Words on a drive's row when it has been powered off.
# SAFETY: needs a human review before it ships
disks-off-safe = Éteint : peut être débranché

# Words on a row while a part is being renamed.
disks-renaming = Renommage

# Words on a row when a part has been renamed.
disks-renamed = Renommé

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = L'ordinateur choisit de nouveau son dossier

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Elle apparaît dans { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Se monte au démarrage dans { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Laissée de côté au démarrage

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Enregistrement

# Words on a row while a part is being resized.
disks-resizing = Redimensionnement

# Words on a row when a part has been resized. { $size } is its new size.
# SAFETY: needs a human review before it ships
disks-resized = Redimensionnée à { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
# SAFETY: needs a human review before it ships
disks-room-files-missing = Les fichiers sont introuvables, on ne peut donc pas faire de place.

# Words on a row while the snapshot is taken before room is made.
# SAFETY: needs a human review before it ships
disks-snapshot-taking = Prise d'un instantané

# Red words when the snapshot taken before making room could not be found afterwards.
# SAFETY: needs a human review before it ships
disks-snapshot-missing = L'instantané est introuvable après coup.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
# SAFETY: needs a human review before it ships
disks-snapshot-failed = { $why } Rien n'a été redimensionné.

# Red words when things changed while the snapshot was being taken.
# SAFETY: needs a human review before it ships
disks-snapshot-changed = Des choses ont changé pendant la prise de l'instantané, rien n'a donc été redimensionné.

# Words on a row while room is being made.
disks-making-room = Libération de place

# Words on a row when room has been made. { $size } is how much.
# SAFETY: needs a human review before it ships
disks-room-made = { $size } de place libérée

# Words on a row while a part is being repaired.
# SAFETY: needs a human review before it ships
disks-repairing = Réparation

# Words on a row when a part has been repaired.
# SAFETY: needs a human review before it ships
disks-repaired = Réparée

# Red words when some faults could not be put right.
# SAFETY: needs a human review before it ships
disks-repair-failed = Certaines erreurs n'ont pas pu être corrigées. Copiez tout ce que vous voulez garder.

# Words on a row while a part is being checked.
disks-checking = Vérification

# Words on a row when the check found no faults.
disks-no-faults = Aucune erreur trouvée

# Red words when the check found faults.
disks-faults-found = Des erreurs ont été trouvées : appuyez sur la clé pour les réparer.

# Words on a row while a part or drive is being formatted.
disks-formatting = Formatage

# Words on a row when a part has been formatted.
disks-formatted = Formatée

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = effacement de tout le disque

# Red words when a freshly formatted part could not be found to give it its folder again.
# SAFETY: needs a human review before it ships
disks-new-part-missing = La nouvelle partition est introuvable.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Le service de disques est inaccessible.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = L'identifiant de la nouvelle partition n'a pas pu être lu.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Suppression

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Supprimée

# Words on a row while a new part is being made.
disks-making-it = Création

# Words on a row when a new part has been made.
disks-made = Créée

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
disks-hide-failed = Cela n'a pas pu être conservé : { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
# SAFETY: needs a human review before it ships
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Donnez-lui d'abord un nom.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] Ce nom est trop long : { $max } caractère au plus.
        [many] Ce nom est trop long : { $max } de caractères au plus.
       *[other] Ce nom est trop long : { $max } caractères au plus.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Utilisez uniquement des lettres, des chiffres, - et _ (sans espaces).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Commencez par une lettre ou un chiffre.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } est déjà utilisé.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] Ce nom est trop long pour ce type : { $max } caractère au plus.
        [many] Ce nom est trop long pour ce type : { $max } de caractères au plus.
       *[other] Ce nom est trop long pour ce type : { $max } caractères au plus.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Les noms FAT ne peuvent pas contenir * ? . , ; : / \ | + = < > [ ] ni ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
# SAFETY: needs a human review before it ships
disks-resize-mount-first =
    { $way ->
        [larger] Montez-la d'abord : ce type ne peut être agrandi que lorsqu'il est monté.
       *[smaller] Montez-la d'abord : ce type ne peut être réduit que lorsqu'il est monté.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
# SAFETY: needs a human review before it ships
disks-resize-cannot =
    { $way ->
        [larger] Ce type ne peut pas être agrandi.
       *[smaller] Ce type ne peut pas être réduit.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Aucun mot de passe n'a été donné, rien n'a donc changé.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Aucun mot de passe n'a été donné, le travail s'est donc arrêté en cours de route.

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = Une autre modification est encore en cours. Réessayez quand elle sera terminée.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = On n'a pas pu s'assurer que le mot de passe serait demandé, rien n'a donc changé.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Le mot de passe qui vient d'être donné n'a pas pu être retiré, si bien que pendant quelques minutes une autre modification d'un disque pourrait ne pas en demander. Une nouvelle tentative est en cours.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = Le mot de passe donné a finalement été retiré : la prochaine modification en demandera un.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Le service de disques est inaccessible, rien n'a donc changé.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Disques a été fermé pendant cette modification.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Disques reste ouvert jusqu'à la fin, pour pouvoir dire comment cela s'est terminé.

# Red words for a refusal. { $reason } says why, without a full stop.
# SAFETY: needs a human review before it ships
disks-nothing-changed = { $reason }, rien n'a donc changé.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
# SAFETY: needs a human review before it ships
disks-part-way = { $reason }, le travail s'est donc arrêté en cours de route.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Le service de disques n'a pas pu être interrogé

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
# SAFETY: needs a human review before it ships
disks-failed-part-way = { $why } Le travail s'est arrêté en cours de route.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Fait : { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Échec : { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Non fait : { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Inconnu : { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-left-unmounted = { $dev } a été démontée pour cela et n'est pas montée maintenant.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-still-unmounted = { $dev } a été démontée pour cela et l'est toujours : montez-la depuis sa ligne.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-may-be-unmounted = { $dev } a peut-être été démontée : regardez sa ligne.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } a été démontée pour cela, et la remonter demande un mot de passe particulier : montez-la depuis sa ligne.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
# SAFETY: needs a human review before it ships
disks-put-back-failed = { $dev } a été démontée pour cela et n'a pas pu être remontée ({ $why }) : montez-la depuis sa ligne.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Formatée, mais son dossier { $dir } ne lui a pas été redonné. { $why } Choisissez de nouveau le dossier depuis sa ligne.

# Red words when a drive is in use and cannot be unmounted.
# SAFETY: needs a human review before it ships
disks-busy = Quelque chose l'utilise encore. Fermez les fenêtres ou programmes qui montrent ses fichiers, puis réessayez.

# Red words when there is not enough room for what is on a part at the size asked for.
# SAFETY: needs a human review before it ships
disks-no-room = Il n'y a pas assez de place pour ce qu'elle contient à cette taille.

# Step of a change. { $dev } is the part.
disks-step-unmount = démontage de { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = nouveau montage de { $dev }

# Step of a change.
disks-step-eject = fermeture du disque pour le retirer

# Step of a change.
disks-step-power-off = extinction du disque

# Step of a change. { $dev } is the part.
disks-step-format = formatage de { $dev }

# Step of a change.
disks-step-create = création de la nouvelle partition

# Step of a change. { $dev } is the part.
disks-step-delete = suppression de { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = redimensionnement de la partition { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = redimensionnement de ce qui se trouve sur { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = réparation de { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = vérification de { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = renommage de { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = enregistrement du dossier et du choix de démarrage de { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = l'étape { $method } sur { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
# SAFETY: needs a human review before it ships
disks-room-ext-offline = Cette partition { $kind } ne peut être réduite que lorsqu'elle n'est pas utilisée, on ne peut donc pas faire de place ici.

# Red words under the Make room row for an xfs part.
# SAFETY: needs a human review before it ships
disks-room-xfs-fixed = Cette partition xfs ne peut pas être réduite.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
# SAFETY: needs a human review before it ships
disks-room-other-kind = Cette partition { $kind } ne peut pas être réduite pendant son utilisation.

# Red words under the Make room row when a snapshot cannot be taken of the part.
# SAFETY: needs a human review before it ships
disks-room-no-snapshot = Un instantané ne peut être pris que du disque du système, on ne peut donc pas faire de place ici.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-not-in-table = Ce n'est pas une partition de la table d'un disque, on ne peut donc pas y faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-no-online-shrink = Le service de disques ne peut pas réduire Btrfs pendant son utilisation ici, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-holds-parts = Elle contient d'autres partitions, on ne peut donc pas y faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-not-gpt = Son disque n'a pas de table GPT, on ne peut donc pas y faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-mount-first = Montez-la d'abord : Btrfs se réduit lorsqu'il est monté.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-mounts-not-one = Elle n'est pas montée à exactement un endroit, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-read-only = Elle est montée en lecture seule, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-read-only-unknown = On n'a pas pu savoir si elle est en lecture seule, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-many-drives = Les fichiers s'étendent sur plus d'un disque, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-drives-unknown = On n'a pas pu savoir sur combien de disques s'étendent les fichiers, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-swap-file = Un fichier d'échange s'y trouve, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-swap-unknown = On n'a pas pu savoir si un fichier d'échange s'y trouve, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-used-unknown = La quantité utilisée n'a pas pu être lue, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-area-unknown = Le service de disques ne dit pas quelle est la taille de la zone des fichiers, elle n'a donc pas pu être vérifiée et on ne peut pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-open-missing = Sa copie ouverte est introuvable, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-encryption-sizes = Les tailles données pour le chiffrement ne concordent pas, on ne peut donc pas faire de place.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-sizes = Les tailles données ne concordent pas, on ne peut donc pas faire de place.

# Red words under the Make room row. { $used } is how much the files use.
# SAFETY: needs a human review before it ships
disks-room-too-little = Elle contient { $used } et il faut en garder davantage, trop peu peut donc être libéré.

# Red words under the Make room row.
# SAFETY: needs a human review before it ships
disks-room-sizes-unread = Ses tailles n'ont pas pu être lues.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Une place de { $least } à { $most } peut être libérée.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
# SAFETY: needs a human review before it ships
disks-room-stopped = { $what }, le travail a donc été arrêté.

# What was found wrong while making room.
disks-room-check-unmounted = Les fichiers ne sont plus montés

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = La partition fait { $is } et devrait faire { $should }

# What was found wrong while making room.
disks-room-check-open = La copie ouverte du chiffrement n'a pas la taille qu'elle devrait avoir

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = La zone des fichiers fait { $is } et devrait faire { $should }

# What was found wrong while making room.
disks-room-check-area-small = La zone des fichiers est plus petite que ce qui est utilisé

# What was found wrong while making room.
disks-room-check-fit = Le contenant ne contient plus son contenu

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
# SAFETY: needs a human review before it ships
disks-room-gone =
    { $what ->
        [part] La partition n'est plus listée par le service de disques, le travail a donc été arrêté.
        [copy] La copie ouverte n'est plus listée par le service de disques, le travail a donc été arrêté.
       *[files] Le système de fichiers n'est plus listé par le service de disques, le travail a donc été arrêté.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = réduction de la zone des fichiers

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = réduction de la copie ouverte du chiffrement

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = réduction de la partition

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = vérification des tailles

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Avant de faire de la place sous { $dev } ({ $at })


# Red words when another change is pressed while one is still being made.
disks-change-going = Une modification est en cours.


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Cliquez de nouveau : { $name } sur { $drive } passe de { $from } à { $to }. Sauvegardez d'abord ses fichiers, car une coupure de courant pendant la réduction peut les faire perdre.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows sur { $name } a été mis en veille prolongée ou laissé en démarrage rapide, donc la réduire pourrait lui faire perdre ses fichiers. Démarrez Windows, choisissez Redémarrer, puis réessayez.

