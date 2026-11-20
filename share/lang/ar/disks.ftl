# machine translated (Arabic), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = الأقراص

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = عن هذه الصفحة

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = تعرض هذه الصفحة كل قرص داخل هذا الحاسوب أو موصول به، على شكل شريط مقسّم إلى أقسام. عرض كل قسم يدل على حجمه، وتظليله يدل على مقدار امتلائه، فيسهل رؤية القسم الممتلئ.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = تحت الرسم، لكل قرص بطاقة. يمكنك أن تفتح قسمًا، أو تركّبه، أو تزيل تركيبه، أو تُخرج القرص بأمان، أو تُطفئه. ويتيح لك القلم على القسم أن تغيّر اسمه، وتختار مجلده، وتركّبه عند بدء التشغيل، وتغيّر حجمه، وتفحصه، وتصلحه، وتهيّئه، أو تحذفه. وتصنع علامة الزائد في المساحة الحرة قسمًا جديدًا.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = يمكنك رؤية القرص الذي يعمل منه النظام، لكن لا يمكنك إجراء أي تغيير عليه يمحو أو ينقل شيئًا. كل تغيير على أي قرص يطلب كلمة مرور المسؤول في كل مرة. تُخفي العين قرصًا أو قسمًا من Folders والحلقة. يبقى هنا بلون باهت، وهو غير مقفل.

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
disks-missing = { $what } غير متاح. لتثبيته، شغّل الأمر: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = جارٍ النظر في الأقراص…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } يتعطل. انسخ الآن أي ملفات تريد الاحتفاظ بها إلى قرص آخر.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } يحتاج إلى نظرة: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] أقراصك والمساحة عليها.
        [one] أقراصك والمساحة عليها، مع قرص موصول.
        [two] أقراصك والمساحة عليها، مع قرصين موصولين.
        [few] أقراصك والمساحة عليها، مع { $plugged } أقراص موصولة.
        [many] أقراصك والمساحة عليها، مع { $plugged } قرصًا موصولًا.
        [zero] أقراصك والمساحة عليها، مع { $plugged } قرص موصول.
       *[other] أقراصك والمساحة عليها، مع { $plugged } قرص موصول.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
# SAFETY: needs a human review before it ships
disks-headline-full =
    { $plugged ->
        [0] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. احذف بعض الملفات أو انقلها قريبًا.
        [one] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. يوجد قرص موصول. احذف بعض الملفات أو انقلها قريبًا.
        [two] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. يوجد قرصان موصولان. احذف بعض الملفات أو انقلها قريبًا.
        [few] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. توجد { $plugged } أقراص موصولة. احذف بعض الملفات أو انقلها قريبًا.
        [many] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. يوجد { $plugged } قرصًا موصولًا. احذف بعض الملفات أو انقلها قريبًا.
        [zero] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. يوجد { $plugged } قرص موصول. احذف بعض الملفات أو انقلها قريبًا.
       *[other] هذا الحاسوب شبه ممتلئ، ولم يبقَ فيه إلا { $free } حرة. يوجد { $plugged } قرص موصول. احذف بعض الملفات أو انقلها قريبًا.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة.
        [one] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة. يوجد قرص موصول.
        [two] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة. يوجد قرصان موصولان.
        [few] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة. توجد { $plugged } أقراص موصولة.
        [many] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة. يوجد { $plugged } قرصًا موصولًا.
        [zero] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة. يوجد { $plugged } قرص موصول.
       *[other] بدأ هذا الحاسوب يمتلئ، وفيه { $free } حرة. يوجد { $plugged } قرص موصول.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة.
        [one] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة. يوجد قرص موصول.
        [two] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة. يوجد قرصان موصولان.
        [few] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة. توجد { $plugged } أقراص موصولة.
        [many] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة. يوجد { $plugged } قرصًا موصولًا.
        [zero] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة. يوجد { $plugged } قرص موصول.
       *[other] في هذا الحاسوب مساحة واسعة، وفيه { $free } حرة. يوجد { $plugged } قرص موصول.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = هذا الحاسوب

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = ملفاتك

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] لا تزال { $free } حرة على القرص الذي يعمل منه النظام.
       *[files] لا تزال { $free } حرة، لملفاتك الخاصة.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: ممتلئ { $full }، حرّ { $free } من { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = من { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = لم ترسل الأقراص أي قراءات عن حالتها بعد

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = سليم، بحسب فحص القرص نفسه

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [zero] لا توجد أقراص اجتازت فحصها
        [one] سليم، بحسب فحص القرص نفسه
        [two] اجتاز القرصان فحصهما
        [few] اجتازت الأقراص الـ { $n } فحوصها
        [many] اجتازت الأقراص الـ { $n } فحوصها
       *[other] اجتازت الأقراص الـ { $n } فحوصها
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] قرص واحد يحتاج إلى نظرة: انظر بطاقته
        [two] قرصان يحتاجان إلى نظرة: انظر بطاقتيهما
        [few] { $n } أقراص تحتاج إلى نظرة: انظر بطاقاتها
        [many] { $n } قرصًا يحتاج إلى نظرة: انظر بطاقاتها
        [zero] { $n } قرص يحتاج إلى نظرة: انظر بطاقاتها
       *[other] { $n } قرص يحتاج إلى نظرة: انظر بطاقاتها
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
disks-kind-container = يضم أقسامًا

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
disks-trouble-readonly = صار للقراءة فقط

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = تعطلت النسخة الاحتياطية لذاكرته

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = جزء منه صار للقراءة فقط

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = تجاوز عمره المتوقع

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
disks-health-failing = يتعطل. انسخ منه الآن كل ما تريد الاحتفاظ به

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = يحتاج إلى عناية: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = سليم

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }، { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = لا يستجيب

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = غلاف القرص متصل، لكن القرص الذي بداخله لا يستجيب. افصله، وانتظر عشر ثوانٍ، ثم صِله بمنفذ أزرق أو USB-C بكابله الخاص. إن بقي على حاله فقد يكون القرص الداخلي انفك أو تعطل.

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
       *[partition] أظهر هذا القسم
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] أخفِ هذا القرص. { $lock }
       *[partition] أخفِ هذا القسم. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = مخفي من Folders والحلقة. { $lock }

# Note on the card of the drive the system runs from.
disks-system-note = النظام يعمل منه، لذلك لا يمكن مسح أي شيء هنا.

# Note on the card of a drive that has no partition table.
disks-no-table = ليس لهذا القرص جدول أقسام، لذلك هو حر كله.

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
disks-change-it = غيّرها

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
disks-startup-note = يُركَّب كلما بدأ الحاسوب. وإن لم يكن موجودًا يبدأ الحاسوب على أي حال.

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
disks-format-erases = سيُمسح كل ما على { $words }. لا يمكنك التراجع عن ذلك.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = انقر مرة أخرى لتغيير حجم { $name } على { $drive } من { $from } إلى { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = مكان لقسم جديد

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = المكان بالجيجابايت

# Tip of the button that makes room under a part that is in use.
disks-room-tip = أفسح مكانًا

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = انقر مرة أخرى لأخذ { $size } من { $name } على { $drive }. سيأخذ Trindle لقطة أولًا.

# Note under the row that makes room.
disks-room-note = يأخذ Trindle لقطة، ثم يصغّر القسم وهو قيد الاستخدام. وتبقى المساحة التي تتحرر فارغة.

# Tip of the button that checks a part for faults.
disks-check-tip = افحصه بحثًا عن أخطاء

# Tip of the button that repairs a part.
disks-repair-tip = أصلحه

# Label of the row where a part is checked and repaired.
disks-faults = الأخطاء

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

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = انقر مرة أخرى لمسح كل ما على { $words }

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }، { $size }، على { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = احذف { $name }، { $size }، على { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = حذف القسم

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = اصنع قسمًا جديدًا هنا

# Label of the row where a new part is made.
disks-new-part = قسم جديد

# Tip of the button that makes a new part.
disks-make-tip = اصنعه

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = انقر مرة أخرى لصنع قسم جديد حجمه { $size } على { $drive }

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
disks-off-safe = آمن للفصل

# Words on a row while a part is being renamed.
disks-renaming = جارٍ تغيير الاسم

# Words on a row when a part has been renamed.
disks-renamed = تغيّر الاسم

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = سيُختار المجلد لك الآن

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = يظهر في { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = يُركَّب عند بدء التشغيل في { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = غير مركّب عند الإقلاع

# Words on a row while a folder or start-up choice is being saved.
disks-saving = جارٍ الحفظ

# Words on a row while a part is being resized.
disks-resizing = جارٍ تغيير الحجم

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = تغيّر الحجم إلى { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = لم يستطع Trindle العثور على الملفات، لذلك لا يستطيع إفساح مكان.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = جارٍ أخذ لقطة

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = لم يستطع Trindle العثور على اللقطة بعد ذلك.

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } لم يتغير أي حجم.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = تغيّر شيء أثناء أخذ Trindle للقطة، فلم يتغير أي حجم.

# Words on a row while room is being made.
disks-making-room = جارٍ إفساح المكان

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = جارٍ إيقاف ملف التبديل

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = جارٍ إعادة تشغيل ملف التبديل

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } يبقى ملف التبديل متوقفًا حتى تعيد التشغيل. { $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = أُفسح مكان بحجم { $size }

# Words on a row while a part is being repaired.
disks-repairing = جارٍ الإصلاح

# Words on a row when a part has been repaired.
disks-repaired = تم الإصلاح

# Red words when some faults could not be put right.
disks-repair-failed = لم يستطع Trindle إصلاح بعض الأخطاء. انسخ أي ملفات تريد الاحتفاظ بها إلى قرص آخر.

# Words on a row while a part is being checked.
disks-checking = جارٍ الفحص

# Words on a row when the check found no faults.
disks-no-faults = لم يُعثر على أخطاء

# Red words when the check found faults.
disks-faults-found = عُثر على أخطاء. اضغط على المفتاح الإنجليزي لإصلاحها.

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
disks-uuid-unread = لم يستطع Trindle قراءة معرّف القسم الجديد.

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
        [one] هذا الاسم طويل جدًا. اجعله حرفًا واحدًا على الأكثر.
        [two] هذا الاسم طويل جدًا. اجعله حرفين على الأكثر.
        [few] هذا الاسم طويل جدًا. اجعله { $max } أحرف على الأكثر.
        [many] هذا الاسم طويل جدًا. اجعله { $max } حرفًا على الأكثر.
        [zero] هذا الاسم طويل جدًا. اجعله { $max } حرف على الأكثر.
       *[other] هذا الاسم طويل جدًا. اجعله { $max } حرف على الأكثر.
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
        [one] هذا الاسم طويل جدًا لهذا القسم. اجعله حرفًا واحدًا على الأكثر.
        [two] هذا الاسم طويل جدًا لهذا القسم. اجعله حرفين على الأكثر.
        [few] هذا الاسم طويل جدًا لهذا القسم. اجعله { $max } أحرف على الأكثر.
        [many] هذا الاسم طويل جدًا لهذا القسم. اجعله { $max } حرفًا على الأكثر.
        [zero] هذا الاسم طويل جدًا لهذا القسم. اجعله { $max } حرف على الأكثر.
       *[other] هذا الاسم طويل جدًا لهذا القسم. اجعله { $max } حرف على الأكثر.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = لا تقبل أسماء FAT هذه الرموز: * ? . , ; : / \ | + = < > [ ] "

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] ركّبه أولًا. لا يمكن تكبير هذا النوع من الأقسام إلا وهو مركّب.
       *[smaller] ركّبه أولًا. لا يمكن تصغير هذا النوع من الأقسام إلا وهو مركّب.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] لا يمكن تكبير هذا النوع.
       *[smaller] لا يمكن تصغير هذا النوع.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = لم تُدخل كلمة مرور، فلم يتغير شيء.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = لم تُدخل كلمة مرور، فتوقف العمل في منتصفه.

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = تغيير آخر لا يزال جاريًا. حاول مرة أخرى عند انتهائه.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = لم يستطع Trindle التأكد من أنه سيطلب كلمة مرورك، فلم يتغير شيء.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = لم يستطع Trindle إلغاء كلمة المرور التي أعطيتها للتو. قد لا يطلب أي تغيير آخر على قرص كلمة مرور لبضع دقائق. يحاول Trindle مرة أخرى.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = أُلغيت كلمة المرور التي أعطيتها الآن، لذلك سيطلب التغيير التالي كلمة مرور.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = لم يستطع Trindle الوصول إلى خدمة الأقراص، فلم يتغير شيء.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = أُغلق برنامج الأقراص أثناء إجراء هذا التغيير.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = سيبقى برنامج الأقراص مفتوحًا حتى ينتهي هذا، لتعرف كيف سار الأمر.

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
disks-still-unmounted = أُزيل تركيب { $dev } من أجل هذا وما زال غير مركّب. ركّبه من صفه.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = ربما أُزيل تركيب { $dev }. انظر إلى صفه.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = أُزيل تركيب { $dev } من أجل هذا. ركّبه مرة أخرى من صفه وأدخل كلمة مروره عندما يُطلب منك ذلك.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = أُزيل تركيب { $dev } من أجل هذا وتعذّر تركيبه مرة أخرى ({ $why }). ركّبه من صفه.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = تمت التهيئة، لكن Trindle لم يستطع إعادة المجلد { $dir } إليه. { $why } اختر المجلد مرة أخرى من صفه.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = ما زال شيء يستخدمه. أغلق النوافذ أو البرامج التي تعرض ملفاته، ثم حاول مرة أخرى.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = هذا الحجم صغير جدًا للملفات الموجودة عليه بالفعل.

# Step of a change. { $dev } is the part.
disks-step-unmount = إزالة تركيب { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = تركيب { $dev } مرة أخرى

# Step of a change.
disks-step-eject = تجهيز القرص للإزالة

# Step of a change.
disks-step-power-off = إطفاء القرص

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = تهيئة { $dev }

# Step of a change.
disks-step-create = إنشاء قسم

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = حذف { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = تغيير حجم { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = تغيير حجم ما على { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = إصلاح { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = فحص { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = تغيير اسم { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = حفظ إعدادات المجلد وبدء التشغيل لـ{ $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = الخطوة { $method } على { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = لا يمكن تصغير هذا القسم من نوع { $kind } إلا إذا لم يكن قيد الاستخدام، لذلك لا يستطيع Trindle إفساح مكان منه هنا.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = لا يمكن تصغير أقسام xfs.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = هذا القسم من نوع { $kind } لا يمكن تصغيره وهو قيد الاستخدام.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = لا يستطيع Trindle أخذ لقطة إلا من قرص النظام نفسه، فلا يستطيع إفساح مكان هنا.

# Red words under the Make room row.
disks-room-not-in-table = ليس قسمًا في جدول أقسام القرص، فلا يستطيع Trindle إفساح مكان منه.

# Red words under the Make room row.
disks-room-no-online-shrink = لا تستطيع خدمة الأقراص تصغير Btrfs وهو قيد الاستخدام هنا، فلا يستطيع Trindle إفساح مكان.

# Red words under the Make room row.
disks-room-holds-parts = فيه أقسام أخرى، فلا يستطيع Trindle إفساح مكان منه.

# Red words under the Make room row.
disks-room-not-gpt = ليس في قرصه جدول أقسام GPT، فلا يستطيع Trindle إفساح مكان منه.

# Red words under the Make room row.
disks-room-mount-first = ركّبه أولًا: يُصغَّر Btrfs وهو مركّب.

# Red words under the Make room row.
disks-room-mounts-not-one = ليس مركّبًا، فلا يستطيع Trindle إفساح مكان.

# Red words under the Make room row.
disks-room-read-only = مركّب للقراءة فقط، فلا يستطيع Trindle إفساح مكان.

# Red words under the Make room row.
disks-room-read-only-unknown = لم يستطع Trindle معرفة هل هو للقراءة فقط، فلا يستطيع إفساح مكان.

# Red words under the Make room row.
disks-room-many-drives = الملفات موزعة على أكثر من قرص، فلا يستطيع Trindle إفساح مكان.

# Red words under the Make room row.
disks-room-drives-unknown = لم يستطع Trindle معرفة عدد الأقراص التي توزعت عليها الملفات، فلا يستطيع إفساح مكان.

# Red words under the Make room row.
disks-room-swap-unknown = لم يستطع Trindle معرفة هل يوجد عليه ملف تبديل، فلا يستطيع إفساح مكان.

# Red words under the Make room row.
disks-room-used-unknown = لم يستطع Trindle قراءة المقدار المستخدم منه، فلا يستطيع إفساح مكان.

# Red words under the Make room row.
disks-room-area-unknown = لا تذكر خدمة الأقراص حجم نظام الملفات، فلا يستطيع Trindle التحقق منه ولا إفساح مكان.

# Red words under the Make room row.
disks-room-open-missing = لم يستطع Trindle العثور على نسخته غير المقفلة، فلا يستطيع إفساح مكان.

# Red words under the Make room row.
disks-room-encryption-sizes = الأحجام المعطاة للتشفير غير متطابقة، فلا يستطيع Trindle إفساح مكان.

# Red words under the Make room row.
disks-room-sizes = الأحجام المعطاة له غير متطابقة، فلا يستطيع Trindle إفساح مكان.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = فيه { $used } ويحتاج إلى إبقاء أكثر من ذلك، فالمساحة التي يمكن تحريرها قليلة جدًا.

# Red words under the Make room row.
disks-room-sizes-unread = تعذّرت قراءة أحجامه.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = يمكنك إفساح مكان بين { $least } و{ $most }.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }، فتوقف Trindle.

# What was found wrong while making room.
disks-room-check-unmounted = الملفات لم تعد مركّبة

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = حجم القسم { $is } وينبغي أن يكون { $should }

# What was found wrong while making room.
disks-room-check-open = النسخة غير المقفلة من التشفير ليست بالحجم الذي ينبغي

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = حجم نظام الملفات { $is } وينبغي أن يكون { $should }

# What was found wrong while making room.
disks-room-check-area-small = نظام الملفات أصغر من المستخدم

# What was found wrong while making room.
disks-room-check-fit = لم يعد نظام الملفات يتسع

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] لم تعد خدمة الأقراص تعرض القسم، فأُوقف العمل.
        [copy] لم تعد خدمة الأقراص تعرض النسخة غير المقفلة، فأُوقف العمل.
       *[files] لم تعد خدمة الأقراص تعرض نظام الملفات، فأُوقف العمل.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = تصغير نظام الملفات

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = تصغير النسخة غير المقفلة من التشفير

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = تصغير القسم

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = فحص الأحجام

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = قبل إفساح مكان على { $dev } ({ $at })

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }

# Red words when another change is pressed while one is still being made.
disks-change-going = جارٍ إجراء تغيير.


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = انقر مرة أخرى لتغيير حجم { $name } على { $drive } من { $from } إلى { $to }. انسخ ملفاته احتياطيًا أولًا، لأن انقطاع الكهرباء أثناء التصغير قد يؤدي إلى فقدانها.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = نظام Windows على { $name } كان في السبات أو تُرك في البدء السريع، لذلك قد يؤدي تصغيره إلى فقدان ملفاته. شغّل Windows، واختر إعادة التشغيل، ثم حاول مرة أخرى.
