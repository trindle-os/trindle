
# machine translated (Simplified Chinese), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = 文本

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = 未命名

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = 打开

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = 新建 (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = 打开 (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = 最近的文件

# Button in the menu that saves the file as it is.
text-save = 保存

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = 保存 (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = 另存为

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = 另存为 (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = 撤销 (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = 重做 (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = 查找 (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = 查找和替换 (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = 转到行 (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = 成品效果 (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = 导出为网页

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = 导出为 PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = 长行自动换行

# Button in the menu that shows or hides the line numbers.
text-line-numbers = 行号

# Button in the menu that opens the Text settings.
text-settings = 设置

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = 关闭 { $name }

# Tip of the button that closes the open bar.
text-close-bar = 关闭此栏

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = 成品效果

# Menu item that copies everything in the text.
text-copy-all = 复制全部文本

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = 剪切

text-copy = 复制

text-paste = 粘贴

text-select-all = 全选

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = 检查拼写

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = 无法用 { $encoding } 写入“{ $c }”。请在状态栏中选择其他编码。

# The words on the status bar: the line and column of the caret.
text-line-column = 第 { $line } 行，第 { $col } 列

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
       *[other] { $words } 个词
    }，{ $letters ->
       *[other] { $letters } 个字符
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = 已选择 { $words ->
       *[other] { $words } 个词
    }，{ $letters ->
       *[other] { $letters } 个字符
    }

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows 换行符

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = 编码：{ $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = 这段文本还没有文件。

# Words on the find bar: the find box's label and placeholder.
text-find = 查找

# Words on the find bar: a switch to find only the same capitals.
text-match-case = 区分大小写

# Words on the find bar: a switch to find only whole words.
text-whole-words = 全词匹配

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = 上一个 (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = 下一个 (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = 替换为

# Button on the replace bar that replaces the word found.
text-replace = 替换

# The words a screen reader gives the Replace button.
text-replace-spoken = 替换找到的词

# Button on the replace bar that replaces every word found.
text-replace-all = 全部替换

# The words a screen reader gives the Replace all button.
text-replace-all-spoken = 替换所有找到的词

# Words on the find bar when a find has been asked for and has no match.
text-none-found = 未找到

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = 第 { $at } 个，共 { $n } 个

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = 找到 { $n } 个

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
       *[other] 已替换 { $n } 处
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = 请输入行号

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 到 { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = 共 { $lines } 行

# Button on the go to line bar that goes to the line typed.
text-go = 转到

# Words on the go to line bar: the box's label.
text-go-to-line = 转到行

# Button that goes back from a bar, a settings page or a list to the text.
text-back = 返回文本

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = 已成为你的词汇之一

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = 不在词典中，词典里也没有相近的词

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = 不在词典中。你是不是想写下面的某个词？

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = 使用此拼写

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = 添加到我的词汇

# Heading over the list of suggested spellings.
text-suggestions = 建议的拼写

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = 你打开或保存的文件会显示在这里。

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = 清空列表

# Heading of the Text settings page.
text-settings-title = 文本设置

# Heading of the group on the settings page for when Text starts.
text-starting = 启动

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = 打开文本时

# First choice: Text opens the files it had open last time.
text-reopen = 重新打开上次的文件

# Second choice: Text opens with an empty page.
text-start-blank = 从空白开始

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = 未保存到文件的内容总会保留，无论选哪一项都会恢复。

# Heading of the group on the settings page for how the text looks.
text-showing = 文本显示

# Label of the size slider: how big the text is.
text-size = 大小

# Name of the size slider, read out by a screen reader.
text-text-size = 文字大小

# Heading of the group on the settings page for spelling.
text-spelling = 拼写

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = 没有安装 Hunspell 词典。

# Label of the list of spelling languages.
text-language = 语言

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = 拼写语言

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = 自动（{ $lang }）

# Heading of the font list on the settings page.
text-font = 字体

# The first font in the list: the font Trindle ships with.
text-own-font = Trindle 自带字体

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = 按以下编码读取文件

# Heading over the encodings a file is saved in.
text-save-in = 保存编码

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = 下次保存时会用此编码写入文件。

# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } 有更改尚未保存到文件。

# Words saying the text has changes and no file yet.
text-not-saved = 这段文本尚未保存到文件。

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = 保存 { $name }？

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = 此后还有 { $left } 个。

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = 不保存并关闭

# Button in the question before closing that keeps the tab open.
text-keep-open = 保持打开

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } 是一个文件夹。

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } 太大，无法在这里打开。

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = 无法打开 { $name }：{ $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = 无法写入 { $name }：{ $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = 无法保存 { $name }：{ $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = 无法读取 { $name }：{ $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = 无法将工作内容保留在 { $folder }：{ $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = 无法保存设置：{ $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = 无法将 { $word } 添加到你的词汇：{ $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = 无法清空列表：{ $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = 无法打开文件选择器。

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = 没有会话总线，无法选择文件。

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = 没有名为 { $lang } 的词典。

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = 无法读取 { $lang } 词典：{ $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }。之前的内容保存在 { $path }。

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } 此次保存之前的内容保存在 { $path }。

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = 保存 { $name } 时文本程序被中止，已保留的保存内容无法读取。

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = 保存 { $name } 时文本程序被中止，之后该文件又被更改，所以保持原样。

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = 保存 { $name } 时文本程序被中止，无法恢复原样：{ $reason }。此次保存之前的内容保存在 { $path }。

# Quiet word beside what was just saved, exported or changed.
text-done = 完成

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = 未命名.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = 自你打开后，它已在别处被更改。

# Button beside that line: save the words here over the changed file.
text-save-anyway = 仍然保存

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = 重新载入

