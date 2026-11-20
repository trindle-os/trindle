# Words of the shared parts that no one program owns: why an app cannot be uninstalled, what updating and renewing say, what the keyring says, the X11 rooms, and the "in use by" lines. Each program shows them where it needs them.

## Uninstalling (src/common/removal.rs)

# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = It's part of Trindle. It stays.
# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = The computer needs it, so it stays.
# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Part of Trindle needs it, so it stays.
# Red words when an app's saved data was not deleted because its id is not a plain name.
common-removal-data-name = Its data wasn't deleted, because the app's name has unusual characters in it.
# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
common-removal-data-link = Its data wasn't deleted, because its folder is a link to somewhere else.
# Red words when an app's saved data could not be deleted.
common-removal-data-failed = Its data couldn't be deleted.
# Said when only an app's saved data is to go and the app is running.
common-removal-data-running = Its data wasn't deleted, because the app is running. Close it first.
# Said when the app's data path is a file, not a folder.
common-removal-data-not-folder = Its data wasn't deleted, because Trindle found a file where its folder should be.

## One update at a time (src/common/updating.rs)

# Name of the updates button, and its tip, while the ring's own update button is putting updates in.
common-updating-short-ring = Updating
# Name of an update button while Apps is putting updates in.
common-updating-short-apps = Updating in Apps
# Name of an update button while the daily security fixes for the apps' boxes are going in.
common-updating-short-boxes = Adding security fixes
# Name of an update button while Debian's own installer is at work.
common-updating-short-system = Debian is updating
# Name of an update button while something not named is installing updates.
common-updating-short-unknown = Updating elsewhere
# Headline of an updates page while the ring is installing updates.
common-updating-sentence-ring = Updates are being installed from the ring.
# Headline of an updates page while Apps is installing updates.
common-updating-sentence-apps = Updates are being installed from Apps.
# Headline of an updates page while the daily security fixes are going into the apps' boxes.
common-updating-sentence-boxes = Security fixes are being installed for your apps.
# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Debian's installer is running, as it does each day for security updates.
# Headline of an updates page while something not named is installing updates.
common-updating-sentence-unknown = Updates are being installed elsewhere.

## What an update does to a version (src/common/updating.rs)

# Beside an app in a list of updates, when what waits is a newer build of the version already installed. { $version } is the version.
common-updating-newer-build-of = a newer build of version { $version }
# Beside an app in a list of updates: the installed version, then the version that waits.
common-updating-version-to = version { $from } to { $to }
# Beside an app in a list of updates, when only the installed version is known. { $version } is that version.
common-updating-now-version = now version { $version }
# Beside an app in a list of updates, when only the new version is known. { $version } is that version.
common-updating-to-version = to version { $version }
# Beside a thing whose own version is shown, when what waits is a newer build of the same version.
common-updating-newer-waiting = a newer build is waiting
# Beside a thing whose own version is shown: the version that waits.
common-updating-version-waiting = version { $version } is waiting
# Beside a thing whose own version is shown, when the new version is not known.
common-updating-update-waiting = an update is waiting

## Why the administrator's password is asked for (src/common/updating.rs)

# Said in the password box when Debian's and Trindle's updates are put in together.
common-updating-reason-all = Debian and Trindle updates change the whole computer for everyone who uses it, so they need an administrator's password. Apps you installed yourself update without one.
# Said in the password box when only Debian's updates are put in.
common-updating-reason-debian = Debian updates change the whole computer for everyone who uses it, so they need an administrator's password.
# Said in the password box when only Trindle's own update is put in.
common-updating-reason-trindle = Trindle's update changes the whole computer for everyone who uses it, so it needs an administrator's password.
# Said in the password box when new firmware is put in.
common-updating-reason-firmware = New firmware changes the computer itself, so it needs an administrator's password.
# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
common-updating-reason-refresh = You need an administrator's password to check for updates to the whole computer, unless you ask from the computer's own screen.

## How an update ended (src/common/updating.rs)

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = because { $why }.
# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = It didn't finish. No reason was given.
# Red words beside an update that did not finish. { $why } is the reason the system gave.
common-updating-failed = It didn't finish. { $why }
# Red words beside an update that was never started. { $why } is the end of a sentence such as "because no administrator's password was given."
common-updating-not-installed = Not installed, { $why }

## Why a step behind the administrator's password was refused (src/common/refused.rs)

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = no administrator's password was given
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = the administrator's password wasn't accepted
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = your account isn't allowed to do that, even with the right password
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = the password box never appeared
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = the administrator's password wasn't accepted, or your account isn't allowed to do that
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = No administrator's password was given
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = The administrator's password wasn't accepted
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = Your account isn't allowed to do that, even with the right password
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = The password box never appeared
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = The administrator's password wasn't accepted, or your account isn't allowed to do that
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = That wasn't done, because no administrator's password was given.
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = That wasn't done, because the administrator's password wasn't accepted.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = That wasn't done. Your account isn't allowed to do it, even with the right password. Log out and back in, then try again.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = That wasn't done, because the password box never appeared. Log out and back in, then try again.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = That wasn't done. Either the administrator's password wasn't accepted, or your account isn't allowed to do it. If no password box appeared, log out and back in, then try again.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = That wasn't done. Something failed, and no reason was given.

## The keyring (src/common/keyring_said.rs)

# Red words when the keyring will not open without the person's password.
common-keyring-locked = The keyring is locked. Log out and in again, or unlock it, then try again.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = There's no keyring on this computer to save it in.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = The keyring didn't answer in time.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = The keyring never answered.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Trindle couldn't get access to the keyring ({ $why }), so it didn't use it.
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] The keyring hasn't answered yet, so the { $what } hasn't been removed. If the keyring removes it after all, this message will say so.
       *[other] The keyring hasn't answered yet, so the { $what } hasn't been saved. If the keyring saves it after all, this message will say so.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] The keyring answered late, and the { $what } has been removed.
       *[other] The keyring answered late, and the { $what } has been saved.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] The keyring never answered, so the { $what } hasn't been removed.
       *[other] The keyring never answered, so the { $what } hasn't been saved.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] The keyring answered late and didn't remove the { $what }: { $why }
       *[other] The keyring answered late and didn't save the { $what }: { $why }
    }

## After an update, which parts are still the old program (src/common/renewed.rs)

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the ring.
common-renewed-ring = the ring
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the window that opens and saves files for other programs.
common-renewed-portal = the open and save window
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the wallpaper.
common-renewed-wallpaper = the wallpaper
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the clipboard.
common-renewed-clipboard = the clipboard
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the keyring.
common-renewed-keyring = the keyring
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that dims the screen and locks it.
common-renewed-idle = dimming and locking
# Name of a part of Trindle, said in the sentence about what is not yet the new version: night light.
common-renewed-nightlight = night light
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that lets other computers find this one.
common-renewed-wsdd = the network finder
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the plugins that run in the background.
common-renewed-plugin = plugins running in the background
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the desktop itself.
common-renewed-compose = the desktop itself
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the link an AI assistant is using.
common-renewed-mcp = the link an AI assistant is using
# Joins two or more names of parts in a list, all but the last: "the ring, the keyring". { $first } is the list so far; { $next } is the next name.
common-renewed-join = { $first }, { $next }
# Joins the last name of a list of parts to the rest: "the ring, the keyring and night light". { $most } is the list so far.
common-renewed-join-last = { $most } and { $last }
# Sentence on Settings and Apps after an update, when some parts stay the old program until the next log out. { $list } is the names of the parts.
common-renewed-sentence = This update is already in use, except for { $list }, which will switch to the new version after you next log out.
# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = This update is already in use, except for { $list }, which will switch to the new version the next time you log out, at the latest.

## Older programs in boxes (src/common/x11_rooms.rs)

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland didn't start
# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = Try again in a minute

## What is using the microphone, camera, screen, input or copied files (src/common/in_use.rs)

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each mark with what it means.
common-in-use-key = What marks mean
# Under the marks in that key: where a mark shows up. On the screen it is a half circle hung from the top edge; on a window tab it is a small dot.
common-in-use-where = The half circles hang from the top middle of the screen. Window tabs show small dots.

# Screen reader name of the microphone dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-microphone = Microphone in use
# Line on pointing at the dots, and its screen reader name, when programs use the microphone. { $who } is the programs' names.
common-in-use-microphone-by = Microphone in use by { $who }
# Screen reader name of the camera dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-camera = Camera in use
# Line on pointing at the dots, and its screen reader name, when programs use the camera. { $who } is the programs' names.
common-in-use-camera-by = Camera in use by { $who }
# Screen reader name of the screen dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-screen = Screen being recorded
# Line on pointing at the dots, and its screen reader name, when programs record the screen. { $who } is the programs' names.
common-in-use-screen-by = Screen being recorded by { $who }
# Screen reader name of the input dot and its row, and the line shown on pointing at the dots, when no program is named: the mouse and keyboard are in another program's hands.
common-in-use-input = Mouse and keyboard busy
# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
common-in-use-input-by = { $who } is using your keyboard and mouse
# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-files = The files you copied are being read
# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
common-in-use-files-by = { $who } is reading the files you copied

# The ring's who-is-using-what row while a plugin reaches devices on the local network.
common-in-use-home-network = Local network used

# The same with the plugin's name.
common-in-use-home-network-by = Local network used by { $who }

# The ring's who-is-using-what row while a plugin uses scanners and USB devices.
common-in-use-usb = Scanners and USB devices in use

# The same with the plugin's name.
common-in-use-usb-by = Scanners and USB devices in use by { $who }
# Line on pointing at the dots when the microphone is switched off and the program listening hears nothing.
common-in-use-microphone-off = Microphone switched off
# Line on pointing at the dots when the microphone is switched off but programs want it. { $who } is the programs' names.
common-in-use-microphone-off-by = Microphone switched off. { $who } tried to use it.
# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
common-in-use-microphone-off-note = You switched it off for every program with the microphone button on the ring.

## A settings file that is damaged or cannot be read (src/common/damaged.rs). Help section: settings-unreadable

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = { $what } was reset, because its file was damaged. The damaged file has been kept next to it.
# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = { $what } wasn't saved, because Trindle couldn't read its file. Nothing has been changed.
# Said where the person is working when one of Steam's own settings files is there but cannot be read, so it is left as it is.
common-steam-unreadable = Steam's settings couldn't be read, so nothing was changed.
# Said when a permission is asked to be changed for something that is not an app's name.
common-flatpak-not-an-app = That isn't an app's name.
# Said when a permission is asked for that Trindle doesn't know.
common-flatpak-not-a-permission = That isn't a permission Trindle can change.
# Said where fonts are being added when the file chosen holds far more than a set of fonts is, so none of it was unpacked.
common-fonts-too-big = That file is too big to be a set of fonts, so nothing was unpacked.
# Said where fonts are being added when the file chosen could not be unpacked.
common-fonts-unpack-failed = That file couldn't be unpacked. It may be damaged, or not the kind it claims to be.
# Said where fonts are being added when the program that unpacks files could not run. { $error } is the system's reason.
common-fonts-unpack-error = That file couldn't be unpacked: { $error }
# Said where fonts are being added when the file chosen holds no fonts. Help has a section on adding fonts.
common-fonts-none = There are no fonts in that file. See Help, adding fonts.
## Displays (src/common/outputs.rs)

# Said where displays are set when the desktop refused the change and gave no reason.
common-displays-refused = The desktop couldn't set the displays that way.
# Said where displays are set when the desktop refused the change. { $why } is the desktop's own reason.
common-displays-refused-why = The desktop couldn't set the displays that way: { $why }.
# Said where displays are set when the displays changed again and again while the change was being made.
common-displays-kept-changing = The displays kept changing, so nothing was changed.
# Said where displays are set when Settings is not running in a desktop.
common-displays-no-desktop = Trindle couldn't find the desktop.
# Said where displays are set when the desktop has no way to list its displays.
common-displays-cannot-say = This desktop can't say what displays there are.
# Said where displays are set when the desktop gave no list of its displays.
common-displays-did-not-say = The desktop didn't say what displays there are.
# Said where displays are set when the change would switch every display off.
common-displays-none-on = That would leave no display on.
# Said where displays are set when the desktop gave no answer within a few seconds, or went away.
common-displays-stopped-answering = The desktop stopped answering.
# Said where the lock screen PIN is set when what was typed is not the right length. { $n } is how many digits a PIN has.
common-pin-digits = A PIN is { $n } digits.
# Said where the lock screen PIN is set when it could not be turned into the secret form it is kept in.
common-pin-not-hashed = The PIN couldn't be saved.
# Said where a theme is chosen when no theme has the name asked for.
common-themes-no-such = There's no theme of that name.
# Said where a theme is saved when it could not be kept. { $error } is the system's reason.
common-themes-not-kept = The theme couldn't be kept: { $error }.
# Said where a theme file is added, when its name doesn't end in .css. { $name } is the file's name.
common-theme-add-not-css = { $name } isn't a theme. A theme is one file whose name ends in .css.
# Said where a theme file is added, when it can't be read. { $name } is the file's name.
common-theme-add-unreadable = Trindle couldn't read { $name }.
# Said where a theme file is added, when it is over 64 KB. { $name } is the file's name.
common-theme-add-too-big = { $name } is too big to be a theme. A theme is at most 64 KB.
# Said where a theme file is added, when it holds more than plain text. { $name } is the file's name.
common-theme-add-not-a-sheet = { $name } isn't a style sheet. It contains more than plain text.
# Said where a theme file is added, when reading it took too long. { $name } is the file's name.
common-theme-add-too-slow = { $name } took too long to read.

# Under Trindle's own look on Settings, Appearance: a few words about it.
common-theme-trindle-about = Trindle's own look: a deep blue night, softly edged.
# Under the Bold theme on Settings, Appearance.
common-theme-bold-about = Black and white with clear edges, bigger words and bigger things to press. For low vision.
# Under the Midnight theme on Settings, Appearance.
common-theme-midnight-about = Dark and flat, in steps of grey: black and grey, no edges, small corners.
# Under the Paper theme on Settings, Appearance.
common-theme-paper-about = Warm white paper on a cream desk, for bright rooms. A light look.
# Under the Snug theme on Settings, Appearance.
common-theme-snug-about = Smaller and closer together, square and cool grey. More fits on a small screen.
## Drives (src/common/volumes.rs)

# Said where a drive is unlocked when the passphrase did not open it.
common-volumes-wrong-passphrase = That passphrase didn't open it.
# Said where a drive is opened when it is open already.
common-volumes-already-open = It's already open.
# Said where a drive is opened when it is locked and no passphrase has been given.
common-volumes-needs-passphrase = A passphrase is needed to open it.
# Said where a drive is opened when the small window that asks for its passphrase could not be shown.
common-volumes-window-failed = The passphrase window wouldn't open.
# What the file kept by the list of programs a box may use is called, in the words above.
common-unreadable-name-box-permissions = App permissions
# What the file that notes which questions an app has already put to the person is called, in the words above.
common-unreadable-name-box-asked = Questions already asked
# What the file that notes which apps came with the image is called, in the words above.
common-unreadable-name-box-offered = Apps offered at first login
# What the lists of a fenced program (its folders, hosts, settings and programs) are called, in the words above.
common-unreadable-name-fence = Fence lists
# What the list of icons the desktop found missing is called, in the words above.
common-unreadable-name-missing-icons = Missing icons list
# What the screen arrangement an older version kept in display.sh is called, in the words above.
common-unreadable-name-old-display = Screen arrangement

## Hardware with no driver (src/common/no_driver.rs)

# Said where the person works when a known chip has no driver yet. The kernel is a version number.
common-no-driver-known-driver = { $chip } doesn't have a driver here yet. Linux { $kernel } is the first version to include one.
# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = { $chip } is missing its firmware, which comes in { $package } { $version }.
# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = There's no driver for this { $class } chip ({ $id }).
# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = Missing { $class } firmware: { $file }.
# Added after a Wi-Fi line.
common-no-driver-stick = A USB Wi-Fi stick works for now.
# Tip of the small round i button that opens Help.
common-no-driver-more = More about this
common-no-driver-class-wifi = Wi-Fi
common-no-driver-class-bluetooth = Bluetooth
common-no-driver-class-sound = sound
common-no-driver-class-graphics = graphics
# Said of a device with no driver that nothing is known about. The name is its name, or its two hex numbers.
common-no-driver-device-none = { $name } has no driver.

# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = More
common-more-spoken = More about this, opens Help

## Pocketkey's "ask for the password again after" (src/common/pocketkey.rs)

# First choice of "Ask for the password again after": the phone stays good for as long as it likes.
common-pocketkey-never = Never
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } minutes.
common-pocketkey-minutes =
    { $n ->
        [one] 1 minute
       *[other] { $n } minutes
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } hours.
common-pocketkey-hours =
    { $n ->
        [one] 1 hour
       *[other] { $n } hours
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } days.
common-pocketkey-days =
    { $n ->
        [one] 1 day
       *[other] { $n } days
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } weeks.
common-pocketkey-weeks =
    { $n ->
        [one] 1 week
       *[other] { $n } weeks
    }
# On the lock and login screens, when the time chosen has passed and the phone is not accepted: { $time } is such as "1 hour".
common-pocketkey-due = It has been more than { $time }, so you need to type your password again.
# Said under the greyed Earlier versions card.
common-snap-no-snapper = Earlier versions need snapper, which is not installed on this computer.
# Said under the greyed Earlier versions card.
common-snap-from-snapshot = Not while the computer is started from a snapshot.
# Said under the greyed Earlier versions card.
common-snap-part-of-system = The home folder is part of the system on this computer, not a part of the disk of its own, so earlier versions cannot be kept apart from the system.
# Said under the greyed Earlier versions card.
common-snap-network = The home folder is kept on another computer, so earlier versions cannot be kept here.
# Said under the greyed Earlier versions card: { $fs } is the kind of file system, such as ext4.
common-snap-not-btrfs = Earlier versions need the home folder on Btrfs, and here it is on { $fs }.
# Said under the greyed Earlier versions card.
common-snap-not-whole = Your home folder is inside another part of the disk, not on a part of its own, so Trindle can't keep earlier versions of it.
# Said under the greyed Earlier versions card.
common-snap-read-only = The home folder cannot be written to at the moment, so nothing can be kept.
# Said under the greyed Earlier versions card.
common-snap-locked-files = A home folder here is kept as one locked file, and a copy of it each day would fill the disk.
# Said under the greyed Earlier versions card.
common-snap-elsewhere = Your home folder is not inside /home, where the copies are taken.
# Said under the greyed Earlier versions card.
common-snap-apart = Your home folder is locked or kept apart from the others, so the copies can't include it.
# Said under the greyed Earlier versions card.
common-snap-own-part = Your home folder is a part of the disk of its own, which the copies of /home do not go into.
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written.
common-firewall-not-protocol-port = "{ $text }" isn't written as protocol:port, such as tcp:24810.
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written before the colon.
common-firewall-bad-protocol = "{ $text }" isn't tcp or udp. Write it in small letters.
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written, { $lowest } the lowest port allowed.
common-firewall-bad-port = "{ $text }" isn't a port from { $lowest } to 65535.
# Said when a port you open by hand isn't written as protocol:port.
common-firewall-own-form = Write it as protocol:port, such as tcp:37666.
# Said when a port you open by hand starts with something other than tcp or udp.
common-firewall-own-protocol = Use tcp or udp.
# Said when a port you open by hand isn't a number from 1 to 65535.
common-firewall-own-port = That isn't a port from 1 to 65535.
# Joins the last port to the others in a list of ports. { $first } is the others, comma separated.
common-firewall-and = { $first } and { $last }
# Heads a list of ports. { $count } is how many, { $list } is the ports, such as TCP 24810 and UDP 24811.
common-firewall-ports =
    { $count ->
        [one] Port { $list }
       *[other] Ports { $list }
    }
# Why a plugin's ports are refused. { $max } is the most it may name.
common-firewall-too-many = A plugin can name at most { $max } ports.
# Why a plugin's ports are refused. { $port } is a port such as TCP 24810.
common-firewall-twice = { $port } is named twice.
# Why a plugin's ports are refused. { $port } is a port such as TCP 80, { $lowest } the lowest allowed.
common-firewall-too-low = { $port } is below { $lowest }. Those ports are kept for the system.
# Why a plugin's port is refused. { $port } is the number.
common-firewall-reserved = Port { $port } is used by Trindle or a common service, so a plugin can't ask for it.
# Why a plugin's port can't be opened. { $port } is such as TCP 24810, { $plugin } is the other plugin's name.
common-firewall-taken = { $port } is already used by { $plugin }, so it can't be opened for this plugin too.
# Why a plugin cannot open another port: { $max } is how many may be open.
common-firewall-full = The most ports that may be open for plugins, { $max }, are open already.
# Told by a notice when the ring has been started again.
common-ring-held-back = The ring stopped several times in a row, so Trindle paused it for a while. It's running again now.
# Told by a notice when the ring has been started again.
common-ring-not-started = The ring could not be started for a while. It is running again now.
# Told by a notice when the ring has been started again.
common-ring-not-ended = The ring stopped answering and could not be ended for a while. It is running again now.
# Told by a notice.
common-ring-not-done = The computer was asked to suspend, restart or shut down and did not do it.
# Told by a notice; the ring is what the round menu is called.
common-ring-by-name = The desktop could not start this ring itself, so it cannot open over a full-screen window and its notifications are not kept on top. Logging out and in again may put it right.
# A group of apps in the ring.
common-category-internet = Internet
# A group of apps in the ring (as above).
common-category-office = Office
# A group of apps in the ring (as above).
common-category-graphics = Graphics
# A group of apps in the ring (as above).
common-category-media = Sound and video
# A group of apps in the ring (as above).
common-category-games = Games
# A group of apps in the ring (as above).
common-category-development = Development
# A group of apps in the ring (as above).
common-category-learning = Learning
# A group of apps in the ring (as above).
common-category-utilities = Utilities
# A group of apps in the ring (as above).
common-category-system = System
# Why an app cannot be started: its launcher names nothing to run.
common-app-no-command = It has no command to run.
# Why a launcher was left alone: { $why } is the reason.
common-launcher-unchanged = Nothing was changed, because { $why }.
# The ring's notice when something is plugged in.
common-drive-arrived-usb = USB drive connected
# The ring's notice when something is plugged in (as above).
common-drive-arrived-card = Memory card connected
# The ring's notice when something is plugged in (as above).
common-drive-arrived-disc = Disc inserted
# The ring's notice when something is plugged in (as above).
common-drive-arrived-other = Drive connected
# Said in the password box after what the password is for.
common-drive-every-time = You'll be asked each time a drive is changed. If you close this box, nothing changes.
# Said in the password box: { $rest } is what is being done, such as "format this drive".
common-drive-password-needed-to = An administrator's password is needed to { $rest }
# Said in the password box when the action is not named.
common-drive-password-needed = An administrator's password is needed to change this drive
# Why a folder cannot be shared.
common-share-not-yours = Only folders that belong to you can be shared.
# Why a share cannot have the name asked for.
common-share-name-is-login = That name is someone's login on this computer, so a share cannot have it. Choose another.
# Why a share cannot have the name asked for.
common-share-bad-name = A share's name cannot hold any of % < > * ? | / \ + = ; : " or a comma.
# Why a folder cannot be shared.
common-share-no-guests = This computer's Samba settings do not allow shares without a password.
# Why a folder cannot be shared.
common-share-most = This computer already shares as many folders as it allows.
# Why a folder cannot be shared.
common-share-gone = That folder is no longer there.
# Why a folder cannot be shared, when the program said nothing.
common-share-no-reason = Samba did not say why.
# Why a folder cannot be shared: { $said } is what the sharing program said.
common-share-said = Samba said: { $said }
# Why a screenshot could not be taken.
common-shot-no-desktop = There is no desktop to photograph.
# Why a screenshot could not be taken.
common-shot-no-display = There is no display to photograph.
# Why a screenshot could not be taken.
common-shot-no-display-there = There's no screen in that part of the desktop.
# Why a screenshot could not be taken.
common-shot-cannot = This desktop cannot be photographed.
# Why a screenshot could not be taken.
common-shot-too-big = The screenshot is too big.
# Why a screenshot could not be taken.
common-shot-no-size = A screen returned an empty picture, so Trindle couldn't take the screenshot.
# Why a screenshot could not be taken.
common-shot-unreadable = The desktop offered no picture this can read.
# Why a screenshot could not be taken.
common-shot-no-memory = There was no memory for the picture.
# Why a screenshot could not be taken.
common-shot-stopped = The desktop stopped photographing the screen.
# Why a screenshot could not be taken.
common-shot-failed = The desktop could not photograph the screen.
# Said where the person is working when no screen reader is installed.
common-reader-none-yet = No screen reader is installed.
# Said wherever hiding a drive is offered or shown.
common-drive-not-a-lock = Hiding does not lock or protect it. A twin of it, one that says exactly the same of itself, is hidden with it.
# Said when typed words are not a change to a key.
common-remap-not-a-key = Not a key Trindle knows. Try Escape, Ctrl+C or Left Click.
# Said when typed words are not a change to a key.
common-remap-too-long = That is more keys than one change can hold.
# Said when typed words are not a change to a key or a button.
common-no-equals = A change needs what is pressed, an equals sign, and what it becomes.
# Said when typed words are not a change to a key.
common-remap-tapped-held = Say what it is tapped and what it is held, such as Escape tapped, Ctrl held.
# Said when typed words are not a change to a controller button.
common-pads-not-a-button = Not a button Trindle knows. Try South, Left Trigger or Escape.
# Said when typed words are not a change to a controller.
common-pads-not-a-zone = A dead zone is from 0% to 50%.
# Said of a line in the controllers file that is not understood.
common-pads-not-a-heading = Trindle doesn't recognise this controller heading.
# Said of a line in the controllers file that is not understood.
common-pads-no-heading = It is under no controller's heading.
# What a theme's tile says when its file holds more than a theme may.
common-theme-too-big = This file is too big to be a theme, so it is not used. A theme is at most 64 KB.
# What a theme's tile says when it is not an ordinary file.
common-theme-not-a-sheet = This is not an ordinary file of words, so it cannot be a theme and is not used.
# What is said of a theme that was chosen and whose file has since gone.
common-theme-gone = Its file is no longer there.
# What is said of a theme on a disk that does not answer.
common-theme-too-slow = Its file took too long to read, so Trindle isn't using it.
# What is said when the folders of themes are on a disk that does not answer.
common-theme-list-slow = The folders of themes took too long to read, so only Trindle's own look is listed.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-big = Your own style sheet, style.css, is over 256 KB, so it is not used.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-not-a-sheet = Your own style sheet, style.css, is not an ordinary file of words, so it is not used.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-slow = Your own style sheet, style.css, took too long to read, so it is not used.
# What is said when colours in the person's own style sheet fall short of what can be read.
common-theme-own-hard-to-read = Some colours are hard to read.
# What is said under the tiles when a folder holds more themes than are shown.
common-theme-not-all = A folder of themes may hold more than are listed: only the first forty themes in a folder are shown, and only its first 400 files are looked at.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-no-target = Your own style sheet, style.css, is a link to a file that is not there, so it is not used.
# The name of a file set aside when damaged, said in a sentence: Clipboard history.
common-damaged-clipboard = Clipboard history
# The name of a file set aside when damaged, said in a sentence: Last-used apps.
common-damaged-last-used = Last-used apps
# What kind of virtual machine this is, said in a sentence.
common-vm-qemu = a QEMU virtual machine (GNOME Boxes, virt-manager and the like)
# What kind of virtual machine this is, said in a sentence.
common-vm-apple = a Mac's virtual machine
# What kind of virtual machine this is, said in a sentence: { $name } is the name systemd gives it.
common-vm-other = a virtual machine ({ $name })
# What a program offered in Software sources is for, in a line.
common-source-claude = Anthropic's Claude app: chat, Cowork and Claude Code
# What a program offered in Software sources is for, in a line.
common-source-tailscale = A private network between your own devices
# What a program offered in Software sources is for, in a line.
common-source-syncthing = Keeps folders the same on your computers, with no cloud. Debian's own version is older.
# What a program offered in Software sources is for, in a line.
common-source-jellyfin = Your own film, music and photo server, for any screen in the house
# What a program offered in Software sources is for, in a line.
common-source-nodejs = The current long-term support version of Node.js, for developers. Debian's own is version 20.
# What a program offered in Software sources is for, in a line.
common-source-github-cli = GitHub from the Terminal, for developers. Debian's own version is older.
# What a program offered in Software sources is for, in a line.
common-source-docker = Runs programs in containers; for developers
# Said in the box that asks for a drive's passphrase: { $name } is the drive.
common-volumes-passphrase-for = Passphrase for { $name }
# The title of the box that asks for a drive's passphrase.
common-volumes-unlock-title = Unlock drive

# What a snapshot is, in the Earlier versions list: made just before software was changed.
common-snap-before-change = Before a software change
# What a snapshot is, in the Earlier versions list: made just after software was changed.
common-snap-after-change = After a software change
# What a snapshot is, in the Earlier versions list: made when the computer started.
common-snap-boot = When the computer started
# What a snapshot is, in the Earlier versions list: one taken on a schedule.
common-snap-regular = A regular copy
# What a snapshot is, in the Earlier versions list, when it has no note.
common-snap-plain = A snapshot
# Said when a font file can't be added. { $name } is the file's name, { $error } the system's reason.
common-pack-not-copied = Trindle couldn't copy { $name }: { $error }
# Said when a file given as a font isn't one. { $name } is the file's name.
common-pack-not-fonts = { $name } isn't a font or an archive of fonts.
# Said when a drive can't be hidden because it has no name of its own to remember it by.
common-hidden-no-name = Trindle can't remember this drive, because it has no name of its own.
# Said when the list of hidden drives can't be read. { $error } is the system's reason.
common-hidden-left = The list of hidden drives was left as it was: { $error }
# Said when a drive can't be released because your login session can't be found.
common-leave-no-session = Trindle couldn't find your login session, so nothing was changed.
# Said under the sound choices when PipeWire gives no answer and says nothing about why.
common-audio-no-answer = PipeWire didn't answer.
# Said when the screen reader can't be turned on or off. { $error } is the first problem, { $undo } the second.
common-screen-reader-undo-failed = { $error } Trindle also couldn't undo the change: { $undo }
# How long copies are kept, in a sentence such as "kept for a week".
common-keep-week = a week
# How long copies are kept, in a sentence such as "kept for two weeks".
common-keep-two-weeks = two weeks
# How long copies are kept, in a sentence such as "kept for a month".
common-keep-month = a month
# How long copies are kept, in a sentence such as "kept for 3 days". { $n } is the number of days.
common-keep-days =
    { $n ->
        [one] 1 day
       *[other] { $n } days
    }

# Said when an app can't be started. { $program } is the command, { $error } the system's reason.
common-app-not-started = Trindle couldn't start { $program }: { $error }

# Said when a settings file is too big to read.
common-keyfile-too-big = That file is too big to read.
# Said when a file Trindle was about to write at login isn't yours.
common-login-file-not-yours = The file there isn't yours.
# What the security updates switch says on a hover while it is on.
common-auto-updates-on = Auto updates on
# What the security updates switch says on a hover while it is off.
common-auto-updates-off = Auto updates off
# Said when polkit's list of the passwords it is keeping for this login couldn't be read, so Trindle can't make sure a password given for a drive is forgotten.
common-leave-list-unread = Couldn't read which passwords are still kept.
