
# machine translated (Bengali), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = লেখা

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = নামহীন

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = খুলুন

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = নতুন (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = খুলুন (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = সাম্প্রতিক ফাইল

# Button in the menu that saves the file as it is.
text-save = সংরক্ষণ করুন

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = সংরক্ষণ করুন (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = নতুন নামে সংরক্ষণ করুন

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = নতুন নামে সংরক্ষণ করুন (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = পূর্বাবস্থা (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = আবার করুন (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = খুঁজুন (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = খুঁজে বদলান (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = লাইনে যান (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = সম্পূর্ণ রূপ (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = ওয়েব পাতা হিসেবে রপ্তানি করুন

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = PDF হিসেবে রপ্তানি করুন

# Button in the menu that turns on wrapping of long lines.
text-wrap = লম্বা লাইন ভেঙে দেখান

# Button in the menu that shows or hides the line numbers.
text-line-numbers = লাইন নম্বর

# Button in the menu that opens the Text settings.
text-settings = সেটিংস

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = { $name } বন্ধ করুন

# Tip of the button that closes the open bar.
text-close-bar = বারটি বন্ধ করুন

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = সম্পূর্ণ রূপ

# Menu item that copies everything in the text.
text-copy-all = পুরো লেখা কপি করুন

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = কাটুন

text-copy = কপি করুন

text-paste = পেস্ট করুন

text-select-all = সব বাছাই করুন

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = বানান দেখুন

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ { $encoding }-এ লেখা যায় না। স্ট্যাটাস বার থেকে অন্য এনকোডিং বেছে নিন।

# The words on the status bar: the line and column of the caret.
text-line-column = লাইন { $line }, কলাম { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words }টি শব্দ
       *[other] { $words }টি শব্দ
    }, { $letters ->
        [one] { $letters }টি অক্ষর
       *[other] { $letters }টি অক্ষর
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words }টি শব্দ
       *[other] { $words }টি শব্দ
    }, { $letters ->
        [one] { $letters }টি অক্ষর
       *[other] { $letters }টি অক্ষর
    } বাছাই করা

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows লাইন শেষ

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = এনকোডিং: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = এই লেখার এখনও কোনো ফাইল নেই।

# Words on the find bar: the find box's label and placeholder.
text-find = খুঁজুন

# Words on the find bar: a switch to find only the same capitals.
text-match-case = বড় ও ছোট হাতের অক্ষর মেলান

# Words on the find bar: a switch to find only whole words.
text-whole-words = পুরো শব্দ

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = আগেরটি (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = পরেরটি (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = এর বদলে বসান

# Button on the replace bar that replaces the word found.
text-replace = বদলান

# The words a screen reader gives the Replace button.
text-replace-spoken = পাওয়া শব্দটি বদলান

# Button on the replace bar that replaces every word found.
# SAFETY: needs a human review before it ships
text-replace-all = সব বদলান

# The words a screen reader gives the Replace all button.
# SAFETY: needs a human review before it ships
text-replace-all-spoken = পাওয়া প্রতিটি শব্দ বদলান

# Words on the find bar when a find has been asked for and has no match.
text-none-found = কিছু পাওয়া যায়নি

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $n }-এর মধ্যে { $at }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n }টি পাওয়া গেছে

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] ১টি বদলানো হয়েছে
       *[other] { $n }টি বদলানো হয়েছে
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = একটি লাইন নম্বর লিখুন

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = ১ থেকে { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = মোট { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = যান

# Words on the go to line bar: the box's label.
text-go-to-line = লাইনে যান

# Button that goes back from a bar, a settings page or a list to the text.
text-back = লেখায় ফিরে যান

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = এখন আপনার শব্দগুলোর একটি

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = অভিধানে নেই, আর অভিধানে এর কাছাকাছি কিছুও নেই

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = অভিধানে নেই। আপনি কি এগুলোর একটি বোঝাতে চেয়েছেন?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = এই বানান ব্যবহার করুন

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = আমার শব্দে যোগ করুন

# Heading over the list of suggested spellings.
text-suggestions = প্রস্তাবিত বানান

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = আপনি যে ফাইল খোলেন বা সংরক্ষণ করেন তা এখানে দেখা যায়।

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = তালিকা মুছে ফেলুন

# Heading of the Text settings page.
text-settings-title = লেখার সেটিংস

# Heading of the group on the settings page for when Text starts.
text-starting = শুরু

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = লেখা খুললে

# First choice: Text opens the files it had open last time.
text-reopen = গতবারের ফাইলগুলো আবার খুলুন

# Second choice: Text opens with an empty page.
text-start-blank = খালি পাতায় শুরু করুন

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = যে কাজ কোনো ফাইলে সংরক্ষিত নয় তা সবসময় রাখা হয় এবং যে পছন্দই হোক ফিরে আসে।

# Heading of the group on the settings page for how the text looks.
text-showing = লেখা দেখানো

# Label of the size slider: how big the text is.
text-size = আকার

# Name of the size slider, read out by a screen reader.
text-text-size = লেখার আকার

# Heading of the group on the settings page for spelling.
text-spelling = বানান

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = কোনো Hunspell অভিধান ইনস্টল করা নেই।

# Label of the list of spelling languages.
text-language = ভাষা

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = বানানের ভাষা

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = স্বয়ংক্রিয় ({ $lang })

# Heading of the font list on the settings page.
text-font = ফন্ট

# The first font in the list: the font Trindle ships with.
text-own-font = Trindle-এর নিজস্ব

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = ফাইলটি এভাবে পড়ুন

# Heading over the encodings a file is saved in.
text-save-in = এতে সংরক্ষণ করুন

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = পরের সংরক্ষণে ফাইলটি এই এনকোডিংয়ে লেখা হবে।

# Words saying the text has changes that are not saved to its file. $path is the file.
# SAFETY: needs a human review before it ships
text-changes-not-saved = { $name }-এর এমন পরিবর্তন আছে যা এর ফাইলে সংরক্ষিত হয়নি।

# Words saying the text has changes and no file yet.
# SAFETY: needs a human review before it ships
text-not-saved = এই লেখা কোনো ফাইলে সংরক্ষিত হয়নি।

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = { $name } সংরক্ষণ করবেন?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = এটির পরে আরও { $left }টি।

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = সংরক্ষণ না করে বন্ধ করুন

# Button in the question before closing that keeps the tab open.
# SAFETY: needs a human review before it ships
text-keep-open = খোলা রাখুন

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } একটি ফোল্ডার।

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } এখানে খোলার জন্য খুব বড়।

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } খোলা যায়নি: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } লেখা যায়নি: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } সংরক্ষণ করা যায়নি: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } পড়া যায়নি: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = { $folder }-এ কাজ রাখা যায়নি: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = সেটিংস সংরক্ষণ করা যায়নি: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } আপনার শব্দে যোগ করা যায়নি: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = তালিকা মোছা যায়নি: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = ফাইল বাছাইয়ের উইন্ডো খোলা যায়নি।

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = সেশন বাস ছাড়া ফাইল বাছাই করা যায় না।

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = { $lang } নামে কোনো অভিধান নেই।

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = { $lang } অভিধান পড়া যায়নি: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }। আগে এতে যা ছিল তা { $path }-এ রাখা আছে।

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } ওই সংরক্ষণের আগে এতে যা ছিল তা { $path }-এ রাখা আছে।

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } সংরক্ষণ হওয়ার সময় লেখা বন্ধ হয়ে গিয়েছিল, আর সংরক্ষণের যা রাখা হয়েছিল তা পড়া যায় না।

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } সংরক্ষণ হওয়ার সময় লেখা বন্ধ হয়ে গিয়েছিল, আর এরপর এটি বদলে গেছে, তাই যেমন আছে তেমনই রাখা হয়েছে।

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } সংরক্ষণ হওয়ার সময় লেখা বন্ধ হয়ে গিয়েছিল, আর আগের অবস্থায় ফেরানো যায়নি: { $reason }। ওই সংরক্ষণের আগে এতে যা ছিল তা { $path }-এ রাখা আছে।

# Quiet word beside what was just saved, exported or changed.
text-done = হয়ে গেছে

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = নামহীন.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = খোলার পর অন্য জায়গায় বদলেছে।

# Button beside that line: save the words here over the changed file.
text-save-anyway = তবুও সংরক্ষণ করুন

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = আবার লোড করুন

