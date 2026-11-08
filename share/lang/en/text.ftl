## Text (src/text): the plain text and Markdown editor, its menus, its find bar, its spelling and its settings.

## The window, the tabs and the menus (src/text/mod.rs)

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Text
# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Untitled
# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Open
# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = New (Ctrl+N)
# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Open (Ctrl+O)
# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Recent files
# Button in the menu that saves the file as it is.
text-save = Save
# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Save (Ctrl+S)
# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Save as
# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Save as (Ctrl+Shift+S)
# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Changed elsewhere since you opened it.
# Button beside that line: save the words here over the changed file.
text-save-anyway = Save anyway
# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Reload
# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Undo (Ctrl+Z)
# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Redo (Ctrl+Shift+Z)
# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Find (Ctrl+F)
# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Find and replace (Ctrl+H)
# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Go to line (Ctrl+G)
# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Finished look (Ctrl+E)
# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Export as a web page
# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Export as PDF
# Button in the menu that turns on wrapping of long lines.
text-wrap = Wrap long lines
# Button in the menu that shows or hides the line numbers.
text-line-numbers = Line numbers
# Button in the menu that opens the Text settings.
text-settings = Settings
# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Close { $name }
# Tip of the button that closes the open bar.
text-close-bar = Close the bar
# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Finished look
# Menu item that copies everything in the text.
text-copy-all = Copy all the text
# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Cut
text-copy = Copy
text-paste = Paste
text-select-all = Select all
# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Check spelling
# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ cannot be written in { $encoding }. Choose another encoding from the status bar.
# The words on the status bar: the line and column of the caret.
text-line-column = Line { $line }, column { $col }
# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } word
       *[other] { $words } words
    }, { $letters ->
        [one] { $letters } letter
       *[other] { $letters } letters
    }
# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } word
       *[other] { $words } words
    }, { $letters ->
        [one] { $letters } letter
       *[other] { $letters } letters
    } selected
# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows line endings
# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Encoding: { $name }
# Words in place of the file's name when the text has never been saved to a file.
text-no-file = This text has no file yet.

## The find bar (src/text/mod.rs)

# Words on the find bar: the find box's label and placeholder.
text-find = Find
# Words on the find bar: a switch to find only the same capitals.
text-match-case = Match case
# Words on the find bar: a switch to find only whole words.
text-whole-words = Whole words
# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Previous (Shift+F3)
# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Next (F3)
# Words on the replace bar: the replace box's label.
text-replace-with = Replace with
# Button on the replace bar that replaces the word found.
text-replace = Replace
# The words a screen reader gives the Replace button.
text-replace-spoken = Replace the word found
# Button on the replace bar that replaces every word found.
text-replace-all = Replace all
# The words a screen reader gives the Replace all button.
text-replace-all-spoken = Replace every word found
# Words on the find bar when a find has been asked for and has no match.
text-none-found = None found
# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } of { $n }
# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } found
# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 replaced
       *[other] { $n } replaced
    }
# Words on the go to line bar when the number typed is not a line number.
text-type-line = Type a line number
# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 to { $lines }
# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = of { $lines }
# Button on the go to line bar that goes to the line typed.
text-go = Go
# Words on the go to line bar: the box's label.
text-go-to-line = Go to line
# Button that goes back from a bar, a settings page or a list to the text.
text-back = Back to the text

## The spelling bar (src/text/mod.rs)

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = is now one of your words
# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = is not in the dictionary, which has nothing like it
# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = is not in the dictionary. Did you mean one of these?
# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Use this spelling
# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Add to my words
# Heading over the list of suggested spellings.
text-suggestions = Suggested spellings

## Recent files (src/text/mod.rs)

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Files you open or save show here.
# Button that clears the list of recent files.
text-clear-list = Clear the list

## The Text settings (src/text/mod.rs)

# Heading of the Text settings page.
text-settings-title = Text settings
# Heading of the group on the settings page for when Text starts.
text-starting = Starting
# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = When Text opens
# First choice: Text opens the files it had open last time.
text-reopen = Reopen last time's files
# Second choice: Text opens with an empty page.
text-start-blank = Start blank
# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Work not saved to a file is always kept and comes back either way.
# Heading of the group on the settings page for how the text looks.
text-showing = Showing the text
# Label of the size slider: how big the text is.
text-size = Size
# Name of the size slider, read out by a screen reader.
text-text-size = Text size
# Heading of the group on the settings page for spelling.
text-spelling = Spelling
# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = No Hunspell dictionaries are installed.
# Label of the list of spelling languages.
text-language = Language
# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Spelling language
# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Automatic ({ $lang })
# Heading of the font list on the settings page.
text-font = Font
# The first font in the list: the font Trindle ships with.
text-own-font = Trindle's own

## Saving and the file dialogs (src/text/mod.rs)

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Read the file as
# Heading over the encodings a file is saved in.
text-save-in = Save in
# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = The next save writes the file in this encoding.
# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } has changes not saved to its file.
# Words saying the text has changes and no file yet.
text-not-saved = This text has not been saved to a file.
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = Save { $name }?
# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = { $left } more after this one.
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = Close without saving
# Button in the question before closing that keeps the tab open.
text-keep-open = Keep it open
# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }
# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } is a folder.
# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } is too big to open here.
# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } could not be opened: { $reason }
# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } could not be written: { $reason }
# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } could not be saved: { $reason }
# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } could not be read: { $reason }
# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Work could not be kept in { $folder }: { $reason }
# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Settings could not be saved: { $reason }
# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } could not be added to your words: { $reason }
# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = The list could not be cleared: { $reason }
# Shown when the file chooser could not be opened.
text-chooser-failed = The file chooser could not be opened.
# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Files cannot be chosen without a session bus.
# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = There is no dictionary called { $lang }.
# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = The { $lang } dictionary could not be read: { $reason }
# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. What it held before is kept in { $path }.
# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } What it held before that save is kept in { $path }.
# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } was being saved when Text was stopped, and what was kept of the save cannot be read.
# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } was being saved when Text was stopped, and has been changed since, so it is left as it is.
# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } was being saved when Text was stopped, and could not be put back as it was: { $reason }. What it held before that save is kept in { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Done

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Untitled.txt
