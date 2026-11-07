# The ring's tray and the updates button: status buttons for sound, battery, network and Bluetooth, other programs' tray icons, and what the updates button says. Part of the ring's own words (see ring.ftl).

## Updates button (src/ring/updates.rs)

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 plugin update needs a look in Apps
       *[other] { $n } plugin updates need a look in Apps
    }
# Answer to a spoken question about updates when none wait.
ring-updates-none = Everything is up to date
# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } waiting: the updates button beside Apps puts it in
       *[other] { $n } waiting: the updates button beside Apps puts them in
    }
# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } waiting: { $look }
# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } waiting: the updates button beside Apps puts { $put } in, and { $look }
# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] See the firmware update
       *[other] See { $n } firmware updates
    }
# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] See { $n } update, firmware among them
       *[other] See { $n } updates, firmware among them
    }
# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] Install { $n } update
       *[other] Install { $n } updates
    }
# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] See the plugin update, which asks for more than before
       *[other] See { $n } plugin updates, which ask for more than before
    }
# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] Install { $put } update; { $look }
       *[other] Install { $put } updates; { $look }
    }
# Name of the updates button when nothing waits.
ring-updates-up-to-date = Up to date
# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Checking is off

## Tray (src/ring/tray.rs)

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Keyboard
# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Screen brightness
# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Keep awake
# Name of the sound button when the sound is muted.
ring-tray-muted = Muted
# Name of the sound button with the volume level.
ring-tray-volume = Volume { $pct }%
# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Battery { $pct }%, charging
        [onbattery] Battery { $pct }%, on battery
        [full] Battery { $pct }%, full
        [pluggedin] Battery { $pct }%, plugged in
       *[other] Battery { $pct }%
    }
# Name of the network button when networking is switched off.
ring-tray-net-off = Networking is off
# Name of the network button when nothing is connected.
ring-tray-not-connected = Not connected
# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Connected through a VPN
# Name of the network button when connected by cable.
ring-tray-wired = Wired
# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Wired, limited connectivity
# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, signal { $strength }%
# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, signal { $strength }%, limited connectivity
# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth is off
# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Connected to { $names }
# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth is on

## Other programs' tray icons (src/ring/sni/)

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Tray icon
# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = All tray icons
# Words on the panel of every tray icon when there are none.
ring-tray-none = No tray icons
# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Show in the ring
# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Take out of the ring
# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Four are already in the ring. Take one out first.
