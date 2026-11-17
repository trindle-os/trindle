
# machine translated (Arabic), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = نُزّل هذا الملف بأمان، لكن لم يتحقق منه أحد، لأن صاحبه لا ينشر توقيعًا.

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
store-gh-keyis-nokeyring = ليس لدى هذا الحاسوب سلسلة مفاتيح، لذلك لا يمكنه حفظ مفتاح

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = ما زال مفتاح GitHub الخاص بك في ملف عادي قديم على هذا الحاسوب، ولم يعد Trindle يقرأه. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } لذلك يستخدم Trindle GitHub بلا مفتاح، حتى لو كان هناك مفتاح محفوظ.

# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = لا يستطيع Trindle إرسال مفتاح GitHub المحفوظ لأن فيه مسافة أو فاصل سطر. لذلك يستخدم Trindle GitHub بلا مفتاح. أزِل المفتاح والصقه من جديد.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = تعذّرت إزالة الملف.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = لم تحفظه سلسلة المفاتيح، لذلك يستخدم Trindle GitHub دون مفتاح.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = المفتاح سطر واحد بلا مسافات. وفي هذا المفتاح فاصل سطر أو مسافة. الصق المفتاح وحده.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = توقف GitHub عن الرد الآن لأن { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = توقف GitHub عن الرد الآن لأن { $which }. { $when } { $hint }

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
store-gh-limit-search-key = يسمح بـ30 عملية بحث في الدقيقة

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = يسمح بـ10 عمليات بحث في الدقيقة

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = استنفدت الحد الذي يسمح به مفتاحك، وهو 5,000 طلب في الساعة

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = يسمح بـ60 طلبًا في الساعة من عنوان واحد بلا مفتاح

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = يرفع مفتاح GitHub هذا الحد إلى 5,000. اضغط زر المفتاح في أعلى يمين البحث.

# Red words when GitHub does not accept the key.
store-gh-key-refused = لم يقبل GitHub المفتاح. أنشئ مفتاحًا جديدًا، أو أزِل هذا المفتاح، باستخدام زر المفتاح في أعلى يمين البحث.

# Red words when GitHub answered with a code and no words.
store-gh-answered = أعاد GitHub الرمز { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = حجب GitHub هذا الطلب، مع أنك لم تبلغ حدّه.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = حجب GitHub هذا الطلب: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [zero] لا توجد مشاريع تعذّر على Trindle فحصها
        [one] تعذّر على Trindle فحص مشروع واحد وجده، لذلك لم يُدرج
        [two] تعذّر على Trindle فحص مشروعين وجدهما، لذلك لم يُدرجا
        [few] تعذّر على Trindle فحص { $n } مشاريع وجدها، لذلك لم تُدرج
        [many] تعذّر على Trindle فحص { $n } مشروعًا وجدها، لذلك لم تُدرج
       *[other] تعذّر على Trindle فحص { $n } مشروع وجدها، لذلك لم تُدرج
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = تعذّر الوصول إلى GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = بحث Trindle فقط عن المشاريع التي فيها AppImage، لأن GitHub لم يرد على البحث عن Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = جارٍ التنزيل

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = جارٍ التثبيت

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = لم يكتمل التنزيل. { $why }

# The size of a download that is under a megabyte.
store-size-under = أقل من 1 ميغابايت
