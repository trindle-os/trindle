# Settings: the Snapshots page (src/settings/panels/snapshots.rs).

## The Snapshots page (src/settings/panels/snapshots.rs)

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Snapshots
# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = snapshots snapshot restore undo rollback roll back backup update broke btrfs snapper boot menu grub earlier versions older version previous deleted file put back recover home copies
# Help line at the foot of the Snapshots page.
settings-snap-help = The system as it was before and after every software change, and whenever you take one. Restoring one puts the whole system back to it from the next start; your own files are left as they are when the home folder has a part of the disk of its own, as Trindle's installer makes it. If the computer will not start, choose Trindle snapshots in the boot menu. Earlier versions keeps a copy of everyone's own files each day, apart from the system's snapshots.
# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = No copy was taken: the disk is nearly full.
# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = today at { $time }
# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = yesterday at { $time }
# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = on { $date }
# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = The computer would not restart. Restart it from the ring instead.
# Said beside the Take a snapshot line while it works.
settings-snap-taking = Taking a snapshot
# Said beside the set aside line while the space is freed.
settings-snap-freeing = Freeing...
# Said beside the set aside line once the space is freed.
settings-snap-freed = Freed
# Said beside a snapshot while it is restored.
settings-snap-restoring = Restoring
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = The system is snapshot { $n } from the next start. Anything changed since then is set aside, not lost.
# Said beside a snapshot while it is deleted.
settings-snap-deleting = Deleting
# Said beside a snapshot once it is deleted.
settings-snap-deleted = Deleted
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = Waiting for the password...
# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = A copy is taken each day
# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = No more are taken
# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Kept
# Said beside the earlier copies while they are cleared.
settings-snap-clearing = Clearing
# Said beside the earlier copies once they are cleared.
settings-snap-cleared = Cleared
# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] No copies yet
        [one] One copy
       *[other] { $n } copies
    }
# Under the number of earlier copies when there are none.
settings-snap-first-soon = The first is taken within a day.
# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = They take { $size }, measured { $when }.
# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Their room is measured once a day.
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = Clear them all now
# Heading of the card about earlier versions of files.
settings-snap-earlier = Earlier versions
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle can keep a copy of everyone's files as they were each day, so an older version of a file, or one deleted by mistake, can be opened or put back from Folders. Each person can read in a copy only what they could read on that day, so a private home stays private; a file that was open to others then stays open to them in that copy until it goes.The copies are apart from the system's snapshots: restoring the system never changes your files, and these never change the system. The cost is room: a file you delete or change still takes its room until the last copy holding it has gone, so deleting things does not free the disk at once. Clearing the copies frees it now, for everyone on this computer, and cannot be undone. No copy is taken while the disk is nearly full. Changing any of this needs an administrator's password.
# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Keep earlier versions of your files
# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Deleted files stay for { $time } before their room is freed.
# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Keep each day's copy for
# Name of the list of how long each day's copy is kept.
settings-snap-how-long = How long each day's copy is kept
# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Looking at the snapshots…
# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = The snapshots could not be read.
# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Started from snapshot { $n }, so nothing done now is kept.
# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] { $n } snapshot to go back to, the newest from { $when }.
       *[other] { $n } snapshots to go back to, the newest from { $when }.
    }
# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = No snapshots yet. One is taken before every software change.
# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, kept until deleted
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = Restore the system to this
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = Delete this snapshot
# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Restart now
# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = They are not set up yet.
# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Reinstall the trindle package.
# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Snapshots are not set up. Reinstalling the trindle package sets them up: sudo apt install --reinstall trindle.
# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = another kind
# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = They need Btrfs; this computer uses { $fs }.
# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Choose Btrfs when installing Debian.
# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Snapshots need the system on Btrfs, and this computer uses { $fs }. Choose Btrfs for the system when installing Debian to have them.
# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = There are no snapshots on this computer.
# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = No snapshots
# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = No snapshots on this computer
# Heading of the line saying how to have snapshots.
settings-snap-to-have = To have them
# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Looking…
# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = The snapshots could not be read
# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = None yet
# Under the None yet heading.
settings-snap-none-line-sub = One is taken before every software change.
# Help text of the Snapshots card (the information button).
settings-snap-list-about = Each snapshot can restore the whole system to how it was then, from the next start, or be deleted. Both ask to be pressed twice and need an administrator's password. After a restore, a button to restart appears beside it.
# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Take a snapshot now
# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Not while started from a snapshot.
# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Kept until you delete it.
# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] One system set aside by restoring
       *[other] { $n } systems set aside by restoring
    }
# Under the set aside heading.
settings-snap-free-space = Free the space once all is well.
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = Free the space they use
# Heading of the card for taking a snapshot.
settings-snap-take-one = Take one
# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = System files locked
# Under the System files locked switch.
settings-snap-locked-sub = Only installs and updates unlock them.
# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Locked
# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Unlocked
# Help text of the Take one card (the information button).
settings-snap-take-about = Take a snapshot before trying something you may want to undo; one you take is kept until you delete it. While the computer is started from a snapshot, nothing taken would be kept, so restart normally first. Each restore sets aside what the system was just before it, so nothing is lost; free the space they use once all is well. With the system files locked, nothing but an install or update can change them, and each has a snapshot before and after it. Changing the lock needs an administrator's password.
# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Started from snapshot { $n }
# Under the Started from snapshot heading.
settings-snap-nothing-kept = Nothing done now is kept.
# Heading of a line on the Before restoring card.
settings-snap-files-too = Your files are in the snapshots too
# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Restoring also puts your files back.
# Heading of the card about what to know before restoring.
settings-snap-before = Before restoring
# Help text of the Before restoring card (the information button).
settings-snap-before-about = Started from a snapshot, nothing done now is kept: if this is how the system should be, press restore on it in the list, then restart. On this computer the home folder may be part of the system, and then restoring a snapshot also puts your own files back to how they were then, so copy anything new somewhere safe first.
