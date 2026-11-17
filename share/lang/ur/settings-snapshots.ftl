
# machine translated (Urdu), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = سنیپ شاٹس

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = سنیپ شاٹس سنیپ شاٹ بحال واپس رول بیک بیک اپ اپ ڈیٹ خراب btrfs snapper بوٹ مینو grub پرانے ورژن پچھلا ورژن حذف شدہ فائل واپس رکھنا بازیافت ہوم نقول

# Help line at the foot of the Snapshots page.
settings-snap-help = سنیپ شاٹ نظام کی ایک نقل ہے، جیسا وہ ہر سافٹ ویئر تبدیلی سے پہلے اور بعد تھا، اور جب بھی آپ ایک لیں۔ سنیپ شاٹ بحال کرنے سے جب آپ اگلی بار دوبارہ شروع کریں گے تو پورا نظام اسی حالت میں واپس آ جاتا ہے۔ آپ کی اپنی فائلیں جیسی ہیں ویسی رہتی ہیں اگر ہوم فولڈر کے لیے ڈسک کا اپنا حصہ ہو، جیسا Trindle کا انسٹالر بناتا ہے۔ اگر کمپیوٹر شروع نہ ہو تو بوٹ مینو میں Trindle snapshots چنیں۔ پرانے ورژن ہر دن سب کی اپنی فائلوں کی ایک نقل رکھتے ہیں، جو نظام کے سنیپ شاٹس سے الگ ہے۔

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Trindle نے نقل نہیں بنائی کیونکہ ڈسک تقریباً بھر چکی ہے۔

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = آج { $time } بجے

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = کل { $time } بجے

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date } کو

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = کمپیوٹر دوبارہ شروع نہیں ہوا۔ اس کی جگہ اسے حلقہ سے دوبارہ شروع کریں۔

# Said beside the Take a snapshot line while it works.
settings-snap-taking = سنیپ شاٹ لیا جا رہا ہے

# Said beside the set aside line while the space is freed.
settings-snap-freeing = جگہ خالی ہو رہی ہے…

# Said beside the set aside line once the space is freed.
settings-snap-freed = خالی ہو گئی

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = بحال ہو رہا ہے

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = جب آپ اگلی بار دوبارہ شروع کریں گے تو نظام ویسا ہو گا جیسا سنیپ شاٹ { $n } میں تھا۔ اس کے بعد جو کچھ بدلا وہ الگ رکھ دیا گیا ہے، ضائع نہیں ہوا۔

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = حذف ہو رہا ہے

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = حذف ہو گیا

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = آپ کے پاس ورڈ کا انتظار ہے…

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = ہر دن ایک نقل بنتی ہے

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = مزید نقلیں نہیں

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = رکھا گیا

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = صاف ہو رہا ہے

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = صاف ہو گیا

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] ابھی کوئی نقل نہیں
        [one] ایک نقل
       *[other] { $n } نقول
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = پہلی ایک دن کے اندر بنتی ہے۔

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = یہ { $size } لیتی ہیں، { $when } ناپا گیا۔

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = ان کی جگہ روزانہ ناپی جاتی ہے۔

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = سب ابھی صاف کریں

# Heading of the card about earlier versions of files.
settings-snap-earlier = پرانے ورژن

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle ہر دن سب کی فائلوں کی ایک نقل رکھ سکتا ہے۔ پھر آپ Folders سے فائل کا پرانا ورژن کھول سکتے ہیں، یا غلطی سے حذف ہوئی فائل واپس لا سکتے ہیں۔ نقل میں ہر شخص صرف وہی دیکھتا ہے جو وہ اس دن دیکھ سکتا تھا، اس لیے نجی ہوم فولڈر نجی رہتے ہیں۔ جو فائل اس وقت دوسروں کے لیے کھلی تھی وہ اس نقل میں ان کے لیے کھلی رہتی ہے جب تک نقل ہٹا نہ دی جائے۔ یہ نقلیں نظام کے سنیپ شاٹس سے الگ ہیں، اور کوئی ایک دوسرے کو نہیں بدلتا۔ یہ ڈسک کی جگہ لیتی ہیں۔ جو فائل آپ حذف کریں یا بدلیں وہ اپنی جگہ تب تک لیتی رہتی ہے جب تک اسے رکھنے والی آخری نقل ہٹا نہ دی جائے۔ نقلیں صاف کرنے سے جگہ فوراً سب کے لیے خالی ہو جاتی ہے، اور آپ اسے واپس نہیں کر سکتے۔ ڈسک تقریباً بھری ہو تو کوئی نقل نہیں بنتی۔ ان میں سے کچھ بھی بدلنے کے لیے ایڈمنسٹریٹر کا پاس ورڈ چاہیے۔

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = اپنی فائلوں کے پرانے ورژن رکھیں

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = حذف شدہ فائلیں { $time } تک رکھی جاتی ہیں، پھر جگہ خالی کر دی جاتی ہے۔

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = ہر دن کی نقل رکھیں

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = ہر دن کی نقل کتنی دیر رکھی جائے

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = سنیپ شاٹس کی جانچ ہو رہی ہے…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = سنیپ شاٹس پڑھے نہیں جا سکے۔

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = آپ سنیپ شاٹ { $n } سے شروع ہوئے ہیں، اس لیے اب آپ جو کچھ کریں گے وہ رکھا نہیں جائے گا۔

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] آپ { $n } سنیپ شاٹ پر واپس جا سکتے ہیں، جو { $when } کا ہے۔
       *[other] آپ { $n } سنیپ شاٹس پر واپس جا سکتے ہیں۔ سب سے نیا { $when } کا ہے۔
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = ابھی کوئی سنیپ شاٹ نہیں۔ ہر سافٹ ویئر تبدیلی سے پہلے ایک لیا جاتا ہے۔

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }، حذف کرنے تک رکھا گیا

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = یہ سنیپ شاٹ بحال کریں

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = یہ سنیپ شاٹ حذف کریں

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = ابھی دوبارہ شروع کریں

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = یہ ابھی ترتیب نہیں دیے گئے۔

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = سنیپ شاٹس ترتیب نہیں دیے گئے۔ trindle پیکیج دوبارہ انسٹال کرنے سے وہ ترتیب پا جاتے ہیں: sudo apt install --reinstall trindle۔

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = کوئی اور قسم

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = سنیپ شاٹس کے لیے Btrfs چاہیے۔ یہ کمپیوٹر { $fs } استعمال کرتا ہے۔

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = سنیپ شاٹس کے لیے نظام Btrfs پر ہونا چاہیے، اور یہ کمپیوٹر { $fs } استعمال کرتا ہے۔ سنیپ شاٹس پانے کے لیے Debian انسٹال کرتے وقت نظام کے لیے Btrfs چنیں۔

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = اس کمپیوٹر پر کوئی سنیپ شاٹ نہیں۔

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = کوئی سنیپ شاٹ نہیں

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = اس کمپیوٹر پر کوئی سنیپ شاٹ نہیں

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = لوڈ ہو رہا ہے…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = سنیپ شاٹس پڑھے نہیں جا سکے

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = ابھی کوئی نہیں

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = ہر سنیپ شاٹ پورے نظام کو اس حالت میں واپس لا سکتا ہے جو تب تھی، جب آپ اگلی بار دوبارہ شروع کریں۔ کسی کو بحال یا حذف کرنے کے لیے ایڈمنسٹریٹر کا پاس ورڈ چاہیے۔

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = ابھی سنیپ شاٹ لیں

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = اس کے لیے عام طور پر دوبارہ شروع کریں۔

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] بحالی کے بعد الگ رکھا گیا ایک پرانا سسٹم
       *[other] بحالیوں کے بعد الگ رکھے گئے { $n } پرانے سسٹم
    }

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = جو جگہ یہ لیتے ہیں اسے خالی کریں

# Heading of the card for taking a snapshot.
settings-snap-take-one = ایک لیں

# Help text of the Take one card (the information button).
settings-snap-take-about = کوئی ایسا کام آزمانے سے پہلے سنیپ شاٹ لیں جسے آپ واپس کرنا چاہ سکتے ہوں۔ جو سنیپ شاٹ آپ لیں وہ آپ کے حذف کرنے تک رکھے جاتے ہیں۔ اگر آپ سنیپ شاٹ سے شروع ہوئے ہوں تو پہلے عام طریقے سے دوبارہ شروع کریں، ورنہ آپ کا نیا سنیپ شاٹ رکھا نہیں جائے گا۔ ہر بحالی نظام کو اس سے ٹھیک پہلے کی حالت میں الگ رکھ دیتی ہے، اس لیے کچھ ضائع نہیں ہوتا۔ نظام کی فائلیں مقفل ہوں تو انہیں صرف انسٹال اور اپ ڈیٹ بدل سکتے ہیں، اور ہر ایک سے پہلے اور بعد میں ایک سنیپ شاٹ بنتا ہے۔ تالا بدلنے کے لیے ایڈمنسٹریٹر کا پاس ورڈ چاہیے۔

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = سنیپ شاٹ { $n } سے شروع ہوا

# Under the Started from snapshot heading.
settings-snap-nothing-kept = تبدیلیاں رکھی نہیں جائیں گی۔

# Heading of a line on the Before restoring card.
settings-snap-files-too = آپ کی فائلیں بھی سنیپ شاٹس میں ہیں

# Under the Your files are in the snapshots too heading.
# SAFETY: needs a human review before it ships
settings-snap-files-back = بحالی آپ کی فائلیں بھی واپس رکھ دیتی ہے۔

# Heading of the card about what to know before restoring.
# SAFETY: needs a human review before it ships
settings-snap-before = بحال کرنے سے پہلے

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = چونکہ آپ سنیپ شاٹ سے شروع ہوئے ہیں، اس لیے اب آپ جو کچھ کریں گے وہ رکھا نہیں جائے گا۔ نظام کو ایسا ہی رکھنے کے لیے فہرست میں اس کے ساتھ یہ سنیپ شاٹ بحال کریں دبائیں، پھر دوبارہ شروع کریں۔ اس کمپیوٹر پر ہوم فولڈر نظام کا حصہ ہو سکتا ہے۔ ایسا ہو تو سنیپ شاٹ بحال کرنے سے آپ کی اپنی فائلیں بھی اس وقت کی حالت میں واپس چلی جاتی ہیں، اس لیے پہلے جو کچھ نیا ہو اسے کہیں محفوظ جگہ کاپی کر لیں۔




# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = سسٹم فائلیں مقفل

# Under the System files locked switch.
settings-snap-locked-sub = انہیں صرف انسٹالیشن اور اپ ڈیٹس کھولتی ہیں۔

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = مقفل

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = کھلا ہوا
