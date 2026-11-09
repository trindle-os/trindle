
# machine translated (Turkish), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Bir Flatpak dosyası

# On an installed thing's line: it came from a .deb file.
store-from-deb = Bir .deb dosyası

# On an installed thing's line: it is a plugin.
store-from-plugin = Eklenti

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = sürüm adayı

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = { $name } kaldırılıyor

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = { $name } kuruluyor

# Tip of the round button on an app's tile that opens the app.
store-list-open = { $name } uygulamasını aç

# Tip of the round button on an app's tile that installs the app.
store-list-install = { $name } uygulamasını kur

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Kurulu

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Göster

# How long is left when it is a minute or less.
store-left-minute = yaklaşık bir dakika kaldı

# How long is left, in minutes.
store-left-minutes = yaklaşık { $n } dakika kaldı

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] yaklaşık 1 saat kaldı
       *[other] yaklaşık { $n } saat kaldı
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = HaGeZi'nin Threat Intelligence listesine göre denetlendi

# Where a download came from: the site and the page that led to it.
store-origin-both = { $site } sitesinden indirildi, { $referrer } sayfasından gelinerek.

# Where a download came from: the site.
store-origin-site = { $site } sitesinden indirildi.

# Where a download came from: the page that led to it.
store-origin-referrer = { $referrer } sayfasından gelinerek ulaşıldı.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle bunun nereden geldiğini söyleyemiyor.

# SAFETY: needs a human review before it ships
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name }, zararlı indirmeler ve dolandırıcılıkla bilinen sitelerin yer aldığı HaGeZi'nin Threat Intelligence listesinde. Bunu yalnızca nereden geldiğinden eminseniz kurun.

# SAFETY: needs a human review before it ships
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } Site HaGeZi'nin Threat Intelligence listesinde yok, ancak liste eksiksiz değildir; bu nedenle bu, güvenli olduğunu kanıtlamaz.

# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } HaGeZi'nin Threat Intelligence listesi alınamadığı için site denetlenemedi.

# Name shown for a review whose writer gave none.
store-review-someone = Biri

# Red words when the keyring could not be reached.
store-key-unreached = Anahtar halkasına ulaşılamadı: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = Saklanan anahtar metin değil.

# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } kaldırılamadı: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = { $path } okunamadı: { $why }
