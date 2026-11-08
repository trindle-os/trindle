
# machine translated (Arabic), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = تثبيت { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = الترحيب

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = القرص

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = الخصوصية

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = كلمة المرور

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = أنت

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = الجاهزية

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = التثبيت

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = تم

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = جارٍ التحضير

# Red line at the foot when the installer's helper could not be started.
installer-no-start = تعذّر بدء المثبّت: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = تعذّرت قراءة الخطة.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = تعذّرت قراءة الخطة: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = توقف التثبيت. لم يُكتب شيء آخر.

# Name of the round button at the foot that goes back a page.
installer-back = رجوع

# Name of the main button at the foot that goes to the next page.
installer-next = التالي

# SAFETY: needs a human review before it ships
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = تثبيت

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = إعادة التشغيل الآن

# Beside the keyboard test box on the first page.
installer-try-keyboard = جرّب لوحة المفاتيح

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = اكتب هنا لتجربة لوحة المفاتيح

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = انقر مرة أخرى للتثبيت

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = انقر مرة أخرى للمسح والتثبيت

# Note beside Next on the first page while no language is chosen.
installer-missing-language = اختر لغة

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = اختر لوحة مفاتيح

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = اختر توقيتًا

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = اختر أين يوضع Trindle

# SAFETY: needs a human review before it ships
# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = اكتب كلمة مرور القرص مرتين، بالشكل نفسه في المرتين

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = اسم تسجيل الدخول مطلوب

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = أعطِ هذا الحاسوب اسمًا

# SAFETY: needs a human review before it ships
# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = اكتب كلمة مرورك مرتين، بالشكل نفسه في المرتين

# Title of the first page.
installer-welcome-title = مرحبًا بك في { $name }

# Sentence under the title of the first page.
installer-welcome-help = اختر لغتك؛ تتبعها لوحة المفاتيح والوقت، ويمكنك تغيير أي منهما.

# Head of the card for choosing the language, and the name of its search box.
installer-language = اللغة

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = لوحة المفاتيح

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = الوقت

# Under a card's head while nothing has been chosen in its list.
installer-select = اختر

# Title of the disk page.
installer-disks-title = أين يوضع { $name }

# SAFETY: needs a human review before it ships
# Sentence under the title of the disk page.
installer-disks-help = يُستبدل كل ما على القرص الذي تختاره، إلا إذا أفسحت مجالًا بجانب ما هو موجود.

# Shown on the disk page when no disk is big enough.
installer-disks-none = لم يُعثر على قرص كبير بما يكفي. يحتاج Trindle إلى 20 GB.

# SAFETY: needs a human review before it ships
# Name and words of the choice to erase a whole disk.
installer-disks-whole = استخدام القرص كله، مع استبدال كل ما عليه

# SAFETY: needs a human review before it ships
# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = الإبقاء على { $what } ومشاركة القرص

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } تبقى لما هو موجود الآن، و{ $mine } لـ Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }، { $size }

# Name of a partition with nothing on it.
installer-empty-space = مساحة فارغة

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } و{ $last }

# Under a disk's name: what is on it.
installer-disk-has = عليه { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = عليه { $list }، مشفّر

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = عليه نظام مشفّر

# Under a disk's name: there is nothing on it.
installer-disk-empty = فارغ

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = عليه ملفات، لكن لا نظام يعرفه Trindle

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = ملف تبادل بحجم { $gb } GB

# Name of the switch that makes a swap file.
installer-swap-switch = إنشاء ملف تبادل

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = الإقلاع الآمن (Secure Boot) مفعّل، لذلك لا يستطيع هذا الحاسوب الإسبات، لكن ملف التبادل ما زال مفيدًا حين تنفد الذاكرة. لا يُنشأ إلا إذا كان في القرص مساحة وفيرة فائضة.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = مساحة للإسبات، أي إبقاء كل شيء مفتوحًا مع إطفاء الطاقة، ومكان يُلجأ إليه حين تنفد الذاكرة. بدونه لا يستطيع الحاسوب الإسبات. لا يُنشأ إلا إذا كان في القرص مساحة وفيرة فائضة.

# SAFETY: needs a human review before it ships
# Title of the page about encrypting the disk.
installer-privacy-title = تشفير القرص

# SAFETY: needs a human review before it ships
# Sentence under the title of that page.
installer-privacy-help = لا يستطيع أحد قراءة القرص المشفّر إذا أخرجه من هذا الحاسوب.

# SAFETY: needs a human review before it ships
# Head of the card with the three choices.
installer-privacy-locked = كيف يُقفل القرص

# SAFETY: needs a human review before it ships
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = عدم تشفير القرص

# SAFETY: needs a human review before it ships
# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = تشفيره، وطلب كلمة المرور عند بدء تشغيل الحاسوب

# SAFETY: needs a human review before it ships
# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = تشفيره، والسماح لشريحة TPM في هذا الحاسوب بفتحه

# Head of the card about the TPM chip.
installer-privacy-about = عن الشريحة

# SAFETY: needs a human review before it ships
# In the card about the chip, on a computer that has one.
installer-privacy-chip = في هذا الحاسوب شريحة TPM. يطلب أول إقلاع بعد التثبيت كلمة المرور مرة واحدة، بينما تتعرّف الشريحة على هذا الحاسوب؛ وبعد ذلك تفتح القرص بنفسها ولا يُكتب شيء. احتفظ بكلمة المرور: فهي طريق الدخول إذا استُبدلت الشريحة أو نُقل القرص إلى حاسوب آخر.

# SAFETY: needs a human review before it ships
# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = ليس في هذا الحاسوب شريحة TPM يستطيع Trindle استخدامها، لذلك ستُطلب كلمة المرور عند كل إقلاع. وفي الحاسوب الذي يملك واحدة يمكن للشريحة فتح القرص بنفسها.

# SAFETY: needs a human review before it ships
# Title of the disk password page.
installer-disk-title = كلمة مرور القرص

# SAFETY: needs a human review before it ships
# Sentence under the title of the disk password page.
installer-disk-help = تُكتب قبل أي شيء آخر عند بدء تشغيل الحاسوب.

# SAFETY: needs a human review before it ships
# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = كلمة مرور القرص

# SAFETY: needs a human review before it ships
# Name of the round button that makes up a passphrase.
installer-disk-suggest = اقتراح عبارة مرور

# SAFETY: needs a human review before it ships
# Label and name of the second box of a password, where it is typed again.
installer-same-again = أعدها كما هي

# SAFETY: needs a human review before it ships
# Red line under the password boxes when the two do not match.
installer-mismatch = كلمتا المرور غير متطابقتين.

# SAFETY: needs a human review before it ships
# Head of the card that tells the person to write the password down.
installer-disk-write-down = دوّنها

# SAFETY: needs a human review before it ships
# Words in that card.
installer-disk-write-down-text = دوّن كلمة المرور هذه على ورقة واحتفظ بها في مكان آمن بعيدًا عن الحاسوب. بدونها لا يمكن فتح القرص، لا من قبلك ولا من قبل أي شخص آخر، أبدًا. ولا طريق للعودة. أربع كلمات عادية تصنع عبارة مرور يسهل كتابتها ويصعب تخمينها.

# Title of the account page.
installer-you-title = عنك

# Sentence under the title of the account page.
installer-you-help = هذا هو الحساب الذي تسجّل الدخول به.

# Head of the card with the name boxes.
installer-account = حسابك

# Label and name of the box for the person's name.
installer-your-name = اسمك

# Label and name of the box for the sign-in name.
installer-user-name = اسم تسجيل الدخول

# Label and name of the box for the computer's name.
installer-host-name = اسم هذا الحاسوب

# SAFETY: needs a human review before it ships
# Head of the card with the password boxes.
installer-your-password = كلمة مرورك

# SAFETY: needs a human review before it ships
# Label and name of the first password box.
installer-password = كلمة المرور

# Label of the strength meter and the name a screen reader gives it.
installer-strength = مدى صعوبة تخمينها

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = اسم تسجيل الدخول مطلوب.

# Red line under the account boxes: a name is too long.
installer-name-too-long = هذا الاسم طويل جدًا.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = ابدأ بحرف صغير.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = حروف صغيرة وأرقام و- و_ فقط.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = هذا الاسم يستخدمه النظام بالفعل.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = أعطِ هذا الحاسوب اسمًا.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = حروف وأرقام و- فقط.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = سيخمّن الحاسوب هذا فورًا.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [zero] سيخمّن الحاسوب هذا في أقل من دقيقة.
        [one] سيخمّن الحاسوب هذا في نحو دقيقة واحدة.
        [two] سيخمّن الحاسوب هذا في نحو دقيقتين.
        [few] سيخمّن الحاسوب هذا في نحو { $n } دقائق.
        [many] سيخمّن الحاسوب هذا في نحو { $n } دقيقة.
       *[other] سيخمّن الحاسوب هذا في نحو { $n } دقيقة.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [zero] سيخمّن الحاسوب هذا في أقل من ساعة.
        [one] سيخمّن الحاسوب هذا في نحو ساعة واحدة.
        [two] سيخمّن الحاسوب هذا في نحو ساعتين.
        [few] سيخمّن الحاسوب هذا في نحو { $n } ساعات.
        [many] سيخمّن الحاسوب هذا في نحو { $n } ساعة.
       *[other] سيخمّن الحاسوب هذا في نحو { $n } ساعة.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [zero] سيخمّن الحاسوب هذا في أقل من يوم.
        [one] سيخمّن الحاسوب هذا في نحو يوم واحد.
        [two] سيخمّن الحاسوب هذا في نحو يومين.
        [few] سيخمّن الحاسوب هذا في نحو { $n } أيام.
        [many] سيخمّن الحاسوب هذا في نحو { $n } يومًا.
       *[other] سيخمّن الحاسوب هذا في نحو { $n } يوم.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [zero] سيخمّن الحاسوب هذا في أقل من سنة.
        [one] سيحتاج الحاسوب إلى نحو سنة واحدة لتخمين هذا.
        [two] سيحتاج الحاسوب إلى نحو سنتين لتخمين هذا.
        [few] سيحتاج الحاسوب إلى نحو { $n } سنوات لتخمين هذا.
        [many] سيحتاج الحاسوب إلى نحو { $n } سنة لتخمين هذا.
       *[other] سيحتاج الحاسوب إلى نحو { $n } سنة لتخمين هذا.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = سيحتاج الحاسوب إلى وقت أطول من التاريخ المدوَّن لتخمين هذا.

# Title of the Ready page.
installer-ready-title = جاهز للتثبيت

# Head of the card that lists what will happen.
installer-ready-nothing-changed = لم يتغير شيء بعد

# Head of the card about security updates.
installer-ready-security = تحديثات الأمان

# In the card about security updates.
installer-ready-security-text = يثبّت { $name } إصلاحات الأمان من Debian بنفسه كل يوم. لا يمكن إيقاف هذا.

# SAFETY: needs a human review before it ships
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = سيُستبدل كل ما على { $disk }.

# SAFETY: needs a human review before it ships
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = يبقى كل ما على { $part }، لكن انسخه احتياطيًا أولًا: قد يؤدي تصغيره إلى فقدان الملفات إذا انقطعت الكهرباء.

# Sentence under the title when no disk has been chosen.
installer-ready-none = لم يُختر قرص بعد.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the whole disk will be used.
installer-ready-whole = كامل { $disk }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-whole-text = يُستبدل كل ما عليه بـ { $name }.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = بجانب { $part }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-beside-text = يُصغَّر { $part } إلى { $size }، ويوضع { $name } في المساحة المحرَّرة.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = لا قرص

# Words of that row.
installer-ready-no-disk-text = لم يُختر قرص.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = غير مشفّر

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-plain-text = يستطيع أي شخص يُخرج القرص أن يقرأه.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = مشفّر

# SAFETY: needs a human review before it ships
# Words of that row when a password is typed at each start.
installer-ready-locked-password = تكتب كلمة مروره في كل مرة يبدأ فيها الحاسوب.

# SAFETY: needs a human review before it ships
# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = كلمة المرور مرة واحدة، ثم تفتحه شريحة TPM في هذا الحاسوب.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = لوحة المفاتيح { $keyboard }، التوقيت { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = جارٍ تثبيت { $name }

# Head of the card that holds the short tour.
installer-tour = جولة قصيرة

# Title of the last page.
installer-done-title = تم تثبيت { $name }

# Sentence under the title of the last page.
installer-done-help = أخرج عصا USB ثم أعد التشغيل.

# SAFETY: needs a human review before it ships
# Head of the card that repeats the warning about the disk password.
installer-done-keep = احتفظ بكلمة مرور القرص بأمان

# SAFETY: needs a human review before it ships
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = القرص مشفّر. يطلب أول إقلاع كلمة مرور القرص مرة واحدة، بينما تتعرّف الشريحة على هذا الحاسوب؛ وبعد ذلك يُفتح من تلقاء نفسه. دوّن كلمة المرور هذه على ورقة واحتفظ بها في مكان آمن بعيدًا عن هذا الحاسوب. بدونها لا يمكن فتح القرص، لا من قبلك ولا من قبل أي شخص آخر، أبدًا.

# SAFETY: needs a human review before it ships
# In that card when the password is typed at each start.
installer-keep-password = القرص مشفّر. دوّن كلمة مرور القرص على ورقة الآن، إن لم تكن قد فعلت، واحتفظ بها في مكان آمن بعيدًا عن هذا الحاسوب. بدونها لا يمكن فتح القرص، لا من قبلك ولا من قبل أي شخص آخر، أبدًا.

# Title of the first slide.
installer-slide-ring = كل شيء في دائرة واحدة

# Words of the first slide.
installer-slide-ring-text = حرّك المؤشر إلى حافة الشاشة وانقر نقرًا مزدوجًا، فتُفتح الحلقة: برامجك حول الخارج، والوقت والمفاتيح في الوسط.

# Title of the second slide.
installer-slide-snapshots = أعد ما أفسده تحديث

# Words of the second slide.
installer-slide-snapshots-text = تُؤخذ لقطة للنظام قبل كل تغيير في البرمجيات وبعده. تعيد الإعدادات النظام كله إلى أي منها، وتستطيع قائمة الإقلاع فعل ذلك حين لا يبدأ الحاسوب.

# Title of the third slide.
installer-slide-apps = تطبيقات لا تعيق طريقك

# Words of the third slide.
installer-slide-apps-text = تطبيقات Flathub وتطبيقات Debian نفسها وAppImage ومصادر البرمجيات في مكان واحد، مع نسخ احتياطية يمكنك حملها إلى حاسوب آخر.

# Title of the fourth slide.
installer-slide-yours = لك، بسرعة

# Words of the fourth slide.
installer-slide-yours-text = فاتح أو داكن، وأصواتك الخاصة، وحافظة تتذكر ما نسخته.

# Title of the fifth slide.
installer-slide-small = صغير وهادئ

# Words of the fifth slide.
installer-slide-small-text = يستهلك سطح المكتب القليل من الذاكرة ويبدأ في بضع ثوانٍ، فيبدو الحاسوب المحمول القديم جديدًا من جديد.

# Title of the tour slide about the ring.
installer-tour-ring = الحلقة هي سطح المكتب كله

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = ضع المؤشر في هذه الزاوية فتُفتح الحلقة

# Note on the ring picture, at the time.
installer-tour-ring-time = الوقت والتاريخ، في الوسط

# Note on the ring picture, at the programs.
installer-tour-ring-programs = برامجك حول الخارج

# Note on the ring picture, at the switches.
installer-tour-ring-switches = الصوت والشاشة وكل تطبيقاتك وإبقاء الجهاز مستيقظًا وإيقاف التشغيل

# Title of the tour slide about Settings.
installer-tour-settings = الإعدادات، بكلمات بسيطة

# Note on the Settings picture, at the search box.
installer-tour-settings-search = اكتب هنا لتجد إعدادًا دون عناء البحث

# Note on the Settings picture, at Network.
installer-tour-settings-network = في الشبكة تنضم إلى شبكة Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = فاتح أو داكن، متى شئت

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = عشرون لونًا، أو اكتب لونك

# Title of the tour slide about Apps.
installer-tour-apps = التطبيقات، لكل شيء آخر

# Note on the Apps picture, at the search box.
installer-tour-apps-search = ابحث عن برنامج باسمه أو بما يفعله

# Note on the Apps picture, at the list.
installer-tour-apps-many = آلاف البرامج، مجانية، من Flathub وDebian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = تحديثات لكل شيء، بما في ذلك Trindle نفسه

# Title of the tour slide about voice control.
installer-tour-voice = يمكنك التحدث إليه بدلًا من ذلك

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = التحكم الصوتي هنا: الإعدادات، ثم الحلقة

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = اضغط إضافة الصوت مرة واحدة. حجمه نحو 150 MB ويستغرق بضع دقائق

# Note on the voice picture, about what to say.
installer-tour-voice-say = ثم اضغط على وسط الحلقة وقل اسم تطبيق، أو «أعلى»، أو «إعدادات Wi-Fi»

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = ما تقوله يُفهم على هذا الحاسوب ولا يذهب إلى مكان آخر

# Title of the tour slide about snapshots.
installer-tour-snapshots = طريق للعودة، قبل أن يتغير أي شيء

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = يأخذ الحاسوب لقطة قبل كل تحديث

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = إذا تعطل شيء، ارجع إلى ما كان عليه من قائمة الإقلاع

# Name of the picture of the ring, and the head of its card.
installer-live-caption = الحلقة، حيث يعيش كل شيء

# Sentence under the title of the welcome from a stick.
installer-live-help = أنت تعمل من العصا، لذلك لم يتغير شيء على هذا الحاسوب.

# Head of the card with the three first things to do.
installer-live-begin = للبدء

# Name of the first thing to do.
installer-live-open = افتح الحلقة

# Line under it.
installer-live-open-line = انقر نقرًا مزدوجًا عند حافة الشاشة، أو اضغط Super.

# Name of the second thing to do.
installer-live-speak = تحدّث إليها

# Line under it when voice control is there.
installer-live-speak-line = اضغط على وسط الحلقة وقل ما تريد.

# Line under it when voice control has to be added.
installer-live-speak-add = أضف الصوت من الإعدادات، الحلقة، ثم اضغط الوسط وتحدّث.

# Name of the third thing to do.
installer-live-tab = كل شيء في الحلقة

# Line under it.
installer-live-tab-line = ما هو مفتوح وإشعاراتك والحافظة، كلها هناك.

# The choice of the dark look.
installer-live-dark = داكن

# The choice of the light look.
installer-live-light = فاتح

# Name the screen reader gives to the choice between dark and light.
installer-live-look = فاتح أو داكن، لسطح المكتب كله

# Name of the round button that closes this window and opens the ring.
installer-live-around = تجوّل أولًا

# SAFETY: needs a human review before it ships
# Name of the main button that opens the installer.
installer-live-install = تثبيت { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = جارٍ إفساح المجال

# Foot of the install page while the disk is prepared.
installer-step-prepare = جارٍ تحضير القرص

# Foot of the install page while a place is made beside another system.
installer-step-place = جارٍ تهيئة مكان لـ Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = جارٍ فحص ما هو موجود على القرص

# Foot of the install page while another system is made smaller.
installer-step-smaller = جارٍ تصغير النظام الآخر

# Foot of the install page while the disk is encrypted.
installer-step-lock = جارٍ قفل القرص

# Foot of the install page while the TPM chip is set up.
installer-step-chip = جارٍ إعداد شريحة الحاسوب

# Foot of the install page while the system is copied.
installer-step-copy = جارٍ نسخ Trindle إلى القرص

# Foot of the install page while settings are written.
installer-step-settings = جارٍ كتابة الإعدادات

# Foot of the install page while the person's account is made.
installer-step-account = جارٍ إنشاء حسابك

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = جارٍ إزالة أجزاء العصا

# Foot of the install page while the start-up files are made.
installer-step-startup = جارٍ إنشاء ملفات بدء التشغيل

# Foot of the install page while the boot menu is set up.
installer-step-boot = جارٍ إعداد قائمة الإقلاع

# Foot of the install page while snapshots are set up.
installer-step-snapshots = جارٍ إعداد اللقطات

# Foot of the install page at the very end.
installer-step-finishing = جارٍ الإنهاء

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = تعذّر بدء { $program }: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = لم ينتهِ { $program }. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = تعذّرت كتابة { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } ليس قرصًا.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } ليس قرصًا في هذا الحاسوب.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = لم يُختر قرص.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = اسم تسجيل الدخول هذا لا يستطيع Trindle استخدامه.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = اسم الحاسوب هذا لا يستطيع Trindle استخدامه.

# Red line at the foot: the language cannot be used.
installer-bad-language = هذه اللغة لا يستطيع Trindle استخدامها.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = لوحة المفاتيح هذه لا يستطيع Trindle استخدامها.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = المنطقة الزمنية هذه لا يستطيع Trindle استخدامها.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = يحوي هذا الاسم شيئًا لا يستطيع Trindle كتابته.

# SAFETY: needs a human review before it ships
# Red line at the foot: a password has a line break in it.
installer-bad-password = لا يمكن أن تحوي كلمة المرور فاصل سطر.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = يجب أن يعمل النصف الثاني من المثبّت بصلاحيات root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = ليس في هذا القرص قسم إقلاع للمشاركة، لذلك لا يستطيع Trindle أن يوضع بجانب ما هو موجود هنا.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = تعذّر العثور على الجزء الجديد من القرص.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = تعذّرت قراءة حجم { $part }.

# SAFETY: needs a human review before it ships
# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = لا يستطيع Trindle تصغير قسم من نوع { $fs }.

# SAFETY: needs a human review before it ships
# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = لا يستطيع Trindle تصغير قسم غير معروف.

# SAFETY: needs a human review before it ships
# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = كلمة المرور مطلوبة لتشفير القرص.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = تعذّر نسخ النظام: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = تعذّر نسخ النظام. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = لا مساحة لملف التبادل.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = تعذّرت قراءة مكان ملف التبادل على القرص.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = تعذّر التعرف على الأقسام الجديدة.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = تعذّر ضبط المنطقة الزمنية: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = تعذّر تثبيت قائمة الإقلاع على هذا الحاسوب.

# Name of that button after one press, when a second press goes ahead.
installer-sure = متأكد؟



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = ما هو مفتوح، والحافظة، وزر يمسح الشاشة

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } هو الذاكرة المحمولة التي يعمل منها Trindle.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } قيد الاستخدام. أغلق ما يستخدمه ثم حاول مرة أخرى.

# Said beside how far the install has got, once it has started.
installer-started = بدأ التثبيت

# The button that ends an install under way.
installer-halt = إيقاف

# Beside Stop after its first press.
installer-halt-sure = إيقاف التثبيت؟ لقد ذهب المحتوى القديم للقرص بالفعل.

# While an install is being stopped.
installer-halting = جارٍ الإيقاف

# When Stop could not let go of the disk.
installer-halt-stuck = تعذّر تحرير القرص. أعد تشغيل الحاسوب لتحاول مرة أخرى.

# The copy is done and the person is still answering.
installer-step-waiting = تم النسخ. بانتظار إجاباتك.



installer-already-running = برنامج تثبيت آخر يعمل بالفعل على هذا الحاسوب.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = يحتاج Trindle إلى 1 GB من الذاكرة على الأقل، وذاكرة هذا الحاسوب أقل من ذلك.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = لا يمكن تشفير القرص إلا على حاسوب يبدأ عبر UEFI، وهذا الحاسوب لا يبدأ بهذه الطريقة.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = نظام Windows على { $part } كان في السبات أو تُرك في البدء السريع، لذلك قد يؤدي تصغيره إلى فقدان ملفاته. شغّل Windows، واختر إعادة التشغيل، ثم حاول مرة أخرى.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = تم تثبيت Trindle، لكن تعذّر إعداد اللقطات: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = تم تثبيت Trindle، لكن تعذّر جعل قائمة الإقلاع تعرض اللقطات: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = لم يكتمل إعداد كل شيء

