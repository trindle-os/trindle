
# machine translated (Turkish), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Metin

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Adsız

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Aç

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Yeni (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Aç (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Son dosyalar

# Button in the menu that saves the file as it is.
text-save = Kaydet

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Kaydet (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Farklı kaydet

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Farklı kaydet (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Geri al (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Yinele (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Bul (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Bul ve değiştir (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Satıra git (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Bitmiş görünüm (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Web sayfası olarak dışa aktar

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = PDF olarak dışa aktar

# Button in the menu that turns on wrapping of long lines.
text-wrap = Uzun satırları kaydır

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Satır numaraları

# Button in the menu that opens the Text settings.
text-settings = Ayarlar

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Kapat: { $name }

# Tip of the button that closes the open bar.
text-close-bar = Çubuğu kapat

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Bitmiş görünüm

# Menu item that copies everything in the text.
text-copy-all = Tüm metni kopyala

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Kes

text-copy = Kopyala

text-paste = Yapıştır

text-select-all = Tümünü seç

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Yazımı denetle

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ karakteri { $encoding } kodlamasında yazılamaz. Durum çubuğundan başka bir kodlama seçin.

# The words on the status bar: the line and column of the caret.
text-line-column = Satır { $line }, sütun { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } sözcük
       *[other] { $words } sözcük
    }, { $letters ->
        [one] { $letters } harf
       *[other] { $letters } harf
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } sözcük
       *[other] { $words } sözcük
    }, { $letters ->
        [one] { $letters } harf
       *[other] { $letters } harf
    } seçili

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows satır sonları

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Kodlama: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Bu metnin henüz dosyası yok.

# Words on the find bar: the find box's label and placeholder.
text-find = Bul

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Büyük/küçük harf eşleşsin

# Words on the find bar: a switch to find only whole words.
text-whole-words = Tam sözcükler

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Önceki (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Sonraki (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Şununla değiştir

# Button on the replace bar that replaces the word found.
text-replace = Değiştir

# The words a screen reader gives the Replace button.
text-replace-spoken = Bulunan sözcüğü değiştir

# Button on the replace bar that replaces every word found.
# SAFETY: needs a human review before it ships
text-replace-all = Tümünü değiştir

# The words a screen reader gives the Replace all button.
# SAFETY: needs a human review before it ships
text-replace-all-spoken = Bulunan her sözcüğü değiştir

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Bulunamadı

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } / { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } bulundu

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 değiştirildi
       *[other] { $n } değiştirildi
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Bir satır numarası yazın

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 - { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = / { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Git

# Words on the go to line bar: the box's label.
text-go-to-line = Satıra git

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Metne dön

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = artık sözcüklerinizden biri

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = sözlükte yok ve sözlükte buna benzer bir şey de yok

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = sözlükte yok. Şunlardan birini mi demek istediniz?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Bu yazımı kullan

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Sözcüklerime ekle

# Heading over the list of suggested spellings.
text-suggestions = Önerilen yazımlar

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Açtığınız ya da kaydettiğiniz dosyalar burada görünür.

# Button that clears the list of recent files.
# SAFETY: needs a human review before it ships
text-clear-list = Listeyi temizle

# Heading of the Text settings page.
text-settings-title = Metin ayarları

# Heading of the group on the settings page for when Text starts.
text-starting = Başlangıç

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Metin açılınca

# First choice: Text opens the files it had open last time.
text-reopen = Geçen seferki dosyaları yeniden aç

# Second choice: Text opens with an empty page.
text-start-blank = Boş başla

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Bir dosyaya kaydedilmemiş çalışma, hangi seçenek seçilirse seçilsin her zaman saklanır ve geri gelir.

# Heading of the group on the settings page for how the text looks.
text-showing = Metnin görünümü

# Label of the size slider: how big the text is.
text-size = Boyut

# Name of the size slider, read out by a screen reader.
text-text-size = Metin boyutu

# Heading of the group on the settings page for spelling.
text-spelling = Yazım

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Kurulu Hunspell sözlüğü yok.

# Label of the list of spelling languages.
text-language = Dil

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Yazım dili

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Otomatik ({ $lang })

# Heading of the font list on the settings page.
text-font = Yazı tipi

# The first font in the list: the font Trindle ships with.
text-own-font = Trindle'ın kendi yazı tipi

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Dosyayı şu olarak oku

# Heading over the encodings a file is saved in.
text-save-in = Şu kodlamayla kaydet

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = Bir sonraki kaydetme dosyayı bu kodlamayla yazar.

# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } dosyasında kaydedilmemiş değişiklikler var.

# Words saying the text has changes and no file yet.
text-not-saved = Bu metin henüz bir dosyaya kaydedilmedi.

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
# SAFETY: needs a human review before it ships
text-save-named = { $name } kaydedilsin mi?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = Bundan sonra { $left } tane daha var.

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = Kaydetmeden kapat

# Button in the question before closing that keeps the tab open.
text-keep-open = Açık kalsın

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } bir klasör.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } burada açmak için çok büyük.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = { $name } açılamadı: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = { $name } yazılamadı: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = { $name } kaydedilemedi: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name } yeniden okunamadı: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Çalışma { $folder } içinde saklanamadı: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Ayarlar kaydedilemedi: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = { $word } sözcüklerinize eklenemedi: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Liste temizlenemedi: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Dosya seçici açılamadı.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Oturum veri yolu olmadan dosya seçilemez.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = { $lang } adlı bir sözlük yok.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = { $lang } sözlüğü okunamadı: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. Önceki içeriği { $path } içinde saklandı.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } O kaydetmeden önceki içeriği { $path } içinde saklandı.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = Metin durdurulduğunda { $name } kaydediliyordu ve kaydın saklanan kısmı okunamıyor.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = Metin durdurulduğunda { $name } kaydediliyordu ve o zamandan beri değiştirilmiş, bu yüzden olduğu gibi bırakıldı.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = Metin durdurulduğunda { $name } kaydediliyordu ve eski haline getirilemedi: { $reason }. O kaydetmeden önceki içeriği { $path } içinde saklandı.

# Quiet word beside what was just saved, exported or changed.
text-done = Bitti

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Adsız.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Açtığınızdan beri başka yerde değişti.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Yine de kaydet

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Yeniden yükle

