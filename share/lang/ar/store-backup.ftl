
# machine translated (Arabic), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = لم يُسترجع شيء، لأن { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = لم يسترجع Trindle إلا جزءًا من النسخة الاحتياطية، لأن { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = النسخة الاحتياطية تحتوي على رابط أو شيء آخر يصل إلى خارج المجلد الذي توضع فيه

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = يوجد على هذا الحاسوب شيء في المكان الذي يجب أن يوضع فيه جزء من النسخة الاحتياطية

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = لا يوجد مكان لفك ضغط النسخة الاحتياطية

# The reason when the file is not a Trindle backup.
store-because-not-trindle = هذا الملف ليس نسخة احتياطية من Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = النسخة الاحتياطية تالفة وتعذّر على Trindle قراءة قائمتها

# The reason when the file is not a Flat backup.
store-because-not-flat = لا يبدو هذا الملف نسخة احتياطية من Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = لا يحتوي هذا الملف على قائمة تطبيقات Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = صُنعت بواسطة إصدار من Flat أحدث مما يستطيع Trindle قراءته

# The reason when the file holds no apps.
store-because-no-apps = لا توجد تطبيقات في هذا الملف

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = تعذّرت قراءة النسخة الاحتياطية: { $why }

# The reason when the program gave none.
store-because-none = لم يُذكر سبب

# The reason when the system said permission was denied.
store-because-permission = لا يُسمح لـ Trindle بالكتابة هناك

# The reason when the disk is full.
store-because-space = لم تعد هناك مساحة على هذا الحاسوب

# The reason when a part of the computer is read-only.
store-because-read-only = ذلك الجزء من هذا الحاسوب للقراءة فقط

# The reason when the person's disk quota is used up.
store-because-quota = المساحة المسموح بها لهذا الحساب قد نفدت

# The reason when a file or program is missing.
store-because-missing = ملف أو برنامج يحتاجه غير موجود

# The reason when no reason could be read from the program's last line.
store-because-failed = فشل

# The reason when the backup has no name to be written under.
store-because-no-name = ليس له اسم

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = إعداداته في النسخة الاحتياطية تصل إلى خارج مجلده الخاص

# The reason when something is too big to be read whole.
store-because-too-big = هو أكبر من أن يُقرأ

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = إعداداته في النسخة الاحتياطية تالفة أو تغيّرت

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = استُرجع كل شيء عدا صورة حسابك. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } ولم يسترجع Trindle صورة حسابك أيضًا. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = تعذّر طلب كلمة المرور: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = تعذّر على Trindle قراءته من النسخة الاحتياطية: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = لم ينشئ Trindle النسخة الاحتياطية، لأن { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = لا شيء بعد

# Beside the size of a part of the backup: about how big it is.
store-backup-about = نحو { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = جارٍ استرجاع إعدادات Trindle وملفات AppImage…

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = استرجع Trindle إعداداتك وبياناتك، لكن Flatpak رفض تثبيت هذه التطبيقات: { $failed }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = استرجع Trindle إعداداتك وبياناتك، لكنه لم يثبّت هذه التطبيقات: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [zero] لم يتخطَّ Trindle أي تطبيق.
        [one] تخطّى Trindle تطبيقًا واحدًا، لأن اسمه غير صالح.
        [two] تخطّى Trindle تطبيقين، لأن اسميهما غير صالحين.
        [few] تخطّى Trindle { $n } تطبيقات، لأن أسماءها غير صالحة.
        [many] تخطّى Trindle { $n } تطبيقًا، لأن أسماءها غير صالحة.
       *[other] تخطّى Trindle { $n } تطبيق، لأن أسماءها غير صالحة.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = المصدر { $name } غير مشفّر، لذلك لم يستخدمه Trindle

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = المصدر { $name } غير موجود على هذا الحاسوب والنسخة الاحتياطية لا تذكر مكانه

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = إعدادات { $name } (لأن نسختها في النسخة الاحتياطية لم تجتز فحوصها)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (لأن اسم مصدره غير صالح)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (لأن { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (تعذّر على Flatpak تثبيته: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = كل شيء (لأنه لا يوجد مكان لفك ضغط النسخة الاحتياطية)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = أذونات كل تطبيق

# Said among what could not be done: one app's settings.
store-flat-app-settings = إعدادات { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. ما كان هناك من قبل محفوظ باسم { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = جارٍ تثبيت { $name } ({ $at } من { $total })…

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = جارٍ استرجاع إعدادات { $name } ({ $at } من { $total })…

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [zero] لم يُثبَّت أي تطبيق.
        [one] ثُبّت تطبيق واحد.
        [two] ثُبّت تطبيقان.
        [few] ثُبّتت { $n } تطبيقات.
        [many] ثُبّت { $n } تطبيقًا.
       *[other] ثُبّت { $n } تطبيق.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
        [zero] لم تُسترجع إعدادات أي تطبيق.
        [one] استُرجعت إعدادات تطبيق واحد.
        [two] استُرجعت إعدادات تطبيقين.
        [few] استُرجعت إعدادات { $n } تطبيقات.
        [many] استُرجعت إعدادات { $n } تطبيقًا.
       *[other] استُرجعت إعدادات { $n } تطبيق.
    }

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [zero] لا تطبيق كان مثبّتًا من قبل.
        [one] كان تطبيق واحد مثبّتًا من قبل.
        [two] كان تطبيقان مثبّتين من قبل.
        [few] كانت { $n } تطبيقات مثبّتة من قبل.
        [many] كان { $n } تطبيقًا مثبّتًا من قبل.
       *[other] كان { $n } تطبيق مثبّتًا من قبل.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = تُركت: { $list }

# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }: أعاد Trindle فقط الأذونات التي تسحب الوصول.
