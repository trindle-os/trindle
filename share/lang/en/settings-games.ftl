# Settings, Games page: what helps Steam games run well that Steam itself does not set.

## Games page (src/settings/panels/games.rs, src/common/games.rs)

# The page's name in the side bar.
settings-games-title = Games
# Words a search finds the page by.
settings-games-keywords = games steam proton crackle stutter sound buffer drive library split lock game controller gamepad joystick pad buttons keys change remap swap stick trigger d-pad xbox playstation nintendo
# What the page is for, in the help.
settings-games-help = Settings that help games run well. Change Steam's own settings in Steam. To change a controller button, press it. It moves to the top of the list and lights up in the picture. Then press the keyboard key it should become. A button can also become another button, or nothing, from its list. Trindle keeps the changes for each make and model of controller, and games use the changed controller. Press Escape to cancel a change you started.
# The sentence at the top of the page.
settings-games-top = Help for games that crackle, stutter or need more space.
# Shown beside a switch while the person is asked for the password.
settings-games-waiting = Waiting for your password…
# The card holding the sound and split-lock switches.
settings-games-smooth = Smoother games
# What that card is for.
settings-games-smooth-about = Fixes for games that crackle or stutter. Both are off to begin with.
# The sound switch.
settings-games-sound = Larger sound buffer
# Added after the words behind the i of its card.
settings-games-sound-sub = For games whose sound crackles. Sound plays a little later.
# After the sound switch is turned on.
settings-games-sound-done-on = Larger buffer on
# After the sound switch is turned on, when the sound system could not be told at once.
settings-games-sound-next-login = On from the next login
# After the sound switch is turned off.
settings-games-sound-done-off = Larger buffer off
# The split-lock switch.
settings-games-split = Older game stutter fix
# Added after the words behind the i of its card.
settings-games-split-sub = Older games stutter less, but one program can slow the rest.
# After the split-lock switch is turned on.
settings-games-split-done-on = Stutter fix on
# After the split-lock switch is turned off.
settings-games-split-done-off = Stutter fix off
# The games drive card.
settings-games-drive-title = Games drive
# What that card is for.
settings-games-drive-about = A second drive for games, ready each time you start up.
# A drive with no name of its own.
settings-games-drive-unnamed = Drive
# One drive in the list: its name, size and kind of filesystem.
settings-games-drive-name = { $name }, { $size }, { $fs }
# The button that makes a drive the games drive.
settings-games-drive-use = Use for games
# Under the drive in use for games.
settings-games-drive-in-use = Games drive at { $place }
# The button that stops using a drive for games.
settings-games-drive-stop = Stop using for games
# After a drive is made the games drive.
settings-games-drive-done = Games drive ready
# After a drive is no longer the games drive.
settings-games-drive-forgotten = No longer used for games
# When there is no second drive.
settings-games-drive-none = No second drive
# When the drives could not be listed.
settings-games-drive-unread = Couldn't list the drives
# When Steam could not be let reach the games drive.
settings-games-drive-steam-failed = Couldn't give Steam the drive
# Added after the words behind the i of its card.
settings-games-drive-steam = Add it in Steam
# Added after the words behind the i of its card.
settings-games-drive-steam-sub = In Steam: Settings, Storage, Add drive.
# Why a Windows-formatted drive cannot be used.
settings-games-drive-windows = This drive is formatted for Windows, and games don't work properly on it. Format it for Linux.
# Why a locked drive cannot be used.
settings-games-drive-locked = Unlock it in Drives first.
# Why a part that holds no files cannot be used.
settings-games-drive-not-files = Not for files
# Why an unknown kind of drive cannot be used.
settings-games-drive-unknown = Games can't run from this

## Steam games (src/settings/panels/games/steam.rs)

# The card listing the person's Steam games.
settings-games-steam-title = Steam games
# What that card is for.
settings-games-steam-about = Fixes for individual games. Close Steam before you change them.
# Shown, and beside every control, while Steam is open.
settings-games-steam-open = Steam is open
# Under that.
settings-games-steam-open-sub = Close Steam to change these.
# Beside a game's controls while the game runs.
settings-games-game-running = The game is running
# Under a game's name.
settings-games-game-sub = Fresh start moves its setup and local saves to the Trash.
# The switch for a game: use the stronger graphics chip.
settings-games-stronger = Use the faster graphics
# After that switch is turned on.
settings-games-stronger-on = Faster graphics on
# After that switch is turned off.
settings-games-stronger-off = Faster graphics off
# The switch for a game: use the community Proton build.
settings-games-proton-one = Use community Proton
# The switch for all games.
settings-games-proton-all = Community Proton for all games
# Added after the words behind the i of its card.
settings-games-proton-all-sub = Installed when a game first needs it.
# While the community Proton build is being installed.
settings-games-proton-installing = Installing community Proton…
# After a game or all games are given community Proton.
settings-games-proton-on = Community Proton chosen
# After the choice is given back to Steam.
settings-games-proton-off = Use Steam's choice
# When community Proton was not found after installing.
settings-games-proton-missing = Couldn't find community Proton.
# The button that gives a game a fresh start.
settings-games-fresh = Fresh start
# After a fresh start.
settings-games-fresh-done = The setup is in the Trash. Steam makes a new one when the game next starts.
# When the game had no Windows setup to move.
settings-games-fresh-none = Nothing to move
# When no Steam games are installed.
settings-games-none = No Steam games found
# Red words when community Proton could not be installed.
settings-games-proton-not-installed = Trindle couldn't install community Proton.
# Red words when Steam's settings could not be changed for a game.
settings-games-proton-not-changed = Trindle couldn't change Steam's settings.
