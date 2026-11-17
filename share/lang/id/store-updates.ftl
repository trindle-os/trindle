# machine translated (Indonesian), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Paket Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Ditahan pada versi { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Ditahan; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Ditahan pada versi { $have }

# On a thing held back with no update waiting.
store-upd-held = Ditahan

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Ditahan pada versi { $have }; belum ada yang lebih baru

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Ditahan; belum ada yang lebih baru

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
       *[other] { $n } pembaruan
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
       *[other] { $n } pembaruan siap
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, total { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
       *[other] { $n } ditahan
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Mulai ulang untuk menyelesaikan pembaruan firmware.

# Headline while the check runs.
store-upd-checking = Memeriksa pembaruan.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Memperbarui { $name }. Anda dapat terus bekerja selagi dipasang.

# Headline while all the updates go in.
store-upd-installing-all = Memasang pembaruan. Anda dapat terus bekerja selagi dipasang.

# Headline when the updates are in and the computer must start again.
store-upd-restart = Mulai ulang komputer untuk menyelesaikan pemasangan pembaruan.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Mulai ulang untuk menyelesaikan pemasangan.

# Added to the headline with no internet.
store-upd-no-more = Komputer ini tidak terhubung ke internet, jadi Trindle tidak dapat mencari pembaruan lagi.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Tidak ada pembaruan yang siap

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Trindle tidak tahu ada pembaruan yang menunggu. Trindle tidak dapat mencari lagi selama komputer ini tidak terhubung ke internet.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = aplikasi

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle dan Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = firmware

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Trindle tidak dapat memeriksa pembaruan untuk { $names }.

# Headline when everything is up to date.
store-upd-current = Semuanya sudah terbaru.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Terakhir diperiksa pada { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Tidak dapat diperiksa tanpa internet

# Red words under a card when a check took too long.
store-upd-too-long = Pemeriksaan terlalu lama.

# Tip of the round button that checks again.
store-upd-check-again = Periksa lagi

# Main button while the check runs.
store-upd-checking-main = Memeriksa

# Main button while the updates go in.
store-upd-installing-main = Memasang

# Main button that installs every update waiting.
# SAFETY: needs a human review before it ships
store-upd-install-n =
    { $n ->
       *[other] Pasang { $n } pembaruan
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = Mulai ulang sekarang

# Main button that checks for updates.
store-upd-check-for = Periksa pembaruan

# Tip of the round button that releases a thing held back.
# SAFETY: needs a human review before it ships
store-upd-release = Izinkan pembaruan untuk { $name }

# Tip of the round button that holds a thing's updates back.
# SAFETY: needs a human review before it ships
store-upd-hold = Tahan { $name }

# A card's state while it is being checked.
store-upd-status-checking = Memeriksa

# A card's state while its updates go in.
store-upd-status-installing = Memasang

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Tidak dapat memeriksa. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } ditahan

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } ditahan

# A card's state when nothing waits.
store-upd-up-to-date = Sudah terbaru

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Dari { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Dari { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Bagian bersama, dari { $from }

# Note under the Apps card.
store-upd-yours-alone = Aplikasi ini hanya untuk Anda, jadi diperbarui tanpa kata sandi. Pembaruan aplikasi juga memasang bagian bersama yang diperlukannya.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Pembaruan yang ditahan tetap pada versinya saat ini. Pembaruan itu tidak disertakan saat Anda memasang semua pembaruan, di sini maupun di ring, sampai Anda mengizinkan pembaruan lagi.

# Heading of the Apps card.
store-upd-card-apps = Aplikasi

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Semua plugin sudah terbaru.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
       *[other] Pembaruan { $n } plugin ditahan.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
       *[other] { $n } plugin punya pembaruan
    }

# Heading of the Plugins card.
store-upd-card-plugins = Plugin

# Held back card: nothing is held back.
store-upd-nothing-held = Tidak ada yang ditahan.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Tidak ditahan

# Note under the Held back card: what releasing does.
store-upd-release-why = Setelah Anda mengizinkan pembaruan lagi, pembaruan itu dipasang bersama yang lain, di sini dan di ring.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Firmware sebuah perangkat

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = firmware perangkat { $id }

# Heading of the Held back card.
store-upd-card-held = Ditahan

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } ke { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [1] 1 pembaruan, yang dipasang sendiri
       *[other] { $n } pembaruan, yang dipasang bersamaan
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Dari Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Menampilkan { $shown } dari { $total } pembaruan Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Komputer ini memakai Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle dan Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Paket Debian dibuat agar bekerja sama. Memasang satu saja dapat memasang paket lain, atau membuatnya tidak selaras, jadi semuanya dipasang bersamaan.

# Note on the Trindle and Debian card: why no row has a Hold back button.
# SAFETY: needs a human review before it ships
store-upd-not-held = Pembaruan Trindle dan Debian membawa perbaikan keamanan untuk seluruh komputer, jadi tidak dapat ditahan.

# Heading of the card about the daily security updates.
store-upd-card-security = Pembaruan keamanan

# On the security card: its state.
store-upd-security-on = Hidup, setiap hari

# On the security card: that holding back never stops these.
store-upd-never-held = Menahan pembaruan tidak pernah menghentikan pembaruan keamanan.

# Firmware card: the computer has no updater.
store-upd-fw-none = Komputer ini tidak punya pembaru firmware.

# Firmware card: restart to finish.
# SAFETY: needs a human review before it ships
store-upd-fw-restart = Mulai ulang untuk menyelesaikan

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Versi { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Firmware baru dipasang saat komputer dimulai ulang.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Pasang fwupd agar firmware tetap terbaru.

# Heading of the Firmware card.
store-upd-card-firmware = Firmware

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd tidak dapat mengambil daftar terbaru.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub tidak dapat dijangkau.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Trindle tidak dapat mengambil daftar Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Trindle tidak dapat mengambil daftar Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Trindle tidak dapat menahannya, karena tidak dapat menyimpan daftar pembaruan yang ditahan: { $why }.

# The same, for a release.
store-upd-not-released-list = Trindle tidak dapat mengizinkan pembaruan lagi, karena tidak dapat menyimpan daftar pembaruan yang ditahan: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Ini ditahan di Aplikasi, tetapi Trindle tidak dapat mengubahnya di Flatpak. Program lain yang memperbarui aplikasi masih bisa memasangnya.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Pembaruan diizinkan lagi di Aplikasi, tetapi Trindle tidak dapat mengubahnya di Flatpak, jadi Flatpak mungkin masih melewatkannya. Tahan, lalu izinkan pembaruan lagi.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Kata Flatpak: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle

# Said when a security fix for a boxed app can't be downloaded. { $package } is the package, { $version } the fixed version.
store-patch-not-fetched = Trindle tidak dapat mengunduh { $package } { $version }.

# Said when a security fix for a boxed app was downloaded but isn't there to use.
store-patch-not-found = Trindle tidak dapat menemukan paket yang diunduhnya.


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Pemeriksaan pembaruan otomatis dimatikan.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Belum diperiksa

# Tip of the round button that ends an update that is running.
store-upd-stop = Berhenti

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Menghentikan

# Under a row whose update was stopped.
store-upd-stopped = Dihentikan. Tekan Perbarui untuk melanjutkan.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Dihentikan. Pasang lagi untuk melanjutkan.

# Note under the Apps card.
store-upd-steam-restarts = Steam dimulai ulang setelah tidak ada game yang berjalan.
