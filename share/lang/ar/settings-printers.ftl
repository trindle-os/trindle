# machine translated (Arabic), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = لا توجد طابعات بعد. أي طابعات يجدها Trindle على شبكتك تظهر أدناه.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } متوقفة مؤقتًا، لذا لن يُطبع عليها شيء الآن.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } تطبع.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = جاهزة للطباعة على { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } الحالة: { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } جاهزة للاستخدام، لكنك لم تختر طابعة افتراضية.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } طابعات جاهزة للاستخدام، لكنك لم تختر طابعة افتراضية.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = هذه المهمة لم تعد موجودة.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = أُلغيت

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = فشل الفحص.

# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = خدمة الطباعة لا تعمل أو لا يمكن الوصول إليها.

# Words in the ring while the network is searched for printers.
settings-printers-searching = جارٍ البحث

# Words in the ring when the search is finished.
settings-printers-search-done = تم

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = أصبحت الافتراضية

# Shown beside a printer after a test page was sent.
settings-printers-sent = أُرسلت

# Shown beside a printer after it was removed.
settings-printers-removed = أُزيلت

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = اكتب عنوان الطابعة أولًا.

# Shown under the By its address line while the printer is added.
settings-printers-adding = جارٍ الإضافة…

# Shown beside a printer or the address line after it was added.
settings-printers-added = تمت الإضافة

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = جارٍ البحث عن برنامج تشغيلها…

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = المهمة { $number } على { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }، { $size }، { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = ألغِ هذه المهمة

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = مهام الطباعة

# Help text (i button) of the Print jobs card.
# SAFETY: needs a human review before it ships
settings-printers-jobs-about = كل مهمة تنتظر الطباعة، على أي طابعة. يحدّث Trindle القائمة كل بضع ثوانٍ ما دامت هذه الصفحة مفتوحة. لا يمكن إلغاء الصفحات التي طُبعت بالفعل.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS لا يستجيب

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = تعذّرت قراءة قائمة الانتظار.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = جارٍ النظر…

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = لا شيء ينتظر

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = متوقفة مؤقتًا

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = خاملة

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = تطبع

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = الافتراضية، { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = اجعلها الافتراضية

# Tip of the button that prints a test page.
settings-printers-test-page = اطبع صفحة اختبار

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = أزل هذه الطابعة

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = يحتاج البحث إلى cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = لتثبيته، شغّل الأمر: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = أضف هذه الطابعة

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = لم يُعثر على شيء جديد

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = أضفها

# Heading of the line where a printer's address is typed.
settings-printers-by-address = أضف بالعنوان

# Heading of the card for adding printers.
settings-printers-add-card = أضف طابعة

# Help text (i button) of the Add a printer card.
settings-printers-add-about = يبحث Trindle أولًا عن الطابعات التي تعلن عن نفسها، ثم يفحص كل جهاز على الشبكة بالتتابع. يجب أن تكون الطابعة مشغَّلة وعلى الشبكة نفسها. بعض الطابعات لا تعلن عن نفسها، وبعض الشبكات لا تمرّر الإعلان. في هذه الحالة أضف الطابعة بعنوانها بدلًا من ذلك، مثل 192.168.1.50. ستجد العنوان على شاشة الطابعة نفسها، ضمن إعدادات الشبكة.

# Name of the page in the list of settings pages.
settings-printers-title = الطابعات

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = printer print scanner طابعة طباعة ماسح ضوئي

# Help text at the top of the page.
settings-printers-help = يجد Trindle الطابعات على شبكتك من تلقاء نفسه. لاستخدام واحدة، اضغط أضف هذه الطابعة بجانبها.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = الطباعة غير مثبتة على هذا الحاسوب.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = جارٍ النظر…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = لا يستطيع Trindle الوصول إلى خدمة الطباعة، لذا لا يمكن الطباعة الآن.

# Heading of the card shown when printing is not installed.
settings-printers-printing = الطباعة

# Help text (i button) of that card.
settings-printers-printing-about = تحتاج الطباعة إلى CUPS، وهو غير موجود على هذا الحاسوب. بمجرد تثبيته، يجد Trindle الطابعات في شبكتك من تلقاء نفسه.

# Line in that card.
settings-printers-unavailable = الطباعة غير متاحة

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = ثبّته بـ: sudo apt install cups

# Line in that card.
settings-printers-then-found = العثور على الطابعات

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = خدمة الطباعة

# Line in the Printers card when there are none.
settings-printers-none = لا توجد طابعات بعد

# Heading of the card that lists the printers.
settings-printers-card = الطابعات

# Help text (i button) of the Printers card.
# SAFETY: needs a human review before it ships
settings-printers-card-about = كل طابعة مُعدّة على هذا الحاسوب. يجب أن يكون حسابك في مجموعة lpadmin لإجراء التغييرات.

# Red words under a page range box that cannot be used.
printers-bad-range = على سبيل المثال: 1-3، 5

# Red words under a copies box that cannot be used.
printers-bad-copies = من 1 إلى 99 نسخة.

# Name of the choice to print on one side of the paper.
printers-sides-one = وجه واحد

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = الوجهان، الحافة الطويلة

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = الوجهان، الحافة القصيرة

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
printers-orientation-portrait = عمودي

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = أفقي

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = لا يُسمح لك بذلك. يجب أن يكون حسابك في مجموعة lpadmin. بعد ذلك سجّل الخروج ثم الدخول.

# Red words when CUPS refuses and gives no reason.
printers-refused = رفض CUPS.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = اسم الطابعة هذا غير مسموح به.

# Red words when printing is not installed on the computer.
printers-not-installed = الطباعة غير مثبتة على هذا الحاسوب.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } لم تعد مُعدّة.

# Red words when the printer chosen is paused.
printers-paused = { $name } متوقفة مؤقتًا.

# Red words when sending the document to CUPS took too long.
printers-too-slow = استغرق إرسال المستند وقتًا طويلًا.

# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = تعذّر على Trindle إلغاء المهمة.

# Red words when a printer could not be made the default.
settings-printers-not-default = تعذّر على Trindle جعلها الطابعة الافتراضية.

# Red words when the test page could not be sent.
settings-printers-not-sent = تعذّر على Trindle إرسال صفحة الاختبار.

# Red words when a printer could not be removed.
settings-printers-not-removed = تعذّر على Trindle إزالة الطابعة.

# Red words when a printer could not be added.
settings-printers-not-added = تعذّر على Trindle إضافة الطابعة.

# Red words when the list of printers could not be read.
settings-printers-not-read = تعذّر على Trindle قراءة قائمة الطابعات.
