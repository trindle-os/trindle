# Apps: the window for a .deb file, and programs kept in a box (src/store/deb.rs, boxed/, parts.rs, loose.rs).

## The window for a .deb file (src/store/deb.rs)

# Window title of the window opened for a .deb file.
store-deb-title = Install a package
# Words at the top of the window while the package is read.
store-deb-reading = Reading the package…
# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = This file isn't a Debian package that can be installed. It may not have finished downloading.
# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } is ready to install in its own folder.
# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = This package is for { $arch } computers, so it can't be installed on this one.
# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = This is Trindle itself. Update it from the Updates page in Apps, not from a file.
# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } is a driver or a background service. It can't run from its own folder, so it can't be installed here.
# Words on the caution card above every package.
store-deb-caution = A package from outside Debian and Trindle installs in its own folder. It's restricted there, but not completely shut away. Only install packages from makers you trust.
# Words at the top of the window once the package is installed.
store-deb-installed = { $name } is installed in its own folder.
# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Checking where this came from…
# Heading in the card that says where the file came from.
store-deb-where = Where it came from
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = Remove it and its data
# Word beside the ring while the package goes in.
store-deb-installing = Installing
# Main button that installs the package.
store-deb-install = Install
# Main button that installs the same version again.
store-deb-reinstall = Reinstall
# Main button that brings the installed package up to this version.
store-deb-update = Update
# Heading of the card for a package not installed yet.
store-deb-goes-in = It installs in its own, restricted folder
# Under that heading.
store-deb-goes-in-note = No password is needed. It can't use the internet or sound, or see your files, until you allow it.
# Heading of the card for a package installed at this version.
store-deb-same-version = Version { $version } is installed in its own folder
# Heading of the card for a package installed at another version.
store-deb-other-version = Version { $version } is installed in its own folder
# Under that heading: the version this brings.
store-deb-other-version-note = This updates it to version { $version } and keeps what it has saved.
# Facts line: how big the package is once installed.
store-deb-fact-size = Size once installed
# Facts line: the computer kind it is made for.
store-deb-fact-arch = Made for
# Text of the About card when the package has no description.
store-deb-no-description = This package has no description.
# Words at the top of the window while the package goes in.
store-deb-installing-note = Installing in its own folder. Downloading the parts it needs can take a few minutes.
# Words at the top of the window once the package is installed.
store-deb-done = It's installed in its own folder. You'll find it in the ring.
# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = It's installed in its own folder, but these parts are missing, so it may not start: { $parts }.
# Words at the top of the window once the package is removed.
store-deb-removed = It's been removed from its own folder, with everything it saved.

## Programs kept in a box (src/store/boxed/, parts.rs, loose.rs)

# Red words when a file is not a Debian package.
store-boxed-not-deb = This file isn't a Debian package that can be installed.
# Step shown while a package is unpacked.
store-boxed-unpacking = Unpacking { $name }
# Step shown while a part a package needs is added.
store-boxed-adding = Adding { $part }
# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } isn't a Debian package that can be installed.
# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } couldn't be unpacked.
# Red words when the program is not one kept in a box.
store-loose-not-boxed = That program doesn't run in a box.
# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Opened without the internet
# Body of that notice: why, and what the program was given.
store-downloaded-why = Because it was downloaded, the program can only read that one file.
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
# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = Older programs can't open
# Body of that notice: what to do.
store-boxed-x-unasked-why = Log out and in again, then try again
# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } couldn't open
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = The terminal can reach more of your computer than other apps can. Only type commands you understand, from people you trust.
# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = No local network
# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } has no network: { $why }
# Why a plugin allowed the local network has none: Trindle could not find its own program.
store-boxed-home-why-self = Trindle's program is missing
# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare isn't installed
# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta isn't installed
# The same: the network room was not made.
store-boxed-home-why-not-made = its network wasn't set up
# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = its network already existed
# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle couldn't set up a safety filter
# The same: the room's rules would not load.
store-boxed-home-why-rules = its network rules couldn't load
# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } couldn't start: { $why }
# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta said: { $why }
# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta couldn't connect it

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = The top folder { $name } is a link, so the package can't be used.
# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } is a driver or a background service. Those can't work from their own folder, so it wasn't installed.
