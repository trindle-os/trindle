# machine translated (Indonesian), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle tidak dapat membaca { $file } di tingkat atas folder ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } tidak memiliki bagian [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Versi Trindle ini mendukung Interface={ $known }, bukan Interface={ $n }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] membutuhkan Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] membutuhkan Id berisi huruf, angka, titik, dan tanda hubung, seperti org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] membutuhkan Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] membutuhkan Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = Summary di [Plugin] hanya satu baris.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
       *[other] [{ $group }] { $key } harus berupa teks biasa dalam satu baris, paling banyak { $max } karakter.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = Izin { $name } tidak ada. Izin yang tersedia adalah { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] membutuhkan Icon: file SVG atau PNG di folder plugin.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = Icon di [Ring] menyebut { $icon }, tetapi itu tidak ada di folder plugin.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] membutuhkan Label, yang ditampilkan saat penunjuk berada di atasnya.

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] membutuhkan Fields, nama-nama kolomnya.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Nama kolom { $name } hanya boleh berisi huruf, angka, tanda hubung, dan garis bawah.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Kolom { $name } tidak memiliki bagian [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] membutuhkan Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
       *[other] [{ $group }] dapat memiliki paling banyak { $most } Choices. Masing-masing harus berupa teks biasa, paling banyak { $chars } karakter.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom harus menyebut nama file di folder plugin. Gunakan hanya huruf, angka, titik, tanda hubung, dan garis bawah. Jangan mulai nama dengan titik atau menyertakan jalur folder.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] adalah pilihan, jadi membutuhkan Choices dengan dua opsi atau lebih, atau ChoicesFrom, yaitu sebuah file.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = Default di [{ $group }] bukan salah satu dari Choices-nya.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] adalah catatan, jadi membutuhkan TextFrom yang menyebut nama file di folder plugin. Gunakan hanya huruf, angka, titik, tanda hubung, dan garis bawah. Jangan mulai nama dengan titik atau menyertakan jalur folder.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] adalah catatan, jadi tidak menyimpan pengaturan dan tidak boleh memiliki Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = Kind di [{ $group }] adalah { $kind }, tetapi harus switch, text, choice, atau note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] adalah sakelar, jadi Default-nya harus true atau false.

# Red words when the [Background] part is given without the background permission.
# SAFETY: needs a human review before it ships
plugins-background-needs-permission = [Background] membutuhkan izin background di Permissions pada [Plugin].

# Red words when a plugin has nothing that makes it do anything.
# SAFETY: needs a human review before it ships
plugins-hooks-nothing = Plugin tidak terhubung ke apa pun. Beri [Ring], [Settings], [Background], [Shortcut], atau izin notifikasi.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } adalah tautan yang mengarah ke luar folder plugin.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = Run di [{ $group }] tidak dapat dibaca sebagai perintah.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = Run di [{ $group }] harus menyebut program di dalam folder plugin, bukan { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = Run di [{ $group }] menyebut { $program }, tetapi plugin tidak berisi program itu (tidak ada, atau tidak dapat dijalankan).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
       *[other] Menu di [Ring] memiliki { $n } item. Paling banyak { $max } yang diizinkan.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Id menu { $id } hanya boleh berisi huruf, angka, tanda hubung, dan garis bawah.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Menu di [Ring] menyebut { $id } dua kali.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Menu di [Ring] menyebut { $id }, yang tidak memiliki bagian [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
       *[other] Label di [{ $group }] harus berupa satu baris pendek, paling banyak { $max } karakter.
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-only-flow = [{ $group }] hanya untuk Flow dari katalog Trindle, yang meminta izin kontrol.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-needs-run = [{ $group }] membutuhkan Run, perintah yang diberi PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
       *[other] ShowWhen di [{ $group }] dapat memiliki paling banyak { $max } karakter.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = ShowWhen di [{ $group }] harus berbentuk kolom=nilai, seperti ShowWhen=mode=home. Sebutkan kolom lain dan nilai yang harus dimilikinya.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = ShowWhen di [{ $group }] harus ditulis kolom=nilai, dan nama kolom hanya boleh berisi huruf, angka, tanda hubung, dan garis bawah.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
       *[other] ShowWhen di [{ $group }] membutuhkan nilai berupa teks biasa dalam satu baris, paling banyak { $max } karakter.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = ShowWhen di [{ $group }] menyebut kolomnya sendiri. Harus menyebut kolom lain yang lebih awal.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = ShowWhen di [{ $group }] menyebut kolom { $other }, yang tidak ada di Fields plugin ini.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = ShowWhen di [{ $group }] menyebut kolom { $other }, yang berada setelahnya di Fields. Harus kolom yang lebih awal.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = ShowWhen di [{ $group }] menyebut kolom { $other }, yang memiliki ShowWhen sendiri. Sebuah kolom hanya dapat bergantung pada kolom yang tidak memilikinya.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = ShowWhen di [{ $group }] menyebut sakelar { $other }, jadi nilainya harus true atau false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = ShowWhen di [{ $group }] bernilai { $value }, yang bukan salah satu Choices dari { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = ShowWhen di [{ $group }] merujuk ke catatan { $other }, yang tidak menyimpan pengaturan. Gunakan sakelar, pilihan, atau kolom teks.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = Listen di [Network] membutuhkan izin network di Permissions pada [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = Listen di [Network] membutuhkan izin listen di Permissions pada [Plugin].

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = Izin listen membutuhkan Listen di [Network] untuk mendaftar port tempat plugin dapat menerima koneksi.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = plugin.ini-nya memakai interface yang tidak didukung oleh versi Trindle ini.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Tidak ada port yang terdaftar untuk didengarkan.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = plugin.ini-nya tidak dapat dibaca.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Belum diizinkan untuk mendengarkan.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = Port yang terdaftar telah berubah. Anda perlu mengizinkan koneksi pada port itu lagi.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none =
    { $n ->
       *[other] { $name } membutuhkan izin Anda sebelum dapat bekerja. Belum ada satu pun dari { $n } izin yang diminta yang Anda izinkan.
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = Anda baru mengizinkan { $a } dari { $n } izin yang diminta { $name }.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Tidak meminta apa pun.
       *[other] Semua { $n } izin yang diminta sudah diizinkan.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = Izinkan

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = dijangkau lewat port yang berbeda

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Tidak terpasang.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = plugin lain

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Plugin itu tidak terpasang.

# Red words when a permission could not be kept. { $why } is the reason.
# SAFETY: needs a human review before it ships
plugins-not-saved = Gagal menyimpan: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Gagal menyalin file plugin.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Plugin tidak berjalan di latar belakang: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle tidak menemukan trindle-plugin untuk memulainya.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
       *[other] tidak mulai dalam { $n } detik.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = tidak ada sesi desktop yang berjalan untuk diikuti, jadi tidak dimulai. Masuk dulu.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = tidak dapat menentukan sesi desktop mana yang diikuti ({ $sessions }), jadi tidak dimulai. Jalankan lagi dengan WAYLAND_DISPLAY diatur ke yang benar.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = ada folder di tempat file seharusnya berada

# Error text when a plugin's id is not an id.
plugins-not-an-id = bukan ID yang valid

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
       *[other] Shortcuts di [Plugin] memiliki { $n } pintasan. Paling banyak { $max } yang diizinkan.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Id pintasan { $id } hanya boleh berisi huruf, angka, tanda hubung, dan garis bawah.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts di [Plugin] menyebut { $id } dua kali.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts di [Plugin] menyebut { $id }, yang tidak memiliki bagian [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Keys { $keys } di [{ $group }] sudah menjadi salah satu pintasan Trindle. Pilih tombol lain.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Keys { $keys } di [{ $group }] sudah dipakai pintasan lain di plugin ini.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] membutuhkan Keys, seperti Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Kombinasi tombol itu bukan pintasan yang valid. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } sudah menjadi salah satu pintasan Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } sudah menjadi salah satu pintasan Anda sendiri.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } sudah dipakai oleh { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Itu bukan pintasan plugin yang terpasang.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle tidak dapat menggunakannya ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle tidak dapat menyimpannya karena { $why }. Tidak ada yang berubah.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Gagal menyimpan: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = Port-nya tidak dapat ditutup: { $why }.

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle tidak dapat membuka port-nya, jadi mendengarkan dimatikan lagi: { $why }.

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = Port-nya tidak dibuka ({ $why }), dan mendengarkan juga tidak dapat dimatikan lagi ({ $kept }). Tidak ada yang terbuka. Matikan di Pengaturan, Izin.


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Hanya pembaca layar Trindle dari katalog yang dapat memakai izin aksesibilitas.

# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = Izin { $name } disebut dua kali.

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = Minta { $home } atau jaringan, jangan keduanya: jaringan menjangkau jaringan lokal dan juga internet.

# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }] membutuhkan Run, perintah yang dijalankan.

# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } Versi lama juga tidak dapat dikembalikan: { $back } Keduanya disimpan di folder plugin.
