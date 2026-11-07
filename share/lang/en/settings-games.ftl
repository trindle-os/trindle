# Settings, Games page: what helps Steam games run well that Steam itself does not set.

## Games page (src/settings/panels/games.rs, src/common/games.rs)

# The page's name in the side bar.
settings-games-title = Games
# Words a search finds the page by.
settings-games-keywords = games steam proton crackle stutter sound buffer drive library split lock game controller gamepad joystick pad buttons keys change remap swap stick trigger d-pad xbox playstation nintendo
# What the page is for, in the help.
settings-games-help = Settings that help games run well. Steam keeps its own settings in Steam. Press a button on the controller and it comes to the top of the list, lit on the picture. Then press the key on the keyboard that it should become. A button can also become another button, or nothing, from its list. Each controller keeps its own changes, by its make and model, and games read the changed controller. Escape stops a change you started.
# The sentence at the top of the page.
settings-games-top = Help for games that crackle, stutter or need more room.
# Shown beside a switch while the person is asked for the password.
settings-games-waiting = Waiting for the password...
# The card holding the sound and split-lock switches.
settings-games-smooth = Smoother games
# What that card is for.
settings-games-smooth-about = Fixes for games that crackle or stutter. Both are off at first.
# The sound switch.
settings-games-sound = Larger sound buffer
# Under the sound switch.
settings-games-sound-sub = For games whose sound crackles. Sound comes a little later.
# After the sound switch is turned on.
settings-games-sound-done-on = Larger buffer on
# After the sound switch is turned on, when the sound system could not be told at once.
settings-games-sound-next-login = On from the next login
# After the sound switch is turned off.
settings-games-sound-done-off = Larger buffer off
# The split-lock switch.
settings-games-split = Older game stutter fix
# Under the split-lock switch.
settings-games-split-sub = Older games stutter less; one program can slow the rest.
# After the split-lock switch is turned on.
settings-games-split-done-on = Stutter fix on
# After the split-lock switch is turned off.
settings-games-split-done-off = Stutter fix off
# The games drive card.
settings-games-drive-title = Games drive
# What that card is for.
settings-games-drive-about = A second drive for games, ready at every start.
# A drive with no name of its own.
settings-games-drive-unnamed = Drive
# One drive in the list: its name, size and kind of filesystem.
settings-games-drive-name = { $name }, { $size }, { $fs }
# Under a drive that can be used for games.
settings-games-drive-ready = Ready for games
# The button that makes a drive the games drive.
settings-games-drive-use = Use for games
# Under the drive in use for games.
settings-games-drive-in-use = Games drive, at { $place }
# The button that stops using a drive for games.
settings-games-drive-stop = Stop using for games
# After a drive is made the games drive.
settings-games-drive-done = Games drive ready
# After a drive is no longer the games drive.
settings-games-drive-forgotten = No longer used for games
# When there is no second drive.
settings-games-drive-none = No second drive
# Under that.
settings-games-drive-none-sub = Fit or plug in a second drive to keep games on it.
# When the drives could not be listed.
settings-games-drive-unread = The drives could not be listed
# When Steam could not be let reach the games drive.
settings-games-drive-steam-failed = Steam could not be given the drive
# How to add the games drive in Steam.
settings-games-drive-steam = Add it in Steam
# Under that.
settings-games-drive-steam-sub = In Steam: Settings, Storage, Add drive.
# Why a Windows-formatted drive cannot be used.
settings-games-drive-windows = Windows format: games break on it. Format it for Linux.
# Why a locked drive cannot be used.
settings-games-drive-locked = Locked: unlock it in Disks first.
# Why a part that holds no files cannot be used.
settings-games-drive-not-files = Holds no files
# Why an unknown kind of drive cannot be used.
settings-games-drive-unknown = Not a format games run from

## Steam games (src/settings/panels/games/steam.rs)

# The card listing the person's Steam games.
settings-games-steam-title = Steam games
# What that card is for.
settings-games-steam-about = Fixes for single games. Steam must be closed to change them.
# Shown, and beside every control, while Steam is open.
settings-games-steam-open = Steam is open
# Under that.
settings-games-steam-open-sub = Close Steam to change these.
# Beside a game's controls while the game runs.
settings-games-game-running = The game is running
# Under a game's name.
settings-games-game-sub = Stronger graphics, community Proton, fresh start. A fresh start moves the game's Windows setup to the Trash, and saves kept inside it go too unless Steam Cloud has them.
# The switch for a game: use the stronger graphics chip.
settings-games-stronger = Use the stronger graphics
# After that switch is turned on.
settings-games-stronger-on = Stronger graphics on
# After that switch is turned off.
settings-games-stronger-off = Stronger graphics off
# The switch for a game: use the community Proton build.
settings-games-proton-one = Use community Proton
# The switch for all games.
settings-games-proton-all = Community Proton for all games
# Under that.
settings-games-proton-all-sub = Installed the first time. Each game can choose.
# While the community Proton build is being installed.
settings-games-proton-installing = Installing community Proton...
# After a game or all games are given community Proton.
settings-games-proton-on = Community Proton chosen
# After the choice is given back to Steam.
settings-games-proton-off = Steam chooses again
# When community Proton was not found after installing.
settings-games-proton-missing = Community Proton could not be found.
# The button that gives a game a fresh start.
settings-games-fresh = Fresh start
# After a fresh start.
settings-games-fresh-done = Moved to the Trash. Steam makes a new one at the next start.
# When the game had no Windows setup to move.
settings-games-fresh-none = Nothing to move
# When no Steam games are installed.
settings-games-none = No Steam games found
# Under that.
settings-games-none-sub = Games you install in Steam are listed here.
