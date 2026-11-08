# machine translated (Indonesian), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
       *[other] { $count } aplikasi
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
       *[other] { $count } pembaruan menunggu
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } untuk dipilih di Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = semua milik Anda sudah terbaru

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Tidak ada aplikasi yang cocok dengan “{ $text }”.

# Line under the title of an empty page of apps.
store-grid-none-here = Belum ada aplikasi di sini.

# Advice in the empty page of a search.
store-grid-try-shorter = Coba kata yang lebih pendek, atau telusuri satu jenis di sebelah kiri.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Cari berdasarkan nama di atas, atau telusuri jenis lain di sebelah kiri.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Menampilkan { $shown } dari { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = Tampilkan { $next } lagi

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Mengambil daftar aplikasi dari Flathub. Pertama kali memakan waktu semenit.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Apps tidak dapat menjangkau Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Komputer ini tidak terhubung ke internet. Apps mencoba lagi sendiri begitu terhubung.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Aplikasi yang dapat dipilih berasal dari Flathub, lewat internet.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } dan pengaturan aplikasi, di sebelah kiri, tetap berfungsi tanpanya.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } dan pengaturan aplikasi, di sebelah kiri, tetap berfungsi tanpa Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } dan pengaturan aplikasi, di sebelah kiri, tetap berfungsi tanpa internet.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Coba lagi

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Hasil

# Red words under the search box when no web browser opens a link.
store-no-browser = Tidak ada peramban web untuk membuka { $url }. Pasang satu dahulu di Apps.

# Said in the card or under the title while the lists are being read.
store-looking = Mencari…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Sekarang ada di Terpasang, dan di cincin

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Dihapus

# The word on an app's dial when installing or removing did not work.
store-done-not = Gagal

# The word on an app's dial when it was installed.
store-done-installed = Terpasang

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak menolak.

# Tab of the Installed page that shows everything.
store-tab-all = Semua

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Berkas Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Berkas deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Plugin

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Belum ada yang terpasang. Temukan penuh dengan hal untuk dicoba.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
       *[other] { $n } plugin terpasang
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
       *[other] { $n } program terpasang
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } hal terpasang

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [1] { $start }, dari Flathub
       *[other] { $start }, semuanya dari Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [1] { $start }, dari berkas Flatpak
       *[other] { $start }, semuanya dari berkas Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [1] { $start }, sebuah AppImage
       *[other] { $start }, semuanya AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [1] { $start }, dari berkas .deb
       *[other] { $start }, semuanya dari berkas .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } dari Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
       *[other] { $n } dari berkas Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
       *[other] { $n } AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
       *[other] { $n } aplikasi
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
       *[other] { $n } dari berkas .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
       *[other] { $n } plugin
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Sudah terbaru.

# End of the Installed line when everything is up to date.
store-installed-state-all = Semuanya sudah terbaru.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
       *[other] { $n } punya pembaruan yang menunggu.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } dari { $total } program cocok dengan “{ $find }”.

# The same when plugins are among the things.
store-installed-match-things = { $shown } dari { $total } hal cocok dengan “{ $find }”.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Tambah AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Cari yang terpasang

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage tidak dapat ditambahkan.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Aplikasi yang Anda pasang dari Temukan, dari GitHub atau sebagai AppImage, program dari berkas .deb, dan plugin muncul di sini.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Tidak ada yang terpasang yang cocok dengan “{ $find }”.

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = Hapus meminta dua kali: tekan lagi dalam tiga detik.

# On an installed thing's line: its version.
store-installed-version = versi { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = ditahan

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = ditahan; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Memperbarui { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Perbarui { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = Tentang

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = Menghapus...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Tidak ada bus sesi untuk menanyakan berkas yang mana.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Pilih AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Pemilih berkas tidak dapat ditanyai.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Menambahkan...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } sudah ada: lihat Terpasang.

# Words once an AppImage is in the ring.
store-add-in-ring = Sudah ditambahkan. Sekarang ada di daftar Anda.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Itu bukan referensi Flatpak yang dapat dipakai komputer ini.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Memasang { $name } dari { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } terpasang.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak tidak mau memasangnya.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } tidak dapat dipasang. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Mengganti...

# Heading of the question when an AppImage has the name of one already there.
# SAFETY: needs a human review before it ships
store-replace-head = { $name } sudah ada

# Small heading over the AppImage being brought.
store-replace-new = Yang dibawa

# Small heading over the AppImage already there.
store-replace-old = Yang sudah ada

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = Lewati: pertahankan yang sudah ada

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Ganti

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = Hapus { $name }?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Data tersimpannya memakai { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Tidak ada data tersimpan.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Menghitung data tersimpannya...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Data yang dihapus tidak dapat dikembalikan.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Hapus, simpan datanya

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Hapus dan hapus datanya

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Hapus datanya saja

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Tidak ada data tersimpan untuk dihapus.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Batal: pertahankan keduanya

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Data tersimpannya sudah dihapus.

# Words while only the data is being deleted.
store-flatpak-data-going = Menghapus data tersimpannya...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Tidak selesai, dan tidak memberi alasan.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Tampilkan letak masalahnya

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Cari di GitHub: ketik, lalu tekan Enter

# Heading of the card of matches from AppImageHub.
store-gh-hub = Di AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Tekan Enter untuk mencari juga di seluruh GitHub.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub tidak ditanyai: tidak ada internet untuk menanyainya.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Menanyai GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Tidak ada lagi di GitHub.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
       *[other] Mencari berkas AppImage dan Flatpak di { $n } proyek...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Tidak ada hasil di GitHub yang punya berkas AppImage atau Flatpak untuk komputer ini.

# Heading of the card of matches from GitHub.
store-gh-on = Di GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Cari di GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Aplikasi di GitHub sebagai berkas AppImage atau Flatpak. Tidak ada yang memeriksanya: pasang hanya dari orang yang Anda percaya.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = GitHub tidak dapat dicari selama komputer ini tidak terhubung ke internet.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak dan AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Lihat berkasnya

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = Pasang

# Link to a project's page on GitHub.
store-gh-open = Buka di GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, rilis { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = Kunci GitHub

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = Kunci GitHub: satu tersimpan

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Anda hampir pasti tidak memerlukannya. Tanpa kunci, GitHub menjawab 60 pertanyaan per jam dari komputer ini; dengan kunci, 5.000. Kunci hanya menaikkan batas itu. Kunci dipakai untuk melihat berkas publik dan tidak lebih: tidak memberi Trindle jalan masuk ke akun Anda, dan Anda tidak mencentang izin apa pun saat membuatnya. Kunci disimpan di komputer ini, di gantungan kunci Anda, dan hanya dikirim ke GitHub.

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = Buat kunci di GitHub, beri nama apa saja, biarkan semua izin, tekan Generate token, lalu tempel yang ditampilkan di sini.

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = Tidak dapat menghapusnya: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = Tidak dapat menyimpannya: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Kosongkan untuk mempertahankannya

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = Tempel kuncinya

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Hapus kunci

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Buat kunci di GitHub

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = Simpan kunci

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Tidak ada di rilis terbarunya yang dapat dipasang di sini.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Ditambahkan

# Line under the title of the Move your apps page.
store-bk-line = Aplikasi dan pengaturan Anda, dalam satu berkas untuk dibawa ke mana saja.

# Heading of the card that makes a backup.
store-bk-card = Cadangkan

# Tip of the round button while the backup is being made.
store-bk-making-tip = Membuat cadangan

# Main button that makes the backup.
store-bk-make = Buat cadangan

# Tip of the round button that shows the backup in Folders.
store-bk-show = Tampilkan di Folders

# Name of the switch for the list of apps in a backup.
store-bk-apps = Aplikasi

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
       *[other] Daftar { $n } aplikasi Flatpak Anda.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } Komputer lain memasangnya baru dari Flathub, jadi berkasnya tetap kecil.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Data milik aplikasi

# Under the Apps' own data switch.
store-bk-data-sub = Pengaturan dan hasil kerja tersimpan setiap aplikasi, tanpa cache-nya.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = Disimpan utuh, karena masing-masing satu berkas.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Pengaturan Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = Cincin, pilihan gambar latar, warna, papan ketik, daya, aplikasi bawaan, dan gambar Anda.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Sekarang { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Tujuannya

# Under that row: the folder and the file's name.
store-bk-where-sub = Folder beranda Anda, sebagai { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = Pulihkan cadangan

# Tip of the round button, and name of the row, that choose a backup file.
# SAFETY: needs a human review before it ships
store-bk-choose = Pilih berkas cadangan

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Cadangan Trindle, atau yang dibuat oleh { $flat }.

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = Aplikasinya dipasang dan datanya serta pengaturannya dikembalikan. Dokumen dan gambar Anda tidak termasuk di dalamnya: salin sendiri.

# Said while a backup is being made.
store-bk-making = Mencadangkan... ini bisa memakan beberapa menit.

# Said when a backup is made: the file and its size.
store-bk-made = Berkasnya { $path } ({ $size }). Salin ke flashdisk USB atau penyimpanan awan untuk dibawa.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Tidak ada cadangan yang dipilih, karena Trindle tidak punya sambungan ke pemilih berkas.

# Title of the window that asks which backup to use.
store-bk-choose-title = Pilih cadangan

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Cadangan Trindle dan Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = Tidak ada cadangan yang dipilih, karena pemilih berkas tidak menjawab.

# Said while a backup file is read.
store-bk-reading = Membaca cadangan...

# Said when a backup was put back.
store-bk-logout = Keluar lalu masuk kembali untuk melihat semua pengaturan.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Keluar lalu masuk kembali untuk melihat semua pengaturan.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
       *[other] { $apps } aplikasi, { $with } dengan pengaturannya. Memulai...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Belum ada sumber milik pembuat yang ditambahkan.
       *[other] { $n } sumber milik pembuat ditambahkan, dijaga tetap terbaru bersama pembaruan Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Ditambahkan, dan dijaga tetap terbaru.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. Sumber ditambahkan, tetapi programnya belum terpasang.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Dipasang dengan cara lain.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Hapus program dan sumbernya

# Link to a maker's page, naming the program.
store-src-open = Buka { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Masuk ke Tailscale

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = Tambah { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Siap ditambahkan

# Name of the field for the address of a source.
store-src-address = Alamat

# Under the address field.
store-src-address-sub = Alamat repositori, dari petunjuk pemasangan pembuatnya.

# Name of the field for the suite of a source.
store-src-suite = Suite

# Under the suite field.
store-src-suite-sub = Biasanya trixie atau stable.

# Name of the field for the components of a source.
store-src-components = Komponen

# Under the components field.
store-src-components-sub = Biasanya main.

# Name of the field for a source's signing key.
# SAFETY: needs a human review before it ships
store-src-key = Kunci penandatangan

# Under the signing key field.
store-src-key-sub = Alamat kunci penandatangannya.

# Name of the field for the program to install from a source.
store-src-program = Program yang dipasang

# Under the program field.
store-src-program-sub = Nama paketnya, jika Anda ingin segera memasangnya.

# Placeholder of the program field.
store-src-program-hint = opsional

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = Tambah sumber ini

# Heading of the card that adds another source.
# SAFETY: needs a human review before it ships
store-src-another = Tambah sumber lain

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Sebuah sumber dapat memasang pembaruan untuk seluruh komputer, jadi tambahkan hanya pembuat yang Anda percaya. Masing-masing meminta kata sandi administrator.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Hapus sumber ini

# Heading of the card of sources already on the computer.
store-src-already = Sudah ada di komputer ini

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Kedua alamat harus diawali https://

# Red words when the suite or components are not plain words.
store-src-bad-words = Suite dan komponen harus berupa kata biasa.

# Red words when the program is not a package name.
store-src-bad-package = Itu bukan nama paket.

# Word on a source's dial while its job starts.
store-src-starting = Memulai

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Membuka halaman masuk Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Masuk di halaman yang terbuka; ini selesai sendiri.

# Said under Tailscale's row when it is connected.
store-src-connected = Terhubung

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = Tautan ke aplikasinya tidak dienkripsi.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = Tidak dienkripsi

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Alamat ini tidak dienkripsi. Seseorang di tengah jalan dapat melihat apa yang dikirim, dan dapat menukar cadangan dengan yang lain.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Saya mengerti dan menerima risiko ini

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Pulihkan

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Batal

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Data tersimpannya tidak dapat dihitung.

store-hub-warning = AppImageHub hanya mendaftar proyek: tidak ada yang meninjau atau memindainya, dan tidak harus ditandatangani. Trindle menjalankan tiap AppImage di dalam kotak dengan folder home pribadi: awalnya ia boleh memakai internet dan suara, dan tidak melihat satu pun berkas Anda.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = Asal sebuah AppImage hanyalah petunjuk, bukan pemeriksaan.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = Di GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = Di GitHub: { $repo }, { $stars } bintang

# Where an AppImage comes from: a website, by its name.
store-from-site = Bukan di GitHub: dari situs web, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Asal tidak diketahui

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Buka situsnya

# When an app was last updated.
store-age-updated = Terakhir diperbarui: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Terakhir diperbarui: { $date }. Tidak diperbarui lebih dari enam bulan, jadi mungkin punya celah keamanan yang belum diperbaiki.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Terakhir diperbarui: tidak diketahui

# When an app's project was made.
store-age-created = Dibuat { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Dibuat: tidak diketahui

# A date the source does not give, on a line of Facts.
store-age-not-known = Tidak diketahui

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Sebelum Anda menambahkan satu

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
       *[other] { $count } AppImage terdaftar di AppImageHub.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Mengambil daftar AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = Daftar AppImageHub tidak dapat diambil.

# The same with no internet and no copy kept.
store-browse-offline = Daftar AppImageHub tidak dapat diambil selama komputer ini tidak terhubung ke internet.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub tidak mendaftar satu pun dari jenis ini.

# Under the rows: how many are shown of how many.
store-browse-showing = Menampilkan { $shown } dari { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Tambahkan ke aplikasi Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Lihat detail dan berkas



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = pemeriksaan pembaruan mati

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT kembali setelah OpenAI menandatangani unduhannya.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } untuk dipilih di Flathub dan AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } untuk dipilih di AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Keduanya

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage tidak diperiksa. Tidak ada yang meninjau atau memindainya.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle menjalankan setiap AppImage di dalam kotak dengan folder rumah pribadi: pada awalnya ia boleh memakai internet dan suara, dan tidak melihat satu pun file Anda.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Jangan tampilkan lagi

# Button on that card that goes on with the install.
store-appimage-warn-ok = OK

# Button on that card that gives the install up.
store-appimage-warn-cancel = Batal

