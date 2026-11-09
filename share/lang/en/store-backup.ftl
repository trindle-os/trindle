# Apps: moving apps and settings to another computer, and the reasons a backup was refused (src/store/refusal.rs, backup.rs, put_back.rs, flat.rs).

## Why a backup was refused (src/store/refusal.rs)

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Nothing was put back, because { $why }.
# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Part of the backup was not put back, because { $why }.
# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = the backup holds a link or something else that would reach outside the folder it goes into
# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = something on this computer is already where part of the backup needs to go
# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = there is nowhere to unpack the backup
# The reason when the backup file cannot be read.
store-because-unreadable = the backup file could not be read
# The reason when the file is not a Trindle backup.
store-because-not-trindle = that file is not a Trindle backup
# The reason when the backup's list is damaged or too big.
store-because-damaged-list = the backup is damaged and its list could not be read
# The reason when the file is not a Flat backup.
store-because-not-flat = that file does not look like a Flat backup
# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = that file does not hold a Flat app list
# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = this was made by a newer Flat than Trindle can read
# The reason when the file holds no apps.
store-because-no-apps = there are no apps in that file
# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = the backup could not be read: { $why }
# The reason when the program gave none.
store-because-none = it gave no reason
# The reason when the system said permission was denied.
store-because-permission = this computer would not let Trindle write there
# The reason when the disk is full.
store-because-space = there is no room left on this computer
# The reason when a part of the computer is read-only.
store-because-read-only = that part of this computer is read-only
# The reason when the person's disk quota is used up.
store-because-quota = the space allowed for this account is used up
# The reason when a file or program is missing.
store-because-missing = a file or program it needs is missing
# The reason when no reason could be read from the program's last line.
store-because-failed = failed
# The reason when the backup has no name to be written under.
store-because-no-name = it has no name
# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = its settings in the backup reach outside its own folder
# The reason when something is too big to be read whole.
store-because-too-big = it is too big to be read whole
# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = its settings in the backup do not match their checksum
# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Put back, except your account picture. { $why }
# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Your account picture was not put back either. { $why }
# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec could not be started: { $why }
# Said when the backup could not be read at all.
store-refusal-not-read-from = It could not be read from the backup: { $why }.

## Making and putting back a backup (src/store/backup.rs)

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = The backup was not made, because { $why }.
# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = nothing yet
# Beside the size of a part of the backup: about how big it is.
store-backup-about = about { $size }
# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Putting back Trindle's settings and AppImages...
# Step shown while the settings and data are put back.
store-backup-step-data = Putting back settings and data...
# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Installing { $count } app from Flathub... this can take a while.
       *[other] Installing { $count } apps from Flathub... this can take a while.
    }
# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Settings and data were put back, but these apps were not installed, because Flatpak refused them: { $failed }.
# The same, when other apps were also left out for another reason.
store-not-installed-both = Settings and data were put back, but these apps were not installed, because Flatpak refused them: { $failed }. Also not installed: { $refused }.
# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Settings and data were put back, but these apps were not installed: { $refused }.

## Restoring from a Flat backup (src/store/flat.rs)

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] 1 app skipped: its name was not valid.
       *[other] { $n } apps skipped: their names were not valid.
    }
# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = the { $name } remote is not encrypted and was not accepted, so it was not used
# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = the { $name } remote is not on this computer and the backup does not say where it is
# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name }'s settings (because the backup's copy of them did not pass its checks)
# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (because the name of its source is not valid)
# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (because { $why })
# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (because Flatpak refused it: { $why })
# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = everything (because there is nowhere to unpack the backup)
# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = what every app is allowed
# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name }'s settings
# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. What was there before is kept as { $path }: { $error }
# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Installing { $name } ({ $at } of { $total })...
# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Putting back { $name }'s settings ({ $at } of { $total })...
# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] Installed { $n } app.
       *[other] Installed { $n } apps.
    }
# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Settings put back for { $n }.
# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } already here, left as it was.
       *[other] { $n } already here, left as they were.
    }
# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Not put back: { $list }
# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat
# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })
