
# machine translated (Urdu), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = اس کمپیوٹر کے بارے میں

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = ہوسٹ نام کمپیوٹر کا نام نیٹ ورک کا نام تعارف سسٹم ورژن debian trindle میموری ریم پروسیسر cpu ڈسک گرافکس اسکرین ماڈل کرنل سرگرمی چلتے پروگرام مصروف ٹاسک مینیجر سسٹم مانیٹر ڈرائیو usb جگہ

# Text behind the info button of the About page.
settings-about-help = اس کمپیوٹر کا نام جیسا نیٹ ورک پر دوسرے کمپیوٹر دیکھتے ہیں، یہ کس چیز سے بنا ہے، اور شروع ہونے میں کتنا وقت لگتا ہے۔ یہ اس وقت کیا کر رہا ہے، اور کون سے پروگرام میموری اور پروسیسر استعمال کر رہے ہیں، یہ سرگرمی میں ہے؛ ڈرائیوز اور ان کی جگہ ڈرائیوز میں ہے، دونوں الگ ونڈوز ہیں: مزید تفصیل کے تیر انہیں کھولتے ہیں۔

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
settings-about-name-bad = صرف حروف، ہندسے اور ہائفن، ہائفن سے شروع یا ختم نہ ہو۔

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = پاس ورڈ کا انتظار ہے...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = کمپیوٹر کا نام تبدیل نہیں ہو سکا۔

# Words under the computer name box after renaming worked.
settings-about-name-done = پہلے سے کھلے پروگرام اگلے لاگ ان کے بعد نیا نام دیکھیں گے۔

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = کمپیوٹر کا نام

# Tip of the save button beside the computer name box.
settings-about-rename = اس کمپیوٹر کا نام تبدیل کریں

# Bold line of the computer name setting.
settings-about-name = کمپیوٹر کا نام

# Quiet line under the computer name setting.
settings-about-name-sub = حروف، ہندسے اور ہائفن۔

# Heading of the card with the computer's name.
settings-about-name-group = نام

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = یہ کمپیوٹر نیٹ ورک پر اور ٹرمینل کے پرامپٹ پر کیسا نظر آتا ہے۔ حروف، ہندسے اور ہائفن، ہائفن سے شروع یا ختم نہ ہو۔ اسے بدلنے کے لیے آپ کا پاس ورڈ پوچھا جاتا ہے۔

# Heading of the card about the computer's hardware.
settings-about-hardware = کمپیوٹر

# Text behind the info button of the hardware card.
settings-about-hardware-about = بنانے والا اور ماڈل، پروسیسر اور وہ ایک وقت میں کتنے تھریڈ چلاتا ہے، گرافکس، اور استعمال میں ہر اسکرین اس کے سائز اور ریفریش ریٹ کے ساتھ۔

# Quiet line under the maker and model.
settings-about-hw-computer = کمپیوٹر

# Quiet line under the processor.
settings-about-hw-processor = پروسیسر

# Quiet line under the graphics.
settings-about-hw-graphics = تصویریں بناتا ہے

# Quiet line under the screens.
settings-about-hw-screen = اسکرین

# Heading of the card about the software.
settings-about-software = سافٹ ویئر

# Text behind the info button of the software card.
settings-about-software-about = Trindle، Debian کے اوپر بنا ایک ڈیسک ٹاپ ہے، جو نیچے کے پروگراموں کی دیکھ بھال کرتا ہے۔ کرنل وہ حصہ ہے جو ہارڈ ویئر سے بات کرتا ہے، اور Trindle کا اپنا کمپوزیٹر Wayland پر ڈیسک ٹاپ بناتا ہے۔

# Quiet line under the Debian version.
settings-about-sw-debian = نیچے چلتا ہے

# Quiet line under the kernel version.
settings-about-sw-kernel = کرنل

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle کا اپنا کمپوزیٹر

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Wayland پر ڈیسک ٹاپ بناتا ہے

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } ایپ
       *[other] { $n } ایپس
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = انسٹال شدہ، حلقہ میں تیار

# Heading of the card with the ways to Activity and Drives.
settings-about-more = مزید تفصیل

# Text behind the info button of the More detail card.
settings-about-more-about = یہ اس وقت کیا کر رہا ہے، اور کون سے پروگرام میموری اور پروسیسر استعمال کر رہے ہیں، یہ سرگرمی میں ہے۔ ہر ڈرائیو، وہ کتنی بھری ہے اور کتنی صحت مند ہے، اور USB ڈرائیوز، ڈرائیوز میں ہیں۔

# Bold line of the setting that opens Activity.
settings-about-activity = دیکھیں کیا چل رہا ہے

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = میموری اور پروسیسر استعمال کرنے والے پروگرام۔

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = سرگرمی

# Bold line of the setting that opens Drives.
settings-about-drives = ڈرائیوز اور جگہ

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = ہر ڈرائیو، وہ کتنی بھری ہے اور کتنی صحت مند ہے۔

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ڈرائیوز

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = شروع ہونے کے اوقات ڈاؤن لوڈز میں محفوظ نہیں ہو سکے۔

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = ڈاؤن لوڈز میں، { $file } کے نام سے

# Label of the word button that saves every start-up time.
settings-about-times-save = محفوظ کریں

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = شروع ہونے کے تمام اوقات ڈاؤن لوڈز میں محفوظ کریں

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = ہر آغاز، محفوظ رکھنے کے لیے

# Quiet line under that setting.
settings-about-times-keep-sub = ڈاؤن لوڈز میں اسپریڈ شیٹ کی صورت میں محفوظ ہوتا ہے۔

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = ہر کالم ایک آغاز ہے، نیا ترین دائیں طرف۔

# Heading of the card with the last ten start-up times.
settings-about-times = پچھلے دس آغاز

# Text behind the info button of the start-up times card.
settings-about-times-about = ہر آغاز کا وقت پاور بٹن سے لاگ ان اسکرین تک ناپا جاتا ہے، ڈسک کا پاس ورڈ لکھنے میں لگا وقت شامل نہیں ہوتا۔ سب سے نیا اوپر ہے۔

# Heading of the memory tile.
settings-about-memory = میموری

# Heading of the disk tile.
settings-about-disk = ڈسک


# Heading of the card with the switch for looking for updates.
settings-about-updates = اپ ڈیٹس

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle لاگ ان کے کچھ ہی دیر بعد اور پھر ہر چند گھنٹے بعد ایپس، پلگ ان، فرم ویئر اور اپنے نئے ورژن دیکھتا ہے، اور Debian کی سکیورٹی اصلاحات ہر روز لگتی ہیں۔ بند کرنے پر Trindle صرف تب دیکھتا ہے جب آپ ایپس میں "اپ ڈیٹس کی جانچ کریں" یا حلقے پر اپ ڈیٹس کا بٹن دباتے ہیں۔ Debian کی سکیورٹی اصلاحات کا اپنا سوئچ ہے؛ بند ہونے پر وہ خود نہیں لگتیں اور معلوم خرابیاں آپ کے اپ ڈیٹ کرنے تک کھلی رہتی ہیں۔ کوئی بھی سوئچ بدلنے پر منتظم کا پاس ورڈ مانگا جاتا ہے، تاکہ کمپیوٹر کی اپنی سروسز بھی یہ انتخاب دیکھ سکیں۔

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = اپ ڈیٹس کی جانچ کریں

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = پاس ورڈ کا انتظار ہے...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = بند ہے، آپ کے کہے بغیر کچھ نہیں دیکھا جاتا۔



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = سکیورٹی اپ ڈیٹس

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = سکیورٹی اصلاحات ہر روز خود لگتی ہیں۔

# Warning line under the Security updates switch when it is off.
settings-about-security-off = سکیورٹی اصلاحات اب خود نہیں لگتیں۔

