
# machine translated (Turkish), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = { $name } kur

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Hoş geldiniz

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Disk

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Gizlilik

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Parola

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Siz

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Hazır

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Kuruluyor

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Bitti

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Hazırlanıyor

# Red line at the foot when the installer's helper could not be started.
installer-no-start = Yükleyici başlatılamadı: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Plan okunamadı.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Plan okunamadı: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = Kurulum durdu. Daha fazla bir şey yazılmadı.

# Name of the round button at the foot that goes back a page.
installer-back = Geri

# Name of the main button at the foot that goes to the next page.
installer-next = İleri

# Name of the main button at the foot of the Ready page that starts the install.
# SAFETY: needs a human review before it ships
installer-install = Kur

# Name of the main button at the foot of the last page that restarts the computer.
# SAFETY: needs a human review before it ships
installer-restart = Şimdi yeniden başlat

# Beside the keyboard test box on the first page.
installer-try-keyboard = Klavyeyi deneyin

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Klavyeyi denemek için buraya yazın

# Note beside Install after one press, when the second press writes beside another system.
# SAFETY: needs a human review before it ships
installer-arm = Kurmak için yeniden tıklayın

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Silip kurmak için yeniden tıklayın

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Bir dil seçin

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Bir klavye seçin

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Bir saat dilimi seçin

# Note beside Next on the disk page while no disk is chosen.
# SAFETY: needs a human review before it ships
installer-missing-disk = Trindle'ın nereye kurulacağını seçin

# Note beside Next on the disk password page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-disk-password = Disk parolasını iki kez yazın, ikisinde de aynı olsun

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Bir oturum açma adı gerekli

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Bu bilgisayara bir ad verin

# Note beside Next on the account page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-password = Parolanızı iki kez yazın, ikisinde de aynı olsun

# Title of the first page.
installer-welcome-title = { $name } uygulamasına hoş geldiniz

# Sentence under the title of the first page.
installer-welcome-help = Dilinizi seçin; klavye ve saat onu izler, ikisini de değiştirebilirsiniz.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Dil

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Klavye

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Saat

# Under a card's head while nothing has been chosen in its list.
installer-select = Seç

# Title of the disk page.
installer-disks-title = { $name } nereye kurulacak

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Yanındaki mevcut sistem için yer açmazsanız, seçtiğiniz diskteki her şey değiştirilir.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Yeterince büyük disk bulunamadı. Trindle 20 GB gerektirir.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Tüm diski kullan, üzerindeki her şeyi değiştir

# Name and words of the choice to keep a partition and share the disk; it says which one.
# SAFETY: needs a human review before it ships
installer-disks-share = { $what } kalsın ve diski paylaşsın

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = Şu an olanlara { $kept }, Trindle'a { $mine } kalır

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Boş alan

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } ve { $last }

# Under a disk's name: what is on it.
installer-disk-has = Üzerinde { $list } var

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Üzerinde { $list } var, şifreli

# Under a disk's name: it holds an encrypted system Trindle cannot read.
# SAFETY: needs a human review before it ships
installer-disk-has-encrypted-system = Üzerinde şifreli bir sistem var

# Under a disk's name: there is nothing on it.
installer-disk-empty = Boş

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Üzerinde dosyalar var, ancak Trindle'ın tanıdığı bir sistem yok

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = { $gb } GB takas dosyası

# Name of the switch that makes a swap file.
installer-swap-switch = Takas dosyası oluştur

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Güvenli Önyükleme açık, bu yüzden bu bilgisayar hazırda bekletilemez, ancak takas dosyası bellek azaldığında yine de yardımcı olur. Yalnızca diskte bol yer varsa oluşturulur.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Her şey açıkken güç kapalı olarak hazırda beklemek için yer ve bellek azaldığında başvurulacak bir yer. Olmazsa bilgisayar hazırda bekletilemez. Yalnızca diskte bol yer varsa oluşturulur.

# Title of the page about encrypting the disk.
installer-privacy-title = Diski şifreleme

# Sentence under the title of that page.
installer-privacy-help = Şifreli bir diski, bu bilgisayardan çıkaran hiç kimse okuyamaz.

# Head of the card with the three choices.
installer-privacy-locked = Diskin nasıl kilitleneceği

# Name and words of the choice not to encrypt the disk.
# SAFETY: needs a human review before it ships
installer-privacy-none = Diski şifreleme

# Name and words of the choice to encrypt the disk with a password typed at start-up.
# SAFETY: needs a human review before it ships
installer-privacy-password = Şifrele ve bilgisayar açılırken parola sor

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
# SAFETY: needs a human review before it ships
installer-privacy-chip-choice = Şifrele ve kilidi bu bilgisayarın TPM yongası açsın

# Head of the card about the TPM chip.
installer-privacy-about = Yonga hakkında

# In the card about the chip, on a computer that has one.
# SAFETY: needs a human review before it ships
installer-privacy-chip = Bu bilgisayarda bir TPM yongası var. Kurulumdan sonraki ilk açılışta yonga bu bilgisayarı öğrenirken parola bir kez sorulur; bundan sonra disk kendiliğinden açılır ve hiçbir şey yazılmaz. Parolayı saklayın: yonga değiştirilirse ya da disk başka bir bilgisayara takılırsa içeri girmenin tek yolu odur.

# In the card about the chip, on a computer that has none Trindle can use.
# SAFETY: needs a human review before it ships
installer-privacy-no-chip = Bu bilgisayarda Trindle'ın kullanabileceği bir TPM yongası yok, bu yüzden parola her açılışta sorulur. Yongası olan bir bilgisayarda yonga diskin kilidini kendisi açabilir.

# Title of the disk password page.
installer-disk-title = Disk parolası

# Sentence under the title of the disk password page.
installer-disk-help = Bilgisayar açılırken her şeyden önce yazılır.

# Head of the card with the disk password boxes, and the name of the first box.
# SAFETY: needs a human review before it ships
installer-disk-password = Disk parolası

# Name of the round button that makes up a passphrase.
# SAFETY: needs a human review before it ships
installer-disk-suggest = Bir parola öbeği öner

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Aynısını yeniden

# Red line under the password boxes when the two do not match.
# SAFETY: needs a human review before it ships
installer-mismatch = İki parola eşleşmiyor.

# Head of the card that tells the person to write the password down.
# SAFETY: needs a human review before it ships
installer-disk-write-down = Bunu bir yere yazın

# Words in that card.
# SAFETY: needs a human review before it ships
installer-disk-write-down-text = Bu parolayı kâğıda yazın ve bilgisayardan uzak, güvenli bir yerde saklayın. Onsuz disk, ne siz ne de başkası tarafından, hiçbir zaman açılamaz. Geri dönmenin yolu yoktur. Dört sıradan kelime, yazması kolay, tahmini zor bir parola öbeği oluşturur.

# Title of the account page.
installer-you-title = Sizin hakkınızda

# Sentence under the title of the account page.
installer-you-help = Oturum açtığınız hesap budur.

# Head of the card with the name boxes.
installer-account = Hesabınız

# Label and name of the box for the person's name.
installer-your-name = Adınız

# Label and name of the box for the sign-in name.
installer-user-name = Oturum açma adı

# Label and name of the box for the computer's name.
installer-host-name = Bu bilgisayarın adı

# Head of the card with the password boxes.
# SAFETY: needs a human review before it ships
installer-your-password = Parolanız

# Label and name of the first password box.
# SAFETY: needs a human review before it ships
installer-password = Parola

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Tahmin edilmesi ne kadar zor

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Bir oturum açma adı gerekli.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Bu ad çok uzun.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Küçük bir harfle başlayın.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Yalnızca küçük harf, rakam, - ve _ kullanılabilir.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Bu ad sistem tarafından zaten kullanılıyor.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Bu bilgisayara bir ad verin.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Yalnızca harf, rakam ve - kullanılabilir.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Bir bilgisayar bunu hemen tahmin eder.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Bir bilgisayar bunu yaklaşık { $n } dakikada tahmin eder.
       *[other] Bir bilgisayar bunu yaklaşık { $n } dakikada tahmin eder.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Bir bilgisayar bunu yaklaşık { $n } saatte tahmin eder.
       *[other] Bir bilgisayar bunu yaklaşık { $n } saatte tahmin eder.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Bir bilgisayar bunu yaklaşık { $n } günde tahmin eder.
       *[other] Bir bilgisayar bunu yaklaşık { $n } günde tahmin eder.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Bir bilgisayarın bunu tahmin etmesi yaklaşık { $n } yıl sürer.
       *[other] Bir bilgisayarın bunu tahmin etmesi yaklaşık { $n } yıl sürer.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Bir bilgisayarın bunu tahmin etmesi kayıtlı tarihten daha uzun sürer.

# Title of the Ready page.
installer-ready-title = Kurulmaya hazır

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Henüz hiçbir şey değiştirilmedi

# Head of the card about security updates.
installer-ready-security = Güvenlik güncellemeleri

# In the card about security updates.
installer-ready-security-text = { $name }, Debian'ın güvenlik düzeltmelerini her gün kendiliğinden kurar. Bu kapatılamaz.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = { $disk } üzerindeki her şey değiştirilecek.

# Sentence under the title when the install goes beside what is there.
# SAFETY: needs a human review before it ships
installer-ready-kept = { $part } üzerindeki her şey korunur, ancak önce yedekleyin: küçültme sırasında elektrik kesilirse dosyalar kaybolabilir.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Henüz bir disk seçilmedi.

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = { $disk } diskinin tamamı

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Üzerindeki her şey { $name } ile değiştirilir.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = { $part } yanında

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-beside-text = { $part } bölümü { $size } boyutuna kadar küçültülür ve { $name } boşalan alana kurulur.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Disk yok

# Words of that row.
installer-ready-no-disk-text = Disk seçilmedi.

# Name of a row in the list: the disk will not be encrypted.
# SAFETY: needs a human review before it ships
installer-ready-plain = Şifrelenmemiş

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-plain-text = Diski çıkaran herkes onu okuyabilir.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Şifreli

# Words of that row when a password is typed at each start.
# SAFETY: needs a human review before it ships
installer-ready-locked-password = Parolasını bilgisayar her açıldığında yazarsınız.

# Words of that row when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-ready-locked-chip = Parola bir kez, sonra bu bilgisayarın TPM yongası kilidi açar.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Klavye { $keyboard }, saat { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = { $name } kuruluyor

# Head of the card that holds the short tour.
installer-tour = Kısa bir tanıtım

# Title of the last page.
installer-done-title = { $name } kuruldu

# Sentence under the title of the last page.
installer-done-help = USB belleği çıkarın, sonra yeniden başlatın.

# Head of the card that repeats the warning about the disk password.
# SAFETY: needs a human review before it ships
installer-done-keep = Disk parolasını güvende tutun

# In that card when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-keep-chip = Disk şifreli. İlk açılışta yonga bu bilgisayarı öğrenirken disk parolası bir kez sorulur; bundan sonra kendiliğinden açılır. O parolayı kâğıda yazın ve bu bilgisayardan uzak, güvenli bir yerde saklayın. Onsuz disk, ne siz ne de başkası tarafından, hiçbir zaman açılamaz.

# In that card when the password is typed at each start.
# SAFETY: needs a human review before it ships
installer-keep-password = Disk şifreli. Henüz yapmadıysanız disk parolasını şimdi kâğıda yazın ve bu bilgisayardan uzak, güvenli bir yerde saklayın. Onsuz disk, ne siz ne de başkası tarafından, hiçbir zaman açılamaz.

# Title of the first slide.
installer-slide-ring = Her şey tek bir dairede

# Words of the first slide.
installer-slide-ring-text = İşaretçiyi ekranın kenarına götürüp çift tıklayın, halka açılır: programlarınız dış çevrede, saat ve anahtarlar ortada.

# Title of the second slide.
installer-slide-snapshots = Bir güncellemenin bozduğunu geri koyun

# Words of the second slide.
installer-slide-snapshots-text = Her yazılım değişikliğinden önce ve sonra sistemin bir anlık görüntüsü alınır. Ayarlar tüm sistemi bunlardan herhangi birine geri koyar; bilgisayar açılmazsa önyükleme menüsü de bunu yapabilir.

# Title of the third slide.
installer-slide-apps = Yolunuza çıkmayan uygulamalar

# Words of the third slide.
installer-slide-apps-text = Flathub uygulamaları, Debian'ın kendi uygulamaları, AppImage'lar ve yazılım kaynakları tek yerde; başka bir bilgisayara taşıyabileceğiniz yedeklerle.

# Title of the fourth slide.
installer-slide-yours = Sizin, çabucak

# Words of the fourth slide.
installer-slide-yours-text = Açık ya da koyu, kendi sesleriniz ve kopyaladıklarınızı hatırlayan bir pano.

# Title of the fifth slide.
installer-slide-small = Küçük ve sessiz

# Words of the fifth slide.
installer-slide-small-text = Masaüstü belleği az kullanır ve birkaç saniyede açılır, böylece eski bir dizüstü bilgisayar yeniden yeni gibi hissettirir.

# Title of the tour slide about the ring.
installer-tour-ring = Halka tüm masaüstüdür

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = İşaretçiyi bu köşeye götürün, halka açılır

# Note on the ring picture, at the time.
installer-tour-ring-time = Saat ve tarih, ortada

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Programlarınız dış çevrede durur

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Ses, ekran, tüm uygulamalarınız, uyanık tut ve kapatma

# Title of the tour slide about Settings.
installer-tour-settings = Ayarlar, sade sözlerle

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Aramadan bir ayar bulmak için buraya yazın

# Note on the Settings picture, at Network.
installer-tour-settings-network = Ağ, bir Wi-Fi ağına katıldığınız yerdir

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Açık ya da koyu, istediğiniz zaman

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Yirmi renk ya da kendi renginizi yazın

# Title of the tour slide about Apps.
installer-tour-apps = Geri kalan her şey için Uygulamalar

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Bir programı adıyla ya da ne yaptığıyla arayın

# Note on the Apps picture, at the list.
installer-tour-apps-many = Flathub ve Debian'dan binlerce ücretsiz program

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Trindle dahil her şey için güncellemeler

# Title of the tour slide about voice control.
installer-tour-voice = Onunla konuşabilirsiniz

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Sesli denetim burada: Ayarlar, sonra Halka

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Ses ekle düğmesine bir kez basın. Yaklaşık 150 MB'tır ve birkaç dakika sürer

# Note on the voice picture, about what to say.
installer-tour-voice-say = Sonra halkanın ortasına dokunun ve bir uygulamanın adını, “sesi yükselt” ya da “Wi-Fi ayarları” deyin

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Söyledikleriniz bu bilgisayarda anlaşılır ve başka hiçbir yere gitmez

# Title of the tour slide about snapshots.
installer-tour-snapshots = Hiçbir şey değişmeden önce geri dönüş yolu

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Bilgisayar her güncellemeden önce bir anlık görüntü alır

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Bir şey bozulursa önyükleme menüsünden eski haline dönün

# Name of the picture of the ring, and the head of its card.
installer-live-caption = Her şeyin yaşadığı yer: halka

# Sentence under the title of the welcome from a stick.
installer-live-help = USB bellekten çalışıyorsunuz, bu yüzden bu bilgisayarda hiçbir şey değiştirilmedi.

# Head of the card with the three first things to do.
installer-live-begin = Başlamak için

# Name of the first thing to do.
installer-live-open = Halkayı açın

# Line under it.
installer-live-open-line = Ekranın kenarına çift tıklayın ya da Super tuşuna basın.

# Name of the second thing to do.
installer-live-speak = Onunla konuşun

# Line under it when voice control is there.
installer-live-speak-line = Halkanın ortasına dokunun ve ne istediğinizi söyleyin.

# Line under it when voice control has to be added.
installer-live-speak-add = Ayarlar, Halka bölümünden sesi ekleyin, sonra ortaya dokunup konuşun.

# Name of the third thing to do.
installer-live-tab = Hepsi halkada

# Line under it.
installer-live-tab-line = Nelerin açık olduğu, bildirimleriniz ve pano hep orada.

# The choice of the dark look.
installer-live-dark = Koyu

# The choice of the light look.
installer-live-light = Açık

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Tüm masaüstü için açık ya da koyu

# Name of the round button that closes this window and opens the ring.
installer-live-around = Önce etrafa bakın

# Name of the main button that opens the installer.
installer-live-install = { $name } kur

# Foot of the install page while room is made on the disk.
installer-step-room = Yer açılıyor

# Foot of the install page while the disk is prepared.
installer-step-prepare = Disk hazırlanıyor

# Foot of the install page while a place is made beside another system.
installer-step-place = Trindle için bir yer açılıyor

# Foot of the install page while the disk is looked at.
installer-step-check = Diskte zaten ne olduğu denetleniyor

# Foot of the install page while another system is made smaller.
# SAFETY: needs a human review before it ships
installer-step-smaller = Diğer sistem küçültülüyor

# Foot of the install page while the disk is encrypted.
# SAFETY: needs a human review before it ships
installer-step-lock = Disk kilitleniyor

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Bilgisayarın yongası ayarlanıyor

# Foot of the install page while the system is copied.
installer-step-copy = Trindle diske kopyalanıyor

# Foot of the install page while settings are written.
installer-step-settings = Ayarlar yazılıyor

# Foot of the install page while the person's account is made.
installer-step-account = Hesabınız oluşturuluyor

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Canlı parçalar çıkarılıyor

# Foot of the install page while the start-up files are made.
installer-step-startup = Açılış dosyaları oluşturuluyor

# Foot of the install page while the boot menu is set up.
installer-step-boot = Önyükleme menüsü ayarlanıyor

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Anlık görüntüler ayarlanıyor

# Foot of the install page at the very end.
installer-step-finishing = Bitiriliyor

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } başlatılamadı: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } bitmedi. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = { $path } yazılamadı: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } bir disk değil.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } bu bilgisayarda bir disk değil.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Disk seçilmedi.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Bu oturum açma adı, Trindle'ın kullanabileceği bir ad değil.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Bu bilgisayar adı, Trindle'ın kullanabileceği bir ad değil.

# Red line at the foot: the language cannot be used.
installer-bad-language = Bu dil, Trindle'ın kullanabileceği bir dil değil.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Bu klavye, Trindle'ın kullanabileceği bir klavye değil.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Bu saat dilimi, Trindle'ın kullanabileceği bir saat dilimi değil.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Bu ad, Trindle'ın yazamadığı bir şey içeriyor.

# Red line at the foot: a password has a line break in it.
# SAFETY: needs a human review before it ships
installer-bad-password = Parola satır sonu içeremez.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = Yükleyicinin ikinci yarısı root olarak çalışmalıdır.

# Red line at the foot: there is no boot partition to share.
# SAFETY: needs a human review before it ships
installer-no-boot-partition = Bu diskte paylaşılacak önyükleme bölümü yok, bu yüzden Trindle burada zaten olanın yanına kurulamaz.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Diskin yeni bölümü bulunamadı.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = { $part } boyutu okunamadı.

# Red line at the foot: a partition of this kind cannot be made smaller.
# SAFETY: needs a human review before it ships
installer-cannot-shrink = Trindle bir { $fs } bölümünü küçültemez.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
# SAFETY: needs a human review before it ships
installer-cannot-shrink-unknown = Trindle bilinmeyen bir bölümü küçültemez.

# Red line at the foot: encrypting needs a password and there is none.
# SAFETY: needs a human review before it ships
installer-disk-password-needed = Diski şifrelemek için bir parola gerekli.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Sistem kopyalanamadı: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Sistem kopyalanamadı. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Takas dosyası için yer yok.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Takas dosyasının diskteki yeri okunamadı.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Yeni bölümler tanımlanamadı.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Saat dilimi ayarlanamadı: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Önyükleme menüsü bu bilgisayara kurulamadı.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Emin misin?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Neyin açık olduğu, pano ve ekranı temizleyen bir düğme

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev }, Trindle'ın çalıştığı bellektir.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } kullanımda. Onu kullananı kapatın, sonra yeniden deneyin.

# Said beside how far the install has got, once it has started.
installer-started = Kurulum başladı

# The button that ends an install under way.
installer-halt = Durdur

# Beside Stop after its first press.
installer-halt-sure = Kurulum durdurulsun mu? Diskin eski içeriği çoktan gitti.

# While an install is being stopped.
installer-halting = Durduruluyor

# When Stop could not let go of the disk.
installer-halt-stuck = Disk serbest bırakılamadı. Yeniden denemek için bilgisayarı yeniden başlatın.

# The copy is done and the person is still answering.
installer-step-waiting = Kopyalandı. Yanıtlarınız bekleniyor.



installer-already-running = Bu bilgisayarda başka bir yükleyici zaten çalışıyor.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle için en az 1 GB bellek gerekir ve bu bilgisayarda daha azı var.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Bir disk yalnızca UEFI ile açılan bir bilgisayarda şifrelenebilir ve bu bilgisayar UEFI ile açılmıyor.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = { $part } üzerindeki Windows hazırda bekletilmiş ya da hızlı başlatma durumunda bırakılmış, bu yüzden küçültmek dosyalarının kaybolmasına yol açabilir. Windows'u başlatın, Yeniden başlat'ı seçin, sonra yeniden deneyin.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle kuruldu, ancak anlık görüntüler ayarlanamadı: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle kuruldu, ancak önyükleme menüsünün anlık görüntüleri listelemesi sağlanamadı: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Her şey ayarlanamadı

