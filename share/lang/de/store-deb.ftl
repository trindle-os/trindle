
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
store-deb-elsewhere = Dieses Paket ist für { $arch }-Computer und kann nicht auf diesem installiert werden.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Dies ist Trindle selbst. Aktualisiere es auf der Seite „Updates“ unter Apps, nicht über eine Datei.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } ist ein Treiber oder ein Hintergrunddienst. Das kann nicht aus einem eigenen Ordner laufen, daher lässt es sich hier nicht installieren.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Ein Paket von außerhalb von Debian und Trindle wird in einem eigenen Ordner installiert. Dort ist es eingeschränkt, aber nicht ganz abgeschottet. Installiere nur Pakete von Herstellern, denen du vertraust.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } ist in einem eigenen Ordner installiert.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Es wird geprüft, woher das kommt …

# Heading in the card that says where the file came from.
store-deb-where = Woher es kam

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Es und seine Daten entfernen

# Word beside the ring while the package goes in.
store-deb-installing = Wird installiert

# Main button that installs the package.
store-deb-install = Installieren

# Main button that installs the same version again.
store-deb-reinstall = Neu installieren

# Main button that brings the installed package up to this version.
store-deb-update = Aktualisieren

# Heading of the card for a package not installed yet.
store-deb-goes-in = Es wird in einem eigenen, eingeschränkten Ordner installiert

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = Kein Passwort nötig. Es kann das Internet und den Ton nicht nutzen und deine Dateien nicht sehen, bis du es erlaubst.

# Heading of the card for a package installed at this version.
store-deb-same-version = Version { $version } ist in einem eigenen Ordner installiert

# Heading of the card for a package installed at another version.
store-deb-other-version = Version { $version } ist in einem eigenen Ordner installiert

# Under that heading: the version this brings.
store-deb-other-version-note = Dadurch wird es auf Version { $version } aktualisiert, und was es gespeichert hat, bleibt erhalten.

# Facts line: how big the package is once installed.
store-deb-fact-size = Größe nach der Installation

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Gemacht für

# Text of the About card when the package has no description.
store-deb-no-description = Dieses Paket hat keine Beschreibung.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Wird in einem eigenen Ordner installiert. Das Herunterladen der benötigten Teile kann einige Minuten dauern.

# Words at the top of the window once the package is installed.
store-deb-done = Es ist in einem eigenen Ordner installiert. Du findest es im Ring.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Es ist in einem eigenen Ordner installiert, aber diese Teile fehlen, darum startet es vielleicht nicht: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Es wurde aus seinem eigenen Ordner entfernt, samt allem, was es gespeichert hat.

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
store-loose-not-boxed = Dieses Programm läuft nicht in einem eigenen Ordner.

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
store-boxed-terminal-told = Das Terminal kann mehr von deinem Computer erreichen als andere Apps. Gib nur Befehle ein, die du verstehst, von Leuten, denen du vertraust.

# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = Kein lokales Netzwerk

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } hat kein Netzwerk: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = Trindles Programm fehlt

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare ist nicht installiert

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta ist nicht installiert

# The same: the network room was not made.
store-boxed-home-why-not-made = sein Netzwerk wurde nicht eingerichtet

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = sein Netzwerk gab es schon

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle konnte keinen Sicherheitsfilter einrichten

# The same: the room's rules would not load.
store-boxed-home-why-rules = seine Netzwerkregeln ließen sich nicht laden

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } konnte nicht starten: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta sagte: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta konnte es nicht verbinden

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = Der oberste Ordner { $name } ist ein Link. Deshalb kann das Paket nicht benutzt werden.

# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } ist ein Treiber oder ein Hintergrunddienst. Solche Programme können nicht aus ihrem eigenen Ordner arbeiten. Deshalb wurde es nicht installiert.



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Ohne Internet geöffnet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Weil sie heruntergeladen wurde, kann das Programm nur diese eine Datei lesen.

# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = Ältere Programme lassen sich nicht öffnen

# Body of that notice: what to do.
store-boxed-x-unasked-why = Melde dich ab und wieder an und versuche es dann noch einmal
