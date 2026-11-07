# machine translated (Bengali), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
# SAFETY: needs a human review before it ships
ring-updates-look =
    { $n ->
        [one] ১টি প্লাগইন আপডেট অ্যাপে একবার দেখা দরকার
       *[other] { $n }টি প্লাগইন আপডেট অ্যাপে একবার দেখা দরকার
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = সবকিছু হালনাগাদ আছে

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n }টি অপেক্ষায় আছে: অ্যাপের পাশের আপডেট বোতামটি সেটি ইনস্টল করে দেবে
       *[other] { $n }টি অপেক্ষায় আছে: অ্যাপের পাশের আপডেট বোতামটি সেগুলো ইনস্টল করে দেবে
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n }টি অপেক্ষায় আছে: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n }টি অপেক্ষায় আছে: অ্যাপের পাশের আপডেট বোতামটি { $put }টি ইনস্টল করে দেবে, আর { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] ফার্মওয়্যার আপডেটটি দেখুন
       *[other] { $n }টি ফার্মওয়্যার আপডেট দেখুন
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] { $n }টি আপডেট দেখুন, এর মধ্যে ফার্মওয়্যার আছে
       *[other] { $n }টি আপডেট দেখুন, এর মধ্যে ফার্মওয়্যার আছে
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] { $n }টি আপডেট ইনস্টল করুন
       *[other] { $n }টি আপডেট ইনস্টল করুন
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
# SAFETY: needs a human review before it ships
ring-updates-see-plugin =
    { $n ->
        [one] প্লাগইন আপডেটটি দেখুন, যা আগের চেয়ে বেশি অনুমতি চায়
       *[other] { $n }টি প্লাগইন আপডেট দেখুন, যা আগের চেয়ে বেশি অনুমতি চায়
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] { $put }টি আপডেট ইনস্টল করুন; { $look }
       *[other] { $put }টি আপডেট ইনস্টল করুন; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = হালনাগাদ আছে

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = কিবোর্ড

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = পর্দার উজ্জ্বলতা

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = জাগিয়ে রাখুন

# Name of the sound button when the sound is muted.
ring-tray-muted = শব্দ বন্ধ

# Name of the sound button with the volume level.
ring-tray-volume = শব্দ { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] ব্যাটারি { $pct }%, চার্জ হচ্ছে
        [onbattery] ব্যাটারি { $pct }%, ব্যাটারিতে চলছে
        [full] ব্যাটারি { $pct }%, পূর্ণ
        [pluggedin] ব্যাটারি { $pct }%, প্লাগ লাগানো
       *[other] ব্যাটারি { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = নেটওয়ার্ক বন্ধ আছে

# Name of the network button when nothing is connected.
ring-tray-not-connected = সংযুক্ত নয়

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = ভিপিএনের মাধ্যমে সংযুক্ত

# Name of the network button when connected by cable.
ring-tray-wired = তারযুক্ত

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = তারযুক্ত, সীমিত সংযোগ

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, সিগন্যাল { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, সিগন্যাল { $strength }%, সীমিত সংযোগ

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth বন্ধ আছে

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = { $names } এর সঙ্গে সংযুক্ত

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth চালু আছে

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = ট্রে আইকন

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = সব ট্রে আইকন

# Words on the panel of every tray icon when there are none.
ring-tray-none = কোনো ট্রে আইকন নেই

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = রিংয়ে দেখান

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = রিং থেকে বের করুন

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = রিংয়ে আগে থেকেই চারটি আছে। আগে একটি বের করুন।



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = দেখা বন্ধ আছে

