
# machine translated (German), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Text

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Unbenannt

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Öffnen

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Neu (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Öffnen (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Zuletzt geöffnete Dateien

# Button in the menu that saves the file as it is.
text-save = Speichern

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Speichern (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Speichern unter

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Speichern unter (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Rückgängig (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Wiederholen (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Suchen (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Suchen und ersetzen (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Zu Zeile gehen (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Fertige Ansicht (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Als Webseite exportieren

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Als PDF exportieren

# Button in the menu that turns on wrapping of long lines.
text-wrap = Lange Zeilen umbrechen

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Zeilennummern

# Button in the menu that opens the Text settings.
text-settings = Einstellungen

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = { $name } schließen

# Tip of the button that closes the open bar.
text-close-bar = Die Leiste schließen

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Fertige Ansicht

# Menu item that copies everything in the text.
text-copy-all = Den ganzen Text kopieren

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Ausschneiden

text-copy = Kopieren

text-paste = Einfügen

text-select-all = Alles auswählen

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Rechtschreibung prüfen

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = „{ $c }“ kann in { $encoding } nicht geschrieben werden. Wähle in der Statusleiste eine andere Kodierung.

# The words on the status bar: the line and column of the caret.
text-line-column = Zeile { $line }, Spalte { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } Wort
       *[other] { $words } Wörter
    }, { $letters ->
        [one] { $letters } Buchstabe
       *[other] { $letters } Buchstaben
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } Wort
       *[other] { $words } Wörter
    }, { $letters ->
        [one] { $letters } Buchstabe
       *[other] { $letters } Buchstaben
    } ausgewählt

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows-Zeilenenden

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Kodierung: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Dieser Text hat noch keine Datei.

# Words on the find bar: the find box's label and placeholder.
text-find = Suchen

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Groß-/Kleinschreibung beachten

# Words on the find bar: a switch to find only whole words.
text-whole-words = Ganze Wörter

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Vorheriger (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Nächster (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Ersetzen durch

# Button on the replace bar that replaces the word found.
text-replace = Ersetzen

# The words a screen reader gives the Replace button.
text-replace-spoken = Das gefundene Wort ersetzen

# Button on the replace bar that replaces every word found.
# SAFETY: needs a human review before it ships
text-replace-all = Alle ersetzen

# The words a screen reader gives the Replace all button.
# SAFETY: needs a human review before it ships
text-replace-all-spoken = Jedes gefundene Wort ersetzen

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Nichts gefunden

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } von { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } gefunden

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 ersetzt
       *[other] { $n } ersetzt
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Gib eine Zeilennummer ein

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 bis { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = von { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Los

# Words on the go to line bar: the box's label.
text-go-to-line = Zu Zeile gehen

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Zurück zum Text

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = gehört jetzt zu deinen Wörtern

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = steht nicht im Wörterbuch, das nichts Ähnliches hat

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = steht nicht im Wörterbuch. Meinst du eines davon?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Diese Schreibweise verwenden

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Zu meinen Wörtern hinzufügen

# Heading over the list of suggested spellings.
text-suggestions = Vorgeschlagene Schreibweisen

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Dateien, die du öffnest oder speicherst, erscheinen hier.

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = Die Liste leeren

# Heading of the Text settings page.
text-settings-title = Texteinstellungen

# Heading of the group on the settings page for when Text starts.
text-starting = Start

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Wenn Text sich öffnet

# First choice: Text opens the files it had open last time.
text-reopen = Die Dateien vom letzten Mal wieder öffnen

# Second choice: Text opens with an empty page.
text-start-blank = Leer starten

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Arbeit, die nicht in einer Datei gespeichert ist, wird immer behalten und kommt in beiden Fällen zurück.

# Heading of the group on the settings page for how the text looks.
text-showing = Anzeige des Textes

# Label of the size slider: how big the text is.
text-size = Größe

# Name of the size slider, read out by a screen reader.
text-text-size = Textgröße

# Heading of the group on the settings page for spelling.
text-spelling = Rechtschreibung

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Es sind keine Hunspell-Wörterbücher installiert.

# Label of the list of spelling languages.
text-language = Sprache

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Sprache der Rechtschreibung

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Automatisch ({ $lang })

# Heading of the font list on the settings page.
text-font = Schrift

# The first font in the list: the font Trindle ships with.
text-own-font = Die von Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Die Datei lesen als

# Heading over the encodings a file is saved in.
text-save-in = Speichern in

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = Beim nächsten Speichern wird die Datei in dieser Kodierung geschrieben.

# Words saying the text has changes that are not saved to its file. $path is the file.
# SAFETY: needs a human review before it ships
text-changes-not-saved = { $name } hat Änderungen, die nicht in der Datei gespeichert sind.

# Words saying the text has changes and no file yet.
text-not-saved = Dieser Text wurde nicht in einer Datei gespeichert.

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = { $name } speichern?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = Nach diesem noch { $left }.

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = Ohne Speichern schließen

# Button in the question before closing that keeps the tab open.
text-keep-open = Offen lassen

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } ist ein Ordner.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } ist zu groß, um es hier zu öffnen.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } konnte nicht geöffnet werden: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } konnte nicht geschrieben werden: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } konnte nicht gespeichert werden: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } konnte nicht gelesen werden: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Die Arbeit konnte nicht in { $folder } behalten werden: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Die Einstellungen konnten nicht gespeichert werden: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } konnte nicht zu deinen Wörtern hinzugefügt werden: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Die Liste konnte nicht geleert werden: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Die Dateiauswahl konnte nicht geöffnet werden.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Ohne einen Session-Bus können keine Dateien ausgewählt werden.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Es gibt kein Wörterbuch namens { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Das Wörterbuch { $lang } konnte nicht gelesen werden: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. Was darin vorher stand, ist in { $path } erhalten.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } Was darin vor diesem Speichern stand, ist in { $path } erhalten.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } wurde gespeichert, als Text angehalten wurde, und was vom Speichern erhalten blieb, kann nicht gelesen werden.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } wurde gespeichert, als Text angehalten wurde, und wurde seitdem geändert, darum bleibt es, wie es ist.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } wurde gespeichert, als Text angehalten wurde, und konnte nicht wie zuvor zurückgesetzt werden: { $reason }. Was darin vor diesem Speichern stand, ist in { $path } erhalten.

# Quiet word beside what was just saved, exported or changed.
text-done = Fertig

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Unbenannt.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Wurde woanders geändert, seit du sie geöffnet hast.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Trotzdem speichern

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Neu laden

