
# machine translated (Urdu), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = ابھی ابھی

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] { $n } منٹ پہلے
       *[other] { $n } منٹ پہلے
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] { $n } گھنٹہ پہلے
       *[other] { $n } گھنٹے پہلے
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] { $n } دن پہلے
       *[other] { $n } دن پہلے
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = آپ کے پاس ورڈ کا انتظار ہے…

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = بلاکنگ چالو ہے

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = بلاکنگ بند ہے

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = محفوظ ہو گیا

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = کسی سائٹ کا نام لکھیں، جیسے example.com۔

# Headline of the page when blocking is off.
settings-blocking-top-off = بلاکنگ بند ہے۔ ویب سائٹس کا پتا پہلے کی طرح آپ کے نیٹ ورک کے اپنے سرورز سے لیا جاتا ہے۔

# Headline of the page while blocking is starting.
settings-blocking-top-starting = بلاکنگ شروع ہو رہی ہے۔

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = اشتہارات اور مضر سائٹس بلاک ہیں، اور ویب سائٹس کا پتا نجی طور پر { $provider } کے ذریعے لیا جاتا ہے۔

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle اس نیٹ ورک پر نجی سرور تک نہیں پہنچ سکتا، اس لیے فی الحال آپ کے نیٹ ورک کے اپنے سرورز استعمال کر رہا ہے۔ بلاکنگ اب بھی کام کرتی ہے۔

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = اشتہارات اور مضر سائٹس بلاک ہیں۔ ویب سائٹس کا پتا آپ کے نیٹ ورک کے اپنے سرورز سے لیا جاتا ہے، جو آپ کی کھولی ہوئی ہر سائٹ دیکھ سکتے ہیں۔

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = اشتہارات اور مضر سائٹس بلاک ہیں۔ ویب سائٹس کا پتا آپ کے VPN کے اپنے سرورز سے لیا جاتا ہے۔

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = اشتہارات اور مضر سائٹس بلاک ہیں۔

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = شروع ہو رہی ہے

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = بند

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = بلاکنگ شروع ہو رہی ہے

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = بلاکنگ بند ہے

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = آج بلاک ہوئیں

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = آج بلاک ہونے والی تلاشیں

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = فہرستیں

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = اپ ڈیٹ ہو رہی ہیں

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = ابھی تک کوئی نہیں

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = سائٹس کی فہرستیں

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = سائٹس درج ہیں، { $ago } اپ ڈیٹ ہوئیں

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = ابھی تک کچھ بلاک نہیں ہوا

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = اس سائٹ کو ہمیشہ اجازت دیں

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = حال ہی میں بلاک ہوئیں

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = سائٹس بلاک ہوتے ہی یہاں نظر آتی ہیں، نئی سب سے پہلے۔ کسی سائٹ کو آئندہ کے لیے گزرنے دینے کے لیے پلس دبائیں۔ Trindle گزرنے دی گئی سائٹس کا کوئی ریکارڈ نہیں رکھتا۔

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = کیا بلاک ہوتا ہے یہ دیکھنے کے لیے بلاکنگ چالو کریں۔

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = سائٹ شامل کریں

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = اس سائٹ کو دوبارہ بلاک کریں

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = ہمیشہ اجازت یافتہ

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = یہ سائٹس، اور ان کے نیچے کے ہر پتے، کبھی بلاک نہیں ہوتے، چاہے وہ فہرستوں میں ہوں۔ کسی سائٹ کو دوبارہ بلاک کرنے کے لیے مائنس دبائیں۔

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = بلاکنگ

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = بلاک اشتہارات ٹریکرز میلویئر دھوکے dns نجی تلاش quad9 mullvad cloudflare اجازت

# Help text at the top of the page.
settings-blocking-help = اس کمپیوٹر کے ہر براؤزر اور ایپ کے لیے اشتہارات، ٹریکرز اور معلوم مضر سائٹس کو بلاک کرتی ہے۔ یہ ویب سائٹس کا پتا بھی مخفی (انکرپٹڈ) کنکشن پر لیتی ہے، تاکہ آپ کا نیٹ ورک نہ دیکھ سکے اور نہ بدل سکے کہ آپ کون سی سائٹس کھولتے ہیں۔

# Text of the switch that turns blocking on or off.
settings-blocking-switch = اشتہارات اور مضر سائٹس بلاک کریں

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = تلاش کہاں جائے

# Help text (i button) of the Blocking card.
settings-blocking-card-about = HaGeZi کی اشتہارات، ٹریکرز اور معلوم مضر سائٹس کی فہرستیں استعمال کرتی ہے، جو دن میں ایک بار اپ ڈیٹ ہوتی ہیں۔ جب بلاکنگ بند ہو تو کچھ نہیں چلتا۔ ویب سائٹس کا پتا یہاں منتخب کیے گئے نجی سرور کے ذریعے مخفی (انکرپٹڈ) کنکشن پر لیا جاتا ہے۔ Quad9 کوئی ریکارڈ نہیں رکھتا کہ کس نے پوچھا۔ "آپ کا نیٹ ورک" پھر بھی بلاک کرتا ہے، لیکن نجی نہیں ہے، کیونکہ آپ کے نیٹ ورک کے اپنے سرورز آپ کی کھولی ہوئی ہر سائٹ دیکھ لیتے ہیں۔

# Heading of the line with the private server dropdown.
settings-blocking-private = نجی سرور

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = آپ کا نیٹ ورک

# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = فائر وال

# Help text (i button) of the Firewall card.
settings-firewall-about = جب فائر وال چالو ہو تو دوسرے کمپیوٹر اس کمپیوٹر سے نہیں جڑ سکتے، سوائے اس کے کہ کنکشن اسی کمپیوٹر نے شروع کیا ہو۔ اس لیے اسی Wi-Fi پر کوئی اندر آنے کی کوشش نہیں کر سکتا۔ براؤزنگ، اپ ڈیٹس اور پرنٹر پھر بھی کام کرتے ہیں۔ جو پروگرام دوسرے کمپیوٹروں کے جڑنے کا انتظار کرتا ہے، جیسے گیم سرور، اس کا پورٹ نیچے کھولنا پڑتا ہے۔ جب فائر وال بند ہو تو نیٹ ورک کا کوئی بھی کمپیوٹر ہر اس پروگرام تک پہنچ سکتا ہے جو کنکشن کا انتظار کر رہا ہو۔ اسے چالو یا بند کرنے کے لیے ایڈمنسٹریٹر کا پاس ورڈ چاہیے۔

# Name of the firewall's on and off switch.
settings-firewall-switch = فائر وال

# Small line under the switch while the firewall is off.
settings-firewall-off-sub = دوسرے کمپیوٹر ان پروگراموں تک پہنچ سکتے ہیں جو ان کا انتظار کرتے ہیں۔

# Said beside the switch once the firewall is on.
settings-firewall-done-on = فائر وال چل رہی ہے

# Said beside the switch once the firewall is off.
settings-firewall-done-off = فائر وال بند ہے

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = روک



# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = فائر وال چالو ہے مگر کام نہیں کر رہی، کیونکہ کمپیوٹر شروع ہونے پر اس کے قواعد لوڈ نہیں ہوئے۔ اسے بند کر کے دوبارہ چالو کریں۔
