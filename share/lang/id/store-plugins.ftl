# machine translated (Indonesian), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Buka pengaturannya

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Tidak berhasil: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = Pembaruan ini memerlukan izin baru. Menunggu Anda.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Versi { $mine } sekarang, { $new } siap

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = Versi { $mine } sekarang, { $new } siap. Meminta lebih banyak dari sebelumnya: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Tombol di sini kembali setelah selesai.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Mencari plugin…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Belum ada plugin yang terpasang, dan belum ada yang ditawarkan.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [1] Belum ada plugin yang terpasang; 1 siap dipasang.
       *[other] Belum ada plugin yang terpasang; { $n } siap dipasang.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
       *[other] { $n } plugin terpasang
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
       *[other] { $n } punya pembaruan
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
       *[other] { $n } pembaruan ditahan
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, semuanya terbaru.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, dan { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, dan { $have }; { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Tidak meminta apa pun.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = Yang diminta: { $list }.

# Title of the Plugins page.
store-plugins-title = Plugin

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Versi { $mine } terpasang; { $new } siap.

# On a plugin's card: its installed version.
store-plugins-version-installed = Versi { $version }, terpasang.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Versi { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
# SAFETY: needs a human review before it ships
store-plugins-install = Pasang { $name }

# Tip of the round button that releases a plugin whose updates are held back.
# SAFETY: needs a human review before it ships
store-plugins-release = Lepaskan { $name }

# Tip of the round button that updates a plugin to a version.
# SAFETY: needs a human review before it ships
store-plugins-update-to = Perbarui ke { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = Asal plugin

# Information on the card that says where plugins come from.
store-plugins-come-from = Plugin hanya berasal dari daftar milik Trindle sendiri, yang dibaca dari trindle.org setiap kali halaman ini dibuka. Untuk menawarkan satu plugin, ajukan: setiap plugin diuji sebelum tayang.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Terpasang, tetapi bagian latar belakangnya tidak berjalan: { $why }

# Red words under a plugin that could not be removed; the reason follows.
# SAFETY: needs a human review before it ships
store-plugins-not-removed = Tidak dapat dihapus: { $why }
