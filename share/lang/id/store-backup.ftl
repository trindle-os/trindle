
# machine translated (Indonesian), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
# SAFETY: needs a human review before it ships
store-refusal-nothing = Tidak ada yang dikembalikan, karena { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
# SAFETY: needs a human review before it ships
store-refusal-part = Sebagian cadangan tidak dikembalikan, karena { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = cadangan berisi tautan atau hal lain yang akan menjangkau keluar dari folder tujuannya

# The reason when something on this computer stands where part of the backup needs to go.
# SAFETY: needs a human review before it ships
store-because-in-the-way = sudah ada sesuatu di komputer ini di tempat yang diperlukan sebagian cadangan

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = tidak ada tempat untuk mengekstrak cadangan

# The reason when the backup file cannot be read.
store-because-unreadable = file cadangan tidak bisa dibaca

# The reason when the file is not a Trindle backup.
store-because-not-trindle = file itu bukan cadangan Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = cadangan rusak dan daftarnya tidak bisa dibaca

# The reason when the file is not a Flat backup.
store-because-not-flat = file itu tidak tampak seperti cadangan Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = file itu tidak berisi daftar aplikasi Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = ini dibuat oleh Flat yang lebih baru daripada yang bisa dibaca Trindle

# The reason when the file holds no apps.
store-because-no-apps = tidak ada aplikasi di file itu

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = cadangan tidak bisa dibaca: { $why }

# The reason when the program gave none.
store-because-none = tidak ada alasan yang diberikan

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = komputer ini tidak mengizinkan Trindle menulis di sana

# The reason when the disk is full.
store-because-space = tidak ada ruang tersisa di komputer ini

# The reason when a part of the computer is read-only.
store-because-read-only = bagian komputer itu hanya bisa dibaca

# The reason when the person's disk quota is used up.
store-because-quota = ruang yang diizinkan untuk akun ini sudah habis

# The reason when a file or program is missing.
store-because-missing = file atau program yang diperlukan tidak ada

# The reason when no reason could be read from the program's last line.
store-because-failed = gagal

# The reason when the backup has no name to be written under.
store-because-no-name = tidak punya nama

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = pengaturannya di cadangan menjangkau keluar dari folder miliknya sendiri

# The reason when something is too big to be read whole.
store-because-too-big = terlalu besar untuk dibaca utuh

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = pengaturannya di cadangan tidak cocok dengan checksum-nya

# Said when the person's account picture was not put back; the reason follows.
# SAFETY: needs a human review before it ships
store-refusal-picture-only = Dikembalikan, kecuali gambar akun Anda. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Gambar akun Anda juga tidak dikembalikan. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec tidak bisa dijalankan: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Tidak bisa dibaca dari cadangan: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Cadangan tidak dibuat, karena { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = belum ada

# Beside the size of a part of the backup: about how big it is.
store-backup-about = sekitar { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Mengembalikan pengaturan dan AppImage Trindle...

# Step shown while the settings and data are put back.
store-backup-step-data = Mengembalikan pengaturan dan data...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
       *[other] Memasang { $count } aplikasi dari Flathub... ini bisa memakan waktu.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Pengaturan dan data sudah dikembalikan, tetapi aplikasi ini tidak dipasang, karena Flatpak menolaknya: { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Pengaturan dan data sudah dikembalikan, tetapi aplikasi ini tidak dipasang, karena Flatpak menolaknya: { $failed }. Juga tidak dipasang: { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Pengaturan dan data sudah dikembalikan, tetapi aplikasi ini tidak dipasang: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
       *[other] { $n } aplikasi dilewati: namanya tidak valid.
    }

# Said beside an app whose source is not secure: the source's name is given.
# SAFETY: needs a human review before it ships
store-flat-not-secure = remote { $name } tidak terenkripsi dan tidak diterima, jadi tidak dipakai

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = remote { $name } tidak ada di komputer ini dan cadangan tidak menyebut di mana letaknya

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = pengaturan { $name } (karena salinannya di cadangan tidak lolos pemeriksaan)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (karena nama sumbernya tidak valid)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (karena { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (karena Flatpak menolaknya: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = semuanya (karena tidak ada tempat untuk mengekstrak cadangan)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = apa yang diizinkan untuk setiap aplikasi

# Said among what could not be done: one app's settings.
store-flat-app-settings = pengaturan { $name }

# Said when an app's settings could not be put back and the old folder was kept.
# SAFETY: needs a human review before it ships
store-flat-kept-as = { $why }. Isi yang ada sebelumnya disimpan sebagai { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Memasang { $name } ({ $at } dari { $total })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Mengembalikan pengaturan { $name } ({ $at } dari { $total })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
       *[other] { $n } aplikasi terpasang.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Pengaturan dikembalikan untuk { $n }.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
       *[other] { $n } sudah ada, dibiarkan apa adanya.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Tidak dikembalikan: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

