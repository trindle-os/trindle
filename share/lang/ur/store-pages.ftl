
# machine translated (Urdu), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } ایپ
       *[other] { $count } ایپس
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } اپ ڈیٹ منتظر ہے
       *[other] { $count } اپ ڈیٹس منتظر ہیں
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub پر چننے کے لیے { $apps }؛ { $more }۔

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = آپ کی ہر چیز تازہ ترین ہے

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }۔

# Line under the title of a search that found nothing.
store-grid-none-found = “{ $text }” سے کوئی ایپ نہیں ملتی۔

# Line under the title of an empty page of apps.
store-grid-none-here = یہاں ابھی کوئی ایپ نہیں ہے۔

# Advice in the empty page of a search.
store-grid-try-shorter = چھوٹا لفظ آزمائیں، یا بائیں طرف کوئی قسم دیکھیں۔

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = اوپر نام سے تلاش کریں، یا بائیں طرف کوئی اور قسم دیکھیں۔

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $apps } میں سے { $shown } دکھائی جا رہی ہیں۔

# Link under a page of apps that shows more of them.
store-grid-show-more = { $next } مزید دکھائیں

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Flathub سے ایپس کی فہرست لائی جا رہی ہے۔ پہلی بار میں ایک منٹ لگتا ہے۔

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = ایپس Flathub تک نہیں پہنچ سکیں۔

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = یہ کمپیوٹر انٹرنیٹ سے جڑا نہیں ہے۔ جڑتے ہی ایپس خود دوبارہ کوشش کرتی ہے۔

# In the waiting page when the computer is not connected.
store-wait-comes-from = چننے کے لیے ایپس انٹرنیٹ کے ذریعے Flathub سے آتی ہیں۔

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } اور ایپس کی سیٹنگز، بائیں طرف، اس کے بغیر چلتی ہیں۔

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } اور ایپس کی سیٹنگز، بائیں طرف، Flathub کے بغیر چلتی ہیں۔

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } اور ایپس کی سیٹنگز، بائیں طرف، انٹرنیٹ کے بغیر چلتی ہیں۔

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = دوبارہ کوشش کریں

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = نتائج

# Red words under the search box when no web browser opens a link.
store-no-browser = { $url } کھولنے کے لیے کوئی ویب براؤزر نہیں ہے۔ پہلے ایپس میں سے ایک حاصل کریں۔

# Said in the card or under the title while the lists are being read.
store-looking = دیکھا جا رہا ہے…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = یہ اب انسٹال شدہ میں ہے، اور حلقے میں بھی

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = ہٹا دیا گیا

# The word on an app's dial when installing or removing did not work.
store-done-not = نہیں ہوا

# The word on an app's dial when it was installed.
store-done-installed = انسٹال ہو گیا

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak نے انکار کر دیا۔

# Tab of the Installed page that shows everything.
store-tab-all = سب

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak فائلیں

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImages

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb فائلیں

# Tab of the Installed page that shows plugins.
store-tab-plugins = پلگ اِن

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = ابھی کچھ انسٹال نہیں ہے۔ دریافت میں آزمانے کے لیے بہت کچھ ہے۔

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 پلگ اِن انسٹال ہے
       *[other] { $n } پلگ اِن انسٹال ہیں
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 پروگرام انسٹال ہے
       *[other] { $n } پروگرام انسٹال ہیں
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } چیزیں انسٹال ہیں

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }، Flathub سے
       *[other] { $start }، سب Flathub سے
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }، ایک Flatpak فائل سے
       *[other] { $start }، سب Flatpak فائلوں سے
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }، ایک AppImage
       *[other] { $start }، سب AppImages
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }، ایک .deb فائل سے
       *[other] { $start }، سب .deb فائلوں سے
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }، { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } Flathub سے

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 Flatpak فائل سے
       *[other] { $n } Flatpak فائلوں سے
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
       *[other] { $n } AppImages
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 ایپ
       *[other] { $n } ایپس
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 .deb فائل سے
       *[other] { $n } .deb فائلوں سے
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 پلگ اِن
       *[other] { $n } پلگ اِن
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }۔ { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = یہ تازہ ترین ہے۔

# End of the Installed line when everything is up to date.
store-installed-state-all = سب تازہ ترین ہیں۔

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 کی اپ ڈیٹ منتظر ہے۔
       *[other] { $n } کی اپ ڈیٹس منتظر ہیں۔
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $total } پروگراموں میں سے { $shown } “{ $find }” سے ملتے ہیں۔

# The same when plugins are among the things.
store-installed-match-things = { $total } چیزوں میں سے { $shown } “{ $find }” سے ملتی ہیں۔

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = AppImage شامل کریں

# Placeholder of the typing box on the Installed page.
store-installed-search = جو انسٹال ہے اس میں تلاش کریں

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage شامل نہیں ہو سکی۔

# In the card of the Installed page when nothing is installed.
store-installed-empty = دریافت سے، GitHub سے یا AppImages کے طور پر انسٹال کی گئی ایپس، .deb فائلوں کے پروگرام اور پلگ اِن یہاں نظر آتے ہیں۔

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = “{ $find }” سے انسٹال شدہ کچھ نہیں ملتا۔

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = ہٹانے کے لیے دو بار پوچھا جاتا ہے: تین سیکنڈ کے اندر اسے دوبارہ دبائیں۔

# On an installed thing's line: its version.
store-installed-version = ورژن { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = روکا گیا

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = روکا گیا؛ { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } اپ ڈیٹ ہو رہی ہے

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } اپ ڈیٹ کریں

# Tip of the round button that shows an installed app's page.
store-installed-about = تعارف

# Under an installed thing's row while it is being taken off.
store-installed-removing = ہٹایا جا رہا ہے...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = یہ پوچھنے کے لیے سیشن بس نہیں ہے کہ کون سی فائل۔

# Title of the window that asks which AppImage to add.
store-add-choose-title = AppImage چنیں

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImages

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = فائل چننے والے سے پوچھا نہیں جا سکا۔

# Words in the Installed card while an AppImage is being added.
store-add-adding = شامل ہو رہی ہے...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } پہلے سے موجود ہے: انسٹال شدہ دیکھیں۔

# Words once an AppImage is in the ring.
store-add-in-ring = شامل کر دیا گیا۔ اب یہ آپ کی فہرست میں ہے۔

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = یہ ایسا Flatpak حوالہ نہیں جو یہ کمپیوٹر استعمال کر سکے۔

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = { $host } سے { $name } انسٹال ہو رہی ہے...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } انسٹال ہو گئی۔

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak نے اسے انسٹال نہیں کیا۔

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } انسٹال نہیں ہو سکی۔ { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = بدلا جا رہا ہے...

# Heading of the question when an AppImage has the name of one already there.
# SAFETY: needs a human review before it ships
store-replace-head = { $name } پہلے سے موجود ہے

# Small heading over the AppImage being brought.
store-replace-new = جو لائی جا رہی ہے

# Small heading over the AppImage already there.
store-replace-old = جو پہلے سے موجود ہے

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = چھوڑ دیں: جو پہلے سے موجود ہے اسے رکھیں

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = بدل دیں

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = کیا { $name } کو ہٹائیں؟

# Under that heading: how much saved data the app has.
# SAFETY: needs a human review before it ships
store-flatpak-ask-size = اس کا محفوظ ڈیٹا { $size } لیتا ہے۔

# Under that heading when the app has saved no data.
# SAFETY: needs a human review before it ships
store-flatpak-ask-none = اس کا کوئی محفوظ ڈیٹا نہیں ہے۔

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = اس کا محفوظ ڈیٹا گنا جا رہا ہے...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = مٹایا ہوا ڈیٹا واپس نہیں آ سکتا۔

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = ہٹائیں، ڈیٹا رکھیں

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = ہٹائیں اور اس کا ڈیٹا مٹائیں

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = صرف اس کا ڈیٹا مٹائیں

# Why Delete its data only is greyed.
store-flatpak-data-only-none = مٹانے کے لیے اس کا کوئی محفوظ ڈیٹا نہیں ہے۔

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = منسوخ کریں: دونوں رکھیں

# Said under the card or the row once only the data has been deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-gone = اس کا محفوظ ڈیٹا مٹا دیا گیا ہے۔

# Words while only the data is being deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-going = اس کا محفوظ ڈیٹا مٹایا جا رہا ہے...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = یہ مکمل نہیں ہوا، اور کوئی وجہ نہیں بتائی۔

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = دکھائیں کہ مسئلہ کہاں ہے

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = GitHub پر تلاش کریں: لکھیں، پھر Enter دبائیں

# Heading of the card of matches from AppImageHub.
store-gh-hub = AppImageHub پر ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = پورے GitHub پر بھی تلاش کرنے کے لیے Enter دبائیں۔

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub سے پوچھا نہیں گیا: پوچھنے کے لیے انٹرنیٹ نہیں ہے۔

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub سے پوچھا جا رہا ہے...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub پر اور کچھ نہیں۔

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] { $n } پروجیکٹ میں AppImage اور Flatpak فائلیں تلاش کی جا رہی ہیں...
       *[other] { $n } پروجیکٹس میں AppImage اور Flatpak فائلیں تلاش کی جا رہی ہیں...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub پر ملنے والوں میں سے کسی کے پاس اس کمپیوٹر کے لیے AppImage یا Flatpak فائل نہیں ہے۔

# Heading of the card of matches from GitHub.
store-gh-on = GitHub پر

# Heading of the card that holds the search box.
store-gh-search-card = GitHub پر تلاش کریں

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = GitHub پر ایپس AppImage یا Flatpak فائلوں کی صورت میں۔ ان کو کوئی نہیں جانچتا: صرف ان لوگوں سے انسٹال کریں جن پر آپ بھروسا کرتے ہیں۔

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = جب تک یہ کمپیوٹر انٹرنیٹ سے جڑا نہیں، GitHub پر تلاش نہیں ہو سکتی۔

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak اور AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = فائلیں دیکھیں

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = انسٹال کریں

# Link to a project's page on GitHub.
store-gh-open = GitHub پر کھولیں

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }، { $size }، ریلیز { $tag }

# Name of the GitHub key's button and of its box.
store-gh-key = GitHub کلید

# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub کلید: ایک محفوظ ہے

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = غالباً آپ کو اس کی ضرورت نہیں۔ کلید کے بغیر GitHub اس کمپیوٹر سے ایک گھنٹے میں 60 سوالوں کا جواب دیتا ہے؛ کلید کے ساتھ 5,000 کا۔ کلید صرف یہ حد بڑھاتی ہے۔ یہ صرف عوامی فائلیں دیکھنے کے لیے استعمال ہوتی ہے اور کسی اور کام کے لیے نہیں: یہ Trindle کو آپ کے اکاؤنٹ تک کوئی راستہ نہیں دیتی، اور بناتے وقت آپ کوئی اجازت نہیں چنتے۔ یہ اس کمپیوٹر پر، آپ کی کی رنگ میں رکھی جاتی ہے، اور صرف GitHub کو بھیجی جاتی ہے۔

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = GitHub پر کلید بنائیں، اسے کوئی بھی نام دیں، ہر اجازت کو ویسے ہی چھوڑ دیں، Generate token دبائیں، پھر جو دکھائی دے اسے یہاں چسپاں کریں۔

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = اسے ہٹایا نہیں جا سکا: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = اسے رکھا نہیں جا سکا: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = رکھنے کے لیے خالی چھوڑیں

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = کلید چسپاں کریں

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = کلید ہٹائیں

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = GitHub پر کلید بنائیں

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = کلید رکھیں

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = اس کی نئی ترین ریلیز میں کچھ بھی یہاں انسٹال نہیں ہوتا۔

# Word on the file's dial and under a GitHub file when it was added.
store-added = شامل کر دیا گیا

# Line under the title of the Move your apps page.
store-bk-line = آپ کی ایپس اور سیٹنگز، ایک فائل میں جو آپ کہیں بھی لے جا سکیں۔

# Heading of the card that makes a backup.
store-bk-card = بیک اپ

# Tip of the round button while the backup is being made.
store-bk-making-tip = بیک اپ بنایا جا رہا ہے

# Main button that makes the backup.
store-bk-make = بیک اپ بنائیں

# Tip of the round button that shows the backup in Folders.
store-bk-show = اسے فولڈرز میں دکھائیں

# Name of the switch for the list of apps in a backup.
store-bk-apps = ایپس

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] آپ کی 1 Flatpak ایپ کی فہرست۔
       *[other] آپ کی { $n } Flatpak ایپس کی فہرست۔
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } دوسرا کمپیوٹر انہیں Flathub سے نئے سرے سے انسٹال کرتا ہے، اس لیے فائل چھوٹی رہتی ہے۔

# Name of the switch for the apps' own data in a backup.
store-bk-data = ایپس کا اپنا ڈیٹا

# Under the Apps' own data switch.
store-bk-data-sub = ہر ایپ کی سیٹنگز اور محفوظ کیا ہوا کام، کیش کے بغیر۔

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImages

# Under the AppImages switch.
store-bk-appimages-sub = پوری کی پوری رکھی جاتی ہیں، کیونکہ یہ اکیلی فائلیں ہیں۔

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle کی سیٹنگز

# Under the Trindle settings switch.
store-bk-settings-sub = حلقہ، وال پیپر کا انتخاب، رنگ، کی بورڈ، پاور، ڈیفالٹ ایپس اور آپ کی تصویر۔

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } ابھی { $size }۔

# Name of the row that says where the backup goes.
store-bk-where = یہ کہاں جاتا ہے

# Under that row: the folder and the file's name.
store-bk-where-sub = آپ کا ہوم فولڈر، { $name } کے نام سے

# Heading of the card that puts a backup back.
store-bk-back-card = بیک اپ واپس رکھیں

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = بیک اپ فائل چنیں

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Trindle کا بیک اپ، یا { $flat } کا بنایا ہوا۔

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = اس کی ایپس انسٹال ہوتی ہیں اور اس کا ڈیٹا اور سیٹنگز واپس رکھی جاتی ہیں۔ آپ کی دستاویزات اور تصویریں اس میں نہیں ہیں: انہیں خود کاپی کریں۔

# Said while a backup is being made.
store-bk-making = بیک اپ ہو رہا ہے... اس میں چند منٹ لگ سکتے ہیں۔

# Said when a backup is made: the file and its size.
store-bk-made = یہ { $path } ({ $size }) ہے۔ اسے ساتھ لے جانے کے لیے USB اسٹک یا کلاؤڈ اسٹوریج پر کاپی کریں۔

# Red words when no connection to the file chooser exists.
store-bk-no-bus = کوئی بیک اپ نہیں چنا گیا، کیونکہ Trindle کا فائل چننے والے سے رابطہ نہیں ہے۔

# Title of the window that asks which backup to use.
store-bk-choose-title = بیک اپ چنیں

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle اور Flat کے بیک اپ

# Red words when the file chooser gave no answer.
store-bk-no-answer = کوئی بیک اپ نہیں چنا گیا، کیونکہ فائل چننے والے نے جواب نہیں دیا۔

# Said while a backup file is read.
store-bk-reading = بیک اپ پڑھا جا رہا ہے...

# Said when a backup was put back.
store-bk-logout = ہر سیٹنگ دیکھنے کے لیے لاگ آؤٹ کر کے واپس لاگ اِن کریں۔

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } ہر سیٹنگ دیکھنے کے لیے لاگ آؤٹ کر کے واپس لاگ اِن کریں۔

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 ایپ، { $with } اپنی سیٹنگز کے ساتھ۔ شروع ہو رہا ہے...
       *[other] { $apps } ایپس، { $with } اپنی سیٹنگز کے ساتھ۔ شروع ہو رہا ہے...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] ابھی بنانے والوں کا کوئی اپنا ذریعہ شامل نہیں ہے۔
        [one] بنانے والے کا 1 ذریعہ شامل ہے، جو Debian کی اپ ڈیٹس کے ساتھ تازہ رکھا جاتا ہے۔
       *[other] بنانے والوں کے { $n } ذرائع شامل ہیں، جو Debian کی اپ ڈیٹس کے ساتھ تازہ رکھے جاتے ہیں۔
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }۔ شامل ہے، اور تازہ رکھا جاتا ہے۔

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }۔ ذریعہ شامل ہے، مگر پروگرام انسٹال نہیں ہے۔

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }۔ کسی اور طریقے سے انسٹال ہے۔

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = اسے اور اس کا ذریعہ ہٹائیں

# Link to a maker's page, naming the program.
store-src-open = { $name } کھولیں

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Tailscale میں لاگ اِن کریں

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = { $name } شامل کریں

# Heading of the card of makers' sources that can be added.
store-src-ready = شامل کرنے کے لیے تیار

# Name of the field for the address of a source.
store-src-address = پتا

# Under the address field.
store-src-address-sub = ریپوزٹری کا پتا، اس کے بنانے والے کی انسٹال ہدایات سے۔

# Name of the field for the suite of a source.
store-src-suite = سویٹ

# Under the suite field.
store-src-suite-sub = عموماً trixie یا stable۔

# Name of the field for the components of a source.
store-src-components = اجزا

# Under the components field.
store-src-components-sub = عموماً main۔

# Name of the field for a source's signing key.
store-src-key = دستخطی کلید

# Under the signing key field.
store-src-key-sub = اس کی دستخطی کلید کا پتا۔

# Name of the field for the program to install from a source.
store-src-program = انسٹال کرنے کا پروگرام

# Under the program field.
store-src-program-sub = اس کے پیکج کا نام، اگر آپ اسے ابھی انسٹال کروانا چاہتے ہیں۔

# Placeholder of the program field.
store-src-program-hint = اختیاری

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = یہ ذریعہ شامل کریں

# Heading of the card that adds another source.
store-src-another = ایک اور ذریعہ شامل کریں

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = ذریعہ پورے کمپیوٹر کے لیے اپ ڈیٹس انسٹال کر سکتا ہے، اس لیے صرف ان بنانے والوں کو شامل کریں جن پر آپ بھروسا کرتے ہیں۔ ہر ایک منتظم کا پاس ورڈ مانگتا ہے۔

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = یہ ذریعہ ہٹائیں

# Heading of the card of sources already on the computer.
store-src-already = اس کمپیوٹر پر پہلے سے موجود

# Red words when the addresses of a new source are not secure.
store-src-bad-address = دونوں پتے https:// سے شروع ہونے چاہییں

# Red words when the suite or components are not plain words.
store-src-bad-words = سویٹ اور اجزا سادہ الفاظ ہونے چاہییں۔

# Red words when the program is not a package name.
store-src-bad-package = یہ پیکج کا نام نہیں ہے۔

# Word on a source's dial while its job starts.
store-src-starting = شروع ہو رہا ہے

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Tailscale کا لاگ اِن صفحہ کھل رہا ہے...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = جو صفحہ کھلا ہے اس پر لاگ اِن کریں؛ یہ خود مکمل ہو جاتا ہے۔

# Said under Tailscale's row when it is connected.
store-src-connected = جڑا ہوا

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = ایپس کا لنک خفیہ کردہ نہیں ہے۔

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = خفیہ کردہ نہیں

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = یہ پتا خفیہ کردہ نہیں ہے۔ راستے میں کوئی بھیجا گیا ڈیٹا دیکھ سکتا ہے، اور بیک اپ کو کسی دوسرے سے بدل بھی سکتا ہے۔

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = میں سمجھتا ہوں اور یہ خطرہ قبول کرتا ہوں

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = بحال کریں

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = منسوخ کریں

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
store-flatpak-ask-uncounted = اس کا محفوظ ڈیٹا گنا نہیں جا سکا۔

store-hub-warning = AppImageHub صرف منصوبوں کی فہرست دیتا ہے: کوئی انہیں جانچتا یا اسکین نہیں کرتا، اور ان کا دستخط شدہ ہونا ضروری نہیں۔ Trindle ہر AppImage کو نجی ہوم فولڈر والے ڈبے میں چلاتا ہے: شروع میں وہ انٹرنیٹ اور آواز استعمال کر سکتی ہے، اور آپ کی کوئی فائل نہیں دیکھتی۔

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = AppImage کہاں سے آئی ہے، یہ ایک اشارہ ہے، جانچ نہیں۔

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = GitHub پر: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = GitHub پر: { $repo }، { $stars } ستارے

# Where an AppImage comes from: a website, by its name.
store-from-site = GitHub پر نہیں: ایک ویب سائٹ سے، { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = ماخذ معلوم نہیں

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = اس کی ویب سائٹ کھولیں

# When an app was last updated.
store-age-updated = آخری اپ ڈیٹ: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = آخری اپ ڈیٹ: { $date }۔ چھ مہینے سے زیادہ سے اپ ڈیٹ نہیں ہوئی، اس لیے اس میں غیر حل شدہ سیکیورٹی خامیاں ہو سکتی ہیں۔

# When the source does not say when an app was last updated.
store-age-updated-unknown = آخری اپ ڈیٹ: معلوم نہیں

# When an app's project was made.
store-age-created = بنایا گیا: { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = بنایا گیا: معلوم نہیں

# A date the source does not give, on a line of Facts.
store-age-not-known = معلوم نہیں

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = ایک شامل کرنے سے پہلے

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] AppImageHub پر { $count } AppImage درج ہے۔
       *[other] AppImageHub پر { $count } AppImage درج ہیں۔
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = AppImageHub کی فہرست لائی جا رہی ہے…

# The same when the list could not be fetched.
store-browse-failed = AppImageHub کی فہرست نہیں لائی جا سکی۔

# The same with no internet and no copy kept.
store-browse-offline = جب تک یہ کمپیوٹر انٹرنیٹ سے جڑا نہ ہو، AppImageHub کی فہرست نہیں لائی جا سکتی۔

# In the list when the kind chosen has none.
store-browse-none = AppImageHub میں اس قسم کی کوئی نہیں ہے۔

# Under the rows: how many are shown of how many.
store-browse-showing = { $n } میں سے { $shown } دکھائی جا رہی ہیں۔

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Trindle کی ایپس میں شامل کریں

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = تفصیل اور فائلیں دیکھیں



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = اپ ڈیٹس کی جانچ بند ہے

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT تب واپس آئے گا جب OpenAI اپنا ڈاؤن لوڈ دستخط کر دے گا۔



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Flathub اور AppImageHub پر چننے کے لیے { $apps }؛ { $more }۔

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = AppImageHub پر چننے کے لیے { $apps }۔

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = دونوں

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage کی جانچ نہیں ہوتی۔ کوئی انہیں نہ دیکھتا ہے نہ اسکین کرتا ہے۔

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle ہر AppImage کو ایک الگ ڈبے میں چلاتا ہے جس کا ہوم فولڈر نجی ہوتا ہے: شروع میں وہ انٹرنیٹ اور آواز استعمال کر سکتا ہے، اور آپ کی کوئی فائل نہیں دیکھتا۔

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = دوبارہ نہ دکھائیں

# Button on that card that goes on with the install.
store-appimage-warn-ok = ٹھیک ہے

# Button on that card that gives the install up.
store-appimage-warn-cancel = منسوخ کریں

