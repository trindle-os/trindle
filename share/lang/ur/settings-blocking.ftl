
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
settings-blocking-waiting = پاس ورڈ کا انتظار ہے...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = بلاکنگ چالو ہے

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = بلاکنگ بند ہے

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = محفوظ ہو گیا

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = کسی سائٹ کا نام لکھیں، جیسے example.com۔

# Headline of the page when blocking is off.
settings-blocking-top-off = بلاکنگ بند ہے۔ تلاش پہلے کی طرح آپ کے نیٹ ورک کے اپنے سرورز پر جاتی ہے۔

# Headline of the page while blocking is starting.
settings-blocking-top-starting = بلاکنگ شروع ہو رہی ہے۔

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = اشتہارات اور مضر سائٹس بلاک ہیں، اور تلاش نجی طور پر { $provider } پر جاتی ہے۔

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = اس نیٹ ورک پر نجی سرور تک رسائی نہیں ہو سکی، اس لیے فی الحال آپ کے نیٹ ورک کے اپنے سرورز استعمال ہو رہے ہیں۔ بلاکنگ اب بھی کام کرتی ہے۔

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = اشتہارات اور مضر سائٹس بلاک ہیں۔ تلاش آپ کے نیٹ ورک کے اپنے سرورز پر جاتی ہے، جو اسے دیکھ سکتے ہیں۔

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = اشتہارات اور مضر سائٹس بلاک ہیں۔ تلاش VPN کے ذریعے آپ کے VPN کے اپنے سرورز پر جاتی ہے۔

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
settings-blocking-tile-fetching = حاصل ہو رہی ہیں

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = ابھی تک کوئی نہیں

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = سائٹس کی فہرستیں

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = سائٹس درج ہیں، { $ago } حاصل کی گئیں

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = ابھی تک کچھ بلاک نہیں ہوا

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = سائٹس یہاں نظر آئیں گی، نئی سب سے پہلے۔

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = اس سائٹ کو ہمیشہ اجازت دیں

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = حال ہی میں بلاک ہوئیں

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = سائٹس بلاک ہوتے ہی یہاں نظر آتی ہیں، نئی سب سے پہلے۔ پلس کا بٹن کسی سائٹ کو آئندہ کے لیے گزرنے دیتا ہے۔ یہاں صرف بلاک شدہ سائٹس دکھائی جاتی ہیں۔ جنہیں گزرنے دیا گیا ہو انہیں کبھی محفوظ نہیں رکھا جاتا۔

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = کیا بلاک ہوتا ہے یہ دیکھنے کے لیے بلاکنگ چالو کریں۔

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = مثلاً example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = سائٹ شامل کریں

# Small line under "Add a site".
settings-blocking-add-sub = یہ اور اس کی تمام ذیلی ڈومینز کبھی بلاک نہیں ہوں گی۔

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = اس سائٹ کو دوبارہ بلاک کریں

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = ہمیشہ اجازت یافتہ

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = یہاں درج سائٹس، اور ان کی تمام ذیلی ڈومینز، کبھی بلاک نہیں ہوتیں، چاہے وہ فہرستوں میں ہوں۔ مائنس کا بٹن کسی سائٹ کو دوبارہ بلاک کر دیتا ہے۔

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = بلاکنگ

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = بلاک اشتہارات ٹریکرز میلویئر دھوکے dns نجی تلاش quad9 mullvad cloudflare اجازت

# Help text at the top of the page.
settings-blocking-help = اس کمپیوٹر کے ہر براؤزر اور ایپ کے لیے اشتہارات، ٹریکرز اور معلوم مضر سائٹس کو بلاک کرتی ہے، اور ناموں کی تلاش مخفی (انکرپٹڈ) کنکشن پر بھیجتی ہے، تاکہ آپ کا نیٹ ورک انہیں پڑھ یا بدل نہ سکے۔

# Text of the switch that turns blocking on or off.
settings-blocking-switch = اشتہارات اور مضر سائٹس بلاک کریں

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = تلاش کہاں جائے

# Help text (i button) of the Blocking card.
settings-blocking-card-about = HaGeZi کی اشتہارات، ٹریکرز اور معلوم مضر سائٹس کی فہرستیں استعمال کرتی ہے، جو دن میں ایک بار حاصل کی جاتی ہیں۔ جب یہ بند ہو تو کچھ نہیں چلتا۔ ناموں کی تلاش یہاں منتخب کیے گئے نجی سرور پر مخفی (انکرپٹڈ) کنکشن سے جاتی ہے۔ Quad9 کوئی ریکارڈ نہیں رکھتا کہ کس نے پوچھا۔ "آپ کا نیٹ ورک" پھر بھی بلاک کرتا ہے، لیکن نجی نہیں ہے: آپ کے نیٹ ورک کے اپنے سرورز ہر تلاش دیکھتے ہیں۔

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = HaGeZi کی فہرستیں، دن میں ایک بار حاصل ہوتی ہیں۔

# Heading of the line with the private server dropdown.
settings-blocking-private = نجی سرور

# Small line under Private server.
settings-blocking-private-sub = Quad9 کوئی ریکارڈ نہیں رکھتا کہ کس نے پوچھا۔

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = آپ کا نیٹ ورک
