
# machine translated (Turkish), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Kaydedildi

# Said beside a button that worked, when nothing more is known.
settings-common-done = Tamam

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Yapılamadı

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Bu sayfa hakkında

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = { $name } hakkında

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } zaten var

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Kalanlar için de aynısını yap ({ $n } tane daha)
       *[other] Kalanlar için de aynısını yap ({ $n } tane daha)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Kalanlar için de aynısını yap

# Tip of the button that keeps the one already there.
# SAFETY: needs a human review before it ships
settings-common-skip = Atla: zaten orada olanı tut

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = Değiştir

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = Zaten orada olanı getirilenle değiştir

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Getirilen

# Heading of the line that shows the one already there.
settings-common-already-line = Zaten orada olan

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = Zaten orada olan değiştirilsin mi?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] { $left } saniye içinde geri döner.
       *[other] { $left } saniye içinde geri döner.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] { $left } saniye içinde geri döner. { $more }
       *[other] { $left } saniye içinde geri döner. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Şimdi geri dön

# Button at the foot of the window that keeps the change on trial.
# SAFETY: needs a human review before it ships
settings-common-keep = Tut

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } kullanılamıyor

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Şununla kurun: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Web sayfası açılamadı: hiçbir web tarayıcısı başlatılamadı.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } açılamadı.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Kaydedilemedi.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Yapılamadı: sistemin ileti yoluna ulaşılamadı. Bilgisayarı yeniden başlatıp yeniden deneyin.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Yapılamadı: bu oturumun ileti yoluna ulaşılamadı. Oturumu kapatıp yeniden açın, sonra yeniden deneyin.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Seçim penceresi açılamadı. Oturumu kapatıp yeniden açın, sonra yeniden deneyin.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Disk dolu.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Diske şu anda yazılamıyor.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = Sistemin bunu yapan bölümü çalışmıyor.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Yanıt vermesi çok uzun sürdü. Yeniden deneyin.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = Parola verilmedi.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = Bu hesabın buna izni yok.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Gereken bir şey bu bilgisayarda eksik.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } ve { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = Halka, ekranın kenarına tıklayınca açılır.

# Word drawn in the middle of that picture.
settings-common-ring-name = Halka

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Dizüstü bilgisayarın bellenimine ulaşılamadı: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Dizüstü bilgisayarın yazılımı şarj sınırını kabul etmedi.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Bu bilgisayar şarjının erken durdurulmasına izin vermiyor.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path } ayarlanamadı: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Trindle'ı sanal makinede denemek

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle, { $machine } içinde çalışıyor. Orada iyi çalışır, ancak sanal makine gerçek bir bilgisayar kadar hızlı veya akıcı olmaz; bu yüzden hızını gerçek donanımda değerlendirin.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle, { $machine } içinde 3B grafik olmadan çalışıyor; bu yüzden normalde grafik yongasının çizeceği her şeyi işlemci çiziyor. Pencereler, videolar ve web sayfaları gerçek bir bilgisayardakinden yavaş hissettirir ve Trindle yardımcı olmak için animasyonlarını kapatır. Üzerinde çalıştığı bilgisayar destekliyorsa, sanal makinenin ekran ayarlarında 3B hızlandırmayı açmak büyük fark yaratır. Gerçek bir bilgisayara kurulunca bunların hiçbiri geçerli olmaz.

