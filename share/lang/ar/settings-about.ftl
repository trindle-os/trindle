# machine translated (Arabic), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = عن هذا الحاسوب

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = اسم المضيف اسم الحاسوب اسم الشبكة حول النظام الإصدار ديبيان ترندل hostname debian trindle الذاكرة رام ram المعالج cpu القرص الرسوميات الشاشة الطراز النواة kernel الإصدار المتجدد النشاط البرامج العاملة مشغول مدير المهام مراقب النظام الأقراص usb المساحة

# Text behind the info button of the About page.
settings-about-help = اسم هذا الحاسوب كما تراه الحواسيب الأخرى على الشبكة، ومم يتكون، وكم يستغرق بدؤه. ما يفعله الآن، والبرامج التي تستخدم الذاكرة والمعالج، في «النشاط»، والأقراص والمساحة عليها في «الأقراص»، وكل منهما نافذة مستقلة: تفتحهما الأسهم في «مزيد من التفاصيل».

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
settings-about-name-bad = حروف وأرقام وشرطات فقط، دون أن يبدأ بشرطة أو ينتهي بها.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = بانتظار كلمة المرور...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = تعذّرت إعادة تسمية الحاسوب.

# Words under the computer name box after renaming worked.
settings-about-name-done = البرامج المفتوحة بالفعل ترى الاسم الجديد بعد تسجيل الدخول التالي.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = اسم الحاسوب

# Tip of the save button beside the computer name box.
settings-about-rename = أعد تسمية هذا الحاسوب

# Bold line of the computer name setting.
settings-about-name = اسم الحاسوب

# Quiet line under the computer name setting.
settings-about-name-sub = حروف وأرقام وشرطات.

# Heading of the card with the computer's name.
settings-about-name-group = الاسم

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = كيف يظهر هذا الحاسوب على الشبكة، وعند موجّه الأوامر في الطرفية. حروف وأرقام وشرطات، دون أن يبدأ بشرطة أو ينتهي بها. تغييره يطلب كلمة مرورك.

# Heading of the card about the computer's hardware.
settings-about-hardware = الحاسوب

# Text behind the info button of the hardware card.
settings-about-hardware-about = الصانع والطراز، والمعالج وعدد الخيوط التي يشغّلها معًا، والرسوميات، وكل شاشة قيد الاستخدام بحجمها ومعدل تحديثها.

# Quiet line under the maker and model.
settings-about-hw-computer = الحاسوب

# Quiet line under the processor.
settings-about-hw-processor = المعالج

# Quiet line under the graphics.
settings-about-hw-graphics = يرسم الصور

# Quiet line under the screens.
settings-about-hw-screen = الشاشة

# Heading of the card about the software.
settings-about-software = البرمجيات

# Text behind the info button of the software card.
settings-about-software-about = Trindle سطح مكتب فوق Debian، الذي يتولى البرامج التي تحته. النواة هي الجزء الذي يتحدث مع العتاد، ومُركِّب Trindle الخاص يرسم سطح المكتب على Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = يعمل تحته

# Quiet line under the kernel version.
settings-about-sw-kernel = النواة

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = مُركِّب Trindle الخاص

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = يرسم سطح المكتب على Wayland

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

# Quiet line under the number of apps.
settings-about-sw-apps-sub = مثبّتة وجاهزة في الحلقة

# Heading of the card with the ways to Activity and Drives.
settings-about-more = مزيد من التفاصيل

# Text behind the info button of the More detail card.
settings-about-more-about = ما يفعله الآن، والبرامج التي تستخدم الذاكرة والمعالج، في «النشاط». كل قرص، ومدى امتلائه وسلامته، ومفاتيح USB، في «الأقراص».

# Bold line of the setting that opens Activity.
settings-about-activity = انظر ما الذي يعمل

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = البرامج التي تستخدم الذاكرة والمعالج.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = النشاط

# Bold line of the setting that opens Drives.
settings-about-drives = الأقراص والمساحة

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = كل قرص، ومدى امتلائه وسلامته.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = الأقراص

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = تعذّر حفظ أوقات البدء في التنزيلات.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = في التنزيلات، باسم { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = احفظ

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = احفظ كل أوقات البدء في التنزيلات

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = كل بدء، للاحتفاظ

# Quiet line under that setting.
settings-about-times-keep-sub = يُحفظ في التنزيلات كجدول بيانات.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = كل عمود هو بدء واحد، والأحدث إلى اليمين.

# Heading of the card with the last ten start-up times.
settings-about-times = آخر عشر مرات بدء

# Text behind the info button of the start-up times card.
# SAFETY: needs a human review before it ships
settings-about-times-about = يُحسب زمن كل بدء من زر التشغيل إلى شاشة الدخول، دون الوقت الذي يُقضى في كتابة كلمة مرور القرص. الأحدث في الأعلى.

# Heading of the memory tile.
settings-about-memory = الذاكرة

# Heading of the disk tile.
settings-about-disk = القرص


# Heading of the card with the switch for looking for updates.
settings-about-updates = التحديثات

# Text behind the info button of the updates card.
settings-about-updates-about = يبحث Trindle عن إصدارات جديدة من التطبيقات والإضافات والبرامج الثابتة ومن نفسه بعد تسجيل دخولك بقليل وكل بضع ساعات، وتُثبَّت إصلاحات Debian الأمنية كل يوم. عند الإيقاف لا يبحث إلا عندما تضغط «البحث عن تحديثات» في التطبيقات، أو زر التحديثات في الحلقة. لإصلاحات Debian الأمنية مفتاحها الخاص، وعند إيقافه لا تُثبَّت من تلقاء نفسها وتبقى الأعطال المعروفة مفتوحة حتى تحدّث. يطلب تغيير أيٍّ من المفتاحين كلمة مرور المسؤول، حتى ترى خدمات الحاسوب نفسه الاختيار أيضًا.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = البحث عن تحديثات

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = بانتظار كلمة المرور...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = متوقف، لا شيء يبحث ما لم تطلب.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = التحديثات الأمنية

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = تُثبَّت الإصلاحات الأمنية من تلقاء نفسها كل يوم.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = لم تعد الإصلاحات الأمنية تُثبَّت من تلقاء نفسها.

