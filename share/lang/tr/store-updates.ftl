# machine translated (Turkish), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian paketleri

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = { $have } sürümünde bekletiliyor; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Bekletiliyor; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = { $have } sürümünde bekletiliyor

# On a thing held back with no update waiting.
store-upd-held = Bekletiliyor

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = { $have } sürümünde bekletiliyor; daha yenisi yok

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Bekletiliyor; daha yenisi yok

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 güncelleme
       *[other] { $n } güncelleme
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 güncelleme hazır
       *[other] { $n } güncelleme hazır
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, toplam { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 tanesi bekletiliyor
       *[other] { $n } tanesi bekletiliyor
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Aygıt yazılımının tamamlanması için yeniden başlatma gerekir.

# Headline while the check runs.
store-upd-checking = Güncellemeler denetleniyor.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = { $name } güncelleniyor. Kurulurken çalışmaya devam edebilirsiniz.

# Headline while all the updates go in.
store-upd-installing-all = Güncellemeler kuruluyor. Kurulurken çalışmaya devam edebilirsiniz.

# Headline when the updates are in and the computer must start again.
store-upd-restart = Bitirmek için yeniden başlatın: güncellemeler kuruldu ve bilgisayar yeniden başladığında kullanıma girer.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Kurulanların tamamlanması için yeniden başlatma gerekir.

# Added to the headline with no internet.
store-upd-no-more = Bu bilgisayar internete bağlı değil, bu yüzden daha fazlası aranamıyor.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Hazır güncelleme yok

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Bu bilgisayarın bildiği bekleyen güncelleme yok. İnternete bağlı değilken daha fazlasını arayamaz.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = uygulamalar

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle ve Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = aygıt yazılımı

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Güncellemeler denetlenemedi: { $names }.

# Headline when everything is up to date.
store-upd-current = Her şey güncel.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Son denetim: { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = İnternet olmadan denetlenemez

# Red words under a card when a check took too long.
store-upd-too-long = Yanıt vermesi çok uzun sürdü.

# Tip of the round button that checks again.
store-upd-check-again = Yeniden denetle

# Main button while the check runs.
store-upd-checking-main = Denetleniyor

# Main button while the updates go in.
store-upd-installing-main = Kuruluyor

# SAFETY: needs a human review before it ships
# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] 1 güncellemeyi kur
       *[other] { $n } güncellemeyi kur
    }

# SAFETY: needs a human review before it ships
# Main button that restarts the computer.
store-upd-restart-now = Şimdi yeniden başlat

# Main button that checks for updates.
store-upd-check-for = Güncellemeleri denetle

# SAFETY: needs a human review before it ships
# Tip of the round button that releases a thing held back.
store-upd-release = { $name } için bekletmeyi kaldır

# SAFETY: needs a human review before it ships
# Tip of the round button that holds a thing's updates back.
store-upd-hold = { $name } güncellemesini beklet

# A card's state while it is being checked.
store-upd-status-checking = Denetleniyor

# A card's state while its updates go in.
store-upd-status-installing = Kuruluyor

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Denetlenemedi. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } bekletiliyor

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } bekletiliyor

# A card's state when nothing waits.
store-upd-up-to-date = Güncel

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Kaynak: { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Kaynak: { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Ortak bir parça, kaynak: { $from }

# Note under the Apps card.
store-upd-yours-alone = Yalnızca size ait olduklarından parolasız güncellenir. Bir uygulama, yeni sürümünün gerek duyduğu ortak parçaları da getirir.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Bekletilen bir güncelleme, siz bekletmeyi kaldırana kadar sahip olduğu sürümde kalır ve buradaki ile halkadaki Tümünü kur işleminin dışında tutulur.

# Heading of the Apps card.
store-upd-card-apps = Uygulamalar

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Her eklenti güncel.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] 1 eklentinin güncellemesi bekletiliyor.
       *[other] { $n } eklentinin güncellemesi bekletiliyor.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 eklentinin güncellemesi var
       *[other] { $n } eklentinin güncellemesi var
    }

# Heading of the Plugins card.
store-upd-card-plugins = Eklentiler

# Held back card: nothing is held back.
store-upd-nothing-held = Bekletilen bir şey yok.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Bekletme kaldırıldı

# Note under the Held back card: what releasing does.
store-upd-release-why = Bekletmesi kaldırılan şey, buradaki ve halkadaki Tümünü kur işlemiyle yeniden diğerleriyle birlikte kurulur.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Bir aygıtın yazılımı

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = { $id } aygıtının yazılımı

# Heading of the Held back card.
store-upd-card-held = Bekletilenler

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } sürümünden { $to } sürümüne

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 güncelleme, tek başına kurulur
       *[other] { $n } güncelleme, birlikte kurulur
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Kaynak: Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Debian güncellemelerinin { $total } tanesinden { $shown } tanesi gösteriliyor.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Bu bilgisayarda Trindle { $version } var.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle ve Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Debian paketleri birbirine uyacak biçimde yapılır; tek başına kurulan bir paket başkalarını da getirebilir ya da onları uyumsuz bırakabilir. Bu yüzden tek bir düğmeyle birlikte kurulurlar.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Trindle'ın ve Debian'ın güncellemeleri tüm bilgisayar için güvenlik düzeltmeleri getirir, bu yüzden bekletilemez.

# Heading of the card about the daily security updates.
store-upd-card-security = Güvenlik güncellemeleri

# On the security card: its state.
store-upd-security-on = Açık, her gün

# On the security card.
store-upd-security-note = Debian düzeltmeleri her gün sormadan kurulur.

# On the security card.
store-upd-security-part = Trindle'ın parçasıdır; kapatılamaz.

# On the security card: that holding back never stops these.
store-upd-never-held = Bir güncellemeyi bekletmek bunları asla durdurmaz.

# Firmware card: the computer has no updater.
store-upd-fw-none = Bu bilgisayarda aygıt yazılımı güncelleyicisi yok.

# Firmware card: restart to finish.
store-upd-fw-restart = Bitirmek için yeniden başlatın

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Sürüm { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Yeni aygıt yazılımı, bilgisayar yeniden başladığında kurulur.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Aygıt yazılımını güncel tutmak için fwupd kurun.

# Heading of the Firmware card.
store-upd-card-firmware = Aygıt yazılımı

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd en yeni listeyi getiremedi.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub'a ulaşılamadı.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Debian listeleri getirilemedi.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Debian listeleri getirilemedi. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Bekletilmedi. Bekletilenlerin listesi saklanamadı: { $why }.

# The same, for a release.
store-upd-not-released-list = Bekletme kaldırılmadı. Bekletilenlerin listesi saklanamadı: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Uygulamalar'da bekletildi, ancak Flatpak'in kendisine bildirilemedi; bu yüzden uygulamaları güncelleyen başka bir şey onu yine de kurabilir.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Uygulamalar'da bekletme kaldırıldı, ancak Flatpak'in kendisine bildirilemedi ve onu yine de dışarıda bırakabilir. Yeniden bekletip tekrar kaldırın.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak şunu söyledi: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Kendiliğinden güncelleme denetimi kapalı. Bakmak için Güncellemeleri denetle düğmesine basın.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Denetlenmedi

# Tip of the round button that ends an update that is running.
store-upd-stop = Durdur

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Durduruluyor

# Under a row whose update was stopped.
store-upd-stopped = Durduruldu. Devam etmek için Güncelle düğmesine basın.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Durduruldu. Devam etmek için Kur düğmesine basın.

# Note under the Apps card.
store-upd-steam-restarts = Steam, çalışan oyun kalmayınca yeniden başlar.

