# machine translated (Turkish), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } uygulama
       *[other] { $count } uygulama
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } güncelleme bekliyor
       *[other] { $count } güncelleme bekliyor
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub'da seçebileceğiniz { $apps }; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = sizin olanların hepsi güncel

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = “{ $text }” ile eşleşen uygulama yok.

# Line under the title of an empty page of apps.
store-grid-none-here = Burada henüz uygulama yok.

# Advice in the empty page of a search.
store-grid-try-shorter = Daha kısa bir sözcük deneyin ya da soldaki bir türe göz atın.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Yukarıdan ada göre arayın ya da soldaki başka bir türe göz atın.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $apps } içinden { $shown } gösteriliyor.

# Link under a page of apps that shows more of them.
store-grid-show-more = { $next } tane daha göster

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Uygulama listesi Flathub'dan getiriliyor. İlk seferde bir dakika sürer.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Uygulamalar, Flathub'a ulaşamadı.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Bu bilgisayar internete bağlı değil. Bağlanır bağlanmaz Uygulamalar kendiliğinden yeniden dener.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Seçilebilecek uygulamalar internet üzerinden Flathub'dan gelir.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } ve soldaki uygulama ayarları bunsuz çalışır.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } ve soldaki uygulama ayarları Flathub olmadan çalışır.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } ve soldaki uygulama ayarları internet olmadan çalışır.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Yeniden dene

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Sonuçlar

# Red words under the search box when no web browser opens a link.
store-no-browser = { $url } adresini açacak bir web tarayıcısı yok. Önce Uygulamalar'dan bir tane edinin.

# Said in the card or under the title while the lists are being read.
store-looking = Bakılıyor…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Artık Kurulu bölümünde ve halkada

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Kaldırıldı

# The word on an app's dial when installing or removing did not work.
store-done-not = Yapılamadı

# The word on an app's dial when it was installed.
store-done-installed = Kuruldu

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak reddetti.

# Tab of the Installed page that shows everything.
store-tab-all = Tümü

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak dosyaları

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage'lar

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb dosyaları

# Tab of the Installed page that shows plugins.
store-tab-plugins = Eklentiler

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Henüz hiçbir şey kurulu değil. Keşfet, denenecek şeylerle dolu.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 eklenti kurulu
       *[other] { $n } eklenti kurulu
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 program kurulu
       *[other] { $n } program kurulu
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } öğe kurulu

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, Flathub'dan
       *[other] { $start }, hepsi Flathub'dan
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, bir Flatpak dosyasından
       *[other] { $start }, hepsi Flatpak dosyalarından
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, bir AppImage
       *[other] { $start }, hepsi AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, bir .deb dosyasından
       *[other] { $start }, hepsi .deb dosyalarından
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } tanesi Flathub'dan

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 tanesi bir Flatpak dosyasından
       *[other] { $n } tanesi Flatpak dosyalarından
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
       *[other] { $n } AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 uygulama
       *[other] { $n } uygulama
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 tanesi bir .deb dosyasından
       *[other] { $n } tanesi .deb dosyalarından
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 eklenti
       *[other] { $n } eklenti
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Güncel.

# End of the Installed line when everything is up to date.
store-installed-state-all = Hepsi güncel.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 tanesinin güncellemesi bekliyor.
       *[other] { $n } tanesinin güncellemesi bekliyor.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $total } programdan { $shown } tanesi “{ $find }” ile eşleşiyor.

# The same when plugins are among the things.
store-installed-match-things = { $total } öğeden { $shown } tanesi “{ $find }” ile eşleşiyor.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = AppImage ekle

# Placeholder of the typing box on the Installed page.
store-installed-search = Kurulu olanlarda ara

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage eklenemedi.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Keşfet'ten, GitHub'dan ya da AppImage olarak kurduğunuz uygulamalar, .deb dosyalarından gelen programlar ve eklentiler burada görünür.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Kurulu olanlardan hiçbiri “{ $find }” ile eşleşmiyor.

# SAFETY: needs a human review before it ships
# Note at the foot of the Installed list.
store-installed-remove-twice = Kaldır iki kez sorar: üç saniye içinde yeniden basın.

# On an installed thing's line: its version.
store-installed-version = sürüm { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = bekletiliyor

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = bekletiliyor; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } güncelleniyor

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } öğesini güncelle

# Tip of the round button that shows an installed app's page.
store-installed-about = Hakkında

# Under an installed thing's row while it is being taken off.
store-installed-removing = Kaldırılıyor...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Hangi dosyanın seçileceğini soracak oturum veri yolu yok.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Bir AppImage seçin

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage'lar

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Dosya seçiciye sorulamadı.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Ekleniyor...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } zaten var: Kurulu bölümüne bakın.

# Words once an AppImage is in the ring.
store-add-in-ring = Eklendi. Artık listenizde.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Bu, bu bilgisayarın kullanabileceği bir Flatpak başvurusu değil.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = { $name }, { $host } kaynağından kuruluyor...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } kuruldu.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak bunu kurmadı.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } kurulamadı. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Değiştiriliyor...

# SAFETY: needs a human review before it ships
# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } zaten var

# Small heading over the AppImage being brought.
store-replace-new = Getirilen

# Small heading over the AppImage already there.
store-replace-old = Zaten olan

# SAFETY: needs a human review before it ships
# Tip of the round button that keeps the AppImage already there.
store-replace-skip = Atla: zaten olanı koru

# SAFETY: needs a human review before it ships
# Button that replaces the AppImage already there with the new one.
store-replace-do = Değiştir

# SAFETY: needs a human review before it ships
# Heading of the card that asks what to do with a Flatpak being removed.
store-flatpak-ask-head = { $name } kaldırılsın mı?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Kayıtlı verileri { $size } yer kaplıyor.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Kayıtlı verisi yok.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Kayıtlı verileri hesaplanıyor...

# SAFETY: needs a human review before it ships
# Under the choices: the two that delete data cannot be undone.
store-flatpak-ask-undone = Silinen veriler geri getirilemez.

# SAFETY: needs a human review before it ships
# Button: remove the app and leave the data it saved.
store-flatpak-keep = Kaldır, verilerini koru

# SAFETY: needs a human review before it ships
# Button (red): remove the app and delete its saved data.
store-flatpak-wipe = Kaldır ve verilerini sil

# SAFETY: needs a human review before it ships
# Button (red): delete only the saved data and keep the app.
store-flatpak-data-only = Yalnızca verilerini sil

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Silinecek kayıtlı verisi yok.

# SAFETY: needs a human review before it ships
# Tip of the round button that shuts the card with nothing done.
store-flatpak-cancel = İptal: ikisini de koru

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Kayıtlı verileri silindi.

# Words while only the data is being deleted.
store-flatpak-data-going = Kayıtlı verileri siliniyor...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Bitmedi ve neden vermedi.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Sorunun nerede olduğunu göster

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = GitHub'da ara: yazın, sonra Enter'a basın

# Heading of the card of matches from AppImageHub.
store-gh-hub = AppImageHub'da ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Tüm GitHub'da da aramak için Enter'a basın.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub'a sorulmadı: sormak için internet yok.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub'a soruluyor...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub'da başka bir şey yok.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] { $n } projede AppImage ve Flatpak dosyaları aranıyor...
       *[other] { $n } projede AppImage ve Flatpak dosyaları aranıyor...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub'daki eşleşmelerin hiçbirinde bu bilgisayar için AppImage ya da Flatpak dosyası yok.

# Heading of the card of matches from GitHub.
store-gh-on = GitHub'da

# Heading of the card that holds the search box.
store-gh-search-card = GitHub'da ara

# SAFETY: needs a human review before it ships
# Line under the title of the GitHub page.
store-gh-line = GitHub'daki uygulamalar, AppImage ya da Flatpak dosyaları olarak. Bunları kimse denetlemez: yalnızca güvendiğiniz kişilerden kurun.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = Bu bilgisayar internete bağlı değilken GitHub'da arama yapılamaz.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak ve AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Dosyalara bak

# SAFETY: needs a human review before it ships
# Button that installs a GitHub project or file.
store-gh-install = Kur

# Link to a project's page on GitHub.
store-gh-open = GitHub'da aç

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, sürüm { $tag }

# SAFETY: needs a human review before it ships
# Name of the GitHub key's button and of its box.
store-gh-key = GitHub anahtarı

# SAFETY: needs a human review before it ships
# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub anahtarı: biri kayıtlı

# SAFETY: needs a human review before it ships
# In the GitHub key's box: what the key is for.
store-gh-key-words = Büyük olasılıkla ihtiyacınız yok. Anahtar olmadan GitHub bu bilgisayardan saatte 60 soruya yanıt verir; anahtarla 5.000'e. Anahtar yalnızca bu sınırı yükseltir. Yalnızca herkese açık dosyalara bakmak için kullanılır, başka bir şey için değil: Trindle'a hesabınıza giriş yolu vermez ve oluştururken hiçbir izin işaretlemezsiniz. Bu bilgisayarda, anahtarlığınızda saklanır ve yalnızca GitHub'a gönderilir.

# SAFETY: needs a human review before it ships
# In the GitHub key's box: how to make a key.
store-gh-key-how = GitHub'da bir anahtar oluşturun, istediğiniz adı verin, hiçbir izne dokunmayın, Generate token düğmesine basın, sonra gösterdiğini buraya yapıştırın.

# SAFETY: needs a human review before it ships
# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = Kaldırılamadı: { $why }

# SAFETY: needs a human review before it ships
# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = Kaydedilemedi: { $why }

# SAFETY: needs a human review before it ships
# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Korumak için boş bırakın

# SAFETY: needs a human review before it ships
# Placeholder of the GitHub key's box.
store-gh-key-paste = Anahtarı yapıştırın

# SAFETY: needs a human review before it ships
# Tip of the round button that removes the stored GitHub key.
store-gh-key-remove = Anahtarı kaldır

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = GitHub'da anahtar oluştur

# SAFETY: needs a human review before it ships
# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = Anahtarı kaydet

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = En yeni sürümündeki hiçbir şey burada kurulmuyor.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Eklendi

# Line under the title of the Move your apps page.
store-bk-line = Uygulamalarınız ve ayarlarınız, her yere götürebileceğiniz tek bir dosyada.

# Heading of the card that makes a backup.
store-bk-card = Yedekle

# Tip of the round button while the backup is being made.
store-bk-making-tip = Yedek oluşturuluyor

# Main button that makes the backup.
store-bk-make = Yedeği oluştur

# Tip of the round button that shows the backup in Folders.
store-bk-show = Klasörler'de göster

# Name of the switch for the list of apps in a backup.
store-bk-apps = Uygulamalar

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] 1 Flatpak uygulamanızın listesi.
       *[other] { $n } Flatpak uygulamanızın listesi.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } Diğer bilgisayar bunları Flathub'dan yeniden kurar, böylece dosya küçük kalır.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Uygulamaların kendi verileri

# Under the Apps' own data switch.
store-bk-data-sub = Her uygulamanın ayarları ve kaydedilmiş işleri, önbellekleri hariç.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage'lar

# Under the AppImages switch.
store-bk-appimages-sub = Tek dosya oldukları için bütün olarak saklanır.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle ayarları

# Under the Trindle settings switch.
store-bk-settings-sub = Halka, duvar kâğıdı seçimi, renkler, klavye, güç, varsayılan uygulamalar ve resminiz.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Şu an { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Nereye kaydedilir

# Under that row: the folder and the file's name.
store-bk-where-sub = Ana klasörünüze, { $name } adıyla

# Heading of the card that puts a backup back.
store-bk-back-card = Yedeği geri yükle

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Bir yedek dosyası seçin

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Bir Trindle yedeği ya da { $flat } ile yapılmış bir yedek.

# SAFETY: needs a human review before it ships
# Note in the card that puts a backup back.
store-bk-back-note = Uygulamaları kurulur, verileri ve ayarları geri konur. Belgeleriniz ve resimleriniz içinde değildir: onları kendiniz kopyalayın.

# Said while a backup is being made.
store-bk-making = Yedekleniyor... bu birkaç dakika sürebilir.

# Said when a backup is made: the file and its size.
store-bk-made = Dosya: { $path } ({ $size }). Yanınızda götürmek için bir USB belleğe ya da bulut depolamaya kopyalayın.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Yedek seçilmedi, çünkü Trindle'ın dosya seçiciyle bağlantısı yok.

# Title of the window that asks which backup to use.
store-bk-choose-title = Bir yedek seçin

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle ve Flat yedekleri

# Red words when the file chooser gave no answer.
store-bk-no-answer = Yedek seçilmedi, çünkü dosya seçici yanıt vermedi.

# Said while a backup file is read.
store-bk-reading = Yedek okunuyor...

# Said when a backup was put back.
store-bk-logout = Tüm ayarları görmek için oturumu kapatıp yeniden açın.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Tüm ayarları görmek için oturumu kapatıp yeniden açın.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 uygulama, { $with } tanesinin ayarları var. Başlatılıyor...
       *[other] { $apps } uygulama, { $with } tanesinin ayarları var. Başlatılıyor...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Henüz üreticilerin kendi kaynaklarından eklenen yok.
        [one] 1 üretici kaynağı eklendi, Debian güncellemeleriyle birlikte güncel tutuluyor.
       *[other] { $n } üretici kaynağı eklendi, Debian güncellemeleriyle birlikte güncel tutuluyor.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Eklendi ve güncel tutuluyor.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. Kaynak eklendi, ama program kurulu değil.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Başka bir yolla kurulmuş.

# SAFETY: needs a human review before it ships
# Tip of the round button that removes a maker's source and its program.
store-src-remove-both = Bunu ve kaynağını kaldır

# Link to a maker's page, naming the program.
store-src-open = { $name } sayfasını aç

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Tailscale'de oturum aç

# SAFETY: needs a human review before it ships
# Tip of the round button that adds a maker's source, naming the program.
store-src-add = { $name } kaynağını ekle

# Heading of the card of makers' sources that can be added.
store-src-ready = Eklenmeye hazır

# Name of the field for the address of a source.
store-src-address = Adres

# Under the address field.
store-src-address-sub = Deponun adresi, üreticisinin kurulum yönergelerinden.

# Name of the field for the suite of a source.
store-src-suite = Dağıtım

# Under the suite field.
store-src-suite-sub = Genellikle trixie ya da stable.

# Name of the field for the components of a source.
store-src-components = Bileşenler

# Under the components field.
store-src-components-sub = Genellikle main.

# SAFETY: needs a human review before it ships
# Name of the field for a source's signing key.
store-src-key = İmza anahtarı

# SAFETY: needs a human review before it ships
# Under the signing key field.
store-src-key-sub = İmza anahtarının adresi.

# Name of the field for the program to install from a source.
store-src-program = Kurulacak program

# Under the program field.
store-src-program-sub = Şimdi kurulmasını istiyorsanız paket adı.

# Placeholder of the program field.
store-src-program-hint = isteğe bağlı

# SAFETY: needs a human review before it ships
# Tip of the round button that adds the source typed.
store-src-add-this = Bu kaynağı ekle

# Heading of the card that adds another source.
store-src-another = Başka bir kaynak ekle

# SAFETY: needs a human review before it ships
# Note in that card.
store-src-another-note = Bir kaynak tüm bilgisayar için güncelleme kurabilir, bu yüzden yalnızca güvendiğiniz üreticileri ekleyin. Her biri bir yönetici parolası ister.

# SAFETY: needs a human review before it ships
# Tip of the round button that removes a source that is already on the computer.
store-src-remove-this = Bu kaynağı kaldır

# Heading of the card of sources already on the computer.
store-src-already = Bu bilgisayarda zaten olanlar

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Her iki adres de https:// ile başlamalı

# Red words when the suite or components are not plain words.
store-src-bad-words = Dağıtım ve bileşenler düz sözcükler olmalı.

# Red words when the program is not a package name.
store-src-bad-package = Bu bir paket adı değil.

# Word on a source's dial while its job starts.
store-src-starting = Başlatılıyor

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Tailscale'in oturum açma sayfası açılıyor...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Açılan sayfada oturum açın; bu kendiliğinden biter.

# Said under Tailscale's row when it is connected.
store-src-connected = Bağlı

# SAFETY: needs a human review before it ships
# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = Uygulamalara giden bağlantı şifreli değil.

# SAFETY: needs a human review before it ships
# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = Şifreli değil

# SAFETY: needs a human review before it ships
# Words of that card.
store-bk-http-card = Bu adres şifreli değil. Yoldaki biri gönderilenleri görebilir ve yedeği başka bir yedekle değiştirebilir.

# SAFETY: needs a human review before it ships
# Label of the tick box on that card; the restore button works only once it is on.
store-bk-http-tick = Bu riski anlıyorum ve kabul ediyorum

# SAFETY: needs a human review before it ships
# Button on that card that restores the backup.
store-bk-http-go = Geri yükle

# SAFETY: needs a human review before it ships
# Button on that card that gives the restore up.
store-bk-http-cancel = İptal

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
store-flatpak-ask-uncounted = Kayıtlı verileri sayılamadı.

store-hub-warning = AppImageHub yalnızca projeleri listeler: kimse onları incelemez ya da taramaz, imzalı olmaları da gerekmez. Trindle her AppImage'ı kendi özel ev klasörü olan bir kutuda çalıştırır: başta internet ve sesi kullanabilir, dosyalarının hiçbirini görmez.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = Bir AppImage'ın nereden geldiği bir ipucudur, denetim değildir.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = GitHub'da: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = GitHub'da: { $repo }, { $stars } yıldız

# Where an AppImage comes from: a website, by its name.
store-from-site = GitHub'da değil: bir web sitesinden, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Kaynak bilinmiyor

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Web sitesini aç

# When an app was last updated.
store-age-updated = Son güncelleme: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Son güncelleme: { $date }. Altı aydan uzun süredir güncellenmedi, bu yüzden giderilmemiş güvenlik açıkları olabilir.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Son güncelleme: bilinmiyor

# When an app's project was made.
store-age-created = Oluşturulma: { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Oluşturulma: bilinmiyor

# A date the source does not give, on a line of Facts.
store-age-not-known = Bilinmiyor

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Birini eklemeden önce

# Title of the card that lists the AppImages.
store-browse-list = AppImage'lar

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] AppImageHub'da { $count } AppImage listeleniyor.
       *[other] AppImageHub'da { $count } AppImage listeleniyor.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = AppImageHub listesi getiriliyor…

# The same when the list could not be fetched.
store-browse-failed = AppImageHub listesi getirilemedi.

# The same with no internet and no copy kept.
store-browse-offline = Bu bilgisayar internete bağlı olmadığı sürece AppImageHub listesi getirilemez.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub bu türden hiçbirini listelemiyor.

# Under the rows: how many are shown of how many.
store-browse-showing = { $n } içinden { $shown } gösteriliyor.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Trindle'ın uygulamalarına ekle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Ayrıntıları ve dosyaları gör



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = güncelleme denetimi kapalı

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT, OpenAI indirmesini imzaladığında geri dönecek.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Flathub ve AppImageHub'da seçebileceğiniz { $apps }; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = AppImageHub'da seçebileceğiniz { $apps }.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = İkisi de

# The same: only Flatpaks.
store-show-flatpaks = Flatpak'ler

# The same: only AppImages.
store-show-appimages = AppImage'lar

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage'lar denetlenmez. Kimse onları incelemez ya da taramaz.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle her AppImage'ı kendine ait bir ev klasörü olan bir kutuda çalıştırır: başta internet ve sesi kullanabilir, dosyalarınızdan hiçbirini göremez.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Bir daha gösterme

# Button on that card that goes on with the install.
store-appimage-warn-ok = Tamam

# Button on that card that gives the install up.
store-appimage-warn-cancel = İptal

