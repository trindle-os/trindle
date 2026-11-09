
# machine translated (Turkish), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
# SAFETY: needs a human review before it ships
store-refusal-nothing = Hiçbir şey geri konmadı, çünkü { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
# SAFETY: needs a human review before it ships
store-refusal-part = Yedeğin bir kısmı geri konmadı, çünkü { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = yedek, içine konacağı klasörün dışına uzanan bir bağlantı ya da başka bir şey içeriyor

# The reason when something on this computer stands where part of the backup needs to go.
# SAFETY: needs a human review before it ships
store-because-in-the-way = bu bilgisayarda yedeğin bir kısmının gideceği yerde zaten bir şey var

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = yedeği açacak bir yer yok

# The reason when the backup file cannot be read.
store-because-unreadable = yedek dosyası okunamadı

# The reason when the file is not a Trindle backup.
store-because-not-trindle = bu dosya bir Trindle yedeği değil

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = yedek bozuk ve listesi okunamadı

# The reason when the file is not a Flat backup.
store-because-not-flat = bu dosya bir Flat yedeğine benzemiyor

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = bu dosyada Flat uygulama listesi yok

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = bu, Trindle'ın okuyabileceğinden daha yeni bir Flat ile yapılmış

# The reason when the file holds no apps.
store-because-no-apps = bu dosyada uygulama yok

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = yedek okunamadı: { $why }

# The reason when the program gave none.
store-because-none = hiçbir neden vermedi

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = bu bilgisayar Trindle'ın oraya yazmasına izin vermedi

# The reason when the disk is full.
store-because-space = bu bilgisayarda yer kalmadı

# The reason when a part of the computer is read-only.
store-because-read-only = bu bilgisayarın o kısmı salt okunur

# The reason when the person's disk quota is used up.
store-because-quota = bu hesaba ayrılan alan doldu

# The reason when a file or program is missing.
store-because-missing = gereken bir dosya ya da program eksik

# The reason when no reason could be read from the program's last line.
store-because-failed = başarısız oldu

# The reason when the backup has no name to be written under.
store-because-no-name = adı yok

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = yedekteki ayarları kendi klasörünün dışına uzanıyor

# The reason when something is too big to be read whole.
store-because-too-big = bütün olarak okunamayacak kadar büyük

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = yedekteki ayarları sağlama toplamıyla uyuşmuyor

# Said when the person's account picture was not put back; the reason follows.
# SAFETY: needs a human review before it ships
store-refusal-picture-only = Geri konuldu, hesap resminiz hariç. { $why }

# Said when something else failed and the account picture was not put back either.
# SAFETY: needs a human review before it ships
store-refusal-picture-too = { $words } Hesap resminiz de geri konmadı. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec başlatılamadı: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Yedekten okunamadı: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Yedek alınmadı, çünkü { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = henüz yok

# Beside the size of a part of the backup: about how big it is.
store-backup-about = yaklaşık { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindle ayarları ve AppImage dosyaları geri konuluyor...

# Step shown while the settings and data are put back.
store-backup-step-data = Ayarlar ve veriler geri konuluyor...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Flathub'dan { $count } uygulama kuruluyor... bu biraz sürebilir.
       *[other] Flathub'dan { $count } uygulama kuruluyor... bu biraz sürebilir.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Ayarlar ve veriler geri konuldu, ama şu uygulamalar kurulmadı, çünkü Flatpak bunları reddetti: { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Ayarlar ve veriler geri konuldu, ama şu uygulamalar kurulmadı, çünkü Flatpak bunları reddetti: { $failed }. Kurulmayanlar ayrıca: { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Ayarlar ve veriler geri konuldu, ama şu uygulamalar kurulmadı: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] 1 uygulama atlandı: adı geçerli değildi.
       *[other] { $n } uygulama atlandı: adları geçerli değildi.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = { $name } kaynağı şifreli değil ve kabul edilmedi, bu yüzden kullanılmadı

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } kaynağı bu bilgisayarda yok ve yedek nerede olduğunu söylemiyor

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name } ayarları (çünkü yedekteki kopyası denetimlerini geçemedi)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (çünkü kaynağının adı geçerli değil)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (çünkü { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (çünkü Flatpak onu reddetti: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = her şey (çünkü yedeği açacak bir yer yok)

# Said among what could not be done: the settings that apply to every app.
# SAFETY: needs a human review before it ships
store-flat-every-app = her uygulamaya verilen izinler

# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name } ayarları

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Önceden orada olan { $path } olarak saklandı: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } kuruluyor ({ $at }/{ $total })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = { $name } ayarları geri konuluyor ({ $at }/{ $total })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } uygulama kuruldu.
       *[other] { $n } uygulama kuruldu.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = { $n } uygulamanın ayarları geri konuldu.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } tanesi zaten vardı, olduğu gibi bırakıldı.
       *[other] { $n } tanesi zaten vardı, olduğu gibi bırakıldı.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Geri konmayanlar: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

