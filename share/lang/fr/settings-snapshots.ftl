# machine translated (French), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Instantanés

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = instantanés instantané snapshot snapshots restaurer annuler retour en arrière sauvegarde mise à jour cassé btrfs snapper menu de démarrage grub versions antérieures ancienne version précédente fichier supprimé remettre récupérer dossier personnel copies

# Help line at the foot of the Snapshots page.
# SAFETY: needs a human review before it ships
settings-snap-help = Le système tel qu'il était avant et après chaque changement de logiciel, et chaque fois que vous en prenez un. En restaurer un remet tout le système dans cet état dès le prochain démarrage ; vos propres fichiers restent tels quels quand le dossier personnel a sa propre partie du disque, comme l'installateur de Trindle la crée. Si l'ordinateur ne démarre pas, choisissez les instantanés de Trindle dans le menu de démarrage. Les versions antérieures gardent chaque jour une copie des fichiers de chacun, à part des instantanés du système.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Aucune copie n'a été faite : le disque est presque plein.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = aujourd'hui à { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = hier à { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = le { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = L'ordinateur n'a pas voulu redémarrer. Redémarrez-le plutôt depuis l'anneau.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Prise d'un instantané

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Libération...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Libéré

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Restauration

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Le système sera l'instantané { $n } dès le prochain démarrage. Tout ce qui a changé depuis est mis de côté, pas perdu.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Suppression

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Supprimé

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = En attente du mot de passe...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Une copie est faite chaque jour

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Plus aucune n'est faite

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Conservées

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = Effacement

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = Effacées

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Aucune copie pour le moment
        [one] Une copie
        [many] { $n } de copies
       *[other] { $n } copies
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = La première est faite dans un délai d'un jour.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Elles prennent { $size }, mesuré { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Leur place est mesurée une fois par jour.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Toutes les effacer maintenant

# Heading of the card about earlier versions of files.
settings-snap-earlier = Versions antérieures

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle peut garder chaque jour une copie des fichiers de chacun tels qu'ils étaient, pour qu'une version plus ancienne d'un fichier, ou un fichier supprimé par erreur, puisse être ouvert ou remis en place depuis Dossiers. Chaque personne ne peut lire dans une copie que ce qu'elle pouvait lire ce jour-là, si bien qu'un dossier personnel privé reste privé ; un fichier qui était alors ouvert aux autres le reste dans cette copie jusqu'à sa disparition. Les copies sont à part des instantanés du système : restaurer le système ne change jamais vos fichiers, et elles ne changent jamais le système. Le coût est la place : un fichier que vous supprimez ou modifiez prend encore sa place jusqu'à ce que la dernière copie qui le contient ait disparu, supprimer des choses ne libère donc pas le disque tout de suite. Effacer les copies le libère maintenant, pour tout le monde sur cet ordinateur, et ne peut pas être annulé. Aucune copie n'est faite quand le disque est presque plein. Changer tout cela demande le mot de passe d'un administrateur.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Garder des versions antérieures de vos fichiers

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Les fichiers supprimés restent { $time } avant que leur place soit libérée.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Garder la copie de chaque jour pendant

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Durée de conservation de la copie de chaque jour

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Examen des instantanés…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Les instantanés n'ont pas pu être lus.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Démarré depuis l'instantané { $n }, rien de ce qui est fait maintenant n'est donc conservé.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] { $n } instantané auquel revenir, le plus récent datant de { $when }.
        [many] { $n } d'instantanés auxquels revenir, le plus récent datant de { $when }.
       *[other] { $n } instantanés auxquels revenir, le plus récent datant de { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Aucun instantané pour le moment. Un est pris avant chaque changement de logiciel.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, conservé jusqu'à sa suppression

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Restaurer le système à cet état

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Supprimer cet instantané

# Tip of the button that restarts the computer after a restore.
# SAFETY: needs a human review before it ships
settings-snap-restart-now = Redémarrer maintenant

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Ils ne sont pas encore configurés.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Réinstallez le paquet trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Les instantanés ne sont pas configurés. Réinstaller le paquet trindle les configure : sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = un autre type

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Ils demandent Btrfs ; cet ordinateur utilise { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Choisissez Btrfs lors de l'installation de Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Les instantanés demandent le système sur Btrfs, et cet ordinateur utilise { $fs }. Choisissez Btrfs pour le système lors de l'installation de Debian pour les avoir.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Il n'y a aucun instantané sur cet ordinateur.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Aucun instantané

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Aucun instantané sur cet ordinateur

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Pour en avoir

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Recherche…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Les instantanés n'ont pas pu être lus

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Aucun pour le moment

# Under the None yet heading.
settings-snap-none-line-sub = Un est pris avant chaque changement de logiciel.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Chaque instantané peut remettre tout le système comme il était alors, dès le prochain démarrage, ou être supprimé. Les deux demandent d'appuyer deux fois et le mot de passe d'un administrateur. Après une restauration, un bouton pour redémarrer apparaît à côté.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Prendre un instantané maintenant

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Pas quand on a démarré depuis un instantané.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Conservé jusqu'à ce que vous le supprimiez.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Un système mis de côté par une restauration
        [many] { $n } de systèmes mis de côté par des restaurations
       *[other] { $n } systèmes mis de côté par des restaurations
    }

# Under the set aside heading.
settings-snap-free-space = Libérez la place une fois que tout va bien.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Libérer la place qu'ils utilisent

# Heading of the card for taking a snapshot.
settings-snap-take-one = En prendre un

# Help text of the Take one card (the information button).
settings-snap-take-about = Prenez un instantané avant d'essayer quelque chose que vous voudrez peut-être annuler ; un instantané que vous prenez est conservé jusqu'à ce que vous le supprimiez. Tant que l'ordinateur est démarré depuis un instantané, rien de ce qui serait pris ne serait conservé, redémarrez donc normalement d'abord. Chaque restauration met de côté ce qu'était le système juste avant, rien n'est donc perdu ; libérez la place qu'ils utilisent une fois que tout va bien.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Démarré depuis l'instantané { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Rien de ce qui est fait maintenant n'est conservé.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Vos fichiers sont aussi dans les instantanés

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Restaurer remet aussi vos fichiers.

# Heading of the card about what to know before restoring.
settings-snap-before = Avant de restaurer

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Démarré depuis un instantané, rien de ce qui est fait maintenant n'est conservé : si c'est ainsi que le système doit être, appuyez sur restaurer dessus dans la liste, puis redémarrez. Sur cet ordinateur, le dossier personnel peut faire partie du système, et restaurer un instantané remet alors aussi vos propres fichiers comme ils étaient, copiez donc d'abord tout ce qui est nouveau en lieu sûr.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Fichiers système verrouillés

# Under the System files locked switch.
settings-snap-locked-sub = Seuls les installations et les mises à jour les déverrouillent.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Verrouillés

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Déverrouillés

