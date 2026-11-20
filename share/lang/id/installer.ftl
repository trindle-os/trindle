
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Pasang { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Selamat datang

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Disk

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Privasi

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Kata sandi

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Anda

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Siap

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Memasang

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Selesai

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Bersiap

# Red line at the foot when the installer's helper could not be started.
installer-no-start = Tidak dapat memulai pemasang: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Tidak dapat membaca rencana.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Tidak dapat membaca rencana: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = Pemasangan berhenti. Tidak ada lagi yang ditulis.

# Name of the round button at the foot that goes back a page.
installer-back = Kembali

# Name of the main button at the foot that goes to the next page.
installer-next = Lanjut

# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Pasang

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Mulai ulang sekarang

# Beside the keyboard test box on the first page.
installer-try-keyboard = Coba papan ketik

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Ketik di sini untuk mencoba papan ketik

# Note beside Install after one press, when the second press writes beside another system.
# SAFETY: needs a human review before it ships
installer-arm = Klik lagi untuk memasang

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Klik lagi untuk menghapus dan memasang

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Pilih bahasa
# Note beside Next on the first page while Trindle's language is not chosen.
installer-missing-trindle-language = Pilih bahasa Trindle

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Pilih papan ketik

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Atur zona waktu

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Pilih tempat Trindle dipasang

# Note beside Next on the disk page while the disks are read again, after one turned out not to be the disk shown.
installer-disks-rereading = Membaca ulang disk

# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = Ketik kata sandi disk dua kali. Keduanya harus sama

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Isi nama masuk

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Beri nama untuk komputer ini

# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = Ketik kata sandi Anda dua kali. Keduanya harus sama

# Title of the first page.
installer-welcome-title = Selamat datang di { $name }

# Sentence under the title of the first page.
installer-welcome-help = Pilih bahasa Anda, dan Trindle akan menyesuaikan sisanya.

# Head of the card for choosing the computer's language (dates, spelling, other programs), and the name of its search box.
installer-language = Bahasa
# Label beside the box at the right of the first page's sentence, for choosing the language of Trindle's own words (only those Trindle has).
installer-trindle-language = Bahasa Trindle

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Papan ketik

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Waktu

# Under a card's head while nothing has been chosen in its list.
installer-select = Pilih

# Title of the disk page.
installer-disks-title = Tempat { $name } dipasang

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Semua isi disk yang Anda pilih akan diganti, kecuali Anda memilih memasang di samping isi yang sudah ada.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Trindle tidak menemukan disk yang cukup besar. Ukuran yang diperlukan 20 GB.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Gunakan seluruh disk, mengganti semua isinya

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Pertahankan { $what } dan bagi disk

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } disisakan untuk isi yang ada sekarang, dan { $mine } untuk Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Ruang kosong

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } dan { $last }

# Under a disk's name: what is on it.
installer-disk-has = Berisi { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Berisi { $list }, terenkripsi

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Berisi sistem terenkripsi

# Under a disk's name: there is nothing on it.
installer-disk-empty = Kosong

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Berisi file, tetapi tidak ada sistem yang dikenali Trindle

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = File swap { $gb } GB

# Name of the switch that makes a swap file.
installer-swap-switch = Buat file swap

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot aktif, sehingga komputer ini tidak dapat hibernasi, tetapi file swap tetap membantu saat memori hampir habis. Trindle hanya membuatnya jika disk punya banyak ruang tersisa.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = File swap memungkinkan komputer berhibernasi, yaitu menyimpan semua yang terbuka saat daya mati. File ini juga memberi komputer ruang tambahan saat memori hampir habis. Tanpa file swap, komputer tidak dapat hibernasi. Trindle hanya membuatnya jika disk punya banyak ruang tersisa.

# Title of the page about encrypting the disk.
installer-privacy-title = Mengenkripsi disk

# Sentence under the title of that page.
installer-privacy-help = Disk terenkripsi tidak dapat dibaca oleh siapa pun yang mencabutnya dari komputer ini.

# Head of the card with the three choices.
installer-privacy-locked = Cara disk dikunci

# Name and words of the choice not to encrypt the disk.
# SAFETY: needs a human review before it ships
installer-privacy-none = Jangan enkripsi disk

# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Enkripsi, dan minta kata sandi saat komputer dinyalakan

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Enkripsi, dan biarkan chip TPM komputer ini membukanya

# Head of the card about the TPM chip.
installer-privacy-about = Tentang chip

# In the card about the chip, on a computer that has one.
installer-privacy-chip = Komputer ini punya chip TPM. Saat pertama kali dinyalakan setelah pemasangan, komputer meminta kata sandi satu kali sementara chip disiapkan. Setelah itu chip membuka disk dan Anda tidak perlu mengetik apa pun. Simpan kata sandinya, karena Anda memerlukannya jika chip diganti atau disk dipindah ke komputer lain.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Komputer ini tidak punya chip TPM yang dapat dipakai Trindle, jadi kata sandi akan diminta setiap kali dinyalakan. Pada komputer yang punya chip, chip dapat membuka disk sendiri.

# Title of the disk password page.
installer-disk-title = Kata sandi disk

# Sentence under the title of the disk password page.
installer-disk-help = Kata sandi ini diketik lebih dahulu sebelum apa pun saat komputer dinyalakan.

# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Kata sandi disk

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Sarankan frasa sandi

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Ketik sekali lagi

# Red line under the password boxes when the two do not match.
installer-mismatch = Kedua kata sandi tidak sama.

# Head of the card that tells the person to write the password down.
installer-disk-write-down = Tuliskan

# Words in that card.
# SAFETY: needs a human review before it ships
installer-disk-write-down-text = Tulis kata sandi ini di kertas dan simpan di tempat aman, jauh dari komputer. Tanpa kata sandi ini, tidak seorang pun dapat membuka disk, bahkan Anda sendiri. Tidak ada jalan masuk lain. Frasa sandi dari empat kata biasa mudah diketik dan sulit ditebak.

# Title of the account page.
installer-you-title = Tentang Anda

# Sentence under the title of the account page.
installer-you-help = Ini adalah akun yang Anda pakai untuk masuk.

# Head of the card with the name boxes.
installer-account = Akun Anda

# Label and name of the box for the person's name.
installer-your-name = Nama Anda

# Label and name of the box for the sign-in name.
installer-user-name = Nama masuk

# Label and name of the box for the computer's name.
installer-host-name = Nama komputer ini

# Head of the card with the password boxes.
installer-your-password = Kata sandi Anda

# Label and name of the first password box.
installer-password = Kata sandi

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Seberapa sulit ditebak

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Isi nama masuk.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Nama itu terlalu panjang.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Mulai dengan huruf a-z.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Hanya a-z, 0-9, - dan _.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Sistem sudah memakai nama itu.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Beri nama untuk komputer ini.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Hanya huruf, angka, dan -.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Komputer akan langsung menebak ini.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
       *[other] Komputer akan menebak ini dalam sekitar { $n } menit.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
       *[other] Komputer akan menebak ini dalam sekitar { $n } jam.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
       *[other] Komputer akan menebak ini dalam sekitar { $n } hari.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
       *[other] Komputer memerlukan sekitar { $n } tahun untuk menebak ini.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Komputer akan butuh waktu lebih lama dari seluruh sejarah tercatat untuk menebak ini.

# Title of the Ready page.
installer-ready-title = Siap dipasang

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Belum ada yang diubah

# Head of the card about security updates.
installer-ready-security = Pembaruan keamanan

# In the card about security updates.
installer-ready-security-text = { $name } memasang perbaikan keamanan Debian sendiri setiap hari. Anda dapat mematikannya di Pengaturan, Tentang komputer ini.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = Semua isi { $disk } akan diganti.

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Semua isi { $part } dipertahankan, tetapi cadangkan dulu. Listrik padam saat memperkecil dapat menghilangkan file.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Belum ada disk yang dipilih.

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = Seluruh { $disk }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Semua isinya diganti oleh { $name }.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Di samping { $part }

# Words of that row.
installer-ready-beside-text = { $part } diperkecil menjadi { $size }, dan { $name } dipasang di ruang yang dikosongkan.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Tanpa disk

# Words of that row.
installer-ready-no-disk-text = Pilih disk.

# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Tidak terenkripsi

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-plain-text = Siapa pun yang mencabut disk ini dapat membacanya.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Terenkripsi

# Words of that row when a password is typed at each start.
installer-ready-locked-password = Anda mengetik kata sandinya setiap kali komputer dinyalakan.

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = Anda mengetik kata sandi satu kali, lalu chip TPM komputer ini membuka disk.

# Added to the end of the disk's row on the last page when a swap file is to be made, with its size in GB.
installer-ready-swap = Ditambah file swap { $gb } GB.

# Under the choice to let the TPM chip unlock the disk, on a computer with no usable chip.
installer-privacy-chip-off = Perlu chip TPM yang dapat dipakai Trindle.

# In the card about the chip when the disk is not being encrypted, on a computer that has a chip.
installer-privacy-chip-none = Komputer ini punya chip TPM, yang dapat membuka disk terenkripsi.

# In the card about the chip when the disk is not being encrypted, on a computer with no chip Trindle can use.
installer-privacy-no-chip-none = Komputer ini tidak punya chip TPM yang dapat dipakai Trindle.

# Words of the language row: Trindle's language (its own name), the keyboard and the time zone chosen.
installer-ready-language = Trindle dalam { $trindle }, papan ketik { $keyboard }, waktu { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Memasang { $name }

# Head of the card that holds the short tour.
installer-tour = Tur singkat

# Title of the last page.
installer-done-title = { $name } sudah terpasang

# Sentence under the title of the last page.
installer-done-help = Cabut USB, lalu mulai ulang.

# Head of the card that repeats the warning about the disk password.
installer-done-keep = Simpan kata sandi disk dengan aman

# In that card when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-keep-chip = Disk ini terenkripsi. Saat pertama kali dinyalakan, komputer meminta kata sandi disk satu kali sementara chip disiapkan. Setelah itu disk terbuka sendiri. Tulis kata sandi itu di kertas dan simpan di tempat aman, jauh dari komputer ini. Tanpa kata sandi itu, tidak seorang pun dapat membuka disk, bahkan Anda sendiri.

# In that card when the password is typed at each start.
# SAFETY: needs a human review before it ships
installer-keep-password = Disk ini terenkripsi. Jika belum, tulis kata sandi disk di kertas sekarang dan simpan di tempat aman, jauh dari komputer ini. Tanpa kata sandi itu, tidak seorang pun dapat membuka disk, bahkan Anda sendiri.

# Title of the first slide.
installer-slide-ring = Semuanya dalam satu lingkaran

# Words of the first slide.
installer-slide-ring-text = Gerakkan penunjuk ke tepi layar lalu klik dua kali untuk membuka ring. Program Anda ada di sekelilingnya, sedangkan waktu dan sakelar ada di tengah.

# Title of the second slide.
installer-slide-snapshots = Batalkan pembaruan yang bermasalah

# Words of the second slide.
installer-slide-snapshots-text = Trindle membuat snapshot sistem sebelum dan sesudah setiap perubahan perangkat lunak. Di Pengaturan, Anda dapat mengembalikan seluruh sistem ke salah satunya. Jika komputer tidak mau menyala, Anda bisa melakukannya dari menu boot.

# Title of the third slide.
installer-slide-apps = Aplikasi yang tidak mengganggu

# Words of the third slide.
installer-slide-apps-text = Aplikasi Flathub, aplikasi Debian sendiri, AppImage, dan sumber perangkat lunak semuanya ada di satu tempat, dengan cadangan yang dapat Anda bawa ke komputer lain.

# Title of the fourth slide.
installer-slide-yours = Buat jadi milik Anda

# Words of the fourth slide.
installer-slide-yours-text = Pilih terang atau gelap, pilih suara Anda sendiri, dan pakai papan klip yang mengingat apa yang Anda salin.

# Title of the fifth slide.
installer-slide-small = Kecil dan tenang

# Words of the fifth slide.
installer-slide-small-text = Desktop hemat memori dan menyala dalam beberapa detik, sehingga laptop lama terasa baru lagi.

# Title of the tour slide about the ring.
installer-tour-ring = Ring adalah seluruh desktop

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Letakkan penunjuk di sudut ini dan ring terbuka

# Note on the ring picture, at the time.
installer-tour-ring-time = Waktu dan tanggal ada di tengah

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Program Anda ada di sekelilingnya

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Sakelar untuk suara, layar, semua aplikasi Anda, menjaga tetap menyala, dan mematikan

# Title of the tour slide about Settings.
installer-tour-settings = Pengaturan, dengan kata sederhana

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Ketik di sini untuk menemukan pengaturan

# Note on the Settings picture, at Network.
installer-tour-settings-network = Di Jaringan Anda bergabung ke jaringan Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Terang atau gelap, kapan pun Anda mau

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Dua puluh warna, atau ketik warna Anda sendiri

# Title of the tour slide about Apps.
installer-tour-apps = Aplikasi, untuk yang lainnya

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Cari program menurut nama atau kegunaannya

# Note on the Apps picture, at the list.
installer-tour-apps-many = Ribuan program gratis dari Flathub dan Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Pembaruan untuk semuanya, termasuk Trindle sendiri

# Title of the tour slide about voice control.
installer-tour-voice = Bicara ke komputer

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Kendali suara ada di Pengaturan, lalu Ring

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Tekan Tambahkan kontrol suara sekali. Unduhannya sekitar 150 MB dan butuh beberapa menit

# Note on the voice picture, about what to say.
installer-tour-voice-say = Lalu ketuk bagian tengah ring dan ucapkan nama aplikasi, “lebih keras” atau “pengaturan Wi-Fi”

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Trindle memahami ucapan Anda di komputer ini, dan tidak pernah dikirim ke tempat lain

# Title of the tour slide about snapshots.
installer-tour-snapshots = Kembali ke keadaan sebelum perubahan

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Trindle membuat snapshot sebelum setiap pembaruan

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Jika ada yang rusak, Anda bisa kembali ke keadaan semula lewat menu boot

# Name of the picture of the ring, and the head of its card.
installer-live-caption = Ring, dengan semuanya di dalamnya

# Sentence under the title of the welcome from a stick.
installer-live-help = Anda menjalankannya dari USB, jadi tidak ada yang diubah di komputer ini.

# Head of the card with the three first things to do.
installer-live-begin = Untuk memulai

# Name of the first thing to do.
installer-live-open = Buka ring

# Line under it.
installer-live-open-line = Klik dua kali di tepi layar, atau tekan Super.

# Name of the second thing to do.
installer-live-speak = Pakai suara Anda

# Line under it when voice control is there.
installer-live-speak-line = Ketuk bagian tengah ring dan ucapkan apa yang Anda mau.

# Line under it when voice control has to be added.
installer-live-speak-add = Setelah Trindle terpasang, tambahkan kontrol suara di Pengaturan, di bawah Ring.

# Name of the third thing to do.
installer-live-tab = Semua ada di ring

# Line under it.
installer-live-tab-line = Yang sedang terbuka, pemberitahuan Anda, dan papan klip semuanya ada di sana.

# The choice of the dark look.
installer-live-dark = Gelap

# The choice of the light look.
installer-live-light = Terang

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Terang atau gelap, untuk seluruh desktop

# Name of the round button that closes this window and opens the ring.
installer-live-around = Lihat-lihat dulu

# Name of the main button that opens the installer.
installer-live-install = Pasang { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Menyiapkan ruang

# Foot of the install page while the disk is prepared.
installer-step-prepare = Menyiapkan disk

# Foot of the install page while a place is made beside another system.
installer-step-place = Menyiapkan ruang untuk Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Memeriksa isi disk saat ini

# Foot of the install page while another system is made smaller.
installer-step-smaller = Memperkecil sistem lain

# Foot of the install page while the disk is encrypted.
installer-step-lock = Mengenkripsi disk

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Menyiapkan chip TPM

# Foot of the install page while the system is copied.
installer-step-copy = Menyalin Trindle ke disk

# Foot of the install page while settings are written.
installer-step-settings = Menulis pengaturan

# Foot of the install page while the person's account is made.
installer-step-account = Membuat akun Anda

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Membuang tambahan live

# Foot of the install page while the start-up files are made.
installer-step-startup = Membuat file awal

# Foot of the install page while the boot menu is set up.
installer-step-boot = Menyiapkan menu boot

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Menyiapkan cuplikan

# Foot of the install page at the very end.
installer-step-finishing = Menyelesaikan

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } tidak dapat dimulai: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } tidak selesai. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = Tidak dapat menulis { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } bukan disk.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } bukan disk di komputer ini.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Belum ada disk yang dipilih.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Nama masuk itu tidak dapat dipakai Trindle.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Nama komputer itu tidak dapat dipakai Trindle.

# Red line at the foot: the language cannot be used.
installer-bad-language = Bahasa itu tidak dapat dipakai Trindle.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Papan ketik itu tidak dapat dipakai Trindle.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Zona waktu itu tidak dapat dipakai Trindle.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Nama itu berisi karakter yang tidak dapat dipakai Trindle.

# Red line at the foot: a password has a line break in it.
installer-bad-password = Kata sandi tidak boleh berisi baris baru.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = Bagian kedua pemasang harus dijalankan sebagai root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Disk ini tidak punya partisi boot untuk dibagi, jadi Trindle tidak dapat dipasang di samping isi yang sudah ada.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Trindle tidak menemukan partisi baru.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = Tidak dapat membaca ukuran { $part }.

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle tidak dapat memperkecil partisi { $fs }.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle tidak dapat memperkecil partisi dengan jenis yang tidak dikenal.

# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = Anda memerlukan kata sandi untuk mengenkripsi disk.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Tidak dapat menyalin sistem: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Tidak dapat menyalin sistem. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Tidak ada ruang untuk file swap.

# Under the swap file switch when the chosen disk or space is too small for one; the switch is then off and cannot be turned on.
installer-swap-too-small = Ruang ini terlalu kecil untuk file swap.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Trindle tidak dapat membaca lokasi file swap di disk.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Trindle tidak dapat mengidentifikasi partisi baru.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Tidak dapat mengatur zona waktu: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Trindle tidak dapat memasang menu boot di komputer ini.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Yakin?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Lihat yang sedang terbuka, pakai papan klip, dan bersihkan layar dengan satu tombol

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } adalah stik tempat Trindle sedang berjalan.

# Red words under the disk, back on the disk page: something on the chosen disk is mounted, swapped to or open.
installer-disk-in-use = { $dev } sedang dipakai. Tutup apa pun yang memakainya, lalu pilih disk itu lagi.

# Said beside how far the install has got, once it has started.
installer-started = Memasang

# The button that ends an install under way.
installer-halt = Berhenti

# Beside Stop after its first press.
installer-halt-sure = Hentikan pemasangan? Isi lama disk sudah hilang.

# While an install is being stopped.
installer-halting = Menghentikan

# When Stop could not let go of the disk.
installer-halt-stuck = Trindle tidak dapat melepas disk. Mulai ulang komputer untuk mencoba lagi.

# The copy is done and the person is still answering.
installer-step-waiting = Tersalin. Menunggu jawaban Anda.



installer-already-running = Penginstal lain sudah berjalan di komputer ini.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle membutuhkan memori minimal 1 GB, dan komputer ini memilikinya kurang dari itu.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Disk hanya dapat dienkripsi pada komputer yang menyala lewat UEFI, dan komputer ini tidak demikian.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows di { $part } dalam keadaan hibernasi atau dimatikan dengan fast start-up. Memperkecilnya sekarang dapat menghilangkan filenya. Jalankan Windows, pilih Restart, lalu coba lagi.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle sudah terpasang, tetapi tidak dapat menyiapkan snapshot: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle sudah terpasang, tetapi tidak dapat menambahkan snapshot ke menu boot: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Tidak semuanya berhasil disiapkan

# Red line under the disk password boxes: the computer gave no random numbers, so no passphrase could be made.
installer-suggest-failed = Trindle tidak dapat membuat frasa sandi. Ketik sendiri.

# Red words on the disk page: the disk was swapped for another after the list was read.
installer-disk-changed = { $dev } bukan disk yang ditampilkan tadi. Pilih disk lagi.

# Red line at the foot: a disk with an old-style partition table cannot be shared with.
installer-no-gpt = { $dev } tidak memakai tabel partisi GPT, sehingga Trindle tidak dapat dipasang berdampingan dengan yang sudah ada di dalamnya.

# Red line at the foot: the boot partition on the disk is too small to share.
installer-boot-partition-small = Partisi boot di disk ini terlalu kecil untuk dipakai bersama, sehingga Trindle tidak dapat dipasang berdampingan dengan yang sudah ada di dalamnya.

# Note on the last page: the swap file was not made, with why.
installer-swap-failed = Trindle sudah terpasang, tetapi tidak dapat membuat file swap: { $why }

# Red line at the foot: a partition's identity could not be read after it was made.
installer-uuid-missing = Trindle tidak dapat mengenali { $dev } setelah menyiapkannya.

# Words on the last page: the disk that will be replaced, with what it holds.
installer-ready-replaced-known = Semua isi { $disk } akan diganti ({ $holds }).

# A disk named with its model and size, then its place such as /dev/sda.
installer-disk-named = { $what }, { $dev }

# Red line at the foot: a program the installer runs stopped before it had all it was given.
installer-run-stopped = { $program } berhenti sebelum mendapat semua yang dibutuhkannya: { $why }

# Red words on the last page: the TPM chip could not be set up to open the disk, so the disk password is asked for at every start.
installer-chip-not-set-up = Chip TPM tidak dapat disiapkan, jadi disk akan meminta kata sandinya setiap kali komputer dinyalakan.

# Red words on the last page: the keyboard for the disk password screen could not be set, so it may be the American one.
installer-start-keyboard-failed = Trindle sudah terpasang, tetapi layar kata sandi disk mungkin memakai keyboard Amerika.

# Red words on the last page: the login screen could not be switched on; { $why } is the reason.
installer-login-screen-failed = Trindle sudah terpasang, tetapi layar masuk tidak dapat diaktifkan: { $why }

# Red words on the last page: Trindle could not be set to take its own updates; { $why } is the reason.
installer-updates-held = Trindle sudah terpasang, tetapi belum akan menerima pembaruannya sendiri: { $why }

# Red words on the last page: earlier versions of people's files (home snapshots) could not be switched on.
installer-home-snapshots-failed = Trindle sudah terpasang, tetapi versi file Anda yang lebih lama tidak aktif. Anda dapat mengaktifkannya di Pengaturan, Snapshot.

# Red words on the last page: Trindle could not be added to the computer's own start-up list; { $why } is the reason.
installer-firmware-entry-failed = Trindle sudah terpasang, tetapi tidak dapat menambahkan dirinya ke daftar mulai komputer: { $why }

# Red line: the account the stick ran as could not be removed from the new system, so the install stopped.
installer-live-account-left = Akun stik { $user } tidak dapat dihapus dari sistem baru.

# Under the swap file's switch when a size smaller than memory is chosen.
installer-swap-no-hibernate = Lebih kecil dari memori komputer ini, sehingga tidak dapat hibernasi.

# In the swap size list: the size of the computer's memory, which hibernating needs.
installer-swap-size-hibernate = { $gb } GB, cukup untuk hibernasi

# In the swap size list: a smaller size.
installer-swap-size = { $gb } GB

# Tip of the swap size list.
installer-swap-size-tip = Ukuran file swap

# Why logind holds off sleep and shutting down while the installer changes the disk (shown by systemd-inhibit --list).
installer-power-held = Trindle sedang dipasang
