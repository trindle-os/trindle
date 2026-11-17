
# machine translated (French), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Un fichier Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Un fichier .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Module

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = version candidate

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's tile while it is being removed.
store-list-removing = Suppression de { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Installation de { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Ouvrir { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Installer { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Installée

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Afficher

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Vérifié avec la liste Threat Intelligence de HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Vous avez téléchargé ceci depuis { $site }, en venant de { $referrer }.

# Where a download came from: the site.
store-origin-site = Téléchargé depuis { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Lien depuis { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle ne peut pas dire d'où cela vient.

# SAFETY: needs a human review before it ships
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name } figure sur la liste Threat Intelligence de HaGeZi, qui recense des sites connus pour leurs téléchargements nuisibles et leurs arnaques. N'installez ceci que si vous êtes sûr de sa provenance.

# SAFETY: needs a human review before it ships
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } Le site ne figure pas sur la liste Threat Intelligence de HaGeZi. Cette liste n'est pas complète, donc cela ne prouve pas qu'il est sûr.

# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } Trindle n'a pas pu vérifier le site, car il n'a pas pu récupérer la liste Threat Intelligence de HaGeZi.

# Name shown for a review whose writer gave none.
store-review-someone = Quelqu'un

# Red words when the keyring could not be reached.
store-key-unreached = Trindle n'a pas pu joindre le trousseau : { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = La clé enregistrée n'est pas du texte.

# SAFETY: needs a human review before it ships
# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } n'a pas pu être supprimé : { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = { $path } n'a pas pu être lu : { $why }

