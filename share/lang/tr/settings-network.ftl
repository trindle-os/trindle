
# machine translated (Turkish), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
# SAFETY: needs a human review before it ships
settings-network-rfkill-hard = { $radio }, bilgisayardaki bir anahtar ya da tuşla kapatılmış. Onu açın, sonra yeniden deneyin.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
# SAFETY: needs a human review before it ships
settings-network-rfkill-refused = { $radio } sistem tarafından engelleniyor ve Trindle engeli kaldıramadı.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Bu parola kabul edilmedi. Yeniden deneyin.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Bu ağ için kayıtlı parola kabul edilmedi. Ağı unutmak için çöp kutusu düğmesini kullanın, sonra yeniden katılın.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle bu ağı bulamadı. Adını ve menzilde olup olmadığını denetleyin.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Bu bilgisayar ağa katıldı ama adres alamadı. Yeniden deneyin ya da yönlendiriciyi yeniden başlatın.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Bu bilgisayar ağa katılamadı. Yönlendiriciye yaklaşın, sonra yeniden deneyin.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Bu bir Wi-Fi parolası olamaz. Denetleyin, sonra yeniden deneyin.

# Said in red when joining a network failed for no known reason.
# SAFETY: needs a human review before it ships
settings-network-join-refused = Ağa katılınamadı.

# Small line of a shared folder: who may open it with a password.
settings-network-access-open = parolanızla açılır

# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = parolanızla açılır ve değiştirilir

# Small line of a shared folder: anyone on the local network may open it.
settings-network-access-guests-open = yakındaki herkes açabilir

# Small line of a shared folder: anyone on the local network may change it.
settings-network-access-guests-change = yakındaki herkes değiştirebilir

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = { $name } paylaşımını durdur

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Yönetilmiyor

# State of a network device that cannot be used.
settings-network-state-unavailable = Kullanılamıyor

# State of a network device that is not connected.
settings-network-state-disconnected = Bağlantı kesildi

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Hazırlanıyor

# State of a network device that is being set up.
settings-network-state-configuring = Ayarlanıyor

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = Parola gerekiyor

# State of a network device that is getting an address.
settings-network-state-address = Adres alınıyor

# State of a network device whose connection is being checked.
settings-network-state-checking = Denetleniyor

# State of a network device that is waiting.
settings-network-state-waiting = Bekliyor

# State of a network device that is connected.
settings-network-state-connected = Bağlı

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Bağlantı kesiliyor

# State of a network device that failed.
settings-network-state-failed = Başarısız

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Kabloyla ve Wi-Fi üzerinden { $name } ağına bağlı.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] { $name } ağına bağlı. Sinyal güçlü.
        [good] { $name } ağına bağlı. Sinyal iyi.
        [fair] { $name } ağına bağlı. Sinyal orta.
       *[other] { $name } ağına bağlı. Sinyal zayıf.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Kabloyla bağlı.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi kapalı, bu yüzden bu bilgisayar bağlı değil.

# Headline of the page when not connected.
settings-network-top-none = Bağlı değil. Aşağıdan bir ağ seçin.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Ağlar aranıyor…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = NetworkManager çalışmadığı için Trindle ağları gösteremiyor.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Sistem veri yolu

# Name of the page in the list of settings pages and its heading.
settings-network-title = Ağ

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi kablosuz internet ethernet kablolu parola bağlan güvenlik duvarı paylaşım paylaş yazıcı klasör

# Help text at the top of the page.
settings-network-help = Wi-Fi'yi açın ya da kapatın, bir ağa katılın, artık istemediğiniz bir ağı unutun ya da diğer bilgisayarların bağlanmasına izin verin.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] { $name } ağından güçlü sinyal.
        [good] { $name } ağından iyi sinyal.
        [fair] { $name } ağından orta sinyal.
       *[other] { $name } ağından zayıf sinyal.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi için daha yeni bir Linux gerekir.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Bu bilgisayarda çalışan bir Wi-Fi yok.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Kapalı. Bir ağa katılmak için açın.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Açık ama bir ağa bağlı değil.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi kullan

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi ve kablo

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi, bu bilgisayarı kablo olmadan bir ağa bağlar. Kapatırsanız ağ aramayı bırakır ve biraz pil tasarrufu sağlar. Adını gizleyen bir ağ, menzildeki ağlar listesinde hiç görünmez. Böyle bir ağa katılmak için adını büyük harfler dahil tam olarak yazın. Bazı Wi-Fi yongaları, Trindle'ın kullandığı Linux sürümünde henüz bulunmayan bir sürücü ister. Bunlar için daha yeni bir Linux sürümü gerekir. Ağ kablosunun kendi satırı vardır; orada bağlanmak ya da bağlantıyı kesmek için bir düğme bulunur.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Diğer bilgisayarların bağlanmasına izin ver

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Paylaşılan klasörler aranıyor…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Paylaşılan klasörler okunamadı

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Henüz erişilemiyor

# Small line under it.
settings-network-not-reachable-sub = Paylaşım kapalıyken diğer bilgisayarlar bunları açamaz.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Paylaşılan klasör yok

# Heading of the Sharing card.
settings-network-sharing = Paylaşım

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Diğer bilgisayarların bağlanmasına izin ver kapalıyken, ağdaki diğer bilgisayarlar bu bilgisayara ancak bu bilgisayar onlarla önce bağlantı kurduysa ulaşabilir. Böylece aynı Wi-Fi'deki kimse içeri girmeye çalışamaz. Evdeki diğer bilgisayarlarla bir klasör ya da yazıcı paylaşmak için bunu açın. Bir klasörü paylaşmak için Folders'ta ona sağ tıklayıp Ağda paylaş'ı seçin. Paylaşılan her klasör, kimin ne yapabileceğini gösterir. Parolanızla: paylaşım parolanızı bilen başkaları dosyaları açabilir ya da açıp değiştirebilir. Evdeki herkes: ağdaki herkes dosyaları parola olmadan açabilir ya da açıp değiştirebilir. Anahtar kapalıyken evdeki diğer bilgisayarlar bunların hiçbirini açamaz.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Durduruluyor…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Parolanız bekleniyor…

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Paylaşım açık

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Engellendi

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi açılamadı.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi kapatılamadı.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
# SAFETY: needs a human review before it ships
settings-network-disconnect = Bağlantıyı kes

# Tip of the button that connects a network or a cable.
# SAFETY: needs a human review before it ships
settings-network-connect = Bağlan

# Shown under a network or cable while it is being connected.
settings-network-connecting = Bağlanıyor…

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Menzilde ağ yok

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Menzildeki ağlar

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Bu bilgisayarın algılayabildiği her Wi-Fi ağı. Sayfayı açtığınızda önce kullandığınız ağ, sonra kayıtlı ağlar, sonra kalanlar sinyal gücüne göre sıralanır. Sayfa açıkken sıra değişmez. Bir ağa katılmak için oka basın. Korumalı bir ağ ilk seferinde parolasını ister. Çöp kutusu düğmesi kayıtlı bir ağı ve parolasını unutturur. Açık bir ağ şifreli değildir, bu yüzden yakındaki kişiler üzerinden gönderdiklerinizi görebilir.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Menzildeki ağları görmek için Wi-Fi'yi açın.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Kayıtlı

# Part of a network's small line: it needs a password.
# SAFETY: needs a human review before it ships
settings-network-secured = Korumalı

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = Açık, şifresiz

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/sn

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Bu ağı unut

# Said in red when a saved network could not be forgotten.
# SAFETY: needs a human review before it ships
settings-network-forget-failed = Ağ unutulamadı.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Parola

# Tip of the button that joins the network.
# SAFETY: needs a human review before it ships
settings-network-join = Katıl

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = İptal

# Shown under a network while it is being joined.
settings-network-joining = Katılınıyor…

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi parolaları en az 8 karakter olmalıdır.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Gizli bir ağa katıl

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Ağ adı

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Parola, varsa

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Ağın adını yazın.

# Shown under a network after it was joined.
settings-network-joined = Katılındı

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
# SAFETY: needs a human review before it ships
settings-network-kept-anyway = { $words } Ağ yine de kaydedildi. Yeniden katılmadan önce çöp kutusu düğmesiyle unutun.

# Said in red when a network would not disconnect.
# SAFETY: needs a human review before it ships
settings-network-disconnect-failed = Bağlantı kesilemedi.


# Heading of the card that lists the ports opened by hand.
settings-network-ports = Açık portlar

# Help text (i button) of the Open ports card.
settings-network-ports-about = Bu bilgisayar karşı tarafla önce bağlantı kurmadıysa her port kapalıdır. Başkalarının bağlanmasını bekleyen bir programa, örneğin bir oyun sunucusuna ya da eşitleme aracına, siz portunu burada açana kadar ulaşılamaz. Açık bir port, bu bilgisayarla aynı yerel ağdaki her bilgisayarın girmesine izin verir ama geniş internetteki kimsenin girmesine izin vermez. Kafe, otel ya da ofis ağında bu, oradaki herkes demektir. Bir portu açmak ya da kapatmak için yöneticinin parolası gerekir.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Açık port yok

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = { $port } portunu kapat

# Heading of the line that opens a port.
settings-network-port-add = Bir port aç

# Placeholder in the box where the port number is typed.
settings-network-port-number = Port numarası

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Protokol

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = İkisi de

# Button that opens the typed port.
settings-network-port-open = Aç

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = 1 ile 65535 arasında bir sayı yazın.

# Red words when the port is already open.
settings-network-port-already = Zaten açık.

# Said under the line once the port is open.
settings-network-port-opened = Açıldı

# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = Kullanıcı adı gerekir

# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = Bu ağ parolanın yanı sıra bir kullanıcı adı da istiyor. Trindle henüz bu tür ağlara katılamıyor.
