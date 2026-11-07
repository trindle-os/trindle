
# machine translated (Arabic), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = نُزّل بأمان، لكن لم يتحقق أحد من هذا الملف: فصاحبه لا ينشر توقيعًا.

# What a project with nothing to install here is told as.
store-gh-no-files = ليس في إصدارات هذا المشروع ملف AppImage أو Flatpak يناسب هذا الحاسوب.

# What a project that has never made a release is told as.
store-gh-no-releases = لا توجد إصدارات لهذا المشروع على GitHub.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = جارٍ البحث عن مفتاح في سلسلة المفاتيح

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = يوجد مفتاح محفوظ

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = لا يوجد مفتاح محفوظ

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = لا توجد سلسلة مفاتيح على هذا الحاسوب، فلا يمكن حفظ مفتاح

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = ما زال مفتاح GitHub الخاص بك في ملف عادي على هذا الحاسوب ويُستخدم من هناك: تعذّر نقله إلى سلسلة المفاتيح. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } لذلك يُسأل GitHub بلا مفتاح، سواء وُجد مفتاح محفوظ أم لا.

# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = يوجد مفتاح GitHub محفوظ لكن لا يمكن إرساله: فيه مسافة أو فاصل سطر، كما يحدث عند لصق المفتاح على جزأين. لذلك يُسأل GitHub بلا مفتاح. أزِل المفتاح والصقه من جديد.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = تعذّرت إزالة الملف.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = لم تقبله سلسلة المفاتيح.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = المفتاح سطر واحد بلا مسافات، وفي ما لصقته فاصل سطر أو مسافة. الصق المفتاح وحده.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = توقف GitHub عن الرد الآن: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = توقف GitHub عن الرد الآن: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = حاول مرة أخرى لاحقًا.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [zero] سيرد من جديد بعد قليل.
        [one] سيرد من جديد بعد نحو دقيقة.
        [two] سيرد من جديد بعد نحو دقيقتين.
        [few] سيرد من جديد بعد نحو { $n } دقائق.
        [many] سيرد من جديد بعد نحو { $n } دقيقة.
       *[other] سيرد من جديد بعد نحو { $n } دقيقة.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = يجيب عن 30 عملية بحث في الدقيقة

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = يجيب عن 10 عمليات بحث في الدقيقة

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = يجيب عن 5,000 سؤال في الساعة بمفتاحك، وقد استُنفدت

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = يجيب عن 60 سؤالًا في الساعة من عنوان واحد بلا مفتاح

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = يرفع مفتاح GitHub هذا الحد إلى 5,000: اضغط على المفتاح في أعلى يمين صفحة البحث في GitHub.

# Red words when GitHub does not accept the key.
store-gh-key-refused = لم يقبل GitHub المفتاح. أنشئ مفتاحًا جديدًا أو أزِله: اضغط على المفتاح في أعلى يمين صفحة البحث في GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = ردّ GitHub بالرمز { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = رفض GitHub هذا الطلب، مع أن حد الردود لم ينفد.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = رفض GitHub هذا الطلب: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [zero] لا مشاريع لم يمكن فحصها
        [one] تعذّر فحص مشروع واحد من المشاريع التي وُجدت، لذلك لم يُدرج
        [two] تعذّر فحص مشروعين من المشاريع التي وُجدت، لذلك لم يُدرجا
        [few] تعذّر فحص { $n } مشاريع من المشاريع التي وُجدت، لذلك لم تُدرج
        [many] تعذّر فحص { $n } مشروعًا من المشاريع التي وُجدت، لذلك لم تُدرج
       *[other] تعذّر فحص { $n } مشروع من المشاريع التي وُجدت، لذلك لم تُدرج
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = تعذّر الوصول إلى GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = بُحث فقط عن المشاريع التي فيها AppImage: لم يرد GitHub على البحث عن Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = جارٍ التنزيل

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = جارٍ التثبيت

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = لم يكتمل التنزيل. { $why }

# The size of a download that is under a megabyte.
store-size-under = أقل من 1 ميغابايت

