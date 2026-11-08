
# machine translated (Indonesian), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Drive

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Tentang halaman ini

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Setiap drive di dalam atau yang dicolokkan ke komputer ini, digambar sebagai batang berisi partisinya. Setiap partisi selebar ukurannya dan terisi sebanyak yang terpakai, jadi partisi yang penuh mudah terlihat.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Di bawah gambar, setiap drive punya kartu. Buka sebuah partisi, mount atau unmount, atau keluarkan drive dengan aman atau matikan dayanya. Pensil pada partisi membuka perubahannya: namanya, folder sendiri dan apakah siap saat dinyalakan, ukurannya, memeriksa dan memperbaikinya, memformatnya atau menghapusnya. Tanda plus pada ruang kosong membuat partisi baru.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = Drive tempat sistem berjalan ditampilkan tetapi tidak pernah ditawari perubahan yang menghapus atau memindahkan isinya. Setiap perubahan pada drive apa pun meminta kata sandi administrator, setiap kali. Mata menyembunyikan drive atau partisi dari Folders dan ring: tetap ada di sini, berwarna abu-abu, dan tidak terkunci.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Setiap partisi dari setiap drive, terisi sebanyak yang terpakai.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Drive Anda

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Drive tidak dapat dibaca saat ini.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Coba lagi

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Cari lagi

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Tidak ada drive ditemukan. Colokkan satu dan drive itu muncul di sini.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Layanan drive

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Bus sistem

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } tidak tersedia. Pasang dengan: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Melihat drive…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } rusak. Salin semua yang ingin Anda simpan darinya sekarang.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } perlu diperiksa: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Drive dan ruang di dalamnya.
        [1] Drive dan ruang di dalamnya, dan satu drive tercolok.
       *[other] Drive dan ruang di dalamnya, dan { $plugged } drive tercolok.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Hampir penuh: hanya { $free } tersisa di komputer ini. Hapus atau pindahkan beberapa berkas segera.
        [1] Hampir penuh: hanya { $free } tersisa di komputer ini, dan satu drive tercolok. Hapus atau pindahkan beberapa berkas segera.
       *[other] Hampir penuh: hanya { $free } tersisa di komputer ini, dan { $plugged } drive tercolok. Hapus atau pindahkan beberapa berkas segera.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Mulai penuh: { $free } tersisa di komputer ini.
        [1] Mulai penuh: { $free } tersisa di komputer ini, dan satu drive tercolok.
       *[other] Mulai penuh: { $free } tersisa di komputer ini, dan { $plugged } drive tercolok.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Ruang lega: { $free } tersisa di komputer ini.
        [1] Ruang lega: { $free } tersisa di komputer ini, dan satu drive tercolok.
       *[other] Ruang lega: { $free } tersisa di komputer ini, dan { $plugged } drive tercolok.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Komputer ini

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Berkas Anda

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } masih kosong, tempat sistem berada.
       *[files] { $free } masih kosong, untuk berkas Anda sendiri.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } penuh, { $free } kosong dari { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = dari { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Tidak ada bacaan, drive tidak melaporkan kondisinya

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Sehat, menurut drive itu sendiri

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
       *[other] Sehat, menurut semua { $n } drive
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
       *[other] { $n } perlu diperiksa, lihat kartunya
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
       *[other] { $n } drive, total { $total }. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } kosong

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Tidak di-mount

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Ruang kosong

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Tidak ada cakram atau kartu di dalamnya

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Belum ada isinya

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB atau lepasan

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, di dalam komputer ini

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB atau lepasan, disembunyikan

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, di dalam komputer ini, disembunyikan

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } penuh, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Partisi

# Name given to a drive that gives no maker or model.
disks-drive-name = Drive

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Swap

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Terenkripsi, terbuka

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Terenkripsi, terkunci

# Kind of a part that holds other parts.
disks-kind-container = Berisi partisi lain

# Kind of a part that has no filesystem on it.
disks-kind-none = Tanpa sistem berkas

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, partisi tempat komputer dinyalakan
        [legacy] { $kind }, partisi tempat komputer lama dinyalakan
       *[loader] { $kind }, partisi milik boot loader
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = komputer apa pun

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = khusus Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = perangkat lama

# Trouble a drive reports about its spare space.
disks-trouble-spare = kehabisan ruang cadangan

# Trouble a drive reports about its heat.
disks-trouble-temperature = terlalu panas

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = mulai aus

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = mengubah dirinya menjadi hanya-baca

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = cadangan memorinya gagal

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = sebagian mengubah dirinya menjadi hanya-baca

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = aus melebihi perkiraan pembuatnya

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
       *[other] { $n } titik rusak
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
       *[other] dipakai selama { $hours } jam
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Rusak: salin semua yang ingin Anda simpan darinya sekarang

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Perlu perhatian: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Sehat

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Tidak ada drive yang menjawab

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Casingnya menjawab, tetapi drive di dalamnya tidak. Cabut, tunggu sepuluh detik, lalu colokkan ke port biru atau USB-C dengan kabelnya sendiri; jika tetap begini, drive di dalamnya mungkin longgar atau rusak.

# Tip of the round button on a drive's card that takes the drive out safely.
# SAFETY: needs a human review before it ships
disks-remove-safely = Keluarkan dengan aman

# Tip of the round button on a drive's card that powers the drive off.
# SAFETY: needs a human review before it ships
disks-power-off-tip = Matikan dayanya

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Hapus seluruh drive

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Tampilkan drive ini
       *[part] Tampilkan partisi ini
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Sembunyikan drive ini. { $lock }
       *[part] Sembunyikan partisi ini. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Disembunyikan dari Folders dan ring. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = Sistem berjalan darinya: tidak ada yang dapat dihapus di sini.

# Note on the card of a drive that has no partition table.
disks-no-table = Tidak ada tabel partisi: seluruh drive kosong.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = tidak di-mount

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = di-mount saat menyala di { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = muncul di { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = di { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Buka

# Tip of the round button that mounts a part.
disks-mount = Mount

# Tip of the round button that unmounts a part.
disks-unmount = Unmount

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Ubah

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Nama

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Nama (opsional)

# Tip of the button that saves a part's new name.
disks-rename-tip = Ganti nama

# Note under the Name row of a part.
disks-name-note = Namanya di sini dan di komputer lain.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Nama folder

# Tip of the button that saves a part's folder.
disks-use-folder = Pakai folder ini

# Label of the row where a part's folder is chosen.
disks-folder = Folder

# Note under the Folder row of a part.
disks-folder-note = Tempat berkasnya muncul. Biarkan kosong agar komputer memilih.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Mount saat menyala

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Siap setiap kali komputer menyala. Jika tidak ada, komputer tetap menyala.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Ukuran dalam GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = Dari { $least } sampai { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = Dari { $least } sampai { $most }. Mount dulu untuk melihat sekecil apa bisa.

# Tip of the button that resizes a part.
disks-resize-tip = Ubah ukuran

# Label of the row where a part is resized.
disks-size = Ukuran

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Ini akan menghapus semua isi { $words }. Tidak dapat dibatalkan.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Klik lagi: { $name } di { $drive } berubah dari { $from } menjadi { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Sediakan ruang untuk partisi baru

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Ruang dalam GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Sediakan ruang

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Klik lagi: { $name } di { $drive } berkurang { $size }, setelah snapshot

# Note under the row that makes room.
disks-room-note = Diperkecil saat dipakai, setelah snapshot, menyisakan ruang kosong.

# Tip of the button that checks a part for faults.
disks-check-tip = Periksa kesalahannya

# Tip of the button that repairs a part.
disks-repair-tip = Perbaiki

# Label of the row where a part is checked and repaired.
disks-faults = Kesalahan

# Note under the Faults row.
disks-faults-note = Mencari kesalahan dalam cara berkasnya disimpan, lalu membetulkannya. Partisi di-unmount selama ini berjalan.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Klik lagi untuk memperbaiki { $name }, di { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Jenis

# Label of the row where a part is formatted.
disks-format = Format

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Format

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = Memulai dari awal sebagai satu partisi, memenuhi drive.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = Menghapus semua isi partisi ini saja.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Klik lagi: semua isi { $words } dihapus

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, di { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Hapus { $name }, { $size }, di { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Hapus partisi ini

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = Ruangnya menjadi kosong, siap untuk partisi baru atau untuk partisi sebelumnya membesar ke sana.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Buat partisi baru di sini

# Label of the row where a new part is made.
disks-new-part = Partisi baru

# Note under the row where a new part is made.
disks-new-part-note = Dibuat di ruang kosong ini, dan diformat siap dipakai.

# Tip of the button that makes a new part.
disks-make-tip = Buat

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-make-warning = Klik lagi untuk membuat partisi { $size } di { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Hapus { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, di { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Partisi baru

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } kosong, di { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Tutup

# Words on a row while a part is being mounted.
disks-mounting = Melakukan mount

# Words on a row when a part has been mounted.
disks-mounted = Sudah di-mount

# Words on a row while a part is being unmounted.
disks-unmounting = Melakukan unmount

# Words on a row when a part has been unmounted.
disks-unmounted = Sudah di-unmount

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Menutupnya

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Aman untuk dilepas

# Words on a drive's row while it is being powered off.
disks-powering-off = Mematikan daya

# Words on a drive's row when it has been powered off.
disks-off-safe = Mati: aman dicabut

# Words on a row while a part is being renamed.
disks-renaming = Mengganti nama

# Words on a row when a part has been renamed.
disks-renamed = Nama diganti

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = Komputer kembali memilih foldernya

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Muncul di { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Di-mount saat menyala di { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Dibiarkan saat menyala

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Menyimpan

# Words on a row while a part is being resized.
disks-resizing = Mengubah ukuran

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Ukuran diubah menjadi { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Berkasnya tidak ditemukan, jadi ruang tidak dapat disediakan.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Membuat snapshot

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Snapshot tidak ditemukan setelahnya.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Tidak ada yang diubah ukurannya.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Ada perubahan saat snapshot dibuat, jadi tidak ada yang diubah ukurannya.

# Words on a row while room is being made.
disks-making-room = Menyediakan ruang

# Words on a row when room has been made. { $size } is how much.
disks-room-made = Ruang { $size } disediakan

# Words on a row while a part is being repaired.
disks-repairing = Memperbaiki

# Words on a row when a part has been repaired.
disks-repaired = Diperbaiki

# Red words when some faults could not be put right.
# SAFETY: needs a human review before it ships
disks-repair-failed = Beberapa kesalahan tidak dapat dibetulkan. Salin semua yang ingin Anda simpan.

# Words on a row while a part is being checked.
disks-checking = Memeriksa

# Words on a row when the check found no faults.
disks-no-faults = Tidak ada kesalahan ditemukan

# Red words when the check found faults.
disks-faults-found = Kesalahan ditemukan: tekan kunci pas untuk memperbaikinya.

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = Memformat

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = Sudah diformat

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = menghapus seluruh drive

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Partisi baru tidak ditemukan.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Layanan drive tidak dapat dijangkau.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Pengenal partisi baru tidak dapat dibaca.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Menghapus

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Dihapus

# Words on a row while a new part is being made.
disks-making-it = Membuatnya

# Words on a row when a new part has been made.
disks-made = Dibuat

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Itu tidak dapat disimpan: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Beri nama dulu.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
       *[other] Nama itu terlalu panjang: paling banyak { $max } huruf.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Gunakan hanya huruf, angka, - dan _ (tanpa spasi).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Mulai dengan huruf atau angka.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } sudah dipakai.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
       *[other] Nama itu terlalu panjang untuk jenis ini: paling banyak { $max } huruf.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Nama FAT tidak boleh berisi * ? . , ; : / \ | + = < > [ ] atau ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Mount dulu: jenis ini hanya dapat diperbesar saat di-mount.
       *[smaller] Mount dulu: jenis ini hanya dapat diperkecil saat di-mount.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Jenis ini tidak dapat diperbesar.
       *[smaller] Jenis ini tidak dapat diperkecil.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Kata sandi tidak diberikan, jadi tidak ada yang berubah.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Kata sandi tidak diberikan, jadi pekerjaan berhenti di tengah jalan.

# Red words when another change is pressed while one is still being made.
disks-change-going = Sebuah perubahan sedang dilakukan.

disks-one-at-a-time = Perubahan lain masih berjalan. Coba lagi setelah selesai.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Tidak dapat dipastikan bahwa kata sandi akan diminta, jadi tidak ada yang berubah.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Kata sandi yang baru diberikan tidak dapat ditarik kembali, jadi selama beberapa menit perubahan lain pada drive mungkin tidak meminta kata sandi. Sedang dicoba lagi.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = Kata sandi yang diberikan kini sudah ditarik kembali: perubahan berikutnya akan memintanya.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Layanan drive tidak dapat dijangkau, jadi tidak ada yang berubah.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Drive ditutup saat perubahan ini sedang dilakukan.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Drive tetap terbuka sampai ini selesai, agar dapat menyampaikan bagaimana akhirnya.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, jadi tidak ada yang berubah.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, jadi pekerjaan berhenti di tengah jalan.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Layanan drive tidak dapat dihubungi

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } Pekerjaan berhenti di tengah jalan.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Selesai: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Gagal: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Belum dilakukan: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Tidak diketahui: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } di-unmount untuk ini dan sekarang tidak di-mount.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } di-unmount untuk ini dan masih di-unmount: mount dari barisnya.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } mungkin telah di-unmount: lihat barisnya.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } di-unmount untuk ini, dan me-mount-nya lagi memerlukan kata sandi tersendiri: mount dari barisnya.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } di-unmount untuk ini dan tidak dapat di-mount lagi ({ $why }): mount dari barisnya.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Sudah diformat, tetapi tidak diberi lagi folder { $dir }. { $why } Pilih folder lagi dari barisnya.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Masih ada yang memakainya. Tutup jendela atau program yang menampilkan berkasnya, lalu coba lagi.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Tidak cukup ruang untuk isinya pada ukuran itu.

# Step of a change. { $dev } is the part.
disks-step-unmount = meng-unmount { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = me-mount { $dev } lagi

# Step of a change.
disks-step-eject = menutup drive agar dapat dilepas

# Step of a change.
disks-step-power-off = mematikan daya drive

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = memformat { $dev }

# Step of a change.
disks-step-create = membuat partisi baru

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = menghapus { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = mengubah ukuran partisi { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = mengubah ukuran isi { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = memperbaiki { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = memeriksa { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = mengganti nama { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = menyimpan pilihan folder dan saat menyala untuk { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = langkah { $method } pada { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Partisi { $kind } ini hanya dapat diperkecil saat tidak dipakai, jadi ruang tidak dapat disediakan dari sini.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Partisi xfs ini tidak dapat diperkecil.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Partisi { $kind } ini tidak dapat diperkecil saat dipakai.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Snapshot hanya dapat dibuat dari disk sistem itu sendiri, jadi ruang tidak dapat disediakan di sini.

# Red words under the Make room row.
disks-room-not-in-table = Ini bukan bagian dari tabel drive, jadi ruang tidak dapat disediakan darinya.

# Red words under the Make room row.
disks-room-no-online-shrink = Layanan drive tidak dapat memperkecil Btrfs saat dipakai di sini, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-holds-parts = Ini berisi partisi lain, jadi ruang tidak dapat disediakan darinya.

# Red words under the Make room row.
disks-room-not-gpt = Drive-nya tidak memiliki tabel GPT, jadi ruang tidak dapat disediakan darinya.

# Red words under the Make room row.
disks-room-mount-first = Mount dulu: Btrfs diperkecil saat di-mount.

# Red words under the Make room row.
disks-room-mounts-not-one = Ini tidak di-mount di tepat satu tempat, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-read-only = Ini di-mount hanya-baca, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-read-only-unknown = Tidak dapat diketahui apakah ini hanya-baca, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-many-drives = Berkasnya tersebar di lebih dari satu drive, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-drives-unknown = Tidak dapat diketahui berapa drive yang dipakai berkasnya, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-swap-file = Ada berkas swap di dalamnya, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-swap-unknown = Tidak dapat diketahui apakah ada berkas swap di dalamnya, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-used-unknown = Jumlah yang terpakai tidak dapat dibaca, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-area-unknown = Layanan drive tidak menyebutkan seberapa besar area berkasnya, jadi tidak dapat diperiksa dan ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-open-missing = Salinan terbukanya tidak ditemukan, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-encryption-sizes = Ukuran yang diberikan untuk enkripsi tidak cocok, jadi ruang tidak dapat disediakan.

# Red words under the Make room row.
disks-room-sizes = Ukuran yang diberikan untuknya tidak cocok, jadi ruang tidak dapat disediakan.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Isinya { $used } dan perlu menyimpan lebih dari itu, jadi terlalu sedikit yang dapat dikosongkan.

# Red words under the Make room row.
disks-room-sizes-unread = Ukurannya tidak dapat dibaca.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Ruang dari { $least } sampai { $most } dapat disediakan.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, jadi pekerjaan dihentikan.

# What was found wrong while making room.
disks-room-check-unmounted = Berkasnya tidak lagi di-mount

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = Partisi berukuran { $is } dan seharusnya { $should }

# What was found wrong while making room.
disks-room-check-open = Salinan terbuka enkripsi tidak berukuran seharusnya

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = Area berkas berukuran { $is } dan seharusnya { $should }

# What was found wrong while making room.
disks-room-check-area-small = Area berkas lebih kecil daripada yang terpakai

# What was found wrong while making room.
disks-room-check-fit = Wadah dan isinya tidak lagi muat

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] Partisi tidak lagi dicantumkan oleh layanan drive, jadi pekerjaan dihentikan.
        [copy] Salinan terbuka tidak lagi dicantumkan oleh layanan drive, jadi pekerjaan dihentikan.
       *[files] Sistem berkas tidak lagi dicantumkan oleh layanan drive, jadi pekerjaan dihentikan.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = memperkecil area berkas

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = memperkecil salinan terbuka enkripsi

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = memperkecil partisi

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = memeriksa ukuran

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Sebelum menyediakan ruang di bawah { $dev } ({ $at })


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Klik sekali lagi: { $name } di { $drive } berubah dari { $from } menjadi { $to }. Cadangkan filenya dulu, karena listrik padam saat memperkecil dapat menghilangkannya.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows di { $name } dalam keadaan hibernasi atau dibiarkan dalam fast start-up, jadi memperkecilnya dapat menghilangkan filenya. Jalankan Windows, pilih Restart, lalu coba lagi.

