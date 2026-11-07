
# machine translated (Urdu), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = ایک Flatpak فائل

# On an installed thing's line: it came from a .deb file.
store-from-deb = ایک .deb فائل

# On an installed thing's line: it is a plugin.
store-from-plugin = پلگ ان

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = ریلیز امیدوار

# Tip of the round button on an app's tile while it is being removed.
# SAFETY: needs a human review before it ships
store-list-removing = { $name } ہٹائی جا رہی ہے

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = { $name } انسٹال ہو رہی ہے

# Tip of the round button on an app's tile that opens the app.
store-list-open = { $name } کھولیں

# Tip of the round button on an app's tile that installs the app.
store-list-install = { $name } انسٹال کریں

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = انسٹال شدہ

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = دکھائیں

# How long is left when it is a minute or less.
store-left-minute = تقریباً ایک منٹ باقی ہے

# How long is left, in minutes.
store-left-minutes = تقریباً { $n } منٹ باقی ہیں

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] تقریباً 1 گھنٹہ باقی ہے
       *[other] تقریباً { $n } گھنٹے باقی ہیں
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = HaGeZi کی Threat Intelligence فہرست سے جانچا گیا

# Where a download came from: the site and the page that led to it.
store-origin-both = { $site } سے ڈاؤن لوڈ کیا گیا، { $referrer } کے ذریعے پہنچے۔

# Where a download came from: the site.
store-origin-site = { $site } سے ڈاؤن لوڈ کیا گیا۔

# Where a download came from: the page that led to it.
store-origin-referrer = { $referrer } کے ذریعے پہنچے۔

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle نہیں بتا سکتا کہ یہ کہاں سے آیا۔

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } نقصان دہ ڈاؤن لوڈز اور فراڈ کے لیے مشہور سائٹوں کی HaGeZi کی Threat Intelligence فہرست میں ہے۔ اسے صرف تب انسٹال کریں جب آپ کو یقین ہو کہ یہ کہاں سے آیا ہے۔

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } یہ سائٹ HaGeZi کی Threat Intelligence فہرست میں نہیں ہے، لیکن فہرست مکمل نہیں ہے، اس لیے اس سے یہ ثابت نہیں ہوتا کہ یہ محفوظ ہے۔

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } سائٹ کی جانچ نہیں ہو سکی، کیونکہ HaGeZi کی Threat Intelligence فہرست حاصل نہیں ہو سکی۔

# Name shown for a review whose writer gave none.
store-review-someone = کوئی صاحب

# Red words when the keyring could not be reached.
store-key-unreached = Keyring تک رسائی نہیں ہو سکی: { $why }۔

# Red words when the key kept in the keyring is not text.
store-key-not-text = محفوظ کی گئی کلید متن نہیں ہے۔

# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } ہٹائی نہیں جا سکی: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = { $path } پڑھی نہیں جا سکی: { $why }
