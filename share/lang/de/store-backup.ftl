
# machine translated (German), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Es wurde nichts zurückgespielt, weil { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Trindle hat nur einen Teil der Sicherung zurückgespielt, weil { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = die Sicherung einen Link oder etwas anderes enthält, das aus dem Ordner herausreichen würde, in den sie kommt

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = auf diesem Computer schon etwas dort liegt, wo ein Teil der Sicherung hin muss

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = es keinen Ort zum Entpacken der Sicherung gibt

# The reason when the file is not a Trindle backup.
store-because-not-trindle = diese Datei keine Trindle-Sicherung ist

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = die Sicherung beschädigt ist und Trindle ihre Liste nicht lesen konnte

# The reason when the file is not a Flat backup.
store-because-not-flat = diese Datei nicht wie eine Flat-Sicherung aussieht

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = diese Datei keine Flat-App-Liste enthält

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = sie von einer neueren Version von Flat gemacht wurde, als Trindle lesen kann

# The reason when the file holds no apps.
store-because-no-apps = in dieser Datei keine Apps sind

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = die Sicherung nicht gelesen werden konnte: { $why }

# The reason when the program gave none.
store-because-none = kein Grund genannt

# The reason when the system said permission was denied.
store-because-permission = Trindle dort nicht schreiben darf

# The reason when the disk is full.
store-because-space = dieser Computer keinen Platz mehr hat

# The reason when a part of the computer is read-only.
store-because-read-only = dieser Teil des Computers nur lesbar ist

# The reason when the person's disk quota is used up.
store-because-quota = der für dieses Konto erlaubte Platz aufgebraucht ist

# The reason when a file or program is missing.
store-because-missing = eine Datei oder ein Programm, das es braucht, fehlt

# The reason when no reason could be read from the program's last line.
store-because-failed = es fehlgeschlagen ist

# The reason when the backup has no name to be written under.
store-because-no-name = sie keinen Namen hat

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = ihre Einstellungen in der Sicherung aus ihrem eigenen Ordner herausreichen

# The reason when something is too big to be read whole.
store-because-too-big = sie zu groß zum Lesen ist

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = ihre Einstellungen in der Sicherung beschädigt oder verändert wurden

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Alles wurde zurückgespielt, außer deinem Kontobild. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Trindle hat auch dein Kontobild nicht zurückgespielt. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = Das Passwort konnte nicht abgefragt werden: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Trindle konnte es nicht aus der Sicherung lesen: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Trindle hat die Sicherung nicht erstellt, weil { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = noch nichts

# Beside the size of a part of the backup: about how big it is.
store-backup-about = etwa { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindles Einstellungen und AppImages werden zurückgespielt …

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Trindle hat deine Einstellungen und Daten zurückgespielt, aber Flatpak wollte diese Apps nicht installieren: { $failed }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Trindle hat deine Einstellungen und Daten zurückgespielt, aber diese Apps nicht installiert: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] Trindle hat 1 App übersprungen, weil ihr Name ungültig war.
       *[other] Trindle hat { $n } Apps übersprungen, weil ihre Namen ungültig waren.
    }

# Said beside an app whose source is not secure: the source's name is given.
# SAFETY: needs a human review before it ships
store-flat-not-secure = die Quelle { $name } nicht verschlüsselt ist, darum hat Trindle sie nicht benutzt

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = die Quelle { $name } nicht auf diesem Computer ist und die Sicherung nicht sagt, wo sie liegt

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = die Einstellungen von { $name } (weil die Kopie in der Sicherung ihre Prüfungen nicht bestanden hat)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (weil der Name ihrer Quelle ungültig ist)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (weil { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (Flatpak konnte es nicht installieren: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = alles (weil es keinen Ort zum Entpacken der Sicherung gibt)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = Berechtigungen für jede App

# Said among what could not be done: one app's settings.
store-flat-app-settings = die Einstellungen von { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Was vorher da war, bleibt als { $path } erhalten: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } wird installiert ({ $at } von { $total }) …

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Die Einstellungen von { $name } werden zurückgespielt ({ $at } von { $total }) …

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } App installiert.
       *[other] { $n } Apps installiert.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
        [one] Die Einstellungen einer App zurückgespielt.
       *[other] Die Einstellungen von { $n } Apps zurückgespielt.
    }

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } App war schon installiert.
       *[other] { $n } Apps waren schon installiert.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Ausgelassen: { $list }

# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }: Trindle hat nur die Berechtigungen zurückgespielt, die Zugriff entziehen.
