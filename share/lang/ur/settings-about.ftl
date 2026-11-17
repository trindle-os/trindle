
# machine translated (Urdu), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = اس کمپیوٹر کے بارے میں

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = ہوسٹ نام کمپیوٹر کا نام نیٹ ورک کا نام تعارف سسٹم ورژن debian trindle میموری ریم پروسیسر cpu ڈسک گرافکس اسکرین ماڈل کرنل سرگرمی چلتے پروگرام مصروف ٹاسک مینیجر سسٹم مانیٹر ڈرائیو usb جگہ

# Text behind the info button of the About page.
settings-about-help = یہ صفحہ اس کمپیوٹر کا نام دکھاتا ہے، جیسا نیٹ ورک پر دوسرے کمپیوٹر اسے دیکھتے ہیں، نیز یہ کہ وہ کس چیز سے بنا ہے اور شروع ہونے میں کتنا وقت لگتا ہے۔ سرگرمی میں دیکھ سکتے ہیں کہ کمپیوٹر اس وقت کیا کر رہا ہے اور کون سے پروگرام میموری اور پروسیسر استعمال کر رہے ہیں۔ ڈرائیوز میں آپ کی ڈرائیوز اور ان کی جگہ نظر آتی ہے۔ ہر ایک اپنی الگ ونڈو میں کھلتا ہے۔ انہیں کھولنے کے لیے مزید تفصیل کے تیر استعمال کریں۔

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = نامعلوم

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }، { $cores } تھریڈ
       *[other] { $model }، { $cores } تھریڈز
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel گرافکس

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD گرافکس

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA گرافکس

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = ورچوئل مشین گرافکس

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = گرافکس

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }، { $hz } Hz پر

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } استعمال میں

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } خالی

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = کل { $amount } میں سے

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }، { $on } سے چالو ہے۔

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }، { $on } سے چالو ہے۔ ڈسک تقریباً بھر چکی ہے۔

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } سیکنڈ

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } منٹ

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } منٹ
       *[other] { $n } منٹ
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } گھنٹہ
       *[other] { $n } گھنٹے
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } دن
       *[other] { $n } دن
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = صرف حروف، ہندسے اور ہائفن استعمال کریں۔ ہائفن سے شروع یا ختم نہ کریں۔

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = آپ کے پاس ورڈ کا انتظار ہے…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = کمپیوٹر کا نام تبدیل نہیں ہو سکا۔

# Words under the computer name box after renaming worked.
settings-about-name-done = پہلے سے کھلے پروگرام آپ کے اگلی بار لاگ ان کرنے کے بعد نیا نام استعمال کریں گے۔

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = کمپیوٹر کا نام

# Tip of the save button beside the computer name box.
settings-about-rename = اس کمپیوٹر کا نام تبدیل کریں

# Bold line of the computer name setting.
settings-about-name = کمپیوٹر کا نام

# Heading of the card with the computer's name.
settings-about-name-group = نام

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = یہ نیٹ ورک پر اور ٹرمینل کے پرامپٹ پر کمپیوٹر کا نام ہے۔ حروف، ہندسے اور ہائفن استعمال کریں، مگر ہائفن سے شروع یا ختم نہ کریں۔ اسے بدلنے کے لیے آپ کو اپنا پاس ورڈ لکھنا ہوگا۔

# Heading of the card about the computer's hardware.
settings-about-hardware = کمپیوٹر

# Text behind the info button of the hardware card.
settings-about-hardware-about = یہ کمپیوٹر کا بنانے والا اور ماڈل، پروسیسر اور گرافکس دکھاتا ہے۔ یہ بھی دکھاتا ہے کہ پروسیسر ایک وقت میں کتنے تھریڈ چلا سکتا ہے، اور استعمال میں ہر اسکرین اس کے سائز اور ریفریش ریٹ کے ساتھ۔

# Heading of the card about the software.
settings-about-software = سافٹ ویئر

# Text behind the info button of the software card.
settings-about-software-about = Trindle ایک ڈیسک ٹاپ ہے جو Debian کے اوپر چلتا ہے، اور Debian نیچے کے پروگراموں کی دیکھ بھال کرتا ہے۔ کرنل وہ حصہ ہے جو ہارڈ ویئر کے ساتھ کام کرتا ہے۔ Trindle کا اپنا کمپوزیٹر Wayland استعمال کر کے ڈیسک ٹاپ بناتا ہے۔

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle کا اپنا کمپوزیٹر

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } ایپ
       *[other] { $n } ایپس
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = مزید تفصیل

# Text behind the info button of the More detail card.
settings-about-more-about = سرگرمی میں دیکھ سکتے ہیں کہ کمپیوٹر اس وقت کیا کر رہا ہے اور کون سے پروگرام میموری اور پروسیسر استعمال کر رہے ہیں۔ ڈرائیوز میں ہر ڈرائیو، وہ کتنی بھری ہے اور کتنی صحت مند ہے، اور USB ڈرائیوز نظر آتی ہیں۔

# Bold line of the setting that opens Activity.
settings-about-activity = دیکھیں کیا چل رہا ہے

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = سرگرمی

# Bold line of the setting that opens Drives.
settings-about-drives = ڈرائیوز اور جگہ

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ڈرائیوز

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle شروع ہونے کے اوقات ڈاؤن لوڈز میں محفوظ نہیں کر سکا۔

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = تاریخ اور وقت,سیکنڈ

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle شروع ہونے کے اوقات.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = ڈاؤن لوڈز میں، { $file } کے نام سے

# Label of the word button that saves every start-up time.
settings-about-times-save = محفوظ کریں

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = شروع ہونے کے تمام اوقات ڈاؤن لوڈز میں محفوظ کریں

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = ہر آغاز کا وقت محفوظ کریں

# Quiet line under that setting.
settings-about-times-keep-sub = ڈاؤن لوڈز میں اسپریڈ شیٹ کی صورت میں محفوظ ہوتا ہے۔

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = ہر کالم ایک آغاز ہے۔ نیا ترین دائیں طرف ہے۔

# Heading of the card with the last ten start-up times.
settings-about-times = پچھلے دس آغاز

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle ہر آغاز کا وقت پاور بٹن سے لاگ ان اسکرین تک ناپتا ہے۔ ڈسک کا پاس ورڈ لکھنے میں لگا وقت شمار نہیں ہوتا۔ سب سے نیا اوپر ہے۔

# Heading of the memory tile.
settings-about-memory = میموری

# Heading of the disk tile.
settings-about-disk = ڈسک

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = مسئلوں کی رپورٹس

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = رپورٹس Buildmorphic کو جاتی ہیں، جو وہ برادری ہے جو Trindle بنانے میں مدد کرتی ہے۔ آپ صرف ای میل پتے سے شامل ہو سکتے ہیں۔ جن مسئلوں کی آپ سب سے پہلے اطلاع دیں، ان پر آپ کریڈٹ اور درجے کماتے ہیں، چاہے آپ انہیں ٹھیک نہ کر سکیں۔ رپورٹ میں صرف تفصیلات کی ایک طے شدہ فہرست ہوتی ہے، اور بھیجیں دبانے تک کچھ نہیں بھیجا جاتا۔ صارف نام یا GitHub نام اختیاری ہے، اور آپ کا صارف نام کبھی آپ کا ای میل پتہ نہیں ہوتا۔

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = کچھ غلط ہونے پر رپورٹ بھیجنے کی پیشکش کریں

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic صارف نام

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub نام

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = برادری کے بارے میں

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] بند۔ { $n } منتظر رپورٹ مٹا دی گئی۔
       *[other] بند۔ { $n } منتظر رپورٹیں مٹا دی گئیں۔
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] بند۔ { $n } منتظر رپورٹ مٹائی نہیں جا سکی۔
       *[other] بند۔ { $n } منتظر رپورٹیں مٹائی نہیں جا سکیں۔
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = اپ ڈیٹس

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle لاگ ان کے کچھ ہی دیر بعد، پھر ہر چند گھنٹے بعد، ایپس، پلگ ان، فرم ویئر اور اپنے نئے ورژن دیکھتا ہے۔ اگر آپ اسے بند کر دیں تو Trindle صرف تب دیکھتا ہے جب آپ ایپس میں "اپ ڈیٹس کی جانچ کریں" یا حلقے پر اپ ڈیٹس کا بٹن دباتے ہیں۔ Debian کی سکیورٹی اصلاحات ہر روز لگتی ہیں اور ان کا اپنا سوئچ ہے۔ اگر آپ وہ بند کر دیں تو وہ خود نہیں لگتیں، اور معلوم سکیورٹی خامیاں آپ کے اپ ڈیٹ کرنے تک دور نہیں ہوتیں۔ کوئی بھی سوئچ بدلنے کے لیے منتظم کا پاس ورڈ چاہیے، تاکہ پورا کمپیوٹر آپ کا انتخاب استعمال کرے۔

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = اپ ڈیٹس کی جانچ کریں

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = آپ کے پاس ورڈ کا انتظار ہے…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = سکیورٹی اپ ڈیٹس

# Warning line under the Security updates switch when it is off.
settings-about-security-off = سکیورٹی اصلاحات اب خود نہیں لگتیں۔

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = بند، لیکن Reporting ایڈ آن کی منتظر رپورٹیں مٹائی نہیں گئیں۔
