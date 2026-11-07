
# machine translated (German), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Ein Paket installieren

# Words at the top of the window while the package is read.
store-deb-reading = Das Paket wird gelesen …

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Diese Datei ist kein installierbares Debian-Paket. Vielleicht wurde der Download nicht fertig.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } kann in einem eigenen Ordner installiert werden.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Dieses Paket ist für { $arch }-Computer und kann nicht auf diesen.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Dies ist Trindle selbst. Es wird auf der Seite „Updates“ unter Apps aktualisiert, nicht über eine Datei.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } ist ein Treiber oder ein Hintergrunddienst, der nicht aus einem eigenen Ordner funktioniert, und kann hier nicht installiert werden.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Ein Paket von außerhalb von Debian und Trindle kommt in einen eigenen Ordner, wo es eingeschränkt, aber nicht ganz abgeschottet ist. Installiere bitte nur Pakete von Herstellern, denen du vertraust.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } ist in einem eigenen Ordner installiert.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Es wird geprüft, woher das kommt …

# Heading in the card that says where the file came from.
store-deb-where = Woher es kam

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Entfernen, samt allem, was es gespeichert hat

# Word beside the ring while the package goes in.
store-deb-installing = Wird installiert

# Main button that installs the package.
store-deb-install = Installieren

# Main button that installs the same version again.
store-deb-reinstall = Neu installieren

# Main button that brings the installed package up to this version.
store-deb-update = Aktualisieren

# Heading of the card for a package not installed yet.
store-deb-goes-in = Es kommt in einen eigenen Ordner und ist standardmäßig eingeschränkt

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = Kein Passwort nötig. Es kommt nicht ins Internet, nutzt keinen Ton und greift nicht auf deine Dateien zu, bis du es erlaubst.

# Heading of the card for a package installed at this version.
store-deb-same-version = Version { $version } ist in einem eigenen Ordner installiert

# Under that heading.
store-deb-same-version-note = Installiere es dort neu, wenn es nicht mehr richtig funktioniert.

# Heading of the card for a package installed at another version.
store-deb-other-version = Version { $version } ist in einem eigenen Ordner installiert

# Under that heading: the version this brings.
store-deb-other-version-note = Wenn du dies dort installierst, wird es auf Version { $version } gebracht, und was es gespeichert hat, bleibt erhalten.

# Facts line: how big the package is once installed.
store-deb-fact-size = Größe nach der Installation

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Gemacht für

# Text of the About card when the package has no description.
store-deb-no-description = Dieses Paket hat keine Beschreibung.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Wird in einem eigenen Ordner installiert. Das Holen dessen, was es braucht, kann einige Minuten dauern.

# Words at the top of the window once the package is installed.
store-deb-done = In einem eigenen Ordner installiert. Du findest es im Ring.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = In einem eigenen Ordner installiert, aber diese Teile wurden nicht gefunden, darum startet es vielleicht nicht: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Aus seinem eigenen Ordner entfernt, samt allem, was es gespeichert hat.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Diese Datei ist kein installierbares Debian-Paket.

# Step shown while a package is unpacked.
store-boxed-unpacking = { $name } wird entpackt

# Step shown while a part a package needs is added.
store-boxed-adding = { $part } wird hinzugefügt

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } ist kein installierbares Debian-Paket.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } konnte nicht entpackt werden.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Dieses Programm liegt nicht in einem eigenen Ordner.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal

# Title of the notice when older programs are off.
store-boxed-x-off = Ältere Programme sind aus

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Einstellungen, Berechtigungen

# Title of the notice when too many older programs are open.
store-boxed-x-many = Zu viele ältere Programme

# Body of that notice.
store-boxed-x-many-close = Schließe eines und versuche es noch einmal

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } konnte nicht geöffnet werden

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = Das Terminal kann mehr von deinem Computer erreichen als andere Apps, weil es das für seine Arbeit braucht. Gib nur Befehle ein, die du verstehst, von Leuten, denen du vertraust.

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = Das Heimnetz ist aus

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } hat kein Netzwerk: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = es konnte sein eigenes Programm nicht finden

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare ist nicht installiert

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta ist nicht installiert

# The same: the network room was not made.
store-boxed-home-why-not-made = sein Netzwerkraum wurde nicht erstellt

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = sein Netzwerkraum war nicht neu

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = der Systemaufruf-Filter konnte nicht erstellt werden

# The same: the room's rules would not load.
store-boxed-home-why-rules = die Regeln des Raums ließen sich nicht laden

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } wollte nicht starten: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta sagte: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta konnte dem Raum nicht beitreten



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Ohne Internet geöffnet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Sie wurde heruntergeladen, deshalb hat das Programm nur diese Datei, nur lesbar.

