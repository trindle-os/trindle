
# machine translated (Urdu), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = ابھی کوئی پرنٹر نہیں۔ آپ کے نیٹ ورک پر ملنے والے نیچے درج ہیں۔

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } رکا ہوا ہے، اس لیے فی الحال اس پر کچھ نہیں چھپے گا۔

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } چھاپ رہا ہے۔

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = { $name } پر چھاپنے کے لیے تیار۔

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } { $state } ہے۔

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } لگا ہوا ہے؛ پہلے چھاپنے کے لیے کچھ منتخب نہیں۔

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } پرنٹر لگے ہوئے ہیں؛ پہلے چھاپنے کے لیے کوئی منتخب نہیں۔

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = یہ ایسا کام نہیں جسے CUPS جانتا ہو۔

# Shown beside a print job after it was cancelled.
# SAFETY: needs a human review before it ships
settings-printers-cancelled = منسوخ ہو گیا

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat ناکام ہوا

# Words in the ring while the network is searched for printers.
settings-printers-searching = تلاش جاری

# Words in the ring when the search is finished.
settings-printers-search-done = مکمل

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = اب ڈیفالٹ

# Shown beside a printer after a test page was sent.
settings-printers-sent = بھیج دیا

# Shown beside a printer after it was removed.
# SAFETY: needs a human review before it ships
settings-printers-removed = ہٹا دیا گیا

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = پہلے پرنٹر کا پتا لکھیں۔

# Shown under the By its address line while the printer is added.
settings-printers-adding = شامل ہو رہا ہے...

# Shown beside a printer or the address line after it was added.
settings-printers-added = شامل ہو گیا

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = اس کا ڈرائیور ڈھونڈا جا رہا ہے...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = { $printer } پر کام { $number }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }، { $size }، { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = یہ کام منسوخ کریں

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = چھپائی کے کام

# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = چھپنے کے انتظار میں ہر کام، کسی بھی پرنٹر پر، جیسا ابھی ہے۔ یہ صفحہ کھلا ہو تو فہرست ہر چند سیکنڈ بعد دوبارہ پڑھی جاتی ہے۔ صلیب کام منسوخ کرتی ہے؛ جو کاغذ چھپ چکا وہ چھپا ہی رہتا ہے۔

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS جواب نہیں دے رہا

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = قطار پڑھی نہیں جا سکتی۔

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = تلاش جاری…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = قطار۔

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = کچھ انتظار میں نہیں

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = کام چھپتے وقت یہاں دکھائی دیتے ہیں۔

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = رکا ہوا

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = فارغ

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = چھاپ رہا ہے

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = ڈیفالٹ، { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = پہلے اسی پر چھاپیں

# Tip of the button that prints a test page.
settings-printers-test-page = ٹیسٹ صفحہ چھاپیں

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = یہ پرنٹر ہٹائیں

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = تلاش کے لیے cups-filters چاہیے

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = اسے یوں انسٹال کریں: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = یہ پرنٹر شامل کریں

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = کچھ نیا نہیں ملا

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = کیا یہ چالو ہے، اور اسی نیٹ ورک پر ہے؟

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = پہلے اعلان کرنے والے، پھر ایک ایک مشین۔

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = شامل کریں

# Heading of the line where a printer's address is typed.
settings-printers-by-address = اس کے پتے سے

# Small line under "By its address".
settings-printers-by-address-sub = جیسے 192.168.1.50، جو اس کی اپنی سکرین پر ہوتا ہے۔

# Heading of the card for adding printers.
settings-printers-add-card = پرنٹر شامل کریں

# Help text (i button) of the Add a printer card.
settings-printers-add-about = جو پرنٹر اپنا اعلان کرتے ہیں انہیں پہلے ڈھونڈا جاتا ہے، پھر نیٹ ورک کی ہر مشین سے ایک ایک کر کے پوچھا جاتا ہے۔ پرنٹر چالو اور اسی نیٹ ورک پر ہونا چاہیے۔ کچھ پرنٹر اپنا اعلان نہیں کرتے، اور کچھ نیٹ ورک اعلان آگے نہیں پہنچاتے: ایسی صورت میں اسے اس کے پتے سے شامل کریں، جیسے 192.168.1.50۔ یہ آپ کو پرنٹر کی اپنی سکرین پر، نیٹ ورک کی ترتیبات میں ملے گا۔

# Name of the page in the list of settings pages.
settings-printers-title = پرنٹر

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = پرنٹر چھاپنا پرنٹ سکینر

# Help text at the top of the page.
settings-printers-help = آپ کے نیٹ ورک کے پرنٹر خود ہی مل جاتے ہیں۔ استعمال کے لیے کسی کے ساتھ شامل کریں دبائیں۔

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = اس کمپیوٹر پر چھپائی انسٹال نہیں ہے۔

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = تلاش جاری…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = چھپائی کی سروس جواب نہیں دے رہی، اس لیے ابھی کچھ نہیں چھپ سکتا۔

# Heading of the card shown when printing is not installed.
settings-printers-printing = چھپائی

# Help text (i button) of that card.
settings-printers-printing-about = چھپائی کے لیے CUPS چاہیے، جو اس کمپیوٹر پر نہیں ہے۔ انسٹال ہونے کے بعد آپ کے نیٹ ورک کے پرنٹر خود ہی مل جاتے ہیں۔

# Line in that card.
settings-printers-unavailable = چھپائی دستیاب نہیں

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = اسے یوں انسٹال کریں: sudo apt install cups

# Line in that card.
settings-printers-then-found = پھر پرنٹر مل جاتے ہیں

# Small line under it.
settings-printers-then-found-sub = آپ کے نیٹ ورک والے، خود ہی۔

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = چھپائی کی سروس۔

# Line in the Printers card when there are none.
settings-printers-none = ابھی کوئی پرنٹر نہیں

# Small line under "No printers yet".
settings-printers-none-sub = اس کے ساتھ ایک شامل کریں۔

# Heading of the card that lists the printers.
settings-printers-card = پرنٹر

# Help text (i button) of the Printers card.
settings-printers-card-about = اس کمپیوٹر پر لگا ہوا ہر پرنٹر۔ ستارہ کسی کو پہلے استعمال ہونے والا پرنٹر بناتا ہے؛ صفحے کا آئیکن اس پر ٹیسٹ صفحہ چھاپتا ہے؛ ردی دان اسے ہٹاتا ہے، دو بار پوچھ کر۔ تبدیلیوں کے لیے آپ کا اکاؤنٹ lpadmin گروپ میں ہونا چاہیے۔

# Red words under a page range box that cannot be used.
printers-bad-range = صفحات جیسے 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = کاپیاں: 1 سے 99

# Name of the choice to print on one side of the paper.
printers-sides-one = ایک طرف

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = دونوں، لمبے کنارے سے

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = دونوں، چھوٹے کنارے سے

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = سیدھا

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = آڑا

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = اجازت نہیں۔ آپ کا اکاؤنٹ lpadmin گروپ میں ہونا چاہیے؛ ہو جانے کے بعد ایک بار لاگ آؤٹ ہو کر دوبارہ لاگ ان کریں۔

# Red words when CUPS refuses and gives no reason.
# SAFETY: needs a human review before it ships
printers-refused = CUPS نے انکار کیا۔

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = اس پرنٹر کا نام ایسا نہیں جس کی CUPS اجازت دیتا ہو۔

# Red words when printing is not installed on the computer.
printers-not-installed = اس کمپیوٹر پر چھپائی انسٹال نہیں ہے۔

# Red words when the printer chosen has been removed since.
printers-gone = { $name } اب لگا ہوا نہیں ہے۔

# Red words when the printer chosen is paused.
printers-paused = { $name } رکا ہوا ہے۔

# Red words when sending the document to CUPS took too long.
printers-too-slow = دستاویز بھیجنے میں بہت دیر لگی۔


