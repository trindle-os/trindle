## The installer (src/installer): puts Trindle on a disk, and the welcome shown when it runs from a stick

## Window and steps along the top (src/installer/mod.rs)

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Install { $name }
# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Welcome
# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Disk
# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Privacy
# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Password
# Name of the fifth stop on that line, the page about the person.
installer-stop-you = You
# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Ready
# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Installing
# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Done
# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Getting ready
# Red line at the foot when the installer's helper could not be started.
installer-no-start = The installer could not start: { $why }
# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = The plan could not be read.
# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = The plan could not be read: { $why }
# Red line at the foot when the helper stopped without saying why.
installer-stopped = The installation stopped. Nothing more has been written.

## Foot of every page

# Name of the round button at the foot that goes back a page.
installer-back = Back
# Name of the main button at the foot that goes to the next page.
installer-next = Next
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Install
# Name of that button after one press, when a second press goes ahead.
installer-sure = Sure?
# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Restart now
# Beside the keyboard test box on the first page.
installer-try-keyboard = Try the keyboard
# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Type here to try the keyboard
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = Click again to install
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = Click again to erase and install
# Note beside Next on the first page while no language is chosen.
installer-missing-language = Choose a language
# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Choose a keyboard
# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Choose a time
# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Choose where Trindle goes
# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = Type the disk password twice, the same both times
# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = A sign-in name is needed
# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Give this computer a name
# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = Type your password twice, the same both times

## First page: language, keyboard and time

# Title of the first page.
installer-welcome-title = Welcome to { $name }
# Sentence under the title of the first page.
installer-welcome-help = Choose your language; the keyboard and the time follow it, and you can change either.
# Head of the card for choosing the language, and the name of its search box.
installer-language = Language
# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Keyboard
# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Time
# Under a card's head while nothing has been chosen in its list.
installer-select = Select

## Disk page

# Title of the disk page.
installer-disks-title = Where { $name } goes
# Sentence under the title of the disk page.
installer-disks-help = Everything on the disk you choose is replaced, unless you make room beside what is there.
# Shown on the disk page when no disk is big enough.
installer-disks-none = No disk big enough was found. Trindle needs 20 GB.
# Name and words of the choice to erase a whole disk.
installer-disks-whole = Use the whole disk, replacing everything on it
# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Keep { $what } and share the disk
# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } left for what is there now, { $mine } for Trindle
# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }
# Name of a partition with nothing on it.
installer-empty-space = Empty space
# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } and { $last }
# Under a disk's name: what is on it.
installer-disk-has = Has { $list } on it
# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Has { $list } on it, encrypted
# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Has an encrypted system on it
# Under a disk's name: there is nothing on it.
installer-disk-empty = Empty
# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Has files on it, but no system Trindle recognises
# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = A swap file of { $gb } GB
# Name of the switch that makes a swap file.
installer-swap-switch = Make a swap file
# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot is on, so this computer can't hibernate, but the swap file still helps when memory runs short. It is only made if the disk has plenty of room to spare.
# Under the swap file's head when the computer can hibernate.
installer-swap-note = Room to hibernate, keeping everything open with the power off, and somewhere to turn when memory runs short. Without it the computer cannot hibernate. It is only made if the disk has plenty of room to spare.

## Privacy page

# Title of the page about encrypting the disk.
installer-privacy-title = Encrypting the disk
# Sentence under the title of that page.
installer-privacy-help = An encrypted disk cannot be read by anyone who takes it out of this computer.
# Head of the card with the three choices.
installer-privacy-locked = How the disk is locked
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = Do not encrypt the disk
# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Encrypt it, and ask for the password when the computer starts
# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Encrypt it, and let this computer's TPM chip unlock it
# Head of the card about the TPM chip.
installer-privacy-about = About the chip
# In the card about the chip, on a computer that has one.
installer-privacy-chip = This computer has a TPM chip. The first start after installing asks for the password once, while the chip learns this computer; after that it unlocks the disk itself and nothing is typed. Keep the password: it is the way in if the chip is replaced or the disk is moved to another computer.
# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = This computer has no TPM chip Trindle can use, so the password will be asked for every time it starts. On a computer that has one, the chip can unlock the disk by itself.

## Disk password page

# Title of the disk password page.
installer-disk-title = The disk password
# Sentence under the title of the disk password page.
installer-disk-help = It is typed before anything else when the computer starts.
# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Disk password
# Name of the round button that makes up a passphrase.
installer-disk-suggest = Suggest a passphrase
# Label and name of the second box of a password, where it is typed again.
installer-same-again = The same again
# Red line under the password boxes when the two do not match.
installer-mismatch = The two passwords do not match.
# Head of the card that tells the person to write the password down.
installer-disk-write-down = Write it down
# Words in that card.
installer-disk-write-down-text = Write this password down on paper and keep it somewhere safe, away from the computer. Without it the disk cannot be opened, by you or by anyone else, ever. There is no way back in. Four ordinary words make a passphrase that is easy to type and hard to guess.

## Account page

# Title of the account page.
installer-you-title = About you
# Sentence under the title of the account page.
installer-you-help = This is the account you sign in with.
# Head of the card with the name boxes.
installer-account = Your account
# Label and name of the box for the person's name.
installer-your-name = Your name
# Label and name of the box for the sign-in name.
installer-user-name = Sign-in name
# Label and name of the box for the computer's name.
installer-host-name = Name of this computer
# Head of the card with the password boxes.
installer-your-password = Your password
# Label and name of the first password box.
installer-password = Password
# Label of the strength meter and the name a screen reader gives it.
installer-strength = How hard it is to guess
# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = A sign-in name is needed.
# Red line under the account boxes: a name is too long.
installer-name-too-long = That name is too long.
# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Start with a small letter.
# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Small letters, digits, - and _ only.
# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = That name is already used by the system.
# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Give this computer a name.
# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Letters, digits and - only.
# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = A computer would guess this straight away.
# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] A computer would guess this in about { $n } minute.
       *[other] A computer would guess this in about { $n } minutes.
    }
# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] A computer would guess this in about { $n } hour.
       *[other] A computer would guess this in about { $n } hours.
    }
# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] A computer would guess this in about { $n } day.
       *[other] A computer would guess this in about { $n } days.
    }
# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] A computer would need about { $n } year to guess this.
       *[other] A computer would need about { $n } years to guess this.
    }
# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = A computer would need longer than recorded history to guess this.

## Ready page

# Title of the Ready page.
installer-ready-title = Ready to install
# Head of the card that lists what will happen.
installer-ready-nothing-changed = Nothing has been changed yet
# Head of the card about security updates.
installer-ready-security = Security updates
# In the card about security updates.
installer-ready-security-text = { $name } installs Debian's security fixes by itself every day. This cannot be turned off.
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = Everything on { $disk } will be replaced.
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Everything on { $part } is kept, but back it up first: making it smaller can lose files if the power fails.
# Sentence under the title when no disk has been chosen.
installer-ready-none = No disk has been chosen yet.
# Name of a row in the list: the whole disk will be used.
installer-ready-whole = The whole of { $disk }
# Words of that row.
installer-ready-whole-text = Everything on it is replaced by { $name }.
# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Beside { $part }
# Words of that row.
installer-ready-beside-text = { $part } is made smaller, down to { $size }, and { $name } goes in the space freed.
# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = No disk
# Words of that row.
installer-ready-no-disk-text = No disk chosen.
# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Not encrypted
# Words of that row.
installer-ready-plain-text = Anyone who takes the disk out can read it.
# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Encrypted
# Words of that row when a password is typed at each start.
installer-ready-locked-password = You type its password each time the computer starts.
# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = The password once, then this computer's TPM chip unlocks it.
# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Keyboard { $keyboard }, time { $zone }.

## Install page

# Title of the page shown while the install runs.
installer-installing-title = Installing { $name }
# Head of the card that holds the short tour.
installer-tour = A short tour

## Done page

# Title of the last page.
installer-done-title = { $name } is installed
# Sentence under the title of the last page.
installer-done-help = Take the USB stick out, then restart.
# Head of the card that repeats the warning about the disk password.
installer-done-keep = Keep the disk password safe
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = The disk is encrypted. The first start asks for the disk password once, while the chip learns this computer; after that it opens on its own. Write that password down on paper and keep it somewhere safe away from this computer. Without it the disk cannot be opened, by you or by anyone else, ever.
# In that card when the password is typed at each start.
installer-keep-password = The disk is encrypted. Write the disk password down on paper now, if you have not already, and keep it somewhere safe away from this computer. Without it the disk cannot be opened, by you or by anyone else, ever.

## Slides shown while the disk is written (src/installer/brand.rs)

# Title of the first slide.
installer-slide-ring = Everything in one circle
# Words of the first slide.
installer-slide-ring-text = Move the pointer to the edge of the screen and double-click, and the ring opens: your programs around the outside, the time and the switches in the middle.
# Title of the second slide.
installer-slide-snapshots = Put back what an update broke
# Words of the second slide.
installer-slide-snapshots-text = A snapshot of the system is taken before and after every software change. Settings puts the whole system back to any of them, and the boot menu can do it when the computer will not start.
# Title of the third slide.
installer-slide-apps = Apps that stay out of the way
# Words of the third slide.
installer-slide-apps-text = Flathub apps, Debian's own, AppImages and software sources in one place, with backups you can carry to another computer.
# Title of the fourth slide.
installer-slide-yours = Yours, quickly
# Words of the fourth slide.
installer-slide-yours-text = Light or dark, your own sounds, and a clipboard that remembers what you copied.
# Title of the fifth slide.
installer-slide-small = Small and quiet
# Words of the fifth slide.
installer-slide-small-text = The desktop is light on memory and starts in a few seconds, so an older laptop feels new again.

## The tour with pictures (src/installer/tour.rs)

# Title of the tour slide about the ring.
installer-tour-ring = The ring is the whole desktop
# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Put the pointer in this corner and the ring opens
# Note on the ring picture, at the time.
installer-tour-ring-time = The time and the date, in the middle
# Note on the ring picture, at the programs.
installer-tour-ring-programs = Your programs sit around the outside
# Note on the ring picture, at the switches.
installer-tour-ring-switches = Sound, screen, all your apps, keep awake, and turning off
# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = What is open, the clipboard, and a button that clears the screen
# Title of the tour slide about Settings.
installer-tour-settings = Settings, in plain words
# Note on the Settings picture, at the search box.
installer-tour-settings-search = Type here to find a setting without hunting for it
# Note on the Settings picture, at Network.
installer-tour-settings-network = Network is where you join a Wi-Fi network
# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Light or dark, whenever you like
# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Twenty colours, or type your own
# Title of the tour slide about Apps.
installer-tour-apps = Apps, for everything else
# Note on the Apps picture, at the search box.
installer-tour-apps-search = Search for a program by name or by what it does
# Note on the Apps picture, at the list.
installer-tour-apps-many = Thousands of programs, free, from Flathub and Debian
# Note on the Apps picture, at updates.
installer-tour-apps-updates = Updates for everything, including Trindle itself
# Title of the tour slide about voice control.
installer-tour-voice = You can talk to it instead
# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Voice control lives here: Settings, then Ring
# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Press Add voice once. It is about 150 MB and takes a few minutes
# Note on the voice picture, about what to say.
installer-tour-voice-say = Then tap the middle of the ring and say an app's name, “louder”, or “Wi-Fi settings”
# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = What you say is understood on this computer and goes nowhere else
# Title of the tour slide about snapshots.
installer-tour-snapshots = A way back, before anything changes
# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = The computer takes a snapshot before every update
# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = If something breaks, go back to how it was, from the boot menu

## The welcome shown when running from a stick (src/installer/welcome.rs)

# Name of the picture of the ring, and the head of its card.
installer-live-caption = The ring, where everything lives
# Sentence under the title of the welcome from a stick.
installer-live-help = You are running from the stick, so nothing on this computer has been changed.
# Head of the card with the three first things to do.
installer-live-begin = To begin
# Name of the first thing to do.
installer-live-open = Open the ring
# Line under it.
installer-live-open-line = Double-click at the edge of the screen, or press Super.
# Name of the second thing to do.
installer-live-speak = Speak to it
# Line under it when voice control is there.
installer-live-speak-line = Tap the middle of the ring and say what you want.
# Line under it when voice control has to be added.
installer-live-speak-add = Add voice in Settings, Ring, then tap the middle and speak.
# Name of the third thing to do.
installer-live-tab = All in the ring
# Line under it.
installer-live-tab-line = What is open, your notifications and the clipboard are all there.
# The choice of the dark look.
installer-live-dark = Dark
# The choice of the light look.
installer-live-light = Light
# Name the screen reader gives to the choice between dark and light.
installer-live-look = Light or dark, for the whole desktop
# Name of the round button that closes this window and opens the ring.
installer-live-around = Look around first
# Name of the main button that opens the installer.
installer-live-install = Install { $name }

## Words the root half sends to the window (src/installer/steps.rs)

# Foot of the install page while room is made on the disk.
installer-step-room = Making room
# Foot of the install page while the disk is prepared.
installer-step-prepare = Preparing the disk
# Foot of the install page while a place is made beside another system.
installer-step-place = Making a place for Trindle
# Foot of the install page while the disk is looked at.
installer-step-check = Checking what is already on the disk
# Foot of the install page while another system is made smaller.
installer-step-smaller = Making the other system smaller
# Foot of the install page while the disk is encrypted.
installer-step-lock = Locking the disk
# Foot of the install page while the TPM chip is set up.
installer-step-chip = Setting up the computer's chip
# Foot of the install page while the system is copied.
installer-step-copy = Copying Trindle onto the disk
# Foot of the install page while settings are written.
installer-step-settings = Writing the settings
# Foot of the install page while the person's account is made.
installer-step-account = Making your account
# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Taking the live parts out
# Foot of the install page while the start-up files are made.
installer-step-startup = Making the start-up files
# Foot of the install page while the boot menu is set up.
installer-step-boot = Setting up the boot menu
# Foot of the install page while snapshots are set up.
installer-step-snapshots = Setting up snapshots
# Foot of the install page at the very end.
installer-step-finishing = Finishing
# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } could not start: { $why }
# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } did not finish. { $why }
# Red line at the foot: a file could not be written.
installer-write-failed = { $path } could not be written: { $why }
# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } is not a disk.
# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } is not a disk on this computer.
# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } is the stick Trindle is running from.
# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } is in use. Close what is using it, then try again.
# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = No disk was chosen.
# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = That sign-in name is not one Trindle can use.
# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = That computer name is not one Trindle can use.
# Red line at the foot: the language cannot be used.
installer-bad-language = The language is not one Trindle can use.
# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = The keyboard is not one Trindle can use.
# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = The time zone is not one Trindle can use.
# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = That name holds something Trindle cannot write down.
# Red line at the foot: a password has a line break in it.
installer-bad-password = A password cannot hold a line break.
# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = The installer's second half must run as root.
installer-already-running = Another installer is already running on this computer.
# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = This disk has no boot partition to share, so Trindle cannot go beside what is already here.
# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = The new part of the disk could not be found.
# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = The size of { $part } could not be read.
# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle cannot make a { $fs } partition smaller.
# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle cannot make an unknown partition smaller.
# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = A password is needed to encrypt the disk.
# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = The system could not be copied: { $why }
# Red line at the foot: the copy of the system failed.
installer-copy-failed = The system could not be copied. { $why }
# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = No room for a swap file.
# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = The swap file's place on the disk could not be read.
# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = The new partitions could not be identified.
# Red line at the foot: the time zone could not be set.
installer-timezone-failed = The time zone could not be set: { $why }
# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = The boot menu could not be installed on this computer.
# Said beside how far the install has got, once it has started.
installer-started = Install started
# The button that ends an install under way.
installer-halt = Stop
# Beside Stop after its first press.
installer-halt-sure = Stop installing? The disk's old contents have already gone.
# While an install is being stopped.
installer-halting = Stopping
# When Stop could not let go of the disk.
installer-halt-stuck = The disk could not be let go. Restart the computer to try again.
# The copy is done and the person is still answering.
installer-step-waiting = Copied. Waiting for your answers.

# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle needs at least 1 GB of memory, and this computer has less.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = A disk can only be encrypted on a computer that starts by UEFI, and this one does not.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows on { $part } was hibernated or left in fast start-up, so making it smaller could lose its files. Start Windows, choose Restart, then try again.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle is installed, but snapshots could not be set up: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle is installed, but the boot menu could not be made to list snapshots: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Not everything was set up
