
# machine translated (Turkish), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Bu bilgisayar hakkında

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = ana bilgisayar adı bilgisayar adı ağ adı hakkında sistem sürüm debian trindle bellek ram işlemci cpu disk grafik ekran model çekirdek sürekli sürüm etkinlik çalışan programlar meşgul görev yöneticisi sistem izleyici diskler sürücüler usb alan

# Text behind the info button of the About page.
settings-about-help = Bu sayfa, bu bilgisayarın ağdaki diğer bilgisayarlara görünen adını, neyden oluştuğunu ve ne kadar sürede açıldığını gösterir. Etkinlik, bilgisayarın şu anda ne yaptığını ve hangi programların belleği ve işlemciyi kullandığını gösterir. Sürücüler, sürücülerinizi ve üzerlerindeki alanı gösterir. Her biri kendi penceresinde açılır. Açmak için Daha fazla ayrıntı bölümündeki okları kullanın.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Bilinmiyor

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } iş parçacığı
       *[other] { $model }, { $cores } iş parçacığı
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel grafik

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD grafik

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA grafik

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Sanal makine grafiği

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Grafik

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }, { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } kullanımda

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } boş

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = toplam { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, { $on } önce açıldı.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, { $on } önce açıldı. Disk neredeyse dolu.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } saniye

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } dakika

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } dakika
       *[other] { $n } dakika
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } saat
       *[other] { $n } saat
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } gün
       *[other] { $n } gün
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Yalnızca harf, rakam ve tire kullanın. Tireyle başlamayın ya da bitirmeyin.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Parolanız bekleniyor…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Bilgisayarın adı değiştirilemedi.

# Words under the computer name box after renaming worked.
settings-about-name-done = Zaten açık olan programlar, bir sonraki oturum açışınızdan sonra yeni adı kullanır.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Bilgisayar adı

# Tip of the save button beside the computer name box.
settings-about-rename = Bu bilgisayarın adını değiştir

# Bold line of the computer name setting.
settings-about-name = Bilgisayar adı

# Heading of the card with the computer's name.
settings-about-name-group = Ad

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Bilgisayarın ağdaki ve Terminal'deki komut satırındaki adı budur. Harf, rakam ve tire kullanın, ama tireyle başlamayın ya da bitirmeyin. Değiştirmek için parolanızı girmeniz gerekir.

# Heading of the card about the computer's hardware.
settings-about-hardware = Bilgisayar

# Text behind the info button of the hardware card.
settings-about-hardware-about = Burada bilgisayarın üreticisi ve modeli, işlemcisi ve grafikleri gösterilir. Ayrıca işlemcinin aynı anda kaç iş parçacığı çalıştırabildiği ve kullanılan her ekranın boyutu ile yenileme hızı da gösterilir.

# Heading of the card about the software.
settings-about-software = Yazılım

# Text behind the info button of the software card.
settings-about-software-about = Trindle, Debian üzerinde çalışan bir masaüstüdür; Debian alttaki programlara bakar. Çekirdek, donanımla çalışan parçadır. Masaüstünü Trindle'ın kendi birleştiricisi Wayland kullanarak çizer.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle'ın kendi birleştiricisi

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } uygulama
       *[other] { $n } uygulama
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Daha fazla ayrıntı

# Text behind the info button of the More detail card.
settings-about-more-about = Etkinlik, bilgisayarın şu anda ne yaptığını ve hangi programların belleği ve işlemciyi kullandığını gösterir. Sürücüler, her sürücüyü, ne kadar dolu ve sağlıklı olduğunu ve USB bellekleri gösterir.

# Bold line of the setting that opens Activity.
settings-about-activity = Nelerin çalıştığını gör

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Etkinlik

# Bold line of the setting that opens Drives.
settings-about-drives = Sürücüler ve alan

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Sürücüler

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle açılış sürelerini İndirilenler klasörüne kaydedemedi.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Tarih ve saat,Saniye

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle başlangıç süreleri.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = İndirilenler klasöründe, { $file } olarak

# Label of the word button that saves every start-up time.
settings-about-times-save = Kaydet

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Tüm açılış sürelerini İndirilenler klasörüne kaydet

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Her açılış süresini kaydet

# Quiet line under that setting.
settings-about-times-keep-sub = İndirilenler klasörüne elektronik tablo olarak kaydedilir.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Her sütun bir açılıştır. En yenisi sağdadır.

# Heading of the card with the last ten start-up times.
settings-about-times = Son on açılış

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle her açılışı güç düğmesinden oturum açma ekranına kadar ölçer. Disk parolasını yazarken geçen süre sayılmaz. En yenisi en üsttedir.

# Heading of the memory tile.
settings-about-memory = Bellek

# Heading of the disk tile.
settings-about-disk = Disk

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Sorun raporları

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Raporlar, Trindle'ın yapımına yardım eden topluluk Buildmorphic'e gider. Yalnızca bir e-posta adresiyle katılabilirsiniz. İlk siz bildirdiğiniz sorunlar için, çözemeseniz bile, puan ve seviye kazanırsınız. Bir rapor yalnızca sabit bir ayrıntı listesi içerir ve siz Gönder'e basmadan hiçbir şey gönderilmez. Kullanıcı adı ya da GitHub adı isteğe bağlıdır ve kullanıcı adınız hiçbir zaman e-posta adresiniz olmaz.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Bir şey ters gittiğinde rapor göndermeyi öner

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic kullanıcı adı

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub adı

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Topluluk hakkında

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Kapalı. Bekleyen { $n } rapor silindi.
       *[other] Kapalı. Bekleyen { $n } rapor silindi.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Kapalı. Bekleyen { $n } rapor silinemedi.
       *[other] Kapalı. Bekleyen { $n } rapor silinemedi.
    }



# Heading of the card with the switch for looking for updates.
settings-about-updates = Güncellemeler

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle, oturum açtıktan kısa süre sonra ve ardından birkaç saatte bir uygulamaların, eklentilerin, aygıt yazılımının ve kendisinin yeni sürümlerini denetler. Bunu kapatırsanız Trindle yalnızca Uygulamalar'da Güncellemeleri denetle düğmesine ya da halkadaki güncelleme düğmesine bastığınızda denetler. Debian'ın güvenlik düzeltmeleri her gün kurulur ve kendi anahtarı vardır. Onu kapatırsanız düzeltmeler kendiliğinden kurulmaz ve bilinen güvenlik açıkları siz güncelleyene kadar kapatılmadan kalır. İki anahtardan birini değiştirmek yönetici parolanızı gerektirir, böylece bilgisayarın tamamı seçiminizi kullanır.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Güncellemeleri denetle

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Parolanız bekleniyor…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Güvenlik güncellemeleri

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Güvenlik düzeltmeleri artık kendiliğinden kurulmuyor.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Kapalı, ancak Reporting eklentisinin bekleyen raporları silinmedi.
