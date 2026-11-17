
# machine translated (Vietnamese), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Văn bản

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Chưa đặt tên

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Mở

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Mới (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Mở (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Tệp gần đây

# Button in the menu that saves the file as it is.
text-save = Lưu

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Lưu (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Lưu thành

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Lưu thành (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Hoàn tác (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Làm lại (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Tìm (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Tìm và thay thế (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Đi tới dòng (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Xem trước (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Xuất thành trang web

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Xuất thành PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Ngắt dòng dài

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Số dòng

# Button in the menu that opens the Text settings.
text-settings = Cài đặt

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Đóng { $name }

# Tip of the button that closes the open bar.
text-close-bar = Đóng thanh

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Xem trước

# Menu item that copies everything in the text.
text-copy-all = Sao chép toàn bộ văn bản

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Cắt

text-copy = Sao chép

text-paste = Dán

text-select-all = Chọn tất cả

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Kiểm tra chính tả

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = Không thể ghi ‘{ $c }’ bằng { $encoding }. Hãy chọn bảng mã khác từ thanh trạng thái.

# The words on the status bar: the line and column of the caret.
text-line-column = Dòng { $line }, cột { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
       *[other] { $words } từ
    }, { $letters ->
       *[other] { $letters } chữ cái
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
       *[other] { $words } từ
    }, { $letters ->
       *[other] { $letters } chữ cái
    } đã chọn

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Kết thúc dòng kiểu Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Bảng mã: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Văn bản này chưa có tệp.

# Words on the find bar: the find box's label and placeholder.
text-find = Tìm

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Phân biệt hoa thường

# Words on the find bar: a switch to find only whole words.
text-whole-words = Nguyên từ

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Trước (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Sau (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Thay bằng

# Button on the replace bar that replaces the word found.
text-replace = Thay thế

# The words a screen reader gives the Replace button.
text-replace-spoken = Thay từ đã tìm thấy

# Button on the replace bar that replaces every word found.
# SAFETY: needs a human review before it ships
text-replace-all = Thay thế tất cả

# The words a screen reader gives the Replace all button.
# SAFETY: needs a human review before it ships
text-replace-all-spoken = Thay mọi từ đã tìm thấy

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Không có kết quả

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } / { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = Tìm thấy { $n }

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
       *[other] Đã thay { $n }
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Hãy nhập số dòng

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 đến { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = trên { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Đi

# Words on the go to line bar: the box's label.
text-go-to-line = Đi tới dòng

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Quay lại văn bản

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = đã được thêm vào các từ của bạn

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = không có trong từ điển, và không có gợi ý nào

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = không có trong từ điển. Có phải bạn muốn viết một trong các từ này?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Dùng cách viết này

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Thêm vào từ của tôi

# Heading over the list of suggested spellings.
text-suggestions = Cách viết gợi ý

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Tệp bạn mở hoặc lưu sẽ hiện ở đây.

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = Xoá danh sách

# Heading of the Text settings page.
text-settings-title = Cài đặt Văn bản

# Heading of the group on the settings page for when Text starts.
text-starting = Khởi động

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Khi Văn bản mở

# First choice: Text opens the files it had open last time.
text-reopen = Mở lại các tệp trước đó

# Second choice: Text opens with an empty page.
text-start-blank = Bắt đầu mới

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Phần việc chưa lưu của bạn luôn được giữ và khôi phục dù bạn chọn cách nào.

# Heading of the group on the settings page for how the text looks.
text-showing = Giao diện văn bản

# Label of the size slider: how big the text is.
text-size = Cỡ

# Name of the size slider, read out by a screen reader.
text-text-size = Cỡ chữ

# Heading of the group on the settings page for spelling.
text-spelling = Chính tả

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Chưa cài từ điển Hunspell nào.

# Label of the list of spelling languages.
text-language = Ngôn ngữ

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Ngôn ngữ chính tả

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Tự động ({ $lang })

# Heading of the font list on the settings page.
text-font = Phông chữ

# The first font in the list: the font Trindle ships with.
text-own-font = Của riêng Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Đọc tệp dưới dạng

# Heading over the encodings a file is saved in.
text-save-in = Lưu bằng

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = Tệp sẽ dùng bảng mã này vào lần lưu tiếp theo.

# Words saying the text has changes that are not saved to its file. $path is the file.
# SAFETY: needs a human review before it ships
text-changes-not-saved = { $name } có thay đổi chưa được lưu.

# Words saying the text has changes and no file yet.
# SAFETY: needs a human review before it ships
text-not-saved = Văn bản này chưa được lưu vào tệp.

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = Lưu { $name }?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after =
    { $left ->
       *[other] Còn { $left } thẻ nữa cần lưu.
    }

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = Đóng mà không lưu

# Button in the question before closing that keeps the tab open.
# SAFETY: needs a human review before it ships
text-keep-open = Giữ mở

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } là một thư mục.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } quá lớn để mở ở đây.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = Trindle không mở được { $name }: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = Trindle không ghi được { $name }: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = Trindle không lưu được { $name }: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = Không thể đọc { $name }: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Trindle không lưu được phần việc của bạn trong { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Không thể lưu cài đặt: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = Trindle không thêm được { $word } vào từ điển của bạn: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Trindle không xoá được danh sách: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Không thể mở bộ chọn tệp.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Trindle không mở được bộ chọn tệp lúc này.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Không có từ điển nào tên là { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Trindle không đọc được từ điển { $lang }: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. Văn bản cũ được lưu trong { $path }.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } Văn bản từ trước lần lưu đó nằm trong { $path }.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = Văn bản đã đóng khi đang lưu { $name }, và Trindle không đọc được phần đã lưu.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = Văn bản đã đóng khi đang lưu { $name }. Tệp đã thay đổi từ đó, nên Trindle để nguyên như cũ.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = Văn bản đã đóng khi đang lưu { $name }, và Trindle không thể trả tệp về như cũ: { $reason }. Văn bản từ trước lần lưu đó nằm trong { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Xong

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Chưa đặt tên.txt

# The misspelt word, shown in quotation marks on the spelling bar. { $word } is the word.
text-spell-word = ‘{ $word }’



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Tệp này đã thay đổi kể từ khi bạn mở nó.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Cứ lưu

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Tải lại

# Shown when a link in the finished look is pressed and is not a web address, or no browser started.
text-link-not-opened = Ở đây chỉ mở được địa chỉ web.
