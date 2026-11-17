# machine translated (German), not yet reviewed by a person.

# The page shown before a Flatpak program is installed, with what it will be able to reach.
# Headline over the page; the source of the program is given.
store-fp-headline-from = { $name } von { $host } installieren?

# Headline over the page for a program that has been downloaded.
store-fp-headline = { $name } installieren?

# Name of the card that lists what the program will be able to reach.
store-fp-card = Worauf es zugreifen kann

# Tooltip of the round button that leaves without installing.
store-fp-cancel = Nicht installieren

# The main button.
store-fp-install = Installieren

# Said while Flatpak is being asked what the program wants.
store-fp-reading = Es wird gelesen, was es anfordert…

# Red words when what the program asks for could not be read.
store-fp-unread = Trindle konnte nicht lesen, was es anfordert.

# Red words after Install was pressed once for a program whose list could not be read.
store-fp-unread-again = Drücke noch einmal auf Installieren, um fortzufahren.

# Said when the program is already on this computer; the name is given.
store-fp-have = { $name } ist schon installiert.

# Said when the program asks to reach nothing outside its own box.
store-fp-none = Nichts außerhalb seiner eigenen Box.

# Each line below is one thing the program will be able to reach.
store-fp-network = Das Internet und dein lokales Netzwerk

store-fp-x11 = Fenster anderer Apps und was du darin tippst

store-fp-sound = Ton und dein Mikrofon

store-fp-devices = Deine Kamera und jedes andere angeschlossene Gerät

store-fp-input = Gamecontroller und andere Eingabegeräte

store-fp-kvm = Schnelle virtuelle Maschinen

store-fp-shm = Speicher, der mit anderen Apps geteilt wird

store-fp-usb = USB-Geräte

store-fp-canbus = CAN-Bus-Geräte

store-fp-cups = Deine Drucker

store-fp-multiarch = 32-Bit-Programme ausführen

store-fp-escape = Jedes Programm auf diesem Computer ausführen

# A reference that also adds a second place to get apps from, named by its address.
store-fp-adds-source = { $host } zu deinen App-Quellen hinzufügen

# Last line, when the program asks for more than the lines above say.
store-fp-other = Dazu weitere Einstellungen

store-fp-bluetooth = Bluetooth-Geräte

store-fp-smartcard = Smartcards

store-fp-ssh = Deine SSH-Schlüssel

store-fp-gpg = Deine GPG-Schlüssel

store-fp-devel = Debugging-Werkzeuge, mit denen es Programme beobachten und steuern kann, die es startet

store-fp-session-bus = Jedes Programm, das in deiner Sitzung läuft

store-fp-system-bus = Jeden Dienst auf diesem Computer

# A place the program can open and change; the place is given, as below.
store-fp-files = { $what } öffnen und ändern

# The same for a place it can only look at.
store-fp-files-read = { $what } ansehen

store-fp-where-host = alle Dateien auf diesem Computer

store-fp-where-system = die Systemdateien dieses Computers

store-fp-where-home = deinen ganzen Home-Ordner, versteckte Dateien eingeschlossen

store-fp-where-path = den Ordner { $path }

# A program on your session that it can talk to; the name is given.
store-fp-talk = Mit { $name } sprechen

# A service the program runs on your session.
store-fp-own = Den Dienst { $name } ausführen

store-fp-talk-system = Mit dem Systemdienst { $name } sprechen

store-fp-own-system = Den Systemdienst { $name } ausführen

# Said in red when Install failed and taking the program out again failed too.
store-fp-undo-failed = Es ist vielleicht nur teilweise installiert.

# Said in red when a bundle could not be copied for a safe read.
store-fp-copy-failed = Die Datei konnte nicht kopiert werden, um sie sicher zu lesen.
