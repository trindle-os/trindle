# machine translated (Turkish), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Bu geçerli bir program adı değil.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Bu kaydedilmedi.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Var olan bir klasör seçin.

# Red words when the whole home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-whole-home = Ev klasörünün tamamı gizli kalır.

# Red words when a folder that holds the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-holds-home = Bu klasör ev klasörünüzü içerir, bu yüzden gizli kalır.

# Red words when a folder outside the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-outside-home = Yalnızca ev klasörünüzün içindeki klasörler paylaşılabilir.

# Red words when a hidden folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-hidden = Gizli klasörler her zaman gizli kalır.

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = Bu klasör parolaları veya e-postaları barındırır, bu yüzden gizli kalır.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Bu programın kullanılacak bir ayar klasörü yok.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Bir sitenin adını yazın, örneğin api.example.com, ya da bu adın altındaki her site için *.example.com.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Bir bilgisayarın adını yazın, örneğin nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Her birini AD=değer biçiminde yazın.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Her birini AD=değer biçiminde, büyük harflerle yazın.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = Bir yapay zekâ programı bu klasörü kullanmak istiyor. Şu anda onu göremiyor.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = Bir yapay zekâ programı bunu yönetici olarak çalıştırmak istiyor. Çitin dışında çalışır ve sizin görebildiğiniz her şeyi görebilir.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Yapay zekâ programı

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
# SAFETY: needs a human review before it ships
store-fence-read-only = Bu klasörü yalnızca okumasına izin ver

# Note under the question about a folder.
store-fence-sees-next = Klasörü programın bir sonraki başlayışında görecek.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = Hayır

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = Yalnızca bu oturum için paylaş

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = Her zaman paylaş

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = İzin verme

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = İzin ver

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Bu komut tamamı gösterilemeyecek kadar uzun, bu yüzden çalıştırılmadı.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Yanıtlamanızı bekleyen başka bir istek var.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Buna izin vermediniz.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } çitin içinde kullanılamaz.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Çitin dışında başlatılamadı.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Klasörün tam yolunu verin.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Onu paylaşmadınız.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Paylaşıldı. Bu program bir sonraki başladığında orada olacak.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Ulaşılacak bilgisayarın adını yazın.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Çitteki yapay zekâ programları o bilgisayara ssh ile ulaşamaz.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = Yapay zekâ programları için anahtar oluşturulamadı: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Çalıştırılacak bir komutun adını yazın.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } burada kullanılamaz.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } çitin içinde kullanılamaz. Dışarıda çalıştırılmasını isteyin.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: çitin istek hattı yok.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: istek gönderilmedi.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock reddetti: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = Çit başlamadı: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = Çit başlamadı: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Yapay zekâ programları burada kendi oturum bilgilerini sizinkinden ayrı tutar. İstediğinde programın içinde oturum açın.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Klasörleriniz yapay zekâ programlarından gizlidir. Birinde çalışmaları için Ayarlar, Yapay zekâ programları bölümünden paylaşın.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = Çit başlamadı: sistem çağrısı süzgeci oluşturulamadı.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = Çit başlamadı: ağ sınırı kurulamadı.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = Çit başlamadı: bubblewrap çalışmadı: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: çitin dışında bulunamadı
