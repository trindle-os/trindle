
# machine translated (Indonesian), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Laporkan masalah

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Ada yang tidak beres. Kirim laporan?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = Laporan ini mengirim tulisan Anda, detail komputer, galat dari boot ini, dan log Trindle. Nama dan alamat (email, jaringan, Wi-Fi) dihapus dari data yang dikumpulkan Trindle. Nama berkas dan alamat web masih bisa muncul di baris log. Berkas dan kata sandi Anda tidak dikirim.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = Apa yang terjadi?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Aplikasi yang mana?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Tidak yakin

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Program lain

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = Desktop

# SAFETY: needs a human review before it ships
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Kirim

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when the person opened it themselves.
report-cancel = Batal

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = Jangan kirim

# Button that opens the exact text of the report.
report-show-details = Tampilkan rincian

# Button that closes the exact text of the report.
report-hide-details = Sembunyikan rincian

# Button under the shown report text that copies it.
report-copy = Salin

# Button under the shown report text that saves it as a file in Downloads.
report-save = Simpan

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Laporan belum bisa dikirim.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Memeriksa.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Terlalu banyak laporan sekaligus.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Menunggu sambungan.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Menunggu sambungan. { $why }

# Under the typing box after the report was sent.
report-sent = Terkirim. Terima kasih.

# Under the typing box while the report is being posted.
report-sending = Mengirim.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Disimpan sebagai Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Masukkan nama pengguna Buildmorphic Anda, bukan alamat email. Jangan pakai @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Nama pengguna harus terdiri dari 2 sampai 31 karakter, berupa huruf kecil, angka, atau tanda hubung.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Nama GitHub hanya boleh berisi huruf, angka, dan tanda hubung.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle tidak dapat menyimpan laporan untuk nanti: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Tidak dapat disimpan: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = tidak ada izin

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = drive penuh

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = folder tidak ada

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = kesalahan pada drive

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl tidak ada.

# Why a report could not be posted: the post could not be started.
report-not-posted = Tidak dapat dikirim.

# Why a report could not be posted: the computer is offline.
report-no-connection = Tidak ada sambungan.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Server membalas dengan { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Tulis beberapa kata lagi tentang apa yang terjadi.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Membaca log.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Tidak ada log untuk dikirim bersama laporan, jadi laporan tidak dapat dikirim. Simpan saja.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Gambar opsional. Hanya pemilik proyek yang bisa melihatnya.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Pilih gambar PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Ambil tangkapan layar

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Hapus gambar

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Gambar Anda, { $width } kali { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Pilih gambar PNG

report-picture-filter = Gambar PNG

# Under the picture while it is being read.
report-picture-reading = Membaca.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Tidak bisa mengambil tangkapan layar.

report-picture-no-chooser = Trindle tidak dapat membuka pemilih berkas.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Bukan gambar PNG.

report-picture-too-big = Lebih dari { $megabytes } MB.

report-picture-too-large = Gambar terlalu besar. Lebarnya paling banyak { $side } piksel dan tingginya { $side } piksel.

report-picture-damaged = Gambar rusak.

report-picture-unreadable = Tidak bisa membacanya.

report-sent-as-bug = Terkirim. Nomor laporan bug Anda adalah { $number }. Terima kasih.

# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle masih dalam tahap beta. Laporan bersifat anonim: nama, alamat email, dan folder Home dihapus. Nama pengguna bersifat opsional.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Nama pengguna Buildmorphic (opsional)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle tidak dapat mengirim laporan. { $why } Tekan Kirim untuk mencoba lagi.
