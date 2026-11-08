
# machine translated (Arabic), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = ملف Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = ملف .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = إضافة

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = مرشح للإصدار

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's tile while it is being removed.
store-list-removing = جارٍ إزالة { $name }

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's tile while it is being installed.
store-list-installing = جارٍ تثبيت { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = فتح { $name }

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's tile that installs the app.
store-list-install = تثبيت { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = مثبّت

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = إظهار

# How long is left when it is a minute or less.
store-left-minute = بقيت نحو دقيقة

# How long is left, in minutes.
store-left-minutes = بقيت نحو { $n } دقيقة

# How long is left, in hours.
store-left-hours =
    { $n ->
        [zero] لم تبقَ ساعات
        [one] بقيت نحو ساعة واحدة
        [two] بقيت نحو ساعتين
        [few] بقيت نحو { $n } ساعات
        [many] بقيت نحو { $n } ساعة
       *[other] بقيت نحو { $n } ساعة
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = جرى الفحص مقابل قائمة HaGeZi's Threat Intelligence

# Where a download came from: the site and the page that led to it.
store-origin-both = نُزّل من { $site }، وجرى الوصول إليه من { $referrer }.

# Where a download came from: the site.
store-origin-site = نُزّل من { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = جرى الوصول إليه من { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = لا يستطيع Trindle معرفة مصدر هذا.

# SAFETY: needs a human review before it ships
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name } موجود في قائمة HaGeZi's Threat Intelligence للمواقع المعروفة بالتنزيلات الضارة والاحتيال. ثبّت هذا فقط إن كنت متأكدًا من مصدره.

# SAFETY: needs a human review before it ships
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } الموقع غير موجود في قائمة HaGeZi's Threat Intelligence، لكن القائمة غير كاملة، لذلك هذا لا يثبت أنه آمن.

# SAFETY: needs a human review before it ships
# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } تعذّر فحص الموقع لأنه لم يمكن جلب قائمة HaGeZi's Threat Intelligence.

# Name shown for a review whose writer gave none.
store-review-someone = شخص ما

# Red words when the keyring could not be reached.
store-key-unreached = تعذّر الوصول إلى سلسلة المفاتيح: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = المفتاح المحفوظ ليس نصًا.

# Red words when a file that held the key could not be removed.
store-key-not-removed = تعذّرت إزالة { $path }: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = تعذّرت قراءة { $path }: { $why }
