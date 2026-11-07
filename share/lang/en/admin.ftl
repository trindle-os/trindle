# Words of the administrator's helper (trindle-admin), which Settings and Apps show when a step behind the administrator's password fails or is refused. The 'usage:' lines it prints to its maker, and the words it hands to other programs to read, are not here.

## Accounts and passwords (src/admin/mod.rs)

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = That is not a valid login name.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = That login name is too long.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = There is no user called { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } is a system account and is left alone.

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = No password was given.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
        [one] The password needs at least { $n } character.
       *[other] The password needs at least { $n } characters.
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = A password cannot hold a line break.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } already exists.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } is the only administrator, so they stay one.

# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = You cannot delete your own account.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = { $name } was deleted, but their picture is still kept for the login screen: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = That picture is too large.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = That is not a picture Trindle made.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = The pictures' folder could not be opened to the login screen: { $why }

## Language, computer name and charging (src/admin/mod.rs)

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = That is not a language name.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Debian's list of languages could not be read: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } is not a language Debian knows.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = A computer name is letters, numbers and hyphens, up to 63, not starting or ending with a hyphen.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = The limit is on for now, but will be lost when the computer is switched off: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Charging is full for now, but the limit will come back at the next start: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

## Sharing and plugins through the firewall (src/admin/mod.rs)

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Other computers can still connect, as the switch could not be put off: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = That is not a plugin's id.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = That person is not known.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = That person's folder is not known.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = The list of plugins let through could not be kept: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = That plugin was not found.

## Updates, apps, firmware and Tailscale (src/admin/mod.rs)

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Nothing was updated: which version of Trindle this computer has could not be told, so Trindle's own update could not be left out.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Installing the updates

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Tidying up

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Installing Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = That is not a package's name.

# Red words when the package to remove is not installed.
admin-package-not-installed = Not done: it is not installed.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = That is not a device fwupd knows.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Could not tell who asked for Tailscale.

## Blocking sites and tools it runs (src/admin/mod.rs)

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Blocking did not start in time.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Blocking is not fully off: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = { $path } could not be put back: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = That is not a site name.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = No such server.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } stopped before it was given everything, so this cannot be called done: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } failed.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } could not be removed: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Nothing was changed, because { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Putting things back as they were did not work either: { $problem }

## Earlier versions of the home folder (src/admin/home_snapshots.rs)

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = That is not a time earlier versions can be kept for.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = The snapshots' folder is not there.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Something that is not a folder is where the snapshots go.

## Shared folders on other computers (src/admin/shares.rs)

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Could not tell who asked.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = This is only run through pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = That is not the name of a computer.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = That is not the name of a shared folder.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = That is not a computer's address.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = That is not the path of a shared folder.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Only your own shared folders can be closed here.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = No shared folder is open there.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } is not as Trindle left it, so nothing is mounted there.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } is not as Trindle left it.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = There are too many shared folders open.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } is not installed. It comes in Debian's { $package }.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = The login could not be read.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = What is mounted on this computer could not be read: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = A program still has something open on it. Close that first.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = That computer is not answering. It is closed when you log out.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = That is not a user id.

## Sharing folders from this computer (src/admin/sharing.rs)

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = The sambashare group could not be made.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Fetching Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Setting up sharing

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = Sharing is not set up yet. Share a folder first.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = This computer is still shown to others on the network: { $why }

## Snapshots of the system (src/admin/snapshots.rs)

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = That is not a snapshot number.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Taken in Settings

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Could not tell which disk the system is on.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = The system is not in a Btrfs subvolume of its own, so it cannot be swapped for a snapshot.

## Extra software sources and their programs (src/admin/sources.rs)

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Could not fetch the signing key from { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = What that address gave back is not a signing key.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Nothing was changed, because { $path } cannot be read: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Fetching its signing key

# Second line of the progress while a source is added.
admin-step-adding-source = Adding it and reading its list

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = The source could not be used. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = The source could not be used, so it was taken away again. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } cannot switch it on or off yet: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = it could not be started for { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = { $user } could not be let use virtual machines, which Cowork needs: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = { $user } could not be let use it without a password: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } is installed, but its setting up did not finish: { $why }


# Line of the progress while a program is installed.
admin-step-installing-it = Installing it

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = That is not one of the ready-made sources.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } is not made for this kind of computer ({ $arch }).


# Last line of the progress while a program is installed.
admin-step-setting-up = Setting it up

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Addresses must start with https:// and hold nothing else.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = The suite and components must be plain words.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = That is not a package name.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Taking it away

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Reading the lists again

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Only sources Trindle added are removed here.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = The source is still there: { $why }

## Restoring a snapshot (src/admin/snapshots.rs)

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = Snapshot { $n } is not on the disk.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Nothing was changed: the snapshot could not be made ready. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Could not set the present system aside: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = the present system is now named { $aside } and not { $sub }, and the computer will not start until it is named { $sub } again: { $why }

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = The snapshot is in place and the computer will start from it. Tidying up the old system did not finish and will be tried again at the next start: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Could not move the snapshots across: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Could not put the snapshot in place: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = The list of what was set aside could not be brought up to date: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Could not remove: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Nothing needed these any more, so they were removed: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Not all updates went in. Kept back: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = The { $feature } is off: its driver ({ $driver }, DKMS) will not build on the new kernel yet.
admin-driver-off = A driver ({ $driver }, DKMS) will not build on the new kernel yet, so what it gives is off.
# Said once after the drivers that are off.
admin-driver-off-rest = The kernel and the rest of the update went in.
# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = virtual camera

admin-language-in-use = { $code } is the language this computer starts in, so it stays.
# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = This computer has no split-lock slowing to turn off.
# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = That drive was not found, or holds the system.
# The drive could not be closed where it was open.
admin-games-drive-busy = The drive is in use. Close what is open on it, then try again: { $why }
# Another software change has the read-only system open.
admin-system-busy = Another software change is under way. Try again when it ends.
# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Nothing was changed: the system could not be unlocked: { $why }
# The read-only system could not be locked again after a change.
admin-system-left-open = The system could not be locked again. It will be locked at the next start: { $why }
# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = /usr/local could not be kept writable: { $why }
# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Only root can run this, and not through pkexec.
# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Needs Btrfs.
