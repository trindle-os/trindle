
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Anlık görüntüler

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = anlık görüntü anlık görüntüler snapshot geri yükle geri al yedek güncelleme bozuldu btrfs snapper önyükleme menüsü grub önceki sürümler eski sürüm silinen dosya geri koy kurtar ev kopyalar

# Help line at the foot of the Snapshots page.
# SAFETY: needs a human review before it ships
settings-snap-help = Anlık görüntü, sistemin bir kopyasıdır: her yazılım değişikliğinden önce ve sonra, bir de siz her aldığınızda alınır. Bir anlık görüntüyü geri yüklemek, bir sonraki yeniden başlatmada tüm sistemi eski haline döndürür. Ev klasörünüzün diskte kendine ait bir bölümü varsa (Trindle'ın kurucusu böyle kurar) kendi dosyalarınız olduğu gibi kalır. Bilgisayar açılmazsa önyükleme menüsünden Trindle anlık görüntüleri'ni seçin. Önceki sürümler, sistemin anlık görüntülerinden ayrı olarak herkesin kendi dosyalarının günlük bir kopyasını saklar.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Disk neredeyse dolu olduğu için Trindle kopya almadı.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = bugün saat { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = dün saat { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date } tarihinde

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = Bilgisayar yeniden başlatılamadı. Bunun yerine halkadan yeniden başlatın.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Anlık görüntü alınıyor

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Boşaltılıyor…

# Said beside the set aside line once the space is freed.
settings-snap-freed = Boşaltıldı

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Geri yükleniyor

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Bir sonraki yeniden başlatmada sistem, { $n } numaralı anlık görüntüdeki haline dönecek. O zamandan beri değişen her şey bir kenara ayrılır, kaybolmaz.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Siliniyor

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Silindi

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Parolanız bekleniyor…

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Her gün bir kopya alınır

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Artık kopya alınmaz

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Saklanır

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = Temizleniyor

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = Temizlendi

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Henüz kopya yok
        [one] Bir kopya
       *[other] { $n } kopya
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = İlki bir gün içinde alınır.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = { $size } yer kaplıyorlar, ölçüm zamanı: { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Kapladıkları yer her gün ölçülür.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Hepsini şimdi temizle

# Heading of the card about earlier versions of files.
settings-snap-earlier = Önceki sürümler

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle, herkesin dosyalarının günlük bir kopyasını saklayabilir. Böylece bir dosyanın eski bir sürümünü ya da yanlışlıkla sildiğiniz bir dosyayı Folders'tan açabilir ya da geri alabilirsiniz. Bir kopyada herkes yalnızca o gün görebildiklerini görür, yani özel ev klasörleri özel kalır. O gün başkalarına açık olan bir dosya da, kopya kaldırılana kadar o kopyada onlara açık kalır. Bu kopyalar sistemin anlık görüntülerinden ayrıdır ve biri ötekini değiştirmez. Disk alanı kullanırlar. Sildiğiniz ya da değiştirdiğiniz bir dosya, onu tutan son kopya kaldırılana kadar yer kaplamaya devam eder. Kopyaları temizlemek alanı hemen boşaltır, bu bilgisayardaki herkes için, ve geri alınamaz. Disk neredeyse doluyken kopya alınmaz. Bunlardan herhangi birini değiştirmek için bir yönetici parolası gerekir.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Dosyalarınızın önceki sürümlerini sakla

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Silinen dosyalar { $time } saklanır, sonra yerleri boşaltılır.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Her günün kopyasını şu kadar sakla

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Her günün kopyasının ne kadar saklanacağı

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Anlık görüntüler denetleniyor…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Anlık görüntüler okunamadı.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = { $n } numaralı anlık görüntüden başlattınız, bu yüzden şimdi yaptığınız hiçbir şey saklanmaz.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] { $when } tarihli { $n } anlık görüntüye geri dönebilirsiniz.
       *[other] { $n } anlık görüntüye geri dönebilirsiniz. En yenisi { $when } tarihli.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Henüz anlık görüntü yok. Her yazılım değişikliğinden önce bir tane alınır.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, silinene kadar saklanır

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Bu anlık görüntüyü geri yükle

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Bu anlık görüntüyü sil

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Şimdi yeniden başlat

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Henüz kurulmadı.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Anlık görüntüler kurulu değil. trindle paketini yeniden kurmak onları kurar: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = başka bir tür

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Anlık görüntüler için Btrfs gerekir. Bu bilgisayar { $fs } kullanıyor.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Anlık görüntüler için sistemin Btrfs üzerinde olması gerekir ve bu bilgisayar { $fs } kullanıyor. Onlara sahip olmak için Debian'ı kurarken sistem için Btrfs'yi seçin.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Bu bilgisayarda anlık görüntü yok.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Anlık görüntü yok

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Bu bilgisayarda anlık görüntü yok

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Yükleniyor…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Anlık görüntüler okunamadı

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Henüz yok

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Her anlık görüntü, bir sonraki yeniden başlatmada tüm sistemi eski haline döndürebilir. Birini geri yüklemek ya da silmek için bir yönetici parolası gerekir.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Şimdi anlık görüntü al

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Bunu yapmak için normal şekilde yeniden başlatın.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Geri yüklemeden sonra kenara ayrılmış bir eski sistem
       *[other] Geri yüklemelerden sonra kenara ayrılmış { $n } eski sistem
    }

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Kapladıkları yeri boşalt

# Heading of the card for taking a snapshot.
settings-snap-take-one = Bir tane al

# Help text of the Take one card (the information button).
settings-snap-take-about = Geri almak isteyebileceğiniz bir şeyi denemeden önce anlık görüntü alın. Aldığınız anlık görüntüler siz silene kadar saklanır. Bir anlık görüntüden başlattıysanız önce normal şekilde yeniden başlatın, yoksa yeni anlık görüntünüz saklanmaz. Her geri yükleme, sistemin hemen öncesindeki halini kenara ayırır, bu yüzden hiçbir şey kaybolmaz. Sistem dosyaları kilitliyken onları yalnızca kurulumlar ve güncellemeler değiştirebilir ve her birinden önce ve sonra bir anlık görüntü alınır. Kilidi değiştirmek için bir yönetici parolası gerekir.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = { $n } numaralı anlık görüntüden başlatıldı

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Değişiklikler saklanmaz.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Dosyalarınız da anlık görüntülerde

# Under the Your files are in the snapshots too heading.
# SAFETY: needs a human review before it ships
settings-snap-files-back = Geri yükleme dosyalarınızı da geri koyar.

# Heading of the card about what to know before restoring.
settings-snap-before = Geri yüklemeden önce

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Bir anlık görüntüden başlattığınız için şimdi yaptığınız hiçbir şey saklanmaz. Sistemin böyle kalmasını istiyorsanız listede yanındaki Bu anlık görüntüyü geri yükle düğmesine basın, sonra yeniden başlatın. Bu bilgisayarda ev klasörü sistemin bir parçası olabilir. Öyleyse bir anlık görüntüyü geri yüklemek kendi dosyalarınızı da o zamanki haline döndürür, bu yüzden önce yeni olan her şeyi güvenli bir yere kopyalayın.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Sistem dosyaları kilitli

# Under the System files locked switch.
settings-snap-locked-sub = Yalnızca kurulumlar ve güncellemeler kilidi açar.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Kilitli

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Kilidi açık
