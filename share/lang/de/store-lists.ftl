# machine translated (German), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Eine Flatpak-Datei

# On an installed thing's line: it came from a .deb file.
store-from-deb = Eine .deb-Datei

# On an installed thing's line: it is a plugin.
store-from-plugin = Plugin

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = Release-Kandidat

# Tip of the round button on an app's tile while it is being removed.
# SAFETY: needs a human review before it ships
store-list-removing = { $name } wird entfernt

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = { $name } wird installiert

# Tip of the round button on an app's tile that opens the app.
store-list-open = { $name } öffnen

# Tip of the round button on an app's tile that installs the app.
store-list-install = { $name } installieren

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Installiert

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Anzeigen

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Geprüft anhand der Threat-Intelligence-Liste von HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Du hast dies von { $site } heruntergeladen, nachdem du von { $referrer } gekommen bist.

# Where a download came from: the site.
store-origin-site = Heruntergeladen von { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Verlinkt von { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle kann nicht sagen, woher dies stammt.

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } steht auf der Threat-Intelligence-Liste von HaGeZi mit Seiten, die für schädliche Downloads und Betrug bekannt sind. Installiere dies nur, wenn du sicher bist, woher es stammt.

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } Die Seite steht nicht auf der Threat-Intelligence-Liste von HaGeZi. Diese Liste ist nicht vollständig, das beweist also nicht, dass sie sicher ist.

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } Trindle konnte die Seite nicht prüfen, weil es die Threat-Intelligence-Liste von HaGeZi nicht laden konnte.

# Name shown for a review whose writer gave none.
store-review-someone = Jemand

# Red words when the keyring could not be reached.
store-key-unreached = Trindle konnte den Schlüsselbund nicht erreichen: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = Der gespeicherte Schlüssel ist kein Text.

# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } konnte nicht entfernt werden: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = { $path } konnte nicht gelesen werden: { $why }
