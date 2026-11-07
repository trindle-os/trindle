
# machine translated (Urdu), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = اس کی سیٹنگز کھولیں

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = نہیں ہوا: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = اس اپ ڈیٹ کو ایک نئی اجازت چاہیے۔ یہ آپ کا انتظار کر رہی ہے۔

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = ابھی ورژن { $mine } ہے، { $new } تیار ہے

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = ابھی ورژن { $mine } ہے، { $new } تیار ہے۔ یہ پہلے سے زیادہ مانگتا ہے: { $more }۔ { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = جب یہ ختم کر لے گا تو یہاں کے بٹن واپس آ جائیں گے۔

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = پلگ اِن تلاش ہو رہے ہیں…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = کوئی پلگ اِن انسٹال نہیں ہے، اور ابھی کوئی پیش بھی نہیں کیا گیا۔

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] ابھی کوئی پلگ اِن انسٹال نہیں ہے؛ 1 انسٹال کے لیے تیار ہے۔
       *[other] ابھی کوئی پلگ اِن انسٹال نہیں ہے؛ { $n } انسٹال کے لیے تیار ہیں۔
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 پلگ اِن انسٹال ہے
       *[other] { $n } پلگ اِن انسٹال ہیں
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 کی اپ ڈیٹ موجود ہے
       *[other] { $n } کی اپ ڈیٹ موجود ہے
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 اپ ڈیٹ روکی گئی ہے
       *[other] { $n } اپ ڈیٹس روکی گئی ہیں
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }؛ { $held }۔

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }۔

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }، سب تازہ ترین۔

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }، اور { $have }۔

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }، اور { $have }؛ { $held }۔

# On a plugin's card when it asks for nothing.
# SAFETY: needs a human review before it ships
store-plugins-asks-nothing = یہ کچھ نہیں مانگتا۔

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = یہ جو مانگتا ہے: { $list }۔

# Title of the Plugins page.
store-plugins-title = پلگ اِن

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = ورژن { $mine } انسٹال ہے؛ { $new } تیار ہے۔

# On a plugin's card: its installed version.
store-plugins-version-installed = ورژن { $version }، انسٹال ہے۔

# On a plugin's card: the version on offer.
store-plugins-version-offered = ورژن { $version }۔

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }۔

# Tip of the round button that installs a plugin.
# SAFETY: needs a human review before it ships
store-plugins-install = { $name } انسٹال کریں

# Tip of the round button that releases a plugin whose updates are held back.
# SAFETY: needs a human review before it ships
store-plugins-release = { $name } کو چھوڑیں

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = { $version } پر اپ ڈیٹ کریں

# Heading of the card that says where plugins come from.
store-plugins-where = پلگ اِن کہاں سے آتے ہیں

# Information on the card that says where plugins come from.
# SAFETY: needs a human review before it ships
store-plugins-come-from = پلگ اِن صرف Trindle کی اپنی فہرست سے آتے ہیں، جو اس صفحے کے ہر بار کھلنے پر trindle.org سے پڑھی جاتی ہے۔ کوئی پیش کرنے کے لیے اسے جمع کرائیں: ہر ایک کے چلنے سے پہلے اس کی جانچ ہوتی ہے۔

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = یہ انسٹال ہے، مگر اس کا پس منظر کا حصہ نہیں چل رہا: { $why }

# Red words under a plugin that could not be removed; the reason follows.
# SAFETY: needs a human review before it ships
store-plugins-not-removed = اسے ہٹایا نہیں جا سکا: { $why }
