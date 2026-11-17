
# machine translated (Urdu), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = محفوظ ہو گیا

# Said beside a button that worked, when nothing more is known.
settings-common-done = ہو گیا

# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = محفوظ|ہو گیا|اب استعمال میں|بدل گیا|مقرر|لے لیا|شامل کیا|آزاد کیا|ہٹا دیا|رکھ لیا|انسٹال ہو گیا|اپ ڈیٹ ہو گیا|شامل ہو گیا|نقل ہو گیا|اب ڈیفالٹ|چلایا گیا|بھیج دیا|جڑ گیا|جڑ رہا ہے|آن|آف|بھلا دیا|80% پر رک جاتا ہے|مکمل چارج ہوتا ہے|لاگ ان پر شروع ہوتا ہے|رکھا ہے، لیکن شروع نہیں ہوتا|تازہ ترین ہے

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = نہیں ہوا

# Tip of the information button at the top of every Settings page.
settings-common-about-page = اس صفحے کے بارے میں

# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = { $name } کے بارے میں

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } پہلے سے موجود ہے

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] باقی کے لیے بھی یہی کریں ({ $n } اور)
       *[other] باقی کے لیے بھی یہی کریں ({ $n } اور)
    }

# SAFETY: needs a human review before it ships
# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = باقی کے لیے بھی یہی کریں

# SAFETY: needs a human review before it ships
# Tip of the button that keeps the one already there.
settings-common-skip = چھوڑیں اور موجودہ کو رکھیں

# SAFETY: needs a human review before it ships
# Button that replaces the one already there.
settings-common-replace = بدل دیں

# SAFETY: needs a human review before it ships
# Spoken name of the Replace button.
settings-common-replace-spoken = موجودہ کو نئی سے بدل دیں

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = نئی

# Heading of the line that shows the one already there.
settings-common-already-line = جو پہلے سے موجود ہے

# SAFETY: needs a human review before it ships
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = جو پہلے سے موجود ہے اسے بدلیں؟

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] { $left } سیکنڈ میں واپس ہو جائے گا۔
       *[other] { $left } سیکنڈ میں واپس ہو جائے گا۔
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] { $left } سیکنڈ میں واپس ہو جائے گا۔ { $more }
       *[other] { $left } سیکنڈ میں واپس ہو جائے گا۔ { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = ابھی واپس جائیں

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = رکھیں

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } دستیاب نہیں ہے

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = اسے انسٹال کرنے کے لیے یہ چلائیں: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Trindle ویب صفحہ نہیں کھول سکا، کیونکہ وہ ویب براؤزر شروع نہیں کر سکا۔

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } کھولا نہیں جا سکا۔

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = یہ محفوظ نہیں ہو سکا۔

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Trindle یہ نہیں کر سکا کیونکہ سسٹم کا ایک حصہ جواب نہیں دے رہا۔ کمپیوٹر دوبارہ شروع کریں، پھر کوشش کریں۔

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Trindle یہ نہیں کر سکا کیونکہ آپ کے لاگ ان کا ایک حصہ جواب نہیں دے رہا۔ لاگ آؤٹ کر کے دوبارہ لاگ ان کریں، پھر کوشش کریں۔

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Trindle فائل یا فولڈر چننے کی ونڈو نہیں کھول سکا۔ لاگ آؤٹ کر کے دوبارہ لاگ ان کریں، پھر کوشش کریں۔

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = ڈسک بھر گئی ہے۔

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Trindle ابھی ڈسک پر نہیں لکھ سکتا۔

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = سسٹم کا جو حصہ یہ کرتا ہے وہ نہیں چل رہا۔

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = جواب میں بہت دیر لگی۔ دوبارہ کوشش کریں۔

# SAFETY: needs a human review before it ships
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = پاس ورڈ نہیں دیا گیا۔

# SAFETY: needs a human review before it ships
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = یہ اکاؤنٹ ایسا نہیں کر سکتا۔

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = اس کمپیوٹر پر اسے درکار کوئی چیز موجود نہیں ہے۔

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } اور { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { "، " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = آپ اسکرین کے کنارے پر ڈبل کلک کر کے حلقہ کھولتے ہیں۔

# Word drawn in the middle of that picture.
settings-common-ring-name = حلقہ

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Trindle لیپ ٹاپ کے چارجنگ کنٹرولز تک نہیں پہنچ سکا: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = لیپ ٹاپ نے چارج کی حد قبول نہیں کی۔

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = یہ کمپیوٹر آپ کو چارجنگ جلدی روکنے نہیں دیتا۔

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path } مقرر نہیں ہو سکا: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = ورچوئل مشین میں Trindle آزمانا

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle { $machine } میں چل رہا ہے۔ وہاں یہ اچھا چلتا ہے، لیکن ورچوئل مشین کبھی اصل کمپیوٹر جتنی تیز اور رواں نہیں ہوتی۔ اس کی رفتار اصل ہارڈ ویئر پر پرکھیں۔

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle { $machine } میں 3D گرافکس کے بغیر چل رہا ہے۔ پروسیسر کو وہ سب کچھ بنانا پڑتا ہے جو عام طور پر گرافکس چپ بناتی ہے۔ ونڈوز، ویڈیوز اور ویب صفحات اصل کمپیوٹر سے سست لگیں گے، اس لیے Trindle اپنی حرکت پذیری چھوڑ دیتا ہے۔ اگر جس کمپیوٹر پر یہ چل رہا ہے وہ کر سکے تو ورچوئل مشین کی ڈسپلے سیٹنگز میں 3D ایکسلریشن چالو کریں۔ اس سے بڑا فرق پڑتا ہے۔ اگر آپ Trindle اصل کمپیوٹر پر انسٹال کریں تو اس میں سے کچھ بھی لاگو نہیں ہوتا۔

# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } یا { $last }
