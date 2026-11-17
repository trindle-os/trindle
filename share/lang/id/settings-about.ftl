
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Tentang komputer ini

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space nama komputer nama jaringan tentang sistem versi memori prosesor grafis layar aktivitas program berjalan sibuk pemantau sistem ruang

# Text behind the info button of the About page.
settings-about-help = Halaman ini menampilkan nama komputer ini seperti yang dilihat komputer lain di jaringan, bahan penyusunnya, dan berapa lama waktu yang dibutuhkan untuk menyala. Aktivitas menampilkan apa yang sedang dikerjakan komputer dan program mana yang memakai memori dan prosesor. Drive menampilkan drive Anda dan ruangnya. Masing-masing dibuka di jendelanya sendiri. Gunakan tanda panah di Detail lainnya untuk membukanya.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Tidak diketahui

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
       *[other] { $model }, { $cores } thread
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Grafis Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Grafis AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Grafis NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Grafis mesin virtual

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Grafis

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } pada { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } terpakai

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } kosong

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = dari { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, menyala selama { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, menyala selama { $on }. Disk hampir penuh.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } detik

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } menit

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
       *[other] { $n } menit
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
       *[other] { $n } jam
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
       *[other] { $n } hari
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Gunakan hanya huruf, angka, dan tanda hubung. Jangan diawali atau diakhiri tanda hubung.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Menunggu kata sandi Anda…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Tidak dapat mengganti nama komputer.

# Words under the computer name box after renaming worked.
settings-about-name-done = Program yang sudah terbuka akan memakai nama baru setelah Anda login berikutnya.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nama komputer

# Tip of the save button beside the computer name box.
settings-about-rename = Ganti nama komputer ini

# Bold line of the computer name setting.
settings-about-name = Nama komputer

# Heading of the card with the computer's name.
settings-about-name-group = Nama

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Ini adalah nama komputer di jaringan dan di prompt pada Terminal. Gunakan huruf, angka, dan tanda hubung, tetapi jangan diawali atau diakhiri tanda hubung. Anda perlu memasukkan kata sandi untuk mengubahnya.

# Heading of the card about the computer's hardware.
settings-about-hardware = Komputer

# Text behind the info button of the hardware card.
settings-about-hardware-about = Bagian ini menampilkan pembuat dan model komputer, prosesornya, dan grafisnya. Bagian ini juga menampilkan berapa thread yang dapat dijalankan prosesor sekaligus, serta setiap layar yang dipakai beserta ukuran dan laju penyegarannya.

# Heading of the card about the software.
settings-about-software = Perangkat lunak

# Text behind the info button of the software card.
settings-about-software-about = Trindle adalah desktop yang berjalan di atas Debian, yang mengurus program di bawahnya. Kernel adalah bagian yang bekerja dengan perangkat keras. Compositor milik Trindle menggambar desktop, memakai Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Compositor milik Trindle

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] { $n } aplikasi
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Detail lainnya

# Text behind the info button of the More detail card.
settings-about-more-about = Aktivitas menampilkan apa yang sedang dikerjakan komputer dan program mana yang memakai memori dan prosesor. Drive menampilkan setiap drive, seberapa penuh dan sehatnya, serta USB.

# Bold line of the setting that opens Activity.
settings-about-activity = Lihat apa yang berjalan

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Aktivitas

# Bold line of the setting that opens Drives.
settings-about-drives = Drive dan ruang

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Drive

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle tidak dapat menyimpan waktu menyala ke Unduhan.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Tanggal dan waktu,Detik

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Waktu mulai Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Di Unduhan, sebagai { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Simpan

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Simpan semua waktu menyala ke Unduhan

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Simpan setiap waktu menyala

# Quiet line under that setting.
settings-about-times-keep-sub = Disimpan ke Unduhan sebagai spreadsheet.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Setiap kolom adalah satu kali menyala. Yang terbaru di kanan.

# Heading of the card with the last ten start-up times.
settings-about-times = Sepuluh waktu menyala terakhir

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle menghitung setiap waktu menyala dari tombol daya sampai layar login. Waktu untuk mengetik kata sandi disk tidak dihitung. Yang terbaru ada di atas.

# Heading of the memory tile.
settings-about-memory = Memori

# Heading of the disk tile.
settings-about-disk = Disk

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Laporan masalah

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Laporan dikirim ke Buildmorphic, komunitas yang membantu membangun Trindle. Anda dapat bergabung hanya dengan alamat email. Anda mendapat kredit dan level untuk masalah yang pertama kali Anda laporkan, meski tidak bisa memperbaikinya. Laporan hanya berisi daftar detail yang tetap, dan tidak ada yang dikirim sampai Anda menekan Kirim. Nama pengguna atau nama GitHub bersifat opsional, dan nama pengguna Anda tidak pernah berupa alamat email.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Tawarkan pengiriman laporan saat ada yang salah

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Nama pengguna Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = Nama GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Tentang komunitas

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
       *[other] Mati. { $n } laporan yang menunggu dihapus.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
       *[other] Mati. { $n } laporan yang menunggu tidak dapat dihapus.
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = Pembaruan

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle memeriksa versi baru aplikasi, plugin, firmware, dan dirinya sendiri tak lama setelah Anda login, lalu setiap beberapa jam. Jika Anda mematikannya, Trindle hanya memeriksa saat Anda menekan Periksa pembaruan di Aplikasi, atau tombol pembaruan di ring. Perbaikan keamanan Debian dipasang setiap hari dan punya sakelar sendiri. Jika Anda mematikannya, perbaikan itu tidak lagi terpasang sendiri, dan celah keamanan yang sudah dikenal tetap terbuka sampai Anda memperbarui. Mengubah salah satu sakelar membutuhkan kata sandi administrator Anda, agar seluruh komputer memakai pilihan Anda.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Periksa pembaruan

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Menunggu kata sandi Anda…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Pembaruan keamanan

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Perbaikan keamanan tidak lagi terpasang sendiri.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Mati, tetapi laporan yang menunggu di add-on Reporting tidak dihapus.
