
# machine translated (German), not yet reviewed by a person.

# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = Kann nicht deinstalliert werden: Teil von Trindle

# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = Kann nicht deinstalliert werden: Der Computer braucht es

# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Kann nicht deinstalliert werden: Ein Teil von Trindle braucht es

# Red words when an app's saved data was not deleted because its id is not a plain name.
# SAFETY: needs a human review before it ships
common-removal-data-name = Die Daten wurden nicht gelöscht: Der Name der App ist kein schlichter Name.

# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
# SAFETY: needs a human review before it ships
common-removal-data-link = Die Daten wurden nicht gelöscht: Ihr Ordner ist ein Link, kein Ordner.

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
common-updating-sentence-boxes = Sicherheitskorrekturen werden in die Ordner der Apps eingespielt.

# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Der Installer von Debian arbeitet, wie jeden Tag bei Sicherheitsupdates.

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
common-updating-reason-all = Die Updates von Debian und Trindle ändern den ganzen Computer, für alle, die ihn nutzen, darum brauchen sie das Passwort eines Administrators. Apps, die du selbst installiert hast, werden ohne eines aktualisiert.

# Said in the password box when only Debian's updates are put in.
# SAFETY: needs a human review before it ships
common-updating-reason-debian = Die Updates von Debian ändern den ganzen Computer, für alle, die ihn nutzen, darum brauchen sie das Passwort eines Administrators.

# Said in the password box when only Trindle's own update is put in.
# SAFETY: needs a human review before it ships
common-updating-reason-trindle = Das Update von Trindle ändert den ganzen Computer, für alle, die ihn nutzen, darum braucht es das Passwort eines Administrators.

# Said in the password box when new firmware is put in.
# SAFETY: needs a human review before it ships
common-updating-reason-firmware = Neue Firmware ändert den Computer selbst, darum braucht sie das Passwort eines Administrators.

# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
# SAFETY: needs a human review before it ships
common-updating-reason-refresh = Die Suche nach Updates für den ganzen Computer braucht das Passwort eines Administrators, wenn sie nicht vom Bildschirm des Computers selbst aus angefragt wird.

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = weil { $why }.

# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = Es wurde nicht fertig und nannte keinen Grund.

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
common-refused-why-not-allowed = dieses Konto darf das nicht, egal welches Passwort eingegeben wird

# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = es war nichts da, das nach dem Administratorpasswort gefragt hätte

# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = das Administratorpasswort wurde nicht akzeptiert, oder dieses Konto darf das nicht

# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Es wurde kein Administratorpasswort eingegeben

# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Das Administratorpasswort wurde nicht akzeptiert

# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Dieses Konto darf das nicht, egal welches Passwort eingegeben wird

# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Es war nichts da, das nach dem Administratorpasswort gefragt hätte

# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Das Administratorpasswort wurde nicht akzeptiert, oder dieses Konto darf das nicht

# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Nicht ausgeführt: Es wurde kein Administratorpasswort eingegeben.

# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Nicht ausgeführt: Das Administratorpasswort wurde nicht akzeptiert.

# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Nicht ausgeführt: Dieses Konto darf das nicht, egal welches Passwort eingegeben wird. Melde dich ab und wieder an und versuche es dann noch einmal.

# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Nicht ausgeführt: Es war nichts da, das nach dem Administratorpasswort gefragt hätte. Melde dich ab und wieder an und versuche es dann noch einmal.

# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Nicht ausgeführt: Das Administratorpasswort wurde nicht akzeptiert, oder dieses Konto darf das nicht. Wenn nichts danach gefragt hat, melde dich ab und wieder an und versuche es dann noch einmal.

# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Nicht ausgeführt: Der Schritt ist fehlgeschlagen und hat nicht gesagt, warum.

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
common-keyring-turn-failed = Der Zug mit dem Schlüsselbund ließ sich nicht ausführen ({ $why }), darum wurde nichts von ihm verlangt.

# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Der Schlüsselbund hat noch nicht geantwortet, darum ist { $what } bisher nicht entfernt. Falls der Schlüsselbund es doch entfernt, steht das in dieser Zeile.
       *[other] Der Schlüsselbund hat noch nicht geantwortet, darum ist { $what } bisher nicht gespeichert. Falls der Schlüsselbund es doch speichert, steht das in dieser Zeile.
    }

# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Der Schlüsselbund hat spät geantwortet: { $what } ist entfernt.
       *[other] Der Schlüsselbund hat spät geantwortet: { $what } ist gespeichert.
    }

# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Der Schlüsselbund hat nie geantwortet, darum ist { $what } nicht entfernt.
       *[other] Der Schlüsselbund hat nie geantwortet, darum ist { $what } nicht gespeichert.
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
common-renewed-sentence = Dieses Update ist schon in Benutzung, außer { $list }; das wird nach der nächsten Abmeldung die neue Version.

# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = Dieses Update ist schon in Benutzung, außer { $list }; das wird spätestens nach der nächsten Abmeldung die neue Version.

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland wollte nicht starten

# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = Versuche es in einer Minute noch einmal

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each dot with what it means.
common-in-use-key = Was die Punkte bedeuten

# Under the dots in that key: where a dot shows up.
common-in-use-where = Die Punkte erscheinen oben in der Mitte des Bildschirms und auf den Reitern der Fenster.

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
common-in-use-input = Maus und Tastatur übernommen

# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-input-by = { $who } nutzt deine Tastatur und Maus

# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
# SAFETY: needs a human review before it ships
common-in-use-files = Die Dateien, die du kopiert hast, werden gelesen

# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
# SAFETY: needs a human review before it ships
common-in-use-files-by = { $who } liest die Dateien, die du kopiert hast

# The ring's who-is-using-what row while a plugin reaches devices on the home network.
# SAFETY: needs a human review before it ships
common-in-use-home-network = Heimnetz in Benutzung

# The same with the plugin's name.
# SAFETY: needs a human review before it ships
common-in-use-home-network-by = Heimnetz in Benutzung durch { $who }

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
common-in-use-microphone-off-by = Mikrofon ausgeschaltet, gewünscht von { $who }

# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
# SAFETY: needs a human review before it ships
common-in-use-microphone-off-note = Für jedes Programm aus, durch den Mikrofonknopf des Rings.

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = { $what } zurückgesetzt: Datei beschädigt, daneben aufbewahrt.

# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = { $what } nicht gespeichert: Die Datei kann nicht gelesen werden, darum blieb sie, wie sie ist.

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
common-no-driver-known-driver = { $chip } hat hier noch keinen Treiber. Er kommt erstmals mit Linux { $kernel }.

# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = Bei { $chip } fehlt die Firmware. Sie ist in { $package } { $version }.

# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = Kein Treiber für einen { $class }-Chip ({ $id }).

# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = Fehlende { $class }-Firmware: { $file }.

# Added after a Wi-Fi line.
common-no-driver-stick = Ein USB-Wi-Fi-Stick funktioniert in der Zwischenzeit.

# Tip of the small round i button that opens Help.
common-no-driver-more = Mehr dazu

common-no-driver-class-wifi = Wi-Fi

common-no-driver-class-bluetooth = Bluetooth

common-no-driver-class-sound = Ton

common-no-driver-class-graphics = Grafik

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
common-pocketkey-due = Das Passwort wird wieder benötigt: Es sind mehr als { $time } vergangen.

# Said when only an app's saved data is to go and the app is running.
# SAFETY: needs a human review before it ships
common-removal-data-running = Die Daten wurden nicht gelöscht: Die App läuft. Schließen Sie sie zuerst.

# Said when the app's data path is a file, not a folder.
# SAFETY: needs a human review before it ships
common-removal-data-not-folder = Die Daten wurden nicht gelöscht: Es ist eine Datei, kein Ordner.


# A choice of "Ask for the password again after", and the time in the screen's label: { $n } minutes.
common-pocketkey-minutes =
    { $n ->
        [one] 1 Minute
       *[other] { $n } Minuten
    }

