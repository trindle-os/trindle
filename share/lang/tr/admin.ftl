
# machine translated (Turkish), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Bu geçerli bir oturum açma adı değil.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Bu oturum açma adı çok uzun.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name } adında kullanıcı yok.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-system = { $name } bir sistem hesabı olduğu için Trindle onu değiştirmedi.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Parola verilmedi.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] Parola en az { $n } karakter olmalı.
       *[other] Parola en az { $n } karakter olmalı.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Parola satır sonu içeremez.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } zaten var.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } tek yöneticidir, bu yüzden yönetici olarak kalmalıdır.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Kendi hesabınızı silemezsiniz.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = Trindle { $name } hesabını sildi, ancak oturum açma ekranındaki resmini kaldıramadı: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Bu resim çok büyük.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Bu, Trindle'ın yaptığı bir resim değil.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle, oturum açma ekranının resim klasörüne erişmesini sağlayamadı: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Bu bir dil adı değil.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle, Debian'ın dil listesini okuyamadı: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian { $code } dilini tanımıyor.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Bilgisayar adında yalnızca harf, rakam ve tire kullanılabilir. En çok 63 karakter olabilir ve tireyle başlayıp bitemez.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = Şarj sınırı şimdilik açık, ancak bilgisayar kapatılınca kaybolacak: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Tam doluluğa şarj şimdilik açık, ancak bilgisayar bir sonraki açıldığında sınır geri gelecek: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Trindle paylaşımı kapatamadığı için diğer bilgisayarlar hâlâ bağlanabilir: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Bu bir eklenti kimliği değil.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Böyle bir kişi yok.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Bu kişinin klasörü yok.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle izin verilen eklentilerin listesini kaydedemedi: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Böyle bir eklenti yok.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Hiçbir şey güncellenmedi. Trindle kendisinin hangi sürümünün kurulu olduğunu anlayamadı, bu yüzden kendi güncellemesini dışarıda bırakamadı.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Güncellemeler kuruluyor

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Toparlanıyor

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle kuruluyor

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Bu bir paket adı değil.

# Red words when the package to remove is not installed.
admin-package-not-installed = Bu paket kurulu değil.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Bu, fwupd'nin bildiği bir aygıt değil.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Tailscale'i kimin istediği bilinmiyor.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Engelleme zamanında başlamadı.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Engelleme tam olarak kapalı değil: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle { $path } öğesini yerine koyamadı: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Bu bir site adı değil.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Böyle bir sunucu yok.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool }, ihtiyacı olan her şeyi almadan durdu, bu yüzden iş tamamlanmadı: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } başarısız oldu.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } kaldırılamadı: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Hiçbir şey değişmedi, çünkü Trindle { $path } öğesini okuyamadı: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle ayrıca her şeyi eski haline getiremedi: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Bu, önceki sürümlerin saklanacağı geçerli bir süre değil.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Önceki sürümler bulunamadı.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Önceki sürümler klasörünün olması gereken yerde bir dosya var.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Kimin istediği bilinmiyor.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = Bu yalnızca pkexec ile çalışır.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Bu bir bilgisayar adı değil.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Bu bir paylaşılan klasör adı değil.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Bu bir bilgisayar adresi değil.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Bu bir paylaşılan klasörün yolu değil.

# Red words in Folders when the shared folder to close is not one of the person's own.
# SAFETY: needs a human review before it ships
admin-share-not-yours = Burada yalnızca kendi paylaşılan klasörlerinizi kapatabilirsiniz.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Orada açık bir paylaşılan klasör yok.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Trindle { $path } konumunu kurduğundan beri orada bir şey değişti, bu yüzden oraya hiçbir şey bağlanmadı.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path }, Trindle'ın bıraktığı gibi değil.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Çok fazla paylaşılan klasör açık.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } kurulu değil. Debian'ın { $package } paketiyle gelir.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Oturum açma bilgisi okunamadı.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle bu bilgisayarda nelerin bağlı olduğunu okuyamadı: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
# SAFETY: needs a human review before it ships
admin-share-busy = Bir program bu klasörde hâlâ bir şey kullanıyor. Programı kapatın, sonra yeniden deneyin.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = O bilgisayar yanıt vermiyor. Oturumu kapattığınızda paylaşılan klasör kapanır.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Bu bir kullanıcı kimliği değil.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare grubu oluşturulamadı.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba alınıyor

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Paylaşım kuruluyor

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = Paylaşım henüz kurulmadı. Önce bir klasör paylaşın.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-shown = Bu bilgisayar ağdaki başkalarına hâlâ görünüyor: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Bu bir anlık görüntü numarası değil.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Ayarlar'da alındı

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle sistemin hangi diskte olduğunu anlayamadı.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = Sistem kendi Btrfs alt biriminde olmadığı için Trindle onu bir anlık görüntüyle değiştiremez.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle imza anahtarını { $url } adresinden alamadı.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = O adres bir imza anahtarı göndermedi.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Hiçbir şey değişmedi, çünkü Trindle { $path } öğesini okuyamadı: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = İmza anahtarı alınıyor

# Second line of the progress while a source is added.
admin-step-adding-source = Ekleniyor ve listesi okunuyor

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Kaynak kullanılamadı. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle kaynağı kullanamadı, bu yüzden yeniden kaldırdı. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } henüz bunu açıp kapatamaz: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle bunu { $user } için başlatamadı: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = Trindle, { $user } kullanıcısının Cowork'ün gerek duyduğu sanal makineleri kullanmasına izin veremedi: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = Trindle, { $user } kullanıcısının bunu parolasız kullanmasına izin veremedi: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } kuruldu, ancak Trindle kurulumunu tamamlayamadı: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Kuruluyor

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Bu, hazır kaynaklardan biri değil.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name }, bu tür bilgisayar ({ $arch }) için yapılmamış.

# Last line of the progress while a program is installed.
admin-step-setting-up = Kuruluyor

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Her adres, https:// ile başlayan düz bir web adresi olmalıdır.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = Takım ve bileşenler düz sözcüklerden oluşmalıdır.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Bu bir paket adı değil.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Kaldırılıyor

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Listeler yeniden okunuyor

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Burada yalnızca Trindle'ın eklediği kaynaklar kaldırılabilir.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = Kaynak hâlâ duruyor: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = { $n } numaralı anlık görüntü diskte yok.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Hiçbir şey değişmedi, çünkü Trindle anlık görüntüyü hazırlayamadı. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle şu anki sistemi kenara alamadı: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = Şu anki sistemin adı artık { $sub } değil, { $aside } ve bilgisayar, adı yeniden { $sub } olana kadar açılmayacak: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle anlık görüntüleri taşıyamadı: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle anlık görüntüyü yerine koyamadı: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle kenara alınan sistemlerin listesini güncelleyemedi: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = { $names } kaldırılamadı

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Trindle bunları kaldırdı, çünkü artık gerek kalmamıştı: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Tüm güncellemeler yüklenmedi. Bunlar geri tutuldu: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } kapalı, çünkü sürücüsü ({ $driver }, DKMS) yeni Linux çekirdeği için henüz derlenemiyor.

admin-driver-off = Sürücü ({ $driver }, DKMS) yeni Linux çekirdeği için henüz derlenemiyor. Ona ihtiyaç duyan özellikler kapalı.

# Said once after the drivers that are off.
admin-driver-off-rest = Çekirdek ve güncellemenin geri kalanı yüklendi.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = sanal kamera

admin-language-in-use = { $code }, bu bilgisayarın açıldığı dildir, bu yüzden kaldırılamaz.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Bu bilgisayarda kapatılacak bölünmüş kilit yavaşlaması yok.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle bu sürücüyü bulamadı ya da bu sistem sürücüsü.

# The drive could not be closed where it was open.
admin-games-drive-busy = Sürücü kullanımda. Üzerindeki tüm dosyaları ve programları kapatıp yeniden deneyin: { $why }

# Another software change has the read-only system open.
admin-system-busy = Başka bir yazılım değişikliği sürüyor. Bittiğinde yeniden deneyin.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Hiçbir şey değişmedi, çünkü Trindle sistemin kilidini açamadı: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle sistemi yeniden kilitleyemedi. Bir sonraki açılışta kilitlenecek: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle /usr/local dizinini yazılabilir tutamadı: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Bunu yalnızca root çalıştırabilir, pkexec ile de çalıştırılamaz.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Bunun için Btrfs gerekir.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = Anlık görüntü yerine konuldu ve bilgisayar ondan açılacak. Trindle eski sistemi temizlemeyi tamamlayamadı. Bilgisayar bir sonraki açıldığında yeniden deneyecek: { $why }


# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = En fazla { $most } port açabilirsiniz.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle açık portların listesini kaydedemedi: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = Güvenlik duvarı hâlâ açık: { $why }

# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = Bu eklenti artık izin verdiklerinizden farklı bağlantı noktaları istiyor. Bunları açabilmesi için ona yeniden izin verin.

# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = Bir program yalnızca onu isteyen kişi için eklenebilir.

# Red words when a snapshot is put back and the name for the system set aside is already used.
admin-restore-name-taken = Az önce başka bir geri yükleme aynı adı kullandı. Birkaç saniye bekleyip yeniden deneyin.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } bağlantısı kesilemedi: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 paylaşılan klasör yanıt vermedi, bu yüzden hâlâ bağlı. Bilgisayar kapanırken bağlantısı kesilecek.
       *[other] { $n } paylaşılan klasör yanıt vermedi, bu yüzden hâlâ bağlı. Bilgisayar kapanırken bağlantıları kesilecek.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = { $path } takas dosyası kapatılamadı: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = { $path } takas dosyası yeniden açılamadı: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path }, sistemin takas dosyalarından biri değil.

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = { $path } takas dosyası kullanılmayacak, çünkü başkaları { $why } öğesini değiştirebilir veya okuyabilir.

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = Takas dosyasının yeri bulunamadı, bu yüzden hazırda bekletmeden uyanma çalışmayabilir.

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = Hazırda bekletme, yeniden başlattıktan sonra yine çalışacak.
