# Settings: the Snapshots page (src/settings/panels/snapshots.rs).

## The Snapshots page (src/settings/panels/snapshots.rs)

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Snapshots
# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = snapshots snapshot restore undo rollback roll back backup update broke btrfs snapper boot menu grub earlier versions older version previous deleted file put back recover home copies
# Help line at the foot of the Snapshots page.
settings-snap-help = A snapshot is a copy of the system as it was before and after every software change, and whenever you take one. Restoring a snapshot puts the whole system back to how it was when you next restart. Your own files stay as they are if your home folder has its own part of the disk, as Trindle's installer sets up. If the computer won't start, choose Trindle snapshots in the boot menu. Earlier versions keeps a daily copy of everyone's own files, separate from the system's snapshots.
# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Trindle didn't take a copy because the disk is nearly full.
# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = today at { $time }
# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = yesterday at { $time }
# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = on { $date }
# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = The computer wouldn't restart. Restart it from the ring instead.
# Said beside the Take a snapshot line while it works.
settings-snap-taking = Taking a snapshot
# Said beside the set aside line while the space is freed.
settings-snap-freeing = Freeing…
# Said beside the set aside line once the space is freed.
settings-snap-freed = Freed
# Said beside a snapshot while it is restored.
settings-snap-restoring = Restoring
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = When you next restart, the system will be as it was in snapshot { $n }. Anything changed since then is set aside, not lost.
# Said beside a snapshot while it is deleted.
settings-snap-deleting = Deleting
# Said beside a snapshot once it is deleted.
settings-snap-deleted = Deleted
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = Waiting for your password…
# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = A copy is taken each day
# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = No more copies
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
settings-snap-measured-daily = Their space is measured daily.
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = Clear them all now
# Heading of the card about earlier versions of files.
settings-snap-earlier = Earlier versions
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle can keep a daily copy of everyone's files. You can then open an older version of a file, or get back one you deleted by mistake, from Folders. In a copy, each person only sees what they could see on that day, so private home folders stay private. A file that others could open then stays open to them in that copy until the copy is removed. These copies are separate from the system's snapshots, and neither changes the other. They do use disk space. A file you delete or change still takes up space until the last copy holding it is removed. Clearing the copies frees the space straight away for everyone, and you can't undo it. No copy is taken while the disk is nearly full. You need an administrator's password to change any of this.
# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Keep earlier versions of your files
# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Deleted files are kept for { $time }, then freed.
# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Keep each day's copy for
# Name of the list of how long each day's copy is kept.
settings-snap-how-long = How long each day's copy is kept
# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Checking the snapshots…
# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Couldn't read the snapshots.
# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = You started from snapshot { $n }, so nothing you do now is kept.
# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] You can go back to { $n } snapshot, from { $when }.
       *[other] You can go back to { $n } snapshots. The newest is from { $when }.
    }
# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = No snapshots yet. One is taken before every software change.
# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, kept until deleted
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = Restore this snapshot
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = Delete this snapshot
# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Restart now
# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = They aren't set up yet.
# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Snapshots are not set up. Reinstalling the trindle package sets them up: sudo apt install --reinstall trindle.
# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = another kind
# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Snapshots need Btrfs. This computer uses { $fs }.
# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Snapshots need the system to be on Btrfs, and this computer uses { $fs }. To have snapshots, choose Btrfs for the system when you install Debian.
# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = There are no snapshots on this computer.
# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = No snapshots
# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = No snapshots on this computer
# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Loading…
# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Couldn't read the snapshots
# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = None yet
# Help text of the Snapshots card (the information button).
settings-snap-list-about = Each snapshot can put the whole system back to how it was, when you next restart. You need an administrator's password to restore or delete one.
# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Take a snapshot now
# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Restart normally to do this.
# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] One old system kept aside after a restore
       *[other] { $n } old systems kept aside after restores
    }
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = Free the space they use
# Heading of the card for taking a snapshot.
settings-snap-take-one = Take one
# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = System files locked
# Added to the Take one card's i.
settings-snap-locked-sub = Only installs and updates unlock them.
# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Locked
# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Unlocked
# Help text of the Take one card (the information button).
settings-snap-take-about = Take a snapshot before you try something you may want to undo. Snapshots you take are kept until you delete them. If you started from a snapshot, restart normally first, or your new snapshot won't be kept. Each restore sets aside the system as it was just before, so nothing is lost. With the system files locked, only installs and updates can change them, and each one gets a snapshot before and after. You need an administrator's password to change the lock.
# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Started from snapshot { $n }
# Under the Started from snapshot heading.
settings-snap-nothing-kept = Changes won't be kept.
# Heading of a line on the Before restoring card.
settings-snap-files-too = Your files are in the snapshots too
# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Restoring also puts your files back.
# Heading of the card about what to know before restoring.
settings-snap-before = Before restoring
# Help text of the Before restoring card (the information button).
settings-snap-before-about = Because you started from a snapshot, nothing you do now is kept. To keep the system like this, press Restore this snapshot beside it in the list, then restart. On this computer, the home folder may be part of the system. If so, restoring a snapshot also puts your own files back to how they were then, so copy anything new somewhere safe first.
