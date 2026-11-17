
# machine translated (Arabic), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = النص

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = بلا عنوان

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = فتح

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = جديد (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = فتح (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = الملفات الأخيرة

# Button in the menu that saves the file as it is.
text-save = حفظ

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = حفظ (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = حفظ باسم

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = حفظ باسم (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = تراجع (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = إعادة (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = بحث (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = بحث واستبدال (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = انتقل إلى سطر (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = معاينة (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = تصدير كصفحة ويب

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = تصدير بصيغة PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = التفاف الأسطر الطويلة

# Button in the menu that shows or hides the line numbers.
text-line-numbers = أرقام الأسطر

# Button in the menu that opens the Text settings.
text-settings = الإعدادات

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = إغلاق { $name }

# Tip of the button that closes the open bar.
text-close-bar = إغلاق الشريط

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = معاينة

# Menu item that copies everything in the text.
text-copy-all = نسخ كل النص

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = قص

text-copy = نسخ

text-paste = لصق

text-select-all = تحديد الكل

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = تدقيق الإملاء

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = لا يمكن كتابة «{ $c }» بترميز { $encoding }. اختر ترميزًا آخر من شريط الحالة.

# The words on the status bar: the line and column of the caret.
text-line-column = السطر { $line }، العمود { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [zero] لا كلمات
        [one] كلمة واحدة
        [two] كلمتان
        [few] { $words } كلمات
        [many] { $words } كلمة
       *[other] { $words } كلمة
    }، { $letters ->
        [zero] لا أحرف
        [one] حرف واحد
        [two] حرفان
        [few] { $letters } أحرف
        [many] { $letters } حرفًا
       *[other] { $letters } حرف
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [zero] لا كلمات
        [one] كلمة واحدة
        [two] كلمتان
        [few] { $words } كلمات
        [many] { $words } كلمة
       *[other] { $words } كلمة
    }، { $letters ->
        [zero] لا أحرف
        [one] حرف واحد
        [two] حرفان
        [few] { $letters } أحرف
        [many] { $letters } حرفًا
       *[other] { $letters } حرف
    } محددة

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = نهايات أسطر Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = الترميز: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = ليس لهذا النص ملف بعد.

# Words on the find bar: the find box's label and placeholder.
text-find = بحث

# Words on the find bar: a switch to find only the same capitals.
text-match-case = مطابقة حالة الأحرف

# Words on the find bar: a switch to find only whole words.
text-whole-words = كلمات كاملة

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = السابق (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = التالي (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = استبدال بـ

# Button on the replace bar that replaces the word found.
text-replace = استبدال

# The words a screen reader gives the Replace button.
text-replace-spoken = استبدال الكلمة التي وُجدت

# Button on the replace bar that replaces every word found.
text-replace-all = استبدال الكل

# The words a screen reader gives the Replace all button.
text-replace-all-spoken = استبدال كل كلمة وُجدت

# Words on the find bar when a find has been asked for and has no match.
text-none-found = لا نتائج

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } من { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = وُجد { $n }

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [zero] لم يُستبدل شيء
        [one] استُبدل واحد
        [two] استُبدل اثنان
        [few] استُبدلت { $n }
        [many] استُبدل { $n }
       *[other] استُبدل { $n }
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = اكتب رقم سطر

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = من 1 إلى { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = من { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = انتقال

# Words on the go to line bar: the box's label.
text-go-to-line = انتقل إلى سطر

# Button that goes back from a bar, a settings page or a list to the text.
text-back = العودة إلى النص

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = أُضيفت إلى كلماتك

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = ليست في القاموس، ولا توجد اقتراحات

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = ليست في القاموس. هل تقصد إحدى هذه؟

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = استخدم هذا الإملاء

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = أضف إلى كلماتي

# Heading over the list of suggested spellings.
text-suggestions = إملاءات مقترحة

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = تظهر هنا الملفات التي تفتحها أو تحفظها.

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = امسح القائمة

# Heading of the Text settings page.
text-settings-title = إعدادات النص

# Heading of the group on the settings page for when Text starts.
text-starting = البدء

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = عند فتح النص

# First choice: Text opens the files it had open last time.
text-reopen = أعد فتح الملفات السابقة

# Second choice: Text opens with an empty page.
text-start-blank = ابدأ من جديد

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = يبقى عملك غير المحفوظ ويُسترجع أيًّا كان الخيار الذي تختاره.

# Heading of the group on the settings page for how the text looks.
text-showing = مظهر النص

# Label of the size slider: how big the text is.
text-size = الحجم

# Name of the size slider, read out by a screen reader.
text-text-size = حجم النص

# Heading of the group on the settings page for spelling.
text-spelling = الإملاء

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = لا توجد قواميس Hunspell مثبّتة.

# Label of the list of spelling languages.
text-language = اللغة

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = لغة الإملاء

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = تلقائي ({ $lang })

# Heading of the font list on the settings page.
text-font = الخط

# The first font in the list: the font Trindle ships with.
text-own-font = خط Trindle الخاص

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = اقرأ الملف بوصفه

# Heading over the encodings a file is saved in.
text-save-in = احفظ بترميز

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = سيستخدم الملف هذا الترميز في المرة القادمة التي تحفظه فيها.

# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = في { $name } تغييرات غير محفوظة.

# Words saying the text has changes and no file yet.
text-not-saved = هذا النص غير محفوظ في ملف.

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = هل تحفظ { $name }؟

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after =
    { $left ->
        [one] علامة تبويب أخرى للحفظ.
        [two] علامتا تبويب أخريان للحفظ.
        [few] { $left } علامات تبويب أخرى للحفظ.
        [many] { $left } علامة تبويب أخرى للحفظ.
        [zero] { $left } علامة تبويب أخرى للحفظ.
       *[other] { $left } علامة تبويب أخرى للحفظ.
    }

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = إغلاق دون حفظ

# Button in the question before closing that keeps the tab open.
text-keep-open = أبقِه مفتوحًا

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } مجلد.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } أكبر من أن يُفتح هنا.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = تعذّر على Trindle فتح { $name }: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = تعذّر على Trindle الكتابة في { $name }: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = تعذّر على Trindle حفظ { $name }: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = تعذّرت قراءة { $name }: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = تعذّر على Trindle حفظ عملك في { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = تعذّر حفظ الإعدادات: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = تعذّر على Trindle إضافة { $word } إلى قاموسك: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = تعذّر على Trindle مسح القائمة: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = تعذّر فتح منتقي الملفات.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = لا يستطيع Trindle فتح منتقي الملفات الآن.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = لا يوجد قاموس باسم { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = تعذّر على Trindle قراءة قاموس { $lang }: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. النص القديم محفوظ في { $path }.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } النص الذي كان قبل ذلك الحفظ موجود في { $path }.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = أُغلق النص أثناء حفظ { $name }، وتعذّر على Trindle قراءة الجزء الذي حُفظ.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = أُغلق النص أثناء حفظ { $name }. وقد تغيّر الملف منذ ذلك الحين، لذلك تركه Trindle كما هو.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = أُغلق النص أثناء حفظ { $name }، وتعذّر على Trindle إرجاع الملف كما كان: { $reason }. النص الذي كان قبل ذلك الحفظ في { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = تم

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = بلا عنوان.txt

# The misspelt word, shown in quotation marks on the spelling bar. { $word } is the word.
text-spell-word = ‘{ $word }’



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = تغيّر هذا الملف منذ فتحته.

# Button beside that line: save the words here over the changed file.
text-save-anyway = احفظ على أي حال

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = إعادة التحميل

text-link-not-opened = لا تُفتح من هنا إلا عناوين الويب.
