# machine translated (Turkish), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Ayarlarını aç

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Yapılamadı: { $why }

# SAFETY: needs a human review before it ships
# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = Bu güncelleme yeni bir izin istiyor. Sizi bekliyor.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Şu an sürüm { $mine }, { $new } hazır

# SAFETY: needs a human review before it ships
# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = Şu an sürüm { $mine }, { $new } hazır. Öncekinden fazlasını istiyor: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = İş bittiğinde buradaki düğmeler geri gelir.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Eklentiler aranıyor…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Kurulu eklenti yok ve henüz sunulan da yok.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Henüz kurulu eklenti yok; 1 tanesi kurulmaya hazır.
       *[other] Henüz kurulu eklenti yok; { $n } tanesi kurulmaya hazır.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 eklenti kurulu
       *[other] { $n } eklenti kurulu
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 tanesinin güncellemesi var
       *[other] { $n } tanesinin güncellemesi var
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 güncelleme bekletiliyor
       *[other] { $n } güncelleme bekletiliyor
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, hepsi güncel.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed } ve { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed } ve { $have }; { $held }.

# SAFETY: needs a human review before it ships
# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Hiçbir şey istemiyor.

# SAFETY: needs a human review before it ships
# On a plugin's card: what it asks for.
store-plugins-asks = İstedikleri: { $list }.

# Title of the Plugins page.
store-plugins-title = Eklentiler

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Sürüm { $mine } kurulu; { $new } hazır.

# On a plugin's card: its installed version.
store-plugins-version-installed = Sürüm { $version }, kurulu.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Sürüm { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# SAFETY: needs a human review before it ships
# Tip of the round button that installs a plugin.
store-plugins-install = { $name } eklentisini kur

# SAFETY: needs a human review before it ships
# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = { $name } eklentisini serbest bırak

# SAFETY: needs a human review before it ships
# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = { $version } sürümüne güncelle

# Heading of the card that says where plugins come from.
store-plugins-where = Eklentiler nereden gelir

# Information on the card that says where plugins come from.
store-plugins-come-from = Eklentiler yalnızca Trindle'ın kendi listesinden gelir; bu sayfa her açıldığında trindle.org adresinden okunur. Bir eklenti sunmak için gönderin: her biri yayına girmeden önce sınanır.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Kuruldu, ancak arka plandaki parçası çalışmıyor: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Kaldırılamadı: { $why }
