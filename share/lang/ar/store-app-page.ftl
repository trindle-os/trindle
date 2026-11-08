
# machine translated (Arabic), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.

# Under the app's name when only the maker is known.
store-app-line-by = من { $by }.

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }، من { $by }.

# The state beside the dot while the app is being taken off.
store-app-state-removing = جارٍ الإزالة

# The state beside the dot while the app is being fetched.
store-app-state-downloading = جارٍ التنزيل

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = جارٍ وضعه في مكانه

# The state beside the dot while the app's own update goes in.
store-app-state-updating = جارٍ التحديث

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = مثبّت، والتحديثات محجوزة

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = مثبّت، والتحديثات محجوزة عند الإصدار { $version }

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = مثبّت، { $waits }

# The same, with the installed version.
store-app-state-waiting-version = مثبّت، الإصدار { $version }، { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = مثبّت، ومحدَّث

# The same, with the installed version.
store-app-state-current-version = مثبّت، الإصدار { $version }، ومحدَّث

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = غير مثبّت

# The same, with the size of the download.
store-app-state-absent-size = غير مثبّت، حجم التنزيل { $size }

# Word beside the spinner while the app is being taken off.
store-app-busy-removing = جارٍ الإزالة

# Word beside the spinner while the app is being put in.
store-app-busy-installing = جارٍ التثبيت

# Tip and spoken name of the round button that gives up a download.
store-app-stop = إيقاف

# Tip and spoken name of the round button that takes the app off.
# SAFETY: needs a human review before it ships
store-app-remove = إزالة

# Round button beside Update or Release that opens the app.
store-app-open-round = فتح

# Main button that releases an app whose updates are held back.
store-app-release = إطلاق

# Main button that updates the app.
store-app-update = تحديث

# Main button that opens the app.
store-app-open = فتح

# Main button that installs the app.
store-app-install = تثبيت

# Button that opens the app's website.
store-app-website = الموقع

# In the picture frame when the app has no pictures.
store-app-no-pictures = لا توجد صور لهذا التطبيق.

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = تعذّر جلب هذه الصورة.

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = جارٍ جلب الصورة…

# In the picture frame when the computer has no internet.
store-app-picture-offline = لا يمكن جلب الصور ما دام هذا الحاسوب غير متصل بالإنترنت.

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = الصورة { $at } من { $n }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = الصورة { $at } من { $n }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = الصورة السابقة

# Tip of the round button that shows the picture after.
store-app-picture-later = الصورة التالية

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = مملوك

# Name of the Facts card.
store-app-facts = معلومات

# Facts line: the size of the download.
store-app-fact-download = حجم التنزيل

# Facts line: where the app comes from.
store-app-fact-source = المصدر

# Facts line: the version.
store-app-fact-version = الإصدار

# Facts line: the licence of an open source app.
store-app-fact-licence = الترخيص

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = الترخيص، غير مفتوح المصدر

# Facts line: who made the app.
store-app-fact-made-by = صنعه

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  من { $n ->
        [zero] لا تقييمات
        [one] تقييم واحد
        [two] تقييمين
        [few] { $count } تقييمات
        [many] { $count } تقييمًا
       *[other] { $count } تقييم
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = لا تقييمات بعد.

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = لا تقييمات للعرض.

# Under the stars while the ratings are being fetched.
store-app-rating-looking = جارٍ البحث عن التقييمات…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = من خدمة المراجعات المفتوحة لدى GNOME.

# Last line of the Rating card when there is no internet.
store-app-rating-offline = هذا الحاسوب غير متصل بالإنترنت.

# Name of the Rating card.
store-app-rating = التقييم

# Text of the About card when the app says nothing of itself.
store-app-no-description = ليس لهذا التطبيق وصف.

# Name of the About card.
store-app-about = حول

# Heading of a review that has no title.
store-app-review-no-title = بلا عنوان

# Under What people say when nobody has written.
store-app-people-none = لم يكتب أحد عن هذا التطبيق بعد.

# Under What people say when there is no internet.
store-app-people-offline = لا يمكن جلب آراء الناس ما دام هذا الحاسوب غير متصل بالإنترنت.

# Under What people say while reviews are being fetched.
store-app-people-looking = جارٍ البحث عن آراء الناس…

# Name of the card of reviews.
store-app-people = ماذا يقول الناس

# Facts line: when the app's project was made.
store-app-fact-created = أُنشئ

