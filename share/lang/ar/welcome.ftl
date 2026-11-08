
# machine translated (Arabic), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = مرحبًا بك في Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = افتح الحلقة

# Title of the third page, where the first apps are picked.
welcome-page-apps = اختر تطبيقاتك

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = المزيد للاختيار

# Title of the fifth page, about plugins.
welcome-page-plugins = الإضافات تضيف المزيد

# Title of the sixth page, about crash reports.
welcome-page-reports = ساعد في إصلاح Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = النقاط

# Title of the last page, with a few things worth knowing.
welcome-page-know = من الجيد أن تعرف

# Sentence under the title of the first page.
welcome-sentence-hello = كل ما تحتاجه موجود في دائرة واحدة، هي الحلقة.

# Sentence under the title of the second page.
welcome-sentence-ring = انقر نقرًا مزدوجًا عند حافة الشاشة تمامًا، أو اضغط مفتاح Super.

# Sentence under the title of the third page.
welcome-sentence-apps = لا يكاد يوجد شيء مثبّت بعد، فاختر التطبيقات التي تريدها.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = اختر المزيد، ثم ثبّت كل ما اخترته بنقرة واحدة.

# SAFETY: needs a human review before it ships
# Sentence under the title of the fifth page.
welcome-sentence-plugins = الإضافات ملحقات صغيرة من التطبيقات، وكل منها يفعل فقط ما تسمح به.

# Sentence under the title of the sixth page.
welcome-sentence-reports = أرسل تقريرًا عند حدوث خطأ، إن اخترت ذلك.

# Sentence under the title of the dots page.
welcome-sentence-dots = النقطة تعني أن برنامجًا يستخدم شيئًا يخصك.

# Sentence under the title of the last page.
welcome-sentence-know = بضعة أشياء تميّز Trindle.

# Top right of every page: where the person is in the tour.
welcome-step = الخطوة { $n } من { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. الخطوة { $n } من { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = تمت رؤية الترحيب. وهو في كل التطبيقات باسم الترحيب؛ احذف هذا الملف لرؤيته مرة أخرى عند تسجيل الدخول التالي.

# Name of the round button at the foot that closes the tour.
welcome-close = إغلاق

# Name of the round button at the foot that goes to the page before.
welcome-back = رجوع

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = التالي

# SAFETY: needs a human review before it ships
# Name of the main button at the foot of the apps page once some are picked.
welcome-install = تثبيت التطبيقات المختارة

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = ابدأ استخدام Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = اتصل بالإنترنت لتثبيت التطبيقات.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = حاول مرة أخرى

# Foot while apps are going in: their names.
welcome-installing = جارٍ تثبيت { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = جارٍ تثبيت { $names }، { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [zero] { $names } في الحلقة الآن.
        [one] { $names } في الحلقة الآن.
        [two] { $names } في الحلقة الآن.
        [few] { $names } في الحلقة الآن.
        [many] { $names } في الحلقة الآن.
       *[other] { $names } في الحلقة الآن.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = تعذّر تثبيت التطبيقات. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = رفض Flatpak ذلك.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = المختار: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first } و{ $second } و{ $more } آخرين

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = الحلقة

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = ما فيها

# Name of the first thing the ring holds.
welcome-held-programs = برامجك

# Line under it.
welcome-held-programs-line = تقع حول الخارج.

# Name of the second thing the ring holds.
welcome-held-time = الوقت والتاريخ

# Line under it.
welcome-held-time-line = يقعان في الوسط.

# Name of the third thing the ring holds.
welcome-held-switches = المفاتيح

# Line under it.
welcome-held-switches-line = الصوت والشاشة وإبقاء الجهاز مستيقظًا والطاقة.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = بالمؤشر

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = بلوحة المفاتيح

# Note under the picture of the Super key.
welcome-ring-super-note = غالبًا ما يحمل مفتاح Super شعار Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = متصفح الويب

# Head of a card of apps: email programs.
welcome-kind-email = البريد الإلكتروني

# Head of a card of apps: chat programs.
welcome-kind-chat = المحادثة

# Head of a card of apps: office programs.
welcome-kind-office = المكتب

# Head of a card of apps: music and video players.
welcome-kind-media = الموسيقى والفيديو

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = الصور والرسم

# Head of a card of apps: notes and writing.
welcome-kind-notes = الملاحظات وكلمات المرور

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF والألعاب

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = لا شيء من هذه على Flathub الآن.

# Beside an app that has been installed.
welcome-app-installed = مثبّت

# Beside an app that is going in now.
welcome-app-installing = جارٍ التثبيت

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }، مختار

# Name of the round button on an app's row that picks it.
welcome-app-pick = اختر { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = في الحلقة

# Head of the card that lists the plugins on offer.
welcome-plugins-card = الإضافات

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = في التطبيقات، تحت الإضافات.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = فتح الإضافات في التطبيقات

# Said in the plugins card while there are none to show.
welcome-plugins-none = الإضافات المتاحة مدرجة في التطبيقات.

# Head of the card on the crash reports page.
welcome-reports-card = تقارير الأعطال

# SAFETY: needs a human review before it ships
# The one question about crash reports.
welcome-reports-question = هل تريد المساعدة في إصلاح Trindle بإرسال تقرير عند حدوث خطأ؟ سترى التقرير دائمًا أولًا.

# SAFETY: needs a human review before it ships
# Note under the question about crash reports.
welcome-reports-note = لا يُرسل شيء دون ضغطتك. سترى التقرير كاملًا أولًا، وهو لا يحوي ملفات أو كلمات مرور أو أسماء شبكات Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = يمكنك تغيير هذا في أي وقت من الإعدادات، تقارير الأعطال.

# SAFETY: needs a human review before it ships
# The answer that turns crash reports off.
welcome-reports-no = لا

# SAFETY: needs a human review before it ships
# The answer that turns crash reports on.
welcome-reports-yes = نعم، اسألني

# SAFETY: needs a human review before it ships
# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = إرسال تقرير عند حدوث خطأ

# Head of a card on the last page.
welcome-know-boxes = التطبيقات تعيش في صناديقها الخاصة

# SAFETY: needs a human review before it ships
# Line in that card.
welcome-know-boxes-line = يسألك التطبيق قبل أن يصل إلى ملفاتك أو الكاميرا أو الميكروفون.

# Head of a card on the last page.
welcome-know-updates = تحديثات الأمان تُثبَّت من تلقائها

# Line in that card.
welcome-know-updates-line = كل يوم، دون سؤال. أما التحديثات الأخرى فتنتظرك.

# Head of a card on the last page.
welcome-know-ring = الإعدادات والتطبيقات في الحلقة

# Line in that card.
welcome-know-ring-line = في وسطها، فوق الوقت.

# Head of a card on the last page.
welcome-know-help = المساعدة في الإعدادات

# Line in that card.
welcome-know-help-line = هذه الجولة في كل التطبيقات باسم الترحيب، متى أردتها مرة أخرى.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = شاشة صغيرة لا شيء عليها سوى الحلقة في الوسط: الوقت في مركزها والتطبيقات المفضلة حولها.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = الحلقة مفتوحة على شاشة صغيرة، وأيقونات بضع إضافات في صف في وسطها، تحت الوقت، حيث تقع أيقونة الإضافة بعد تثبيتها.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = رسمان صغيران لأماكن ظهور النقاط: على علامة تبويب نافذة في الحلقة، وفي أعلى منتصف الشاشة.

welcome-dots-where-tab = على تبويب نافذة

welcome-dots-where-corner = أعلى منتصف الشاشة

welcome-picture-ring = شاشة صغيرة. يتحرك المؤشر إلى الحافة العلوية تمامًا وينقر نقرًا مزدوجًا، فتفتح الحلقة في الوسط.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = الضغط على مفتاح Super، وهو يفتح الحلقة.

# Title of the first page, which asks how much the person knows.
welcome-page-level = كم تعرف؟

# Sentence under that title.
welcome-sentence-level = اختر الأقرب، وستتخطى الجولة ما تعرفه.

# Name of the answer for someone who has not used Linux.
welcome-level-new = جديد على Linux

# Line in that answer's card.
welcome-level-new-line = كل صفحة، بما في ذلك ما يفعله أي سطح مكتب Linux.

# Name of the answer for someone who has used Linux.
welcome-level-linux = استخدمت Linux من قبل

# Line in that answer's card.
welcome-level-linux-line = ما يخص Trindle وحده فقط. تُتخطى أساسيات Linux.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = استخدمت Trindle من قبل

# Line in that answer's card.
welcome-level-trindle-line = تُغلق الجولة الآن ولا تظهر مرة أخرى.

# Title of the page about files and folders.
welcome-page-files = ملفاتك

# Sentence under that title.
welcome-sentence-files = كل ما يخصك موجود في مجلدك الرئيسي.

# Title of the page about getting software.
welcome-page-software = الحصول على البرامج

# Sentence under that title.
welcome-sentence-software = تأتي التطبيقات من Apps، لا من ملفات إعداد تنزلها.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = كلمة مرور المسؤول

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = التغييرات على الحاسوب كله تطلب كلمة مرور أولاً.

# Title of the page about snapshots.
welcome-page-snapshots = اللقطات

# Sentence under that title.
welcome-sentence-snapshots = يحتفظ Trindle بنسخ من النظام، فيمكن التراجع عن أي تحديث.

# Title of the page about the person's own words.
welcome-page-words = كلماتك الخاصة

# Sentence under that title.
welcome-sentence-words = غيّر أي كلمة يعرضها Trindle، أو أضف لغتك الخاصة.

# Head of a card on the files page.
welcome-files-home = مجلد رئيسي

# Line in that card.
welcome-files-home-line = المستندات والتنزيلات والصور بداخله. تفتح Folders عنده.

# Head of a card on the files page.
welcome-files-drives = بلا حروف أقراص

# Line in that card.
welcome-files-drives-line = تظهر ذاكرة USB أو قرص آخر في Folders عند توصيلها.

# Head of a card on the files page.
welcome-files-hidden = أسماء مخفية

# Line in that card.
welcome-files-hidden-line = الاسم الذي يبدأ بنقطة، مثل .config، مخفي. يحمل إعدادات تطبيق.

# Head of a card on the files page.
welcome-files-bin = سلة المحذوفات

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = الملف المحذوف ينتظر في السلة حتى تفرّغها.

# Head of a card on the software page.
welcome-software-apps = مكان واحد للتطبيقات

# Line in that card.
welcome-software-apps-line = افتح Apps من الحلقة للعثور على البرامج وتثبيتها وإزالتها.

# Head of a card on the software page.
welcome-software-exe = بلا ملفات إعداد

# Line in that card.
welcome-software-exe-line = ملف إعداد Windows المنتهي بـ .exe لا يعمل هنا. ابحث عن التطبيق في Apps.

# Head of a card on the software page.
welcome-software-updates = التحديثات في مكان واحد

# Line in that card.
welcome-software-updates-line = تُبقي Apps وSettings كل شيء محدّثاً، فلا يحتاج أي تطبيق إلى محدّث خاص به.

# Head of a card on the software page.
welcome-software-remove = الإزالة نظيفة

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = إزالة تطبيق تزيله كله ولا تترك شيئاً خلفها.

# Head of a card on the administrator password page.
welcome-admin-why = لماذا يسأل

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = التثبيت للجميع أو تغيير قرص يؤثر في كل المستخدمين، لذلك يسأل الحاسوب أولاً.

# Head of a card on the administrator password page.
welcome-admin-who = كلمة مرور من

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = كلمة مرور مسؤول، وهي عادة كلمة مرور الحساب الذي تسجل به الدخول.

# Head of a card on the administrator password page.
welcome-admin-not = متى لا يسأل

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = التطبيقات التي تثبتها لنفسك وملفاتك الخاصة لا تحتاج إليها أبداً.

# Head of a card on the snapshots page.
welcome-snap-taken = تؤخذ لك

# Line in that card.
welcome-snap-taken-line = في نظام Btrfs تؤخذ لقطة قبل كل تغيير في البرامج وبعده.

# Head of a card on the snapshots page.
welcome-snap-list = مدرجة في Settings

# Line in that card.
welcome-snap-list-line = تعرضها Settings، ثم Snapshots. وتأخذ الكاميرا لقطة الآن.

# Head of a card on the snapshots page.
welcome-snap-boot = تُستخدم إن لم يبدأ التشغيل

# Line in that card.
welcome-snap-boot-line = اختر Trindle snapshots في قائمة الإقلاع، ثم اختر لقطة.

# Head of a card on the your own words page.
welcome-words-change = غيّر كلمة

# Line in that card.
welcome-words-change-line = ابحث عن أي كلمة يعرضها Trindle واكتب كلمتك. يمكن أن تحل "Open it, brew" محل "Click here to open the file".

# Head of a card on the your own words page.
welcome-words-share = شارك ورقة

# Line in that card.
welcome-words-share-line = صدّر كلماتك كملف، أو استورد ملفاً أرسله صديق. يُفحص قبل استخدامه.

# Head of a card on the your own words page.
welcome-words-all = كل البرامج دفعة واحدة

# Line in that card.
welcome-words-all-line = يقرأ Trindle كله كلماته من مكان واحد، فيظهر تغيير واحد في كل مكان.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = إنها في Settings، ثم Region and language. تشرحها Help خطوة بخطوة.

# Name of the round button that opens that Settings page.
welcome-words-open = افتح Region and language
