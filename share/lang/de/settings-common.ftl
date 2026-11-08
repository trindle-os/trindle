
# machine translated (German), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Gespeichert

# Said beside a button that worked, when nothing more is known.
settings-common-done = Erledigt

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Nicht erledigt

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Über diese Seite

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = Über { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } ist bereits vorhanden

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Für den Rest genauso verfahren ({ $n } weiteres)
       *[other] Für den Rest genauso verfahren ({ $n } weitere)
    }

# Label of the switch that answers the same way for all that wait.
# SAFETY: needs a human review before it ships
settings-common-same-for-rest = Für den Rest genauso verfahren

# Tip of the button that keeps the one already there.
# SAFETY: needs a human review before it ships
settings-common-skip = Überspringen: das bereits vorhandene behalten

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = Ersetzen

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = Das bereits Vorhandene durch das Neue ersetzen

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Das Neue

# Heading of the line that shows the one already there.
settings-common-already-line = Das bereits Vorhandene

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = Das bereits Vorhandene ersetzen?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Es geht in { $left } Sekunde zurück.
       *[other] Es geht in { $left } Sekunden zurück.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Es geht in { $left } Sekunde zurück. { $more }
       *[other] Es geht in { $left } Sekunden zurück. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Jetzt zurück

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Behalten

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } ist nicht verfügbar

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Installiere es mit: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Die Webseite konnte nicht geöffnet werden: Es ließ sich kein Webbrowser starten.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } konnte nicht geöffnet werden.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Es konnte nicht gespeichert werden.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Nicht erledigt: Der Nachrichtenbus des Systems war nicht erreichbar. Starte den Computer neu und versuche es dann erneut.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Nicht erledigt: Der Nachrichtenbus dieser Anmeldung war nicht erreichbar. Melde dich ab und wieder an und versuche es dann erneut.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Das Auswahlfenster konnte nicht geöffnet werden. Melde dich ab und wieder an und versuche es dann erneut.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Der Datenträger ist voll.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Auf den Datenträger kann gerade nicht geschrieben werden.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = Der Teil des Systems, der dies erledigt, läuft nicht.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Die Antwort dauerte zu lange. Versuche es erneut.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = Das Passwort wurde nicht eingegeben.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = Dieses Konto hat dafür keine Berechtigung.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Auf diesem Computer fehlt etwas, das dafür nötig ist.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } und { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = Der Ring öffnet sich mit einem Klick am Bildschirmrand.

# Word drawn in the middle of that picture.
settings-common-ring-name = Der Ring

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Die Firmware des Laptops war nicht erreichbar: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Die Firmware des Laptops hat die Ladegrenze nicht angenommen.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Dieser Computer erlaubt es nicht, das Laden vorzeitig zu beenden.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path } konnte nicht gesetzt werden: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Trindle in einer virtuellen Maschine ausprobieren

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle läuft in { $machine }. Es funktioniert dort gut, aber eine virtuelle Maschine ist nie ganz so schnell und flüssig wie ein echter Computer. Beurteile die Geschwindigkeit daher bitte auf echter Hardware.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle läuft in { $machine } ohne 3D-Grafik, daher muss der Prozessor alles zeichnen, was sonst ein Grafikchip übernimmt. Fenster, Videos und Webseiten wirken langsamer als auf einem echten Computer, und Trindle lässt seine Animationen weg, um zu helfen. Wenn der Computer, auf dem es läuft, es kann, bringt das Einschalten der 3D-Beschleunigung in den Anzeigeeinstellungen der virtuellen Maschine viel. Auf einem echten Computer installiert, gilt nichts davon.
