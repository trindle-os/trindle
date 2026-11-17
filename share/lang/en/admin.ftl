# Words of the administrator's helper (trindle-admin), which Settings and Apps show when a step behind the administrator's password fails or is refused. The 'usage:' lines it prints to its maker, and the words it hands to other programs to read, are not here.

## Accounts and passwords (src/admin/mod.rs)

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = That isn't a valid login name.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = That login name is too long.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = There's no user called { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } is a system account, so Trindle hasn't changed it.

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = No password was given.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
        [one] The password needs at least { $n } character.
       *[other] The password needs at least { $n } characters.
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = A password can't have a line break.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } already exists.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } is the only administrator, so they must stay an administrator.

# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = You can't delete your own account.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = Trindle deleted { $name }'s account, but couldn't remove their login screen picture: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = That picture is too large.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = That isn't a picture Trindle made.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle couldn't give the login screen access to the pictures folder: { $why }

## Language, computer name and charging (src/admin/mod.rs)

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = That isn't a language name.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle couldn't read Debian's list of languages: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian doesn't recognise the language { $code }.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = A computer name can only use letters, numbers and hyphens. It can be up to 63 characters long and can't start or end with a hyphen.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = The charging limit is on for now, but it will be lost when the computer is switched off: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Charging to full is on for now, but the limit will come back the next time the computer starts: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

## Sharing and plugins through the firewall (src/admin/mod.rs)

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Other computers can still connect, because Trindle couldn't switch sharing off: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = That isn't a plugin ID.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = There's no such person.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = There's no folder for that person.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle couldn't save the list of allowed plugins: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = There's no such plugin.

## Updates, apps, firmware and Tailscale (src/admin/mod.rs)

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Nothing was updated. Trindle couldn't tell which version of itself is installed, so it couldn't leave its own update out.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Installing the updates

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Tidying up

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Installing Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = That isn't a package name.

# Red words when the package to remove is not installed.
admin-package-not-installed = That package isn't installed.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = That isn't a device fwupd knows.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle can't tell who wants Tailscale.

## Blocking sites and tools it runs (src/admin/mod.rs)

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Blocking didn't start in time.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Blocking isn't fully off: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle couldn't put { $path } back: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = That isn't a site name.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = No such server.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } stopped before it had all it needed, so the job isn't finished: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } failed.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = Couldn't remove { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Nothing was changed, because Trindle couldn't read { $path }: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle also couldn't put things back as they were: { $problem }

## Earlier versions of the home folder (src/admin/home_snapshots.rs)

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = That isn't a valid length of time to keep earlier versions.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Can't find the earlier versions.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = There's a file where the folder for earlier versions should be.

## Shared folders on other computers (src/admin/shares.rs)

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Can't tell who asked.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = This only runs through pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = That isn't the name of a computer.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = That isn't the name of a shared folder.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = That isn't a computer's address.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = That isn't the path of a shared folder.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = You can only close your own shared folders here.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = No shared folder is open there.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Something has changed { $path } since Trindle set it up, so nothing is connected there.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } isn't as Trindle left it.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = There are too many shared folders open.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } isn't installed. It comes with Debian's { $package } package.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Couldn't read the login.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle couldn't read what is mounted on this computer: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = A program is still using something in this folder. Close the program, then try again.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = That computer isn't answering. The shared folder will close when you log out.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = That isn't a user ID.

## Sharing folders from this computer (src/admin/sharing.rs)

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Couldn't make the sambashare group.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Getting Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Setting up sharing

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = Sharing isn't set up yet. Share a folder first.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = This computer is still visible to others on the network: { $why }

## Snapshots of the system (src/admin/snapshots.rs)

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = That isn't a snapshot number.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Taken in Settings

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle couldn't tell which disk the system is on.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = The system isn't in a Btrfs subvolume of its own, so Trindle can't swap it for a snapshot.

## Extra software sources and their programs (src/admin/sources.rs)

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle couldn't get the signing key from { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = That address didn't send back a signing key.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Nothing was changed, because Trindle couldn't read { $path }: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Getting its signing key

# Second line of the progress while a source is added.
admin-step-adding-source = Adding it and reading its list

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Couldn't use the source. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle couldn't use the source, so it removed it again. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } can't switch it on or off yet: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle couldn't start it for { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Trindle couldn't let { $user } use virtual machines, which Cowork needs: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = Trindle couldn't let { $user } use it without a password: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } is installed, but Trindle couldn't finish setting it up: { $why }


# Line of the progress while a program is installed.
admin-step-installing-it = Installing it

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = That isn't one of the ready-made sources.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } isn't made for this kind of computer ({ $arch }).


# Last line of the progress while a program is installed.
admin-step-setting-up = Setting it up

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Each address must be a plain web address that starts with https://.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = The suite and components must be plain words.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = That isn't a package name.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Removing it

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Reading the lists again

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Only sources that Trindle added can be removed here.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = The source is still there: { $why }

## Restoring a snapshot (src/admin/snapshots.rs)

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = Snapshot { $n } isn't on the disk.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Nothing was changed, because Trindle couldn't get the snapshot ready. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle couldn't set the current system aside: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = The current system is now called { $aside } instead of { $sub }, and the computer won't start until it is called { $sub } again: { $why }

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = The snapshot is in place, and the computer will start from it. Trindle couldn't finish tidying up the old system. It will try again when the computer next starts: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle couldn't move the snapshots across: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle couldn't put the snapshot in place: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle couldn't update the list of systems that were set aside: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Couldn't remove { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Trindle removed these because nothing needed them any more: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Not all the updates were installed. These were kept back: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off. Keep "DKMS) will not build": Help finds its page by those words.
admin-driver-off-known = The { $feature } is off because its driver ({ $driver }, DKMS) will not build for the new Linux kernel yet.
admin-driver-off = The driver ({ $driver }, DKMS) will not build for the new Linux kernel yet. Features that need it are off.
# Said once after the drivers that are off.
admin-driver-off-rest = The kernel and the rest of the update were installed.
# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = virtual camera

admin-language-in-use = { $code } is the language this computer starts in, so it can't be removed.
# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = This computer has no split-lock slowdown to switch off.
# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle couldn't find that drive, or it's the system drive.
# The drive could not be closed where it was open.
admin-games-drive-busy = The drive is in use. Close any files or programs on it, then try again: { $why }
# Another software change has the read-only system open.
admin-system-busy = Another software change is under way. Try again when it has finished.
# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Nothing was changed, because Trindle couldn't unlock the system: { $why }
# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle couldn't lock the system again. It will be locked at the next start: { $why }
# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle couldn't keep /usr/local writable: { $why }
# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Only root can run this, and not through pkexec.
# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = This needs Btrfs.

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = You can open at most { $most } ports.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle couldn't save the list of open ports: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = The firewall is still on: { $why }
# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = This plugin now asks for other network ports than the ones you allowed. Allow it again to open them.
# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = A program can only be added for the person who asks.
# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = Another restore used the same name a moment ago. Wait a few seconds, then try again.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } couldn't be disconnected: { $error }
# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 shared folder didn't respond, so it's still connected. It will be disconnected when the computer shuts down.
       *[other] { $n } shared folders didn't respond, so they're still connected. They will be disconnected when the computer shuts down.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = Couldn't switch off the swap file { $path }: { $why }
# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = Couldn't switch the swap file { $path } back on: { $why }
# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } isn't one of the system's swap files.
# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = Trindle won't use the swap file { $path }, because others could change or read { $why }.
# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = Couldn't find where the swap file is now, so waking from hibernation may not work.
# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = Hibernation will work again after you restart.
