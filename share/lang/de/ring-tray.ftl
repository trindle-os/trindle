
# machine translated (German), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 Plugin-Update muss in Apps angesehen werden
       *[other] { $n } Plugin-Updates müssen in Apps angesehen werden
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Alles ist auf dem neuesten Stand

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } wartet: Die Update-Taste neben Apps spielt es ein
       *[other] { $n } warten: Die Update-Taste neben Apps spielt sie ein
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } warten: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } warten: Die Update-Taste neben Apps spielt { $put } ein, und { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Firmware-Update ansehen
       *[other] { $n } Firmware-Updates ansehen
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] { $n } Update ansehen, darunter Firmware
       *[other] { $n } Updates ansehen, darunter Firmware
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] { $n } Update installieren
       *[other] { $n } Updates installieren
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Plugin-Update ansehen, das mehr verlangt als zuvor
       *[other] { $n } Plugin-Updates ansehen, die mehr verlangen als zuvor
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] { $put } Update installieren; { $look }
       *[other] { $put } Updates installieren; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Aktuell

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Tastatur

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Bildschirmhelligkeit

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Wachhalten

# Name of the sound button when the sound is muted.
ring-tray-muted = Stumm

# Name of the sound button with the volume level.
ring-tray-volume = Lautstärke { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Akku { $pct }%, wird geladen
        [onbattery] Akku { $pct }%, im Akkubetrieb
        [full] Akku { $pct }%, voll
        [pluggedin] Akku { $pct }%, angeschlossen
       *[other] Akku { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Netzwerk ist aus

# Name of the network button when nothing is connected.
ring-tray-not-connected = Nicht verbunden

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Über ein VPN verbunden

# Name of the network button when connected by cable.
ring-tray-wired = Kabel

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Kabel, eingeschränkte Verbindung

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, Signal { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, Signal { $strength }%, eingeschränkte Verbindung

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth ist aus

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Verbunden mit { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth ist an

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Symbol im Infobereich

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Alle Tray-Symbole

# Words on the panel of every tray icon when there are none.
ring-tray-none = Keine Tray-Symbole

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Im Ring zeigen

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Aus dem Ring nehmen

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Vier sind schon im Ring. Nimm zuerst eines heraus.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Die Suche ist aus

