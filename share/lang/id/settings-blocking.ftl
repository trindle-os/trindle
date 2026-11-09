
# machine translated (Indonesian), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = baru saja

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
       *[other] { $n } menit lalu
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
       *[other] { $n } jam lalu
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
       *[other] { $n } hari lalu
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Menunggu kata sandi...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Pemblokiran aktif

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Pemblokiran mati

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Tersimpan

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Ketik nama situs, misalnya example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Pemblokiran mati. Pencarian nama diarahkan ke server jaringan Anda sendiri, seperti sebelumnya.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Pemblokiran sedang dimulai.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Iklan dan situs berbahaya diblokir, dan pencarian nama dikirim secara pribadi ke { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Server pribadi tidak dapat dijangkau di jaringan ini, jadi server jaringan Anda sendiri dipakai untuk sementara. Pemblokiran tetap berjalan.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Iklan dan situs berbahaya diblokir. Pencarian nama diarahkan ke server jaringan Anda sendiri, yang dapat melihatnya.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Iklan dan situs berbahaya diblokir. Pencarian nama diarahkan ke server VPN Anda sendiri, melalui VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Iklan dan situs berbahaya diblokir.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Memulai

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Mati

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = pemblokiran sedang dimulai

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = pemblokiran mati

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Diblokir hari ini

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = pencarian nama diblokir hari ini

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Daftar

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Mengambil

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Belum ada

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = daftar situs

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = situs terdaftar, diambil { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Belum ada yang diblokir

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Situs muncul di sini, yang terbaru di atas.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Selalu izinkan situs ini

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Baru saja diblokir

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Situs muncul di sini saat diblokir, yang terbaru di atas. Tanda plus mengizinkan satu situs mulai sekarang. Hanya situs yang diblokir yang ditampilkan di sini. Situs yang diizinkan tidak pernah disimpan.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Nyalakan pemblokiran untuk melihat apa yang diblokirnya.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = mis. example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Tambah situs

# Small line under "Add a site".
settings-blocking-add-sub = Situs itu dan semua subdomainnya tidak pernah diblokir.

# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Blokir situs ini lagi

# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = Selalu diizinkan

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Situs yang terdaftar di sini, beserta semua subdomainnya, tidak pernah diblokir, bahkan jika ada di daftar. Tanda minus memblokir satu situs lagi.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Pemblokiran

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = block adverts ads trackers malware scams dns private lookups quad9 mullvad cloudflare allow blokir iklan pelacak penipuan pribadi pencarian izinkan

# Help text at the top of the page.
settings-blocking-help = Memblokir iklan, pelacak, dan situs berbahaya yang dikenal untuk setiap peramban dan aplikasi di komputer ini, dan mengirim pencarian namanya lewat sambungan terenkripsi, sehingga jaringan yang Anda pakai tidak dapat membaca atau mengubahnya.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Blokir iklan dan situs berbahaya

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Tujuan pencarian nama

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Memakai daftar iklan, pelacak, dan situs berbahaya dari HaGeZi, diambil sekali sehari. Saat dimatikan, tidak ada yang berjalan. Pencarian nama dikirim lewat sambungan terenkripsi ke server pribadi yang dipilih di sini. Quad9 tidak mencatat siapa yang bertanya. "Jaringan Anda" tetap memblokir, tetapi tidak pribadi: server jaringan Anda sendiri melihat setiap pencarian.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Daftar HaGeZi, diambil sekali sehari.

# Heading of the line with the private server dropdown.
settings-blocking-private = Server pribadi

# Small line under Private server.
settings-blocking-private-sub = Quad9 tidak mencatat siapa yang bertanya.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Jaringan Anda
