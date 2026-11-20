
# machine translated (French), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Ce n'est pas un identifiant valide.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Cet identifiant est trop long.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-none = Aucun utilisateur nommé { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-system = { $name } est un compte système, Trindle ne l'a donc pas modifié.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Aucun mot de passe n'a été donné.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] Le mot de passe doit comporter au moins { $n } caractère.
        [many] Le mot de passe doit comporter au moins { $n } de caractères.
       *[other] Le mot de passe doit comporter au moins { $n } caractères.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Un mot de passe ne peut pas contenir de saut de ligne.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-exists = { $name } existe déjà.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } est le seul administrateur, il doit donc le rester.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Vous ne pouvez pas supprimer votre propre compte.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = Trindle a supprimé le compte de { $name }, mais n'a pas pu retirer sa photo de l'écran de connexion : { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Cette image est trop grande.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Ce n'est pas une image créée par Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle n'a pas pu donner à l'écran de connexion l'accès au dossier des images : { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Ce n'est pas un nom de langue.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle n'a pas pu lire la liste des langues de Debian : { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian ne reconnaît pas la langue { $code }.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Un nom d'ordinateur ne peut contenir que des lettres, des chiffres et des traits d'union. Il peut compter jusqu'à 63 caractères et ne peut ni commencer ni finir par un trait d'union.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = La limite de charge est active pour l'instant, mais elle sera perdue à l'extinction de l'ordinateur : { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = La charge complète est activée pour l'instant, mais la limite reviendra au prochain démarrage de l'ordinateur : { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Les autres ordinateurs peuvent encore se connecter, car Trindle n'a pas pu désactiver le partage : { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Ce n'est pas un identifiant de module.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Cette personne n'existe pas.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Cette personne n'a pas de dossier.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle n'a pas pu enregistrer la liste des modules autorisés : { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Ce module n'existe pas.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Rien n'a été mis à jour. Trindle n'a pas pu déterminer quelle version de lui-même est installée, il n'a donc pas pu exclure sa propre mise à jour.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Installation des mises à jour

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Rangement

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Installation de Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Ce n'est pas un nom de paquet.

# Red words when the package to remove is not installed.
# SAFETY: needs a human review before it ships
admin-package-not-installed = Ce paquet n'est pas installé.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Ce n'est pas un appareil connu de fwupd.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first } ; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle ne sait pas qui veut Tailscale.

# Red words in Settings when the site blocking did not start in time.
# SAFETY: needs a human review before it ships
admin-blocking-late = Le blocage n'a pas démarré à temps.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
# SAFETY: needs a human review before it ships
admin-blocking-not-off = Le blocage n'est pas entièrement désactivé : { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle n'a pas pu remettre { $path } en place : { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Ce n'est pas un nom de site.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Aucun serveur de ce nom.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool } : { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } s'est arrêté avant d'avoir reçu tout ce dont il avait besoin, le travail n'est donc pas terminé : { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } a échoué.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = Impossible de supprimer { $path } : { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Rien n'a été modifié, car Trindle n'a pas pu lire { $path } : { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle n'a pas non plus pu tout remettre dans son état précédent : { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Ce n'est pas une durée valide pour conserver les versions antérieures.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Versions antérieures introuvables.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Il y a un fichier à l'emplacement où devrait se trouver le dossier des versions antérieures.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Impossible de savoir qui a demandé.

# Red words when a step that must go through the administrator's password was started another way.
# SAFETY: needs a human review before it ships
admin-pkexec-only = Ceci ne s'exécute que par pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
# SAFETY: needs a human review before it ships
admin-share-computer-invalid = Ce n'est pas un nom d'ordinateur.

# Red words in Folders when the name of the shared folder is not one.
# SAFETY: needs a human review before it ships
admin-share-name-invalid = Ce n'est pas un nom de dossier partagé.

# Red words in Folders when the address of the computer is not an address.
# SAFETY: needs a human review before it ships
admin-share-address-invalid = Ce n'est pas une adresse d'ordinateur.

# Red words in Folders when the path of the shared folder is not one.
# SAFETY: needs a human review before it ships
admin-share-path-invalid = Ce n'est pas le chemin d'un dossier partagé.

# Red words in Folders when the shared folder to close is not one of the person's own.
# SAFETY: needs a human review before it ships
admin-share-not-yours = Seuls vos propres dossiers partagés peuvent être fermés ici.

# Red words in Folders when no shared folder is open at the place named.
# SAFETY: needs a human review before it ships
admin-share-not-open = Aucun dossier partagé n'est ouvert à cet endroit.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
# SAFETY: needs a human review before it ships
admin-share-folder-changed = Quelque chose a modifié { $path } depuis que Trindle l'a mis en place, rien n'y est donc connecté.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
# SAFETY: needs a human review before it ships
admin-share-private-changed = { $path } a changé depuis le passage de Trindle.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path } : { $why }

# Red words in Folders when no more shared folders can be opened.
# SAFETY: needs a human review before it ships
admin-share-too-many = Trop de dossiers partagés sont ouverts.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } n'est pas installé. Il se trouve dans le paquet Debian { $package }.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Impossible de lire l'identifiant.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle n'a pas pu lire ce qui est monté sur cet ordinateur : { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
# SAFETY: needs a human review before it ships
admin-share-busy = Un programme utilise encore quelque chose dans ce dossier. Fermez-le, puis réessayez.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
# SAFETY: needs a human review before it ships
admin-share-not-answering = Cet ordinateur ne répond pas. Le dossier partagé se fermera quand vous fermerez votre session.

# Red words when the number given for a person is not a user number.
# SAFETY: needs a human review before it ships
admin-user-id-invalid = Ce n'est pas un identifiant d'utilisateur.

# Red words in Settings when the group that may share folders could not be made.
# SAFETY: needs a human review before it ships
admin-sharing-group = Impossible de créer le groupe sambashare.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Récupération de Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Mise en place du partage

# Red words in Settings when a sharing password is set before sharing is set up.
# SAFETY: needs a human review before it ships
admin-sharing-not-set-up = Le partage n'est pas encore en place. Partagez d'abord un dossier.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-shown = Cet ordinateur est encore visible des autres sur le réseau : { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Ce n'est pas un numéro d'instantané.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Pris dans les réglages

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle n'a pas pu savoir sur quel disque se trouve le système.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
# SAFETY: needs a human review before it ships
admin-system-subvolume = Le système n'est pas dans un sous-volume Btrfs à lui seul, Trindle ne peut donc pas le remplacer par un instantané.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle n'a pas pu récupérer la clé de signature depuis { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Cette adresse n'a pas renvoyé de clé de signature.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Rien n'a été modifié, car Trindle n'a pas pu lire { $path } : { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Récupération de sa clé de signature

# Second line of the progress while a source is added.
admin-step-adding-source = Ajout et lecture de sa liste

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Impossible d'utiliser la source. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle n'a pas pu utiliser la source, il l'a donc retirée. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } ne peut pas encore l'activer ni le désactiver : { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle n'a pas pu le démarrer pour { $user } : { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = Trindle n'a pas pu autoriser { $user } à utiliser des machines virtuelles, dont Cowork a besoin : { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = Trindle n'a pas pu autoriser { $user } à l'utiliser sans mot de passe : { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } est installé, mais Trindle n'a pas pu terminer sa configuration : { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Installation

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Ce n'est pas l'une des sources prêtes à l'emploi.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } n'est pas conçu pour ce type d'ordinateur ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Configuration

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Chaque adresse doit être une simple adresse web qui commence par https://.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = La suite et les composants doivent être des mots simples.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Ce n'est pas un nom de paquet.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Retrait

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Nouvelle lecture des listes

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Seules les sources ajoutées par Trindle peuvent être retirées ici.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = La source est toujours là : { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
admin-snapshot-missing = L'instantané { $n } n'est pas sur le disque.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-not-ready = Rien n'a été modifié, car Trindle n'a pas pu préparer l'instantané. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-aside = Trindle n'a pas pu mettre le système actuel de côté : { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = Le système actuel s'appelle maintenant { $aside } au lieu de { $sub }, et l'ordinateur ne démarrera pas tant qu'il ne s'appellera pas de nouveau { $sub } : { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-snapshots-across = Trindle n'a pas pu déplacer les instantanés : { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first } ; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-in-place = Trindle n'a pas pu mettre l'instantané en place : { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-list = Trindle n'a pas pu mettre à jour la liste des systèmes mis de côté : { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
# SAFETY: needs a human review before it ships
admin-restore-not-removed = Impossible de supprimer { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
# SAFETY: needs a human review before it ships
admin-tidied-away = Trindle a supprimé ces éléments, car plus rien n'en avait besoin : { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Toutes les mises à jour n'ont pas été installées. Ont été retenues : { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = La fonction « { $feature } » est désactivée, car son pilote ({ $driver }, DKMS) ne se compile pas encore pour le nouveau noyau Linux.

admin-driver-off = Le pilote ({ $driver }, DKMS) ne se compile pas encore pour le nouveau noyau Linux. Les fonctions qui en ont besoin sont désactivées.

# Said once after the drivers that are off.
admin-driver-off-rest = Le noyau et le reste de la mise à jour ont été installés.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = caméra virtuelle

admin-language-in-use = { $code } est la langue dans laquelle cet ordinateur démarre, elle ne peut donc pas être supprimée.

# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Cet ordinateur n'a pas de ralentissement « split-lock » à désactiver.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle n'a pas trouvé ce disque, ou c'est le disque du système.

# The drive could not be closed where it was open.
admin-games-drive-busy = Le disque est utilisé. Fermez les fichiers ou les programmes ouverts dessus, puis réessayez : { $why }

# Another software change has the read-only system open.
admin-system-busy = Une autre modification de logiciel est en cours. Réessayez quand elle sera terminée.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Rien n'a été modifié, car Trindle n'a pas pu déverrouiller le système : { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle n'a pas pu reverrouiller le système. Il le sera au prochain démarrage : { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle n'a pas pu garder /usr/local modifiable : { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Seul root peut l'exécuter, et pas avec pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Btrfs requis.

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = L'instantané est en place et l'ordinateur démarrera à partir de lui. Trindle n'a pas pu finir de nettoyer l'ancien système. Il réessaiera au prochain démarrage de l'ordinateur : { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = Vous pouvez ouvrir au plus { $most } ports.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle n'a pas pu enregistrer la liste des ports ouverts : { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = Le pare-feu est toujours activé : { $why }

# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = Ce module demande maintenant d'autres ports réseau que ceux que vous avez autorisés. Autorisez-le à nouveau pour les ouvrir.

# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = Un programme ne peut être ajouté que pour la personne qui le demande.

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = Une autre restauration a utilisé le même nom il y a un instant. Attendez quelques secondes, puis réessayez.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } n'a pas pu être déconnecté : { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 dossier partagé n'a pas répondu, il est donc toujours connecté. Il sera déconnecté à l'arrêt de l'ordinateur.
       *[other] { $n } dossiers partagés n'ont pas répondu, ils sont donc toujours connectés. Ils seront déconnectés à l'arrêt de l'ordinateur.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = Impossible de désactiver le fichier d'échange { $path } : { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = Impossible de réactiver le fichier d'échange { $path } : { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } n'est pas un fichier d'échange du système.

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = Trindle n'utilisera pas le fichier d'échange { $path }, car d'autres pourraient modifier ou lire { $why }.

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = Impossible de trouver où est le fichier d'échange, la sortie d'hibernation peut échouer.

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = L'hibernation refonctionnera après un redémarrage.

# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = La puce est déjà en cours de configuration. Attendez la fin.

# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = Ce n'est pas le mot de passe du disque.

# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = Trop de mots de passe incorrects. Attendez une minute, puis réessayez.

# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = La puce a déverrouillé le disque au dernier démarrage. Rien à faire.

# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = Le disque a changé depuis le démarrage. Redémarrez et réessayez.

# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = Puce de sécurité introuvable. Activez-la dans le BIOS et réessayez.

# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = clevis, l'outil qui configure la puce, n'est pas installé.

# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = Le disque n'a plus de place pour une autre clé.

# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = Puce non configurée. Le mot de passe ouvre toujours le disque.

# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = La puce refonctionne, mais son ancienne clé n'a pas pu être retirée.
