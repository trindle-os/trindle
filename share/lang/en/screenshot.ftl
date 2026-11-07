## Screenshots, the eye dropper and Trindle's own portals (src/screenshot): the picture window, what the desktop asks through the portals, and the file chooser

## The screenshot window (src/screenshot/mod.rs, toolbar.rs, marks.rs, pin.rs)

# Name of the screenshot overlay that covers the screen (what a screen reader calls it).
screenshot-name = Screenshot
# Note on the overlay before any part is chosen.
screenshot-drag = Drag across the part to keep
# Name of the round button that keeps the whole screen.
screenshot-whole = Whole screen
# Name of the round button that closes the overlay or a pinned picture.
screenshot-close = Close
# Name of the round button that gives the screenshot up.
screenshot-cancel = Cancel
# Name of the round button that takes back the last mark.
screenshot-undo = Undo
# Name of the round button that puts back the last mark taken away.
screenshot-redo = Redo
# Name of the round button that copies the picture to the clipboard, and of the copy button of a colour.
screenshot-copy = Copy
# Name of a colour's button after it has been copied.
screenshot-copied = Copied
# Word on the button that saves the picture in the Screenshots folder.
screenshot-save = Save
# Word on the button that asks where to save the picture.
screenshot-save-as = Save as
# Name of the round button that keeps the picture on the screen as a small window.
screenshot-pin = Pin on screen
# Name of the drawing tool that draws freehand.
screenshot-tool-pen = Pen
# Name of the drawing tool that draws a straight line.
screenshot-tool-line = Line
# Name of the drawing tool that draws an arrow.
screenshot-tool-arrow = Arrow
# Name of the drawing tool that draws a box.
screenshot-tool-box = Box
# Name of the drawing tool that draws a circle.
screenshot-tool-circle = Circle
# Name of the drawing tool that highlights.
screenshot-tool-highlighter = Highlighter
# Name of the drawing tool that writes words.
screenshot-tool-words = Words
# Name of the drawing tool that puts numbered spots.
screenshot-tool-numbers = Numbers
# Name of the drawing tool that blurs a patch.
screenshot-tool-blur = Blur
# Name of the round button that draws in red.
screenshot-colour-red = Red
# Name of the round button that draws in blue.
screenshot-colour-blue = Blue
# Name of the round button that draws in green.
screenshot-colour-green = Green
# Name of the round button that draws in orange.
screenshot-colour-orange = Orange
# Name of the round button that draws in purple.
screenshot-colour-purple = Purple
# Name of the round button that draws in white.
screenshot-colour-white = White
# Name of the round button that draws in black.
screenshot-colour-black = Black
# Name of the round button that draws thin lines.
screenshot-thickness-thin = Thin
# Name of the round button that draws lines of middle thickness.
screenshot-thickness-medium = Medium
# Name of the round button that draws thick lines.
screenshot-thickness-thick = Thick
# Name the screen reader gives to a screenshot pinned on the screen.
screenshot-pinned = Pinned screenshot
# Title of the window that asks where to save the screenshot.
screenshot-save-title = Save the screenshot
# Title of the notice that says the screenshot could not be saved.
screenshot-not-saved = Screenshot not saved
# Title of the notice that says the screenshot could not be copied.
screenshot-not-copied = Screenshot not copied
# Words of that notice.
screenshot-clipboard-refused = The clipboard could not take it.
# Words of the notice that the screenshot could not be saved: the picture was not made.
screenshot-no-picture = the picture could not be made
# Words of the notice that the screenshot could not be saved: no bus to ask on.
screenshot-no-bus = there is no session bus to ask where
# Words of the notice that the screenshot could not be saved: the file chooser could not be asked.
screenshot-no-chooser = the file chooser could not be asked
# Words of the notice that the screenshot could not be saved: every file name was taken.
screenshot-no-free-name = no free name
# Words of the notice that the screenshot could not be saved: the path and why it failed.
screenshot-write-failed = { $path }: { $why }

## The eye dropper (src/screenshot/colours.rs)

# Name of the eye dropper's window, and its title.
screenshot-eye-title = Eye dropper
# Words that float beside the i of the eye dropper's window.
screenshot-eye-help = Press the eye dropper, then click anywhere on the screen to take the colour under the pointer. Escape gives up. Each colour is copied as a hex code straight away and kept in the list below, newest first, so it can be copied again later.
# Name of the i button beside the title.
screenshot-eye-about = About the eye dropper
# Name of the main button that takes a colour.
screenshot-eye-take = Take a colour
# Sentence under the title while no colour has been taken.
screenshot-eye-none = Take a colour from anywhere on the screen.
# Sentence under the title once colours have been taken.
screenshot-eye-taken =
    { $n ->
        [one] One colour taken: { $newest }.
       *[other] { $n } colours taken; the newest is { $newest }.
    }
# Shown in the big card while no colour has been taken.
screenshot-eye-latest-empty = The colour you take shows here, large, ready to copy.
# Name of the card that holds the latest colour.
screenshot-eye-latest = Latest colour
# Name of the round button that clears the list of colours.
screenshot-eye-clear = Clear the list
# Head of the card with the list of colours taken.
screenshot-eye-list = Colours taken
# Shown in the list card while no colour has been taken.
screenshot-eye-list-empty = Colours taken with the eye dropper are kept here, newest first.
# Name of the round button that takes a colour out of the list.
screenshot-eye-remove = Remove
# What the saved list of colours is called when it is found damaged and set aside.
screenshot-eye-damaged = Eye dropper colours

## The picker for sharing the screen (src/screenshot/cast_pick.rs)

# Title of the window that asks what an app may share.
screenshot-pick-title = Share your screen with { $app }?
# Sentence under the title when several things can be picked.
screenshot-pick-many = Pick one or more. Only what you pick is shared, until the app stops or closes.
# Sentence under the title when one thing can be picked.
screenshot-pick-one = Pick one. Only what you pick is shared, until the app stops or closes.
# Heading over the screens that can be shared.
screenshot-pick-screens = Screens
# Heading over the windows that can be shared.
screenshot-pick-windows = Windows
# Shown when there is nothing to share.
screenshot-pick-nothing = There is nothing to share.
# Name of the choice that shares every screen at once.
screenshot-pick-desktop = The whole desktop
# Words under that choice.
screenshot-pick-desktop-detail = Every screen, as they sit
# Words under a screen's choice: its size in pixels.
screenshot-pick-size = { $width } by { $height }
# What the screen reader says for a choice that is picked.
screenshot-pick-chosen = { $name }, chosen
# Name of the round button that refuses to share.
screenshot-pick-refuse = Don't share
# Name of the main button that shares what is picked.
screenshot-pick-share = Share

## The portals (src/screenshot/portal_*.rs)

# Name of the button that says no in a question from a program, and of its answer to the program.
screenshot-portal-deny = Deny
# Name of the main button that says yes in a question from a program.
screenshot-portal-allow = Allow
# Name of the round button that gives up a window from a program.
screenshot-portal-cancel = Cancel
# Name of the main button that opens, and the title of a window that opens a file.
screenshot-portal-open = Open
# Title of the window that saves a file, and the word of its main button.
screenshot-portal-save = Save
# Name of the box where a name is typed.
screenshot-portal-name = Name
# Heading when an unknown program asks who the person is.
screenshot-account-asks-unknown = A program asks who you are
# Heading when a program asks who the person is.
screenshot-account-asks = { $who } asks who you are
# Name the screen reader gives to the person's picture.
screenshot-account-picture = Your picture
# Words under the person's name.
screenshot-account-signed-in = Signed in as { $id }
# Name of the search box in the window that asks which app should open a file.
screenshot-apps-search = Search apps
# Shown when no app can open the file.
screenshot-apps-none = No app here can open it
# Name of the switch that remembers the choice.
screenshot-apps-always = Always use this app for this kind of file
# Title of the window that asks which app should open a file.
screenshot-apps-open-with = Open { $name } with
# Title of that window when the file's name is not known.
screenshot-apps-choose = Choose an app
# Name of the screen sharing that other programs see, and the name Trindle gives itself to PipeWire.
screenshot-cast-name = Trindle screen sharing
# What other programs show for the shared screen.
screenshot-cast-node = Screen shared with an app
# Name used for a program that cannot be named, in the window that asks what to share.
screenshot-cast-an-app = An app
# Title of the window that asks what to share.
screenshot-cast-window = Share the screen
# Title of the window that saves several files into a folder.
screenshot-files-save-many = Save into a folder
# Title of the window that chooses a folder.
screenshot-files-folder = Choose a folder
# What a program holding the computer awake is called when it gave no name.
screenshot-inhibit-who = A program
# Heading when a program wants to add an app to the person's apps and cannot be named.
screenshot-launcher-ask = Add this to your apps?
# Heading when a named program wants to add an app to the person's apps.
screenshot-launcher-asks = { $who } wants to add this to your apps
# Name the screen reader gives to the app's icon.
screenshot-launcher-icon = Its icon
# Words under the name, when the app opens a web address.
screenshot-launcher-opens = Opens { $target }
# Title of the window that asks to add an app.
screenshot-launcher-title = Add an app
# Name of the main button that adds the app.
screenshot-launcher-add = Add
# Heading when a program that cannot be named offers a wallpaper.
screenshot-wallpaper-ask = Set this picture as your wallpaper?
# Heading when a program offers a wallpaper.
screenshot-wallpaper-asks = { $who } offers a new wallpaper
# Name the screen reader gives to the picture offered.
screenshot-wallpaper-picture = The new wallpaper
# Name of the main button that sets the wallpaper.
screenshot-wallpaper-set = Set as wallpaper

## Printing (src/screenshot/portal_print.rs)

# Red line under the printer box when no printer is set up.
screenshot-print-no-printer = No printer set up. Add one in Settings, Printers.
# Red line under the printer box when the printing system does not answer.
screenshot-print-not-answering = Printing is not answering.
# Red line under the printer box when printing is not installed.
screenshot-print-not-installed = Printing is not installed on this computer.
# Red line under the printer box when the chosen printer is paused.
screenshot-print-paused = { $name } is paused.
# Label and name of the box where the printer is chosen.
screenshot-print-printer = Printer
# Label and name of the choice of which pages are printed.
screenshot-print-pages = Pages
# The choice to print all the pages.
screenshot-print-all = All
# The choice to print a range of pages.
screenshot-print-range = Range
# Label and name of the box for the number of copies.
screenshot-print-copies = Copies
# Label and name of the choice of printing on both sides.
screenshot-print-sides = Both sides
# Label and name of the choice of colour, and the choice for colour.
screenshot-print-colour = Colour
# The choice to print in black and white.
screenshot-print-mono = Black and white
# Label and name of the choice of paper size.
screenshot-print-paper = Paper
# Label and name of the choice of how the page stands.
screenshot-print-page = Page
# Name of the main button that prints, and the title when nothing else is known.
screenshot-print-go = Print
# Title of the print window for a named document.
screenshot-print-doc = Print { $doc }
# Title of the print window when the program has no document name.
screenshot-print-from = Print from { $who }
# Title of the notice that says a print did not go.
screenshot-print-failed = Could not print
# Words of that notice when the choices made were not kept.
screenshot-print-lost = Its choices were not kept. Print again.

## The file chooser (src/screenshot/file_chooser.rs)

# Line in the menu on a file or folder: change its name.
screenshot-chooser-rename = Rename
# Line in the menu on a file or folder: cut it.
screenshot-chooser-cut = Cut
# Line in the menu: paste what was cut or copied.
screenshot-chooser-paste = Paste
# Line in the menu and name of the round button: make a new folder.
screenshot-chooser-new-folder = New folder
# Line in the menu: move to the Trash.
screenshot-chooser-trash = Move to Trash
# Line in the menu on a file or folder: delete it for good, after asking.
screenshot-chooser-delete = Delete permanently
# Question before deleting for good.
screenshot-chooser-delete-ask = Delete permanently?
# Line in the menu: copy the place of the thing as text.
screenshot-chooser-copy-location = Copy location
# Line in the menu on a folder: pin it to Folders' side bar.
screenshot-chooser-pin = Pin to the sidebar
# Line in the menu on a folder pinned already.
screenshot-chooser-unpin = Unpin
# Red line after a pin could not be kept.
screenshot-chooser-pin-not-kept = The pin could not be saved.
# Red line after some items could not be deleted.
screenshot-chooser-not-deleted =
    { $n ->
        [one] One item could not be deleted: { $names }.
       *[other] { $n } items could not be deleted: { $names }.
    }
# Name of the first place in the side bar, and of the only part of the path when Recent is shown.
screenshot-chooser-recent = Recent
# Name of the person's home folder in the side bar.
screenshot-chooser-home = Home
# Name the screen reader gives to the path along the top.
screenshot-chooser-folder = Folder
# Shown when a folder has nothing in it.
screenshot-chooser-empty = Nothing here
# Name of the choice of which kind of file is shown.
screenshot-chooser-kind = Kind of file
# Said where a place on another computer is not answering.
screenshot-chooser-not-answering = The other computer is not answering.
# What the saved list of last folders is called when it is found damaged and set aside.
screenshot-chooser-damaged = Chooser folders
# Words under the box when the folder a name points into is not there.
screenshot-chooser-no-folder = That folder is not there.
# Words under the box when saving would replace a file.
screenshot-chooser-replace-ask = { $name } is already there. Press Replace, or Enter again, to put this in its place.
# Words under the box when a key is pressed on Recent, which is not a folder.
screenshot-chooser-recent-not-folder = Recent is not a folder. Open one first.
# Words under the box when a key is pressed with nothing picked.
screenshot-chooser-nothing-picked = Nothing is picked.
# Words under the box while a file by that name is already there.
screenshot-chooser-there = { $name } is already there.
# Name of the round button that hides the hidden files.
screenshot-chooser-hide-hidden = Hide hidden files
# Name of the round button that shows the hidden files.
screenshot-chooser-show-hidden = Show hidden files
# Words of the main button when the program gave none, to choose a folder.
screenshot-chooser-choose-folder = Choose folder
# Head of the question when a folder of that name is already there.
screenshot-chooser-clash-folder = A folder called { $name } is already there
# Head of the question when a file of that name is already there.
screenshot-chooser-clash-file = { $name } is already there
# Name of the round button that leaves a pasted thing out.
screenshot-chooser-skip = Skip
# Name of the round button that keeps both under different names.
screenshot-chooser-both = Keep both
# Word on the button that replaces what is there.
screenshot-chooser-replace = Replace
# What the screen reader says for the button that replaces a folder.
screenshot-chooser-replace-spoken = Replace: the folder already there goes to the Trash
# Name of the main button that merges two folders.
screenshot-chooser-merge = Merge
# Words in the question when folders are being pasted.
screenshot-chooser-rest-folders = The same answer for every folder already there.
# Words in the question when files are being pasted.
screenshot-chooser-rest-files = The same answer for every file already there.
# Name of the switch that gives the same answer to the rest.
screenshot-chooser-same = Do the same for the rest
# Words under the box after Cut.
screenshot-chooser-was-cut = Cut { $what }. Paste it into a folder.
# Words under the box after Copy.
screenshot-chooser-was-copied = Copied { $what }. Paste it into a folder.
# Words under the box when Paste is pressed and there is nothing to paste.
screenshot-chooser-nothing-cut = Nothing has been cut or copied.
# Words under the box when Paste is pressed while a paste is still going.
screenshot-chooser-paste-busy = A paste is still under way.
# Red line after some items could not be moved.
screenshot-chooser-not-moved =
    { $n ->
        [one] One item could not be moved: { $names }.
       *[other] { $n } items could not be moved: { $names }.
    }
# Red line after some items could not be copied.
screenshot-chooser-not-copied =
    { $n ->
        [one] One item could not be copied: { $names }.
       *[other] { $n } items could not be copied: { $names }.
    }
# Red line after some items could not be put in the Trash.
screenshot-chooser-not-trashed =
    { $n ->
        [one] One item could not be put in the Trash: { $names }.
       *[other] { $n } items could not be put in the Trash: { $names }.
    }
# Name of the box where a file or folder is renamed.
screenshot-chooser-new-name = New name for { $old }
# What several things are called in a line such as Cut.
screenshot-chooser-items =
    { $n ->
        [one] { $n } item
       *[other] { $n } items
    }

## Email (src/screenshot/portal_email.rs)

# Title of the notice when an email could not be started.
screenshot-mail-failed = Could not start the email
# Why: no mail program is chosen or installed.
screenshot-mail-no-app = No mail program is set. Choose one in Settings, Default apps.
# Why: a program asked to attach more files than allowed.
screenshot-mail-too-many = Too many files to attach.
# Why: a file, or all the files together, are too big.
screenshot-mail-too-big = The files are too big to attach.
# Why: a file could not be read as a plain file.
screenshot-mail-bad-file = A file could not be attached.
# Why: an address was not a plain address.
screenshot-mail-bad-address = An address was not valid.
# Why: the subject was too long or had odd characters.
screenshot-mail-bad-subject = The subject was not valid.
# Why: the words were too long or had odd characters.
screenshot-mail-bad-words = The words were not valid.
# Why: no private place to keep the files.
screenshot-mail-no-place = The files could not be kept safely.
# Why: the mail program did not start.
screenshot-mail-not-started = The mail program did not start.
