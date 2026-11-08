# machine translated (Indonesian), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Teks

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Tanpa judul

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Buka

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Baru (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Buka (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = File terbaru

# Button in the menu that saves the file as it is.
text-save = Simpan

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Simpan (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Simpan sebagai

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Simpan sebagai (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Urungkan (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Ulangi (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Cari (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Cari dan ganti (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Ke baris (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Tampilan jadi (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Ekspor sebagai halaman web

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Ekspor sebagai PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Bungkus baris panjang

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Nomor baris

# Button in the menu that opens the Text settings.
text-settings = Pengaturan

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Tutup { $name }

# Tip of the button that closes the open bar.
text-close-bar = Tutup bilah

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Tampilan jadi

# Menu item that copies everything in the text.
text-copy-all = Salin semua teks

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Potong

text-copy = Salin

text-paste = Tempel

text-select-all = Pilih semua

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Periksa ejaan

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ tidak dapat ditulis dalam { $encoding }. Pilih pengodean lain dari bilah status.

# The words on the status bar: the line and column of the caret.
text-line-column = Baris { $line }, kolom { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
       *[other] { $words } kata
    }, { $letters ->
       *[other] { $letters } huruf
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
       *[other] { $words } kata
    }, { $letters ->
       *[other] { $letters } huruf
    } dipilih

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Akhir baris Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Pengodean: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Teks ini belum memiliki file.

# Words on the find bar: the find box's label and placeholder.
text-find = Cari

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Sesuaikan huruf besar

# Words on the find bar: a switch to find only whole words.
text-whole-words = Kata utuh

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Sebelumnya (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Berikutnya (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Ganti dengan

# Button on the replace bar that replaces the word found.
text-replace = Ganti

# The words a screen reader gives the Replace button.
text-replace-spoken = Ganti kata yang ditemukan

# SAFETY: needs a human review before it ships
# Button on the replace bar that replaces every word found.
text-replace-all = Ganti semua

# SAFETY: needs a human review before it ships
# The words a screen reader gives the Replace all button.
text-replace-all-spoken = Ganti setiap kata yang ditemukan

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Tidak ditemukan

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } dari { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } ditemukan

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
       *[other] { $n } diganti
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Ketik nomor baris

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 sampai { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = dari { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Pergi

# Words on the go to line bar: the box's label.
text-go-to-line = Ke baris

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Kembali ke teks

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = kini menjadi salah satu kata Anda

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = tidak ada di kamus, dan kamus tidak punya yang mirip

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = tidak ada di kamus. Maksud Anda salah satu dari ini?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Gunakan ejaan ini

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Tambahkan ke kata saya

# Heading over the list of suggested spellings.
text-suggestions = Ejaan yang disarankan

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = File yang Anda buka atau simpan muncul di sini.

# SAFETY: needs a human review before it ships
# Button that clears the list of recent files.
text-clear-list = Kosongkan daftar

# Heading of the Text settings page.
text-settings-title = Pengaturan Teks

# Heading of the group on the settings page for when Text starts.
text-starting = Memulai

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Saat Teks dibuka

# First choice: Text opens the files it had open last time.
text-reopen = Buka kembali file terakhir kali

# Second choice: Text opens with an empty page.
text-start-blank = Mulai kosong

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Pekerjaan yang belum disimpan ke file selalu disimpan dan kembali, apa pun pilihannya.

# Heading of the group on the settings page for how the text looks.
text-showing = Tampilan teks

# Label of the size slider: how big the text is.
text-size = Ukuran

# Name of the size slider, read out by a screen reader.
text-text-size = Ukuran teks

# Heading of the group on the settings page for spelling.
text-spelling = Ejaan

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Tidak ada kamus Hunspell yang terpasang.

# Label of the list of spelling languages.
text-language = Bahasa

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Bahasa ejaan

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Otomatis ({ $lang })

# Heading of the font list on the settings page.
text-font = Fon

# The first font in the list: the font Trindle ships with.
text-own-font = Bawaan Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Baca file sebagai

# Heading over the encodings a file is saved in.
text-save-in = Simpan dalam

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = Penyimpanan berikutnya menulis file dalam pengodean ini.

# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } memiliki perubahan yang belum disimpan ke filenya.

# Words saying the text has changes and no file yet.
text-not-saved = Teks ini belum disimpan ke file.

# SAFETY: needs a human review before it ships
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = Simpan { $name }?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = { $left } lagi setelah ini.

# SAFETY: needs a human review before it ships
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = Tutup tanpa menyimpan

# SAFETY: needs a human review before it ships
# Button in the question before closing that keeps the tab open.
text-keep-open = Biarkan terbuka

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } adalah folder.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } terlalu besar untuk dibuka di sini.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } tidak dapat dibuka: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } tidak dapat ditulis: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } tidak dapat disimpan: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } tidak dapat dibaca: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Pekerjaan tidak dapat disimpan di { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Pengaturan tidak dapat disimpan: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } tidak dapat ditambahkan ke kata Anda: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Daftar tidak dapat dikosongkan: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Pemilih file tidak dapat dibuka.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = File tidak dapat dipilih tanpa session bus.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Tidak ada kamus bernama { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Kamus { $lang } tidak dapat dibaca: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. Isinya sebelumnya disimpan di { $path }.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } Isinya sebelum penyimpanan itu disimpan di { $path }.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } sedang disimpan saat Teks dihentikan, dan hasil penyimpanan yang tersisa tidak dapat dibaca.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } sedang disimpan saat Teks dihentikan, dan sejak itu telah berubah, jadi dibiarkan apa adanya.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } sedang disimpan saat Teks dihentikan, dan tidak dapat dikembalikan seperti semula: { $reason }. Isinya sebelum penyimpanan itu disimpan di { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Selesai

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Tanpa judul.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Berubah di tempat lain sejak Anda membukanya.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Tetap simpan

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Muat ulang

