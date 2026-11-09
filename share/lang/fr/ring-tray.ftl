
# machine translated (French), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 mise à jour de module demande un examen dans Applications
        [many] { $n } mises à jour de modules demandent un examen dans Applications
       *[other] { $n } mises à jour de modules demandent un examen dans Applications
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Tout est à jour

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } en attente : le bouton des mises à jour, à côté d'Applications, l'installe
        [many] { $n } en attente : le bouton des mises à jour, à côté d'Applications, les installe
       *[other] { $n } en attente : le bouton des mises à jour, à côté d'Applications, les installe
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } en attente : { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } en attente : le bouton des mises à jour, à côté d'Applications, en installe { $put }, et { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Voir la mise à jour du micrologiciel
        [many] Voir { $n } mises à jour du micrologiciel
       *[other] Voir { $n } mises à jour du micrologiciel
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] Voir { $n } mise à jour, dont une du micrologiciel
        [many] Voir { $n } mises à jour, dont du micrologiciel
       *[other] Voir { $n } mises à jour, dont du micrologiciel
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] Installer { $n } mise à jour
        [many] Installer { $n } mises à jour
       *[other] Installer { $n } mises à jour
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Voir la mise à jour du module, qui demande plus qu'avant
        [many] Voir { $n } mises à jour de modules, qui demandent plus qu'avant
       *[other] Voir { $n } mises à jour de modules, qui demandent plus qu'avant
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] Installer { $put } mise à jour ; { $look }
        [many] Installer { $put } mises à jour ; { $look }
       *[other] Installer { $put } mises à jour ; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = À jour

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Clavier

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Luminosité de l'écran

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Garder éveillé

# Name of the sound button when the sound is muted.
ring-tray-muted = Son coupé

# Name of the sound button with the volume level.
ring-tray-volume = Volume { $pct } %

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Batterie { $pct } %, en charge
        [onbattery] Batterie { $pct } %, sur batterie
        [full] Batterie { $pct } %, pleine
        [pluggedin] Batterie { $pct } %, branchée
       *[other] Batterie { $pct } %
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Le réseau est désactivé

# Name of the network button when nothing is connected.
ring-tray-not-connected = Non connecté

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Connecté via un VPN

# Name of the network button when connected by cable.
ring-tray-wired = Filaire

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Filaire, connectivité limitée

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, signal { $strength } %

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, signal { $strength } %, connectivité limitée

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Le Bluetooth est désactivé

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Connecté à { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Le Bluetooth est activé

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Icône de la zone de notification

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Toutes les icônes de la zone de notification

# Words on the panel of every tray icon when there are none.
ring-tray-none = Aucune icône de la zone de notification

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Afficher dans l'anneau

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Retirer de l'anneau

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Quatre sont déjà dans l'anneau. Retirez-en une d'abord.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = La vérification est désactivée

