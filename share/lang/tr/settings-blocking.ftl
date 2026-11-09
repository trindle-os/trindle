
# machine translated (Turkish), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = az önce

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] { $n } dakika önce
       *[other] { $n } dakika önce
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] { $n } saat önce
       *[other] { $n } saat önce
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] { $n } gün önce
       *[other] { $n } gün önce
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Parola bekleniyor...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Engelleme açık

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Engelleme kapalı

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Kaydedildi

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Bir sitenin adını yazın, örneğin example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Engelleme kapalı. Adres sorguları, eskisi gibi ağınızın kendi sunucularına gider.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Engelleme başlıyor.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Reklamlar ve kötü siteler engelleniyor; adres sorguları özel olarak { $provider } sunucusuna gidiyor.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Özel sunucuya bu ağda ulaşılamıyor, bu yüzden şimdilik ağınızın kendi sunucuları kullanılıyor. Engelleme yine de çalışıyor.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Reklamlar ve kötü siteler engelleniyor. Adres sorguları, onları görebilen ağınızın kendi sunucularına gidiyor.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Reklamlar ve kötü siteler engelleniyor. Adres sorguları VPN üzerinden, VPN'inizin kendi sunucularına gidiyor.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Reklamlar ve kötü siteler engelleniyor.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Başlıyor

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Kapalı

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = engelleme başlıyor

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = engelleme kapalı

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Bugün engellenen

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = bugün engellenen sorgu

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Listeler

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Alınıyor

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Henüz yok

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = site listeleri

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = site listede, { $ago } alındı

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Henüz hiçbir şey engellenmedi

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Siteler burada görünür, en yenisi üstte.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Bu siteye her zaman izin ver

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Son engellenenler

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Siteler engellendikçe burada görünür, en yenisi üstte. Artı düğmesi, bir siteye bundan sonra izin verir. Burada yalnızca engellenen siteler gösterilir. İzin verilen siteler hiçbir zaman saklanmaz.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Neleri engellediğini görmek için engellemeyi açın.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = örn. example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Site ekle

# Small line under "Add a site".
# SAFETY: needs a human review before it ships
settings-blocking-add-sub = Bu site ve tüm alt alan adları hiçbir zaman engellenmez.

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = Bu siteyi yeniden engelle

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Her zaman izin verilenler

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Burada listelenen siteler ve tüm alt alan adları, listelerde olsalar bile hiçbir zaman engellenmez. Eksi düğmesi, birini yeniden engeller.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Engelleme

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = engelle reklam reklamlar izleyici izleyiciler kötü amaçlı yazılım dolandırıcılık dns özel sorgular quad9 mullvad cloudflare izin ver

# Help text at the top of the page.
settings-blocking-help = Bu bilgisayardaki her tarayıcı ve uygulama için reklamları, izleyicileri ve bilinen kötü siteleri engeller; ad sorgularını şifreli bir bağlantıyla gönderir, böylece bağlı olduğunuz ağ bunları okuyamaz ya da değiştiremez.

# Text of the switch that turns blocking on or off.
# SAFETY: needs a human review before it ships
settings-blocking-switch = Reklamları ve kötü siteleri engelle

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Sorguların gideceği yer

# Help text (i button) of the Blocking card.
settings-blocking-card-about = HaGeZi'nin reklam, izleyici ve bilinen kötü site listelerini kullanır; günde bir kez alınır. Kapalıyken hiçbir şey çalışmaz. Ad sorguları, burada seçilen özel sunucuya şifreli bir bağlantıyla gider. Quad9, kimin sorduğuna dair kayıt tutmaz. "Ağınız" yine engeller ama özel değildir: ağınızın kendi sunucuları her sorguyu görür.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = HaGeZi listeleri, günde bir kez alınır.

# Heading of the line with the private server dropdown.
settings-blocking-private = Özel sunucu

# Small line under Private server.
settings-blocking-private-sub = Quad9, kimin sorduğuna dair kayıt tutmaz.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Ağınız

