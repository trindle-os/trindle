# machine translated (Arabic), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = لا توجد طابعات بعد. أي طابعات تُوجد على شبكتك تظهر أدناه.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } متوقفة مؤقتًا، لذا لن يُطبع عليها شيء الآن.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } تطبع.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = جاهزة للطباعة على { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } الحالة: { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } جاهزة للاستخدام؛ ولم تُختر طابعة للطباعة عليها أولًا.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } طابعات جاهزة للاستخدام؛ ولم تُختر واحدة للطباعة عليها أولًا.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = هذه ليست مهمة يعرفها CUPS.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = أُلغيت

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = فشل lpstat

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
settings-printers-adding = جارٍ الإضافة...

# Shown beside a printer or the address line after it was added.
settings-printers-added = تمت الإضافة

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = جارٍ البحث عن برنامج تشغيلها...

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
settings-printers-jobs-about = كل مهمة تنتظر الطباعة، على أي طابعة، كما هي الآن. تُقرأ القائمة من جديد كل بضع ثوانٍ ما دامت هذه الصفحة مفتوحة. علامة الصليب تلغي مهمة؛ وما طُبع من الورق يبقى مطبوعًا.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS لا يردّ

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = تتعذّر قراءة قائمة الانتظار.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = جارٍ النظر…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = قائمة الانتظار.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = لا شيء ينتظر

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = تظهر المهام هنا أثناء طباعتها.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = متوقفة مؤقتًا

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = خاملة

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = تطبع

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = الافتراضية، { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = اطبع على هذه أولًا

# Tip of the button that prints a test page.
settings-printers-test-page = اطبع صفحة اختبار

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = أزل هذه الطابعة

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = يحتاج البحث إلى cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = ثبّته بـ: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = أضف هذه الطابعة

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = لم يُعثر على شيء جديد

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = هل هي مشغَّلة، وعلى الشبكة نفسها؟

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = المعلنة أولًا، ثم كل جهاز بالتتابع.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = أضفها

# Heading of the line where a printer's address is typed.
settings-printers-by-address = بعنوانها

# Small line under "By its address".
settings-printers-by-address-sub = مثل 192.168.1.50، على شاشتها نفسها.

# Heading of the card for adding printers.
settings-printers-add-card = أضف طابعة

# Help text (i button) of the Add a printer card.
settings-printers-add-about = يُبحث أولًا عن الطابعات التي تعلن عن نفسها، ثم يُسأل كل جهاز على الشبكة بالتتابع. يجب أن تكون الطابعة مشغَّلة وعلى الشبكة نفسها. بعض الطابعات لا تعلن عن نفسها، وبعض الشبكات لا تمرّر الإعلان: أضف واحدة بعنوانها بدلًا من ذلك، مثل 192.168.1.50. تجده على شاشة الطابعة نفسها، ضمن إعدادات الشبكة.

# Name of the page in the list of settings pages.
settings-printers-title = الطابعات

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = printer print scanner طابعة طباعة ماسح ضوئي

# Help text at the top of the page.
settings-printers-help = تُعثر على الطابعات في شبكتك من تلقاء نفسها. اضغط إضافة بجانب واحدة لاستخدامها.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = الطباعة غير مثبتة على هذا الحاسوب.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = جارٍ النظر…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = خدمة الطباعة لا تردّ، لذا لا يمكن الطباعة الآن.

# Heading of the card shown when printing is not installed.
settings-printers-printing = الطباعة

# Help text (i button) of that card.
settings-printers-printing-about = تحتاج الطباعة إلى CUPS، وهو غير موجود على هذا الحاسوب. بمجرد تثبيته، تُوجد الطابعات في شبكتك من تلقاء نفسها.

# Line in that card.
settings-printers-unavailable = الطباعة غير متاحة

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = ثبّته بـ: sudo apt install cups

# Line in that card.
settings-printers-then-found = بعدها تُوجد الطابعات

# Small line under it.
settings-printers-then-found-sub = تلك الموجودة على شبكتك، من تلقاء نفسها.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = خدمة الطباعة.

# Line in the Printers card when there are none.
settings-printers-none = لا توجد طابعات بعد

# Small line under "No printers yet".
settings-printers-none-sub = أضف واحدة بجانب هذا.

# Heading of the card that lists the printers.
settings-printers-card = الطابعات

# Help text (i button) of the Printers card.
# SAFETY: needs a human review before it ships
settings-printers-card-about = كل طابعة مُعدّة على هذا الحاسوب. النجمة تجعل واحدة الطابعة المستخدمة أولًا؛ وأيقونة الصفحة تطبع عليها صفحة اختبار؛ وسلة المهملات تزيلها، بعد سؤالين. التغييرات تحتاج إلى أن يكون حسابك في مجموعة lpadmin.

# Red words under a page range box that cannot be used.
printers-bad-range = صفحات مثل 1-3، 5

# Red words under a copies box that cannot be used.
printers-bad-copies = النسخ: من 1 إلى 99

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
printers-not-allowed = غير مسموح. يجب أن يكون حسابك في مجموعة lpadmin؛ سجّل الخروج ثم الدخول بعد ذلك.

# Red words when CUPS refuses and gives no reason.
printers-refused = رفض CUPS.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = اسم تلك الطابعة ليس مما يسمح به CUPS.

# Red words when printing is not installed on the computer.
printers-not-installed = الطباعة غير مثبتة على هذا الحاسوب.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } لم تعد مُعدّة.

# Red words when the printer chosen is paused.
printers-paused = { $name } متوقفة مؤقتًا.

# Red words when sending the document to CUPS took too long.
printers-too-slow = استغرق إرسال المستند وقتًا طويلًا.

