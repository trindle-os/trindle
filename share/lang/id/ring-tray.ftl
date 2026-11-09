
# machine translated (Indonesian), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
       *[other] { $n } pembaruan plugin perlu dilihat di Aplikasi
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Semuanya sudah terbaru

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
       *[other] { $n } menunggu: tombol pembaruan di samping Aplikasi akan memasangnya
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } menunggu: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } menunggu: tombol pembaruan di samping Aplikasi akan memasang { $put }, dan { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
       *[other] Lihat { $n } pembaruan firmware
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
       *[other] Lihat { $n } pembaruan, termasuk firmware
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
       *[other] Pasang { $n } pembaruan
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
       *[other] Lihat { $n } pembaruan plugin, yang meminta lebih dari sebelumnya
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
       *[other] Pasang { $put } pembaruan; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Sudah terbaru

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Papan ketik

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Kecerahan layar

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Tetap menyala

# Name of the sound button when the sound is muted.
ring-tray-muted = Senyap

# Name of the sound button with the volume level.
ring-tray-volume = Volume { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Baterai { $pct }%, mengisi daya
        [onbattery] Baterai { $pct }%, memakai baterai
        [full] Baterai { $pct }%, penuh
        [pluggedin] Baterai { $pct }%, tersambung ke listrik
       *[other] Baterai { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Jaringan mati

# Name of the network button when nothing is connected.
ring-tray-not-connected = Tidak tersambung

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Tersambung melalui VPN

# Name of the network button when connected by cable.
ring-tray-wired = Kabel

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Kabel, konektivitas terbatas

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, sinyal { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, sinyal { $strength }%, konektivitas terbatas

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth mati

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Tersambung ke { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth menyala

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Ikon baki

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Semua ikon baki

# Words on the panel of every tray icon when there are none.
ring-tray-none = Tidak ada ikon baki

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Tampilkan di cincin

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Keluarkan dari cincin

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Sudah ada empat di cincin. Keluarkan satu dulu.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Pemeriksaan mati

