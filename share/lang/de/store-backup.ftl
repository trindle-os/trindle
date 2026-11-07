
# machine translated (German), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Es wurde nichts zurückgespielt, weil { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Ein Teil der Sicherung wurde nicht zurückgespielt, weil { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = die Sicherung einen Link oder etwas anderes enthält, das aus dem Ordner herausreichen würde, in den sie kommt

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = auf diesem Computer schon etwas dort liegt, wo ein Teil der Sicherung hin muss

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = es keinen Ort zum Entpacken der Sicherung gibt

# The reason when the backup file cannot be read.
store-because-unreadable = die Sicherungsdatei nicht gelesen werden konnte

# The reason when the file is not a Trindle backup.
store-because-not-trindle = diese Datei keine Trindle-Sicherung ist

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = die Sicherung beschädigt ist und ihre Liste nicht gelesen werden konnte

# The reason when the file is not a Flat backup.
store-because-not-flat = diese Datei nicht wie eine Flat-Sicherung aussieht

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = diese Datei keine Flat-App-Liste enthält

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = sie von einem neueren Flat gemacht wurde, als Trindle lesen kann

# The reason when the file holds no apps.
store-because-no-apps = in dieser Datei keine Apps sind

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = die Sicherung nicht gelesen werden konnte: { $why }

# The reason when the program gave none.
store-because-none = es keinen Grund angegeben hat

# The reason when the system said permission was denied.
store-because-permission = dieser Computer Trindle dort nicht schreiben lässt

# The reason when the disk is full.
store-because-space = auf diesem Computer kein Platz mehr ist

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
store-because-too-big = sie zu groß ist, um ganz gelesen zu werden

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = ihre Einstellungen in der Sicherung nicht zu ihrer Prüfsumme passen

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Zurückgespielt, außer deinem Kontobild. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Dein Kontobild wurde auch nicht zurückgespielt. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec konnte nicht gestartet werden: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Es konnte nicht aus der Sicherung gelesen werden: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Die Sicherung wurde nicht erstellt, weil { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = noch nichts

# Beside the size of a part of the backup: about how big it is.
store-backup-about = etwa { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindles Einstellungen und AppImages werden zurückgespielt ...

# Step shown while the settings and data are put back.
store-backup-step-data = Einstellungen und Daten werden zurückgespielt ...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] { $count } App wird von Flathub installiert ... das kann dauern.
       *[other] { $count } Apps werden von Flathub installiert ... das kann dauern.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Einstellungen und Daten wurden zurückgespielt, aber diese Apps wurden nicht installiert, weil Flatpak sie abgelehnt hat: { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Einstellungen und Daten wurden zurückgespielt, aber diese Apps wurden nicht installiert, weil Flatpak sie abgelehnt hat: { $failed }. Ebenfalls nicht installiert: { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Einstellungen und Daten wurden zurückgespielt, aber diese Apps wurden nicht installiert: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] 1 App übersprungen: Ihr Name war ungültig.
       *[other] { $n } Apps übersprungen: Ihre Namen waren ungültig.
    }

# Said beside an app whose source is not secure: the source's name is given.
# SAFETY: needs a human review before it ships
store-flat-not-secure = die Quelle { $name } nicht verschlüsselt ist und nicht akzeptiert wurde, deshalb wurde sie nicht benutzt

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = die Quelle { $name } nicht auf diesem Computer ist und die Sicherung nicht sagt, wo sie liegt

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = die Einstellungen von { $name } (weil die Kopie in der Sicherung ihre Prüfungen nicht bestanden hat)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (weil der Name ihrer Quelle ungültig ist)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (weil { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (weil Flatpak es abgelehnt hat: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = alles (weil es keinen Ort zum Entpacken der Sicherung gibt)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = was jede App darf

# Said among what could not be done: one app's settings.
store-flat-app-settings = die Einstellungen von { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Was vorher da war, bleibt als { $path } erhalten: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } wird installiert ({ $at } von { $total }) ...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Die Einstellungen von { $name } werden zurückgespielt ({ $at } von { $total }) ...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } App installiert.
       *[other] { $n } Apps installiert.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Einstellungen zurückgespielt für { $n }.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } schon da, so gelassen, wie sie war.
       *[other] { $n } schon da, so gelassen, wie sie waren.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Nicht zurückgespielt: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

