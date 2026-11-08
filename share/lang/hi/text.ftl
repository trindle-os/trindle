
# machine translated (Hindi), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = टेक्स्ट

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = बिना नाम

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = खोलें

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = नया (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = खोलें (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = हाल की फ़ाइलें

# Button in the menu that saves the file as it is.
text-save = सहेजें

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = सहेजें (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = इस नाम से सहेजें

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = इस नाम से सहेजें (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = वापस लें (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = फिर से करें (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = खोजें (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = खोजें और बदलें (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = पंक्ति पर जाएँ (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = तैयार रूप (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = वेब पेज के रूप में निर्यात करें

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = PDF के रूप में निर्यात करें

# Button in the menu that turns on wrapping of long lines.
text-wrap = लंबी पंक्तियाँ लपेटें

# Button in the menu that shows or hides the line numbers.
text-line-numbers = पंक्ति संख्या

# Button in the menu that opens the Text settings.
text-settings = सेटिंग्स

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = { $name } बंद करें

# Tip of the button that closes the open bar.
text-close-bar = पट्टी बंद करें

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = तैयार रूप

# Menu item that copies everything in the text.
text-copy-all = पूरा टेक्स्ट कॉपी करें

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = काटें

text-copy = कॉपी करें

text-paste = चिपकाएँ

text-select-all = सभी चुनें

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = वर्तनी जाँचें

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ को { $encoding } में लिखा नहीं जा सकता। स्थिति पट्टी से कोई और एन्कोडिंग चुनें।

# The words on the status bar: the line and column of the caret.
text-line-column = पंक्ति { $line }, स्तंभ { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } शब्द
       *[other] { $words } शब्द
    }, { $letters ->
        [one] { $letters } अक्षर
       *[other] { $letters } अक्षर
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } शब्द
       *[other] { $words } शब्द
    }, { $letters ->
        [one] { $letters } अक्षर
       *[other] { $letters } अक्षर
    } चुने गए

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows पंक्ति समाप्ति

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = एन्कोडिंग: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = इस टेक्स्ट की अभी कोई फ़ाइल नहीं है।

# Words on the find bar: the find box's label and placeholder.
text-find = खोजें

# Words on the find bar: a switch to find only the same capitals.
text-match-case = अक्षर-रूप मिलाएँ

# Words on the find bar: a switch to find only whole words.
text-whole-words = पूरे शब्द

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = पिछला (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = अगला (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = इससे बदलें

# Button on the replace bar that replaces the word found.
text-replace = बदलें

# The words a screen reader gives the Replace button.
text-replace-spoken = मिला हुआ शब्द बदलें

# Button on the replace bar that replaces every word found.
text-replace-all = सब बदलें

# The words a screen reader gives the Replace all button.
text-replace-all-spoken = मिला हुआ हर शब्द बदलें

# Words on the find bar when a find has been asked for and has no match.
text-none-found = कुछ नहीं मिला

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $n } में से { $at }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } मिले

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 बदला गया
       *[other] { $n } बदले गए
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = पंक्ति संख्या लिखें

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 से { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = { $lines } में से

# Button on the go to line bar that goes to the line typed.
text-go = जाएँ

# Words on the go to line bar: the box's label.
text-go-to-line = पंक्ति पर जाएँ

# Button that goes back from a bar, a settings page or a list to the text.
text-back = टेक्स्ट पर वापस

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = अब आपके शब्दों में है

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = शब्दकोश में नहीं है, और उससे मिलता-जुलता कुछ भी नहीं है

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = शब्दकोश में नहीं है। क्या आपका मतलब इनमें से कोई था?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = यह वर्तनी इस्तेमाल करें

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = मेरे शब्दों में जोड़ें

# Heading over the list of suggested spellings.
text-suggestions = सुझाई गई वर्तनियाँ

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = आपकी खोली या सहेजी फ़ाइलें यहाँ दिखती हैं।

# Button that clears the list of recent files.
text-clear-list = सूची साफ़ करें

# Heading of the Text settings page.
text-settings-title = टेक्स्ट सेटिंग्स

# Heading of the group on the settings page for when Text starts.
text-starting = शुरुआत

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = जब टेक्स्ट खुले

# First choice: Text opens the files it had open last time.
text-reopen = पिछली बार की फ़ाइलें फिर खोलें

# Second choice: Text opens with an empty page.
text-start-blank = खाली शुरू करें

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = जो काम किसी फ़ाइल में सहेजा नहीं गया वह हमेशा रखा जाता है और दोनों तरह से वापस आता है।

# Heading of the group on the settings page for how the text looks.
text-showing = टेक्स्ट दिखाना

# Label of the size slider: how big the text is.
text-size = आकार

# Name of the size slider, read out by a screen reader.
text-text-size = टेक्स्ट का आकार

# Heading of the group on the settings page for spelling.
text-spelling = वर्तनी

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = कोई Hunspell शब्दकोश इंस्टॉल नहीं है।

# Label of the list of spelling languages.
text-language = भाषा

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = वर्तनी की भाषा

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = अपने आप ({ $lang })

# Heading of the font list on the settings page.
text-font = फ़ॉन्ट

# The first font in the list: the font Trindle ships with.
text-own-font = Trindle का अपना

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = फ़ाइल को इस रूप में पढ़ें

# Heading over the encodings a file is saved in.
text-save-in = इसमें सहेजें

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = अगली बार सहेजने पर फ़ाइल इसी एन्कोडिंग में लिखी जाएगी।

# Words saying the text has changes that are not saved to its file. $path is the file.
# SAFETY: needs a human review before it ships
text-changes-not-saved = { $name } में ऐसे बदलाव हैं जो उसकी फ़ाइल में सहेजे नहीं गए।

# Words saying the text has changes and no file yet.
text-not-saved = यह टेक्स्ट किसी फ़ाइल में सहेजा नहीं गया है।

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = { $name } सहेजें?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = इसके बाद { $left } और।

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = बिना सहेजे बंद करें

# Button in the question before closing that keeps the tab open.
text-keep-open = खुला रखें

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } एक फ़ोल्डर है।

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } यहाँ खोलने के लिए बहुत बड़ा है।

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } खोला नहीं जा सका: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } लिखा नहीं जा सका: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } सहेजा नहीं जा सका: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } पढ़ा नहीं जा सका: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = { $folder } में काम रखा नहीं जा सका: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = सेटिंग्स सहेजी नहीं जा सकीं: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } आपके शब्दों में जोड़ा नहीं जा सका: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = सूची साफ़ नहीं की जा सकी: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = फ़ाइल चयनकर्ता खोला नहीं जा सका।

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = सेशन बस के बिना फ़ाइलें चुनी नहीं जा सकतीं।

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = { $lang } नाम का कोई शब्दकोश नहीं है।

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = { $lang } शब्दकोश पढ़ा नहीं जा सका: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }। जो इसमें पहले था वह { $path } में रखा है।

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } उस बार सहेजने से पहले जो इसमें था वह { $path } में रखा है।

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = टेक्स्ट के रुकते समय { $name } सहेजा जा रहा था, और सहेजे हुए का जो हिस्सा रखा गया उसे पढ़ा नहीं जा सकता।

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = टेक्स्ट के रुकते समय { $name } सहेजा जा रहा था, और तब से वह बदल चुका है, इसलिए उसे जैसा है वैसा छोड़ दिया गया।

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = टेक्स्ट के रुकते समय { $name } सहेजा जा रहा था, और उसे पहले जैसा वापस नहीं रखा जा सका: { $reason }। उस बार सहेजने से पहले जो इसमें था वह { $path } में रखा है।

# Quiet word beside what was just saved, exported or changed.
text-done = हो गया

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = बिना नाम.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = आपके खोलने के बाद कहीं और बदली गई है।

# Button beside that line: save the words here over the changed file.
text-save-anyway = फिर भी सहेजें

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = फिर से लोड करें

