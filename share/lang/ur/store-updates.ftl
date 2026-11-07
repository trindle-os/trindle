
# machine translated (Urdu), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian کے پیکجز

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = ورژن { $have } پر روکا گیا؛ { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = روکا گیا؛ { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = ورژن { $have } پر روکا گیا

# On a thing held back with no update waiting.
store-upd-held = روکا گیا

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = ورژن { $have } پر روکا گیا؛ ابھی کچھ نیا نہیں

# On the Held back card: nothing newer waits.
store-upd-held-nothing = روکا گیا؛ ابھی کچھ نیا نہیں

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 اپ ڈیٹ
       *[other] { $n } اپ ڈیٹس
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 اپ ڈیٹ تیار ہے
       *[other] { $n } اپ ڈیٹس تیار ہیں
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }، کل { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 روکی گئی ہے
       *[other] { $n } روکی گئی ہیں
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }؛ { $held }

# A line made into a sentence.
store-upd-sentence = { $text }۔

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = فرم ویئر کو مکمل کرنے کے لیے ری اسٹارٹ چاہیے۔

# Headline while the check runs.
store-upd-checking = اپ ڈیٹس کی جانچ ہو رہی ہے۔

# Headline while one update goes in on its own; its name is given.
# SAFETY: needs a human review before it ships
store-upd-updating-one = { $name } اپ ڈیٹ ہو رہا ہے۔ اس کے انسٹال ہونے کے دوران آپ کام جاری رکھ سکتے ہیں۔

# Headline while all the updates go in.
# SAFETY: needs a human review before it ships
store-upd-installing-all = اپ ڈیٹس انسٹال ہو رہی ہیں۔ ان کے انسٹال ہونے کے دوران آپ کام جاری رکھ سکتے ہیں۔

# Headline when the updates are in and the computer must start again.
# SAFETY: needs a human review before it ships
store-upd-restart = مکمل کرنے کے لیے ری اسٹارٹ کریں: اپ ڈیٹس انسٹال ہو چکی ہیں، اور کمپیوٹر دوبارہ شروع ہونے پر استعمال میں آئیں گی۔

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = جو انسٹال ہوا ہے اسے مکمل کرنے کے لیے ری اسٹارٹ چاہیے۔

# Added to the headline with no internet.
store-upd-no-more = یہ کمپیوٹر انٹرنیٹ سے جڑا نہیں ہے، اس لیے مزید تلاش نہیں ہو سکتی۔

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = کوئی اپ ڈیٹ تیار نہیں

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = اس کمپیوٹر کے علم میں کوئی اپ ڈیٹ منتظر نہیں۔ انٹرنیٹ سے جڑے بغیر یہ مزید تلاش نہیں کر سکتا۔

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = ایپس

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle اور Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = فرم ویئر

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = { $names } کے لیے اپ ڈیٹس کی جانچ نہیں ہو سکی۔

# Headline when everything is up to date.
store-upd-current = سب کچھ تازہ ترین ہے۔

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = آخری جانچ { $time } پر ہوئی۔

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = انٹرنیٹ کے بغیر جانچ نہیں ہو سکتی

# Red words under a card when a check took too long.
store-upd-too-long = جواب دینے میں بہت دیر لگی۔

# Tip of the round button that checks again.
store-upd-check-again = دوبارہ جانچیں

# Main button while the check runs.
store-upd-checking-main = جانچ ہو رہی ہے

# Main button while the updates go in.
store-upd-installing-main = انسٹال ہو رہا ہے

# Main button that installs every update waiting.
# SAFETY: needs a human review before it ships
store-upd-install-n =
    { $n ->
        [one] 1 اپ ڈیٹ انسٹال کریں
       *[other] { $n } اپ ڈیٹس انسٹال کریں
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = ابھی ری اسٹارٹ کریں

# Main button that checks for updates.
store-upd-check-for = اپ ڈیٹس تلاش کریں

# Tip of the round button that releases a thing held back.
# SAFETY: needs a human review before it ships
store-upd-release = { $name } کو چھوڑیں

# Tip of the round button that holds a thing's updates back.
# SAFETY: needs a human review before it ships
store-upd-hold = { $name } کو روک رکھیں

# A card's state while it is being checked.
store-upd-status-checking = جانچ ہو رہی ہے

# A card's state while its updates go in.
store-upd-status-installing = انسٹال ہو رہا ہے

# A card's state when its check failed; the reason follows.
store-upd-status-failed = جانچ نہیں ہو سکی۔ { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }، { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }؛ { $held } روکی گئیں

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } روکی گئیں

# A card's state when nothing waits.
store-upd-up-to-date = تازہ ترین

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = { $from } سے، { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = { $from } سے

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = ایک مشترک حصہ، { $from } سے

# Note under the Apps card.
store-upd-yours-alone = صرف آپ کی ہیں، اس لیے بغیر پاس ورڈ کے اپ ڈیٹ ہوتی ہیں۔ ایپ اپنے نئے ورژن کو درکار مشترک حصے ساتھ لاتی ہے۔

# Note under the Apps card: what holding back does.
# SAFETY: needs a human review before it ships
store-upd-hold-why = روکی گئی اپ ڈیٹ اسی ورژن پر رہتی ہے جو اس کے پاس ہے، اور یہاں اور حلقے پر "سب انسٹال کریں" سے باہر رہتی ہے، جب تک آپ اسے چھوڑ نہ دیں۔

# Heading of the Apps card.
store-upd-card-apps = ایپس

# Plugins card: every plugin is up to date.
store-upd-plugins-current = ہر پلگ اِن تازہ ترین ہے۔

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] 1 پلگ اِن کی اپ ڈیٹ روکی گئی ہے۔
       *[other] { $n } پلگ اِنز کی اپ ڈیٹس روکی گئی ہیں۔
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 پلگ اِن کی اپ ڈیٹ موجود ہے
       *[other] { $n } پلگ اِنز کی اپ ڈیٹ موجود ہے
    }

# Heading of the Plugins card.
store-upd-card-plugins = پلگ اِن

# Held back card: nothing is held back.
store-upd-nothing-held = کچھ بھی روکا ہوا نہیں۔

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = چھوڑ دیا گیا

# Note under the Held back card: what releasing does.
store-upd-release-why = چھوڑی گئی چیز دوبارہ باقی کے ساتھ انسٹال ہوتی ہے، یہاں اور حلقے پر "سب انسٹال کریں" سے۔

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = کسی آلے کا فرم ویئر

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = آلہ { $id } کا فرم ویئر

# Heading of the Held back card.
store-upd-card-held = روکا ہوا

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } سے { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 اپ ڈیٹ، جو اکیلے انسٹال ہوتی ہے
       *[other] { $n } اپ ڈیٹس، جو ایک ساتھ انسٹال ہوتی ہیں
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Debian سے

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Debian کی { $total } اپ ڈیٹس میں سے { $shown } دکھائی جا رہی ہیں۔

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = اس کمپیوٹر میں Trindle { $version } ہے۔

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle اور Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
# SAFETY: needs a human review before it ships
store-upd-debian-why = Debian کے پیکجز ایک دوسرے کے ساتھ بیٹھنے کے لیے بنے ہیں، اور اکیلا انسٹال کیا گیا پیکج دوسروں کو ساتھ لا سکتا ہے یا انہیں بے میل چھوڑ سکتا ہے۔ اس لیے وہ ایک ہی بٹن سے ایک ساتھ انسٹال ہوتے ہیں۔

# Note on the Trindle and Debian card: why no row has a Hold back button.
# SAFETY: needs a human review before it ships
store-upd-not-held = Trindle اور Debian کی اپ ڈیٹس پورے کمپیوٹر کے لیے سیکیورٹی کی اصلاحات لاتی ہیں، اس لیے انہیں روکا نہیں جا سکتا۔

# Heading of the card about the daily security updates.
# SAFETY: needs a human review before it ships
store-upd-card-security = سیکیورٹی اپ ڈیٹس

# On the security card: its state.
# SAFETY: needs a human review before it ships
store-upd-security-on = آن، ہر روز

# On the security card.
# SAFETY: needs a human review before it ships
store-upd-security-note = Debian کی اصلاحات ہر روز بغیر پوچھے انسٹال ہوتی ہیں۔

# On the security card.
# SAFETY: needs a human review before it ships
store-upd-security-part = Trindle کا حصہ؛ اسے بند نہیں کیا جا سکتا۔

# On the security card: that holding back never stops these.
# SAFETY: needs a human review before it ships
store-upd-never-held = کسی اپ ڈیٹ کو روکنے سے یہ کبھی نہیں رکتیں۔

# Firmware card: the computer has no updater.
store-upd-fw-none = اس کمپیوٹر میں فرم ویئر اپ ڈیٹر نہیں ہے۔

# Firmware card: restart to finish.
store-upd-fw-restart = مکمل کرنے کے لیے ری اسٹارٹ کریں

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = ورژن { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = نیا فرم ویئر کمپیوٹر کے ری اسٹارٹ ہونے پر انسٹال ہوتا ہے۔

# Note on the firmware card when the updater is missing.
store-upd-fw-install = فرم ویئر کو تازہ ترین رکھنے کے لیے fwupd انسٹال کریں۔

# Heading of the Firmware card.
store-upd-card-firmware = فرم ویئر

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd تازہ ترین فہرست نہیں لا سکا۔

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub تک رسائی نہیں ہو سکی۔

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Debian کی فہرستیں نہیں لائی جا سکیں۔

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Debian کی فہرستیں نہیں لائی جا سکیں۔ { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = روکا نہیں گیا۔ روکی ہوئی چیزوں کی فہرست محفوظ نہیں ہو سکی: { $why }۔

# The same, for a release.
store-upd-not-released-list = چھوڑا نہیں گیا۔ روکی ہوئی چیزوں کی فہرست محفوظ نہیں ہو سکی: { $why }۔

# Red words when Flatpak could not be told a thing is held back.
# SAFETY: needs a human review before it ships
store-upd-held-not-told = ایپس میں روک دیا گیا، مگر خود Flatpak کو بتایا نہیں جا سکا، اس لیے ایپس اپ ڈیٹ کرنے والی کوئی اور چیز اسے پھر بھی انسٹال کر سکتی ہے۔

# Red words when Flatpak could not be told a thing is released.
# SAFETY: needs a human review before it ships
store-upd-released-not-told = ایپس میں چھوڑ دیا گیا، مگر خود Flatpak کو بتایا نہیں جا سکا اور وہ اسے اب بھی چھوڑ سکتا ہے۔ اسے روکیں اور دوبارہ چھوڑیں۔

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak نے کہا: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = اپ ڈیٹس کی خودکار جانچ بند ہے۔ دیکھنے کے لیے "اپ ڈیٹس کی جانچ کریں" دبائیں۔

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = جانچ نہیں ہوئی

# Tip of the round button that ends an update that is running.
store-upd-stop = روکیں

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = رک رہا ہے

# Under a row whose update was stopped.
store-upd-stopped = رک گیا۔ جاری رکھنے کے لیے "اپ ڈیٹ" دبائیں۔

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = رک گیا۔ جاری رکھنے کے لیے "انسٹال کریں" دبائیں۔

# Note under the Apps card.
store-upd-steam-restarts = Steam تب دوبارہ شروع ہوتا ہے جب کوئی گیم نہ چل رہا ہو۔

