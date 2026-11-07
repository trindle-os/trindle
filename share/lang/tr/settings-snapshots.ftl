
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Anlık görüntüler

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = anlık görüntü anlık görüntüler snapshot geri yükle geri al yedek güncelleme bozuldu btrfs snapper önyükleme menüsü grub önceki sürümler eski sürüm silinen dosya geri koy kurtar ev kopyalar

# Help line at the foot of the Snapshots page.
# SAFETY: needs a human review before it ships
settings-snap-help = Sistemin her yazılım değişikliğinden önceki ve sonraki hali ve siz bir tane aldığınızda. Birini geri yüklemek, bir sonraki başlatmadan itibaren tüm sistemi ona döndürür; ev klasörünün diskte kendine ait bir bölümü varsa (Trindle'ın kurucusu böyle yapar) kendi dosyalarınıza dokunulmaz. Bilgisayar başlamazsa önyükleme menüsünden Trindle anlık görüntüleri'ni seçin. Önceki sürümler, sistemin anlık görüntülerinden ayrı olarak herkesin kendi dosyalarının bir kopyasını her gün saklar.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Kopya alınmadı: disk neredeyse dolu.

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
settings-snap-freeing = Boşaltılıyor...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Boşaltıldı

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Geri yükleniyor

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Bir sonraki başlatmadan itibaren sistem { $n } numaralı anlık görüntü olacak. O zamandan beri değişen her şey bir kenara ayrılır, kaybolmaz.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Siliniyor

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Silindi

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Parola bekleniyor...

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
settings-snap-measured-daily = Kapladıkları yer günde bir kez ölçülür.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Hepsini şimdi temizle

# Heading of the card about earlier versions of files.
settings-snap-earlier = Önceki sürümler

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle, herkesin dosyalarının her günkü halinin bir kopyasını saklayabilir; böylece bir dosyanın eski bir sürümü ya da yanlışlıkla silinmiş bir dosya Klasörler'den açılabilir ya da geri konabilir. Herkes bir kopyada yalnızca o gün okuyabildiğini okuyabilir, yani özel bir ev klasörü özel kalır; o gün başkalarına açık olan bir dosya da kopyada, kopya silinene kadar onlara açık kalır. Kopyalar sistemin anlık görüntülerinden ayrıdır: sistemi geri yüklemek dosyalarınızı asla değiştirmez, bunlar da sistemi değiştirmez. Bedeli yerdir: sildiğiniz ya da değiştirdiğiniz bir dosya, onu tutan son kopya gidene kadar yer kaplamaya devam eder, bu yüzden bir şeyleri silmek diski hemen boşaltmaz. Kopyaları temizlemek yeri şimdi boşaltır, bu bilgisayardaki herkes için, ve geri alınamaz. Disk neredeyse doluyken kopya alınmaz. Bunlardan herhangi birini değiştirmek bir yönetici parolası gerektirir.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Dosyalarınızın önceki sürümlerini sakla

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Silinen dosyalar, yerleri boşaltılmadan önce { $time } kalır.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Her günün kopyasını şu kadar sakla

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Her günün kopyasının ne kadar saklanacağı

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Anlık görüntülere bakılıyor…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Anlık görüntüler okunamadı.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = { $n } numaralı anlık görüntüden başlatıldı, bu yüzden şimdi yapılan hiçbir şey saklanmaz.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] Geri dönülecek { $n } anlık görüntü var, en yenisi { $when } tarihli.
       *[other] Geri dönülecek { $n } anlık görüntü var, en yenisi { $when } tarihli.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Henüz anlık görüntü yok. Her yazılım değişikliğinden önce bir tane alınır.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, silinene kadar saklanır

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Sistemi buna geri yükle

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Bu anlık görüntüyü sil

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Şimdi yeniden başlat

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Henüz kurulmadı.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = trindle paketini yeniden kurun.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Anlık görüntüler kurulu değil. trindle paketini yeniden kurmak onları kurar: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = başka bir tür

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Btrfs gerekir; bu bilgisayar { $fs } kullanıyor.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Debian'ı kurarken Btrfs'yi seçin.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Anlık görüntüler için sistemin Btrfs üzerinde olması gerekir ve bu bilgisayar { $fs } kullanıyor. Onlara sahip olmak için Debian'ı kurarken sistem için Btrfs'yi seçin.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Bu bilgisayarda anlık görüntü yok.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Anlık görüntü yok

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Bu bilgisayarda anlık görüntü yok

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Onlara sahip olmak için

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Aranıyor…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Anlık görüntüler okunamadı

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Henüz yok

# Under the None yet heading.
settings-snap-none-line-sub = Her yazılım değişikliğinden önce bir tane alınır.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Her anlık görüntü, bir sonraki başlatmadan itibaren tüm sistemi o zamanki haline döndürebilir ya da silinebilir. İkisi de iki kez basılmasını ister ve bir yönetici parolası gerektirir. Geri yüklemeden sonra yanında yeniden başlatma düğmesi görünür.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Şimdi anlık görüntü al

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Bir anlık görüntüden başlatılmışken olmaz.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Siz silene kadar saklanır.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Geri yükleme ile kenara ayrılmış bir sistem
       *[other] Geri yükleme ile kenara ayrılmış { $n } sistem
    }

# Under the set aside heading.
settings-snap-free-space = Her şey yolundayken yeri boşaltın.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Kapladıkları yeri boşalt

# Heading of the card for taking a snapshot.
settings-snap-take-one = Bir tane al

# Help text of the Take one card (the information button).
settings-snap-take-about = Geri almak isteyebileceğiniz bir şeyi denemeden önce anlık görüntü alın; aldığınız, siz silene kadar saklanır. Bilgisayar bir anlık görüntüden başlatılmışken alınan hiçbir şey saklanmaz, bu yüzden önce normal şekilde yeniden başlatın. Her geri yükleme, sistemin hemen öncesindeki halini kenara ayırır, bu yüzden hiçbir şey kaybolmaz; her şey yolundayken kapladıkları yeri boşaltın.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = { $n } numaralı anlık görüntüden başlatıldı

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Şimdi yapılan hiçbir şey saklanmaz.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Dosyalarınız da anlık görüntülerde

# Under the Your files are in the snapshots too heading.
# SAFETY: needs a human review before it ships
settings-snap-files-back = Geri yükleme dosyalarınızı da geri koyar.

# Heading of the card about what to know before restoring.
settings-snap-before = Geri yüklemeden önce

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Bir anlık görüntüden başlatıldığında şimdi yapılan hiçbir şey saklanmaz: sistemin böyle kalmasını istiyorsanız listede onun için geri yükle'ye basın, sonra yeniden başlatın. Bu bilgisayarda ev klasörü sistemin bir parçası olabilir; o durumda bir anlık görüntüyü geri yüklemek kendi dosyalarınızı da o zamanki haline döndürür, bu yüzden önce yeni olan her şeyi güvenli bir yere kopyalayın.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Sistem dosyaları kilitli

# Under the System files locked switch.
settings-snap-locked-sub = Yalnızca kurulumlar ve güncellemeler kilidi açar.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Kilitli

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Kilidi açık

