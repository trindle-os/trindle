
# machine translated (Urdu), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = کچھ بھی واپس نہیں رکھا گیا، کیونکہ { $why }۔

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = بیک اپ کا کچھ حصہ واپس نہیں رکھا گیا، کیونکہ { $why }۔

# SAFETY: needs a human review before it ships
# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = بیک اپ میں ایک لنک یا کوئی اور چیز ہے جو اس فولڈر سے باہر پہنچے گی جس میں اسے رکھا جا رہا ہے

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = اس کمپیوٹر پر کوئی چیز پہلے ہی وہاں ہے جہاں بیک اپ کے ایک حصے کو جانا ہے

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = بیک اپ کھولنے کے لیے کوئی جگہ نہیں ہے

# The reason when the backup file cannot be read.
store-because-unreadable = بیک اپ فائل پڑھی نہیں جا سکی

# The reason when the file is not a Trindle backup.
store-because-not-trindle = وہ فائل Trindle کا بیک اپ نہیں ہے

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = بیک اپ خراب ہے اور اس کی فہرست پڑھی نہیں جا سکی

# The reason when the file is not a Flat backup.
store-because-not-flat = وہ فائل Flat کے بیک اپ جیسی نہیں لگتی

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = اس فائل میں Flat کی ایپس کی فہرست نہیں ہے

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = یہ Flat کے ایسے نئے ورژن نے بنایا جسے Trindle پڑھ نہیں سکتا

# The reason when the file holds no apps.
store-because-no-apps = اس فائل میں کوئی ایپ نہیں ہے

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = بیک اپ پڑھا نہیں جا سکا: { $why }

# The reason when the program gave none.
store-because-none = اس نے کوئی وجہ نہیں بتائی

# SAFETY: needs a human review before it ships
# The reason when the system said permission was denied.
store-because-permission = یہ کمپیوٹر Trindle کو وہاں لکھنے نہیں دے گا

# The reason when the disk is full.
store-because-space = اس کمپیوٹر پر جگہ باقی نہیں رہی

# The reason when a part of the computer is read-only.
store-because-read-only = اس کمپیوٹر کا وہ حصہ صرف پڑھنے کے لیے ہے

# The reason when the person's disk quota is used up.
store-because-quota = اس اکاؤنٹ کے لیے دی گئی جگہ ختم ہو گئی ہے

# The reason when a file or program is missing.
store-because-missing = جو فائل یا پروگرام درکار ہے وہ موجود نہیں

# The reason when no reason could be read from the program's last line.
store-because-failed = ناکام ہو گیا

# The reason when the backup has no name to be written under.
store-because-no-name = اس کا کوئی نام نہیں

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = بیک اپ میں اس کی ترتیبات اس کے اپنے فولڈر سے باہر پہنچتی ہیں

# The reason when something is too big to be read whole.
store-because-too-big = یہ اتنا بڑا ہے کہ پورا پڑھا نہیں جا سکتا

# SAFETY: needs a human review before it ships
# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = بیک اپ میں اس کی ترتیبات اپنے چیک سم سے نہیں ملتیں

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = واپس رکھ دیا گیا، سوائے آپ کے اکاؤنٹ کی تصویر کے۔ { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } آپ کے اکاؤنٹ کی تصویر بھی واپس نہیں رکھی گئی۔ { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec شروع نہیں ہو سکا: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = اسے بیک اپ سے پڑھا نہیں جا سکا: { $why }۔

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = بیک اپ نہیں بنا، کیونکہ { $why }۔

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = ابھی کچھ نہیں

# Beside the size of a part of the backup: about how big it is.
store-backup-about = تقریباً { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindle کی ترتیبات اور AppImages واپس رکھی جا رہی ہیں...

# Step shown while the settings and data are put back.
store-backup-step-data = ترتیبات اور ڈیٹا واپس رکھا جا رہا ہے...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Flathub سے { $count } ایپ انسٹال ہو رہی ہے... اس میں کچھ وقت لگ سکتا ہے۔
       *[other] Flathub سے { $count } ایپس انسٹال ہو رہی ہیں... اس میں کچھ وقت لگ سکتا ہے۔
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = ترتیبات اور ڈیٹا واپس رکھ دیے گئے، مگر یہ ایپس انسٹال نہیں ہوئیں، کیونکہ Flatpak نے انہیں قبول نہیں کیا: { $failed }۔

# The same, when other apps were also left out for another reason.
store-not-installed-both = ترتیبات اور ڈیٹا واپس رکھ دیے گئے، مگر یہ ایپس انسٹال نہیں ہوئیں، کیونکہ Flatpak نے انہیں قبول نہیں کیا: { $failed }۔ یہ بھی انسٹال نہیں ہوئیں: { $refused }۔

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = ترتیبات اور ڈیٹا واپس رکھ دیے گئے، مگر یہ ایپس انسٹال نہیں ہوئیں: { $refused }۔

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] 1 ایپ چھوڑ دی گئی: اس کا نام درست نہیں تھا۔
       *[other] { $n } ایپس چھوڑ دی گئیں: ان کے نام درست نہیں تھے۔
    }

# SAFETY: needs a human review before it ships
# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = { $name } ریموٹ خفیہ کردہ نہیں ہے اور قبول نہیں کیا گیا، اس لیے استعمال نہیں ہوا

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } ریموٹ اس کمپیوٹر پر نہیں ہے اور بیک اپ یہ نہیں بتاتا کہ وہ کہاں ہے

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name } کی ترتیبات (کیونکہ بیک اپ میں ان کی نقل جانچ میں پاس نہیں ہوئی)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (کیونکہ اس کے ماخذ کا نام درست نہیں)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (کیونکہ { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (کیونکہ Flatpak نے اسے قبول نہیں کیا: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = سب کچھ (کیونکہ بیک اپ کھولنے کے لیے کوئی جگہ نہیں)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = جو کچھ ہر ایپ کو کرنے کی اجازت ہے

# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name } کی ترتیبات

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }۔ جو پہلے وہاں تھا وہ { $path } کے طور پر رکھا گیا ہے: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } انسٹال ہو رہی ہے ({ $total } میں سے { $at })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = { $name } کی ترتیبات واپس رکھی جا رہی ہیں ({ $total } میں سے { $at })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } ایپ انسٹال ہوئی۔
       *[other] { $n } ایپس انسٹال ہوئیں۔
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = { $n } کی ترتیبات واپس رکھی گئیں۔

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } پہلے سے موجود ہے، جیسی تھی ویسی چھوڑ دی گئی۔
       *[other] { $n } پہلے سے موجود ہیں، جیسی تھیں ویسی چھوڑ دی گئیں۔
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = واپس نہیں رکھا گیا: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })
