# machine translated (Arabic), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [zero] لا توجد تحديثات إضافات تحتاج إلى نظرة في التطبيقات
        [one] تحديث إضافة واحد يحتاج إلى نظرة في التطبيقات
        [two] تحديثا إضافتين يحتاجان إلى نظرة في التطبيقات
        [few] { $n } تحديثات إضافات تحتاج إلى نظرة في التطبيقات
        [many] { $n } تحديث إضافة يحتاج إلى نظرة في التطبيقات
       *[other] { $n } تحديث إضافة يحتاج إلى نظرة في التطبيقات
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = كل شيء محدَّث

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [zero] لا شيء بالانتظار
        [one] يوجد تحديث واحد بالانتظار. استخدم زر التحديثات بجانب التطبيقات لتثبيته.
        [two] يوجد تحديثان بالانتظار. استخدم زر التحديثات بجانب التطبيقات لتثبيتهما.
        [few] توجد { $n } تحديثات بالانتظار. استخدم زر التحديثات بجانب التطبيقات لتثبيتها.
        [many] يوجد { $n } تحديثاً بالانتظار. استخدم زر التحديثات بجانب التطبيقات لتثبيتها.
       *[other] يوجد { $n } تحديث بالانتظار. استخدم زر التحديثات بجانب التطبيقات لتثبيتها.
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look =
    { $n ->
        [zero] لا توجد تحديثات بالانتظار. { $look }.
        [one] يوجد تحديث واحد بالانتظار. { $look }.
        [two] يوجد تحديثان بالانتظار. { $look }.
        [few] { $n } تحديثات بالانتظار. { $look }.
        [many] { $n } تحديثًا بالانتظار. { $look }.
       *[other] { $n } تحديث بالانتظار. { $look }.
    }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = عدد التحديثات بالانتظار: { $n }. استخدم زر التحديثات بجانب التطبيقات لتثبيت { $put } منها. { $look }.

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [zero] لا توجد تحديثات للبرنامج الثابت
        [one] اعرض تحديث البرنامج الثابت
        [two] اعرض تحديثي البرنامج الثابت
        [few] اعرض { $n } تحديثات للبرنامج الثابت
        [many] اعرض { $n } تحديثًا للبرنامج الثابت
       *[other] اعرض { $n } تحديث للبرنامج الثابت
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [zero] لا توجد تحديثات
        [one] اعرض { $n } تحديثًا، بما فيه تحديث للبرنامج الثابت
        [two] اعرض { $n } تحديثين، بما فيها تحديث للبرنامج الثابت
        [few] اعرض { $n } تحديثات، بما فيها تحديث للبرنامج الثابت
        [many] اعرض { $n } تحديثًا، بما فيها تحديث للبرنامج الثابت
       *[other] اعرض { $n } تحديث، بما فيها تحديث للبرنامج الثابت
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [zero] لا توجد تحديثات للتثبيت
        [one] ثبّت { $n } تحديثًا
        [two] ثبّت { $n } تحديثين
        [few] ثبّت { $n } تحديثات
        [many] ثبّت { $n } تحديثًا
       *[other] ثبّت { $n } تحديث
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [zero] لا توجد تحديثات إضافات
        [one] اعرض تحديث الإضافة، وهو يطلب أذونات أكثر من قبل
        [two] اعرض تحديثي الإضافتين، وهما يطلبان أذونات أكثر من قبل
        [few] اعرض { $n } تحديثات إضافات، وهي تطلب أذونات أكثر من قبل
        [many] اعرض { $n } تحديث إضافة، وهي تطلب أذونات أكثر من قبل
       *[other] اعرض { $n } تحديث إضافة، وهي تطلب أذونات أكثر من قبل
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [zero] لا شيء للتثبيت؛ { $look }
        [one] ثبّت { $put } تحديثًا؛ { $look }
        [two] ثبّت { $put } تحديثين؛ { $look }
        [few] ثبّت { $put } تحديثات؛ { $look }
        [many] ثبّت { $put } تحديثًا؛ { $look }
       *[other] ثبّت { $put } تحديث؛ { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = محدَّث

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = لوحة المفاتيح

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = سطوع الشاشة

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = أبقِ الحاسوب مستيقظًا

# Name of the sound button when the sound is muted.
ring-tray-muted = الصوت مكتوم

# Name of the sound button with the volume level.
ring-tray-volume = مستوى الصوت { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] البطارية { $pct }%، قيد الشحن
        [onbattery] البطارية { $pct }%، تعمل بالبطارية
        [full] البطارية { $pct }%، ممتلئة
        [pluggedin] البطارية { $pct }%، موصولة بالكهرباء
       *[other] البطارية { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = الشبكة متوقفة

# Name of the network button when nothing is connected.
ring-tray-not-connected = غير متصل

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = متصل عبر VPN

# Name of the network button when connected by cable.
ring-tray-wired = سلكي

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = سلكي، بلا إنترنت

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }، الإشارة { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }، الإشارة { $strength }%، وصول محدود

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth متوقف

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = متصل بـ { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth يعمل

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = أيقونة الصينية

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = كل أيقونات الصينية

# Words on the panel of every tray icon when there are none.
ring-tray-none = لا توجد أيقونات صينية

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = أظهر في الحلقة

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = أزل من الحلقة

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = في الحلقة أربع أيقونات بالفعل. أخرج واحدة أولاً.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = الفحص متوقف
# Name of the updates button when a restart to finish installing was put off with Remind me later and the time has come.
ring-updates-restart = أعد التشغيل لإنهاء التثبيت
