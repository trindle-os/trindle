# machine translated (Russian), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Текст

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Без названия

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Открыть

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Создать (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Открыть (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Недавние файлы

# Button in the menu that saves the file as it is.
text-save = Сохранить

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Сохранить (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Сохранить как

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Сохранить как (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Отменить (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Повторить (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Найти (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Найти и заменить (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Перейти к строке (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Готовый вид (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Экспортировать как веб-страницу

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Экспортировать в PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Переносить длинные строки

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Номера строк

# Button in the menu that opens the Text settings.
text-settings = Параметры

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Закрыть { $name }

# Tip of the button that closes the open bar.
text-close-bar = Закрыть панель

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Готовый вид

# Menu item that copies everything in the text.
text-copy-all = Скопировать весь текст

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Вырезать

text-copy = Копировать

text-paste = Вставить

text-select-all = Выделить всё

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Проверять орфографию

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = Символ «{ $c }» нельзя записать в кодировке { $encoding }. Выберите другую кодировку в строке состояния.

# The words on the status bar: the line and column of the caret.
text-line-column = Строка { $line }, столбец { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } слово
        [few] { $words } слова
        [many] { $words } слов
       *[other] { $words } слова
    }, { $letters ->
        [one] { $letters } буква
        [few] { $letters } буквы
        [many] { $letters } букв
       *[other] { $letters } буквы
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } слово
        [few] { $words } слова
        [many] { $words } слов
       *[other] { $words } слова
    }, { $letters ->
        [one] { $letters } буква
        [few] { $letters } буквы
        [many] { $letters } букв
       *[other] { $letters } буквы
    } выделено

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Концы строк Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Кодировка: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = У этого текста пока нет файла.

# Words on the find bar: the find box's label and placeholder.
text-find = Найти

# Words on the find bar: a switch to find only the same capitals.
text-match-case = С учётом регистра

# Words on the find bar: a switch to find only whole words.
text-whole-words = Слова целиком

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Предыдущее (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Следующее (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Заменить на

# Button on the replace bar that replaces the word found.
text-replace = Заменить

# The words a screen reader gives the Replace button.
text-replace-spoken = Заменить найденное слово

# Button on the replace bar that replaces every word found.
text-replace-all = Заменить все

# The words a screen reader gives the Replace all button.
text-replace-all-spoken = Заменить каждое найденное слово

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Не найдено

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } из { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = Найдено: { $n }

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] Заменена { $n }
        [few] Заменено { $n }
        [many] Заменено { $n }
       *[other] Заменено { $n }
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Введите номер строки

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = от 1 до { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = из { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Перейти

# Words on the go to line bar: the box's label.
text-go-to-line = Перейти к строке

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Назад к тексту

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = теперь входит в ваши слова

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = нет в словаре, там нет ничего похожего

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = нет в словаре. Вы имели в виду одно из этих?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Использовать это написание

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Добавить в мои слова

# Heading over the list of suggested spellings.
text-suggestions = Предлагаемые написания

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Здесь будут файлы, которые вы открываете или сохраняете.

# SAFETY: needs a human review before it ships
# Button that clears the list of recent files.
text-clear-list = Очистить список

# Heading of the Text settings page.
text-settings-title = Параметры «Текста»

# Heading of the group on the settings page for when Text starts.
text-starting = Запуск

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = При открытии «Текста»

# First choice: Text opens the files it had open last time.
text-reopen = Открыть файлы прошлого раза

# Second choice: Text opens with an empty page.
text-start-blank = Начать с пустой страницы

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Работа, не сохранённая в файл, всегда сохраняется и возвращается при любом выборе.

# Heading of the group on the settings page for how the text looks.
text-showing = Отображение текста

# Label of the size slider: how big the text is.
text-size = Размер

# Name of the size slider, read out by a screen reader.
text-text-size = Размер текста

# Heading of the group on the settings page for spelling.
text-spelling = Орфография

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Словари Hunspell не установлены.

# Label of the list of spelling languages.
text-language = Язык

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Язык орфографии

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Автоматически ({ $lang })

# Heading of the font list on the settings page.
text-font = Шрифт

# The first font in the list: the font Trindle ships with.
text-own-font = Собственный шрифт Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Читать файл как

# Heading over the encodings a file is saved in.
text-save-in = Сохранять в

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = При следующем сохранении файл будет записан в этой кодировке.

# SAFETY: needs a human review before it ships
# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = В { $name } есть изменения, не сохранённые в файл.

# SAFETY: needs a human review before it ships
# Words saying the text has changes and no file yet.
text-not-saved = Этот текст не сохранён в файл.

# SAFETY: needs a human review before it ships
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = Сохранить { $name }?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = Ещё после этой: { $left }.

# SAFETY: needs a human review before it ships
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = Закрыть без сохранения

# Button in the question before closing that keeps the tab open.
text-keep-open = Оставить открытой

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } является папкой.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } слишком велик, чтобы открыть его здесь.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = Не удалось открыть { $name }: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = Не удалось записать { $name }: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = Не удалось сохранить { $name }: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = Не удалось прочитать { $name }: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Не удалось сохранить работу в { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Не удалось сохранить параметры: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = Не удалось добавить { $word } в ваши слова: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Не удалось очистить список: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Не удалось открыть окно выбора файла.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Без шины сеанса выбрать файлы нельзя.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Словаря «{ $lang }» нет.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Не удалось прочитать словарь «{ $lang }»: { $reason }

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. То, что было в файле раньше, сохранено в { $path }.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } То, что было в файле до этого сохранения, сохранено в { $path }.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } сохранялся, когда «Текст» был остановлен, и сохранённое от этого сохранения нельзя прочитать.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } сохранялся, когда «Текст» был остановлен, и с тех пор изменился, поэтому оставлен как есть.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } сохранялся, когда «Текст» был остановлен, и его не удалось вернуть в прежний вид: { $reason }. То, что было в файле до этого сохранения, сохранено в { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Готово

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Без имени.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Изменено в другом месте после того, как вы это открыли.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Всё равно сохранить

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Загрузить заново

