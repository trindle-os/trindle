
# machine translated (Arabic), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = قبل لحظات

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [zero] قبل أقل من دقيقة
        [one] قبل دقيقة واحدة
        [two] قبل دقيقتين
        [few] قبل { $n } دقائق
        [many] قبل { $n } دقيقة
       *[other] قبل { $n } دقيقة
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [zero] قبل أقل من ساعة
        [one] قبل ساعة واحدة
        [two] قبل ساعتين
        [few] قبل { $n } ساعات
        [many] قبل { $n } ساعة
       *[other] قبل { $n } ساعة
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [zero] قبل أقل من يوم
        [one] قبل يوم واحد
        [two] قبل يومين
        [few] قبل { $n } أيام
        [many] قبل { $n } يومًا
       *[other] قبل { $n } يوم
    }

# SAFETY: needs a human review before it ships
# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
settings-blocking-waiting = بانتظار كلمة المرور...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = الحظر يعمل

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = الحظر متوقف

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = تم الحفظ

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = اكتب اسم موقع، مثل example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = الحظر متوقف. تذهب عمليات البحث إلى خوادم شبكتك نفسها كما كان من قبل.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = الحظر قيد البدء.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = تُحظر الإعلانات والمواقع السيئة، وتذهب عمليات البحث بسرية إلى { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = لا يمكن الوصول إلى الخادم الخاص في هذه الشبكة، لذلك تُستخدم خوادم شبكتك نفسها الآن. ما زال الحظر يعمل.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = تُحظر الإعلانات والمواقع السيئة. تذهب عمليات البحث إلى خوادم شبكتك نفسها، التي تستطيع رؤيتها.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = تُحظر الإعلانات والمواقع السيئة. تذهب عمليات البحث إلى خوادم VPN نفسها، عبر VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = تُحظر الإعلانات والمواقع السيئة.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = قيد البدء

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = متوقف

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = الحظر قيد البدء

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = الحظر متوقف

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = المحظور اليوم

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = عمليات بحث محظورة اليوم

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = القوائم

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = جارٍ الجلب

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = لا شيء بعد

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = قوائم المواقع

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = موقعًا مدرجًا، جُلبت { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = لم يُحظر شيء بعد

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = تظهر المواقع هنا، الأحدث أولًا.

# SAFETY: needs a human review before it ships
# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = السماح بهذا الموقع دائمًا

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = المحظور حديثًا

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = تظهر المواقع هنا عند حظرها، الأحدث أولًا. زر الزائد يسمح بموقع منها من الآن فصاعدًا. لا تُعرض هنا إلا المواقع المحظورة. أما المواقع التي سُمح بها فلا تُحفظ أبدًا.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = شغّل الحظر لترى ما يحظره.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = مثل example.com

# SAFETY: needs a human review before it ships
# Heading of the line with the box for adding a site.
settings-blocking-add = إضافة موقع

# SAFETY: needs a human review before it ships
# Small line under "Add a site".
settings-blocking-add-sub = لا يُحظر هو ولا أي من نطاقاته الفرعية أبدًا.

# SAFETY: needs a human review before it ships
# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = حظر هذا الموقع من جديد

# SAFETY: needs a human review before it ships
# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = مسموح به دائمًا

# SAFETY: needs a human review before it ships
# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = المواقع المدرجة هنا ونطاقاتها الفرعية كلها لا تُحظر أبدًا، حتى لو كانت في القوائم. زر الناقص يحظر موقعًا من جديد.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = الحظر

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = حظر إعلانات متتبعات برمجيات خبيثة احتيال dns خاص بحث block adverts ads trackers malware scams private lookups quad9 mullvad cloudflare allow

# Help text at the top of the page.
settings-blocking-help = يحظر الإعلانات والمتتبعات والمواقع السيئة المعروفة لكل متصفح وتطبيق على هذا الحاسوب، ويرسل عمليات البحث عن الأسماء عبر اتصال مشفّر، فلا تستطيع الشبكة التي تتصل بها قراءتها أو تغييرها.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = حظر الإعلانات والمواقع السيئة

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = وجهة عمليات البحث

# Help text (i button) of the Blocking card.
settings-blocking-card-about = يستخدم قوائم HaGeZi للإعلانات والمتتبعات والمواقع السيئة المعروفة، وتُجلب مرة في اليوم. حين يكون هذا متوقفًا لا يعمل شيء. تذهب عمليات البحث عن الأسماء عبر اتصال مشفّر إلى الخادم الخاص المختار هنا. لا يحتفظ Quad9 بسجل عمّن سأل. خيار "شبكتك" يحظر أيضًا، لكنه ليس خاصًا: فخوادم شبكتك نفسها ترى كل عملية بحث.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = قوائم HaGeZi، تُجلب مرة في اليوم.

# Heading of the line with the private server dropdown.
settings-blocking-private = الخادم الخاص

# Small line under Private server.
settings-blocking-private-sub = لا يحتفظ Quad9 بسجل عمّن سأل.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = شبكتك
