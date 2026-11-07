
# machine translated (Hindi), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 प्लगइन अपडेट को ऐप्स में देखना है
       *[other] { $n } प्लगइन अपडेट को ऐप्स में देखना है
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = सब कुछ अद्यतन है

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } प्रतीक्षा में है: ऐप्स के पास का अपडेट बटन इसे लगा देगा
       *[other] { $n } प्रतीक्षा में हैं: ऐप्स के पास का अपडेट बटन इन्हें लगा देगा
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } प्रतीक्षा में: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } प्रतीक्षा में: ऐप्स के पास का अपडेट बटन { $put } को लगा देगा, और { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] फ़र्मवेयर अपडेट देखें
       *[other] { $n } फ़र्मवेयर अपडेट देखें
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] { $n } अपडेट देखें, उनमें फ़र्मवेयर भी है
       *[other] { $n } अपडेट देखें, उनमें फ़र्मवेयर भी है
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] { $n } अपडेट लगाएँ
       *[other] { $n } अपडेट लगाएँ
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] प्लगइन अपडेट देखें, जो पहले से ज़्यादा अनुमति माँगता है
       *[other] { $n } प्लगइन अपडेट देखें, जो पहले से ज़्यादा अनुमति माँगते हैं
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] { $put } अपडेट लगाएँ; { $look }
       *[other] { $put } अपडेट लगाएँ; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = अद्यतन है

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = कीबोर्ड

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = स्क्रीन की चमक

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = जगाए रखें

# Name of the sound button when the sound is muted.
ring-tray-muted = म्यूट

# Name of the sound button with the volume level.
ring-tray-volume = आवाज़ { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] बैटरी { $pct }%, चार्ज हो रही है
        [onbattery] बैटरी { $pct }%, बैटरी पर
        [full] बैटरी { $pct }%, पूरी चार्ज
        [pluggedin] बैटरी { $pct }%, प्लग लगा है
       *[other] बैटरी { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = नेटवर्क बंद है

# Name of the network button when nothing is connected.
ring-tray-not-connected = कनेक्ट नहीं है

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = VPN से कनेक्ट है

# Name of the network button when connected by cable.
ring-tray-wired = तार वाला

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = तार वाला, सीमित कनेक्टिविटी

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, सिग्नल { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, सिग्नल { $strength }%, सीमित कनेक्टिविटी

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth बंद है

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = { $names } से कनेक्ट है

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth चालू है

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = ट्रे आइकन

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = सभी ट्रे आइकन

# Words on the panel of every tray icon when there are none.
ring-tray-none = कोई ट्रे आइकन नहीं

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = रिंग में दिखाएँ

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = रिंग से निकालें

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = रिंग में पहले से चार हैं। पहले एक निकालें।



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = जाँच बंद है

