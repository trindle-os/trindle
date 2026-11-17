
# machine translated (Indonesian), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Pasang paket

# Words at the top of the window while the package is read.
store-deb-reading = Membaca paket…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = File ini bukan paket Debian yang dapat dipasang. Mungkin unduhannya belum selesai.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } siap dipasang di foldernya sendiri.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Paket ini untuk komputer { $arch }, jadi tidak dapat dipasang di komputer ini.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Ini adalah Trindle sendiri. Perbarui dari halaman Pembaruan di Aplikasi, bukan dari file.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } adalah driver atau layanan latar belakang. Paket ini tidak dapat berjalan dari foldernya sendiri, jadi tidak dapat dipasang di sini.

# SAFETY: needs a human review before it ships
# Words on the caution card above every package.
store-deb-caution = Paket dari luar Debian dan Trindle dipasang di foldernya sendiri. Di sana paket dibatasi, tetapi tidak sepenuhnya terkurung. Pasang hanya paket dari pembuat yang Anda percaya.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } terpasang di foldernya sendiri.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Memeriksa asal file ini…

# Heading in the card that says where the file came from.
store-deb-where = Asalnya

# SAFETY: needs a human review before it ships
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = Hapus paket beserta datanya

# Word beside the ring while the package goes in.
store-deb-installing = Memasang

# SAFETY: needs a human review before it ships
# Main button that installs the package.
store-deb-install = Pasang

# SAFETY: needs a human review before it ships
# Main button that installs the same version again.
store-deb-reinstall = Pasang ulang

# SAFETY: needs a human review before it ships
# Main button that brings the installed package up to this version.
store-deb-update = Perbarui

# SAFETY: needs a human review before it ships
# Heading of the card for a package not installed yet.
store-deb-goes-in = Dipasang di folder tersendiri yang dibatasi

# SAFETY: needs a human review before it ships
# Under that heading.
store-deb-goes-in-note = Tidak perlu kata sandi. Paket tidak dapat memakai internet atau suara, atau melihat file Anda, sampai Anda mengizinkannya.

# Heading of the card for a package installed at this version.
store-deb-same-version = Versi { $version } terpasang di foldernya sendiri

# Heading of the card for a package installed at another version.
store-deb-other-version = Versi { $version } terpasang di foldernya sendiri

# Under that heading: the version this brings.
store-deb-other-version-note = Ini memperbarui ke versi { $version } dan tetap menyimpan data yang sudah disimpannya.

# Facts line: how big the package is once installed.
store-deb-fact-size = Ukuran setelah terpasang

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Dibuat untuk

# Text of the About card when the package has no description.
store-deb-no-description = Paket ini tidak memiliki deskripsi.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Memasang di foldernya sendiri. Mengunduh bagian yang dibutuhkannya dapat memakan waktu beberapa menit.

# Words at the top of the window once the package is installed.
store-deb-done = Terpasang di foldernya sendiri. Anda akan menemukannya di ring.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Terpasang di foldernya sendiri, tetapi bagian berikut tidak ada, jadi mungkin tidak dapat dimulai: { $parts }.

# SAFETY: needs a human review before it ships
# Words at the top of the window once the package is removed.
store-deb-removed = Dihapus dari foldernya sendiri, beserta semua yang disimpannya.

# Red words when a file is not a Debian package.
store-boxed-not-deb = File ini bukan paket Debian yang dapat dipasang.

# Step shown while a package is unpacked.
store-boxed-unpacking = Mengekstrak { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = Menambahkan { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } bukan paket Debian yang dapat dipasang.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } tidak dapat diekstrak.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Program itu tidak berjalan dalam kotak.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal

# Title of the notice when older programs are off.
store-boxed-x-off = Program lama dimatikan

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Pengaturan, Izin

# Title of the notice when too many older programs are open.
store-boxed-x-many = Terlalu banyak program lama

# Body of that notice.
store-boxed-x-many-close = Tutup satu lalu coba lagi

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } tidak dapat dibuka

# SAFETY: needs a human review before it ships
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = Terminal dapat menjangkau lebih banyak isi komputer Anda daripada aplikasi lain. Ketik hanya perintah yang Anda pahami, dari orang yang Anda percaya.

# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = Tanpa jaringan lokal

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } tidak memiliki jaringan: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = program Trindle tidak ditemukan

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare belum terpasang

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta belum terpasang

# The same: the network room was not made.
store-boxed-home-why-not-made = jaringannya tidak disiapkan

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = jaringannya sudah ada

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle tidak dapat menyiapkan filter keamanan

# The same: the room's rules would not load.
store-boxed-home-why-rules = aturan jaringannya tidak dapat dimuat

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } tidak dapat dimulai: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta menyatakan: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta tidak dapat menyambungkannya

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = Folder teratas { $name } adalah tautan, jadi paket ini tidak dapat dipakai.

# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } adalah driver atau layanan latar belakang. Yang seperti itu tidak dapat berfungsi dari folder sendiri, jadi tidak dipasang.


# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Dibuka tanpa internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Karena diunduh, program hanya dapat membaca satu file itu.


# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = Program lama tidak dapat dibuka

# Body of that notice: what to do.
store-boxed-x-unasked-why = Keluar lalu masuk lagi, kemudian coba lagi
