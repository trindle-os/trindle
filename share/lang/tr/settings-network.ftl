
# machine translated (Turkish), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
# SAFETY: needs a human review before it ships
settings-network-rfkill-hard = { $radio }, bilgisayardaki bir anahtar ya da tuşla kapatılmış. Onu açın, sonra yeniden deneyin.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
# SAFETY: needs a human review before it ships
settings-network-rfkill-refused = { $radio } sistem tarafından engelleniyor ve engel kaldırılamadı.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Bu parola kabul edilmedi. Yeniden deneyin.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Bu ağ için kayıtlı parola kabul edilmedi. Ağı çöp kutusu düğmesiyle unutun, sonra yeniden katılın.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Bu ağ bulunamadı. Adını ve menzilde olup olmadığını denetleyin.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Ağ bu bilgisayarı içeri aldı ama adres vermedi. Yeniden deneyin ya da yönlendiriciyi yeniden başlatın.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Ağ bu bilgisayarın katılmasına izin vermedi. Ağa yaklaşın, sonra yeniden deneyin.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Bu parola bir Wi-Fi ağı için uygun değil. Denetleyin, sonra yeniden deneyin.

# Said in red when joining a network failed for no known reason.
# SAFETY: needs a human review before it ships
settings-network-join-refused = Ağa katılınamadı.

# Small line of a shared folder: who may open it with a password.
settings-network-access-open = parolanızla açılır

# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = parolanızla açılır ve değiştirilir

# Small line of a shared folder: anyone at home may open it.
settings-network-access-guests-open = evdeki herkes açabilir

# Small line of a shared folder: anyone at home may change it.
settings-network-access-guests-change = evdeki herkes değiştirebilir

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
settings-network-state-configuring = Yapılandırılıyor

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
        [strong] { $name } ağına bağlı, güçlü sinyal.
        [good] { $name } ağına bağlı, iyi sinyal.
        [fair] { $name } ağına bağlı, orta sinyal.
       *[other] { $name } ağına bağlı, zayıf sinyal.
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
settings-network-top-gone = Ağlara bakılamıyor: NetworkManager çalışmıyor.

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
settings-network-chip = Yongası daha yeni bir Linux gerektiriyor.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Bu bilgisayarda çalışan bir Wi-Fi yok.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Kapalı. Bir ağa katılmak için açın.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Açık, ancak bir ağa katılmadı.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi kullan

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi ve kablo

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi, bu bilgisayarı kablosuz olarak bir ağa katar. Kapatıldığında ağ aramayı bırakır ve biraz pil tasarrufu sağlar. Adını yayınlamayan bir ağ, menzildeki ağlar listesinde hiç görünmez: ona buradan, adını büyük harfler dahil tam olarak yazarak katılın. Bu bilgisayarın Wi-Fi yongası, Linux'unun hiçbir sürücüsünün henüz desteklemediği bir yongaysa, Trindle'ın çalıştığı Linux'tan daha yeni bir Linux gerekir. Ağ kablosunun, bağlanmak ya da bağlantıyı kesmek için düğmesi olan kendi satırı vardır.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Diğer bilgisayarların bağlanmasına izin ver

# Small line under that switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch-sub = Evde klasörleri ve yazıcıları paylaşmak için.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Paylaşılan klasörler aranıyor…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Paylaşılan klasörler okunamadı

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Henüz erişilemiyor

# Small line under it.
settings-network-not-reachable-sub = Anahtar kapalıyken başkaları bunları açamaz.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Paylaşılan klasör yok

# Small line under it.
settings-network-no-shared-sub = Paylaşmak için Klasörler'de birine sağ tıklayın.

# Heading of the Sharing card.
settings-network-sharing = Paylaşım

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Diğer bilgisayarların bağlanmasına izin ver kapalıyken, ağdaki hiçbir şey, bu bilgisayar önce istemedikçe ona ulaşamaz; böylece aynı Wi-Fi'deki kimse şansını deneyemez. Evdeki diğer bilgisayarlarla bir klasörü ya da yazıcıyı paylaşmak için bunu açın. Bir klasörü paylaşmak için Klasörler'de sağ tıklayıp Ağda paylaş'ı seçin. Her biri, kimin ne yapabileceğini söyler. Parolanızla: başkaları, paylaşım parolanızla dosyaları açabilir ya da açıp değiştirebilir. Evdeki herkes: ağdaki herkes dosyaları parola olmadan açabilir ya da açıp değiştirebilir. Anahtar kapalıyken evdeki diğer bilgisayarların hiçbiri bunları açamaz.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Durduruluyor…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Parola bekleniyor...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Paylaşıma izin verildi

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Dışarıda tutuldu

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
settings-network-connecting = Bağlanıyor...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Menzilde ağ yok

# Small line under it.
settings-network-none-in-range-sub = Bu sayfa her açıldığında yeniden aranır.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Menzildeki ağlar

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Bu bilgisayarın duyabildiği her Wi-Fi ağı. Sayfa açılırken, önce kullanımdaki, sonra burada kayıtlı olanlar, sonra kalanlar sinyal gücüne göre sıralanır; sayfa açıkken bir satır yerinde kalır. Birine katılmak için oklara basın; korumalı bir ağ ilk seferinde parolasını ister. Çöp kutusu, kayıtlı bir ağı ve parolasını unutur. Açık bir ağ şifreli değildir, bu yüzden yakındaki başkaları üzerinden gönderilenleri görebilir.

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
settings-network-joining = Katılınıyor...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi parolaları en az 8 karakter olmalıdır.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Gizli bir ağa katıl

# Small line under it.
settings-network-hidden-sub = Adını büyük harfler dahil tam olarak yazın.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Ağ adı

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Parola, varsa

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Ağın adı gerekli.

# Shown under a network after it was joined.
settings-network-joined = Katılındı

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
# SAFETY: needs a human review before it ships
settings-network-kept-anyway = { $words } Yine de kaydedildi: yeniden katılmadan önce çöp kutusu düğmesiyle unutun.

# Said in red when a network would not disconnect.
# SAFETY: needs a human review before it ships
settings-network-disconnect-failed = Bağlantı kesilmedi.

