# Words of the shared parts that no one program owns: why an app cannot be uninstalled, what updating and renewing say, what the keyring says, the X11 rooms, and the "in use by" lines. Each program shows them where it needs them.

## Uninstalling (src/common/removal.rs)

# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = Can't uninstall: part of Trindle
# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = Can't uninstall: the computer needs it
# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Can't uninstall: part of Trindle needs it
# Red words when an app's saved data was not deleted because its id is not a plain name.
common-removal-data-name = Its data was not deleted: the app's name is not a plain name.
# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
common-removal-data-link = Its data was not deleted: its folder is a link, not a folder.
# Red words when an app's saved data could not be deleted.
common-removal-data-failed = Its data could not be deleted.
# Said when only an app's saved data is to go and the app is running.
common-removal-data-running = Its data was not deleted: the app is running. Close it first.
# Said when the app's data path is a file, not a folder.
common-removal-data-not-folder = Its data was not deleted: it is a file, not a folder.

## One update at a time (src/common/updating.rs)

# Name of the updates button, and its tip, while the ring's own update button is putting updates in.
common-updating-short-ring = Updating
# Name of an update button while Apps is putting updates in.
common-updating-short-apps = Updating in Apps
# Name of an update button while the daily security fixes for the apps' boxes are going in.
common-updating-short-boxes = Security fixes going in
# Name of an update button while Debian's own installer is at work.
common-updating-short-system = Debian is updating
# Name of an update button while something not named is installing updates.
common-updating-short-unknown = Updating elsewhere
# Headline of an updates page while the ring is installing updates.
common-updating-sentence-ring = Updates are being installed from the ring.
# Headline of an updates page while Apps is installing updates.
common-updating-sentence-apps = Updates are being installed from Apps.
# Headline of an updates page while the daily security fixes are going into the apps' boxes.
common-updating-sentence-boxes = Security fixes are going into the apps' boxes.
# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Debian's installer is at work, as it is each day for security updates.
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
common-updating-reason-all = Debian's and Trindle's updates change the whole computer, for everyone who uses it, so they need an administrator's password. Apps you installed yourself update without one.
# Said in the password box when only Debian's updates are put in.
common-updating-reason-debian = Debian's updates change the whole computer, for everyone who uses it, so they need an administrator's password.
# Said in the password box when only Trindle's own update is put in.
common-updating-reason-trindle = Trindle's update changes the whole computer, for everyone who uses it, so it needs an administrator's password.
# Said in the password box when new firmware is put in.
common-updating-reason-firmware = New firmware changes the computer itself, so it needs an administrator's password.
# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
common-updating-reason-refresh = Looking for the whole computer's updates needs an administrator's password when it is not asked from the computer's own screen.

## How an update ended (src/common/updating.rs)

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = because { $why }.
# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = It did not finish, and gave no reason.
# Red words beside an update that did not finish. { $why } is the reason the system gave.
common-updating-failed = It did not finish. { $why }
# Red words beside an update that was never started. { $why } is the end of a sentence such as "because no administrator's password was given."
common-updating-not-installed = Not installed, { $why }

## Why a step behind the administrator's password was refused (src/common/refused.rs)

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = no administrator's password was given
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = the administrator's password was not accepted
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = this login is not allowed to do that, whatever password is given
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = nothing was there to ask for the administrator's password
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = the administrator's password was not accepted, or this login is not allowed to do that
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = No administrator's password was given
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = The administrator's password was not accepted
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = This login is not allowed to do that, whatever password is given
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = Nothing was there to ask for the administrator's password
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = The administrator's password was not accepted, or this login is not allowed to do that
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = Not done: no administrator's password was given.
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = Not done: the administrator's password was not accepted.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = Not done: this login is not allowed to do that, whatever password is given. Log out and back in, then try again.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = Not done: nothing was there to ask for the administrator's password. Log out and back in, then try again.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = Not done: the administrator's password was not accepted, or this login is not allowed to do that. If nothing asked for it, log out and back in, then try again.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Not done: the step failed and did not say why.

## The keyring (src/common/keyring_said.rs)

# Red words when the keyring will not open without the person's password.
common-keyring-locked = The keyring is locked. Log out and in again, or unlock it, then try again.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = There is no keyring on this computer to keep it in.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = The keyring did not answer in time.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = The keyring never answered.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = The turn with the keyring could not be taken ({ $why }), so nothing was asked of it.
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] The keyring has not answered yet, so the { $what } is not removed so far. If the keyring removes it after all, this line will say so.
       *[other] The keyring has not answered yet, so the { $what } is not kept so far. If the keyring keeps it after all, this line will say so.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] The keyring answered late: the { $what } is removed.
       *[other] The keyring answered late: the { $what } is kept.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] The keyring never answered, so the { $what } is not removed.
       *[other] The keyring never answered, so the { $what } is not kept.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] The keyring answered late and did not remove the { $what }: { $why }
       *[other] The keyring answered late and did not keep the { $what }: { $why }
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
common-renewed-sentence = This update is already in use, except { $list }, which will be the new version after the next log out.
# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = This update is already in use, except { $list }, which will be the new version after the next log out at the latest.

## Older programs in boxes (src/common/x11_rooms.rs)

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland would not start
# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = Try again in a minute

## What is using the microphone, camera, screen, input or copied files (src/common/in_use.rs)

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each dot with what it means.
common-in-use-key = What the dots mean
# Under the dots in that key: where a dot shows up.
common-in-use-where = Dots show at the top middle of the screen and on window tabs.

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
common-in-use-input = Mouse and keyboard taken
# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
common-in-use-input-by = { $who } is using your keyboard and mouse
# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-files = The files you copied are being read
# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
common-in-use-files-by = { $who } is reading the files you copied

# The ring's who-is-using-what row while a plugin reaches devices on the home network.
common-in-use-home-network = Home network in use

# The same with the plugin's name.
common-in-use-home-network-by = Home network in use by { $who }

# The ring's who-is-using-what row while a plugin uses scanners and USB devices.
common-in-use-usb = Scanners and USB devices in use

# The same with the plugin's name.
common-in-use-usb-by = Scanners and USB devices in use by { $who }
# Line on pointing at the dots when the microphone is switched off and the program listening hears nothing.
common-in-use-microphone-off = Microphone switched off
# Line on pointing at the dots when the microphone is switched off but programs want it. { $who } is the programs' names.
common-in-use-microphone-off-by = Microphone switched off, wanted by { $who }
# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
common-in-use-microphone-off-note = Off for every program, by the ring's microphone button.

## A settings file that is damaged or cannot be read (src/common/damaged.rs). Help section: settings-unreadable

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = { $what } reset: file damaged, kept beside it.
# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = { $what } not saved: its file cannot be read, so it was left as it is.
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
common-no-driver-known-driver = { $chip } has no driver here yet. It first comes with Linux { $kernel }.
# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = { $chip } is missing its firmware. It is in { $package } { $version }.
# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = No driver for a { $class } chip ({ $id }).
# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = Missing { $class } firmware: { $file }.
# Added after a Wi-Fi line.
common-no-driver-stick = A USB Wi-Fi stick works meanwhile.
# Tip of the small round i button that opens Help.
common-no-driver-more = More about this
common-no-driver-class-wifi = Wi-Fi
common-no-driver-class-bluetooth = Bluetooth
common-no-driver-class-sound = sound
common-no-driver-class-graphics = graphics

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
common-pocketkey-due = Password needed again: it has been more than { $time }.
