# machine translated (Swahili), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Maandishi

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Bila jina

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Fungua

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Mpya (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Fungua (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Faili za hivi karibuni

# Button in the menu that saves the file as it is.
text-save = Hifadhi

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Hifadhi (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Hifadhi kama

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Hifadhi kama (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Tendua (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Rudia (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Tafuta (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Tafuta na ubadilishe (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Nenda kwenye mstari (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Mwonekano wa mwisho (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Hamisha kama ukurasa wa wavuti

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Hamisha kama PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Kunja mistari mirefu

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Namba za mistari

# Button in the menu that opens the Text settings.
text-settings = Mipangilio

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Funga { $name }

# Tip of the button that closes the open bar.
text-close-bar = Funga upau

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Mwonekano wa mwisho

# Menu item that copies everything in the text.
text-copy-all = Nakili maandishi yote

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Kata

text-copy = Nakili

text-paste = Bandika

text-select-all = Chagua yote

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Kagua tahajia

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ haiwezi kuandikwa katika { $encoding }. Chagua usimbaji mwingine kutoka upau wa hali.

# The words on the status bar: the line and column of the caret.
text-line-column = Mstari { $line }, safu { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] neno { $words }
       *[other] maneno { $words }
    }, { $letters ->
        [one] herufi { $letters }
       *[other] herufi { $letters }
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] neno { $words }
       *[other] maneno { $words }
    }, { $letters ->
        [one] herufi { $letters }
       *[other] herufi { $letters }
    } vimechaguliwa

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Miisho ya mistari ya Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Usimbaji: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Maandishi haya bado hayana faili.

# Words on the find bar: the find box's label and placeholder.
text-find = Tafuta

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Linganisha herufi kubwa na ndogo

# Words on the find bar: a switch to find only whole words.
text-whole-words = Maneno kamili

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Iliyotangulia (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Inayofuata (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Badilisha na

# Button on the replace bar that replaces the word found.
text-replace = Badilisha

# The words a screen reader gives the Replace button.
text-replace-spoken = Badilisha neno lililopatikana

# Button on the replace bar that replaces every word found.
# SAFETY: needs a human review before it ships
text-replace-all = Badilisha yote

# The words a screen reader gives the Replace all button.
# SAFETY: needs a human review before it ships
text-replace-all-spoken = Badilisha kila neno lililopatikana

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Hakuna kilichopatikana

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } kati ya { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } zimepatikana

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 imebadilishwa
       *[other] { $n } zimebadilishwa
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Andika namba ya mstari

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 hadi { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = kati ya { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Nenda

# Words on the go to line bar: the box's label.
text-go-to-line = Nenda kwenye mstari

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Rudi kwenye maandishi

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = sasa ni mojawapo ya maneno yako

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = haimo kwenye kamusi, ambayo haina kinachofanana nayo

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = haimo kwenye kamusi. Je, ulimaanisha mojawapo ya haya?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Tumia tahajia hii

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Ongeza kwenye maneno yangu

# Heading over the list of suggested spellings.
text-suggestions = Tahajia zinazopendekezwa

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Faili unazofungua au kuhifadhi zinaonekana hapa.

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = Futa orodha

# Heading of the Text settings page.
text-settings-title = Mipangilio ya Maandishi

# Heading of the group on the settings page for when Text starts.
text-starting = Kuanza

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Maandishi yanapofunguka

# First choice: Text opens the files it had open last time.
text-reopen = Fungua tena faili za mara iliyopita

# Second choice: Text opens with an empty page.
text-start-blank = Anza na ukurasa mtupu

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Kazi isiyohifadhiwa kwenye faili daima huhifadhiwa na hurudi kwa vyovyote vile.

# Heading of the group on the settings page for how the text looks.
text-showing = Kuonyesha maandishi

# Label of the size slider: how big the text is.
text-size = Ukubwa

# Name of the size slider, read out by a screen reader.
text-text-size = Ukubwa wa maandishi

# Heading of the group on the settings page for spelling.
text-spelling = Tahajia

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Hakuna kamusi za Hunspell zilizosakinishwa.

# Label of the list of spelling languages.
text-language = Lugha

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Lugha ya tahajia

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Otomatiki ({ $lang })

# Heading of the font list on the settings page.
text-font = Fonti

# The first font in the list: the font Trindle ships with.
text-own-font = Ya Trindle yenyewe

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Soma faili kama

# Heading over the encodings a file is saved in.
text-save-in = Hifadhi katika

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = Uhifadhi unaofuata utaandika faili katika usimbaji huu.

# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } ina mabadiliko ambayo hayajahifadhiwa kwenye faili yake.

# Words saying the text has changes and no file yet.
text-not-saved = Maandishi haya hayajahifadhiwa kwenye faili.

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = Hifadhi { $name }?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = { $left } zaidi baada ya hii.

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = Funga bila kuhifadhi

# Button in the question before closing that keeps the tab open.
text-keep-open = Iache wazi

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } ni folda.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } ni kubwa mno kufunguliwa hapa.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } haikuweza kufunguliwa: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } haikuweza kuandikwa: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } haikuweza kuhifadhiwa: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } haikuweza kusomwa: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Kazi haikuweza kuhifadhiwa katika { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Mipangilio haikuweza kuhifadhiwa: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } haikuweza kuongezwa kwenye maneno yako: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Orodha haikuweza kufutwa: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Kichagua faili hakikuweza kufunguliwa.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Faili haziwezi kuchaguliwa bila basi la kipindi.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Hakuna kamusi inayoitwa { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Kamusi ya { $lang } haikuweza kusomwa: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. Kilichokuwa ndani yake kabla kimehifadhiwa katika { $path }.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } Kilichokuwa ndani yake kabla ya uhifadhi huo kimehifadhiwa katika { $path }.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } ilikuwa ikihifadhiwa Maandishi iliposimamishwa, na kilichohifadhiwa cha uhifadhi huo hakiwezi kusomwa.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } ilikuwa ikihifadhiwa Maandishi iliposimamishwa, na imebadilishwa tangu hapo, kwa hiyo imeachwa kama ilivyo.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } ilikuwa ikihifadhiwa Maandishi iliposimamishwa, na haikuweza kurudishwa kama ilivyokuwa: { $reason }. Kilichokuwa ndani yake kabla ya uhifadhi huo kimehifadhiwa katika { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Imekamilika

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Bila jina.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Imebadilishwa kwingine tangu uifungue.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Hifadhi hata hivyo

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Pakia upya

