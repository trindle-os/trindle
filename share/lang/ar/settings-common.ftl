# machine translated (Arabic), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = تم الحفظ

# Said beside a button that worked, when nothing more is known.
settings-common-done = تم

# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = تم الحفظ|تم|قيد الاستخدام الآن|تغيّر|عُيّن|أُخذ|أُضيف|تحرر|أُزيل|خُزّن|ثُبّت|حُدّث|انضم|نُسخ|افتراضي الآن|شُغّل|أُرسل|متصل|جارٍ الاتصال|يعمل|متوقف|نُسي|يتوقف عند 80%|يشحن حتى الامتلاء|يبدأ عند الدخول|محفوظ لكن لا يبدأ|محدّث

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = لم يتم

# Tip of the information button at the top of every Settings page.
settings-common-about-page = عن هذه الصفحة

# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = عن { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } موجود بالفعل

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] افعل الشيء نفسه مع الباقي (واحد آخر)
        [two] افعل الشيء نفسه مع الباقي (اثنان آخران)
        [few] افعل الشيء نفسه مع الباقي ({ $n } أخرى)
        [many] افعل الشيء نفسه مع الباقي ({ $n } أخرى)
        [zero] افعل الشيء نفسه مع الباقي ({ $n } أخرى)
       *[other] افعل الشيء نفسه مع الباقي ({ $n } أخرى)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = افعل الشيء نفسه مع الباقي

# Tip of the button that keeps the one already there.
settings-common-skip = تخطَّ وأبقِ الموجود حاليًا

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = استبدل

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = استبدل الموجود بالجديد

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = الجديد

# Heading of the line that shows the one already there.
settings-common-already-line = الموجود بالفعل

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = استبدال الموجود بالفعل؟

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [zero] سيُلغى التغيير الآن.
        [one] سيُلغى التغيير خلال ثانية.
        [two] سيُلغى التغيير خلال ثانيتين.
        [few] سيُلغى التغيير خلال { $left } ثوانٍ.
        [many] سيُلغى التغيير خلال { $left } ثانية.
       *[other] سيُلغى التغيير خلال { $left } ثانية.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [zero] سيُلغى التغيير الآن. { $more }
        [one] سيُلغى التغيير خلال ثانية. { $more }
        [two] سيُلغى التغيير خلال ثانيتين. { $more }
        [few] سيُلغى التغيير خلال { $left } ثوانٍ. { $more }
        [many] سيُلغى التغيير خلال { $left } ثانية. { $more }
       *[other] سيُلغى التغيير خلال { $left } ثانية. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = تراجع الآن

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = أبقِ

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } غير متاح

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = لتثبيته، شغّل الأمر: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = تعذّر على Trindle فتح صفحة الويب لأنه لم يستطع تشغيل متصفح.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = تعذّر فتح { $name }.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = تعذّر الحفظ.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = تعذّر على Trindle تنفيذ ذلك لأن جزءًا من النظام لا يستجيب. أعد تشغيل الحاسوب ثم حاول مرة أخرى.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = تعذّر على Trindle تنفيذ ذلك لأن جزءًا من جلسة دخولك لا يستجيب. اخرج ثم ادخل من جديد، ثم حاول مرة أخرى.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = تعذّر على Trindle فتح نافذة اختيار ملف أو مجلد. اخرج ثم ادخل من جديد، ثم حاول مرة أخرى.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = القرص ممتلئ.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = لا يستطيع Trindle الكتابة على القرص الآن.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = الجزء من النظام الذي يقوم بذلك لا يعمل.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = تأخر في الرد كثيرًا. حاول مرة أخرى.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = لم تُعطَ كلمة المرور.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = لا يستطيع هذا الحساب فعل ذلك.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = ينقص هذا الحاسوب شيء يحتاج إليه.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } و{ $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { "، " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = تُفتح الحلقة بنقرتين متتاليتين عند حافة الشاشة.

# Word drawn in the middle of that picture.
settings-common-ring-name = الحلقة

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = تعذّر على Trindle الوصول إلى أدوات التحكم في شحن الحاسوب المحمول: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = لم يقبل الحاسوب المحمول حد الشحن.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = لا يسمح هذا الحاسوب بإيقاف الشحن مبكرًا.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = تعذّر ضبط { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = تجربة Trindle في جهاز افتراضي

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = يعمل Trindle في { $machine }. يعمل جيدًا هناك، لكن الجهاز الافتراضي ليس سريعًا أو سلسًا تمامًا مثل حاسوب حقيقي. احكم على سرعته على عتاد حقيقي.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = يعمل Trindle في { $machine } دون رسوميات ثلاثية الأبعاد. يضطر المعالج إلى رسم كل ما ترسمه شريحة الرسوميات عادةً. ستبدو النوافذ ومقاطع الفيديو وصفحات الويب أبطأ منها على حاسوب حقيقي، لذلك يستغني Trindle عن حركاته. إن كان الحاسوب الذي يعمل عليه يستطيع ذلك، فعّل التسريع ثلاثي الأبعاد في إعدادات عرض الجهاز الافتراضي. فهذا يحدث فرقًا كبيرًا. لا ينطبق شيء من هذا إذا ثبّتّ Trindle على حاسوب حقيقي.

# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } أو { $last }
