
# machine translated (Turkish), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Bu bilgisayar hakkında

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = ana bilgisayar adı bilgisayar adı ağ adı hakkında sistem sürüm debian trindle bellek ram işlemci cpu disk grafik ekran model çekirdek sürekli sürüm etkinlik çalışan programlar meşgul görev yöneticisi sistem izleyici diskler sürücüler usb alan

# Text behind the info button of the About page.
settings-about-help = Bu bilgisayarın ağdaki diğer bilgisayarlara görünen adı, neyden oluştuğu ve ne kadar sürede açıldığı. Şu anda ne yaptığı ve hangi programların belleği ve işlemciyi kullandığı Etkinlik'te, sürücüler ve üzerlerindeki alan ise Sürücüler'de; her biri ayrı bir pencere: Daha fazla ayrıntı bölümündeki oklar bunları açar.

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
settings-about-name-bad = Yalnızca harf, rakam ve tire kullanılabilir; tireyle başlayamaz ya da bitemez.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Parola bekleniyor...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Bilgisayarın adı değiştirilemedi.

# Words under the computer name box after renaming worked.
settings-about-name-done = Zaten açık olan programlar yeni adı bir sonraki oturum açışta görür.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Bilgisayar adı

# Tip of the save button beside the computer name box.
settings-about-rename = Bu bilgisayarın adını değiştir

# Bold line of the computer name setting.
settings-about-name = Bilgisayar adı

# Quiet line under the computer name setting.
settings-about-name-sub = Harf, rakam ve tire.

# Heading of the card with the computer's name.
settings-about-name-group = Ad

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Bu bilgisayarın ağda ve Terminal'deki komut satırında nasıl göründüğü. Harf, rakam ve tire kullanılabilir; tireyle başlayamaz ya da bitemez. Değiştirmek için parolanız istenir.

# Heading of the card about the computer's hardware.
settings-about-hardware = Bilgisayar

# Text behind the info button of the hardware card.
settings-about-hardware-about = Üretici ve model, işlemci ve aynı anda kaç iş parçacığı çalıştırdığı, grafik ve kullanılan her ekranın boyutu ile yenileme hızı.

# Quiet line under the maker and model.
settings-about-hw-computer = Bilgisayar

# Quiet line under the processor.
settings-about-hw-processor = İşlemci

# Quiet line under the graphics.
settings-about-hw-graphics = Görüntüleri çizer

# Quiet line under the screens.
settings-about-hw-screen = Ekran

# Heading of the card about the software.
settings-about-software = Yazılım

# Text behind the info button of the software card.
settings-about-software-about = Trindle, Debian üzerinde çalışan bir masaüstüdür; Debian alttaki programlara bakar. Çekirdek, donanımla konuşan parçadır; masaüstünü ise Trindle'ın kendi birleştiricisi Wayland üzerinde çizer.

# Quiet line under the Debian version.
settings-about-sw-debian = Altta çalışır

# Quiet line under the kernel version.
settings-about-sw-kernel = Çekirdek

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle'ın kendi birleştiricisi

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Masaüstünü Wayland üzerinde çizer

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } uygulama
       *[other] { $n } uygulama
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Kurulu, halkada hazır

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Daha fazla ayrıntı

# Text behind the info button of the More detail card.
settings-about-more-about = Şu anda ne yaptığı ve hangi programların belleği ve işlemciyi kullandığı Etkinlik'te. Her sürücü, ne kadar dolu ve sağlıklı olduğu ve USB bellekler Sürücüler'de.

# Bold line of the setting that opens Activity.
settings-about-activity = Nelerin çalıştığını gör

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Belleği ve işlemciyi kullanan programlar.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Etkinlik

# Bold line of the setting that opens Drives.
settings-about-drives = Sürücüler ve alan

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Her sürücü, ne kadar dolu ve sağlıklı olduğu.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Sürücüler

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Açılış süreleri İndirilenler klasörüne kaydedilemedi.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = İndirilenler klasöründe, { $file } olarak

# Label of the word button that saves every start-up time.
settings-about-times-save = Kaydet

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Tüm açılış sürelerini İndirilenler klasörüne kaydet

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Her açılış, saklamak için

# Quiet line under that setting.
settings-about-times-keep-sub = İndirilenler klasörüne elektronik tablo olarak kaydedilir.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Her sütun bir açılıştır; en yenisi sağdadır.

# Heading of the card with the last ten start-up times.
settings-about-times = Son on açılış

# Text behind the info button of the start-up times card.
settings-about-times-about = Her açılış, güç düğmesinden oturum açma ekranına kadar ölçülür; disk parolasını yazarken geçen süre dahil değildir. En yenisi en üsttedir.

# Heading of the memory tile.
settings-about-memory = Bellek

# Heading of the disk tile.
settings-about-disk = Disk



# Heading of the card with the switch for looking for updates.
settings-about-updates = Güncellemeler

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle, oturum açtıktan kısa süre sonra ve birkaç saatte bir uygulamaların, eklentilerin, aygıt yazılımının ve kendisinin yeni sürümlerine bakar; Debian'ın güvenlik düzeltmeleri de her gün kurulur. Kapatılırsa Trindle yalnızca Uygulamalar'da Güncellemeleri denetle düğmesine ya da halkadaki güncelleme düğmesine bastığınızda bakar. Debian'ın güvenlik düzeltmelerinin kendi anahtarı vardır; kapatılırsa kendiliğinden kurulmazlar ve bilinen hatalar siz güncelleyene kadar açık kalır. İki anahtardan birini değiştirmek yönetici parolanızı ister, böylece bilgisayarın kendi hizmetleri de seçimi görür.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Güncellemeleri denetle

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Parola bekleniyor...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Kapalıyken siz istemedikçe hiçbir şey bakmaz.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Güvenlik güncellemeleri

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Güvenlik düzeltmeleri her gün kendiliğinden kurulur.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Güvenlik düzeltmeleri artık kendiliğinden kurulmuyor.

