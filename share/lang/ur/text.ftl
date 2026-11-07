
# machine translated (Urdu), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = متن

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = بے نام

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = کھولیں

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = نیا (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = کھولیں (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = حالیہ فائلیں

# Button in the menu that saves the file as it is.
text-save = محفوظ کریں

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = محفوظ کریں (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = اس نام سے محفوظ کریں

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = اس نام سے محفوظ کریں (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = واپس (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = دوبارہ (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = تلاش (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = تلاش اور تبدیلی (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = سطر پر جائیں (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = مکمل صورت (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = ویب صفحے کے طور پر برآمد کریں

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = PDF کے طور پر برآمد کریں

# Button in the menu that turns on wrapping of long lines.
text-wrap = لمبی سطریں لپیٹیں

# Button in the menu that shows or hides the line numbers.
text-line-numbers = سطر کے نمبر

# Button in the menu that opens the Text settings.
text-settings = ترتیبات

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = { $name } بند کریں

# Tip of the button that closes the open bar.
text-close-bar = پٹی بند کریں

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = مکمل صورت

# Menu item that copies everything in the text.
text-copy-all = سارا متن کاپی کریں

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = کاٹیں

text-copy = کاپی کریں

text-paste = چسپاں کریں

text-select-all = سب منتخب کریں

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = ہجے جانچیں

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ { $encoding } میں لکھا نہیں جا سکتا۔ حالت کی پٹی سے دوسری انکوڈنگ چنیں۔

# The words on the status bar: the line and column of the caret.
text-line-column = سطر { $line }، کالم { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } لفظ
       *[other] { $words } الفاظ
    }، { $letters ->
        [one] { $letters } حرف
       *[other] { $letters } حروف
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } لفظ
       *[other] { $words } الفاظ
    }، { $letters ->
        [one] { $letters } حرف
       *[other] { $letters } حروف
    } منتخب

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows سطر کے اختتام

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = انکوڈنگ: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = اس متن کی ابھی کوئی فائل نہیں۔

# Words on the find bar: the find box's label and placeholder.
text-find = تلاش

# Words on the find bar: a switch to find only the same capitals.
text-match-case = بڑے چھوٹے حروف کا لحاظ

# Words on the find bar: a switch to find only whole words.
text-whole-words = پورے الفاظ

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = پچھلا (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = اگلا (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = اس سے بدلیں

# Button on the replace bar that replaces the word found.
text-replace = بدلیں

# The words a screen reader gives the Replace button.
text-replace-spoken = ملا ہوا لفظ بدلیں

# Button on the replace bar that replaces every word found.
text-replace-all = سب بدلیں

# The words a screen reader gives the Replace all button.
text-replace-all-spoken = ہر ملا ہوا لفظ بدلیں

# Words on the find bar when a find has been asked for and has no match.
text-none-found = کچھ نہیں ملا

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } از { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } ملے

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 بدلا گیا
       *[other] { $n } بدلے گئے
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = سطر کا نمبر لکھیں

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 سے { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = از { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = جائیں

# Words on the go to line bar: the box's label.
text-go-to-line = سطر پر جائیں

# Button that goes back from a bar, a settings page or a list to the text.
text-back = متن پر واپس

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = اب آپ کے الفاظ میں شامل ہے

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = لغت میں نہیں ہے، اور اس سے ملتا جلتا کچھ نہیں

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = لغت میں نہیں ہے۔ کیا آپ کا مطلب ان میں سے کوئی تھا؟

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = یہ ہجے استعمال کریں

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = میرے الفاظ میں شامل کریں

# Heading over the list of suggested spellings.
text-suggestions = تجویز کردہ ہجے

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = جو فائلیں آپ کھولیں یا محفوظ کریں وہ یہاں دکھائی دیتی ہیں۔

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = فہرست صاف کریں

# Heading of the Text settings page.
text-settings-title = متن کی ترتیبات

# Heading of the group on the settings page for when Text starts.
text-starting = آغاز

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = جب متن کھلے

# First choice: Text opens the files it had open last time.
text-reopen = پچھلی بار کی فائلیں دوبارہ کھولیں

# Second choice: Text opens with an empty page.
text-start-blank = خالی شروع کریں

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = جو کام فائل میں محفوظ نہ ہو وہ ہمیشہ رکھا جاتا ہے اور دونوں صورتوں میں واپس آ جاتا ہے۔

# Heading of the group on the settings page for how the text looks.
text-showing = متن دکھانا

# Label of the size slider: how big the text is.
text-size = سائز

# Name of the size slider, read out by a screen reader.
text-text-size = متن کا سائز

# Heading of the group on the settings page for spelling.
text-spelling = ہجے

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = کوئی Hunspell لغت انسٹال نہیں۔

# Label of the list of spelling languages.
text-language = زبان

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = ہجے کی زبان

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = خودکار ({ $lang })

# Heading of the font list on the settings page.
text-font = فونٹ

# The first font in the list: the font Trindle ships with.
text-own-font = Trindle کا اپنا

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = فائل کو اس طرح پڑھیں

# Heading over the encodings a file is saved in.
text-save-in = اس میں محفوظ کریں

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = اگلی بار محفوظ کرنے پر فائل اسی انکوڈنگ میں لکھی جاتی ہے۔

# Words saying the text has changes that are not saved to its file. $path is the file.
# SAFETY: needs a human review before it ships
text-changes-not-saved = { $name } میں ایسی تبدیلیاں ہیں جو اس کی فائل میں محفوظ نہیں ہوئیں۔

# Words saying the text has changes and no file yet.
# SAFETY: needs a human review before it ships
text-not-saved = یہ متن کسی فائل میں محفوظ نہیں ہوا۔

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = { $name } محفوظ کریں؟

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = اس کے بعد { $left } اور۔

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = بغیر محفوظ کیے بند کریں

# Button in the question before closing that keeps the tab open.
# SAFETY: needs a human review before it ships
text-keep-open = کھلا رہنے دیں

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } ایک فولڈر ہے۔

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } یہاں کھولنے کے لیے بہت بڑا ہے۔

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } کھل نہیں سکا: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } لکھا نہیں جا سکا: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } محفوظ نہیں ہو سکا: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } پڑھا نہیں جا سکا: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = { $folder } میں کام رکھا نہیں جا سکا: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = ترتیبات محفوظ نہیں ہو سکیں: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } آپ کے الفاظ میں شامل نہیں ہو سکا: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = فہرست صاف نہیں ہو سکی: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = فائل منتخب کرنے والی ونڈو کھل نہیں سکی۔

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = سیشن بس کے بغیر فائلیں منتخب نہیں ہو سکتیں۔

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = { $lang } نام کی کوئی لغت نہیں۔

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = { $lang } لغت پڑھی نہیں جا سکی: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }۔ جو اس میں پہلے تھا وہ { $path } میں رکھا ہے۔

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } جو اس محفوظ کرنے سے پہلے اس میں تھا وہ { $path } میں رکھا ہے۔

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } محفوظ ہو رہا تھا جب متن روک دیا گیا، اور محفوظ کرنے کا جو حصہ رکھا گیا وہ پڑھا نہیں جا سکتا۔

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } محفوظ ہو رہا تھا جب متن روک دیا گیا، اور اس کے بعد بدل چکا ہے، اس لیے اسے جیسا ہے ویسا چھوڑ دیا گیا۔

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } محفوظ ہو رہا تھا جب متن روک دیا گیا، اور اسے پہلے جیسا واپس رکھا نہیں جا سکا: { $reason }۔ جو اس محفوظ کرنے سے پہلے اس میں تھا وہ { $path } میں رکھا ہے۔

# Quiet word beside what was just saved, exported or changed.
text-done = ہو گیا

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = بے نام.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = آپ کے کھولنے کے بعد کہیں اور بدل گئی ہے۔

# Button beside that line: save the words here over the changed file.
text-save-anyway = پھر بھی محفوظ کریں

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = دوبارہ لوڈ کریں

