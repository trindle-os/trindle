# Settings page "Keyboard and mouse": layouts, Caps Lock, changed keys, programs' keyboards, game controllers, mouse and touchpad. The names of keys that a person types (Escape, Ctrl, Caps Lock) are read by the program and are not here.

## Page (src/settings/panels/keyboard.rs)

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Keyboard and mouse
# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout language typing caps lock on-screen keyboard button ring mouse touchpad trackpad pointer speed tap click scroll wheel slower faster natural left handed plugin shortcut keys remap change swap ctrl escape super alt mac side buttons tapped held made by programs keyd
# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Everything here changes straight away. You can switch to a second layout by pressing Alt and Shift together.

## Top sentence

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Typing in { $layout }.
# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt and Shift swap to { $layout }.
# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock does nothing.
# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock is { $key }.
# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = another Ctrl
# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape
# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace
# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = This computer doesn't have the list of keyboard layouts.
# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Keyboard
# Heading of the line in that card.
settings-keyboard-no-list-heading = The keyboard layout list isn't available
# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = To install it, run: sudo apt install xkb-data
# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] You changed one key or button.
       *[other] You changed { $n } keys and buttons.
    }

## Layout

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Layout
# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Layout
# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Variant
# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Standard
# First choice in the list of second layouts: no second layout.
settings-keyboard-none = None
# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Switch to
# Words behind the i of the layout card.
settings-keyboard-layout-about = This sets what each key types. You can choose a layout and a variant of it. Press Alt and Shift together to switch to a second layout, and again to switch back. Changes happen straight away.
# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = In use now

## Other keys

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock
# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Another Ctrl
# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape
# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace
# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Nothing
# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Use Caps Lock as
# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Keyboard button in the ring
# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Other keys
# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock can work as another key instead, such as a second Ctrl or Escape. The keyboard button sits in the middle of the ring and opens a keyboard on the screen. You can use it to type with a mouse or a touch screen. It shows up the next time the ring opens.

## Keys and buttons

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = A program can't change this.
# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Press the key or mouse button to change.
# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = You didn't press anything, so nothing was added.
# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Type the key to use instead, such as Escape or Ctrl+C, or type Nothing.
# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle couldn't tell which key that was. Choose the key from the list instead.
# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Every keyboard and mouse
# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = You've changed this key yourself below, and that change is used.
# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Change { $key } on
# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Change { $key } to
# Name of the bin button that takes a change away.
settings-keyboard-take-away = Remove this change
# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = Line { $n } was ignored: { $why }
# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Couldn't read the changed keys
# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Your keys work as normal until Trindle can read the changes. { $why }
# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = Not saved. The file of changed keys couldn't be read, so it was left as it is.
# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Choose the key or button to change
# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Press the key or button to change
# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Change a key or button
# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = You've already changed { $key }.
# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Keys and buttons
# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Any key or mouse button can do something else. It can become another key or button, or keys pressed together such as Ctrl+C, or keys pressed one after another such as H, I. It can also do nothing, or do one thing when tapped and another when held, such as Escape when tapped and Ctrl when held. This works everywhere, starting from the login screen. Ctrl, Alt and an F key always open a text console, and typing trindle-settings --keys default there puts every key back.

## Ready-made swaps

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Swap Ctrl and Caps Lock
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: tap Escape, hold Ctrl
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Swap Escape and Caps Lock
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Swap Alt and Super, as on a Mac keyboard
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = The mouse's side buttons copy and paste

## Programs' keyboards and mice

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = Trust { $name }
# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = None so far
# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Keyboards and mice made by programs
# Words behind the i of that card.
settings-keyboard-made-about = Remapping tools and game launchers can make their own keyboard or mouse. Their keys and clicks can still type and play, but they can't press protected buttons, such as Shut down or Allow, or answer a question meant for you. Switch one on here to let it count as you.

## Game controllers

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Controller { $vendor }:{ $product }
# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } on { $title }
# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = What { $title } does
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Swap A and B
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Swap X and Y
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Reverse up and down on the right stick

## Plugin shortcuts

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Plugin shortcuts
# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } isn't used: { $why }
# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Keys for { $label }
# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Saved

## The screenshot and the shortcut list

# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Screenshots, reports and shortcuts
# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Take a screenshot
# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Show the shortcut list
# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Take a screenshot now
# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Report a problem
# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Report a problem now
# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Show the list now

## Restarting the desktop

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Restart the desktop
# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Restart the ring and helpers
# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Restart everything
# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Keys for { $label }
# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Restart now. Open windows stay as they are.
# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Ask, then restart everything
# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Restarted
# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = Another shortcut already uses { $keys }.
# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Not saved. { $why }

## Mouse and touchpad

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = For the touchpad too.
# Title of the mouse card.
settings-keyboard-mouse-title = Mouse
# Words behind the i of the mouse card.
settings-keyboard-mouse-about = Pointer speed is how far the pointer moves when you move the mouse. It applies to the touchpad too. Scroll speed is how far a page moves for each turn of the wheel. Choose Slower, Normal or Faster, or set it anywhere on the slider. With natural scrolling, the page moves the way the wheel turns, as on a phone. Left-handed swaps the left and right buttons.
# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Pointer speed
# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Scroll speed
# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Natural scrolling
# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Left-handed
# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Clicking aid
# Small words under the clicking helper line.
settings-keyboard-clicker-about = Clicks from a dwell or switch clicker count as your own.
# Title of the touchpad card.
settings-keyboard-touchpad-title = Touchpad
# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = Scroll speed is how far a page moves when you scroll with two fingers. Tap to click makes a light tap count as a click. With natural scrolling, the page moves the way your fingers move, as on a phone. Off while typing stops your palm from moving the pointer while you type.
# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tap to click
# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Off while typing
# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Slower
# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal
# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Faster

## USB ports

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB ports
# Words behind the i of that card.
settings-keyboard-usb-about = If a USB port fails to start, Trindle switches it off and on once to try again. It never does this more than once for each port.
# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle also couldn't undo the change: { $undo }
