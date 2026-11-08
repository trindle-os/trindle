# Words of Drives (trindle-drives), the window that shows the drives in or plugged into this computer and changes them: the page, the cards, the floating boxes of changes, and how every change ends. Sizes such as 1.5 GB, percentages and the names of file systems (exFAT, ext4, NTFS, FAT) are written by the program and passed in.

## The page (src/disks/mod.rs)

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Drives

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = About this page

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Every drive in or plugged into this computer, drawn as a bar of its parts. Each part is as wide as it is big and filled as far as it is used, so a full part is easy to spot.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Under the picture, each drive has a card. Open a part, mount or unmount it, or take a drive out safely or power it off. The pencil on a part floats its changes: its name, a folder of its own and whether it is ready at start-up, its size, checking and repairing it, formatting it or deleting it. The plus in free space makes a new part.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = The drive the system runs from is shown but never offered for any change that erases or moves things. Every change to any drive asks for an administrator's password, each time. The eye hides a drive or a part from Folders and the ring: it stays here, greyed, and is not locked.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Each part of each drive, filled as far as it is used.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Your drives

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = The drives cannot be read right now.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Try again

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Look again

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = No drives were found. Plug one in and it appears here.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = The drive service

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = The system bus

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } is not available. Install it with: sudo apt install { $package }

## The sentence at the top (src/disks/mod.rs)

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Looking at the drives…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } is failing. Copy anything you want to keep off it now.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } needs a look: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Drives and the space on them.
        [one] Drives and the space on them, and a drive plugged in.
       *[other] Drives and the space on them, and { $plugged } drives plugged in.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Nearly full: only { $free } free on this computer. Delete or move some files soon.
        [one] Nearly full: only { $free } free on this computer, and a drive plugged in. Delete or move some files soon.
       *[other] Nearly full: only { $free } free on this computer, and { $plugged } drives plugged in. Delete or move some files soon.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Getting full: { $free } free on this computer.
        [one] Getting full: { $free } free on this computer, and a drive plugged in.
       *[other] Getting full: { $free } free on this computer, and { $plugged } drives plugged in.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Plenty of room: { $free } free on this computer.
        [one] Plenty of room: { $free } free on this computer, and a drive plugged in.
       *[other] Plenty of room: { $free } free on this computer, and { $plugged } drives plugged in.
    }

## How full the places are (src/disks/mod.rs)

# Title of the card with the ring for the disk the system is on.
disks-ring-system = This computer

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Your files

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } still free, where the system is.
       *[files] { $free } still free, for your own files.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } full, { $free } free of { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = of { $whole }

## The drives together (src/disks/mod.rs)

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = No readings, the drives do not say how they are

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Healthy, as the drive itself says

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
       *[other] Healthy, as all { $n } drives say
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } needs a look, see its card
       *[other] { $n } need a look, see their cards
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 drive, { $total } in all. { $health }.
       *[other] { $n } drives, { $total } in all. { $health }.
    }

## The map of the drives (src/disks/mod.rs)

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } free

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Not mounted

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Free space

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = No disc or card in it

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Nothing on it yet

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB or removable

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, inside this computer

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB or removable, hidden

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, inside this computer, hidden

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } full, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

## Parts and drives (src/disks/mod.rs)

# Name given to a part that has no label and no device name.
disks-part-name = Part

# Name given to a drive that gives no maker or model.
disks-drive-name = Drive

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Swap

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Encrypted, unlocked

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Encrypted, locked

# Kind of a part that holds other parts.
disks-kind-container = Holds other parts

# Kind of a part that has no filesystem on it.
disks-kind-none = No filesystem

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, the part a computer starts from
        [legacy] { $kind }, the part an older computer starts from
       *[loader] { $kind }, a boot loader's own part
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = any computer

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = Linux only

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = old devices

## Health (src/disks/plan.rs and src/disks/mod.rs)

# Trouble a drive reports about its spare space.
disks-trouble-spare = running out of spare space

# Trouble a drive reports about its heat.
disks-trouble-temperature = too hot

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = wearing out

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = has made itself read-only

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = its memory backup has failed

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = part of it has made itself read-only

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = worn past what its maker expects

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } damaged spot
       *[other] { $n } damaged spots
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] used for { $hours } hour
       *[other] used for { $hours } hours
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Failing: copy anything you want to keep off it now

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Needs attention: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Healthy

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

## The card of a drive (src/disks/mod.rs)

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = No drive answering

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = The case answers, but not the drive inside it. Unplug it, wait ten seconds and plug it into a blue or USB-C port with its own cable; if it stays like this, the drive inside may have come loose or failed.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Remove safely

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Power it off

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
disks-erase-whole = Erase the whole drive

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Show this drive
       *[part] Show this part
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Hide this drive. { $lock }
       *[part] Hide this part. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Hidden from Folders and the ring. { $lock }

# Note on the card of the drive the system runs from.
disks-system-note = The system runs from it: nothing here can be erased.

# Note on the card of a drive that has no partition table.
disks-no-table = No partition table: the whole drive is free.

## The rows of parts (src/disks/mod.rs)

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = not mounted

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = mounts at start-up in { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = appears in { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = in { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Open

# Tip of the round button that mounts a part.
disks-mount = Mount

# Tip of the round button that unmounts a part.
disks-unmount = Unmount

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Change it

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Name

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Name (optional)

# Tip of the button that saves a part's new name.
disks-rename-tip = Rename it

# Note under the Name row of a part.
disks-name-note = What it is called here and on other computers.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Folder name

# Tip of the button that saves a part's folder.
disks-use-folder = Use this folder

# Label of the row where a part's folder is chosen.
disks-folder = Folder

# Note under the Folder row of a part.
disks-folder-note = Where its files appear. Leave it empty to let the computer choose.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Mount at start-up

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Ready whenever the computer starts. If it is not there, the computer starts anyway.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Size in GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = From { $least } to { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = From { $least } to { $most }. Mount it to see how small it can go.

# Tip of the button that resizes a part.
disks-resize-tip = Resize it

# Label of the row where a part is resized.
disks-size = Size

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
disks-format-erases = This will erase everything on { $words }. It cannot be undone.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning = Click again: { $name } on { $drive } goes from { $from } to { $to }

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Click again: { $name } on { $drive } goes from { $from } to { $to }. Back up its files first, as a power cut while it shrinks can lose them.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows on { $name } was hibernated or left in fast start-up, so making it smaller could lose its files. Start Windows, choose Restart, then try again.

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Make room for a new part

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Room in GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Make room

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
disks-room-warning = Click again: { $name } on { $drive } loses { $size }, after a snapshot

# Note under the row that makes room.
disks-room-note = Made smaller while in use, after a snapshot, leaving free space.

# Tip of the button that checks a part for faults.
disks-check-tip = Check it for faults

# Tip of the button that repairs a part.
disks-repair-tip = Repair it

# Label of the row where a part is checked and repaired.
disks-faults = Faults

# Note under the Faults row.
disks-faults-note = Looks for faults in how its files are kept, and puts them right. It is unmounted while this runs.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
disks-repair-warning = Click again to repair { $name }, on { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Kind

# Label of the row where a part is formatted.
disks-format = Format

# Tip of the button that formats a part.
disks-format-tip = Format it

# Note under the row that erases the whole drive.
disks-format-whole = Starts it afresh as one part, filling the drive.

# Note under the row that formats one part.
disks-format-part = Erases everything on this part only.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
disks-format-warning = Click again: everything on { $words } is erased

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, on { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-delete-name = Delete { $name }, { $size }, on { $drive }

# Label of the row where a part is deleted.
disks-delete-label = Delete this part

# Note under the row that deletes a part.
disks-delete-note = Its space becomes free, ready for a new part or for the one before it to grow into.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Make a new part here

# Label of the row where a new part is made.
disks-new-part = New part

# Note under the row where a new part is made.
disks-new-part-note = Made in this free space, and formatted ready to use.

# Tip of the button that makes a new part.
disks-make-tip = Make it

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = Click again to make a { $size } part on { $drive }

## The floating box of changes (src/disks/mod.rs)

# Title of the box for erasing a whole drive. { $name } is the drive.
disks-sheet-erase = Erase { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, on { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = A new part

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } free, on { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Close

## How a change goes (src/disks/mod.rs)

# Words on a row while a part is being mounted.
disks-mounting = Mounting

# Words on a row when a part has been mounted.
disks-mounted = Mounted

# Words on a row while a part is being unmounted.
disks-unmounting = Unmounting

# Words on a row when a part has been unmounted.
disks-unmounted = Unmounted

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Closing it

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Safe to remove

# Words on a drive's row while it is being powered off.
disks-powering-off = Powering off

# Words on a drive's row when it has been powered off.
disks-off-safe = Off: safe to unplug

# Words on a row while a part is being renamed.
disks-renaming = Renaming

# Words on a row when a part has been renamed.
disks-renamed = Renamed

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = The computer chooses its folder again

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = It appears in { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Mounts at start-up in { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Left alone at start-up

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Saving

# Words on a row while a part is being resized.
disks-resizing = Resizing

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Resized to { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = The files could not be found, so room cannot be made.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Taking a snapshot

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = The snapshot could not be found afterwards.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Nothing was resized.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Things changed while the snapshot was taken, so nothing was resized.

# Words on a row while room is being made.
disks-making-room = Making room

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } of room made

# Words on a row while a part is being repaired.
disks-repairing = Repairing

# Words on a row when a part has been repaired.
disks-repaired = Repaired

# Red words when some faults could not be put right.
disks-repair-failed = Some faults could not be put right. Copy off anything you want to keep.

# Words on a row while a part is being checked.
disks-checking = Checking

# Words on a row when the check found no faults.
disks-no-faults = No faults found

# Red words when the check found faults.
disks-faults-found = Faults found: press the spanner to repair them.

# Words on a row while a part or drive is being formatted.
disks-formatting = Formatting

# Words on a row when a part has been formatted.
disks-formatted = Formatted

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = erasing the whole drive

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = The new part could not be found.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = The drive service could not be reached.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = The new part's identifier could not be read.

# Words on a row while a part is being deleted.
disks-deleting = Deleting

# Words on a row when a part has been deleted.
disks-deleted = Deleted

# Words on a row while a new part is being made.
disks-making-it = Making it

# Words on a row when a new part has been made.
disks-made = Made

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = That could not be kept: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

## Decisions about names and sizes (src/disks/plan.rs)

# Red words under the folder name box when it is empty.
disks-name-empty = Give it a name first.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] That name is too long: { $max } letter at most.
       *[other] That name is too long: { $max } letters at most.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Use only letters, numbers, - and _ (no spaces).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Start it with a letter or a number.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } is already in use.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] That name is too long for this kind: { $max } letter at most.
       *[other] That name is too long for this kind: { $max } letters at most.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT names cannot hold * ? . , ; : / \ | + = < > [ ] or ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Mount it first: this kind can only be made larger while mounted.
       *[smaller] Mount it first: this kind can only be made smaller while mounted.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] This kind cannot be made larger.
       *[smaller] This kind cannot be made smaller.
    }

## What a change says and what became of it (src/disks/plan.rs)

# Red words when the password box was closed with no password. Said whole, as a sentence.
disks-no-password = No password was given, so nothing changed.

# Red words when the password box was closed with no password, after some steps of a change were already done.
disks-no-password-part-way = No password was given, so the work stopped part-way.

# Red words when another change is pressed while one is still being made.
disks-change-going = A change is being made.
disks-one-at-a-time = Another change is still being made. Try again when it has finished.

# Red words when it could not be made sure that the password would be asked for.
disks-not-sure = It could not be made sure that the password would be asked for, so nothing changed.

# Red words after how a change ended, when the password just given could not be taken back.
disks-standing = The password just given could not be taken back, so for a few minutes another change to a drive may not ask for one. It is being tried again.

# Words after how a change ended, when the password just given has been taken back after all.
disks-taken-back = The password given has now been taken back: the next change will ask for one.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = The drive service could not be reached, so nothing changed.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Drives was closed while this change was being made.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Drives stays open until this has finished, so that it can say how it ended.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, so nothing changed.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, so the work stopped part-way.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = The drive service could not be asked

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } The work stopped part-way.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Done: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Failed: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Not done: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Not known: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } was unmounted for this and is not mounted now.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } was unmounted for this and is still unmounted: mount it from its row.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } may have been unmounted: look at its row.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } was unmounted for this, and mounting it again needs a password of its own: mount it from its row.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } was unmounted for this and could not be mounted again ({ $why }): mount it from its row.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Formatted, but it was not given its folder { $dir } again. { $why } Choose the folder again from its row.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Something is still using it. Close any windows or programs showing its files, then try again.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = There is not enough room for what is on it at that size.

## Steps of a change, in the account of one that stopped part-way (src/disks/plan.rs)

# Step of a change. { $dev } is the part.
disks-step-unmount = unmounting { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = mounting { $dev } again

# Step of a change.
disks-step-eject = closing the drive to be removed

# Step of a change.
disks-step-power-off = powering the drive off

# Step of a change. { $dev } is the part.
disks-step-format = formatting { $dev }

# Step of a change.
disks-step-create = making the new part

# Step of a change. { $dev } is the part.
disks-step-delete = deleting { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = resizing the part { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = resizing what is on { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = repairing { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = checking { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = renaming { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = saving the folder and start-up choice of { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = the step { $method } on { $dev }

## Making room under a part that is in use (src/disks/room.rs)

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = This { $kind } part can only be made smaller when it is not in use, so room cannot be made from it here.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = This xfs part cannot be made smaller.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = This { $kind } part cannot be made smaller while it is in use.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = A snapshot can only be taken of the system's own disk, so room cannot be made here.

# Red words under the Make room row.
disks-room-not-in-table = It is not a part of a drive's table, so room cannot be made from it.

# Red words under the Make room row.
disks-room-no-online-shrink = The drive service cannot make Btrfs smaller while it is in use here, so room cannot be made.

# Red words under the Make room row.
disks-room-holds-parts = It holds other parts, so room cannot be made from it.

# Red words under the Make room row.
disks-room-not-gpt = Its drive does not have a GPT table, so room cannot be made from it.

# Red words under the Make room row.
disks-room-mount-first = Mount it first: Btrfs is made smaller while it is mounted.

# Red words under the Make room row.
disks-room-mounts-not-one = It is not mounted in exactly one place, so room cannot be made.

# Red words under the Make room row.
disks-room-read-only = It is mounted read-only, so room cannot be made.

# Red words under the Make room row.
disks-room-read-only-unknown = It could not be told whether it is read-only, so room cannot be made.

# Red words under the Make room row.
disks-room-many-drives = The files spread over more than one drive, so room cannot be made.

# Red words under the Make room row.
disks-room-drives-unknown = It could not be told how many drives the files spread over, so room cannot be made.

# Red words under the Make room row.
disks-room-swap-file = A swap file lies on it, so room cannot be made.

# Red words under the Make room row.
disks-room-swap-unknown = It could not be told whether a swap file lies on it, so room cannot be made.

# Red words under the Make room row.
disks-room-used-unknown = How much of it is in use could not be read, so room cannot be made.

# Red words under the Make room row.
disks-room-area-unknown = The drive service does not say how big the files' area is, so it could not be checked and room cannot be made.

# Red words under the Make room row.
disks-room-open-missing = The open copy of it could not be found, so room cannot be made.

# Red words under the Make room row.
disks-room-encryption-sizes = The sizes given for the encryption do not agree, so room cannot be made.

# Red words under the Make room row.
disks-room-sizes = The sizes given for it do not agree, so room cannot be made.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = It holds { $used } and needs more than that kept, so too little can be freed.

# Red words under the Make room row.
disks-room-sizes-unread = Its sizes could not be read.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Room from { $least } to { $most } can be made.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, so the work was stopped.

# What was found wrong while making room.
disks-room-check-unmounted = The files are no longer mounted

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = The part is { $is } and should be { $should }

# What was found wrong while making room.
disks-room-check-open = The open copy of the encryption is not the size it should be

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = The files' area is { $is } and should be { $should }

# What was found wrong while making room.
disks-room-check-area-small = The files' area is smaller than what is in use

# What was found wrong while making room.
disks-room-check-fit = What holds what no longer fits

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] The part is no longer listed by the drive service, so the work was stopped.
        [copy] The open copy is no longer listed by the drive service, so the work was stopped.
       *[files] The filesystem is no longer listed by the drive service, so the work was stopped.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = making the files' area smaller

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = making the open copy of the encryption smaller

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = making the part smaller

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = checking the sizes

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Before making room under { $dev } ({ $at })
