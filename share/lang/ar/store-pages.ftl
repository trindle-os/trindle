
# machine translated (Arabic), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [zero] لا تطبيقات
        [one] تطبيق واحد
        [two] تطبيقان
        [few] { $count } تطبيقات
        [many] { $count } تطبيقًا
       *[other] { $count } تطبيق
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [zero] لا تحديثات منتظرة
        [one] تحديث واحد منتظر
        [two] تحديثان منتظران
        [few] { $count } تحديثات منتظرة
        [many] { $count } تحديثًا منتظرًا
       *[other] { $count } تحديث منتظر
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } للاختيار من Flathub؛ { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = كل ما لديك محدَّث

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = لا توجد تطبيقات تطابق «{ $text }».

# Line under the title of an empty page of apps.
store-grid-none-here = لا توجد تطبيقات هنا بعد.

# Advice in the empty page of a search.
store-grid-try-shorter = جرّب كلمة أقصر، أو تصفّح نوعًا من القائمة على اليسار.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = ابحث بالاسم في الأعلى، أو تصفّح نوعًا آخر من القائمة على اليسار.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = يُعرض { $shown } من { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = اعرض { $next } أخرى

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = جارٍ جلب قائمة التطبيقات من Flathub. المرة الأولى تستغرق دقيقة.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = تعذّر على التطبيقات الوصول إلى Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = هذا الحاسوب غير متصل بالإنترنت. ستحاول التطبيقات من جديد من تلقاء نفسها حالما يتصل.

# In the waiting page when the computer is not connected.
store-wait-comes-from = التطبيقات المعروضة للاختيار تأتي من Flathub عبر الإنترنت.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } وإعدادات التطبيقات، على اليسار، تعمل بدونه.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } وإعدادات التطبيقات، على اليسار، تعمل بدون Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } وإعدادات التطبيقات، على اليسار، تعمل بدون الإنترنت.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = حاول مرة أخرى

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = النتائج

# Red words under the search box when no web browser opens a link.
store-no-browser = لا يوجد متصفح ويب لفتح { $url }. احصل على واحد من التطبيقات أولًا.

# Said in the card or under the title while the lists are being read.
store-looking = جارٍ البحث…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = هو في المثبّتة الآن، وفي الحلقة

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = أُزيل

# The word on an app's dial when installing or removing did not work.
store-done-not = لم يتم

# The word on an app's dial when it was installed.
store-done-installed = مثبّت

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = رفض Flatpak.

# Tab of the Installed page that shows everything.
store-tab-all = الكل

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = ملفات Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = ملفات Deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = الإضافات

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = لا شيء مثبّت بعد. صفحة الاستكشاف مليئة بأشياء تستحق التجربة.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [zero] لا إضافات مثبّتة
        [one] إضافة واحدة مثبّتة
        [two] إضافتان مثبّتتان
        [few] { $n } إضافات مثبّتة
        [many] { $n } إضافة مثبّتة
       *[other] { $n } إضافة مثبّتة
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [zero] لا برامج مثبّتة
        [one] برنامج واحد مثبّت
        [two] برنامجان مثبّتان
        [few] { $n } برامج مثبّتة
        [many] { $n } برنامجًا مثبّتًا
       *[other] { $n } برنامج مثبّت
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = عدد العناصر المثبّتة: { $n }

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }، من Flathub
        [few] { $start }، كلها من Flathub
        [many] { $start }، كلها من Flathub
        [two] { $start }، كلها من Flathub
        [zero] { $start }، كلها من Flathub
       *[other] { $start }، كلها من Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }، من ملف Flatpak
        [few] { $start }، كلها من ملفات Flatpak
        [many] { $start }، كلها من ملفات Flatpak
        [two] { $start }، كلها من ملفات Flatpak
        [zero] { $start }، كلها من ملفات Flatpak
       *[other] { $start }، كلها من ملفات Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }، وهو AppImage
        [few] { $start }، كلها AppImage
        [many] { $start }، كلها AppImage
        [two] { $start }، كلها AppImage
        [zero] { $start }، كلها AppImage
       *[other] { $start }، كلها AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }، من ملف .deb
        [few] { $start }، كلها من ملفات .deb
        [many] { $start }، كلها من ملفات .deb
        [two] { $start }، كلها من ملفات .deb
        [zero] { $start }، كلها من ملفات .deb
       *[other] { $start }، كلها من ملفات .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }، { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } من Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [zero] لا شيء من ملفات Flatpak
        [one] واحد من ملف Flatpak
        [two] اثنان من ملفات Flatpak
        [few] { $n } من ملفات Flatpak
        [many] { $n } من ملفات Flatpak
       *[other] { $n } من ملفات Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [zero] لا AppImage
        [one] AppImage واحد
        [two] AppImage اثنان
        [few] { $n } من AppImage
        [many] { $n } من AppImage
       *[other] { $n } من AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [zero] لا تطبيقات
        [one] تطبيق واحد
        [two] تطبيقان
        [few] { $n } تطبيقات
        [many] { $n } تطبيقًا
       *[other] { $n } تطبيق
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [zero] لا شيء من ملفات .deb
        [one] واحد من ملف .deb
        [two] اثنان من ملفات .deb
        [few] { $n } من ملفات .deb
        [many] { $n } من ملفات .deb
       *[other] { $n } من ملفات .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [zero] لا إضافات
        [one] إضافة واحدة
        [two] إضافتان
        [few] { $n } إضافات
        [many] { $n } إضافة
       *[other] { $n } إضافة
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = وهو محدَّث.

# End of the Installed line when everything is up to date.
store-installed-state-all = كلها محدَّثة.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [zero] لا شيء له تحديث منتظر.
        [one] واحد له تحديث منتظر.
        [two] اثنان لهما تحديثان منتظران.
        [few] { $n } لها تحديثات منتظرة.
        [many] { $n } لها تحديثات منتظرة.
       *[other] { $n } لها تحديثات منتظرة.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } من { $total } برنامجًا تطابق «{ $find }».

# The same when plugins are among the things.
store-installed-match-things = { $shown } من { $total } عنصرًا تطابق «{ $find }».

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = أضف AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = ابحث في المثبّت

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = تعذّرت إضافة AppImage.

# In the card of the Installed page when nothing is installed.
store-installed-empty = التطبيقات التي تثبّتها من الاستكشاف أو من GitHub أو بوصفها AppImage، والبرامج من ملفات .deb، والإضافات، تظهر هنا.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = لا شيء مثبّت يطابق «{ $find }».

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = الإزالة تسأل مرتين: اضغط عليها مرة أخرى خلال ثلاث ثوانٍ.

# On an installed thing's line: its version.
store-installed-version = الإصدار { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = محجوز

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = محجوز؛ { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = جارٍ تحديث { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = حدّث { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = حول

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = جارٍ الإزالة...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = لا يوجد ناقل جلسة لسؤاله عن الملف.

# Title of the window that asks which AppImage to add.
store-add-choose-title = اختر AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = تعذّر سؤال منتقي الملفات.

# Words in the Installed card while an AppImage is being added.
store-add-adding = جارٍ الإضافة...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } موجود بالفعل: انظر المثبّتة.

# Words once an AppImage is in the ring.
store-add-in-ring = تمت الإضافة. أصبح الآن في قائمتك.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = هذا ليس مرجع Flatpak يستطيع هذا الحاسوب استخدامه.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = جارٍ تثبيت { $name } من { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = تم تثبيت { $name }.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = رفض Flatpak تثبيته.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = تعذّر تثبيت { $name }. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = جارٍ الاستبدال...

# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } موجود بالفعل

# Small heading over the AppImage being brought.
store-replace-new = الجديد الذي يُجلب

# Small heading over the AppImage already there.
store-replace-old = الموجود بالفعل

# Tip of the round button that keeps the AppImage already there.
store-replace-skip = تخطَّ: أبقِ الموجود بالفعل

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = استبدال

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = لم ينتهِ، ولم يذكر سببًا.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = اعرض موضع المشكلة

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = ابحث في GitHub: اكتب ثم اضغط Enter

# Heading of the card of matches from AppImageHub.
store-gh-hub = على AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = اضغط Enter للبحث في GitHub كله أيضًا.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = لم يُسأل GitHub: لا يوجد إنترنت لسؤاله عبره.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = جارٍ سؤال GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = لا شيء آخر على GitHub.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [zero] لا مشاريع للبحث فيها عن ملفات AppImage وFlatpak...
        [one] جارٍ البحث عن ملفات AppImage وFlatpak في مشروع واحد...
        [two] جارٍ البحث عن ملفات AppImage وFlatpak في مشروعين...
        [few] جارٍ البحث عن ملفات AppImage وFlatpak في { $n } مشاريع...
        [many] جارٍ البحث عن ملفات AppImage وFlatpak في { $n } مشروعًا...
       *[other] جارٍ البحث عن ملفات AppImage وFlatpak في { $n } مشروع...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = لا تحتوي أي من النتائج على GitHub على ملف AppImage أو Flatpak يناسب هذا الحاسوب.

# Heading of the card of matches from GitHub.
store-gh-on = على GitHub

# Heading of the card that holds the search box.
store-gh-search-card = ابحث في GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = تطبيقات على GitHub بوصفها ملفات AppImage أو Flatpak. لا أحد يفحصها: ثبّت فقط من أشخاص تثق بهم.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = لا يمكن البحث في GitHub ما دام هذا الحاسوب غير متصل بالإنترنت.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak وAppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = اعرض الملفات

# Button that installs a GitHub project or file.
store-gh-install = تثبيت

# Link to a project's page on GitHub.
store-gh-open = افتح على GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }، { $size }، الإصدار { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = مفتاح GitHub

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = مفتاح GitHub: يوجد مفتاح محفوظ

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = من شبه المؤكد أنك لا تحتاج إلى واحد. بدون مفتاح يجيب GitHub عن 60 سؤالًا في الساعة من هذا الحاسوب؛ ومعه يجيب عن 5,000. المفتاح يرفع هذا الحد فقط. ويُستخدم للاطلاع على الملفات العامة وليس لشيء غيره: فهو لا يعطي Trindle أي طريق إلى حسابك، ولا تحدد أي أذونات عند إنشائه. يُحفظ على هذا الحاسوب، في سلسلة مفاتيحك، ولا يُرسل إلا إلى GitHub.

# In the GitHub key's box: how to make a key.
store-gh-key-how = أنشئ مفتاحًا على GitHub، وأعطه أي اسم، واترك كل الأذونات كما هي، واضغط إنشاء الرمز، ثم الصق هنا ما يعرضه.

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = تعذّرت إزالته: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = تعذّر حفظه: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = اتركه فارغًا للإبقاء عليه

# Placeholder of the GitHub key's box.
store-gh-key-paste = الصق المفتاح

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = أزِل المفتاح

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = أنشئ مفتاحًا على GitHub

# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = احفظ المفتاح

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = لا شيء في أحدث إصدار منه يُثبَّت هنا.

# Word on the file's dial and under a GitHub file when it was added.
store-added = أُضيف

# Line under the title of the Move your apps page.
store-bk-line = تطبيقاتك وإعداداتك، في ملف واحد تحمله إلى أي مكان.

# Heading of the card that makes a backup.
store-bk-card = نسخ احتياطي

# Tip of the round button while the backup is being made.
store-bk-making-tip = جارٍ إنشاء النسخة الاحتياطية

# Main button that makes the backup.
store-bk-make = أنشئ النسخة الاحتياطية

# Tip of the round button that shows the backup in Folders.
store-bk-show = اعرضها في المجلدات

# Name of the switch for the list of apps in a backup.
store-bk-apps = التطبيقات

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [zero] قائمة بلا تطبيقات Flatpak.
        [one] قائمة تطبيق Flatpak الواحد لديك.
        [two] قائمة تطبيقَي Flatpak لديك.
        [few] قائمة تطبيقات Flatpak الـ { $n } لديك.
        [many] قائمة تطبيقات Flatpak الـ { $n } لديك.
       *[other] قائمة تطبيقات Flatpak الـ { $n } لديك.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } يثبّتها الحاسوب الآخر جديدة من Flathub، فيبقى الملف صغيرًا.

# Name of the switch for the apps' own data in a backup.
store-bk-data = بيانات التطبيقات نفسها

# Under the Apps' own data switch.
store-bk-data-sub = إعدادات كل تطبيق وعمله المحفوظ، دون ملفات التخزين المؤقت.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = تُحفظ كاملة، لأنها ملفات مفردة.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = إعدادات Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = الحلقة، واختيار الخلفية، والألوان، ولوحة المفاتيح، والطاقة، والتطبيقات الافتراضية، وصورتك.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } الحجم الآن { $size }.

# Name of the row that says where the backup goes.
store-bk-where = أين تذهب

# Under that row: the folder and the file's name.
store-bk-where-sub = مجلدك الشخصي، باسم { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = استرجع نسخة احتياطية

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = اختر ملف نسخة احتياطية

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = نسخة Trindle الاحتياطية، أو نسخة أنشأها { $flat }.

# Note in the card that puts a backup back.
store-bk-back-note = تُثبَّت تطبيقاتها وتُسترجع بياناتها وإعداداتها. مستنداتك وصورك ليست فيها: انسخها بنفسك.

# Said while a backup is being made.
store-bk-making = جارٍ النسخ الاحتياطي... قد يستغرق هذا بضع دقائق.

# Said when a backup is made: the file and its size.
store-bk-made = هي { $path } ({ $size }). انسخها إلى فلاشة USB أو تخزين سحابي لتأخذها معك.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = لم تُختر نسخة احتياطية، لأنه ليس لدى Trindle اتصال بمنتقي الملفات.

# Title of the window that asks which backup to use.
store-bk-choose-title = اختر نسخة احتياطية

# Name of the filter in the window that asks which backup to use.
store-bk-filter = نسخ Trindle وFlat الاحتياطية

# Red words when the file chooser gave no answer.
store-bk-no-answer = لم تُختر نسخة احتياطية، لأن منتقي الملفات لم يرد.

# Said while a backup file is read.
store-bk-reading = جارٍ قراءة النسخة الاحتياطية...

# Said when a backup was put back.
store-bk-logout = سجّل الخروج ثم الدخول لترى كل إعداد.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } سجّل الخروج ثم الدخول لترى كل إعداد.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [zero] لا تطبيقات. جارٍ البدء...
        [one] تطبيق واحد، { $with } منها بإعداداتها. جارٍ البدء...
        [two] تطبيقان، { $with } منها بإعداداتها. جارٍ البدء...
        [few] { $apps } تطبيقات، { $with } منها بإعداداتها. جارٍ البدء...
        [many] { $apps } تطبيقًا، { $with } منها بإعداداتها. جارٍ البدء...
       *[other] { $apps } تطبيق، { $with } منها بإعداداتها. جارٍ البدء...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] لم تُضف مصادر خاصة بالمصنّعين بعد.
        [one] أُضيف مصدر مصنّع واحد، ويبقى محدَّثًا مع تحديثات Debian.
        [two] أُضيف مصدرا مصنّعين، ويبقيان محدَّثين مع تحديثات Debian.
        [few] أُضيفت { $n } مصادر مصنّعين، وتبقى محدَّثة مع تحديثات Debian.
        [many] أُضيف { $n } مصدر مصنّعين، ويبقى محدَّثًا مع تحديثات Debian.
        [zero] أُضيف { $n } مصدر مصنّعين، ويبقى محدَّثًا مع تحديثات Debian.
       *[other] أُضيف { $n } مصدر مصنّعين، ويبقى محدَّثًا مع تحديثات Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. أُضيف، ويبقى محدَّثًا.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. أُضيف المصدر، لكن البرنامج غير مثبّت.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. مثبّت بطريقة أخرى.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = أزِله مع مصدره

# Link to a maker's page, naming the program.
store-src-open = افتح { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = سجّل الدخول إلى Tailscale

# Tip of the round button that adds a maker's source, naming the program.
store-src-add = أضف { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = جاهز للإضافة

# Name of the field for the address of a source.
store-src-address = العنوان

# Under the address field.
store-src-address-sub = عنوان المستودع، من تعليمات التثبيت لدى مصنّعه.

# Name of the field for the suite of a source.
store-src-suite = الإصدار (suite)

# Under the suite field.
store-src-suite-sub = عادةً trixie أو stable.

# Name of the field for the components of a source.
store-src-components = المكوّنات

# Under the components field.
store-src-components-sub = عادةً main.

# Name of the field for a source's signing key.
store-src-key = مفتاح التوقيع

# Under the signing key field.
store-src-key-sub = عنوان مفتاح توقيعه.

# Name of the field for the program to install from a source.
store-src-program = البرنامج المراد تثبيته

# Under the program field.
store-src-program-sub = اسم حزمته، إذا أردت تثبيته الآن.

# Placeholder of the program field.
store-src-program-hint = اختياري

# Tip of the round button that adds the source typed.
store-src-add-this = أضف هذا المصدر

# Heading of the card that adds another source.
store-src-another = أضف مصدرًا آخر

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = يستطيع المصدر تثبيت تحديثات للحاسوب كله، لذلك أضف فقط مصنّعين تثق بهم. يطلب كل مصدر كلمة مرور المدير.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = أزِل هذا المصدر

# Heading of the card of sources already on the computer.
store-src-already = موجودة بالفعل على هذا الحاسوب

# Red words when the addresses of a new source are not secure.
store-src-bad-address = يجب أن يبدأ العنوانان بـ https://

# Red words when the suite or components are not plain words.
store-src-bad-words = يجب أن يكون الإصدار والمكوّنات كلمات عادية.

# Red words when the program is not a package name.
store-src-bad-package = هذا ليس اسم حزمة.

# Word on a source's dial while its job starts.
store-src-starting = جارٍ البدء

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = جارٍ فتح صفحة تسجيل الدخول إلى Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = سجّل الدخول في الصفحة التي فُتحت؛ وينتهي هذا من تلقاء نفسه.

# Said under Tailscale's row when it is connected.
store-src-connected = متصل

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = الرابط المؤدي إلى التطبيقات غير مشفّر.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = غير مشفّر

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = هذا العنوان غير مشفّر. يستطيع شخص في الطريق أن يرى ما يُرسل، وأن يستبدل النسخة الاحتياطية بأخرى مختلفة.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = أفهم هذا الخطر وأقبله

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = استرجاع

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = إلغاء

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = هل تريد إزالة { $name }؟

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = تشغل بياناته المحفوظة { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = لا توجد له بيانات محفوظة.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = جارٍ حساب بياناته المحفوظة...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = لا يمكن استعادة البيانات المحذوفة.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = إزالة مع الإبقاء على بياناته

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = إزالة وحذف بياناته

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = حذف بياناته فقط

# Why Delete its data only is greyed.
store-flatpak-data-only-none = لا توجد له بيانات محفوظة لحذفها.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = إلغاء: الإبقاء على الاثنين

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = حُذفت بياناته المحفوظة.

# Words while only the data is being deleted.
store-flatpak-data-going = جارٍ حذف بياناته المحفوظة...

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = تعذّر حساب بياناته المحفوظة.

store-hub-warning = يكتفي AppImageHub بسرد المشاريع: لا أحد يراجعها أو يفحصها، وليس لازماً أن تكون موقّعة. يشغّل Trindle كل AppImage داخل صندوق له مجلد منزل خاص: في البداية قد يستخدم الإنترنت والصوت، ولا يرى أياً من ملفاتك.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = مصدر AppImage تلميح وليس فحصاً.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = على GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = على GitHub: { $repo }، { $stars } نجمة

# Where an AppImage comes from: a website, by its name.
store-from-site = ليس على GitHub: من موقع ويب، { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = المصدر غير معروف

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = افتح موقعه

# When an app was last updated.
store-age-updated = آخر تحديث: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = آخر تحديث: { $date }. لم يُحدَّث منذ أكثر من ستة أشهر، لذا قد تكون فيه ثغرات أمنية غير مصلحة.

# When the source does not say when an app was last updated.
store-age-updated-unknown = آخر تحديث: غير معروف

# When an app's project was made.
store-age-created = أُنشئ في { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = تاريخ الإنشاء: غير معروف

# A date the source does not give, on a line of Facts.
store-age-not-known = غير معروف

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = قبل أن تضيف واحداً

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [zero] لا يوجد أي AppImage في قائمة AppImageHub.
        [one] يوجد AppImage واحد في قائمة AppImageHub.
        [two] يوجد AppImage اثنان في قائمة AppImageHub.
        [few] يوجد { $count } AppImage في قائمة AppImageHub.
        [many] يوجد { $count } AppImage في قائمة AppImageHub.
       *[other] يوجد { $count } AppImage في قائمة AppImageHub.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = جارٍ جلب قائمة AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = تعذّر جلب قائمة AppImageHub.

# The same with no internet and no copy kept.
store-browse-offline = لا يمكن جلب قائمة AppImageHub ما دام هذا الحاسوب غير متصل بالإنترنت.

# In the list when the kind chosen has none.
store-browse-none = لا يسرد AppImageHub أياً من هذا النوع.

# Under the rows: how many are shown of how many.
store-browse-showing = عرض { $shown } من { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = أضف إلى تطبيقات Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = اعرض التفاصيل والملفات



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = البحث عن التحديثات متوقف

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = سيعود ChatGPT عندما توقّع OpenAI تنزيله.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } للاختيار من Flathub وAppImageHub؛ { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } للاختيار من AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = كلاهما

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = ملفات AppImage لا يجري فحصها. لا أحد يراجعها أو يفحصها.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = يشغّل Trindle كل ملف AppImage داخل صندوق له مجلد منزلي خاص: في البداية قد يستخدم الإنترنت والصوت، ولا يرى أيًّا من ملفاتك.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = لا تعرض هذا مرة أخرى

# Button on that card that goes on with the install.
store-appimage-warn-ok = حسنًا

# Button on that card that gives the install up.
store-appimage-warn-cancel = إلغاء

