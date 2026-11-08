
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
store-deb-trindle = Ini adalah Trindle sendiri. Trindle diperbarui dari halaman Pembaruan di Aplikasi, bukan dari file.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } adalah driver atau layanan latar belakang, yang tidak dapat berfungsi dari foldernya sendiri, jadi tidak dapat dipasang di sini.

# SAFETY: needs a human review before it ships
# Words on the caution card above every package.
store-deb-caution = Paket dari luar Debian dan Trindle dipasang di foldernya sendiri, tempat paket itu dibatasi tetapi tidak sepenuhnya terkurung. Pasanglah hanya paket dari pembuat yang Anda percaya.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } terpasang di foldernya sendiri.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Memeriksa asal file ini…

# Heading in the card that says where the file came from.
store-deb-where = Asalnya

# SAFETY: needs a human review before it ships
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = Hapus paket beserta yang disimpannya

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
store-deb-goes-in = Dipasang di foldernya sendiri, dibatasi secara bawaan

# SAFETY: needs a human review before it ships
# Under that heading.
store-deb-goes-in-note = Tidak perlu kata sandi. Paket tidak dapat menjangkau internet, suara, atau file Anda sampai Anda mengizinkannya.

# Heading of the card for a package installed at this version.
store-deb-same-version = Versi { $version } terpasang di foldernya sendiri

# Under that heading.
store-deb-same-version-note = Pasang ulang di sana jika paket berhenti berfungsi dengan benar.

# Heading of the card for a package installed at another version.
store-deb-other-version = Versi { $version } terpasang di foldernya sendiri

# Under that heading: the version this brings.
store-deb-other-version-note = Memasang ini di sana akan memperbaruinya ke versi { $version }, dengan tetap menyimpan data yang sudah disimpannya.

# Facts line: how big the package is once installed.
store-deb-fact-size = Ukuran setelah terpasang

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Dibuat untuk

# Text of the About card when the package has no description.
store-deb-no-description = Paket ini tidak memiliki deskripsi.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Memasang di foldernya sendiri. Mengambil yang dibutuhkannya dapat memakan waktu beberapa menit.

# Words at the top of the window once the package is installed.
store-deb-done = Terpasang di foldernya sendiri. Anda akan menemukannya di ring.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Terpasang di foldernya sendiri, tetapi bagian berikut tidak ditemukan, jadi mungkin tidak dapat dimulai: { $parts }.

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
store-loose-not-boxed = Program itu tidak disimpan dalam kotak.

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
store-boxed-terminal-told = Terminal dapat menjangkau lebih banyak isi komputer Anda daripada aplikasi lain, karena pekerjaannya memerlukan itu. Ketik hanya perintah yang Anda pahami, dari orang yang Anda percaya.

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = Jaringan rumah mati

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } tidak memiliki jaringan: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = tidak dapat menemukan programnya sendiri

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare belum terpasang

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta belum terpasang

# The same: the network room was not made.
store-boxed-home-why-not-made = ruang jaringannya tidak dibuat

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = ruang jaringannya bukan yang baru

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = filter panggilan sistem tidak dapat dibuat

# The same: the room's rules would not load.
store-boxed-home-why-rules = aturan ruang itu tidak dapat dimuat

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } tidak dapat dimulai: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta menyatakan: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta tidak dapat bergabung ke ruang itu


# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Dibuka tanpa internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = File ini diunduh, jadi program hanya mendapat file itu, hanya baca.

