# machine translated (Arabic), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = الأقراص

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = عن هذه الصفحة

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = كل قرص داخل هذا الحاسوب أو موصول به، مرسوم على هيئة شريط من الأقسام. عرض كل قسم يتناسب مع حجمه ويمتلئ بقدر ما استُخدم منه، فيسهل رؤية القسم الممتلئ.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = تحت الرسم، لكل قرص بطاقة. افتح قسمًا، أو ركّبه، أو أزل تركيبه، أو أخرج القرص بأمان أو أطفئه. يفتح القلم على القسم تغييراته: اسمه، ومجلد خاص به، وهل يكون جاهزًا عند بدء التشغيل، وحجمه، وفحصه وإصلاحه، وتهيئته أو حذفه. تصنع علامة الزائد في المساحة الحرة قسمًا جديدًا.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = القرص الذي يعمل منه النظام يظهر لكنه لا يُعرض عليه أي تغيير يمحو أو ينقل شيئًا. كل تغيير على أي قرص يطلب كلمة مرور المسؤول في كل مرة. تُخفي العين قرصًا أو قسمًا من المجلدات والحلقة: يبقى هنا باهتًا وليس مقفلًا.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = كل قسم من كل قرص، ممتلئ بقدر ما استُخدم منه.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = أقراصك

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = تعذّرت قراءة الأقراص الآن.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = حاول مرة أخرى

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = انظر مرة أخرى

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = لم يُعثر على أي قرص. صِل قرصًا وسيظهر هنا.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = خدمة الأقراص

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = ناقل النظام

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } غير متاح. ثبّته بالأمر: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = جارٍ النظر في الأقراص…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } يتعطل. انسخ منه الآن كل ما تريد الاحتفاظ به.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } يحتاج إلى نظرة: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] الأقراص والمساحة عليها.
        [one] الأقراص والمساحة عليها، وقرص موصول.
        [two] الأقراص والمساحة عليها، وقرصان موصولان.
        [few] الأقراص والمساحة عليها، و{ $plugged } أقراص موصولة.
        [many] الأقراص والمساحة عليها، و{ $plugged } قرصًا موصولًا.
        [zero] الأقراص والمساحة عليها، و{ $plugged } قرص موصول.
       *[other] الأقراص والمساحة عليها، و{ $plugged } قرص موصول.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
# SAFETY: needs a human review before it ships
disks-headline-full =
    { $plugged ->
        [0] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب. احذف بعض الملفات أو انقلها قريبًا.
        [one] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب، وقرص موصول. احذف بعض الملفات أو انقلها قريبًا.
        [two] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب، وقرصان موصولان. احذف بعض الملفات أو انقلها قريبًا.
        [few] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب، و{ $plugged } أقراص موصولة. احذف بعض الملفات أو انقلها قريبًا.
        [many] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب، و{ $plugged } قرصًا موصولًا. احذف بعض الملفات أو انقلها قريبًا.
        [zero] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب، و{ $plugged } قرص موصول. احذف بعض الملفات أو انقلها قريبًا.
       *[other] ممتلئ تقريبًا: لم يبقَ إلا { $free } حرة في هذا الحاسوب، و{ $plugged } قرص موصول. احذف بعض الملفات أو انقلها قريبًا.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] يمتلئ: { $free } حرة في هذا الحاسوب.
        [one] يمتلئ: { $free } حرة في هذا الحاسوب، وقرص موصول.
        [two] يمتلئ: { $free } حرة في هذا الحاسوب، وقرصان موصولان.
        [few] يمتلئ: { $free } حرة في هذا الحاسوب، و{ $plugged } أقراص موصولة.
        [many] يمتلئ: { $free } حرة في هذا الحاسوب، و{ $plugged } قرصًا موصولًا.
        [zero] يمتلئ: { $free } حرة في هذا الحاسوب، و{ $plugged } قرص موصول.
       *[other] يمتلئ: { $free } حرة في هذا الحاسوب، و{ $plugged } قرص موصول.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] مساحة واسعة: { $free } حرة في هذا الحاسوب.
        [one] مساحة واسعة: { $free } حرة في هذا الحاسوب، وقرص موصول.
        [two] مساحة واسعة: { $free } حرة في هذا الحاسوب، وقرصان موصولان.
        [few] مساحة واسعة: { $free } حرة في هذا الحاسوب، و{ $plugged } أقراص موصولة.
        [many] مساحة واسعة: { $free } حرة في هذا الحاسوب، و{ $plugged } قرصًا موصولًا.
        [zero] مساحة واسعة: { $free } حرة في هذا الحاسوب، و{ $plugged } قرص موصول.
       *[other] مساحة واسعة: { $free } حرة في هذا الحاسوب، و{ $plugged } قرص موصول.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = هذا الحاسوب

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = ملفاتك

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] لا تزال { $free } حرة، حيث النظام.
       *[files] لا تزال { $free } حرة، لملفاتك الخاصة.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: ممتلئ { $full }، حرّ { $free } من { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = من { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = لا قراءات، فالأقراص لا تذكر حالتها

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = سليم، كما يقول القرص نفسه

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [two] سليمان، كما يقول القرصان
        [few] سليمة، كما تقول الأقراص الـ{ $n }
        [many] سليمة، كما تقول الأقراص الـ{ $n }
        [one] سليمة، كما تقول الأقراص الـ{ $n }
        [zero] سليمة، كما تقول الأقراص الـ{ $n }
       *[other] سليمة، كما تقول الأقراص الـ{ $n }
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] قرص واحد يحتاج إلى نظرة، انظر بطاقته
        [two] قرصان يحتاجان إلى نظرة، انظر بطاقتيهما
        [few] { $n } أقراص تحتاج إلى نظرة، انظر بطاقاتها
        [many] { $n } قرصًا يحتاج إلى نظرة، انظر بطاقاتها
        [zero] { $n } قرص يحتاج إلى نظرة، انظر بطاقاتها
       *[other] { $n } قرص يحتاج إلى نظرة، انظر بطاقاتها
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] قرص واحد، { $total } في المجموع. { $health }.
        [two] قرصان، { $total } في المجموع. { $health }.
        [few] { $n } أقراص، { $total } في المجموع. { $health }.
        [many] { $n } قرصًا، { $total } في المجموع. { $health }.
        [zero] { $n } قرص، { $total } في المجموع. { $health }.
       *[other] { $n } قرص، { $total } في المجموع. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } حرة

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = غير مركّب

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = مساحة حرة

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = لا قرص أو بطاقة فيه

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = لا شيء عليه بعد

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }، USB أو قابل للإزالة

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }، داخل هذا الحاسوب

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }، USB أو قابل للإزالة، مخفي

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }، داخل هذا الحاسوب، مخفي

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }، ممتلئ { $full }، { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }، { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }، { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = قسم

# Name given to a drive that gives no maker or model.
disks-drive-name = قرص

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }، { $size }

# Kind of a part that is swap space.
disks-kind-swap = تبديل

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = مشفّر، مفتوح

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = مشفّر، مقفل

# Kind of a part that holds other parts.
disks-kind-container = يضم أقسامًا أخرى

# Kind of a part that has no filesystem on it.
disks-kind-none = بلا نظام ملفات

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }، EFI، القسم الذي يبدأ منه الحاسوب
        [legacy] { $kind }، القسم الذي يبدأ منه حاسوب أقدم
       *[loader] { $kind }، قسم خاص بمحمّل الإقلاع
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = أي حاسوب

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = لينكس فقط

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = ويندوز

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = الأجهزة القديمة

# Trouble a drive reports about its spare space.
disks-trouble-spare = تنفد المساحة الاحتياطية

# Trouble a drive reports about its heat.
disks-trouble-temperature = شديد الحرارة

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = يبلى

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = جعل نفسه للقراءة فقط

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = تعطلت النسخة الاحتياطية لذاكرته

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = جعل جزء منه نفسه للقراءة فقط

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = بلي أكثر مما يتوقعه صانعه

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] موضع تالف واحد
        [two] موضعان تالفان
        [few] { $n } مواضع تالفة
        [many] { $n } موضعًا تالفًا
        [zero] { $n } موضع تالف
       *[other] { $n } موضع تالف
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °م

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] استُخدم ساعة واحدة
        [two] استُخدم ساعتين
        [few] استُخدم { $hours } ساعات
        [many] استُخدم { $hours } ساعة
        [zero] استُخدم { $hours } ساعة
       *[other] استُخدم { $hours } ساعة
    }

# How a drive says it is when it is failing, on its card.
# SAFETY: needs a human review before it ships
disks-health-failing = يتعطل: انسخ منه الآن كل ما تريد الاحتفاظ به

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = يحتاج إلى عناية: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = سليم

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }، { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = لا قرص يستجيب

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = الغلاف يستجيب لكن القرص الذي بداخله لا يستجيب. افصله، وانتظر عشر ثوانٍ، ثم صِله بمنفذ أزرق أو USB-C بكابله الخاص. إن بقي على حاله فقد يكون القرص الداخلي انفك أو تعطل.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = إزالة آمنة

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = أطفئه

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = امسح القرص كله

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] أظهر هذا القرص
       *[part] أظهر هذا القسم
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] أخفِ هذا القرص. { $lock }
       *[part] أخفِ هذا القسم. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = مخفي من المجلدات والحلقة. { $lock }

# Note on the card of the drive the system runs from.
disks-system-note = النظام يعمل منه: لا يمكن مسح أي شيء هنا.

# Note on the card of a drive that has no partition table.
disks-no-table = لا جدول أقسام: القرص كله حر.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = غير مركّب

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = يُركَّب عند بدء التشغيل في { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = يظهر في { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = في { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = افتح

# Tip of the round button that mounts a part.
disks-mount = ركّب

# Tip of the round button that unmounts a part.
disks-unmount = أزل التركيب

# Tip of the round button on a part that opens the box of changes.
disks-change-it = غيّره

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = الاسم

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = الاسم (اختياري)

# Tip of the button that saves a part's new name.
disks-rename-tip = غيّر اسمه

# Note under the Name row of a part.
disks-name-note = ما يُسمّى به هنا وعلى الحواسيب الأخرى.

# Placeholder of the typing box for a part's folder.
disks-folder-name = اسم المجلد

# Tip of the button that saves a part's folder.
disks-use-folder = استخدم هذا المجلد

# Label of the row where a part's folder is chosen.
disks-folder = المجلد

# Note under the Folder row of a part.
disks-folder-note = حيث تظهر ملفاته. اتركه فارغًا ليختار الحاسوب.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = التركيب عند بدء التشغيل

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = جاهز كلما بدأ الحاسوب. إن لم يكن موجودًا يبدأ الحاسوب على أي حال.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = الحجم بالجيجابايت

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = من { $least } إلى { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = من { $least } إلى { $most }. ركّبه لترى إلى أي حد يمكن تصغيره.

# Tip of the button that resizes a part.
disks-resize-tip = غيّر حجمه

# Label of the row where a part is resized.
disks-size = الحجم

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = سيُمسح كل ما على { $words }. لا يمكن التراجع عن ذلك.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = انقر مرة أخرى: سيتغير حجم { $name } على { $drive } من { $from } إلى { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = أفسح مكانًا لقسم جديد

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = المكان بالجيجابايت

# Tip of the button that makes room under a part that is in use.
disks-room-tip = أفسح مكانًا

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = انقر مرة أخرى: سيفقد { $name } على { $drive } حجم { $size }، بعد لقطة

# Note under the row that makes room.
disks-room-note = يُصغَّر أثناء الاستخدام، بعد لقطة، ويترك مساحة حرة.

# Tip of the button that checks a part for faults.
disks-check-tip = افحصه بحثًا عن أخطاء

# Tip of the button that repairs a part.
disks-repair-tip = أصلحه

# Label of the row where a part is checked and repaired.
disks-faults = الأخطاء

# Note under the Faults row.
disks-faults-note = يبحث عن أخطاء في طريقة حفظ ملفاته ويصلحها. يُزال تركيبه أثناء ذلك.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = انقر مرة أخرى لإصلاح { $name } على { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = النوع

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = تهيئة

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = هيّئه

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = يبدؤه من جديد كقسم واحد يملأ القرص.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = يمسح كل ما على هذا القسم وحده.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = انقر مرة أخرى: سيُمسح كل ما على { $words }

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }، { $size }، على { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = احذف { $name }، { $size }، على { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = احذف هذا القسم

# Note under the row that deletes a part.
disks-delete-note = تصبح مساحته حرة، جاهزة لقسم جديد أو ليتمدد فيها القسم الذي قبله.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = اصنع قسمًا جديدًا هنا

# Label of the row where a new part is made.
disks-new-part = قسم جديد

# Note under the row where a new part is made.
disks-new-part-note = يُصنع في هذه المساحة الحرة ويُهيَّأ جاهزًا للاستخدام.

# Tip of the button that makes a new part.
disks-make-tip = اصنعه

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = انقر مرة أخرى لصنع قسم حجمه { $size } على { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = امسح { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }، على { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = قسم جديد

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } حرة، على { $drive }

# Tip of the round button that closes the box of changes.
disks-close = أغلق

# Words on a row while a part is being mounted.
disks-mounting = جارٍ التركيب

# Words on a row when a part has been mounted.
disks-mounted = رُكّب

# Words on a row while a part is being unmounted.
disks-unmounting = جارٍ إزالة التركيب

# Words on a row when a part has been unmounted.
disks-unmounted = أُزيل التركيب

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = جارٍ إغلاقه

# Words on a drive's row when it is safe to remove.
disks-safe-remove = آمن للإزالة

# Words on a drive's row while it is being powered off.
disks-powering-off = جارٍ الإطفاء

# Words on a drive's row when it has been powered off.
disks-off-safe = مطفأ: آمن للفصل

# Words on a row while a part is being renamed.
disks-renaming = جارٍ تغيير الاسم

# Words on a row when a part has been renamed.
disks-renamed = تغيّر الاسم

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = سيختار الحاسوب مجلده من جديد

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = يظهر في { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = يُركَّب عند بدء التشغيل في { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = متروك عند بدء التشغيل

# Words on a row while a folder or start-up choice is being saved.
disks-saving = جارٍ الحفظ

# Words on a row while a part is being resized.
disks-resizing = جارٍ تغيير الحجم

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = تغيّر الحجم إلى { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = تعذّر العثور على الملفات، فلا يمكن إفساح مكان.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = جارٍ أخذ لقطة

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = تعذّر العثور على اللقطة بعد ذلك.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } لم يتغير أي حجم.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = تغيرت الأمور أثناء أخذ اللقطة، فلم يتغير أي حجم.

# Words on a row while room is being made.
disks-making-room = جارٍ إفساح المكان

# Words on a row when room has been made. { $size } is how much.
disks-room-made = أُفسح مكان بحجم { $size }

# Words on a row while a part is being repaired.
disks-repairing = جارٍ الإصلاح

# Words on a row when a part has been repaired.
disks-repaired = تم الإصلاح

# Red words when some faults could not be put right.
disks-repair-failed = تعذّر إصلاح بعض الأخطاء. انسخ كل ما تريد الاحتفاظ به.

# Words on a row while a part is being checked.
disks-checking = جارٍ الفحص

# Words on a row when the check found no faults.
disks-no-faults = لم يُعثر على أخطاء

# Red words when the check found faults.
disks-faults-found = عُثر على أخطاء: اضغط على المفتاح الإنجليزي لإصلاحها.

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = جارٍ التهيئة

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = تمت التهيئة

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = مسح القرص كله

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = تعذّر العثور على القسم الجديد.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = تعذّر الوصول إلى خدمة الأقراص.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = تعذّرت قراءة معرّف القسم الجديد.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = جارٍ الحذف

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = تم الحذف

# Words on a row while a new part is being made.
disks-making-it = جارٍ صنعه

# Words on a row when a new part has been made.
disks-made = تم صنعه

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = تعذّر حفظ ذلك: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = أعطه اسمًا أولًا.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] هذا الاسم طويل جدًا: حرف واحد على الأكثر.
        [two] هذا الاسم طويل جدًا: حرفان على الأكثر.
        [few] هذا الاسم طويل جدًا: { $max } أحرف على الأكثر.
        [many] هذا الاسم طويل جدًا: { $max } حرفًا على الأكثر.
        [zero] هذا الاسم طويل جدًا: { $max } حرف على الأكثر.
       *[other] هذا الاسم طويل جدًا: { $max } حرف على الأكثر.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = استخدم الحروف والأرقام و- و_ فقط (بلا مسافات).

# Red words under the folder name box when it starts with a dash.
disks-name-start = ابدأه بحرف أو رقم.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } مستخدم بالفعل.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] هذا الاسم طويل جدًا لهذا النوع: حرف واحد على الأكثر.
        [two] هذا الاسم طويل جدًا لهذا النوع: حرفان على الأكثر.
        [few] هذا الاسم طويل جدًا لهذا النوع: { $max } أحرف على الأكثر.
        [many] هذا الاسم طويل جدًا لهذا النوع: { $max } حرفًا على الأكثر.
        [zero] هذا الاسم طويل جدًا لهذا النوع: { $max } حرف على الأكثر.
       *[other] هذا الاسم طويل جدًا لهذا النوع: { $max } حرف على الأكثر.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = لا تقبل أسماء FAT هذه الرموز: * ? . , ; : / \ | + = < > [ ] "

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] ركّبه أولًا: لا يمكن تكبير هذا النوع إلا وهو مركّب.
       *[smaller] ركّبه أولًا: لا يمكن تصغير هذا النوع إلا وهو مركّب.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] لا يمكن تكبير هذا النوع.
       *[smaller] لا يمكن تصغير هذا النوع.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = لم تُعطَ كلمة مرور، فلم يتغير شيء.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = لم تُعطَ كلمة مرور، فتوقف العمل في منتصفه.

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = تغيير آخر لا يزال جاريًا. حاول مرة أخرى عند انتهائه.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = تعذّر التأكد من أن كلمة المرور ستُطلب، فلم يتغير شيء.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = تعذّر سحب كلمة المرور التي أُعطيت للتو، فقد لا يطلب تغيير آخر على قرص كلمة مرور لبضع دقائق. تجري المحاولة مرة أخرى.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = سُحبت كلمة المرور التي أُعطيت الآن: سيطلب التغيير التالي كلمة مرور.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = تعذّر الوصول إلى خدمة الأقراص، فلم يتغير شيء.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = أُغلق برنامج الأقراص أثناء إجراء هذا التغيير.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = يبقى برنامج الأقراص مفتوحًا حتى ينتهي هذا، ليقول كيف انتهى.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }، فلم يتغير شيء.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }، فتوقف العمل في منتصفه.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = تعذّر سؤال خدمة الأقراص

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } توقف العمل في منتصفه.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = تم: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = فشل: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = لم يتم: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = غير معروف: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = أُزيل تركيب { $dev } من أجل هذا وهو غير مركّب الآن.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = أُزيل تركيب { $dev } من أجل هذا وما زال غير مركّب: ركّبه من صفه.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = ربما أُزيل تركيب { $dev }: انظر إلى صفه.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = أُزيل تركيب { $dev } من أجل هذا، ويحتاج تركيبه مرة أخرى إلى كلمة مرور خاصة به: ركّبه من صفه.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = أُزيل تركيب { $dev } من أجل هذا وتعذّر تركيبه مرة أخرى ({ $why }): ركّبه من صفه.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = تمت التهيئة، لكنه لم يُعطَ مجلده { $dir } من جديد. { $why } اختر المجلد مرة أخرى من صفه.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = ما زال شيء يستخدمه. أغلق النوافذ أو البرامج التي تعرض ملفاته، ثم حاول مرة أخرى.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = لا توجد مساحة كافية لما عليه بهذا الحجم.

# Step of a change. { $dev } is the part.
disks-step-unmount = إزالة تركيب { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = تركيب { $dev } مرة أخرى

# Step of a change.
disks-step-eject = إغلاق القرص لإزالته

# Step of a change.
disks-step-power-off = إطفاء القرص

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = تهيئة { $dev }

# Step of a change.
disks-step-create = صنع القسم الجديد

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = حذف { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = تغيير حجم القسم { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = تغيير حجم ما على { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = إصلاح { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = فحص { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = تغيير اسم { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = حفظ المجلد واختيار بدء التشغيل لـ{ $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = الخطوة { $method } على { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = هذا القسم من نوع { $kind } لا يمكن تصغيره إلا إذا لم يكن قيد الاستخدام، فلا يمكن إفساح مكان منه هنا.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = لا يمكن تصغير قسم xfs هذا.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = هذا القسم من نوع { $kind } لا يمكن تصغيره وهو قيد الاستخدام.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = لا يمكن أخذ لقطة إلا من قرص النظام نفسه، فلا يمكن إفساح مكان هنا.

# Red words under the Make room row.
disks-room-not-in-table = ليس جزءًا من جدول القرص، فلا يمكن إفساح مكان منه.

# Red words under the Make room row.
disks-room-no-online-shrink = لا تستطيع خدمة الأقراص تصغير Btrfs وهو قيد الاستخدام هنا، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-holds-parts = يضم أقسامًا أخرى، فلا يمكن إفساح مكان منه.

# Red words under the Make room row.
disks-room-not-gpt = قرصه ليس فيه جدول GPT، فلا يمكن إفساح مكان منه.

# Red words under the Make room row.
disks-room-mount-first = ركّبه أولًا: يُصغَّر Btrfs وهو مركّب.

# Red words under the Make room row.
disks-room-mounts-not-one = ليس مركّبًا في مكان واحد بالضبط، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-read-only = مركّب للقراءة فقط، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-read-only-unknown = تعذّر معرفة هل هو للقراءة فقط، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-many-drives = الملفات موزعة على أكثر من قرص، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-drives-unknown = تعذّر معرفة عدد الأقراص التي توزعت عليها الملفات، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-swap-file = يوجد عليه ملف تبديل، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-swap-unknown = تعذّر معرفة هل يوجد عليه ملف تبديل، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-used-unknown = تعذّرت قراءة المقدار المستخدم منه، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-area-unknown = لا تذكر خدمة الأقراص حجم منطقة الملفات، فتعذّر التحقق ولا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-open-missing = تعذّر العثور على نسخته المفتوحة، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-encryption-sizes = الأحجام المعطاة للتشفير لا تتفق، فلا يمكن إفساح مكان.

# Red words under the Make room row.
disks-room-sizes = الأحجام المعطاة له لا تتفق، فلا يمكن إفساح مكان.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = يحتوي على { $used } ويحتاج إلى بقاء أكثر من ذلك، فالمقدار الذي يمكن تحريره قليل جدًا.

# Red words under the Make room row.
disks-room-sizes-unread = تعذّرت قراءة أحجامه.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = يمكن إفساح مكان من { $least } إلى { $most }.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }، فأُوقف العمل.

# What was found wrong while making room.
disks-room-check-unmounted = الملفات لم تعد مركّبة

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = حجم القسم { $is } وينبغي أن يكون { $should }

# What was found wrong while making room.
disks-room-check-open = النسخة المفتوحة من التشفير ليست بالحجم الذي ينبغي

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = حجم منطقة الملفات { $is } وينبغي أن يكون { $should }

# What was found wrong while making room.
disks-room-check-area-small = منطقة الملفات أصغر من المستخدم

# What was found wrong while making room.
disks-room-check-fit = ما يحتوي على غيره لم يعد يتسع له

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] لم تعد خدمة الأقراص تعرض القسم، فأُوقف العمل.
        [copy] لم تعد خدمة الأقراص تعرض النسخة المفتوحة، فأُوقف العمل.
       *[files] لم تعد خدمة الأقراص تعرض نظام الملفات، فأُوقف العمل.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = تصغير منطقة الملفات

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = تصغير النسخة المفتوحة من التشفير

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = تصغير القسم

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = فحص الأحجام

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = قبل إفساح مكان تحت { $dev } ({ $at })

# Red words when another change is pressed while one is still being made.
disks-change-going = جارٍ إجراء تغيير.


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = انقر مرة أخرى: سيتغير حجم { $name } على { $drive } من { $from } إلى { $to }. انسخ ملفاته احتياطيًا أولًا، فقد يؤدي انقطاع الكهرباء أثناء التصغير إلى فقدانها.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = نظام Windows على { $name } كان في السبات أو تُرك في البدء السريع، لذلك قد يؤدي تصغيره إلى فقدان ملفاته. شغّل Windows، واختر إعادة التشغيل، ثم حاول مرة أخرى.

