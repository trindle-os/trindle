
# machine translated (German), not yet reviewed by a person.

# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = Es gehört zu Trindle und bleibt.

# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = Der Computer braucht es, darum bleibt es.

# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Ein Teil von Trindle braucht es, darum bleibt es.

# Red words when an app's saved data was not deleted because its id is not a plain name.
# SAFETY: needs a human review before it ships
common-removal-data-name = Die Daten wurden nicht gelöscht, weil der Name der App ungewöhnliche Zeichen enthält.

# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
# SAFETY: needs a human review before it ships
common-removal-data-link = Die Daten wurden nicht gelöscht, weil ihr Ordner ein Link auf einen anderen Ort ist.

# Red words when an app's saved data could not be deleted.
# SAFETY: needs a human review before it ships
common-removal-data-failed = Die Daten konnten nicht gelöscht werden.

# Name of the updates button, and its tip, while the ring's own update button is putting updates in.
common-updating-short-ring = Aktualisierung läuft

# Name of an update button while Apps is putting updates in.
common-updating-short-apps = Aktualisierung in Apps

# Name of an update button while the daily security fixes for the apps' boxes are going in.
common-updating-short-boxes = Sicherheitskorrekturen werden eingespielt

# Name of an update button while Debian's own installer is at work.
common-updating-short-system = Debian wird aktualisiert

# Name of an update button while something not named is installing updates.
common-updating-short-unknown = Aktualisierung an anderer Stelle

# Headline of an updates page while the ring is installing updates.
common-updating-sentence-ring = Updates werden über den Ring installiert.

# Headline of an updates page while Apps is installing updates.
common-updating-sentence-apps = Updates werden über Apps installiert.

# Headline of an updates page while the daily security fixes are going into the apps' boxes.
common-updating-sentence-boxes = Sicherheitskorrekturen werden für deine Apps installiert.

# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Der Installer von Debian läuft, wie jeden Tag für Sicherheitsupdates.

# Headline of an updates page while something not named is installing updates.
common-updating-sentence-unknown = Updates werden an anderer Stelle installiert.

# Beside an app in a list of updates, when what waits is a newer build of the version already installed. { $version } is the version.
common-updating-newer-build-of = ein neuerer Build von Version { $version }

# Beside an app in a list of updates: the installed version, then the version that waits.
common-updating-version-to = Version { $from } auf { $to }

# Beside an app in a list of updates, when only the installed version is known. { $version } is that version.
common-updating-now-version = jetzt Version { $version }

# Beside an app in a list of updates, when only the new version is known. { $version } is that version.
common-updating-to-version = auf Version { $version }

# Beside a thing whose own version is shown, when what waits is a newer build of the same version.
common-updating-newer-waiting = ein neuerer Build wartet

# Beside a thing whose own version is shown: the version that waits.
common-updating-version-waiting = Version { $version } wartet

# Beside a thing whose own version is shown, when the new version is not known.
common-updating-update-waiting = ein Update wartet

# Said in the password box when Debian's and Trindle's updates are put in together.
# SAFETY: needs a human review before it ships
common-updating-reason-all = Die Updates von Debian und Trindle ändern den ganzen Computer für alle, die ihn nutzen, darum brauchen sie das Passwort eines Administrators. Apps, die du selbst installiert hast, werden ohne Passwort aktualisiert.

# Said in the password box when only Debian's updates are put in.
# SAFETY: needs a human review before it ships
common-updating-reason-debian = Die Updates von Debian ändern den ganzen Computer für alle, die ihn nutzen, darum brauchen sie das Passwort eines Administrators.

# Said in the password box when only Trindle's own update is put in.
# SAFETY: needs a human review before it ships
common-updating-reason-trindle = Das Update von Trindle ändert den ganzen Computer für alle, die ihn nutzen, darum braucht es das Passwort eines Administrators.

# Said in the password box when new firmware is put in.
# SAFETY: needs a human review before it ships
common-updating-reason-firmware = Neue Firmware ändert den Computer selbst, darum braucht sie das Passwort eines Administrators.

# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
# SAFETY: needs a human review before it ships
common-updating-reason-refresh = Um nach Updates für den ganzen Computer zu suchen, brauchst du das Passwort eines Administrators, außer du fragst am Bildschirm des Computers selbst an.

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = weil { $why }.

# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = Es wurde nicht fertig. Es wurde kein Grund genannt.

# Red words beside an update that did not finish. { $why } is the reason the system gave.
common-updating-failed = Es wurde nicht fertig. { $why }

# Red words beside an update that was never started. { $why } is the end of a sentence such as "because no administrator's password was given."
common-updating-not-installed = Nicht installiert, { $why }

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = es wurde kein Administratorpasswort eingegeben

# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = das Administratorpasswort wurde nicht akzeptiert

# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = dein Konto darf das nicht, auch nicht mit dem richtigen Passwort

# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = das Passwortfenster ist nie erschienen

# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = das Administratorpasswort wurde nicht akzeptiert, oder dein Konto darf das nicht

# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Es wurde kein Administratorpasswort eingegeben

# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Das Administratorpasswort wurde nicht akzeptiert

# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Dein Konto darf das nicht, auch nicht mit dem richtigen Passwort

# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Das Passwortfenster ist nie erschienen

# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Das Administratorpasswort wurde nicht akzeptiert, oder dein Konto darf das nicht

# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Das wurde nicht ausgeführt, weil kein Administratorpasswort eingegeben wurde.

# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Das wurde nicht ausgeführt, weil das Administratorpasswort nicht akzeptiert wurde.

# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Das wurde nicht ausgeführt. Dein Konto darf das nicht, auch nicht mit dem richtigen Passwort. Melde dich ab und wieder an und versuche es dann noch einmal.

# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Das wurde nicht ausgeführt, weil das Passwortfenster nie erschienen ist. Melde dich ab und wieder an und versuche es dann noch einmal.

# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Das wurde nicht ausgeführt. Entweder wurde das Administratorpasswort nicht akzeptiert, oder dein Konto darf das nicht. Wenn kein Passwortfenster erschienen ist, melde dich ab und wieder an und versuche es dann noch einmal.

# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Das wurde nicht ausgeführt. Etwas ist fehlgeschlagen, und es wurde kein Grund genannt.

# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = Der Schlüsselbund ist gesperrt. Melde dich ab und wieder an oder entsperre ihn und versuche es dann noch einmal.

# Red words when this computer has no keyring to keep a login or key in.
# SAFETY: needs a human review before it ships
common-keyring-missing = Auf diesem Computer gibt es keinen Schlüsselbund, in dem es gespeichert werden könnte.

# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Der Schlüsselbund hat nicht rechtzeitig geantwortet.

# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Der Schlüsselbund hat nie geantwortet.

# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Trindle konnte nicht auf den Schlüsselbund zugreifen ({ $why }), darum wurde er nicht benutzt.

# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Der Schlüsselbund hat noch nicht geantwortet, darum wurde { $what } noch nicht entfernt. Falls der Schlüsselbund es doch entfernt, steht das in dieser Meldung.
       *[other] Der Schlüsselbund hat noch nicht geantwortet, darum wurde { $what } noch nicht gespeichert. Falls der Schlüsselbund es doch speichert, steht das in dieser Meldung.
    }

# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Der Schlüsselbund hat spät geantwortet, und { $what } wurde entfernt.
       *[other] Der Schlüsselbund hat spät geantwortet, und { $what } wurde gespeichert.
    }

# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Der Schlüsselbund hat nie geantwortet, darum wurde { $what } nicht entfernt.
       *[other] Der Schlüsselbund hat nie geantwortet, darum wurde { $what } nicht gespeichert.
    }

# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Der Schlüsselbund hat spät geantwortet und { $what } nicht entfernt: { $why }
       *[other] Der Schlüsselbund hat spät geantwortet und { $what } nicht gespeichert: { $why }
    }

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the ring.
common-renewed-ring = den Ring

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the window that opens and saves files for other programs.
common-renewed-portal = das Fenster zum Öffnen und Speichern

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the wallpaper.
common-renewed-wallpaper = den Hintergrund

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the clipboard.
common-renewed-clipboard = die Zwischenablage

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the keyring.
common-renewed-keyring = den Schlüsselbund

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that dims the screen and locks it.
common-renewed-idle = Abdunkeln und Sperren

# Name of a part of Trindle, said in the sentence about what is not yet the new version: night light.
common-renewed-nightlight = das Nachtlicht

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that lets other computers find this one.
common-renewed-wsdd = die Netzwerksuche

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the plugins that run in the background.
common-renewed-plugin = im Hintergrund laufende Plugins

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the desktop itself.
common-renewed-compose = die Oberfläche selbst

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the link an AI assistant is using.
common-renewed-mcp = die Verbindung, die ein KI-Assistent nutzt

# Joins two or more names of parts in a list, all but the last: "the ring, the keyring". { $first } is the list so far; { $next } is the next name.
common-renewed-join = { $first }, { $next }

# Joins the last name of a list of parts to the rest: "the ring, the keyring and night light". { $most } is the list so far.
common-renewed-join-last = { $most } und { $last }

# Sentence on Settings and Apps after an update, when some parts stay the old program until the next log out. { $list } is the names of the parts.
common-renewed-sentence = Dieses Update ist schon in Benutzung, außer bei { $list }; dort wechselt es nach deiner nächsten Abmeldung auf die neue Version.

# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = Dieses Update ist schon in Benutzung, außer bei { $list }; dort wechselt es spätestens bei deiner nächsten Abmeldung auf die neue Version.

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland wurde nicht gestartet

# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = Versuche es in einer Minute noch einmal

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each mark with what it means.
common-in-use-key = Was Zeichen bedeuten

# Under the marks in that key: where a mark shows up. On the screen it is a half circle hung from the top edge; on a window tab it is a small dot.
common-in-use-where = Die Halbkreise hängen oben in der Mitte des Bildschirms. Fensterreiter zeigen kleine Punkte.

# Screen reader name of the microphone dot and its row, and the line shown on pointing at the dots, when no program is named.
# SAFETY: needs a human review before it ships
common-in-use-microphone = Mikrofon in Benutzung

# Line on pointing at the dots, and its screen reader name, when programs use the microphone. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-microphone-by = Mikrofon in Benutzung durch { $who }

# Screen reader name of the camera dot and its row, and the line shown on pointing at the dots, when no program is named.
# SAFETY: needs a human review before it ships
common-in-use-camera = Kamera in Benutzung

# Line on pointing at the dots, and its screen reader name, when programs use the camera. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-camera-by = Kamera in Benutzung durch { $who }

# Screen reader name of the screen dot and its row, and the line shown on pointing at the dots, when no program is named.
# SAFETY: needs a human review before it ships
common-in-use-screen = Bildschirm wird aufgenommen

# Line on pointing at the dots, and its screen reader name, when programs record the screen. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-screen-by = Bildschirm wird aufgenommen von { $who }

# Screen reader name of the input dot and its row, and the line shown on pointing at the dots, when no program is named: the mouse and keyboard are in another program's hands.
# SAFETY: needs a human review before it ships
common-in-use-input = Maus und Tastatur belegt

# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-input-by = { $who } nutzt deine Tastatur und Maus

# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
# SAFETY: needs a human review before it ships
common-in-use-files = Die Dateien, die du kopiert hast, werden gelesen

# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-files-by = { $who } liest die Dateien, die du kopiert hast

# The ring's who-is-using-what row while a plugin reaches devices on the local network.
# SAFETY: needs a human review before it ships
common-in-use-home-network = Lokales Netzwerk genutzt

# The same with the plugin's name.
# SAFETY: needs a human review before it ships
common-in-use-home-network-by = Lokales Netzwerk genutzt von { $who }

# The ring's who-is-using-what row while a plugin uses scanners and USB devices.
# SAFETY: needs a human review before it ships
common-in-use-usb = Scanner und USB-Geräte in Benutzung

# The same with the plugin's name.
# SAFETY: needs a human review before it ships
common-in-use-usb-by = Scanner und USB-Geräte in Benutzung durch { $who }

# Line on pointing at the dots when the microphone is switched off and the program listening hears nothing.
# SAFETY: needs a human review before it ships
common-in-use-microphone-off = Mikrofon ausgeschaltet

# Line on pointing at the dots when the microphone is switched off but programs want it. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-microphone-off-by = Mikrofon ausgeschaltet. { $who } wollte es nutzen.

# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
# SAFETY: needs a human review before it ships
common-in-use-microphone-off-note = Du hast es mit dem Mikrofonknopf am Ring für jedes Programm ausgeschaltet.

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = { $what } wurde zurückgesetzt, weil die Datei beschädigt war. Die beschädigte Datei wurde daneben aufbewahrt.

# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = { $what } wurde nicht gespeichert, weil Trindle die Datei nicht lesen konnte. Es wurde nichts geändert.

# What the file kept by the list of programs a box may use is called, in the words above.
common-unreadable-name-box-permissions = App-Berechtigungen

# What the file that notes which questions an app has already put to the person is called, in the words above.
common-unreadable-name-box-asked = Schon gestellte Fragen

# What the file that notes which apps came with the image is called, in the words above.
common-unreadable-name-box-offered = Beim ersten Anmelden angebotene Apps

# What the lists of a fenced program (its folders, hosts, settings and programs) are called, in the words above.
common-unreadable-name-fence = Zaunlisten

# What the list of icons the desktop found missing is called, in the words above.
common-unreadable-name-missing-icons = Liste fehlender Symbole

# What the screen arrangement an older version kept in display.sh is called, in the words above.
common-unreadable-name-old-display = Bildschirmanordnung

# Said where the person works when a known chip has no driver yet. The kernel is a version number.
common-no-driver-known-driver = { $chip } hat hier noch keinen Treiber. Linux { $kernel } ist die erste Version, die einen enthält.

# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = Bei { $chip } fehlt die Firmware, sie ist in { $package } { $version } enthalten.

# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = Es gibt keinen Treiber für diesen { $class }-Chip ({ $id }).

# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = Fehlende { $class }-Firmware: { $file }.

# Added after a Wi-Fi line.
common-no-driver-stick = Ein USB-Wi-Fi-Stick funktioniert vorerst.

# Tip of the small round i button that opens Help.
common-no-driver-more = Mehr dazu

common-no-driver-class-wifi = Wi-Fi

common-no-driver-class-bluetooth = Bluetooth

common-no-driver-class-sound = Ton

common-no-driver-class-graphics = Grafik

# Said of a device with no driver that nothing is known about. The name is its name, or its two hex numbers.
common-no-driver-device-none = { $name } hat keinen Treiber.

# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Mehr

common-more-spoken = Mehr dazu, öffnet die Hilfe


# machine translated (German), not yet reviewed by a person.

# First choice of "Ask for the password again after": the phone stays good for as long as it likes.
# SAFETY: needs a human review before it ships
common-pocketkey-never = Nie

# A choice of "Ask for the password again after", and the time in the screen's label: { $n } hours.
# SAFETY: needs a human review before it ships
common-pocketkey-hours =
    { $n ->
        [one] 1 Stunde
       *[other] { $n } Stunden
    }

# A choice of "Ask for the password again after", and the time in the screen's label: { $n } days.
# SAFETY: needs a human review before it ships
common-pocketkey-days =
    { $n ->
        [one] 1 Tag
       *[other] { $n } Tage
    }

# A choice of "Ask for the password again after", and the time in the screen's label: { $n } weeks.
# SAFETY: needs a human review before it ships
common-pocketkey-weeks =
    { $n ->
        [one] 1 Woche
       *[other] { $n } Wochen
    }

# On the lock and login screens, when the time chosen has passed and the phone is not accepted: { $time } is such as "1 hour".
# SAFETY: needs a human review before it ships
common-pocketkey-due = Es sind mehr als { $time } vergangen, darum musst du dein Passwort noch einmal eingeben.

# Said under the greyed Earlier versions card.
common-snap-no-snapper = Frühere Versionen brauchen snapper. Das ist auf diesem Computer nicht installiert.

# Said under the greyed Earlier versions card.
common-snap-from-snapshot = Nicht, solange der Computer von einem Snapshot gestartet wurde.

# Said under the greyed Earlier versions card.
common-snap-part-of-system = Der persönliche Ordner ist auf diesem Computer Teil des Systems und kein eigener Teil der Festplatte. Deshalb können frühere Versionen nicht getrennt vom System aufbewahrt werden.

# Said under the greyed Earlier versions card.
common-snap-network = Der persönliche Ordner liegt auf einem anderen Computer. Deshalb können hier keine früheren Versionen aufbewahrt werden.

# Said under the greyed Earlier versions card: { $fs } is the kind of file system, such as ext4.
common-snap-not-btrfs = Frühere Versionen brauchen den persönlichen Ordner auf Btrfs. Hier liegt er auf { $fs }.

# Said under the greyed Earlier versions card.
common-snap-not-whole = Dein persönlicher Ordner liegt innerhalb eines anderen Teils der Festplatte und nicht auf einem eigenen Teil, deshalb kann Trindle keine früheren Versionen davon aufbewahren.

# Said under the greyed Earlier versions card.
common-snap-read-only = Der persönliche Ordner kann im Moment nicht beschrieben werden. Deshalb kann nichts aufbewahrt werden.

# Said under the greyed Earlier versions card.
common-snap-locked-files = Ein persönlicher Ordner wird hier als eine gesperrte Datei aufbewahrt. Jeden Tag eine Kopie davon würde die Festplatte füllen.

# Said under the greyed Earlier versions card.
common-snap-elsewhere = Dein persönlicher Ordner liegt nicht in /home, wo die Kopien gemacht werden.

# Said under the greyed Earlier versions card.
common-snap-apart = Dein persönlicher Ordner ist gesperrt oder getrennt von den anderen gespeichert. Deshalb können die Kopien ihn nicht enthalten.

# Said under the greyed Earlier versions card.
common-snap-own-part = Dein persönlicher Ordner ist ein eigener Teil der Festplatte. In die Kopien von /home kommt er nicht hinein.

# Why a plugin's port line in plugin.ini is refused. { $text } is what was written.
common-firewall-not-protocol-port = „{ $text }“ ist nicht als Protokoll:Port geschrieben, zum Beispiel tcp:24810.

# Why a plugin's port line in plugin.ini is refused. { $text } is what was written before the colon.
common-firewall-bad-protocol = „{ $text }“ ist weder tcp noch udp. Schreibe es klein.

# Why a plugin's port line in plugin.ini is refused. { $text } is what was written, { $lowest } the lowest port allowed.
common-firewall-bad-port = „{ $text }“ ist kein Port von { $lowest } bis 65535.

# Said when a port you open by hand isn't written as protocol:port.
common-firewall-own-form = Schreibe es als Protokoll:Port, zum Beispiel tcp:37666.

# Said when a port you open by hand starts with something other than tcp or udp.
common-firewall-own-protocol = Nimm tcp oder udp.

# Said when a port you open by hand isn't a number from 1 to 65535.
common-firewall-own-port = Das ist kein Port von 1 bis 65535.

# Joins the last port to the others in a list of ports. { $first } is the others, comma separated.
common-firewall-and = { $first } und { $last }

# Heads a list of ports. { $count } is how many, { $list } is the ports, such as TCP 24810 and UDP 24811.
common-firewall-ports =
    { $count ->
        [one] Port { $list }
       *[other] Ports { $list }
    }

# Why a plugin's ports are refused. { $max } is the most it may name.
common-firewall-too-many = Ein Plugin kann höchstens { $max } Ports nennen.

# Why a plugin's ports are refused. { $port } is a port such as TCP 24810.
common-firewall-twice = { $port } ist zweimal genannt.

# Why a plugin's ports are refused. { $port } is a port such as TCP 80, { $lowest } the lowest allowed.
common-firewall-too-low = { $port } liegt unter { $lowest }. Diese Ports sind für das System reserviert.

# Why a plugin's port is refused. { $port } is the number.
common-firewall-reserved = Port { $port } wird von Trindle oder einem gängigen Dienst benutzt. Ein Plugin kann ihn deshalb nicht anfordern.

# Why a plugin's port can't be opened. { $port } is such as TCP 24810, { $plugin } is the other plugin's name.
common-firewall-taken = { $port } wird schon von { $plugin } benutzt. Deshalb kann er nicht auch für dieses Plugin geöffnet werden.

# Why a plugin cannot open another port: { $max } is how many may be open.
common-firewall-full = Die höchste erlaubte Zahl offener Ports für Plugins, { $max }, ist schon erreicht.

# Told by a notice when the ring has been started again.
common-ring-held-back = Der Ring ist mehrmals hintereinander angehalten worden, deshalb hat Trindle ihn eine Weile pausiert. Jetzt läuft er wieder.

# Told by a notice when the ring has been started again.
common-ring-not-started = Der Ring konnte eine Weile nicht gestartet werden. Jetzt läuft er wieder.

# Told by a notice when the ring has been started again.
common-ring-not-ended = Der Ring hat nicht mehr geantwortet und konnte eine Weile nicht beendet werden. Jetzt läuft er wieder.

# Told by a notice.
common-ring-not-done = Der Computer sollte in den Ruhezustand gehen, neu starten oder herunterfahren und hat es nicht getan.

# Told by a notice; the ring is what the round menu is called.
common-ring-by-name = Die Arbeitsumgebung konnte diesen Ring nicht selbst starten. Deshalb kann er sich nicht über einem Vollbildfenster öffnen, und seine Hinweise bleiben nicht im Vordergrund. Abmelden und wieder Anmelden hilft vielleicht.

# A group of apps in the ring.
common-category-internet = Internet

# A group of apps in the ring (as above).
common-category-office = Büro

# A group of apps in the ring (as above).
common-category-graphics = Grafik

# A group of apps in the ring (as above).
common-category-media = Ton und Video

# A group of apps in the ring (as above).
common-category-games = Spiele

# A group of apps in the ring (as above).
common-category-development = Entwicklung

# A group of apps in the ring (as above).
common-category-learning = Lernen

# A group of apps in the ring (as above).
common-category-utilities = Hilfsprogramme

# A group of apps in the ring (as above).
common-category-system = System

# Why an app cannot be started: its launcher names nothing to run.
common-app-no-command = Es gibt keinen Befehl zum Ausführen.

# Why a launcher was left alone: { $why } is the reason.
common-launcher-unchanged = Es wurde nichts geändert, weil { $why }.

# The ring's notice when something is plugged in.
common-drive-arrived-usb = USB-Laufwerk angeschlossen

# The ring's notice when something is plugged in (as above).
common-drive-arrived-card = Speicherkarte angeschlossen

# The ring's notice when something is plugged in (as above).
common-drive-arrived-disc = Disc eingelegt

# The ring's notice when something is plugged in (as above).
common-drive-arrived-other = Laufwerk angeschlossen

# Said in the password box after what the password is for.
common-drive-every-time = Du wirst jedes Mal gefragt, wenn ein Laufwerk geändert wird. Wenn du dieses Fenster schließt, ändert sich nichts.

# Said in the password box: { $rest } is what is being done, such as "format this drive".
common-drive-password-needed-to = Ein Administratorpasswort wird gebraucht, um { $rest }

# Said in the password box when the action is not named.
common-drive-password-needed = Ein Administratorpasswort wird gebraucht, um dieses Laufwerk zu ändern

# Why a folder cannot be shared.
common-share-not-yours = Nur Ordner, die dir gehören, können freigegeben werden.

# Why a share cannot have the name asked for.
common-share-name-is-login = Dieser Name ist der Anmeldename von jemandem auf diesem Computer. Deshalb kann eine Freigabe ihn nicht haben. Wähle einen anderen.

# Why a share cannot have the name asked for.
common-share-bad-name = Der Name einer Freigabe darf keines dieser Zeichen enthalten: % < > * ? | / \ + = ; : " und auch kein Komma.

# Why a folder cannot be shared.
common-share-no-guests = Die Samba-Einstellungen dieses Computers erlauben keine Freigaben ohne Passwort.

# Why a folder cannot be shared.
common-share-most = Dieser Computer gibt schon so viele Ordner frei, wie er erlaubt.

# Why a folder cannot be shared.
common-share-gone = Dieser Ordner ist nicht mehr da.

# Why a folder cannot be shared, when the program said nothing.
common-share-no-reason = Samba hat nicht gesagt, warum.

# Why a folder cannot be shared: { $said } is what the sharing program said.
common-share-said = Samba sagt: { $said }

# Why a screenshot could not be taken.
common-shot-no-desktop = Es gibt keine Arbeitsfläche zum Fotografieren.

# Why a screenshot could not be taken.
common-shot-no-display = Es gibt keinen Bildschirm zum Fotografieren.

# Why a screenshot could not be taken.
common-shot-no-display-there = In diesem Teil des Desktops gibt es keinen Bildschirm.

# Why a screenshot could not be taken.
common-shot-cannot = Diese Arbeitsfläche kann nicht fotografiert werden.

# Why a screenshot could not be taken.
common-shot-too-big = Der Screenshot ist zu groß.

# Why a screenshot could not be taken.
common-shot-no-size = Ein Bildschirm hat ein leeres Bild geliefert, deshalb konnte Trindle keinen Screenshot machen.

# Why a screenshot could not be taken.
common-shot-unreadable = Die Arbeitsfläche hat kein Bild angeboten, das gelesen werden kann.

# Why a screenshot could not be taken.
common-shot-no-memory = Für das Bild war nicht genug Speicher da.

# Why a screenshot could not be taken.
common-shot-stopped = Die Arbeitsfläche hat aufgehört, den Bildschirm zu fotografieren.

# Why a screenshot could not be taken.
common-shot-failed = Die Arbeitsfläche konnte den Bildschirm nicht fotografieren.

# Said where the person is working when no screen reader is installed.
common-reader-none-yet = Es ist kein Screenreader installiert.

# Said wherever hiding a drive is offered or shown.
common-drive-not-a-lock = Verstecken sperrt oder schützt es nicht. Ein Zwilling davon, der genau dasselbe über sich sagt, wird mit versteckt.

# Said when typed words are not a change to a key.
common-remap-not-a-key = Das ist keine Taste, die Trindle kennt. Versuche Escape, Strg+C oder Linksklick.

# Said when typed words are not a change to a key.
common-remap-too-long = Das sind mehr Tasten, als eine Änderung fassen kann.

# Said when typed words are not a change to a key or a button.
common-no-equals = Eine Änderung braucht das, was gedrückt wird, ein Gleichheitszeichen und das, was daraus wird.

# Said when typed words are not a change to a key.
common-remap-tapped-held = Sage, was beim kurzen Tippen und was beim Halten passiert, zum Beispiel Escape getippt, Strg gehalten.

# Said when typed words are not a change to a controller button.
common-pads-not-a-button = Das ist keine Taste, die Trindle kennt. Versuche South, Left Trigger oder Escape.

# Said when typed words are not a change to a controller.
common-pads-not-a-zone = Eine tote Zone geht von 0 % bis 50 %.

# Said of a line in the controllers file that is not understood.
common-pads-not-a-heading = Trindle erkennt diese Controller-Überschrift nicht.

# Said of a line in the controllers file that is not understood.
common-pads-no-heading = Die Zeile steht unter keiner Controller-Überschrift.

# What a theme's tile says when its file holds more than a theme may.
common-theme-too-big = Diese Datei ist zu groß für ein Thema und wird deshalb nicht benutzt. Ein Thema ist höchstens 64 KB groß.

# What a theme's tile says when it is not an ordinary file.
common-theme-not-a-sheet = Das ist keine gewöhnliche Textdatei. Deshalb kann sie kein Thema sein und wird nicht benutzt.

# What is said of a theme that was chosen and whose file has since gone.
common-theme-gone = Die Datei ist nicht mehr da.

# What is said of a theme on a disk that does not answer.
common-theme-too-slow = Das Lesen der Datei hat zu lange gedauert, deshalb nutzt Trindle sie nicht.

# What is said when the folders of themes are on a disk that does not answer.
common-theme-list-slow = Das Lesen der Themenordner hat zu lange gedauert. Deshalb steht nur das eigene Aussehen von Trindle in der Liste.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-big = Dein eigenes Stylesheet style.css ist größer als 256 KB und wird deshalb nicht benutzt.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-not-a-sheet = Dein eigenes Stylesheet style.css ist keine gewöhnliche Textdatei und wird deshalb nicht benutzt.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-slow = Das Lesen deines eigenen Stylesheets style.css hat zu lange gedauert. Deshalb wird es nicht benutzt.

# What is said when colours in the person's own style sheet fall short of what can be read.
common-theme-own-hard-to-read = Einige Farben sind schlecht lesbar.

# What is said under the tiles when a folder holds more themes than are shown.
common-theme-not-all = Ein Themenordner kann mehr enthalten, als aufgelistet wird: Pro Ordner werden nur die ersten vierzig Themen gezeigt, und nur die ersten 400 Dateien werden angesehen.

# What is said of the person's own style sheet when it cannot be used.
common-theme-own-no-target = Dein eigenes Stylesheet style.css ist ein Link auf eine Datei, die nicht da ist, und wird deshalb nicht benutzt.

# The name of a file set aside when damaged, said in a sentence: Clipboard history.
common-damaged-clipboard = Zwischenablage-Verlauf

# The name of a file set aside when damaged, said in a sentence: Last-used apps.
common-damaged-last-used = Zuletzt benutzte Apps

# What kind of virtual machine this is, said in a sentence.
common-vm-qemu = eine virtuelle QEMU-Maschine (GNOME Boxes, virt-manager und ähnliche)

# What kind of virtual machine this is, said in a sentence.
common-vm-apple = eine virtuelle Maschine eines Macs

# What kind of virtual machine this is, said in a sentence: { $name } is the name systemd gives it.
common-vm-other = eine virtuelle Maschine ({ $name })

# What a program offered in Software sources is for, in a line.
common-source-claude = Die Claude-App von Anthropic: Chat, Cowork und Claude Code

# What a program offered in Software sources is for, in a line.
common-source-tailscale = Ein privates Netzwerk zwischen deinen eigenen Geräten

# What a program offered in Software sources is for, in a line.
common-source-syncthing = Hält Ordner auf deinen Computern gleich, ohne Cloud. Die Version von Debian ist älter.

# What a program offered in Software sources is for, in a line.
common-source-jellyfin = Dein eigener Server für Filme, Musik und Fotos, für jeden Bildschirm im Haus

# What a program offered in Software sources is for, in a line.
common-source-nodejs = Das aktuelle Node.js mit Langzeitunterstützung, für Entwickler. Debian hat selbst Version 20.

# What a program offered in Software sources is for, in a line.
common-source-github-cli = GitHub im Terminal, für Entwickler. Die Version von Debian ist älter.

# What a program offered in Software sources is for, in a line.
common-source-docker = Führt Programme in Containern aus; für Entwickler

# Said in the box that asks for a drive's passphrase: { $name } is the drive.
common-volumes-passphrase-for = Passphrase für { $name }

# The title of the box that asks for a drive's passphrase.
common-volumes-unlock-title = Laufwerk entsperren

# What a snapshot is, in the Earlier versions list: made just before software was changed.
common-snap-before-change = Vor einer Softwareänderung

# What a snapshot is, in the Earlier versions list: made just after software was changed.
common-snap-after-change = Nach einer Softwareänderung

# What a snapshot is, in the Earlier versions list: made when the computer started.
common-snap-boot = Beim Start des Computers

# What a snapshot is, in the Earlier versions list: one taken on a schedule.
common-snap-regular = Eine regelmäßige Kopie

# What a snapshot is, in the Earlier versions list, when it has no note.
common-snap-plain = Ein Snapshot

# Said when a font file can't be added. { $name } is the file's name, { $error } the system's reason.
common-pack-not-copied = Trindle konnte { $name } nicht kopieren: { $error }

# Said when a file given as a font isn't one. { $name } is the file's name.
common-pack-not-fonts = { $name } ist weder eine Schriftart noch ein Archiv mit Schriftarten.

# Said when a drive can't be hidden because it has no name of its own to remember it by.
common-hidden-no-name = Trindle kann sich dieses Laufwerk nicht merken, weil es keinen eigenen Namen hat.

# Said when the list of hidden drives can't be read. { $error } is the system's reason.
common-hidden-left = Die Liste der versteckten Laufwerke blieb, wie sie war: { $error }

# Said when a drive can't be released because your login session can't be found.
common-leave-no-session = Trindle konnte deine Anmeldesitzung nicht finden. Deshalb wurde nichts geändert.

# Said under the sound choices when PipeWire gives no answer and says nothing about why.
common-audio-no-answer = PipeWire hat nicht geantwortet.

# Said when the screen reader can't be turned on or off. { $error } is the first problem, { $undo } the second.
common-screen-reader-undo-failed = { $error } Trindle konnte die Änderung auch nicht rückgängig machen: { $undo }

# How long copies are kept, in a sentence such as "kept for a week".
common-keep-week = eine Woche

# How long copies are kept, in a sentence such as "kept for two weeks".
common-keep-two-weeks = zwei Wochen

# How long copies are kept, in a sentence such as "kept for a month".
common-keep-month = einen Monat

# How long copies are kept, in a sentence such as "kept for 3 days". { $n } is the number of days.
common-keep-days =
    { $n ->
        [one] 1 Tag
       *[other] { $n } Tage
    }

# Said when an app can't be started. { $program } is the command, { $error } the system's reason.
common-app-not-started = Trindle konnte { $program } nicht starten: { $error }

# Said when a settings file is too big to read.
common-keyfile-too-big = Diese Datei ist zu groß zum Lesen.

# Said when a file Trindle was about to write at login isn't yours.
common-login-file-not-yours = Die Datei dort gehört dir nicht.

# Said when only an app's saved data is to go and the app is running.
# SAFETY: needs a human review before it ships
common-removal-data-running = Die Daten wurden nicht gelöscht, weil die App läuft. Schließe sie zuerst.

# Said when the app's data path is a file, not a folder.
# SAFETY: needs a human review before it ships
common-removal-data-not-folder = Die Daten wurden nicht gelöscht, weil Trindle dort eine Datei gefunden hat, wo ihr Ordner sein sollte.


# A choice of "Ask for the password again after", and the time in the screen's label: { $n } minutes.
common-pocketkey-minutes =
    { $n ->
        [one] 1 Minute
       *[other] { $n } Minuten
    }

common-steam-unreadable = Die Einstellungen von Steam konnten nicht gelesen werden, daher wurde nichts geändert.

common-flatpak-not-an-app = Das ist kein App-Name.

common-flatpak-not-a-permission = Das ist keine Berechtigung, die Trindle ändern kann.

common-fonts-too-big = Diese Datei ist zu groß für einen Satz Schriften, daher wurde nichts entpackt.

common-fonts-unpack-failed = Diese Datei konnte nicht entpackt werden. Sie ist vielleicht beschädigt oder nicht das, was sie vorgibt zu sein.

common-fonts-unpack-error = Diese Datei konnte nicht entpackt werden: { $error }

common-fonts-none = In dieser Datei sind keine Schriften. Siehe Hilfe, Eigene Schriften hinzufügen.

common-displays-refused = Der Desktop konnte die Anzeigen nicht so einstellen.

common-displays-refused-why = Der Desktop konnte die Anzeigen nicht so einstellen: { $why }.

common-displays-kept-changing = Die Anzeigen haben sich ständig geändert, daher wurde nichts geändert.

# Said where displays are set when Settings is not running in a desktop.
common-displays-no-desktop = Trindle konnte den Desktop nicht finden.

common-displays-cannot-say = Dieser Desktop kann nicht sagen, welche Anzeigen es gibt.

common-displays-did-not-say = Der Desktop hat nicht gesagt, welche Anzeigen es gibt.

common-displays-none-on = Dann wäre keine Anzeige an.

common-displays-stopped-answering = Der Desktop antwortet nicht mehr.

common-pin-digits = Eine PIN hat { $n } Ziffern.

common-pin-not-hashed = Die PIN konnte nicht gespeichert werden.

common-themes-no-such = Es gibt kein Thema mit diesem Namen.

common-themes-not-kept = Das Thema konnte nicht behalten werden: { $error }.

# Said where a theme file is added, when its name doesn't end in .css. { $name } is the file's name.
common-theme-add-not-css = { $name } ist kein Thema. Ein Thema ist eine Datei, deren Name auf .css endet.

# Said where a theme file is added, when it can't be read. { $name } is the file's name.
common-theme-add-unreadable = Trindle konnte { $name } nicht lesen.

# Said where a theme file is added, when it is over 64 KB. { $name } is the file's name.
common-theme-add-too-big = { $name } ist zu groß für ein Thema. Ein Thema ist höchstens 64 KB groß.

# Said where a theme file is added, when it holds more than plain text. { $name } is the file's name.
common-theme-add-not-a-sheet = { $name } ist kein Stylesheet. Die Datei enthält mehr als Text.

# Said where a theme file is added, when reading it took too long. { $name } is the file's name.
common-theme-add-too-slow = Das Lesen von { $name } hat zu lange gedauert.

common-volumes-wrong-passphrase = Mit dieser Passphrase ließ es sich nicht öffnen.

common-volumes-already-open = Es ist schon offen.

common-volumes-needs-passphrase = Zum Öffnen braucht es eine Passphrase.

common-volumes-window-failed = Das Fenster für die Passphrase ließ sich nicht öffnen.

# Under Trindle's own look on Settings, Appearance: a few words about it.
common-theme-trindle-about = Trindles eigener Look: eine tiefblaue Nacht mit weichen Kanten.

# Under the Bold theme on Settings, Appearance.
common-theme-bold-about = Schwarz und Weiß mit klaren Kanten, größere Schrift und größere Schaltflächen. Für schwache Augen.

# Under the Midnight theme on Settings, Appearance.
common-theme-midnight-about = Dunkel und flach, in Graustufen: Schwarz und Grau, ohne Kanten, kleine Ecken.

# Under the Paper theme on Settings, Appearance.
common-theme-paper-about = Warmweißes Papier auf einem cremefarbenen Tisch, für helle Räume. Ein heller Look.

# Under the Snug theme on Settings, Appearance.
common-theme-snug-about = Kleiner und enger, eckig und kühles Grau. Auf einem kleinen Bildschirm passt mehr hin.

# What the security updates switch says on a hover while it is on.
common-auto-updates-on = Automatische Updates an

# What the security updates switch says on a hover while it is off.
common-auto-updates-off = Automatische Updates aus

# Said when polkit's list of the passwords it is keeping for this login couldn't be read, so Trindle can't make sure a password given for a drive is forgotten.
common-leave-list-unread = Es ließ sich nicht lesen, welche Passwörter noch gespeichert sind.
