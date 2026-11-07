
# machine translated (French), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Ce n'est pas un identifiant valide.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Cet identifiant est trop long.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-none = Il n'y a aucun utilisateur nommé { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-system = { $name } est un compte système et n'est pas touché.

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
admin-user-only-admin = { $name } est le seul administrateur, il le reste donc.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Vous ne pouvez pas supprimer votre propre compte.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = { $name } a été supprimé, mais sa photo reste conservée pour l'écran de connexion : { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Cette image est trop grande.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Ce n'est pas une image créée par Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Le dossier des images n'a pas pu être ouvert à l'écran de connexion : { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Ce n'est pas un nom de langue.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = La liste des langues de Debian n'a pas pu être lue : { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } n'est pas une langue connue de Debian.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Un nom d'ordinateur se compose de lettres, de chiffres et de traits d'union, jusqu'à 63, sans commencer ni finir par un trait d'union.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = La limite est active pour l'instant, mais sera perdue à l'extinction de l'ordinateur : { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = La charge est complète pour l'instant, mais la limite reviendra au prochain démarrage : { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Les autres ordinateurs peuvent encore se connecter, car l'interrupteur n'a pas pu être désactivé : { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Ce n'est pas un identifiant d'extension.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Cette personne est inconnue.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Le dossier de cette personne est inconnu.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = La liste des extensions autorisées n'a pas pu être conservée : { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Cette extension est introuvable.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Rien n'a été mis à jour : la version de Trindle présente sur cet ordinateur n'a pas pu être déterminée, donc la mise à jour de Trindle elle-même n'a pas pu être exclue.

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
admin-package-not-installed = Non effectué : il n'est pas installé.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Ce n'est pas un appareil connu de fwupd.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first } ; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Impossible de savoir qui a demandé Tailscale.

# Red words in Settings when the site blocking did not start in time.
# SAFETY: needs a human review before it ships
admin-blocking-late = Le blocage n'a pas démarré à temps.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
# SAFETY: needs a human review before it ships
admin-blocking-not-off = Le blocage n'est pas entièrement désactivé : { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = { $path } n'a pas pu être remis en place : { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Ce n'est pas un nom de site.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Aucun serveur de ce nom.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool } : { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } s'est arrêté avant d'avoir tout reçu, on ne peut donc pas dire que c'est terminé : { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } a échoué.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } n'a pas pu être supprimé : { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Rien n'a été modifié, car { $why } : { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Le retour à l'état précédent n'a pas fonctionné non plus : { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Ce n'est pas une durée de conservation possible pour les versions antérieures.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Le dossier des instantanés n'existe pas.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Quelque chose qui n'est pas un dossier se trouve à l'emplacement des instantanés.

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
admin-share-folder-changed = { $path } n'est pas dans l'état où Trindle l'a laissé, rien n'y est donc monté.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
# SAFETY: needs a human review before it ships
admin-share-private-changed = { $path } n'est pas dans l'état où Trindle l'a laissé.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path } : { $why }

# Red words in Folders when no more shared folders can be opened.
# SAFETY: needs a human review before it ships
admin-share-too-many = Trop de dossiers partagés sont ouverts.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } n'est pas installé. Il se trouve dans le paquet Debian { $package }.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = L'identifiant n'a pas pu être lu.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Ce qui est monté sur cet ordinateur n'a pas pu être lu : { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
# SAFETY: needs a human review before it ships
admin-share-busy = Un programme a encore quelque chose d'ouvert dessus. Fermez-le d'abord.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
# SAFETY: needs a human review before it ships
admin-share-not-answering = Cet ordinateur ne répond pas. Il est fermé quand vous fermez votre session.

# Red words when the number given for a person is not a user number.
# SAFETY: needs a human review before it ships
admin-user-id-invalid = Ce n'est pas un identifiant d'utilisateur.

# Red words in Settings when the group that may share folders could not be made.
# SAFETY: needs a human review before it ships
admin-sharing-group = Le groupe sambashare n'a pas pu être créé.

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
admin-disk-unknown = Impossible de savoir sur quel disque se trouve le système.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
# SAFETY: needs a human review before it ships
admin-system-subvolume = Le système n'est pas dans un sous-volume Btrfs à lui seul, il ne peut donc pas être remplacé par un instantané.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Impossible de récupérer la clé de signature depuis { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Ce que cette adresse a renvoyé n'est pas une clé de signature.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Rien n'a été modifié, car { $path } est illisible : { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Récupération de sa clé de signature

# Second line of the progress while a source is added.
admin-step-adding-source = Ajout et lecture de sa liste

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = La source n'a pas pu être utilisée. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = La source n'a pas pu être utilisée, elle a donc été retirée. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } ne peut pas encore l'activer ni le désactiver : { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = il n'a pas pu être démarré pour { $user } : { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = { $user } n'a pas pu être autorisé à utiliser des machines virtuelles, dont Cowork a besoin : { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = { $user } n'a pas pu être autorisé à l'utiliser sans mot de passe : { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } est installé, mais sa configuration ne s'est pas terminée : { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Installation

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Ce n'est pas l'une des sources prêtes à l'emploi.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } n'est pas conçu pour ce type d'ordinateur ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Configuration

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Les adresses doivent commencer par https:// et ne rien contenir d'autre.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = La suite et les composants doivent être des mots simples.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Ce n'est pas un nom de paquet.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Retrait

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Nouvelle lecture des listes

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Seules les sources ajoutées par Trindle sont retirées ici.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = La source est toujours là : { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
admin-snapshot-missing = L'instantané { $n } n'est pas sur le disque.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-not-ready = Rien n'a été modifié : l'instantané n'a pas pu être préparé. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-aside = Impossible de mettre le système actuel de côté : { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = le système actuel s'appelle maintenant { $aside } et non { $sub }, et l'ordinateur ne démarrera pas tant qu'il ne s'appellera pas de nouveau { $sub } : { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-snapshots-across = Impossible de déplacer les instantanés : { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first } ; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-in-place = Impossible de mettre l'instantané en place : { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-list = La liste de ce qui a été mis de côté n'a pas pu être mise à jour : { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
# SAFETY: needs a human review before it ships
admin-restore-not-removed = Impossible de supprimer : { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
# SAFETY: needs a human review before it ships
admin-tidied-away = Plus rien n'en avait besoin, ils ont donc été supprimés : { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Toutes les mises à jour n'ont pas été installées. Retenus : { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = La fonction « { $feature } » est désactivée : son pilote ({ $driver }, DKMS) ne se compile pas encore sur le nouveau noyau.

admin-driver-off = Un pilote ({ $driver }, DKMS) ne se compile pas encore sur le nouveau noyau, donc ce qu'il apporte est désactivé.

# Said once after the drivers that are off.
admin-driver-off-rest = Le noyau et le reste de la mise à jour ont été installés.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = caméra virtuelle

admin-language-in-use = { $code } est la langue dans laquelle cet ordinateur démarre, elle reste donc.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Cet ordinateur n'a pas de ralentissement « split-lock » à désactiver.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Ce disque est introuvable ou contient le système.

# The drive could not be closed where it was open.
admin-games-drive-busy = Le disque est utilisé. Fermez ce qui est ouvert dessus, puis réessayez : { $why }

# Another software change has the read-only system open.
admin-system-busy = Une autre modification de logiciel est en cours. Réessayez quand elle sera terminée.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Rien n'a été modifié : le système n'a pas pu être déverrouillé : { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Le système n'a pas pu être reverrouillé. Il le sera au prochain démarrage : { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = /usr/local n'a pas pu rester modifiable : { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Seul root peut l'exécuter, et pas avec pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Nécessite Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = L'instantané est en place et l'ordinateur démarrera à partir de lui. Le nettoyage de l'ancien système n'est pas terminé et sera retenté au prochain démarrage : { $why }

