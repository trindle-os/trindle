
# machine translated (Arabic), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = اللقطات

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = لقطات لقطة استرجاع تراجع رجوع نسخة احتياطية تحديث تعطل btrfs snapper قائمة الإقلاع grub إصدارات سابقة إصدار أقدم إصدار سابق ملف محذوف إعادة استعادة المنزل نسخ

# Help line at the foot of the Snapshots page.
settings-snap-help = النظام كما كان قبل كل تغيير في البرامج وبعده، وكلما أخذت لقطة بنفسك. استرجاع لقطة يعيد النظام كله إليها من التشغيل التالي؛ وتبقى ملفاتك كما هي إذا كان للمجلد الشخصي جزء خاص به من القرص، كما يجعله مثبّت Trindle. وإذا لم يبدأ الحاسوب، اختر لقطات Trindle من قائمة الإقلاع. وتحتفظ الإصدارات السابقة بنسخة من ملفات كل شخص كل يوم، بمعزل عن لقطات النظام.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = لم تؤخذ نسخة: القرص ممتلئ تقريبًا.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = اليوم في { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = أمس في { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = في { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = لم يُعد الحاسوب تشغيل نفسه. أعد تشغيله من الحلقة بدلًا من ذلك.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = جارٍ أخذ لقطة

# Said beside the set aside line while the space is freed.
settings-snap-freeing = جارٍ التحرير...

# Said beside the set aside line once the space is freed.
settings-snap-freed = تم التحرير

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = جارٍ الاسترجاع

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = النظام سيكون اللقطة { $n } من التشغيل التالي. كل ما تغيّر منذ ذلك الحين يُحفظ جانبًا، لا يضيع.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = جارٍ الحذف

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = تم الحذف

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = بانتظار كلمة المرور...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = تؤخذ نسخة كل يوم

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = لا تؤخذ نسخ أخرى

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = تم الحفظ

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = جارٍ المسح

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = تم المسح

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] لا نسخ بعد
        [one] نسخة واحدة
        [two] نسختان
        [few] { $n } نسخ
        [many] { $n } نسخة
        [zero] { $n } نسخة
       *[other] { $n } نسخة
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = تؤخذ الأولى خلال يوم.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = تشغل { $size }، قيست { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = تُقاس مساحتها مرة في اليوم.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = امسحها كلها الآن

# Heading of the card about earlier versions of files.
settings-snap-earlier = الإصدارات السابقة

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = يستطيع Trindle الاحتفاظ بنسخة من ملفات الجميع كما كانت كل يوم، فيمكن فتح إصدار أقدم من ملف، أو ملف حُذف بالخطأ، أو إرجاعه من المجلدات. يستطيع كل شخص أن يقرأ في النسخة ما كان يستطيع قراءته في ذلك اليوم فقط، فيبقى المجلد الخاص خاصًا؛ أما الملف الذي كان مفتوحًا للآخرين فيبقى مفتوحًا لهم في تلك النسخة إلى أن تزول. النسخ منفصلة عن لقطات النظام: استرجاع النظام لا يغيّر ملفاتك أبدًا، وهذه النسخ لا تغيّر النظام. الثمن هو المساحة: الملف الذي تحذفه أو تغيّره يظل يشغل مساحته إلى أن تزول آخر نسخة تحتويه، لذا لا يحرر الحذف القرص فورًا. ومسح النسخ يحرر المساحة الآن لكل من يستخدم هذا الحاسوب، ولا يمكن التراجع عنه. لا تؤخذ نسخة حين يكون القرص ممتلئًا تقريبًا. وتغيير أي من هذا يتطلب كلمة مرور المدير.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = احتفظ بإصدارات سابقة من ملفاتك

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = تبقى الملفات المحذوفة { $time } قبل تحرير مساحتها.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = احتفظ بنسخة كل يوم لمدة

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = مدة الاحتفاظ بنسخة كل يوم

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = جارٍ النظر في اللقطات…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = تعذّرت قراءة اللقطات.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = بدأ الحاسوب من اللقطة { $n }، لذلك لا يُحفظ أي شيء يُفعل الآن.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [zero] لا لقطات للرجوع إليها.
        [one] لقطة واحدة للرجوع إليها، وهي الأحدث من { $when }.
        [two] لقطتان للرجوع إليهما، والأحدث من { $when }.
        [few] { $n } لقطات للرجوع إليها، والأحدث من { $when }.
        [many] { $n } لقطة للرجوع إليها، والأحدث من { $when }.
       *[other] { $n } لقطة للرجوع إليها، والأحدث من { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = لا لقطات بعد. تؤخذ لقطة قبل كل تغيير في البرامج.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }، تبقى إلى أن تُحذف

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = أعد النظام إلى هذه اللقطة

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = احذف هذه اللقطة

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = أعد التشغيل الآن

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = لم تُعدّ بعد.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = أعد تثبيت حزمة trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = اللقطات غير معدّة. إعادة تثبيت حزمة trindle تعدّها: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = نوع آخر

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = تحتاج إلى Btrfs؛ وهذا الحاسوب يستخدم { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = اختر Btrfs عند تثبيت Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = تحتاج اللقطات إلى أن يكون النظام على Btrfs، وهذا الحاسوب يستخدم { $fs }. اختر Btrfs للنظام عند تثبيت Debian لتحصل عليها.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = لا توجد لقطات على هذا الحاسوب.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = لا لقطات

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = لا لقطات على هذا الحاسوب

# Heading of the line saying how to have snapshots.
settings-snap-to-have = للحصول عليها

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = جارٍ النظر…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = تعذّرت قراءة اللقطات

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = لا شيء بعد

# Under the None yet heading.
settings-snap-none-line-sub = تؤخذ لقطة قبل كل تغيير في البرامج.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = تستطيع كل لقطة إعادة النظام كله إلى ما كان عليه حينها، من التشغيل التالي، أو أن تُحذف. كلاهما يطلب الضغط مرتين ويحتاج إلى كلمة مرور المدير. وبعد الاسترجاع يظهر بجانبها زر لإعادة التشغيل.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = خذ لقطة الآن

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = ليس أثناء التشغيل من لقطة.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = تبقى إلى أن تحذفها.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [zero] لا أنظمة محفوظة جانبًا بالاسترجاع
        [one] نظام واحد محفوظ جانبًا بالاسترجاع
        [two] نظامان محفوظان جانبًا بالاسترجاع
        [few] { $n } أنظمة محفوظة جانبًا بالاسترجاع
        [many] { $n } نظامًا محفوظًا جانبًا بالاسترجاع
       *[other] { $n } نظام محفوظ جانبًا بالاسترجاع
    }

# Under the set aside heading.
settings-snap-free-space = حرّر المساحة حين يسير كل شيء على ما يرام.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = حرّر المساحة التي تشغلها

# Heading of the card for taking a snapshot.
settings-snap-take-one = خذ واحدة

# Help text of the Take one card (the information button).
settings-snap-take-about = خذ لقطة قبل تجربة شيء قد ترغب في التراجع عنه؛ وتبقى اللقطة التي تأخذها إلى أن تحذفها. وما دام الحاسوب يعمل من لقطة، لن يُحفظ شيء مما تأخذه، لذا أعد التشغيل بشكل عادي أولًا. كل استرجاع يحفظ جانبًا ما كان عليه النظام قبله مباشرة، فلا يضيع شيء؛ حرّر المساحة التي تشغلها حين يسير كل شيء على ما يرام.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = بدأ من اللقطة { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = لا يُحفظ أي شيء يُفعل الآن.

# Heading of a line on the Before restoring card.
settings-snap-files-too = ملفاتك موجودة في اللقطات أيضًا

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = الاسترجاع يعيد ملفاتك أيضًا.

# Heading of the card about what to know before restoring.
settings-snap-before = قبل الاسترجاع

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = عند التشغيل من لقطة، لا يُحفظ أي شيء يُفعل الآن: إذا كان هذا هو الشكل الذي يجب أن يكون عليه النظام، فاضغط استرجاع عليها في القائمة، ثم أعد التشغيل. على هذا الحاسوب قد يكون المجلد الشخصي جزءًا من النظام، وعندها يعيد استرجاع لقطة ملفاتك أنت أيضًا إلى ما كانت عليه حينها، لذا انسخ أي شيء جديد إلى مكان آمن أولًا.



# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = ملفات النظام مقفلة

# Under the System files locked switch.
settings-snap-locked-sub = لا يفتح قفلها إلا التثبيت والتحديث.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = مقفلة

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = غير مقفلة

