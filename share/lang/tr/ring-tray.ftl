
# machine translated (Turkish), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 eklenti güncellemesi için Uygulamalar'a bakmak gerekiyor
       *[other] { $n } eklenti güncellemesi için Uygulamalar'a bakmak gerekiyor
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Her şey güncel

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } bekliyor: Uygulamalar'ın yanındaki güncelleme düğmesi onu kurar
       *[other] { $n } bekliyor: Uygulamalar'ın yanındaki güncelleme düğmesi hepsini kurar
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } bekliyor: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } bekliyor: Uygulamalar'ın yanındaki güncelleme düğmesi { $put } tanesini kurar, ve { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Donanım yazılımı güncellemesine bak
       *[other] { $n } donanım yazılımı güncellemesine bak
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] { $n } güncellemeye bak, aralarında donanım yazılımı da var
       *[other] { $n } güncellemeye bak, aralarında donanım yazılımı da var
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] { $n } güncellemeyi kur
       *[other] { $n } güncellemeyi kur
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Önceden daha fazla izin isteyen eklenti güncellemesine bak
       *[other] Önceden daha fazla izin isteyen { $n } eklenti güncellemesine bak
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] { $put } güncellemeyi kur; { $look }
       *[other] { $put } güncellemeyi kur; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Güncel

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Klavye

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Ekran parlaklığı

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Uyanık tut

# Name of the sound button when the sound is muted.
ring-tray-muted = Sessiz

# Name of the sound button with the volume level.
ring-tray-volume = Ses düzeyi %{ $pct }

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Pil %{ $pct }, şarj oluyor
        [onbattery] Pil %{ $pct }, pille çalışıyor
        [full] Pil %{ $pct }, dolu
        [pluggedin] Pil %{ $pct }, prize takılı
       *[other] Pil %{ $pct }
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Ağ kapalı

# Name of the network button when nothing is connected.
ring-tray-not-connected = Bağlı değil

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = VPN üzerinden bağlı

# Name of the network button when connected by cable.
ring-tray-wired = Kablolu

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Kablolu, sınırlı bağlantı

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, sinyal %{ $strength }

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, sinyal %{ $strength }, sınırlı bağlantı

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth kapalı

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = { $names } ile bağlı

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth açık

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Tepsi simgesi

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Tüm tepsi simgeleri

# Words on the panel of every tray icon when there are none.
ring-tray-none = Tepsi simgesi yok

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Halkada göster

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Halkadan çıkar

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Halkada zaten dört tane var. Önce birini çıkar.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Denetim kapalı

