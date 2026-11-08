
# machine translated (Urdu), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = ڈرائیوز

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = اس صفحے کے بارے میں

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = اس کمپیوٹر میں لگی یا اس سے جڑی ہر ڈرائیو، اپنے پارٹیشنز کی پٹی کی صورت میں۔ ہر پارٹیشن اپنے سائز کے برابر چوڑا ہے اور جتنا استعمال ہوا اتنا بھرا ہے، اس لیے بھرا ہوا پارٹیشن آسانی سے نظر آ جاتا ہے۔

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = تصویر کے نیچے ہر ڈرائیو کا ایک کارڈ ہے۔ کسی پارٹیشن کو کھولیں، ماؤنٹ یا ان ماؤنٹ کریں، یا ڈرائیو کو محفوظ طریقے سے نکالیں یا بند کر دیں۔ پارٹیشن پر قلم اس کی تبدیلیاں کھولتا ہے: اس کا نام، اس کا اپنا فولڈر اور یہ کہ آیا یہ شروع ہوتے ہی تیار ہو، اس کا سائز، اسے جانچنا اور درست کرنا، فارمیٹ کرنا یا مٹانا۔ خالی جگہ میں جمع کا نشان نیا پارٹیشن بناتا ہے۔

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = جس ڈرائیو سے نظام چلتا ہے وہ دکھائی جاتی ہے مگر کوئی ایسی تبدیلی اس کے لیے پیش نہیں کی جاتی جو چیزوں کو مٹائے یا ہٹائے۔ کسی بھی ڈرائیو میں ہر تبدیلی کے لیے ہر بار منتظم کا پاس ورڈ مانگا جاتا ہے۔ آنکھ ڈرائیو یا پارٹیشن کو فولڈرز اور حلقے سے چھپاتی ہے: وہ یہاں دھندلی حالت میں رہتی ہے، اور مقفل نہیں ہوتی۔

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = ہر ڈرائیو کا ہر پارٹیشن، جتنا استعمال ہوا اتنا بھرا ہوا۔

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = آپ کی ڈرائیوز

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = ڈرائیوز ابھی پڑھی نہیں جا سکتیں۔

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = دوبارہ کوشش کریں

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = دوبارہ دیکھیں

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = کوئی ڈرائیو نہیں ملی۔ ایک لگائیں تو یہاں نظر آئے گی۔

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = ڈرائیو سروس

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = سسٹم بس

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } دستیاب نہیں ہے۔ اسے اس طرح انسٹال کریں: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = ڈرائیوز دیکھی جا رہی ہیں…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } خراب ہو رہی ہے۔ جو کچھ رکھنا چاہتے ہیں وہ ابھی اس سے نقل کر لیں۔

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } کو دیکھنے کی ضرورت ہے: { $troubles }۔

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] ڈرائیوز اور ان پر جگہ۔
        [one] ڈرائیوز اور ان پر جگہ، اور ایک ڈرائیو لگی ہوئی۔
       *[other] ڈرائیوز اور ان پر جگہ، اور { $plugged } ڈرائیوز لگی ہوئی۔
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] تقریباً بھری ہوئی: اس کمپیوٹر پر صرف { $free } خالی ہے۔ جلد کچھ فائلیں مٹائیں یا منتقل کریں۔
        [one] تقریباً بھری ہوئی: اس کمپیوٹر پر صرف { $free } خالی ہے، اور ایک ڈرائیو لگی ہوئی ہے۔ جلد کچھ فائلیں مٹائیں یا منتقل کریں۔
       *[other] تقریباً بھری ہوئی: اس کمپیوٹر پر صرف { $free } خالی ہے، اور { $plugged } ڈرائیوز لگی ہوئی ہیں۔ جلد کچھ فائلیں مٹائیں یا منتقل کریں۔
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] بھرتی جا رہی ہے: اس کمپیوٹر پر { $free } خالی ہے۔
        [one] بھرتی جا رہی ہے: اس کمپیوٹر پر { $free } خالی ہے، اور ایک ڈرائیو لگی ہوئی ہے۔
       *[other] بھرتی جا رہی ہے: اس کمپیوٹر پر { $free } خالی ہے، اور { $plugged } ڈرائیوز لگی ہوئی ہیں۔
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] کافی جگہ ہے: اس کمپیوٹر پر { $free } خالی ہے۔
        [one] کافی جگہ ہے: اس کمپیوٹر پر { $free } خالی ہے، اور ایک ڈرائیو لگی ہوئی ہے۔
       *[other] کافی جگہ ہے: اس کمپیوٹر پر { $free } خالی ہے، اور { $plugged } ڈرائیوز لگی ہوئی ہیں۔
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = یہ کمپیوٹر

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = آپ کی فائلیں

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } ابھی خالی ہے، جہاں نظام ہے۔
       *[files] { $free } ابھی خالی ہے، آپ کی اپنی فائلوں کے لیے۔
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } بھری ہوئی، { $whole } میں سے { $free } خالی

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = { $whole } میں سے

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = کوئی ریڈنگ نہیں، ڈرائیوز نہیں بتاتیں کہ ان کا حال کیسا ہے

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = صحت مند، جیسا ڈرائیو خود بتاتی ہے

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] صحت مند، جیسا تمام { $n } ڈرائیوز بتاتی ہیں
       *[other] صحت مند، جیسا تمام { $n } ڈرائیوز بتاتی ہیں
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } کو دیکھنے کی ضرورت ہے، اس کا کارڈ دیکھیں
       *[other] { $n } کو دیکھنے کی ضرورت ہے، ان کے کارڈ دیکھیں
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 ڈرائیو، کل { $total }۔ { $health }۔
       *[other] { $n } ڈرائیوز، کل { $total }۔ { $health }۔
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } خالی

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = ماؤنٹ نہیں

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = خالی جگہ

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = اس میں ڈسک یا کارڈ نہیں

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = ابھی اس پر کچھ نہیں

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }، USB یا نکالی جانے والی

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }، اس کمپیوٹر کے اندر

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }، USB یا نکالی جانے والی، چھپی ہوئی

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }، اس کمپیوٹر کے اندر، چھپی ہوئی

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }، { $full } بھرا ہوا، { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }، { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }، { $words }: { $parts }۔

# Name given to a part that has no label and no device name.
disks-part-name = پارٹیشن

# Name given to a drive that gives no maker or model.
disks-drive-name = ڈرائیو

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }، { $size }

# Kind of a part that is swap space.
disks-kind-swap = سواپ

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = خفیہ کاری شدہ، کھلا ہوا

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = خفیہ کاری شدہ، مقفل

# Kind of a part that holds other parts.
disks-kind-container = دوسرے پارٹیشنز رکھتا ہے

# Kind of a part that has no filesystem on it.
disks-kind-none = کوئی فائل سسٹم نہیں

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }، EFI، وہ پارٹیشن جس سے کمپیوٹر شروع ہوتا ہے
        [legacy] { $kind }، وہ پارٹیشن جس سے پرانا کمپیوٹر شروع ہوتا ہے
       *[loader] { $kind }، بوٹ لوڈر کا اپنا پارٹیشن
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = کسی بھی کمپیوٹر کے لیے

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = صرف Linux کے لیے

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows کے لیے

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = پرانے آلات کے لیے

# Trouble a drive reports about its spare space.
disks-trouble-spare = اضافی جگہ ختم ہو رہی ہے

# Trouble a drive reports about its heat.
disks-trouble-temperature = بہت گرم ہے

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = گھس رہی ہے

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = خود کو صرف پڑھنے والی بنا لیا ہے

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = اس کی میموری کا بیک اپ خراب ہو گیا ہے

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = اس کے ایک حصے نے خود کو صرف پڑھنے والا بنا لیا ہے

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = ساز کی توقع سے زیادہ گھس چکی ہے

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } خراب جگہ
       *[other] { $n } خراب جگہیں
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] { $hours } گھنٹہ استعمال ہوئی
       *[other] { $hours } گھنٹے استعمال ہوئی
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = خراب ہو رہی ہے: جو کچھ رکھنا چاہتے ہیں وہ ابھی اس سے نقل کر لیں

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = توجہ درکار ہے: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = صحت مند

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }، { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = کوئی ڈرائیو جواب نہیں دے رہی

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = خول جواب دیتا ہے، مگر اس کے اندر کی ڈرائیو نہیں۔ اسے نکالیں، دس سیکنڈ رکیں اور اسے اس کی اپنی تار کے ساتھ نیلی یا USB-C پورٹ میں لگائیں؛ اگر یہی حال رہے تو ممکن ہے اندر کی ڈرائیو ڈھیلی ہو گئی ہو یا خراب ہو گئی ہو۔

# Tip of the round button on a drive's card that takes the drive out safely.
# SAFETY: needs a human review before it ships
disks-remove-safely = محفوظ طریقے سے نکالیں

# Tip of the round button on a drive's card that powers the drive off.
# SAFETY: needs a human review before it ships
disks-power-off-tip = اسے بند کریں

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = پوری ڈرائیو مٹائیں

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] یہ ڈرائیو دکھائیں
       *[part] یہ پارٹیشن دکھائیں
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] یہ ڈرائیو چھپائیں۔ { $lock }
       *[part] یہ پارٹیشن چھپائیں۔ { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = فولڈرز اور حلقے سے چھپی ہوئی۔ { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = نظام اسی سے چلتا ہے: یہاں کچھ بھی مٹایا نہیں جا سکتا۔

# Note on the card of a drive that has no partition table.
disks-no-table = کوئی پارٹیشن ٹیبل نہیں: پوری ڈرائیو خالی ہے۔

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = ماؤنٹ نہیں

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = شروع ہوتے ہی { $dir } میں ماؤنٹ ہوتا ہے

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = { $dir } میں ظاہر ہوتا ہے

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = { $dir } میں

# Tip of the round button that opens the folder a part is mounted in.
disks-open = کھولیں

# Tip of the round button that mounts a part.
disks-mount = ماؤنٹ کریں

# Tip of the round button that unmounts a part.
disks-unmount = ان ماؤنٹ کریں

# Tip of the round button on a part that opens the box of changes.
disks-change-it = بدلیں

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = نام

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = نام (اختیاری)

# Tip of the button that saves a part's new name.
disks-rename-tip = نام بدلیں

# Note under the Name row of a part.
disks-name-note = یہاں اور دوسرے کمپیوٹروں پر اسے کس نام سے پکارا جاتا ہے۔

# Placeholder of the typing box for a part's folder.
disks-folder-name = فولڈر کا نام

# Tip of the button that saves a part's folder.
disks-use-folder = یہ فولڈر استعمال کریں

# Label of the row where a part's folder is chosen.
disks-folder = فولڈر

# Note under the Folder row of a part.
disks-folder-note = اس کی فائلیں کہاں ظاہر ہوتی ہیں۔ کمپیوٹر کو چننے دینے کے لیے خالی چھوڑیں۔

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = شروع ہوتے ہی ماؤنٹ کریں

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = کمپیوٹر شروع ہوتے ہی تیار۔ اگر یہ موجود نہ ہو تو بھی کمپیوٹر شروع ہو جاتا ہے۔

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = سائز GB میں

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = { $least } سے { $most } تک۔

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = { $least } سے { $most } تک۔ یہ دیکھنے کے لیے کہ یہ کتنا چھوٹا ہو سکتا ہے، اسے ماؤنٹ کریں۔

# Tip of the button that resizes a part.
# SAFETY: needs a human review before it ships
disks-resize-tip = سائز بدلیں

# Label of the row where a part is resized.
disks-size = سائز

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = اس سے { $words } پر موجود سب کچھ مٹ جائے گا۔ اسے واپس نہیں کیا جا سکتا۔

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = دوبارہ کلک کریں: { $drive } پر { $name } کا سائز { $from } سے { $to } ہو جائے گا

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = نئے پارٹیشن کے لیے جگہ بنائیں

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = جگہ GB میں

# Tip of the button that makes room under a part that is in use.
# SAFETY: needs a human review before it ships
disks-room-tip = جگہ بنائیں

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = دوبارہ کلک کریں: { $drive } پر { $name } سے { $size } کم ہو جائے گا، اسنیپ شاٹ کے بعد

# Note under the row that makes room.
disks-room-note = استعمال کے دوران، اسنیپ شاٹ کے بعد چھوٹا کیا جاتا ہے، اور خالی جگہ چھوڑی جاتی ہے۔

# Tip of the button that checks a part for faults.
disks-check-tip = خرابیوں کے لیے جانچیں

# Tip of the button that repairs a part.
# SAFETY: needs a human review before it ships
disks-repair-tip = مرمت کریں

# Label of the row where a part is checked and repaired.
disks-faults = خرابیاں

# Note under the Faults row.
disks-faults-note = یہ دیکھتا ہے کہ اس کی فائلیں جس طرح رکھی گئی ہیں اس میں کوئی خرابی تو نہیں، اور اسے درست کرتا ہے۔ اس دوران یہ ان ماؤنٹ رہتا ہے۔

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = { $drive } پر { $name } کی مرمت کے لیے دوبارہ کلک کریں

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = قسم

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = فارمیٹ

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = فارمیٹ کریں

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = اسے ایک پارٹیشن کے طور پر نئے سرے سے شروع کرتا ہے، جو پوری ڈرائیو بھر دیتا ہے۔

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = صرف اس پارٹیشن کا سب کچھ مٹاتا ہے۔

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = دوبارہ کلک کریں: { $words } پر سب کچھ مٹ جائے گا

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $drive } پر { $name }، { $size }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = { $drive } پر { $name }، { $size } مٹائیں

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = یہ پارٹیشن مٹائیں

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = اس کی جگہ خالی ہو جاتی ہے، نئے پارٹیشن کے لیے تیار یا اس سے پہلے والے کے بڑھنے کے لیے۔

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = یہاں نیا پارٹیشن بنائیں

# Label of the row where a new part is made.
disks-new-part = نیا پارٹیشن

# Note under the row where a new part is made.
disks-new-part-note = اس خالی جگہ میں بنایا جاتا ہے، اور استعمال کے لیے تیار فارمیٹ کیا جاتا ہے۔

# Tip of the button that makes a new part.
disks-make-tip = بنائیں

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-make-warning = { $drive } پر { $size } کا پارٹیشن بنانے کے لیے دوبارہ کلک کریں

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = { $name } مٹائیں

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $drive } پر { $size }

# Title of the box for making a new part in free space.
disks-sheet-new = ایک نیا پارٹیشن

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $drive } پر { $size } خالی

# Tip of the round button that closes the box of changes.
disks-close = بند کریں

# Words on a row while a part is being mounted.
disks-mounting = ماؤنٹ ہو رہا ہے

# Words on a row when a part has been mounted.
disks-mounted = ماؤنٹ ہو گیا

# Words on a row while a part is being unmounted.
disks-unmounting = ان ماؤنٹ ہو رہا ہے

# Words on a row when a part has been unmounted.
disks-unmounted = ان ماؤنٹ ہو گیا

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = بند کی جا رہی ہے

# Words on a drive's row when it is safe to remove.
disks-safe-remove = نکالنا محفوظ ہے

# Words on a drive's row while it is being powered off.
disks-powering-off = بند ہو رہی ہے

# Words on a drive's row when it has been powered off.
disks-off-safe = بند: نکالنا محفوظ ہے

# Words on a row while a part is being renamed.
disks-renaming = نام بدلا جا رہا ہے

# Words on a row when a part has been renamed.
disks-renamed = نام بدل دیا گیا

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = کمپیوٹر دوبارہ اس کا فولڈر خود چنتا ہے

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = یہ { $dir } میں ظاہر ہوتا ہے

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = شروع ہوتے ہی { $dir } میں ماؤنٹ ہوتا ہے

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = شروع ہوتے وقت اسے چھوڑ دیا جاتا ہے

# Words on a row while a folder or start-up choice is being saved.
disks-saving = محفوظ ہو رہا ہے

# Words on a row while a part is being resized.
disks-resizing = سائز بدلا جا رہا ہے

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = سائز { $size } ہو گیا

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = فائلیں نہیں ملیں، اس لیے جگہ نہیں بنائی جا سکتی۔

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = اسنیپ شاٹ لیا جا رہا ہے

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = اسنیپ شاٹ بعد میں نہیں ملا۔

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } کسی کا سائز نہیں بدلا گیا۔

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = اسنیپ شاٹ لیتے وقت چیزیں بدل گئیں، اس لیے کسی کا سائز نہیں بدلا گیا۔

# Words on a row while room is being made.
disks-making-room = جگہ بنائی جا رہی ہے

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } جگہ بنا دی گئی

# Words on a row while a part is being repaired.
disks-repairing = مرمت ہو رہی ہے

# Words on a row when a part has been repaired.
disks-repaired = مرمت ہو گئی

# Red words when some faults could not be put right.
disks-repair-failed = کچھ خرابیاں درست نہیں ہو سکیں۔ جو کچھ رکھنا چاہتے ہیں وہ نقل کر لیں۔

# Words on a row while a part is being checked.
disks-checking = جانچ ہو رہی ہے

# Words on a row when the check found no faults.
disks-no-faults = کوئی خرابی نہیں ملی

# Red words when the check found faults.
disks-faults-found = خرابیاں ملیں: انہیں درست کرنے کے لیے رینچ دبائیں۔

# Words on a row while a part or drive is being formatted.
disks-formatting = فارمیٹ ہو رہا ہے

# Words on a row when a part has been formatted.
disks-formatted = فارمیٹ ہو گیا

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = پوری ڈرائیو مٹانا

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = نیا پارٹیشن نہیں ملا۔

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = ڈرائیو سروس تک رسائی نہیں ہو سکی۔

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = نئے پارٹیشن کی شناخت پڑھی نہیں جا سکی۔

# Words on a row while a part is being deleted.
disks-deleting = مٹایا جا رہا ہے

# Words on a row when a part has been deleted.
disks-deleted = مٹا دیا گیا

# Words on a row while a new part is being made.
disks-making-it = بنایا جا رہا ہے

# Words on a row when a new part has been made.
disks-made = بنا دیا گیا

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = یہ محفوظ نہیں رکھا جا سکا: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
# SAFETY: needs a human review before it ships
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = پہلے اسے کوئی نام دیں۔

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] یہ نام بہت لمبا ہے: زیادہ سے زیادہ { $max } حرف۔
       *[other] یہ نام بہت لمبا ہے: زیادہ سے زیادہ { $max } حروف۔
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = صرف حروف، ہندسے، - اور _ استعمال کریں (خالی جگہ نہیں)۔

# Red words under the folder name box when it starts with a dash.
disks-name-start = اسے حرف یا ہندسے سے شروع کریں۔

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } پہلے ہی استعمال میں ہے۔

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] یہ نام اس قسم کے لیے بہت لمبا ہے: زیادہ سے زیادہ { $max } حرف۔
       *[other] یہ نام اس قسم کے لیے بہت لمبا ہے: زیادہ سے زیادہ { $max } حروف۔
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT کے ناموں میں * ? . , ; : / \ | + = < > [ ] یا " نہیں ہو سکتے۔

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] پہلے اسے ماؤنٹ کریں: اس قسم کو ماؤنٹ ہونے کی حالت میں ہی بڑا کیا جا سکتا ہے۔
       *[smaller] پہلے اسے ماؤنٹ کریں: اس قسم کو ماؤنٹ ہونے کی حالت میں ہی چھوٹا کیا جا سکتا ہے۔
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] اس قسم کو بڑا نہیں کیا جا سکتا۔
       *[smaller] اس قسم کو چھوٹا نہیں کیا جا سکتا۔
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = کوئی پاس ورڈ نہیں دیا گیا، اس لیے کچھ نہیں بدلا۔

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = کوئی پاس ورڈ نہیں دیا گیا، اس لیے کام بیچ میں رک گیا۔

# Red words when another change is pressed while one is still being made.
disks-change-going = ایک تبدیلی ہو رہی ہے۔

disks-one-at-a-time = ایک اور تبدیلی ابھی ہو رہی ہے۔ ختم ہونے پر دوبارہ کوشش کریں۔

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = یہ یقینی نہیں بنایا جا سکا کہ پاس ورڈ مانگا جائے گا، اس لیے کچھ نہیں بدلا۔

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = ابھی دیا گیا پاس ورڈ واپس نہیں لیا جا سکا، اس لیے چند منٹ تک ڈرائیو میں اگلی تبدیلی پاس ورڈ نہ مانگے۔ اسے دوبارہ واپس لینے کی کوشش ہو رہی ہے۔

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = دیا گیا پاس ورڈ اب واپس لے لیا گیا ہے: اگلی تبدیلی پاس ورڈ مانگے گی۔

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = ڈرائیو سروس تک رسائی نہیں ہو سکی، اس لیے کچھ نہیں بدلا۔

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = یہ تبدیلی ہوتے وقت ڈرائیوز بند ہو گئی تھی۔

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = ڈرائیوز اس کے ختم ہونے تک کھلی رہتی ہے، تاکہ بتا سکے کہ یہ کیسے ختم ہوئی۔

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }، اس لیے کچھ نہیں بدلا۔

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }، اس لیے کام بیچ میں رک گیا۔

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = ڈرائیو سروس سے پوچھا نہیں جا سکا

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } کام بیچ میں رک گیا۔

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }۔

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = ہو گیا: { $steps }۔

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = ناکام: { $steps }۔

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = نہیں ہوا: { $steps }۔

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = معلوم نہیں: { $steps }۔

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } اس کام کے لیے ان ماؤنٹ کیا گیا تھا اور اب ماؤنٹ نہیں ہے۔

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } اس کام کے لیے ان ماؤنٹ کیا گیا تھا اور اب بھی ان ماؤنٹ ہے: اسے اس کی قطار سے ماؤنٹ کریں۔

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } شاید ان ماؤنٹ ہو گیا ہو: اس کی قطار دیکھیں۔

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } اس کام کے لیے ان ماؤنٹ کیا گیا تھا، اور دوبارہ ماؤنٹ کرنے کے لیے اس کا اپنا پاس ورڈ درکار ہے: اسے اس کی قطار سے ماؤنٹ کریں۔

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } اس کام کے لیے ان ماؤنٹ کیا گیا تھا اور دوبارہ ماؤنٹ نہیں ہو سکا ({ $why }): اسے اس کی قطار سے ماؤنٹ کریں۔

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = فارمیٹ ہو گیا، لیکن اسے دوبارہ اس کا فولڈر { $dir } نہیں دیا گیا۔ { $why } اس کی قطار سے فولڈر دوبارہ چنیں۔

# Red words when a drive is in use and cannot be unmounted.
disks-busy = کوئی چیز اب بھی اسے استعمال کر رہی ہے۔ اس کی فائلیں دکھانے والی ونڈوز یا پروگرام بند کریں، پھر دوبارہ کوشش کریں۔

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = اس سائز پر اس میں موجود چیزوں کے لیے کافی جگہ نہیں ہے۔

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } کو ان ماؤنٹ کرنا

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } کو دوبارہ ماؤنٹ کرنا

# Step of a change.
disks-step-eject = نکالی جانے والی ڈرائیو کو بند کرنا

# Step of a change.
disks-step-power-off = ڈرائیو کو بند کرنا

# Step of a change. { $dev } is the part.
disks-step-format = { $dev } کو فارمیٹ کرنا

# Step of a change.
disks-step-create = نیا پارٹیشن بنانا

# Step of a change. { $dev } is the part.
disks-step-delete = { $dev } کو مٹانا

# Step of a change. { $dev } is the part.
disks-step-resize-part = پارٹیشن { $dev } کا سائز بدلنا

# Step of a change. { $dev } is the part.
disks-step-resize = { $dev } پر موجود چیزوں کا سائز بدلنا

# Step of a change. { $dev } is the part.
disks-step-repair = { $dev } کی مرمت کرنا

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } کی جانچ کرنا

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } کا نام بدلنا

# Step of a change. { $dev } is the part.
disks-step-fstab = { $dev } کا فولڈر اور شروع ہوتے وقت کا انتخاب محفوظ کرنا

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = { $dev } پر قدم { $method }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = اس { $kind } پارٹیشن کو صرف تب چھوٹا کیا جا سکتا ہے جب یہ استعمال میں نہ ہو، اس لیے یہاں اس سے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = اس xfs پارٹیشن کو چھوٹا نہیں کیا جا سکتا۔

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = اس { $kind } پارٹیشن کو استعمال کے دوران چھوٹا نہیں کیا جا سکتا۔

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = اسنیپ شاٹ صرف نظام کی اپنی ڈسک کا لیا جا سکتا ہے، اس لیے یہاں جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-not-in-table = یہ ڈرائیو کی ٹیبل کا پارٹیشن نہیں ہے، اس لیے اس سے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-no-online-shrink = ڈرائیو سروس یہاں استعمال کے دوران Btrfs کو چھوٹا نہیں کر سکتی، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-holds-parts = اس میں دوسرے پارٹیشنز ہیں، اس لیے اس سے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-not-gpt = اس کی ڈرائیو میں GPT ٹیبل نہیں ہے، اس لیے اس سے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-mount-first = پہلے اسے ماؤنٹ کریں: Btrfs کو ماؤنٹ ہونے کی حالت میں چھوٹا کیا جاتا ہے۔

# Red words under the Make room row.
disks-room-mounts-not-one = یہ ٹھیک ایک جگہ ماؤنٹ نہیں ہے، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-read-only = یہ صرف پڑھنے کے لیے ماؤنٹ ہے، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-read-only-unknown = یہ معلوم نہیں ہو سکا کہ یہ صرف پڑھنے کے لیے ہے یا نہیں، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-many-drives = فائلیں ایک سے زیادہ ڈرائیوز پر پھیلی ہیں، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-drives-unknown = یہ معلوم نہیں ہو سکا کہ فائلیں کتنی ڈرائیوز پر پھیلی ہیں، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-swap-file = اس پر سواپ فائل موجود ہے، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-swap-unknown = یہ معلوم نہیں ہو سکا کہ اس پر سواپ فائل ہے یا نہیں، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-used-unknown = یہ پڑھا نہیں جا سکا کہ اس کا کتنا حصہ استعمال میں ہے، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-area-unknown = ڈرائیو سروس نہیں بتاتی کہ فائلوں کا رقبہ کتنا بڑا ہے، اس لیے جانچ نہیں ہو سکی اور جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-open-missing = اس کی کھلی نقل نہیں ملی، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-encryption-sizes = خفیہ کاری کے لیے دیے گئے سائز آپس میں نہیں ملتے، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row.
disks-room-sizes = اس کے لیے دیے گئے سائز آپس میں نہیں ملتے، اس لیے جگہ نہیں بنائی جا سکتی۔

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = اس میں { $used } ہے اور اس سے زیادہ رکھنا ضروری ہے، اس لیے بہت کم جگہ خالی ہو سکتی ہے۔

# Red words under the Make room row.
disks-room-sizes-unread = اس کے سائز پڑھے نہیں جا سکے۔

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = { $least } سے { $most } تک جگہ بنائی جا سکتی ہے۔

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }، اس لیے کام روک دیا گیا۔

# What was found wrong while making room.
disks-room-check-unmounted = فائلیں اب ماؤنٹ نہیں ہیں

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = پارٹیشن { $is } ہے اور اسے { $should } ہونا چاہیے

# What was found wrong while making room.
disks-room-check-open = خفیہ کاری کی کھلی نقل کا سائز وہ نہیں جو ہونا چاہیے

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = فائلوں کا رقبہ { $is } ہے اور اسے { $should } ہونا چاہیے

# What was found wrong while making room.
disks-room-check-area-small = فائلوں کا رقبہ استعمال شدہ چیزوں سے چھوٹا ہے

# What was found wrong while making room.
disks-room-check-fit = جو چیز جس میں رکھی ہے وہ اب اس میں پوری نہیں آتی

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] ڈرائیو سروس اب پارٹیشن کی فہرست نہیں دیتی، اس لیے کام روک دیا گیا۔
        [copy] ڈرائیو سروس اب کھلی نقل کی فہرست نہیں دیتی، اس لیے کام روک دیا گیا۔
       *[files] ڈرائیو سروس اب فائل سسٹم کی فہرست نہیں دیتی، اس لیے کام روک دیا گیا۔
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = فائلوں کا رقبہ چھوٹا کرنا

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = خفیہ کاری کی کھلی نقل کو چھوٹا کرنا

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = پارٹیشن کو چھوٹا کرنا

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = سائز جانچنا

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = { $dev } کے نیچے جگہ بنانے سے پہلے ({ $at })

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = دوبارہ کلک کریں: { $drive } پر { $name } کا سائز { $from } سے { $to } ہو جائے گا۔ پہلے اس کی فائلوں کا بیک اپ لیں، کیونکہ چھوٹا ہوتے وقت بجلی جانے سے وہ ضائع ہو سکتی ہیں۔

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name } پر Windows ہائبرنیٹ تھا یا فاسٹ اسٹارٹ اپ میں چھوڑا گیا تھا، اس لیے اسے چھوٹا کرنے سے اس کی فائلیں ضائع ہو سکتی ہیں۔ Windows شروع کریں، ری اسٹارٹ چنیں، پھر دوبارہ کوشش کریں۔

