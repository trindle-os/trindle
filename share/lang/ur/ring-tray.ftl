
# machine translated (Urdu), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 پلگ اِن اپ ڈیٹ کو ایپس میں دیکھنے کی ضرورت ہے
       *[other] { $n } پلگ اِن اپ ڈیٹس کو ایپس میں دیکھنے کی ضرورت ہے
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = سب کچھ تازہ ترین ہے

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } منتظر ہے: ایپس کے ساتھ والا اپ ڈیٹ بٹن اسے انسٹال کر دے گا
       *[other] { $n } منتظر ہیں: ایپس کے ساتھ والا اپ ڈیٹ بٹن انہیں انسٹال کر دے گا
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } منتظر ہیں: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } منتظر ہیں: ایپس کے ساتھ والا اپ ڈیٹ بٹن { $put } انسٹال کر دے گا، اور { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] فرم ویئر اپ ڈیٹ دیکھیں
       *[other] { $n } فرم ویئر اپ ڈیٹس دیکھیں
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] { $n } اپ ڈیٹ دیکھیں، جن میں فرم ویئر شامل ہے
       *[other] { $n } اپ ڈیٹس دیکھیں، جن میں فرم ویئر شامل ہے
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] { $n } اپ ڈیٹ انسٹال کریں
       *[other] { $n } اپ ڈیٹس انسٹال کریں
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] پلگ اِن اپ ڈیٹ دیکھیں، جو پہلے سے زیادہ اجازتیں مانگتی ہے
       *[other] { $n } پلگ اِن اپ ڈیٹس دیکھیں، جو پہلے سے زیادہ اجازتیں مانگتی ہیں
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] { $put } اپ ڈیٹ انسٹال کریں؛ { $look }
       *[other] { $put } اپ ڈیٹس انسٹال کریں؛ { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = تازہ ترین

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = کی بورڈ

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = اسکرین کی روشنی

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = جاگتا رکھیں

# Name of the sound button when the sound is muted.
ring-tray-muted = خاموش

# Name of the sound button with the volume level.
ring-tray-volume = آواز { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] بیٹری { $pct }%، چارج ہو رہی ہے
        [onbattery] بیٹری { $pct }%، بیٹری پر چل رہا ہے
        [full] بیٹری { $pct }%، بھر گئی
        [pluggedin] بیٹری { $pct }%، پلگ لگا ہوا ہے
       *[other] بیٹری { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = نیٹ ورک بند ہے

# Name of the network button when nothing is connected.
ring-tray-not-connected = منسلک نہیں

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = VPN کے ذریعے منسلک

# Name of the network button when connected by cable.
ring-tray-wired = تار والا

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = تار والا، محدود رابطہ

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }، سگنل { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }، سگنل { $strength }%، محدود رابطہ

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth بند ہے

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = { $names } سے منسلک

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth چالو ہے

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = ٹرے آئیکن

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = تمام ٹرے آئیکن

# Words on the panel of every tray icon when there are none.
ring-tray-none = کوئی ٹرے آئیکن نہیں

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = رِنگ میں دکھائیں

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = رِنگ سے نکالیں

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = رِنگ میں پہلے ہی چار ہیں۔ پہلے ایک نکالیں۔



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = جانچ بند ہے

