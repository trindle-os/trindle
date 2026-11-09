
# machine translated (Indonesian), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Berkas Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Berkas .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Plugin

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = kandidat rilis

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = Menghapus { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Memasang { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Buka { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Pasang { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Terpasang

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Tampilkan

# How long is left when it is a minute or less.
store-left-minute = sisa sekitar satu menit

# How long is left, in minutes.
store-left-minutes = sisa sekitar { $n } menit

# How long is left, in hours.
store-left-hours =
    { $n ->
       *[other] sisa sekitar { $n } jam
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Diperiksa terhadap daftar Threat Intelligence HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Diunduh dari { $site }, dicapai dari { $referrer }.

# Where a download came from: the site.
store-origin-site = Diunduh dari { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Dicapai dari { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle tidak dapat mengetahui dari mana ini berasal.

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } ada di daftar Threat Intelligence HaGeZi, yaitu situs yang dikenal menyebarkan unduhan berbahaya dan penipuan. Pasang hanya jika Anda yakin dari mana asalnya.

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } Situs ini tidak ada di daftar Threat Intelligence HaGeZi, tetapi daftar itu tidak lengkap, jadi ini tidak membuktikan situs itu aman.

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } Situs ini tidak dapat diperiksa, karena daftar Threat Intelligence HaGeZi tidak dapat diambil.

# Name shown for a review whose writer gave none.
store-review-someone = Seseorang

# Red words when the keyring could not be reached.
# SAFETY: needs a human review before it ships
store-key-unreached = Gantungan kunci tidak dapat dijangkau: { $why }.

# Red words when the key kept in the keyring is not text.
# SAFETY: needs a human review before it ships
store-key-not-text = Kunci yang tersimpan bukan teks.

# Red words when a file that held the key could not be removed.
# SAFETY: needs a human review before it ships
store-key-not-removed = { $path } tidak dapat dihapus: { $why }

# Red words when a file that held the key could not be read.
# SAFETY: needs a human review before it ships
store-key-not-read = { $path } tidak dapat dibaca: { $why }

