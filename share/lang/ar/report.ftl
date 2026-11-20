
# machine translated (Arabic), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = الإبلاغ عن مشكلة

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = حدث خطأ. هل تريد إرسال تقرير؟

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = يرسل هذا كلماتك وتفاصيل حاسوبك وأخطاء هذا التشغيل وسجل Trindle. تُحذف الأسماء والعناوين (البريد الإلكتروني والشبكة وWi-Fi) مما يجمعه Trindle. وقد تبقى أسماء الملفات وعناوين الويب في أسطر السجل. لا يرسل ملفاتك ولا كلمات مرورك.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = ماذا حدث؟

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = أي تطبيق؟

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = لست متأكدًا

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = برنامج آخر

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = سطح المكتب

# SAFETY: needs a human review before it ships
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = إرسال

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when the person opened it themselves.
report-cancel = إلغاء

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = لا ترسل

# Button that opens the exact text of the report.
report-show-details = إظهار التفاصيل

# Button that closes the exact text of the report.
report-hide-details = إخفاء التفاصيل

# Button under the shown report text that copies it.
report-copy = نسخ

# Button under the shown report text that saves it as a file in Downloads.
report-save = حفظ

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = لا يمكن إرسال التقارير بعد.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = جارٍ التحقق.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = تقارير كثيرة في وقت واحد.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = بانتظار الاتصال.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = بانتظار الاتصال. { $why }

# Under the typing box after the report was sent.
report-sent = أُرسل. شكرًا لك.

# Under the typing box while the report is being posted.
report-sending = جارٍ الإرسال.

# Under the typing box after Save. $name is the file's name.
report-saved-as = حُفظ باسم Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = أدخل اسم مستخدم Buildmorphic وليس عنوان بريد إلكتروني. لا تستخدم @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = يجب أن يكون طول اسم المستخدم من 2 إلى 31 حرفًا، ويمكن أن يضم أحرفًا صغيرة وأرقامًا وشرطات.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = يتكون اسم GitHub من حروف وأرقام وشرطات فقط.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = تعذّر على Trindle حفظ التقرير لوقت لاحق: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = تعذّر الحفظ: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = لا إذن

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = القرص ممتلئ

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = المجلد مفقود

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = خطأ في القرص

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl غير موجود.

# Why a report could not be posted: the post could not be started.
report-not-posted = تعذّر إرساله.

# Why a report could not be posted: the computer is offline.
report-no-connection = لا اتصال.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = أرسل الخادم الرمز { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = اكتب بضع كلمات أخرى عمّا حدث.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = جارٍ قراءة السجل.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = لا يوجد سجل يُرسل مع التقرير، لذا لا يمكن إرساله. احفظه بدلاً من ذلك.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = صورة اختيارية. لا يراها إلا صاحب المشروع.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = اختر صورة PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = التقط لقطة شاشة

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = أزل الصورة

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = صورتك، { $width } في { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = اختر صورة PNG

report-picture-filter = صور PNG

# Under the picture while it is being read.
report-picture-reading = جارٍ القراءة.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = تعذّر التقاط لقطة الشاشة.

report-picture-no-chooser = تعذّر على Trindle فتح نافذة اختيار الملفات.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = ليست صورة PNG.

report-picture-too-big = أكبر من { $megabytes } ميغابايت.

report-picture-too-large = الصورة كبيرة جدًا. يمكن أن يصل عرضها إلى { $side } بكسل وارتفاعها إلى { $side } بكسل.

report-picture-damaged = الصورة تالفة.

report-picture-unreadable = تعذّرت قراءتها.

report-sent-as-bug = أُرسل. رقم تقرير الخلل الخاص بك هو { $number }. شكرًا لك.

# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle في مرحلة تجريبية. التقارير مجهولة الهوية: تُحذف الأسماء وعناوين البريد الإلكتروني والمجلدات الرئيسية. اسم المستخدم اختياري.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = اسم مستخدم Buildmorphic (اختياري)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = تعذّر على Trindle إرسال التقرير. { $why } اضغط على إرسال للمحاولة مرة أخرى.
