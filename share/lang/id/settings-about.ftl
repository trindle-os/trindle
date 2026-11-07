
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Tentang komputer ini

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space nama komputer nama jaringan tentang sistem versi memori prosesor grafis layar aktivitas program berjalan sibuk pemantau sistem ruang

# Text behind the info button of the About page.
settings-about-help = Nama komputer ini seperti yang dilihat komputer lain di jaringan, bahan penyusunnya, dan berapa lama waktu yang dibutuhkan untuk menyala. Apa yang sedang dikerjakannya, serta program mana yang memakai memori dan prosesor, ada di Aktivitas, sedangkan drive dan ruangnya ada di Drive, masing-masing di jendelanya sendiri: tanda panah di Detail lainnya membukanya.

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
settings-about-name-bad = Hanya huruf, angka, dan tanda hubung, tidak diawali atau diakhiri tanda hubung.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Menunggu kata sandi...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Nama komputer tidak dapat diganti.

# Words under the computer name box after renaming worked.
settings-about-name-done = Program yang sudah terbuka melihat nama baru setelah login berikutnya.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nama komputer

# Tip of the save button beside the computer name box.
settings-about-rename = Ganti nama komputer ini

# Bold line of the computer name setting.
settings-about-name = Nama komputer

# Quiet line under the computer name setting.
settings-about-name-sub = Huruf, angka, dan tanda hubung.

# Heading of the card with the computer's name.
settings-about-name-group = Nama

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Cara komputer ini muncul di jaringan, dan di prompt pada Terminal. Huruf, angka, dan tanda hubung, tidak diawali atau diakhiri tanda hubung. Mengubahnya meminta kata sandi Anda.

# Heading of the card about the computer's hardware.
settings-about-hardware = Komputer

# Text behind the info button of the hardware card.
settings-about-hardware-about = Pembuat dan model, prosesor dan berapa thread yang dijalankannya sekaligus, grafis, dan setiap layar yang dipakai beserta ukuran dan laju penyegarannya.

# Quiet line under the maker and model.
settings-about-hw-computer = Komputer

# Quiet line under the processor.
settings-about-hw-processor = Prosesor

# Quiet line under the graphics.
settings-about-hw-graphics = Menggambar tampilan

# Quiet line under the screens.
settings-about-hw-screen = Layar

# Heading of the card about the software.
settings-about-software = Perangkat lunak

# Text behind the info button of the software card.
settings-about-software-about = Trindle adalah desktop di atas Debian, yang mengurus program di bawahnya. Kernel adalah bagian yang berbicara dengan perangkat keras, dan compositor milik Trindle menggambar desktop, di Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Berjalan di bawahnya

# Quiet line under the kernel version.
settings-about-sw-kernel = Kernel

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Compositor milik Trindle

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Menggambar desktop, di Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] { $n } aplikasi
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Terpasang, siap di ring

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Detail lainnya

# Text behind the info button of the More detail card.
settings-about-more-about = Apa yang sedang dikerjakannya, serta program mana yang memakai memori dan prosesor, ada di Aktivitas. Setiap drive, seberapa penuh dan sehatnya, serta USB, ada di Drive.

# Bold line of the setting that opens Activity.
settings-about-activity = Lihat apa yang berjalan

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Program yang memakai memori dan prosesor.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Aktivitas

# Bold line of the setting that opens Drives.
settings-about-drives = Drive dan ruang

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Setiap drive, seberapa penuh dan sehatnya.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Drive

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Waktu menyala tidak dapat disimpan ke Unduhan.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Di Unduhan, sebagai { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Simpan

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Simpan semua waktu menyala ke Unduhan

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Setiap kali menyala, untuk disimpan

# Quiet line under that setting.
settings-about-times-keep-sub = Disimpan ke Unduhan sebagai spreadsheet.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Setiap kolom adalah satu kali menyala, yang terbaru di kanan.

# Heading of the card with the last ten start-up times.
settings-about-times = Sepuluh kali menyala terakhir

# Text behind the info button of the start-up times card.
settings-about-times-about = Setiap kali menyala dihitung dari tombol daya sampai layar login, tanpa waktu untuk mengetik kata sandi disk. Yang terbaru ada di atas.

# Heading of the memory tile.
settings-about-memory = Memori

# Heading of the disk tile.
settings-about-disk = Disk


# Heading of the card with the switch for looking for updates.
settings-about-updates = Pembaruan

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle mencari versi baru aplikasi, plugin, firmware, dan dirinya sendiri tak lama setelah Anda login dan setiap beberapa jam, dan perbaikan keamanan Debian masuk setiap hari. Jika dimatikan, Trindle hanya mencari saat Anda menekan Periksa pembaruan di Aplikasi, atau tombol pembaruan di ring. Perbaikan keamanan Debian punya sakelar sendiri; jika dimatikan, perbaikan itu tidak lagi terpasang sendiri dan celah yang sudah dikenal tetap terbuka sampai Anda memperbarui. Mengubah salah satu sakelar meminta kata sandi administrator Anda, agar layanan komputer itu sendiri juga mengetahui pilihan tersebut.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Periksa pembaruan

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Menunggu kata sandi...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Mati, tidak ada yang mencari kecuali Anda minta.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Pembaruan keamanan

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Perbaikan keamanan terpasang sendiri setiap hari.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Perbaikan keamanan tidak lagi terpasang sendiri.

