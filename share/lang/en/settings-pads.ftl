# Settings page "Game controllers": a picture of the controller that lights as it is pressed, a list of its buttons, and what each becomes.

# The page's name in the side bar.
settings-pads-title = Game controllers

# The sentence at the top of the page when a controller is plugged in.
settings-pads-top-idle = Press a button on the controller to change it.
# The sentence at the top of the page when a button has been pressed; { $name } is the button.
settings-pads-top-picked = Now press the key for { $name }.

# Heading of the line shown when no game controller is plugged in.
settings-pads-none = None plugged in
# Name of the list that chooses which controller is shown; { $title } is the one shown now.
settings-pads-which = Which controller
# What the picture is, for the screen reader.
settings-pads-picture = Picture of { $title }. Each button lights up as you press it.

# Title of the card of the controller's buttons.
settings-pads-buttons-title = Buttons
# Words behind the i of that card.
settings-pads-buttons-about = Every button and trigger, and every direction on the sticks and d-pad. The one you pressed last is at the top. If you change a button into a key, it types that key in a game that has no controller support.
# Name of the round button that starts changing a button; { $name } is the button.
settings-pads-change = Change { $name } into a key
# Name of the round button that stops changing a button; { $name } is the button.
settings-pads-stop = Stop changing { $name }
# Name of the list of what a button becomes; { $name } is the button.
settings-pads-choose = What { $name } does
# What the list says while a button waits for its key: the clue, and a choice that does nothing.
settings-pads-waiting = Press a key or choose a button
# The next choice in that list: the Escape key.
settings-pads-escape = Escape
# Where a choice in that list comes from; { $what } is the key, mouse button or controller button.
settings-pads-on-keyboard = Keyboard: { $what }
settings-pads-on-mouse = Mouse: { $what }
settings-pads-on-pad = Game controller: { $what }
# The next choice in that list: nothing at all.
settings-pads-nothing = Nothing
# The next choices in that list: the mouse buttons.
settings-pads-left-click = Left click
settings-pads-right-click = Right click
settings-pads-middle-click = Middle click
# Name of a way of the d-pad; { $way } is up, down, left or right.
settings-pads-dpad = D-pad { $way }
# Name of a way of a stick; { $stick } is Left or Right and { $way } is up, down, left or right.
settings-pads-stick-way = { $stick } stick { $way }
settings-pads-left = left
settings-pads-right = right
settings-pads-up = up
settings-pads-down = down
# The two sticks, at the start of a name such as Left stick up.
settings-pads-stick-left = Left
settings-pads-stick-right = Right

# Title of the card of the ready-made changes.
settings-pads-quick-title = Common changes
# Title of the card of changes that are not one button into another.
settings-pads-other-title = Other changes
# Words behind the i of that card.
settings-pads-other-about = Reverse or swap the sticks, or set a dead zone. Type a change like this: Left Stick dead zone = 10%, or Right Stick Y = inverted.

# Title of the card that chooses a profile for each game.
settings-pads-profiles-title = Profiles
# Words behind the i of that card.
settings-pads-profiles-about = A profile is a set of changes. Standard changes nothing, so games with their own controller support see the controller as it is. Every Steam game uses Standard unless you choose a profile for it here. Trindle switches to that profile when the game starts. Any other game means any game that Steam didn't start.
# The one switch that turns every controller change off at once.
settings-pads-changes = Controller changes
# The game whose profile is shown.
settings-pads-game = Game
settings-pads-other-game = Any other game
settings-pads-game-running = { $name } (running)
settings-pads-game-number = Steam game { $n }
# The profile the shown game uses.
settings-pads-profile = Profile
settings-pads-standard = Standard
settings-pads-profile-add = Add a profile
settings-pads-profile-remove = Remove { $name }
settings-pads-profile-name = Profile name
# The name a new profile is given.
settings-pads-profile-new = Profile { $n }
settings-pads-profile-empty = Type a profile name.
settings-pads-profile-standard = Standard is taken.
settings-pads-profile-taken = That name is taken.
settings-pads-profile-gone = That profile is gone.
settings-pads-not-a-game = Not a Steam app number.
# Why the buttons cannot be changed while the shown game uses Standard.
settings-pads-standard-why = Standard changes nothing. Add a profile first.
# The headline while the shown game uses Standard, and while all changes are off.
settings-pads-top-standard = Using Standard, with no changes.
settings-pads-top-off = Controller changes are off.

# Heading of the line shown when the file of changed controller buttons could not be read.
settings-pads-unread = Trindle couldn't read your changed buttons
# Small words under that line; { $why } is the reason.
settings-pads-unread-about = Until then, every button works as normal and nothing is saved. { $why }
# Shown beside a change that was not saved because the file of changed buttons could not be read.
settings-pads-not-saved-unread = Not saved. The file of changed buttons couldn't be read, so it was left as it is.
