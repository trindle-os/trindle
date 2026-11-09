# machine translated (Indonesian), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Plugin Anda

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = plugin pengaya ekstensi terpasang hapus pengaturan

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = Plugin menambah Trindle: ikon di cincin, pengaturan di sini, notifikasi, atau sesuatu yang berjalan sejak login. Masing-masing berjalan terisolasi, tidak diizinkan apa pun sampai Anda mengizinkannya di Izin, yang dibuka tombol di ujung judul halaman ini. Kartu setiap plugin di sini berisi pengaturannya sendiri; tong sampah di kartu, ditekan dua kali, menghapus plugin itu.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Jendela yang menanyakan apa yang boleh dipakainya tidak dapat dibuka.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Versi { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Versi { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } tidak meminta apa pun, jadi berjalan terisolasi tanpa izin apa pun.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
       *[other] { $name } meminta { $list }; { $allowed } dari { $asks } diizinkan. Berjalan terisolasi, hanya dengan yang Anda izinkan.
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = Izinkan menampilkan setiap hal yang diminta, dengan sakelar.

# Heading of the line about a plugin on its card.
settings-plugins-about-it = Tentangnya

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Ubah apa yang boleh dipakainya

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Hapus plugin ini

# Red words when a plugin could not be removed.
# SAFETY: needs a human review before it ships
settings-plugins-not-removed = Tidak dapat dihapus.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Ketik di sini

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } tidak memiliki pengaturan sendiri.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } tidak memiliki pengaturan untuk ditampilkan saat ini.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Tidak ada plugin yang terpasang.
       *[other] { $n } plugin terpasang: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Plugin

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Belum ada yang terpasang

# Under the None installed heading.
settings-plugins-none-sub = Aplikasi memiliki plugin untuk ditambahkan.

# Tip of the button at the end of the page's title that opens the Permissions page.
# SAFETY: needs a human review before it ships
settings-plugins-open-permissions = Buka Izin

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Menanyakan ke { $name }.

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } tidak menjawab. Layanannya mungkin berhenti.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Tersimpan.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } tidak menerimanya.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } tidak dapat melakukannya.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } tidak dapat melakukannya: { $why }.

