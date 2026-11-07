
# machine translated (Turkish), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Sürücüler

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Bu sayfa hakkında

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Bu bilgisayarın içindeki veya takılı olan her sürücü, bölümlerinden oluşan bir çubuk olarak çizilir. Her bölüm büyüklüğü kadar geniştir ve kullanıldığı kadar dolu gösterilir; böylece dolu bir bölüm kolayca fark edilir.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Resmin altında her sürücünün bir kartı vardır. Bir bölümü açın, bağlayın veya bağlantısını kesin, ya da bir sürücüyü güvenle çıkarın veya kapatın. Bir bölümdeki kalem, değişikliklerini açar: adı, kendi klasörü ve açılışta hazır olup olmadığı, boyutu, denetlenmesi ve onarılması, biçimlendirilmesi veya silinmesi. Boş alandaki artı yeni bir bölüm oluşturur.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = Sistemin çalıştığı sürücü gösterilir ancak bir şeyleri silen veya taşıyan hiçbir değişiklik için sunulmaz. Herhangi bir sürücüde yapılan her değişiklik, her seferinde bir yöneticinin parolasını ister. Göz, bir sürücüyü veya bölümü Klasörler'den ve halkadan gizler: burada gri olarak kalır ve kilitlenmez.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Her sürücünün her bölümü, kullanıldığı kadar dolu.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Sürücüleriniz

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Sürücüler şu an okunamıyor.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Yeniden dene

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Yeniden bak

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Hiç sürücü bulunamadı. Bir sürücü takın, burada görünür.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Sürücü hizmeti

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Sistem veriyolu

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } kullanılamıyor. Şununla kurun: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Sürücülere bakılıyor…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } bozuluyor. Saklamak istediğiniz her şeyi şimdi ondan kopyalayın.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } sürücüsüne bakmak gerekiyor: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Sürücüler ve üzerlerindeki alan.
        [one] Sürücüler ve üzerlerindeki alan, ayrıca takılı bir sürücü.
       *[other] Sürücüler ve üzerlerindeki alan, ayrıca takılı { $plugged } sürücü.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Neredeyse dolu: bu bilgisayarda yalnızca { $free } boş. Yakında bazı dosyaları silin veya taşıyın.
        [one] Neredeyse dolu: bu bilgisayarda yalnızca { $free } boş, ayrıca takılı bir sürücü var. Yakında bazı dosyaları silin veya taşıyın.
       *[other] Neredeyse dolu: bu bilgisayarda yalnızca { $free } boş, ayrıca takılı { $plugged } sürücü var. Yakında bazı dosyaları silin veya taşıyın.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Doluyor: bu bilgisayarda { $free } boş.
        [one] Doluyor: bu bilgisayarda { $free } boş, ayrıca takılı bir sürücü var.
       *[other] Doluyor: bu bilgisayarda { $free } boş, ayrıca takılı { $plugged } sürücü var.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Bol yer var: bu bilgisayarda { $free } boş.
        [one] Bol yer var: bu bilgisayarda { $free } boş, ayrıca takılı bir sürücü var.
       *[other] Bol yer var: bu bilgisayarda { $free } boş, ayrıca takılı { $plugged } sürücü var.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Bu bilgisayar

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Dosyalarınız

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] Sistemin bulunduğu yerde { $free } hâlâ boş.
       *[files] Kendi dosyalarınız için { $free } hâlâ boş.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: %{ $full } dolu, { $whole } içinde { $free } boş

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = / { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Okuma yok, sürücüler durumlarını bildirmiyor

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Sağlıklı, sürücünün kendisinin bildirdiği gibi

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] Sağlıklı, { $n } sürücünün hepsinin bildirdiği gibi
       *[other] Sağlıklı, { $n } sürücünün hepsinin bildirdiği gibi
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } sürücüye bakmak gerekiyor, kartına bakın
       *[other] { $n } sürücüye bakmak gerekiyor, kartlarına bakın
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 sürücü, toplam { $total }. { $health }.
       *[other] { $n } sürücü, toplam { $total }. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } boş

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Bağlı değil

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Boş alan

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = İçinde disk veya kart yok

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Üzerinde henüz bir şey yok

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB veya çıkarılabilir

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, bu bilgisayarın içinde

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB veya çıkarılabilir, gizli

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, bu bilgisayarın içinde, gizli

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, %{ $full } dolu, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Bölüm

# Name given to a drive that gives no maker or model.
disks-drive-name = Sürücü

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Takas

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Şifreli, kilidi açık

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Şifreli, kilitli

# Kind of a part that holds other parts.
disks-kind-container = Başka bölümler içerir

# Kind of a part that has no filesystem on it.
disks-kind-none = Dosya sistemi yok

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, bilgisayarın başladığı bölüm
        [legacy] { $kind }, eski bir bilgisayarın başladığı bölüm
       *[loader] { $kind }, bir önyükleyicinin kendi bölümü
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = her bilgisayar

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = yalnızca Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = eski aygıtlar

# Trouble a drive reports about its spare space.
disks-trouble-spare = yedek alanı tükeniyor

# Trouble a drive reports about its heat.
disks-trouble-temperature = çok sıcak

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = eskiyor

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = kendini salt okunur yaptı

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = bellek yedeği bozuldu

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = bir kısmı kendini salt okunur yaptı

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = üreticinin beklediğinden fazla yıpranmış

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } hasarlı nokta
       *[other] { $n } hasarlı nokta
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] { $hours } saat kullanılmış
       *[other] { $hours } saat kullanılmış
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Bozuluyor: saklamak istediğiniz her şeyi şimdi ondan kopyalayın

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Dikkat gerekiyor: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Sağlıklı

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Yanıt veren sürücü yok

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Kutu yanıt veriyor, ancak içindeki sürücü vermiyor. Çıkarın, on saniye bekleyin ve kendi kablosuyla mavi veya USB-C bir bağlantı noktasına takın; böyle kalırsa içindeki sürücü gevşemiş veya bozulmuş olabilir.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Güvenle çıkar

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Kapat

# SAFETY: needs a human review before it ships
# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
disks-erase-whole = Sürücünün tamamını sil

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Bu sürücüyü göster
       *[part] Bu bölümü göster
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Bu sürücüyü gizle. { $lock }
       *[part] Bu bölümü gizle. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Klasörler'den ve halkadan gizli. { $lock }

# SAFETY: needs a human review before it ships
# Note on the card of the drive the system runs from.
disks-system-note = Sistem bundan çalışır: burada hiçbir şey silinemez.

# Note on the card of a drive that has no partition table.
disks-no-table = Bölüm tablosu yok: sürücünün tamamı boş.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = bağlı değil

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = açılışta { $dir } içine bağlanır

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = { $dir } içinde görünür

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = { $dir } içinde

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Aç

# Tip of the round button that mounts a part.
disks-mount = Bağla

# Tip of the round button that unmounts a part.
disks-unmount = Bağlantıyı kes

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Değiştir

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Ad

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Ad (isteğe bağlı)

# Tip of the button that saves a part's new name.
disks-rename-tip = Yeniden adlandır

# Note under the Name row of a part.
disks-name-note = Burada ve diğer bilgisayarlarda bu adla anılır.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Klasör adı

# Tip of the button that saves a part's folder.
disks-use-folder = Bu klasörü kullan

# Label of the row where a part's folder is chosen.
disks-folder = Klasör

# Note under the Folder row of a part.
disks-folder-note = Dosyalarının göründüğü yer. Bilgisayarın seçmesi için boş bırakın.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Açılışta bağla

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Bilgisayar her başladığında hazır olur. Orada değilse bilgisayar yine de başlar.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = GB cinsinden boyut

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = { $least } ile { $most } arasında.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = { $least } ile { $most } arasında. Ne kadar küçülebileceğini görmek için bağlayın.

# Tip of the button that resizes a part.
disks-resize-tip = Yeniden boyutlandır

# Label of the row where a part is resized.
disks-size = Boyut

# SAFETY: needs a human review before it ships
# Said above the Format and Erase buttons at all times. { $words } names what is erased.
disks-format-erases = Bu, { $words } üzerindeki her şeyi siler. Geri alınamaz.

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning = Yeniden tıklayın: { $drive } üzerindeki { $name }, { $from } boyutundan { $to } boyutuna geçer

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Yeni bir bölüm için yer aç

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = GB cinsinden yer

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Yer aç

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
disks-room-warning = Yeniden tıklayın: { $drive } üzerindeki { $name }, bir anlık görüntüden sonra { $size } kaybeder

# Note under the row that makes room.
disks-room-note = Kullanılırken, bir anlık görüntüden sonra küçültülür ve boş alan bırakılır.

# Tip of the button that checks a part for faults.
disks-check-tip = Hatalara karşı denetle

# Tip of the button that repairs a part.
disks-repair-tip = Onar

# Label of the row where a part is checked and repaired.
disks-faults = Hatalar

# Note under the Faults row.
disks-faults-note = Dosyalarının saklanma biçimindeki hataları arar ve düzeltir. Bu çalışırken bağlantısı kesilir.

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
disks-repair-warning = { $drive } üzerindeki { $name } bölümünü onarmak için yeniden tıklayın

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Tür

# SAFETY: needs a human review before it ships
# Label of the row where a part is formatted.
disks-format = Biçimlendir

# SAFETY: needs a human review before it ships
# Tip of the button that formats a part.
disks-format-tip = Biçimlendir

# Note under the row that erases the whole drive.
disks-format-whole = Sürücüyü tamamını dolduran tek bir bölüm olarak baştan başlatır.

# Note under the row that formats one part.
disks-format-part = Yalnızca bu bölümdeki her şeyi siler.

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the format or erase button. { $words } names what is erased.
disks-format-warning = Yeniden tıklayın: { $words } üzerindeki her şey silinir

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $drive } üzerindeki { $name }, { $size }

# SAFETY: needs a human review before it ships
# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-delete-name = { $drive } üzerindeki { $name } ({ $size }) bölümünü sil

# SAFETY: needs a human review before it ships
# Label of the row where a part is deleted.
disks-delete-label = Bu bölümü sil

# SAFETY: needs a human review before it ships
# Note under the row that deletes a part.
disks-delete-note = Alanı boşalır; yeni bir bölüm için veya öncekinin büyümesi için hazır olur.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Burada yeni bir bölüm oluştur

# Label of the row where a new part is made.
disks-new-part = Yeni bölüm

# Note under the row where a new part is made.
disks-new-part-note = Bu boş alanda oluşturulur ve kullanıma hazır biçimlendirilir.

# Tip of the button that makes a new part.
disks-make-tip = Oluştur

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = { $drive } üzerinde { $size } boyutunda bir bölüm oluşturmak için yeniden tıklayın

# SAFETY: needs a human review before it ships
# Title of the box for erasing a whole drive. { $name } is the drive.
disks-sheet-erase = { $name } sürücüsünü sil

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $drive } üzerinde, { $size }

# Title of the box for making a new part in free space.
disks-sheet-new = Yeni bölüm

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $drive } üzerinde, { $size } boş

# Tip of the round button that closes the box of changes.
disks-close = Kapat

# Words on a row while a part is being mounted.
disks-mounting = Bağlanıyor

# Words on a row when a part has been mounted.
disks-mounted = Bağlandı

# Words on a row while a part is being unmounted.
disks-unmounting = Bağlantısı kesiliyor

# Words on a row when a part has been unmounted.
disks-unmounted = Bağlantısı kesildi

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Kapatılıyor

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Çıkarmak güvenli

# Words on a drive's row while it is being powered off.
disks-powering-off = Kapatılıyor

# Words on a drive's row when it has been powered off.
disks-off-safe = Kapalı: çıkarmak güvenli

# Words on a row while a part is being renamed.
disks-renaming = Yeniden adlandırılıyor

# Words on a row when a part has been renamed.
disks-renamed = Yeniden adlandırıldı

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = Klasörünü yine bilgisayar seçer

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = { $dir } içinde görünür

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Açılışta { $dir } içine bağlanır

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Açılışta dokunulmaz

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Kaydediliyor

# Words on a row while a part is being resized.
disks-resizing = Yeniden boyutlandırılıyor

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = { $size } boyutuna getirildi

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Dosyalar bulunamadı, bu yüzden yer açılamaz.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Anlık görüntü alınıyor

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Anlık görüntü sonradan bulunamadı.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Hiçbir şey yeniden boyutlandırılmadı.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Anlık görüntü alınırken bir şeyler değişti, bu yüzden hiçbir şey yeniden boyutlandırılmadı.

# Words on a row while room is being made.
disks-making-room = Yer açılıyor

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } yer açıldı

# Words on a row while a part is being repaired.
disks-repairing = Onarılıyor

# Words on a row when a part has been repaired.
disks-repaired = Onarıldı

# Red words when some faults could not be put right.
disks-repair-failed = Bazı hatalar düzeltilemedi. Saklamak istediğiniz her şeyi kopyalayın.

# Words on a row while a part is being checked.
disks-checking = Denetleniyor

# Words on a row when the check found no faults.
disks-no-faults = Hata bulunamadı

# Red words when the check found faults.
disks-faults-found = Hata bulundu: onarmak için anahtar simgesine basın.

# SAFETY: needs a human review before it ships
# Words on a row while a part or drive is being formatted.
disks-formatting = Biçimlendiriliyor

# SAFETY: needs a human review before it ships
# Words on a row when a part has been formatted.
disks-formatted = Biçimlendirildi

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = sürücünün tamamını silme

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Yeni bölüm bulunamadı.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Sürücü hizmetine ulaşılamadı.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Yeni bölümün tanımlayıcısı okunamadı.

# SAFETY: needs a human review before it ships
# Words on a row while a part is being deleted.
disks-deleting = Siliniyor

# SAFETY: needs a human review before it ships
# Words on a row when a part has been deleted.
disks-deleted = Silindi

# Words on a row while a new part is being made.
disks-making-it = Oluşturuluyor

# Words on a row when a new part has been made.
disks-made = Oluşturuldu

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Bu saklanamadı: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Önce bir ad verin.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] Bu ad çok uzun: en çok { $max } harf.
       *[other] Bu ad çok uzun: en çok { $max } harf.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Yalnızca harf, rakam, - ve _ kullanın (boşluk yok).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Bir harf veya rakamla başlatın.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } zaten kullanımda.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] Bu ad bu tür için çok uzun: en çok { $max } harf.
       *[other] Bu ad bu tür için çok uzun: en çok { $max } harf.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT adları * ? . , ; : / \ | + = < > [ ] veya " içeremez.

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Önce bağlayın: bu tür yalnızca bağlıyken büyütülebilir.
       *[smaller] Önce bağlayın: bu tür yalnızca bağlıyken küçültülebilir.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Bu tür büyütülemez.
       *[smaller] Bu tür küçültülemez.
    }

# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password. Said whole, as a sentence.
disks-no-password = Parola verilmedi, bu yüzden hiçbir şey değişmedi.

# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password, after some steps of a change were already done.
disks-no-password-part-way = Parola verilmedi, bu yüzden iş yarıda kaldı.

# Red words when another change is pressed while one is still being made.
disks-change-going = Bir değişiklik yapılıyor.

disks-one-at-a-time = Başka bir değişiklik hâlâ yapılıyor. Bittiğinde yeniden deneyin.

# SAFETY: needs a human review before it ships
# Red words when it could not be made sure that the password would be asked for.
disks-not-sure = Parolanın sorulacağından emin olunamadı, bu yüzden hiçbir şey değişmedi.

# SAFETY: needs a human review before it ships
# Red words after how a change ended, when the password just given could not be taken back.
disks-standing = Az önce verilen parola geri alınamadı; bu yüzden birkaç dakika boyunca bir sürücüde yapılacak başka bir değişiklik parola istemeyebilir. Yeniden deneniyor.

# SAFETY: needs a human review before it ships
# Words after how a change ended, when the password just given has been taken back after all.
disks-taken-back = Verilen parola şimdi geri alındı: bir sonraki değişiklik parola isteyecek.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Sürücü hizmetine ulaşılamadı, bu yüzden hiçbir şey değişmedi.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Bu değişiklik yapılırken Sürücüler kapatıldı.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Nasıl bittiğini söyleyebilmek için bu bitene kadar Sürücüler açık kalır.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, bu yüzden hiçbir şey değişmedi.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, bu yüzden iş yarıda kaldı.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Sürücü hizmetine sorulamadı

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } İş yarıda kaldı.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Yapıldı: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Başarısız: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Yapılmadı: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Bilinmiyor: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } bunun için bağlantısı kesildi ve şimdi bağlı değil.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } bunun için bağlantısı kesildi ve hâlâ bağlı değil: satırından bağlayın.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } bağlantısı kesilmiş olabilir: satırına bakın.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } bunun için bağlantısı kesildi ve yeniden bağlamak ayrı bir parola gerektiriyor: satırından bağlayın.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } bunun için bağlantısı kesildi ve yeniden bağlanamadı ({ $why }): satırından bağlayın.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Biçimlendirildi, ancak { $dir } klasörü yeniden verilmedi. { $why } Klasörü satırından yeniden seçin.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Bir şey hâlâ onu kullanıyor. Dosyalarını gösteren pencereleri veya programları kapatın, sonra yeniden deneyin.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Üzerindekiler için istenen boyutta yeterli yer yok.

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } bağlantısını kesme

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } yeniden bağlama

# Step of a change.
disks-step-eject = çıkarılacak sürücüyü kapatma

# Step of a change.
disks-step-power-off = sürücüyü kapatma

# Step of a change. { $dev } is the part.
disks-step-format = { $dev } biçimlendirme

# Step of a change.
disks-step-create = yeni bölümü oluşturma

# Step of a change. { $dev } is the part.
disks-step-delete = { $dev } silme

# Step of a change. { $dev } is the part.
disks-step-resize-part = { $dev } bölümünü yeniden boyutlandırma

# Step of a change. { $dev } is the part.
disks-step-resize = { $dev } üzerindekileri yeniden boyutlandırma

# Step of a change. { $dev } is the part.
disks-step-repair = { $dev } onarma

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } denetleme

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } yeniden adlandırma

# Step of a change. { $dev } is the part.
disks-step-fstab = { $dev } klasör ve açılış seçimini kaydetme

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = { $dev } üzerinde { $method } adımı

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Bu { $kind } bölümü yalnızca kullanımda değilken küçültülebilir, bu yüzden buradan yer açılamaz.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Bu xfs bölümü küçültülemez.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Bu { $kind } bölümü kullanımdayken küçültülemez.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Anlık görüntü yalnızca sistemin kendi diskinden alınabilir, bu yüzden burada yer açılamaz.

# Red words under the Make room row.
disks-room-not-in-table = Bir sürücünün tablosundaki bir bölüm değil, bu yüzden bundan yer açılamaz.

# Red words under the Make room row.
disks-room-no-online-shrink = Sürücü hizmeti burada kullanımdayken Btrfs'yi küçültemez, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-holds-parts = Başka bölümler içeriyor, bu yüzden bundan yer açılamaz.

# Red words under the Make room row.
disks-room-not-gpt = Sürücüsünde GPT tablosu yok, bu yüzden bundan yer açılamaz.

# Red words under the Make room row.
disks-room-mount-first = Önce bağlayın: Btrfs bağlıyken küçültülür.

# Red words under the Make room row.
disks-room-mounts-not-one = Tam olarak tek bir yere bağlı değil, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-read-only = Salt okunur bağlı, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-read-only-unknown = Salt okunur olup olmadığı anlaşılamadı, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-many-drives = Dosyalar birden fazla sürücüye yayılmış, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-drives-unknown = Dosyaların kaç sürücüye yayıldığı anlaşılamadı, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-swap-file = Üzerinde bir takas dosyası var, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-swap-unknown = Üzerinde bir takas dosyası olup olmadığı anlaşılamadı, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-used-unknown = Ne kadarının kullanımda olduğu okunamadı, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-area-unknown = Sürücü hizmeti dosyaların alanının ne kadar büyük olduğunu söylemiyor, bu yüzden denetlenemedi ve yer açılamaz.

# Red words under the Make room row.
disks-room-open-missing = Açık kopyası bulunamadı, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-encryption-sizes = Şifreleme için verilen boyutlar uyuşmuyor, bu yüzden yer açılamaz.

# Red words under the Make room row.
disks-room-sizes = Bunun için verilen boyutlar uyuşmuyor, bu yüzden yer açılamaz.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = { $used } içeriyor ve bundan fazlasının saklanması gerekiyor, bu yüzden çok az yer boşaltılabilir.

# Red words under the Make room row.
disks-room-sizes-unread = Boyutları okunamadı.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = { $least } ile { $most } arasında yer açılabilir.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, bu yüzden iş durduruldu.

# What was found wrong while making room.
disks-room-check-unmounted = Dosyalar artık bağlı değil

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = Bölüm { $is } boyutunda ve { $should } olmalı

# What was found wrong while making room.
disks-room-check-open = Şifrelemenin açık kopyası olması gereken boyutta değil

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = Dosyaların alanı { $is } boyutunda ve { $should } olmalı

# What was found wrong while making room.
disks-room-check-area-small = Dosyaların alanı kullanımdakinden küçük

# What was found wrong while making room.
disks-room-check-fit = İçeriği taşıyan alan artık sığmıyor

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] Bölüm artık sürücü hizmetinin listesinde yok, bu yüzden iş durduruldu.
        [copy] Açık kopya artık sürücü hizmetinin listesinde yok, bu yüzden iş durduruldu.
       *[files] Dosya sistemi artık sürücü hizmetinin listesinde yok, bu yüzden iş durduruldu.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = dosyaların alanını küçültme

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = şifrelemenin açık kopyasını küçültme

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = bölümü küçültme

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = boyutları denetleme

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = { $dev } altında yer açmadan önce ({ $at })

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Yeniden tıklayın: { $drive } üzerindeki { $name } { $from } boyutundan { $to } boyutuna düşecek. Önce dosyalarını yedekleyin, çünkü küçültme sırasında elektrik kesilirse dosyalar kaybolabilir.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name } üzerindeki Windows hazırda bekletilmiş ya da hızlı başlatma durumunda bırakılmış, bu yüzden küçültmek dosyalarının kaybolmasına yol açabilir. Windows'u başlatın, Yeniden başlat'ı seçin, sonra yeniden deneyin.

