# machine translated (Indonesian), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Tersimpan

# Said beside a button that worked, when nothing more is known.
settings-common-done = Selesai

# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = tersimpan|selesai|sedang dipakai|diubah|diatur|diambil|ditambahkan|dibebaskan|dihapus|disimpan|terpasang|diperbarui|bergabung|disalin|sekarang bawaan|diputar|terkirim|tersambung|menyambung|hidup|mati|dilupakan|berhenti di 80%|mengisi sampai penuh|mulai saat login|disimpan, tetapi tidak dijalankan|sudah terbaru

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Tidak berhasil

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Tentang halaman ini

# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = Tentang { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } sudah ada

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
       *[other] Lakukan hal yang sama untuk sisanya ({ $n } lagi)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Lakukan hal yang sama untuk sisanya

# Tip of the button that keeps the one already there.
# SAFETY: needs a human review before it ships
settings-common-skip = Lewati dan pertahankan yang sudah ada

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = Ganti

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = Ganti yang sudah ada dengan yang baru

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Yang baru

# Heading of the line that shows the one already there.
settings-common-already-line = Yang sudah ada

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = Ganti yang sudah ada?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
       *[other] Dibatalkan dalam { $left } detik.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
       *[other] Dibatalkan dalam { $left } detik. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Urungkan sekarang

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Pertahankan

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } tidak tersedia

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Untuk memasangnya, jalankan: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Trindle tidak dapat membuka halaman web, karena tidak dapat memulai peramban web.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = Tidak dapat membuka { $name }.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Tidak dapat disimpan.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Trindle tidak dapat melakukannya karena bagian sistem tidak merespons. Nyalakan ulang komputer, lalu coba lagi.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Trindle tidak dapat melakukannya karena bagian login Anda tidak merespons. Keluar dan masuk lagi, lalu coba lagi.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Trindle tidak dapat membuka jendela untuk memilih file atau folder. Keluar dan masuk lagi, lalu coba lagi.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Disk penuh.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Trindle tidak dapat menulis ke disk sekarang.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = Bagian sistem yang mengerjakan ini tidak berjalan.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Jawabannya terlalu lama. Coba lagi.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = Kata sandi tidak diberikan.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = Akun ini tidak diizinkan melakukannya.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Ada yang dibutuhkan tetapi tidak ada di komputer ini.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } dan { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Word drawn in the middle of that picture.
settings-common-ring-name = Cincin

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Trindle tidak dapat menjangkau kontrol pengisian daya laptop: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Laptop tidak menerima batas pengisian daya.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Komputer ini tidak mengizinkan Anda menghentikan pengisian lebih awal.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Tidak dapat mengatur { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Mencoba Trindle di mesin virtual

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle berjalan di { $machine }. Berjalan baik di sana, tetapi mesin virtual tidak pernah secepat atau selancar komputer sungguhan. Nilailah kecepatannya di perangkat keras sungguhan.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle berjalan di { $machine } tanpa grafis 3D. Prosesor harus menggambar semua yang biasanya digambar chip grafis. Jendela, video, dan halaman web akan terasa lebih lambat daripada di komputer sungguhan, jadi Trindle menghilangkan animasinya. Jika komputer yang menjalankannya mampu, nyalakan akselerasi 3D di pengaturan layar mesin virtual. Itu sangat membantu. Semua ini tidak berlaku jika Anda memasang Trindle di komputer sungguhan.

# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } atau { $last }
