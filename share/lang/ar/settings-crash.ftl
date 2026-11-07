
# machine translated (Arabic), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = تقارير الأعطال

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = تقارير أعطال أخطاء علل إرسال خصوصية قياس عن بعد buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = عندما يتعطل أحد برامج Trindle، تحتفظ إضافة تقارير التعطل بملاحظة قصيرة على هذا الحاسوب. ولا ترسل شيئًا حتى تشغّل أمر الإرسال الخاص بها وتوافق على كل تقرير، وهو يعرض التقرير كاملًا أولًا. لا يدخل اسمك واسمك على GitHub إلا إذا كتبتهما هنا. ولا ترسل أبدًا رسالة أو اسم ملف أو مكانك أو أي شيء عن برامج أخرى. وإزالة الإضافة توقف كل شيء.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = الأعطال

# Under the Crashes switch.
settings-crash-crashes-short = أين في شيفرة Trindle حدث العطل.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = تُرسل الأعطال البرنامج وإصداره ونوع العطل وموضعه في شيفرة Trindle.

# Heading and label of the switch for sending errors.
settings-crash-errors = الأخطاء

# Under the Errors switch.
settings-crash-errors-short = اسم الخطأ الثابت، وموضعه.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = تُرسل الأخطاء البرنامج وإصداره والاسم الثابت الذي يعطيه Trindle للخطأ وموضعه في شيفرته.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = عن هذا الحاسوب

# Under the About this computer switch.
settings-crash-computer-short = Linux، المعالج، الذاكرة، الرسوميات، الشاشات.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = يضيف خيار عن هذا الحاسوب إلى كل تقرير: إصدار Linux الذي يعمل عليه، ونوع المعالج، والذاكرة مقرّبة إلى الأدنى، وWayland أو X11، وصانع الرسوميات، وعدد الشاشات.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = ما الذي يُرسل

# Help text of the What is sent card: { $switches } is what each switch sends.
# SAFETY: needs a human review before it ships
settings-crash-what-about = { $switches } لا تحمل التقارير أي اسم ما لم تكتب اسم مستخدم Buildmorphic في المربع أدناه، وهذا يمنحك الفضل فقط.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = لم يتغير شيء، لأن { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = لم يتغير شيء، لأن الملف الذي يحفظه ليس كما كتبه Trindle.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = لا يوجد مجلد للإعدادات.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = تعذّر إنشاء مجلد الإعدادات.

# Red words when a setting could not be written to its file.
settings-crash-not-written = تعذّرت كتابة الإعداد.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = عطل ({ $signal }) في { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = خطأ { $code } في { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = استُلم

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = لم يُرسل: المتتبع مغلق

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = لم يُرسل: أكبر من اللازم

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = لم يُرسل: رُفض

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = لم يُرسل: كثير جدًا دفعة واحدة

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = لم يُرسل

# Said beside the link line once it is saved.
settings-crash-saved = تم الحفظ

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = امسح القائمة

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = تعذّر مسح القائمة.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = تمت الإزالة

# Heading of the line that holds the clear button.
settings-crash-kept-here = محفوظة على هذا الحاسوب فقط

# Under the Kept on this computer only heading.
settings-crash-newest-first = الأحدث أولًا، حتى خمسين.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = لا شيء بعد

# Under the Nothing yet heading.
settings-crash-listed-here = يُدرج هنا كل تقرير، تمامًا كما أُرسل.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = اعرض بالضبط ما أُرسل

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }، مرة واحدة: { $answer }
        [two] { $when }، مرتين: { $answer }
        [few] { $when }، { $n } مرات: { $answer }
        [many] { $when }، { $n } مرة: { $answer }
        [zero] { $when }، { $n } مرة: { $answer }
       *[other] { $when }، { $n } مرة: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = ما أُرسل حتى الآن

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = يُدرج كل تقرير تمامًا كما أُرسل، الأحدث أولًا، ويُحفظ على هذا الحاسوب فقط. استُلم يعني أن Buildmorphic حصل عليه. لم يُرسل: المتتبع مغلق يعني أن متتبع أخطاء Trindle لا يستقبل تقارير؛ رُفض يعني أنه رُفض لأنه مكتوب خطأً؛ كثير جدًا دفعة واحدة يعني أن عددًا كبيرًا وصل في وقت واحد.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = بعد حدوث عطل يُطلب منك القرار، مع عرض التقرير كاملًا، هل يُرسل.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = لا يُرسل شيء، ولا يُطلب منك شيء بعد حدوث عطل.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = لا يُرسل شيء: الأعطال والأخطاء كلاهما متوقف.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] تُبلَّغ أخطاء برامج Trindle دون ذكر اسم.
        [both] تُبلَّغ أعطال برامج Trindle وأخطاؤها دون ذكر اسم.
       *[crashes] تُبلَّغ أعطال برامج Trindle دون ذكر اسم.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = اسأل عن المشكلات وشارك التقارير

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = الإيقاف يخفي كل شيء ويحذف التقارير المنتظرة.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = متوقف. لم تكن هناك تقارير منتظرة.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [zero] متوقف. لم يُحذف أي تقرير منتظر.
        [one] متوقف. حُذف تقرير منتظر واحد.
        [two] متوقف. حُذف تقريران منتظران.
        [few] متوقف. حُذفت { $n } تقارير منتظرة.
        [many] متوقف. حُذف { $n } تقريرًا منتظرًا.
       *[other] متوقف. حُذف { $n } تقرير منتظر.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = حول المجتمع

# Heading of the line with the community button.
settings-crash-go-to-bm = التقارير تذهب إلى Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = المجتمع الذي يساعد في بناء Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = اسم المستخدم

# Heading of the line with the username box.
settings-crash-your-username = اسم المستخدم الخاص بك في Buildmorphic

# Under the username heading.
settings-crash-username-sub = اختياري. ليس عنوان بريد إلكتروني أبدًا.

# Heading of a line about joining with an email address.
settings-crash-join = انضم بعنوان بريد إلكتروني فقط

# Under the Join heading.
settings-crash-join-sub = اكسب رصيدًا ومستويات عن المشكلات التي تبلّغ عنها أولًا.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = يحمل التقرير قائمة ثابتة فقط

# Under the fixed list heading.
settings-crash-fixed-sub = تراه أولًا، ولا يُرسل شيء دون أن تضغط أنت.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = التقارير التي ترسلها

# Help text of the Reports you send card (the information button).
settings-crash-reports-about = عندما يتعطل أحد برامج Trindle، يمكن أن تُفتح نافذة تسأل هل ترسل تقريرًا. تعرض النص بالضبط أولًا: كلماتك، وطراز الحاسوب والرسوميات، والخطأ، والسجل قبل لحظات من العطل. لا تُقرأ الملفات ولا كلمات المرور ولا أسماء Wi-Fi أبدًا، وتُمحى الأسماء والعناوين. ويمكنك فتح النافذة نفسها في أي وقت من زر الإبلاغ عن مشكلة في الحلقة.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = اسم github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = اسمك في GitHub

# Small words under that heading.
settings-crash-github-about = اختياري. يظهر بوصفه المُبلِّغ.

