
# machine translated (Arabic), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = حزم Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = محجوز عند الإصدار { $have }؛ { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = محجوز؛ { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = محجوز عند الإصدار { $have }

# On a thing held back with no update waiting.
store-upd-held = محجوز

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = محجوز عند الإصدار { $have }؛ لا شيء أحدث بعد

# On the Held back card: nothing newer waits.
store-upd-held-nothing = محجوز؛ لا شيء أحدث بعد

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [zero] لا تحديثات
        [one] تحديث واحد
        [two] تحديثان
        [few] { $n } تحديثات
        [many] { $n } تحديثًا
       *[other] { $n } تحديث
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [zero] لا تحديثات جاهزة
        [one] تحديث واحد جاهز
        [two] تحديثان جاهزان
        [few] { $n } تحديثات جاهزة
        [many] { $n } تحديثًا جاهزًا
       *[other] { $n } تحديث جاهز
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }، { $size } في المجموع

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [zero] لا شيء محجوز
        [one] واحد محجوز
        [two] اثنان محجوزان
        [few] { $n } محجوزة
        [many] { $n } محجوزًا
       *[other] { $n } محجوز
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }؛ { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = يحتاج البرنامج الثابت إلى إعادة تشغيل ليكتمل.

# Headline while the check runs.
store-upd-checking = جارٍ البحث عن تحديثات.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = جارٍ تحديث { $name }. يمكنك متابعة العمل أثناء ذلك.

# Headline while all the updates go in.
store-upd-installing-all = جارٍ تثبيت التحديثات. يمكنك متابعة العمل أثناء ذلك.

# Headline when the updates are in and the computer must start again.
store-upd-restart = أعد التشغيل ليكتمل الأمر: التحديثات مثبّتة، وتدخل حيز الاستخدام عندما يبدأ الحاسوب من جديد.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = ما ثُبّت يحتاج إلى إعادة تشغيل ليكتمل.

# Added to the headline with no internet.
store-upd-no-more = هذا الحاسوب غير متصل بالإنترنت، فلا يمكن البحث عن المزيد.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = لا توجد تحديثات جاهزة

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = لا توجد تحديثات منتظرة يعرفها هذا الحاسوب. ولا يستطيع البحث عن المزيد ما دام غير متصل بالإنترنت.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = التطبيقات

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle وDebian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = البرنامج الثابت

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = تعذّر البحث عن تحديثات لـ { $names }.

# Headline when everything is up to date.
store-upd-current = كل شيء محدَّث.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = آخر فحص عند { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = لا يمكن الفحص دون إنترنت

# Red words under a card when a check took too long.
store-upd-too-long = استغرق الرد وقتًا طويلًا.

# Tip of the round button that checks again.
store-upd-check-again = افحص من جديد

# Main button while the check runs.
store-upd-checking-main = جارٍ الفحص

# Main button while the updates go in.
store-upd-installing-main = جارٍ التثبيت

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [zero] لا تحديثات للتثبيت
        [one] ثبّت تحديثًا واحدًا
        [two] ثبّت تحديثين
        [few] ثبّت { $n } تحديثات
        [many] ثبّت { $n } تحديثًا
       *[other] ثبّت { $n } تحديث
    }

# Main button that restarts the computer.
store-upd-restart-now = أعد التشغيل الآن

# Main button that checks for updates.
store-upd-check-for = ابحث عن تحديثات

# Tip of the round button that releases a thing held back.
store-upd-release = أطلق { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = احجز { $name }

# A card's state while it is being checked.
store-upd-status-checking = جارٍ الفحص

# A card's state while its updates go in.
store-upd-status-installing = جارٍ التثبيت

# A card's state when its check failed; the reason follows.
store-upd-status-failed = تعذّر الفحص. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }، { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }؛ { $held } محجوز

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } محجوز

# A card's state when nothing waits.
store-upd-up-to-date = محدَّث

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = من { $from }، { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = من { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = جزء مشترك، من { $from }

# Note under the Apps card.
store-upd-yours-alone = لك وحدك، لذلك تتحدّث دون كلمة مرور. ويجلب التطبيق الأجزاء المشتركة التي يحتاجها إصداره الجديد.

# Note under the Apps card: what holding back does.
store-upd-hold-why = التحديث المحجوز يبقى عند الإصدار الذي هو عليه، وخارج تثبيت الكل هنا وفي الحلقة، إلى أن تطلقه.

# Heading of the Apps card.
store-upd-card-apps = التطبيقات

# Plugins card: every plugin is up to date.
store-upd-plugins-current = كل إضافة محدَّثة.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [zero] لا تحديثات محجوزة للإضافات.
        [one] تحديث إضافة واحدة محجوز.
        [two] تحديثا إضافتين محجوزان.
        [few] تحديثات { $n } إضافات محجوزة.
        [many] تحديثات { $n } إضافة محجوزة.
       *[other] تحديثات { $n } إضافة محجوزة.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [zero] لا إضافة لها تحديث
        [one] إضافة واحدة لها تحديث
        [two] إضافتان لهما تحديث
        [few] { $n } إضافات لها تحديث
        [many] { $n } إضافة لها تحديث
       *[other] { $n } إضافة لها تحديث
    }

# Heading of the Plugins card.
store-upd-card-plugins = الإضافات

# Held back card: nothing is held back.
store-upd-nothing-held = لا شيء محجوز.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = أُطلق

# Note under the Held back card: what releasing does.
store-upd-release-why = ما يُطلق يدخل مع البقية من جديد، من تثبيت الكل هنا وفي الحلقة.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = البرنامج الثابت لجهاز

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = البرنامج الثابت للجهاز { $id }

# Heading of the Held back card.
store-upd-card-held = محجوز

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = من { $from } إلى { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [zero] لا تحديثات
        [one] تحديث واحد، يُثبَّت وحده
        [two] تحديثان، يُثبَّتان معًا
        [few] { $n } تحديثات، تُثبَّت معًا
        [many] { $n } تحديثًا، تُثبَّت معًا
       *[other] { $n } تحديث، تُثبَّت معًا
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = من Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = يُعرض { $shown } من { $total } من تحديثات Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = على هذا الحاسوب Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle وDebian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = حزم Debian مصنوعة لتنسجم معًا، وتثبيت واحدة منفردة قد يجلب معها حزمًا أخرى أو يتركها غير متوافقة. لذلك تُثبَّت معًا، بزر واحد.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = تحديثات Trindle وDebian تحمل إصلاحات أمنية للحاسوب كله، لذلك لا يمكن حجزها.

# Heading of the card about the daily security updates.
store-upd-card-security = التحديثات الأمنية

# On the security card: its state.
store-upd-security-on = تعمل، كل يوم

# On the security card.
store-upd-security-note = تُثبَّت إصلاحات Debian كل يوم دون سؤال.

# On the security card.
store-upd-security-part = جزء من Trindle؛ ولا يمكن إيقافه.

# On the security card: that holding back never stops these.
store-upd-never-held = حجز تحديث لا يوقف هذه التحديثات أبدًا.

# Firmware card: the computer has no updater.
store-upd-fw-none = ليس في هذا الحاسوب أداة لتحديث البرنامج الثابت.

# Firmware card: restart to finish.
store-upd-fw-restart = أعد التشغيل ليكتمل الأمر

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = الإصدار { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = يُثبَّت البرنامج الثابت الجديد عندما يُعاد تشغيل الحاسوب.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = ثبّت fwupd لإبقاء البرنامج الثابت محدَّثًا.

# Heading of the Firmware card.
store-upd-card-firmware = البرنامج الثابت

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = تعذّر على fwupd جلب أحدث قائمة.

# Red words when Flathub could not be reached.
store-upd-no-flathub = تعذّر الوصول إلى Flathub.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = تعذّر جلب قوائم Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = تعذّر جلب قوائم Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = لم يُحجز. تعذّر حفظ قائمة المحجوزات: { $why }.

# The same, for a release.
store-upd-not-released-list = لم يُطلق. تعذّر حفظ قائمة المحجوزات: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = حُجز في التطبيقات، لكن تعذّر إخبار Flatpak نفسه، فقد يثبّته شيء آخر يحدّث التطبيقات.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = أُطلق في التطبيقات، لكن تعذّر إخبار Flatpak نفسه وقد يتركه خارجًا. احجزه ثم أطلقه من جديد.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } قال Flatpak: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle



# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = البحث التلقائي عن التحديثات متوقف. اضغط «البحث عن تحديثات» لتبحث.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = لم يُفحص

# Tip of the round button that ends an update that is running.
store-upd-stop = إيقاف

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = جارٍ الإيقاف

# Under a row whose update was stopped.
store-upd-stopped = أُوقف. اضغط «تحديث» للمتابعة.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = أُوقف. اضغط «تثبيت» للمتابعة.

# Note under the Apps card.
store-upd-steam-restarts = يُعاد تشغيل Steam عندما لا تكون أي لعبة قيد التشغيل.

