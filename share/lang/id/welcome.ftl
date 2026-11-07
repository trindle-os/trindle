
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Selamat datang di Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Buka ring

# Title of the third page, where the first apps are picked.
welcome-page-apps = Pilih aplikasi Anda

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Lebih banyak pilihan

# Title of the fifth page, about plugins.
welcome-page-plugins = Plugin menambah fitur

# Title of the sixth page, about crash reports.
welcome-page-reports = Bantu perbaiki Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Titik-titik

# Title of the last page, with a few things worth knowing.
welcome-page-know = Perlu diketahui

# Sentence under the title of the first page.
welcome-sentence-hello = Semua yang Anda perlukan ada dalam satu lingkaran, yaitu ring.

# Sentence under the title of the second page.
welcome-sentence-ring = Klik dua kali tepat di tepi layar, atau tekan tombol Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Hampir belum ada yang terpasang, jadi pilih aplikasi yang Anda mau.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Pilih lebih banyak, lalu pasang semua yang Anda pilih dengan satu klik.

# Sentence under the title of the fifth page.
welcome-sentence-plugins = Plugin adalah tambahan kecil dari Aplikasi, dan masing-masing hanya melakukan apa yang Anda izinkan.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Kirim laporan saat ada yang salah, jika Anda mau.

# Sentence under the title of the dots page.
welcome-sentence-dots = Satu titik berarti sebuah program sedang memakai sesuatu milik Anda.

# Sentence under the title of the last page.
welcome-sentence-know = Beberapa hal yang membuat Trindle berbeda.

# Top right of every page: where the person is in the tour.
welcome-step = Langkah { $n } dari { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Langkah { $n } dari { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
# SAFETY: needs a human review before it ships
welcome-done-file-note = Sambutan sudah dilihat. Ada di Semua aplikasi sebagai Welcome; hapus file ini untuk melihatnya lagi saat login berikutnya.

# Name of the round button at the foot that closes the tour.
welcome-close = Tutup

# Name of the round button at the foot that goes to the page before.
welcome-back = Kembali

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Lanjut

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Pasang aplikasi yang dipilih

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Mulai memakai Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Sambungkan ke internet untuk memasang aplikasi.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Coba lagi

# Foot while apps are going in: their names.
welcome-installing = Memasang { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Memasang { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
       *[other] { $names } sekarang ada di ring.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Aplikasi tidak dapat dipasang. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak menolak.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Dipilih: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } dan { $more } lainnya

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = Ring

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Isinya

# Name of the first thing the ring holds.
welcome-held-programs = Program Anda

# Line under it.
welcome-held-programs-line = Berada di sekelilingnya.

# Name of the second thing the ring holds.
welcome-held-time = Waktu dan tanggal

# Line under it.
welcome-held-time-line = Berada di tengah.

# Name of the third thing the ring holds.
welcome-held-switches = Sakelar

# Line under it.
welcome-held-switches-line = Suara, layar, tetap menyala, dan daya.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Dengan penunjuk

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Dengan papan ketik

# Note under the picture of the Super key.
welcome-ring-super-note = Tombol Super sering bergambar logo Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Peramban web

# Head of a card of apps: email programs.
welcome-kind-email = Email

# Head of a card of apps: chat programs.
welcome-kind-chat = Obrolan

# Head of a card of apps: office programs.
welcome-kind-office = Perkantoran

# Head of a card of apps: music and video players.
welcome-kind-media = Musik dan video

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Gambar dan menggambar

# Head of a card of apps: notes and writing.
welcome-kind-notes = Catatan dan menulis

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF dan dokumen

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Tidak satu pun ada di Flathub saat ini.

# Beside an app that has been installed.
welcome-app-installed = Terpasang

# Beside an app that is going in now.
welcome-app-installing = Memasang

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, dipilih

# Name of the round button on an app's row that picks it.
welcome-app-pick = Pilih { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = Di ring

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Plugin

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = Di Aplikasi, di bawah Plugin.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Buka plugin di Aplikasi

# Said in the plugins card while there are none to show.
welcome-plugins-none = Plugin yang tersedia tercantum di Aplikasi.

# Head of the card on the crash reports page.
welcome-reports-card = Laporan kerusakan

# The one question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-question = Bantu perbaiki Trindle dengan mengirim laporan saat ada yang salah? Anda selalu melihatnya lebih dahulu.

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = Tidak ada yang dikirim tanpa Anda menekan tombol. Anda melihat seluruh laporan lebih dahulu, dan laporan tidak berisi file, kata sandi, atau nama Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = Anda dapat mengubahnya kapan saja di Pengaturan, Laporan kerusakan.

# The answer that turns crash reports off.
# SAFETY: needs a human review before it ships
welcome-reports-no = Tidak

# The answer that turns crash reports on.
# SAFETY: needs a human review before it ships
welcome-reports-yes = Ya, tanya saya

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Kirim laporan saat ada yang salah

# Head of a card on the last page.
welcome-know-boxes = Aplikasi hidup di kotaknya sendiri

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Aplikasi bertanya sebelum menjangkau file, kamera, atau mikrofon Anda.

# Head of a card on the last page.
welcome-know-updates = Pembaruan keamanan terpasang sendiri

# Line in that card.
welcome-know-updates-line = Setiap hari, tanpa bertanya. Pembaruan lain menunggu Anda.

# Head of a card on the last page.
welcome-know-ring = Pengaturan dan Aplikasi ada di ring

# Line in that card.
welcome-know-ring-line = Di tengahnya, di atas waktu.

# Head of a card on the last page.
welcome-know-help = Bantuan ada di Pengaturan

# Line in that card.
welcome-know-help-line = Tur ini ada di Semua aplikasi sebagai Welcome, kapan pun Anda mau lagi.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Layar kecil tanpa apa pun selain ring di tengah: waktu di bagian tengahnya, aplikasi favorit di sekelilingnya.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = Ring terbuka di layar kecil, dengan ikon beberapa plugin berjajar di tengahnya, di bawah waktu, tempat ikon plugin berada setelah dipasang.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Dua gambar kecil tempat titik muncul: pada tab jendela di cincin, dan di tengah atas layar.

welcome-dots-where-tab = Pada tab jendela

welcome-dots-where-corner = Tengah atas layar

welcome-picture-ring = Layar kecil. Penunjuk bergerak ke tepi paling atas dan mengklik dua kali, lalu ring terbuka di tengah.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Tombol Super ditekan, yang membuka ring.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Seberapa banyak yang Anda ketahui?

# Sentence under that title.
welcome-sentence-level = Pilih yang paling dekat, dan tur akan melewatkan apa yang sudah Anda tahu.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Baru mengenal Linux

# Line in that answer's card.
welcome-level-new-line = Setiap halaman, termasuk apa yang dilakukan desktop Linux mana pun.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Pernah memakai Linux

# Line in that answer's card.
welcome-level-linux-line = Hanya yang khas Trindle. Dasar-dasar Linux dilewati.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Pernah memakai Trindle

# Line in that answer's card.
welcome-level-trindle-line = Menutup tur sekarang dan tidak menampilkannya lagi.

# Title of the page about files and folders.
welcome-page-files = File Anda

# Sentence under that title.
welcome-sentence-files = Semua milik Anda ada di folder rumah Anda.

# Title of the page about getting software.
welcome-page-software = Mendapatkan perangkat lunak

# Sentence under that title.
welcome-sentence-software = Aplikasi datang dari Aplikasi, bukan dari file penyiapan yang Anda unduh.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = Kata sandi administrator

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Perubahan pada seluruh komputer meminta kata sandi terlebih dahulu.

# Title of the page about snapshots.
welcome-page-snapshots = Snapshot

# Sentence under that title.
welcome-sentence-snapshots = Trindle menyimpan salinan sistem, jadi pembaruan bisa dibatalkan.

# Title of the page about the person's own words.
welcome-page-words = Kata-kata Anda sendiri

# Sentence under that title.
welcome-sentence-words = Ubah kata apa pun yang ditampilkan Trindle, atau tambahkan bahasa Anda sendiri.

# Head of a card on the files page.
welcome-files-home = Folder rumah

# Line in that card.
welcome-files-home-line = Dokumen, Unduhan, dan Gambar ada di dalamnya. Folder terbuka di sana.

# Head of a card on the files page.
welcome-files-drives = Tanpa huruf drive

# Line in that card.
welcome-files-drives-line = Flash disk USB atau disk lain muncul di Folder saat Anda menancapkannya.

# Head of a card on the files page.
welcome-files-hidden = Nama tersembunyi

# Line in that card.
welcome-files-hidden-line = Nama yang diawali titik, seperti .config, disembunyikan. Isinya pengaturan sebuah aplikasi.

# Head of a card on the files page.
# SAFETY: needs a human review before it ships
welcome-files-bin = Tempat sampah

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = File yang dihapus menunggu di tempat sampah sampai Anda mengosongkannya.

# Head of a card on the software page.
welcome-software-apps = Satu tempat untuk aplikasi

# Line in that card.
welcome-software-apps-line = Buka Aplikasi dari cincin untuk mencari, memasang, dan menghapus program.

# Head of a card on the software page.
welcome-software-exe = Tanpa file penyiapan

# Line in that card.
welcome-software-exe-line = File penyiapan Windows berakhiran .exe tidak berjalan di sini. Cari aplikasinya di Aplikasi.

# Head of a card on the software page.
welcome-software-updates = Pembaruan di satu tempat

# Line in that card.
welcome-software-updates-line = Aplikasi dan Pengaturan menjaga semuanya tetap mutakhir, jadi tidak ada aplikasi yang butuh pembaru sendiri.

# Head of a card on the software page.
# SAFETY: needs a human review before it ships
welcome-software-remove = Penghapusan yang bersih

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Menghapus aplikasi membuang seluruhnya dan tidak meninggalkan sisa.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Mengapa diminta

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Memasang untuk semua orang atau mengubah disk memengaruhi setiap pengguna, jadi komputer bertanya dahulu.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = Kata sandi siapa

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = Kata sandi seorang administrator, biasanya yang Anda pakai untuk masuk.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Kapan tidak diminta

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Aplikasi yang Anda pasang untuk diri sendiri dan file Anda sendiri tidak pernah memerlukannya.

# Head of a card on the snapshots page.
welcome-snap-taken = Dibuat untuk Anda

# Line in that card.
welcome-snap-taken-line = Pada sistem Btrfs, satu snapshot dibuat sebelum dan sesudah setiap perubahan perangkat lunak.

# Head of a card on the snapshots page.
welcome-snap-list = Terdaftar di Pengaturan

# Line in that card.
welcome-snap-list-line = Pengaturan, Snapshot mendaftarkannya. Kamera membuat satu sekarang.

# Head of a card on the snapshots page.
welcome-snap-boot = Dipakai jika tidak mau menyala

# Line in that card.
welcome-snap-boot-line = Pilih snapshot Trindle di menu boot, lalu sebuah snapshot.

# Head of a card on the your own words page.
welcome-words-change = Ubah sebuah kata

# Line in that card.
welcome-words-change-line = Temukan kata apa pun yang ditampilkan Trindle dan ketik kata Anda sendiri. "Buka, brew" bisa menggantikan "Klik di sini untuk membuka file".

# Head of a card on the your own words page.
# SAFETY: needs a human review before it ships
welcome-words-share = Bagikan satu lembar

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-words-share-line = Ekspor kata-kata Anda sebagai file, atau impor yang dikirim teman. File diperiksa sebelum dipakai.

# Head of a card on the your own words page.
welcome-words-all = Semua program sekaligus

# Line in that card.
welcome-words-all-line = Seluruh Trindle membaca kata-katanya dari satu tempat, jadi satu perubahan terlihat di mana-mana.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Ada di Pengaturan, Wilayah dan bahasa. Bantuan menjelaskannya langkah demi langkah.

# Name of the round button that opens that Settings page.
welcome-words-open = Buka Wilayah dan bahasa
