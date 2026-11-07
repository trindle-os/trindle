
# machine translated (Arabic), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = افتح إعداداتها

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = لم يتم: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = يحتاج هذا التحديث إلى إذن جديد. وهو بانتظارك.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = الإصدار { $mine } حاليًا، والإصدار { $new } جاهز

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = الإصدار { $mine } حاليًا، والإصدار { $new } جاهز. يطلب أكثر مما سبق: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = ستعود الأزرار هنا عندما ينتهي.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = جارٍ البحث عن الإضافات…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = لا توجد إضافات مثبّتة، ولا توجد إضافات معروضة بعد.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [zero] لا توجد إضافات مثبّتة بعد؛ ولا واحدة جاهزة للتثبيت.
        [one] لا توجد إضافات مثبّتة بعد؛ إضافة واحدة جاهزة للتثبيت.
        [two] لا توجد إضافات مثبّتة بعد؛ إضافتان جاهزتان للتثبيت.
        [few] لا توجد إضافات مثبّتة بعد؛ { $n } إضافات جاهزة للتثبيت.
        [many] لا توجد إضافات مثبّتة بعد؛ { $n } إضافة جاهزة للتثبيت.
       *[other] لا توجد إضافات مثبّتة بعد؛ { $n } إضافة جاهزة للتثبيت.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [zero] لا توجد إضافات مثبّتة
        [one] إضافة واحدة مثبّتة
        [two] إضافتان مثبّتتان
        [few] { $n } إضافات مثبّتة
        [many] { $n } إضافة مثبّتة
       *[other] { $n } إضافة مثبّتة
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [zero] لا إضافة لها تحديث
        [one] إضافة واحدة لها تحديث
        [two] إضافتان لهما تحديث
        [few] { $n } إضافات لها تحديث
        [many] { $n } إضافة لها تحديث
       *[other] { $n } إضافة لها تحديث
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [zero] لا تحديثات محجوزة
        [one] تحديث واحد محجوز
        [two] تحديثان محجوزان
        [few] { $n } تحديثات محجوزة
        [many] { $n } تحديثًا محجوزًا
       *[other] { $n } تحديث محجوز
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }؛ { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }، وكلها محدّثة.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }، و{ $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }، و{ $have }؛ { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = لا تطلب شيئًا.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = ما تطلبه: { $list }.

# Title of the Plugins page.
store-plugins-title = الإضافات

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = الإصدار { $mine } مثبّت؛ والإصدار { $new } جاهز.

# On a plugin's card: its installed version.
store-plugins-version-installed = الإصدار { $version }، مثبّت.

# On a plugin's card: the version on offer.
store-plugins-version-offered = الإصدار { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
store-plugins-install = ثبّت { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = أطلق { $name }

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = حدّث إلى { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = من أين تأتي الإضافات

# Information on the card that says where plugins come from.
store-plugins-come-from = تأتي الإضافات من قائمة Trindle نفسها فقط، وتُقرأ من trindle.org في كل مرة تُفتح فيها هذه الصفحة. لعرض إضافة، قدّمها: تُختبر كل واحدة قبل أن تُنشر.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = تم تثبيتها، لكن جزءها الذي يعمل في الخلفية لا يعمل: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = تعذّرت إزالتها: { $why }

