
# machine translated (Urdu), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = سنیپ شاٹس

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = سنیپ شاٹس سنیپ شاٹ بحال واپس رول بیک بیک اپ اپ ڈیٹ خراب btrfs snapper بوٹ مینو grub پرانے ورژن پچھلا ورژن حذف شدہ فائل واپس رکھنا بازیافت ہوم نقول

# Help line at the foot of the Snapshots page.
settings-snap-help = ہر سافٹ ویئر تبدیلی سے پہلے اور بعد، اور جب بھی آپ ایک لیں، نظام جیسا تھا۔ بحال کرنے سے اگلے آغاز سے پورا نظام اس حالت میں واپس آ جاتا ہے؛ آپ کی اپنی فائلیں جیسی ہیں ویسی رہتی ہیں اگر ہوم فولڈر کے لیے ڈسک کا اپنا حصہ ہو، جیسا Trindle کا انسٹالر بناتا ہے۔ اگر کمپیوٹر شروع نہ ہو تو بوٹ مینو میں Trindle snapshots چنیں۔ پرانے ورژن ہر دن سب کی اپنی فائلوں کی نقل رکھتے ہیں، جو نظام کے سنیپ شاٹس سے الگ ہے۔

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = کوئی نقل نہیں بنی: ڈسک تقریباً بھر چکی ہے۔

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
settings-snap-freeing = جگہ خالی ہو رہی ہے...

# Said beside the set aside line once the space is freed.
settings-snap-freed = خالی ہو گئی

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = بحال ہو رہا ہے

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = اگلے آغاز سے نظام سنیپ شاٹ { $n } ہو گا۔ اس کے بعد جو کچھ بدلا وہ الگ رکھ دیا گیا ہے، ضائع نہیں ہوا۔

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = حذف ہو رہا ہے

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = حذف ہو گیا

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = پاس ورڈ کا انتظار ہے...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = ہر دن ایک نقل بنتی ہے

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = مزید نہیں بنتیں

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
settings-snap-measured-daily = ان کی جگہ دن میں ایک بار ناپی جاتی ہے۔

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = سب ابھی صاف کریں

# Heading of the card about earlier versions of files.
settings-snap-earlier = پرانے ورژن

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle ہر دن سب کی فائلوں کی نقل جیسی وہ تھیں رکھ سکتا ہے، تاکہ فائل کا پرانا ورژن، یا غلطی سے حذف ہوئی فائل، فولڈرز سے کھولی یا واپس رکھی جا سکے۔ ہر شخص نقل میں صرف وہی پڑھ سکتا ہے جو وہ اس دن پڑھ سکتا تھا، اس لیے نجی ہوم نجی رہتا ہے؛ جو فائل اس وقت دوسروں کے لیے کھلی تھی وہ اس نقل میں ان کے لیے کھلی رہتی ہے جب تک نقل ختم نہ ہو۔ یہ نقول نظام کے سنیپ شاٹس سے الگ ہیں: نظام بحال کرنے سے آپ کی فائلیں کبھی نہیں بدلتیں، اور یہ نظام کو کبھی نہیں بدلتیں۔ قیمت جگہ ہے: جو فائل آپ حذف کریں یا بدلیں وہ اپنی جگہ تب تک لیتی رہتی ہے جب تک اسے رکھنے والی آخری نقل ختم نہ ہو، اس لیے چیزیں حذف کرنے سے ڈسک فوراً خالی نہیں ہوتی۔ نقول صاف کرنے سے یہ ابھی خالی ہو جاتی ہے، اس کمپیوٹر کے ہر شخص کے لیے، اور اسے واپس نہیں کیا جا سکتا۔ ڈسک تقریباً بھری ہو تو کوئی نقل نہیں بنتی۔ ان میں سے کچھ بھی بدلنے کے لیے ایڈمنسٹریٹر کا پاس ورڈ چاہیے۔

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = اپنی فائلوں کے پرانے ورژن رکھیں

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = حذف شدہ فائلیں جگہ خالی ہونے سے پہلے { $time } تک رہتی ہیں۔

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = ہر دن کی نقل رکھیں

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = ہر دن کی نقل کتنی دیر رکھی جائے

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = سنیپ شاٹس دیکھے جا رہے ہیں…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = سنیپ شاٹس پڑھے نہیں جا سکے۔

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = سنیپ شاٹ { $n } سے شروع ہوا، اس لیے اب جو کچھ کیا جائے وہ رکھا نہیں جائے گا۔

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] واپس جانے کے لیے { $n } سنیپ شاٹ، نیا ترین { $when } کا۔
       *[other] واپس جانے کے لیے { $n } سنیپ شاٹس، نیا ترین { $when } کا۔
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = ابھی کوئی سنیپ شاٹ نہیں۔ ہر سافٹ ویئر تبدیلی سے پہلے ایک لیا جاتا ہے۔

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }، حذف کرنے تک رکھا گیا

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = نظام کو اس حالت میں بحال کریں

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = یہ سنیپ شاٹ حذف کریں

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = ابھی دوبارہ شروع کریں

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = یہ ابھی ترتیب نہیں دیے گئے۔

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = trindle پیکیج دوبارہ انسٹال کریں۔

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = سنیپ شاٹس ترتیب نہیں دیے گئے۔ trindle پیکیج دوبارہ انسٹال کرنے سے وہ ترتیب پا جاتے ہیں: sudo apt install --reinstall trindle۔

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = کوئی اور قسم

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = انہیں Btrfs چاہیے؛ یہ کمپیوٹر { $fs } استعمال کرتا ہے۔

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Debian انسٹال کرتے وقت Btrfs چنیں۔

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = سنیپ شاٹس کے لیے نظام Btrfs پر ہونا چاہیے، اور یہ کمپیوٹر { $fs } استعمال کرتا ہے۔ انہیں پانے کے لیے Debian انسٹال کرتے وقت نظام کے لیے Btrfs چنیں۔

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = اس کمپیوٹر پر کوئی سنیپ شاٹ نہیں۔

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = کوئی سنیپ شاٹ نہیں

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = اس کمپیوٹر پر کوئی سنیپ شاٹ نہیں

# Heading of the line saying how to have snapshots.
settings-snap-to-have = انہیں پانے کے لیے

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = تلاش جاری…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = سنیپ شاٹس پڑھے نہیں جا سکے

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = ابھی کوئی نہیں

# Under the None yet heading.
settings-snap-none-line-sub = ہر سافٹ ویئر تبدیلی سے پہلے ایک لیا جاتا ہے۔

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = ہر سنیپ شاٹ پورے نظام کو اگلے آغاز سے اس حالت میں بحال کر سکتا ہے جو تب تھی، یا حذف کیا جا سکتا ہے۔ دونوں کے لیے دو بار دبانا پڑتا ہے اور ایڈمنسٹریٹر کا پاس ورڈ چاہیے۔ بحالی کے بعد اس کے ساتھ دوبارہ شروع کرنے کا بٹن آتا ہے۔

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = ابھی سنیپ شاٹ لیں

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = سنیپ شاٹ سے شروع ہونے کی حالت میں نہیں۔

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = آپ کے حذف کرنے تک رکھا جاتا ہے۔

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] بحالی سے ایک نظام الگ رکھا گیا
       *[other] بحالی سے { $n } نظام الگ رکھے گئے
    }

# Under the set aside heading.
settings-snap-free-space = سب ٹھیک ہو جائے تو جگہ خالی کریں۔

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = جو جگہ یہ لیتے ہیں اسے خالی کریں

# Heading of the card for taking a snapshot.
settings-snap-take-one = ایک لیں

# Help text of the Take one card (the information button).
settings-snap-take-about = کوئی ایسا کام آزمانے سے پہلے سنیپ شاٹ لیں جسے آپ واپس کرنا چاہ سکتے ہوں؛ جو آپ لیں وہ آپ کے حذف کرنے تک رکھا جاتا ہے۔ جب کمپیوٹر سنیپ شاٹ سے شروع ہوا ہو تو لیا گیا کچھ نہیں رکھا جاتا، اس لیے پہلے عام طور پر دوبارہ شروع کریں۔ ہر بحالی نظام کو اس سے ٹھیک پہلے کی حالت میں الگ رکھ دیتی ہے، اس لیے کچھ ضائع نہیں ہوتا؛ سب ٹھیک ہو جائے تو ان کی لی ہوئی جگہ خالی کریں۔

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = سنیپ شاٹ { $n } سے شروع ہوا

# Under the Started from snapshot heading.
settings-snap-nothing-kept = اب جو کچھ کیا جائے وہ رکھا نہیں جاتا۔

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
settings-snap-before-about = سنیپ شاٹ سے شروع ہونے پر اب جو کچھ کیا جائے وہ رکھا نہیں جاتا: اگر نظام کو اسی حالت میں ہونا چاہیے تو فہرست میں اس پر بحال کریں دبائیں، پھر دوبارہ شروع کریں۔ اس کمپیوٹر پر ہوم فولڈر نظام کا حصہ ہو سکتا ہے، اور تب سنیپ شاٹ بحال کرنے سے آپ کی اپنی فائلیں بھی اس وقت کی حالت میں واپس چلی جاتی ہیں، اس لیے پہلے جو کچھ نیا ہو اسے کہیں محفوظ جگہ کاپی کر لیں۔




# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = سسٹم فائلیں مقفل

# Under the System files locked switch.
settings-snap-locked-sub = انہیں صرف انسٹالیشن اور اپ ڈیٹس کھولتی ہیں۔

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = مقفل

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = کھلا ہوا

