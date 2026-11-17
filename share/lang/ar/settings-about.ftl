# machine translated (Arabic), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = عن هذا الحاسوب

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = اسم المضيف اسم الحاسوب اسم الشبكة حول النظام الإصدار ديبيان ترندل hostname debian trindle الذاكرة رام ram المعالج cpu القرص الرسوميات الشاشة الطراز النواة kernel الإصدار المتجدد النشاط البرامج العاملة مشغول مدير المهام مراقب النظام الأقراص usb المساحة

# Text behind the info button of the About page.
settings-about-help = تعرض هذه الصفحة اسم هذا الحاسوب كما تراه الحواسيب الأخرى على الشبكة، ومم يتكون، وكم يستغرق بدؤه. يعرض «النشاط» ما يفعله الحاسوب الآن والبرامج التي تستخدم الذاكرة والمعالج. وتعرض «الأقراص» أقراصك ومساحتها. يُفتح كل منهما في نافذة مستقلة. استخدم الأسهم في «مزيد من التفاصيل» لفتحهما.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = غير معروف

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }، خيط واحد
        [two] { $model }، خيطان
        [few] { $model }، { $cores } خيوط
        [many] { $model }، { $cores } خيطًا
        [zero] { $model }، { $cores } خيط
       *[other] { $model }، { $cores } خيط
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = رسوميات Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = رسوميات AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = رسوميات NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = رسوميات الجهاز الافتراضي

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = الرسوميات

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } بمعدل { $hz } هرتز

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } مستخدمة

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } حرة

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = من { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }، يعمل منذ { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }، يعمل منذ { $on }. القرص شبه ممتلئ.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } ثانية

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } دقيقة

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] دقيقة واحدة
        [two] دقيقتين
        [few] { $n } دقائق
        [many] { $n } دقيقة
        [zero] { $n } دقيقة
       *[other] { $n } دقيقة
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] ساعة واحدة
        [two] ساعتين
        [few] { $n } ساعات
        [many] { $n } ساعة
        [zero] { $n } ساعة
       *[other] { $n } ساعة
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] يوم واحد
        [two] يومين
        [few] { $n } أيام
        [many] { $n } يومًا
        [zero] { $n } يوم
       *[other] { $n } يوم
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = استخدم حروفًا وأرقامًا وشرطات فقط. لا تبدأ بشرطة ولا تنتهِ بها.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = بانتظار كلمة مرورك…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = تعذّرت إعادة تسمية الحاسوب.

# Words under the computer name box after renaming worked.
settings-about-name-done = ستستخدم البرامج المفتوحة بالفعل الاسم الجديد بعد تسجيل دخولك التالي.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = اسم الحاسوب

# Tip of the save button beside the computer name box.
settings-about-rename = أعد تسمية هذا الحاسوب

# Bold line of the computer name setting.
settings-about-name = اسم الحاسوب

# Heading of the card with the computer's name.
settings-about-name-group = الاسم

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = هذا هو اسم الحاسوب على الشبكة وعند موجّه الأوامر في الطرفية. استخدم حروفًا وأرقامًا وشرطات، ولا تبدأ بشرطة ولا تنتهِ بها. عليك إدخال كلمة مرورك لتغييره.

# Heading of the card about the computer's hardware.
settings-about-hardware = الحاسوب

# Text behind the info button of the hardware card.
settings-about-hardware-about = تعرض هذه الصفحة الصانع والطراز والمعالج والرسوميات. وتعرض أيضًا عدد الخيوط التي يستطيع المعالج تشغيلها معًا، وكل شاشة قيد الاستخدام بحجمها ومعدل تحديثها.

# Heading of the card about the software.
settings-about-software = البرمجيات

# Text behind the info button of the software card.
settings-about-software-about = Trindle سطح مكتب يعمل فوق Debian، الذي يتولى البرامج التي تحته. النواة هي الجزء الذي يتعامل مع العتاد. ومُركِّب Trindle الخاص يرسم سطح المكتب باستخدام Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = مُركِّب Trindle الخاص

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] تطبيق واحد
        [two] تطبيقان
        [few] { $n } تطبيقات
        [many] { $n } تطبيقًا
        [zero] { $n } تطبيق
       *[other] { $n } تطبيق
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = مزيد من التفاصيل

# Text behind the info button of the More detail card.
settings-about-more-about = يعرض «النشاط» ما يفعله الحاسوب الآن والبرامج التي تستخدم الذاكرة والمعالج. وتعرض «الأقراص» كل قرص ومدى امتلائه وسلامته، وأي مفاتيح USB.

# Bold line of the setting that opens Activity.
settings-about-activity = انظر ما الذي يعمل

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = النشاط

# Bold line of the setting that opens Drives.
settings-about-drives = الأقراص والمساحة

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = الأقراص

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = تعذّر على Trindle حفظ أوقات البدء في التنزيلات.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = التاريخ والوقت,الثواني

# File name of the saved start-up times, in Downloads.
settings-about-times-file = أوقات بدء تشغيل Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = في التنزيلات، باسم { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = احفظ

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = احفظ كل أوقات البدء في التنزيلات

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = احفظ وقت كل بدء

# Quiet line under that setting.
settings-about-times-keep-sub = يُحفظ في التنزيلات كجدول بيانات.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = كل عمود هو بدء واحد، والأحدث إلى اليمين.

# Heading of the card with the last ten start-up times.
settings-about-times = آخر عشر مرات بدء

# Text behind the info button of the start-up times card.
# SAFETY: needs a human review before it ships
settings-about-times-about = يقيس Trindle زمن كل بدء من زر التشغيل إلى شاشة الدخول. لا يُحسب الوقت الذي تقضيه في كتابة كلمة مرور القرص. الأحدث في الأعلى.

# Heading of the memory tile.
settings-about-memory = الذاكرة

# Heading of the disk tile.
settings-about-disk = القرص

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = تقارير المشكلات

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = تذهب التقارير إلى Buildmorphic، المجتمع الذي يساعد في بناء Trindle. يمكنك الانضمام ببريد إلكتروني فقط. تكسب نقاطًا ومستويات عن المشكلات التي تكون أول من يبلّغ عنها، حتى لو لم تستطع إصلاحها. يحتوي التقرير على قائمة محددة من التفاصيل فقط، ولا يُرسل شيء حتى تضغط إرسال. اسم المستخدم أو اسم GitHub اختياري، واسم المستخدم لا يكون أبدًا عنوان بريدك الإلكتروني.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = اعرض إرسال تقرير عند حدوث خطأ

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = اسم مستخدم Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = اسم GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = عن المجتمع

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [zero] متوقف. لم يُحذف أي تقرير منتظر.
        [one] متوقف. حُذف تقرير منتظر واحد.
        [two] متوقف. حُذف تقريران منتظران.
        [few] متوقف. حُذفت { $n } تقارير منتظرة.
        [many] متوقف. حُذف { $n } تقريرًا منتظرًا.
       *[other] متوقف. حُذف { $n } تقرير منتظر.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [zero] متوقف. حُذفت كل التقارير المنتظرة.
        [one] متوقف. تعذّر حذف تقرير منتظر واحد.
        [two] متوقف. تعذّر حذف تقريرين منتظرين.
        [few] متوقف. تعذّر حذف { $n } تقارير منتظرة.
        [many] متوقف. تعذّر حذف { $n } تقريرًا منتظرًا.
       *[other] متوقف. تعذّر حذف { $n } تقرير منتظر.
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = التحديثات

# Text behind the info button of the updates card.
settings-about-updates-about = يبحث Trindle عن إصدارات جديدة من التطبيقات والإضافات والبرامج الثابتة ومن نفسه بعد تسجيل دخولك بقليل، ثم كل بضع ساعات. إذا أوقفت هذا المفتاح، فلن يبحث Trindle إلا عندما تضغط «البحث عن تحديثات» في التطبيقات، أو زر التحديثات في الحلقة. تُثبَّت إصلاحات Debian الأمنية كل يوم ولها مفتاحها الخاص. إذا أوقفته، فلن تُثبَّت من تلقاء نفسها وتبقى الثغرات الأمنية المعروفة دون إصلاح حتى تحدّث. يتطلب تغيير أيٍّ من المفتاحين كلمة مرور المسؤول، حتى يستخدم الحاسوب كله اختيارك.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = البحث عن تحديثات

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = بانتظار كلمة مرورك…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = التحديثات الأمنية

# Warning line under the Security updates switch when it is off.
settings-about-security-off = لم تعد الإصلاحات الأمنية تُثبَّت من تلقاء نفسها.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = متوقف، لكن التقارير المنتظرة في إضافة Reporting لم تُحذف.
