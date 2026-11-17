
# machine translated (French), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Installer un paquet

# Words at the top of the window while the package is read.
store-deb-reading = Lecture du paquet…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Ce fichier n'est pas un paquet Debian installable. Son téléchargement n'est peut-être pas terminé.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } est prêt à être installé dans son propre dossier.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Ce paquet est pour des ordinateurs { $arch }, il ne peut donc pas être installé sur celui-ci.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = C'est Trindle lui-même. Mettez-le à jour depuis la page Mises à jour d'Apps, pas depuis un fichier.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } est un pilote ou un service en arrière-plan. Il ne peut pas fonctionner depuis son propre dossier, il ne peut donc pas être installé ici.

# SAFETY: needs a human review before it ships
# Words on the caution card above every package.
store-deb-caution = Un paquet venu d'ailleurs que Debian et Trindle s'installe dans son propre dossier. Il y est restreint, mais pas totalement isolé. N'installez que des paquets de créateurs en qui vous avez confiance.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } est installé dans son propre dossier.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Vérification de la provenance…

# Heading in the card that says where the file came from.
store-deb-where = D'où il vient

# SAFETY: needs a human review before it ships
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = Le supprimer, avec ses données

# Word beside the ring while the package goes in.
store-deb-installing = Installation

# Main button that installs the package.
store-deb-install = Installer

# Main button that installs the same version again.
store-deb-reinstall = Réinstaller

# Main button that brings the installed package up to this version.
store-deb-update = Mettre à jour

# Heading of the card for a package not installed yet.
store-deb-goes-in = Il s'installe dans son propre dossier restreint

# SAFETY: needs a human review before it ships
# Under that heading.
store-deb-goes-in-note = Aucun mot de passe n'est nécessaire. Il ne peut utiliser ni Internet ni le son, ni voir vos fichiers, tant que vous ne l'autorisez pas.

# Heading of the card for a package installed at this version.
store-deb-same-version = La version { $version } est installée dans son propre dossier

# Heading of the card for a package installed at another version.
store-deb-other-version = La version { $version } est installée dans son propre dossier

# Under that heading: the version this brings.
store-deb-other-version-note = Cela le met à la version { $version } et garde ce qu'il a enregistré.

# Facts line: how big the package is once installed.
store-deb-fact-size = Taille une fois installé

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Conçu pour

# Text of the About card when the package has no description.
store-deb-no-description = Ce paquet n'a pas de description.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Installation dans son propre dossier. Le téléchargement des éléments dont il a besoin peut prendre quelques minutes.

# Words at the top of the window once the package is installed.
store-deb-done = Il est installé dans son propre dossier. Vous le trouverez dans l'anneau.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Il est installé dans son propre dossier, mais ces éléments sont manquants, il peut donc ne pas démarrer : { $parts }.

# SAFETY: needs a human review before it ships
# Words at the top of the window once the package is removed.
store-deb-removed = Il a été supprimé de son propre dossier, avec tout ce qu'il avait enregistré.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Ce fichier n'est pas un paquet Debian installable.

# Step shown while a package is unpacked.
store-boxed-unpacking = Décompression de { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = Ajout de { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } n'est pas un paquet Debian installable.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } n'a pas pu être décompressé.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Ce programme ne s'exécute pas dans une boîte.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal

# Title of the notice when older programs are off.
store-boxed-x-off = Les anciens programmes sont désactivés

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Réglages, Autorisations

# Title of the notice when too many older programs are open.
store-boxed-x-many = Trop d'anciens programmes

# Body of that notice.
store-boxed-x-many-close = Fermez-en un et réessayez

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } n'a pas pu s'ouvrir

# SAFETY: needs a human review before it ships
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = Le terminal peut accéder à plus de choses sur votre ordinateur que les autres applications. Ne tapez que des commandes que vous comprenez, venant de personnes de confiance.

# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = Pas de réseau local

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } n'a pas de réseau : { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = le programme de Trindle est introuvable

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare n'est pas installé

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta n'est pas installé

# The same: the network room was not made.
store-boxed-home-why-not-made = son réseau n'a pas été configuré

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = son réseau existait déjà

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle n'a pas pu mettre en place un filtre de sécurité

# The same: the room's rules would not load.
store-boxed-home-why-rules = ses règles réseau n'ont pas pu être chargées

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } n'a pas pu démarrer : { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta a dit : { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta n'a pas pu le connecter

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = Le dossier de premier niveau { $name } est un lien, le paquet ne peut donc pas être utilisé.

# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } est un pilote ou un service d'arrière-plan. Ceux-ci ne peuvent pas fonctionner depuis leur propre dossier, il n'a donc pas été installé.



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Ouvert sans internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Comme il a été téléchargé, le programme ne peut lire que ce fichier.

# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = Les anciens programmes ne peuvent pas s'ouvrir

# Body of that notice: what to do.
store-boxed-x-unasked-why = Déconnectez-vous puis reconnectez-vous, puis réessayez
