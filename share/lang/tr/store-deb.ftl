
# machine translated (Turkish), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Paket kur

# Words at the top of the window while the package is read.
store-deb-reading = Paket okunuyor…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Bu dosya kurulabilir bir Debian paketi değil. İndirilmesi bitmemiş olabilir.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } kendi klasörüne kurulmaya hazır.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Bu paket { $arch } bilgisayarlar için, bu yüzden bu bilgisayara kurulamaz.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Bu Trindle'ın kendisi. Bir dosyadan değil, Uygulamalar'daki Güncellemeler sayfasından güncelleyin.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } bir sürücü ya da arka plan hizmeti. Kendi klasöründen çalışamaz, bu yüzden burada kurulamaz.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Debian ve Trindle dışından gelen bir paket kendi klasörüne kurulur. Orada kısıtlıdır, ama tamamen kapatılmış değildir. Yalnızca güvendiğiniz yapımcıların paketlerini kurun.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } kendi klasörüne kuruldu.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Nereden geldiği denetleniyor…

# Heading in the card that says where the file came from.
store-deb-where = Nereden geldiği

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Paketi ve verilerini kaldır

# Word beside the ring while the package goes in.
store-deb-installing = Kuruluyor

# Main button that installs the package.
# SAFETY: needs a human review before it ships
store-deb-install = Kur

# Main button that installs the same version again.
store-deb-reinstall = Yeniden kur

# Main button that brings the installed package up to this version.
store-deb-update = Güncelle

# Heading of the card for a package not installed yet.
store-deb-goes-in = Kendi kısıtlı klasörüne kurulur

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = Parola gerekmez. Siz izin verene kadar interneti veya sesi kullanamaz, dosyalarınızı göremez.

# Heading of the card for a package installed at this version.
store-deb-same-version = { $version } sürümü kendi klasörüne kurulu

# Heading of the card for a package installed at another version.
store-deb-other-version = { $version } sürümü kendi klasörüne kurulu

# Under that heading: the version this brings.
store-deb-other-version-note = Bu, onu { $version } sürümüne günceller ve kaydettiklerini korur.

# Facts line: how big the package is once installed.
store-deb-fact-size = Kurulunca boyutu

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Hedef bilgisayar

# Text of the About card when the package has no description.
store-deb-no-description = Bu paketin açıklaması yok.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Kendi klasörüne kuruluyor. Gereken parçaların indirilmesi birkaç dakika sürebilir.

# Words at the top of the window once the package is installed.
store-deb-done = Kendi klasörüne kuruldu. Halkada bulabilirsiniz.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Kendi klasörüne kuruldu, ama şu parçalar eksik, bu yüzden başlamayabilir: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Kendi klasöründen, kaydettiği her şeyle birlikte kaldırıldı.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Bu dosya kurulabilir bir Debian paketi değil.

# Step shown while a package is unpacked.
store-boxed-unpacking = { $name } açılıyor

# Step shown while a part a package needs is added.
store-boxed-adding = { $part } ekleniyor

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } kurulabilir bir Debian paketi değil.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path } açılamadı.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Bu program kutuda çalışmıyor.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Uçbirim

# Title of the notice when older programs are off.
store-boxed-x-off = Eski programlar kapalı

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Ayarlar, İzinler

# Title of the notice when too many older programs are open.
store-boxed-x-many = Çok fazla eski program

# Body of that notice.
store-boxed-x-many-close = Birini kapatıp yeniden deneyin

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } açılamadı

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = Uçbirim, bilgisayarınızın diğer uygulamalardan daha fazlasına erişebilir. Yalnızca anladığınız ve güvendiğiniz kişilerden gelen komutları yazın.

# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = Yerel ağ yok

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } için ağ yok: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = Trindle'ın programı eksik

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare kurulu değil

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta kurulu değil

# The same: the network room was not made.
store-boxed-home-why-not-made = ağı kurulmadı

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = ağı zaten vardı

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle bir güvenlik süzgeci kuramadı

# The same: the room's rules would not load.
store-boxed-home-why-rules = ağ kuralları yüklenemedi

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } başlayamadı: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta şunu dedi: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta ona bağlanamadı

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = Üst klasör { $name } bir bağlantı, bu yüzden paket kullanılamaz.

# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } bir sürücü ya da arka plan hizmetidir. Bunlar kendi klasöründen çalışamaz, bu yüzden kurulmadı.


# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = İnternet olmadan açıldı

# Body of that notice: why, and what the program was given.
store-downloaded-why = İndirildiği için program yalnızca o tek dosyayı okuyabilir.
store-boxed-x-unasked = Eski programlar açılamıyor
store-boxed-x-unasked-why = Oturumu kapatıp yeniden açın, sonra yeniden deneyin
