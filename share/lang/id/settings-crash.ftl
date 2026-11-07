
# machine translated (Indonesian), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Laporan kerusakan

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = laporan kerusakan crash error bug kesalahan kirim terkirim privasi telemetri buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = Saat salah satu program Trindle mogok, add-on laporan kerusakan menyimpan catatan singkat di komputer ini. Add-on itu tidak mengirim apa pun sampai Anda menjalankan perintah kirimnya dan menjawab ya untuk setiap laporan, yang lebih dulu menampilkan seluruh laporan. Nama dan nama GitHub Anda hanya masuk jika Anda mengetiknya di sini. Tidak pernah pesan, nama berkas, lokasi Anda, atau apa pun tentang program lain. Menghapus add-on menghentikan semuanya.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Kerusakan

# Under the Crashes switch.
settings-crash-crashes-short = Di mana dalam kode Trindle kerusakan terjadi.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Kerusakan mengirim program, versinya, jenis kerusakan, dan di mana dalam kode Trindle kerusakan terjadi.

# Heading and label of the switch for sending errors.
settings-crash-errors = Galat

# Under the Errors switch.
settings-crash-errors-short = Nama tetap galat, dan di mana.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Galat mengirim program, versinya, nama tetap yang diberikan Trindle pada galat, dan di mana dalam kodenya.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = Tentang komputer ini

# Under the About this computer switch.
settings-crash-computer-short = Linux, prosesor, memori, grafis, layar.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = Tentang komputer ini menambahkan ke setiap laporan: Linux yang dipakai, jenis prosesor, memori yang dibulatkan ke bawah, Wayland atau X11, pembuat grafis, dan jumlah layar.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Apa yang dikirim

# Help text of the What is sent card: { $switches } is what each switch sends.
# SAFETY: needs a human review before it ships
settings-crash-what-about = { $switches } Laporan tidak membawa nama kecuali Anda mengetik nama pengguna Buildmorphic di kotak di bawah, yang hanya memberi Anda penghargaan.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Tidak ada yang diubah, karena { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Tidak ada yang diubah, karena berkas penyimpannya tidak seperti yang ditulis Trindle.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Tidak ada folder pengaturan.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Folder pengaturan tidak dapat dibuat.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Pengaturan tidak dapat ditulis.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Kerusakan ({ $signal }) di { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Galat { $code } di { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Diterima

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Tidak terkirim: pelacak ditutup

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Tidak terkirim: terlalu besar

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Tidak terkirim: ditolak

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Tidak terkirim: terlalu banyak sekaligus

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Tidak terkirim

# Said beside the link line once it is saved.
settings-crash-saved = Tersimpan

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Kosongkan daftar

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Daftar tidak dapat dikosongkan.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Dihapus

# Heading of the line that holds the clear button.
settings-crash-kept-here = Disimpan hanya di komputer ini

# Under the Kept on this computer only heading.
settings-crash-newest-first = Terbaru dulu, hingga lima puluh.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Belum ada

# Under the Nothing yet heading.
settings-crash-listed-here = Setiap laporan dicantumkan di sini, persis seperti yang dikirim.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Tampilkan persis apa yang dikirim

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
       *[other] { $when }, { $n } kali: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Terkirim sejauh ini

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Setiap laporan dicantumkan persis seperti yang dikirim, terbaru dulu, dan disimpan hanya di komputer ini. Diterima berarti Buildmorphic sudah menerimanya. Tidak terkirim: pelacak ditutup berarti pelacak bug Trindle tidak menerima laporan; ditolak berarti ditolak karena dibuat dengan salah; terlalu banyak berarti terlalu banyak datang sekaligus.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Setelah kerusakan Anda ditanya, dengan seluruh laporan ditampilkan, apakah akan mengirimnya.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Tidak ada yang dikirim, dan Anda tidak ditanya setelah kerusakan.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Tidak ada yang dikirim: kerusakan dan galat sama-sama dimatikan.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Kesalahan program Trindle dilaporkan secara anonim.
        [both] Kerusakan dan kesalahan program Trindle dilaporkan secara anonim.
       *[crashes] Kerusakan program Trindle dilaporkan secara anonim.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Tanyakan tentang masalah dan berbagi laporan

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Mati menyembunyikan semuanya dan menghapus laporan yang menunggu.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Mati. Tidak ada laporan yang menunggu.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
       *[other] Mati. { $n } laporan yang menunggu dihapus.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Tentang komunitas

# Heading of the line with the community button.
settings-crash-go-to-bm = Laporan dikirim ke Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = Komunitas yang membantu membangun Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = nama pengguna

# Heading of the line with the username box.
settings-crash-your-username = Nama pengguna Buildmorphic Anda

# Under the username heading.
settings-crash-username-sub = Opsional. Jangan pernah alamat email.

# Heading of a line about joining with an email address.
settings-crash-join = Bergabung hanya dengan alamat email

# Under the Join heading.
settings-crash-join-sub = Dapatkan kredit dan level untuk masalah yang pertama kali Anda laporkan.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Laporan hanya berisi daftar tetap

# Under the fixed list heading.
settings-crash-fixed-sub = Anda melihatnya dulu, dan tidak ada yang terkirim tanpa tekanan Anda.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Laporan yang Anda kirim

# Help text of the Reports you send card (the information button).
# SAFETY: needs a human review before it ships
settings-crash-reports-about = Saat salah satu program Trindle mengalami kerusakan, sebuah jendela dapat terbuka menanyakan apakah akan mengirim laporan. Jendela itu menampilkan teks persisnya lebih dulu: kata-kata Anda, model dan grafis komputer, galat, dan log sesaat sebelumnya. Berkas, kata sandi, dan nama Wi-Fi tidak pernah dibaca, dan nama serta alamat dikosongkan. Anda juga dapat membuka jendela yang sama kapan saja dari tombol Laporkan masalah di ring.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = nama github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Nama GitHub Anda

# Small words under that heading.
settings-crash-github-about = Opsional. Ditampilkan sebagai Dilaporkan oleh.

