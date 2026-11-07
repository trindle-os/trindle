# Apps: the window for a .deb file, and programs kept in a box (src/store/deb.rs, boxed/, parts.rs, loose.rs).

## The window for a .deb file (src/store/deb.rs)

# Window title of the window opened for a .deb file.
store-deb-title = Install a package
# Words at the top of the window while the package is read.
store-deb-reading = Reading the package…
# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = This file is not a Debian package that can be installed. It may not have finished downloading.
# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } is ready to install in its own folder.
# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = This package is for { $arch } computers, so it cannot go on this one.
# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = This is Trindle itself. It updates from Apps' Updates page, not from a file.
# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } is a driver or a background service, which cannot work from its own folder, so it cannot be installed here.
# Words on the caution card above every package.
store-deb-caution = A package from outside Debian and Trindle goes in its own folder, where it is restricted but not fully shut away. Please install only ones from makers you trust.
# Words at the top of the window once the package is installed.
store-deb-installed = { $name } is installed in its own folder.
# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Checking where this came from…
# Heading in the card that says where the file came from.
store-deb-where = Where it came from
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = Remove it and what it saved
# Word beside the ring while the package goes in.
store-deb-installing = Installing
# Main button that installs the package.
store-deb-install = Install
# Main button that installs the same version again.
store-deb-reinstall = Reinstall
# Main button that brings the installed package up to this version.
store-deb-update = Update
# Heading of the card for a package not installed yet.
store-deb-goes-in = It goes in its own folder, restricted by default
# Under that heading.
store-deb-goes-in-note = No password needed. It cannot reach the internet, sound or your files until you allow it.
# Heading of the card for a package installed at this version.
store-deb-same-version = Version { $version } is installed in its own folder
# Under that heading.
store-deb-same-version-note = Reinstall it there if it has stopped working properly.
# Heading of the card for a package installed at another version.
store-deb-other-version = Version { $version } is installed in its own folder
# Under that heading: the version this brings.
store-deb-other-version-note = Installing this there brings it up to version { $version }, keeping what it has saved.
# Facts line: how big the package is once installed.
store-deb-fact-size = Size once installed
# Facts line: the computer kind it is made for.
store-deb-fact-arch = Made for
# Text of the About card when the package has no description.
store-deb-no-description = This package has no description.
# Words at the top of the window while the package goes in.
store-deb-installing-note = Installing in its own folder. Fetching what it needs can take a few minutes.
# Words at the top of the window once the package is installed.
store-deb-done = Installed in its own folder. You will find it in the ring.
# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Installed in its own folder, but these parts could not be found, so it may not start: { $parts }.
# Words at the top of the window once the package is removed.
store-deb-removed = Removed from its own folder, with everything it saved.

## Programs kept in a box (src/store/boxed/, parts.rs, loose.rs)

# Red words when a file is not a Debian package.
store-boxed-not-deb = This file is not a Debian package that can be installed.
# Step shown while a package is unpacked.
store-boxed-unpacking = Unpacking { $name }
# Step shown while a part a package needs is added.
store-boxed-adding = Adding { $part }
# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } is not a Debian package that can be installed.
# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } could not be unpacked.
# Red words when the program is not one kept in a box.
store-loose-not-boxed = That program is not kept in a box.
# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Opened without the internet
# Body of that notice: why, and what the program was given.
store-downloaded-why = It was downloaded, so the program has only that file, read only.
# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal
# Title of the notice when older programs are off.
store-boxed-x-off = Older programs are off
# Body of that notice: where to turn them on.
store-boxed-x-off-where = Settings, Permissions
# Title of the notice when too many older programs are open.
store-boxed-x-many = Too many older programs
# Body of that notice.
store-boxed-x-many-close = Close one and try again
# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } could not open
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = The terminal can reach more of your computer than other apps can, because it needs to for its work. Only type in commands you understand, from people you trust.
# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = Home network is off
# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } has no network: { $why }
# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = it could not find its own program
# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare is not installed
# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta is not installed
# The same: the network room was not made.
store-boxed-home-why-not-made = its network room was not made
# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = its network room was not new
# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = the system call filter could not be made
# The same: the room's rules would not load.
store-boxed-home-why-rules = the room's rules would not load
# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } would not start: { $why }
# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta said: { $why }
# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta could not join the room
