
# machine translated (Swahili), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] sasisho 1 la programu jalizi linahitaji kutazamwa kwenye Programu
       *[other] masasisho { $n } ya programu jalizi yanahitaji kutazamwa kwenye Programu
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Kila kitu kimesasishwa

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } linasubiri: kitufe cha masasisho kando ya Programu kitalisakinisha
       *[other] { $n } yanasubiri: kitufe cha masasisho kando ya Programu kitayasakinisha
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } yanasubiri: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } yanasubiri: kitufe cha masasisho kando ya Programu kitasakinisha { $put }, na { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Tazama sasisho la firmware
       *[other] Tazama masasisho { $n } ya firmware
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] Tazama sasisho { $n }, firmware likiwemo
       *[other] Tazama masasisho { $n }, firmware ikiwemo
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] Sakinisha sasisho { $n }
       *[other] Sakinisha masasisho { $n }
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Tazama sasisho la programu jalizi, linaloomba ruhusa zaidi ya awali
       *[other] Tazama masasisho { $n } ya programu jalizi, yanayoomba ruhusa zaidi ya awali
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] Sakinisha sasisho { $put }; { $look }
       *[other] Sakinisha masasisho { $put }; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Imesasishwa

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Kibodi

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Mwangaza wa skrini

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Zuia kulala

# Name of the sound button when the sound is muted.
ring-tray-muted = Sauti imenyamazishwa

# Name of the sound button with the volume level.
ring-tray-volume = Sauti { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Betri { $pct }%, inachaji
        [onbattery] Betri { $pct }%, inatumia betri
        [full] Betri { $pct }%, imejaa
        [pluggedin] Betri { $pct }%, imechomekwa
       *[other] Betri { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Mtandao umezimwa

# Name of the network button when nothing is connected.
ring-tray-not-connected = Haijaunganishwa

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Imeunganishwa kupitia VPN

# Name of the network button when connected by cable.
ring-tray-wired = Kwa kebo

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Kwa kebo, muunganisho mdogo

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, mawimbi { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, mawimbi { $strength }%, muunganisho mdogo

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth imezimwa

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Imeunganishwa na { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth imewashwa

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Ikoni ya trei

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Aikoni zote za treya

# Words on the panel of every tray icon when there are none.
ring-tray-none = Hakuna aikoni za treya

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Onyesha kwenye pete

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Toa kwenye pete

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Nne tayari ziko kwenye pete. Toa moja kwanza.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Ukaguzi umezimwa

