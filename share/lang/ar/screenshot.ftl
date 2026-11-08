# machine translated (Arabic), not yet reviewed by a person.

# Name of the screenshot overlay that covers the screen (what a screen reader calls it).
screenshot-name = لقطة الشاشة

# Note on the overlay before any part is chosen.
screenshot-drag = اسحب فوق الجزء الذي تريد الاحتفاظ به

# Name of the round button that keeps the whole screen.
screenshot-whole = الشاشة كلها

# Name of the round button that closes the overlay or a pinned picture.
screenshot-close = إغلاق

# Name of the round button that gives the screenshot up.
# SAFETY: needs a human review before it ships
screenshot-cancel = إلغاء

# Name of the round button that takes back the last mark.
screenshot-undo = تراجع

# Name of the round button that puts back the last mark taken away.
screenshot-redo = إعادة

# Name of the round button that copies the picture to the clipboard, and of the copy button of a colour.
screenshot-copy = نسخ

# Name of a colour's button after it has been copied.
screenshot-copied = تم النسخ

# Word on the button that saves the picture in the Screenshots folder.
screenshot-save = حفظ

# Word on the button that asks where to save the picture.
screenshot-save-as = حفظ باسم

# Name of the round button that keeps the picture on the screen as a small window.
screenshot-pin = ثبّت على الشاشة

# Name of the drawing tool that draws freehand.
screenshot-tool-pen = قلم

# Name of the drawing tool that draws a straight line.
screenshot-tool-line = خط

# Name of the drawing tool that draws an arrow.
screenshot-tool-arrow = سهم

# Name of the drawing tool that draws a box.
screenshot-tool-box = مربع

# Name of the drawing tool that draws a circle.
screenshot-tool-circle = دائرة

# Name of the drawing tool that highlights.
screenshot-tool-highlighter = قلم تمييز

# Name of the drawing tool that writes words.
screenshot-tool-words = كلمات

# Name of the drawing tool that puts numbered spots.
screenshot-tool-numbers = أرقام

# Name of the drawing tool that blurs a patch.
screenshot-tool-blur = تمويه

# Name of the round button that draws in red.
screenshot-colour-red = أحمر

# Name of the round button that draws in blue.
screenshot-colour-blue = أزرق

# Name of the round button that draws in green.
screenshot-colour-green = أخضر

# Name of the round button that draws in orange.
screenshot-colour-orange = برتقالي

# Name of the round button that draws in purple.
screenshot-colour-purple = بنفسجي

# Name of the round button that draws in white.
screenshot-colour-white = أبيض

# Name of the round button that draws in black.
screenshot-colour-black = أسود

# Name of the round button that draws thin lines.
screenshot-thickness-thin = رفيع

# Name of the round button that draws lines of middle thickness.
screenshot-thickness-medium = متوسط

# Name of the round button that draws thick lines.
screenshot-thickness-thick = سميك

# Name the screen reader gives to a screenshot pinned on the screen.
screenshot-pinned = لقطة شاشة مثبتة

# Title of the window that asks where to save the screenshot.
screenshot-save-title = احفظ لقطة الشاشة

# Title of the notice that says the screenshot could not be saved.
screenshot-not-saved = لم تُحفظ لقطة الشاشة

# Title of the notice that says the screenshot could not be copied.
screenshot-not-copied = لم تُنسخ لقطة الشاشة

# Words of that notice.
screenshot-clipboard-refused = تعذّر على الحافظة قبولها.

# Words of the notice that the screenshot could not be saved: the picture was not made.
screenshot-no-picture = تعذّر إنشاء الصورة

# Words of the notice that the screenshot could not be saved: no bus to ask on.
screenshot-no-bus = لا يوجد ناقل جلسة للسؤال عن المكان

# Words of the notice that the screenshot could not be saved: the file chooser could not be asked.
screenshot-no-chooser = تعذّر سؤال أداة اختيار الملفات

# Words of the notice that the screenshot could not be saved: every file name was taken.
screenshot-no-free-name = لا يوجد اسم متاح

# Words of the notice that the screenshot could not be saved: the path and why it failed.
screenshot-write-failed = { $path }: { $why }

# Name of the eye dropper's window, and its title.
screenshot-eye-title = أداة انتقاء اللون

# Words that float beside the i of the eye dropper's window.
screenshot-eye-help = اضغط على أداة انتقاء اللون، ثم انقر في أي مكان على الشاشة لأخذ اللون تحت المؤشر. زر Escape يتراجع. يُنسخ كل لون فورًا برمز سداسي عشري ويُحفظ في القائمة أدناه، الأحدث أولًا، لتنسخه مرة أخرى لاحقًا.

# Name of the i button beside the title.
screenshot-eye-about = عن أداة انتقاء اللون

# Name of the main button that takes a colour.
screenshot-eye-take = خذ لونًا

# Sentence under the title while no colour has been taken.
screenshot-eye-none = خذ لونًا من أي مكان على الشاشة.

# Sentence under the title once colours have been taken.
screenshot-eye-taken =
    { $n ->
        [zero] لم يُؤخذ أي لون.
        [one] أُخذ لون واحد: { $newest }.
        [two] أُخذ لونان؛ الأحدث { $newest }.
        [few] أُخذت { $n } ألوان؛ الأحدث { $newest }.
        [many] أُخذ { $n } لونًا؛ الأحدث { $newest }.
       *[other] أُخذ { $n } لون؛ الأحدث { $newest }.
    }

# Shown in the big card while no colour has been taken.
screenshot-eye-latest-empty = اللون الذي تأخذه يظهر هنا كبيرًا، جاهزًا للنسخ.

# Name of the card that holds the latest colour.
screenshot-eye-latest = آخر لون

# Name of the round button that clears the list of colours.
# SAFETY: needs a human review before it ships
screenshot-eye-clear = امسح القائمة

# Head of the card with the list of colours taken.
screenshot-eye-list = الألوان المأخوذة

# Shown in the list card while no colour has been taken.
screenshot-eye-list-empty = الألوان المأخوذة بأداة الانتقاء تُحفظ هنا، الأحدث أولًا.

# Name of the round button that takes a colour out of the list.
# SAFETY: needs a human review before it ships
screenshot-eye-remove = إزالة

# What the saved list of colours is called when it is found damaged and set aside.
screenshot-eye-damaged = ألوان أداة الانتقاء

# Title of the window that asks what an app may share.
# SAFETY: needs a human review before it ships
screenshot-pick-title = مشاركة شاشتك مع { $app }؟

# Sentence under the title when several things can be picked.
# SAFETY: needs a human review before it ships
screenshot-pick-many = اختر واحدًا أو أكثر. لا يُشارَك إلا ما تختاره، حتى يتوقف التطبيق أو يُغلق.

# Sentence under the title when one thing can be picked.
# SAFETY: needs a human review before it ships
screenshot-pick-one = اختر واحدًا. لا يُشارَك إلا ما تختاره، حتى يتوقف التطبيق أو يُغلق.

# Heading over the screens that can be shared.
screenshot-pick-screens = الشاشات

# Heading over the windows that can be shared.
screenshot-pick-windows = النوافذ

# Shown when there is nothing to share.
screenshot-pick-nothing = لا يوجد ما يمكن مشاركته.

# Name of the choice that shares every screen at once.
screenshot-pick-desktop = سطح المكتب كله

# Words under that choice.
screenshot-pick-desktop-detail = كل الشاشات كما هي

# Words under a screen's choice: its size in pixels.
screenshot-pick-size = { $width } في { $height }

# What the screen reader says for a choice that is picked.
screenshot-pick-chosen = { $name }، مختار

# Name of the round button that refuses to share.
# SAFETY: needs a human review before it ships
screenshot-pick-refuse = لا تشارك

# Name of the main button that shares what is picked.
# SAFETY: needs a human review before it ships
screenshot-pick-share = شارك

# Name of the button that says no in a question from a program, and of its answer to the program.
# SAFETY: needs a human review before it ships
screenshot-portal-deny = ارفض

# Name of the main button that says yes in a question from a program.
# SAFETY: needs a human review before it ships
screenshot-portal-allow = اسمح

# Name of the round button that gives up a window from a program.
# SAFETY: needs a human review before it ships
screenshot-portal-cancel = إلغاء

# Name of the main button that opens, and the title of a window that opens a file.
screenshot-portal-open = فتح

# Title of the window that saves a file, and the word of its main button.
screenshot-portal-save = حفظ

# Name of the box where a name is typed.
screenshot-portal-name = الاسم

# Heading when an unknown program asks who the person is.
# SAFETY: needs a human review before it ships
screenshot-account-asks-unknown = برنامج يسأل من أنت

# Heading when a program asks who the person is.
# SAFETY: needs a human review before it ships
screenshot-account-asks = { $who } يسأل من أنت

# Name the screen reader gives to the person's picture.
screenshot-account-picture = صورتك

# Words under the person's name.
screenshot-account-signed-in = مسجّل الدخول باسم { $id }

# Name of the search box in the window that asks which app should open a file.
screenshot-apps-search = ابحث عن تطبيقات

# Shown when no app can open the file.
screenshot-apps-none = لا يوجد تطبيق هنا يستطيع فتحه

# Name of the switch that remembers the choice.
screenshot-apps-always = استخدم هذا التطبيق دائمًا لهذا النوع من الملفات

# Title of the window that asks which app should open a file.
screenshot-apps-open-with = افتح { $name } باستخدام

# Title of that window when the file's name is not known.
screenshot-apps-choose = اختر تطبيقًا

# Name of the screen sharing that other programs see, and the name Trindle gives itself to PipeWire.
screenshot-cast-name = مشاركة شاشة Trindle

# What other programs show for the shared screen.
screenshot-cast-node = شاشة مشاركة مع تطبيق

# Name used for a program that cannot be named, in the window that asks what to share.
screenshot-cast-an-app = تطبيق

# Title of the window that asks what to share.
# SAFETY: needs a human review before it ships
screenshot-cast-window = شارك الشاشة

# Title of the window that saves several files into a folder.
screenshot-files-save-many = احفظ في مجلد

# Title of the window that chooses a folder.
screenshot-files-folder = اختر مجلدًا

# What a program holding the computer awake is called when it gave no name.
screenshot-inhibit-who = برنامج

# Heading when a program wants to add an app to the person's apps and cannot be named.
# SAFETY: needs a human review before it ships
screenshot-launcher-ask = إضافة هذا إلى تطبيقاتك؟

# Heading when a named program wants to add an app to the person's apps.
# SAFETY: needs a human review before it ships
screenshot-launcher-asks = يريد { $who } إضافة هذا إلى تطبيقاتك

# Name the screen reader gives to the app's icon.
screenshot-launcher-icon = أيقونته

# Words under the name, when the app opens a web address.
screenshot-launcher-opens = يفتح { $target }

# Title of the window that asks to add an app.
screenshot-launcher-title = أضف تطبيقًا

# Name of the main button that adds the app.
# SAFETY: needs a human review before it ships
screenshot-launcher-add = أضف

# Heading when a program that cannot be named offers a wallpaper.
# SAFETY: needs a human review before it ships
screenshot-wallpaper-ask = تعيين هذه الصورة خلفية لشاشتك؟

# Heading when a program offers a wallpaper.
# SAFETY: needs a human review before it ships
screenshot-wallpaper-asks = يقترح { $who } خلفية جديدة

# Name the screen reader gives to the picture offered.
screenshot-wallpaper-picture = الخلفية الجديدة

# Name of the main button that sets the wallpaper.
# SAFETY: needs a human review before it ships
screenshot-wallpaper-set = عيّنها خلفية

# Red line under the printer box when no printer is set up.
screenshot-print-no-printer = لا توجد طابعة مُعدّة. أضف واحدة في الإعدادات، الطابعات.

# Red line under the printer box when the printing system does not answer.
screenshot-print-not-answering = الطباعة لا تردّ.

# Red line under the printer box when printing is not installed.
screenshot-print-not-installed = الطباعة غير مثبتة على هذا الحاسوب.

# Red line under the printer box when the chosen printer is paused.
screenshot-print-paused = { $name } متوقفة مؤقتًا.

# Label and name of the box where the printer is chosen.
screenshot-print-printer = الطابعة

# Label and name of the choice of which pages are printed.
screenshot-print-pages = الصفحات

# The choice to print all the pages.
screenshot-print-all = الكل

# The choice to print a range of pages.
screenshot-print-range = نطاق

# Label and name of the box for the number of copies.
screenshot-print-copies = النسخ

# Label and name of the choice of printing on both sides.
screenshot-print-sides = الوجهان

# Label and name of the choice of colour, and the choice for colour.
screenshot-print-colour = ملوّن

# The choice to print in black and white.
screenshot-print-mono = أبيض وأسود

# Label and name of the choice of paper size.
screenshot-print-paper = الورق

# Label and name of the choice of how the page stands.
screenshot-print-page = الصفحة

# Name of the main button that prints, and the title when nothing else is known.
screenshot-print-go = اطبع

# Title of the print window for a named document.
screenshot-print-doc = اطبع { $doc }

# Title of the print window when the program has no document name.
screenshot-print-from = اطبع من { $who }

# Title of the notice that says a print did not go.
screenshot-print-failed = تعذّرت الطباعة

# Words of that notice when the choices made were not kept.
screenshot-print-lost = لم تُحفظ اختياراتها. اطبع مرة أخرى.

# Line in the menu on a file or folder: change its name.
screenshot-chooser-rename = إعادة تسمية

# Line in the menu on a file or folder: cut it.
screenshot-chooser-cut = قص

# Line in the menu: paste what was cut or copied.
screenshot-chooser-paste = لصق

# Line in the menu and name of the round button: make a new folder.
screenshot-chooser-new-folder = مجلد جديد

# Line in the menu: move to the Trash.
# SAFETY: needs a human review before it ships
screenshot-chooser-trash = انقل إلى سلة المهملات

# Name of the first place in the side bar, and of the only part of the path when Recent is shown.
screenshot-chooser-recent = الأخيرة

# Name of the person's home folder in the side bar.
screenshot-chooser-home = المنزل

# Name the screen reader gives to the path along the top.
screenshot-chooser-folder = المجلد

# Shown when a folder has nothing in it.
screenshot-chooser-empty = لا شيء هنا

# Name of the choice of which kind of file is shown.
screenshot-chooser-kind = نوع الملف

# Said where a place on another computer is not answering.
screenshot-chooser-not-answering = الحاسوب الآخر لا يردّ.

# What the saved list of last folders is called when it is found damaged and set aside.
screenshot-chooser-damaged = مجلدات أداة الاختيار

# Words under the box when the folder a name points into is not there.
screenshot-chooser-no-folder = ذلك المجلد غير موجود.

# Words under the box when saving would replace a file.
# SAFETY: needs a human review before it ships
screenshot-chooser-replace-ask = { $name } موجود بالفعل. اضغط استبدال، أو Enter مرة أخرى، لوضع هذا مكانه.

# Words under the box when a key is pressed on Recent, which is not a folder.
screenshot-chooser-recent-not-folder = الأخيرة ليست مجلدًا. افتح مجلدًا أولًا.

# Words under the box when a key is pressed with nothing picked.
screenshot-chooser-nothing-picked = لم يُختر شيء.

# Words under the box while a file by that name is already there.
screenshot-chooser-there = { $name } موجود بالفعل.

# Name of the round button that hides the hidden files.
screenshot-chooser-hide-hidden = أخفِ الملفات المخفية

# Name of the round button that shows the hidden files.
screenshot-chooser-show-hidden = أظهر الملفات المخفية

# Words of the main button when the program gave none, to choose a folder.
screenshot-chooser-choose-folder = اختر المجلد

# Head of the question when a folder of that name is already there.
screenshot-chooser-clash-folder = يوجد بالفعل مجلد باسم { $name }

# Head of the question when a file of that name is already there.
screenshot-chooser-clash-file = { $name } موجود بالفعل

# Name of the round button that leaves a pasted thing out.
# SAFETY: needs a human review before it ships
screenshot-chooser-skip = تخطَّ

# Name of the round button that keeps both under different names.
# SAFETY: needs a human review before it ships
screenshot-chooser-both = احتفظ بالاثنين

# Word on the button that replaces what is there.
# SAFETY: needs a human review before it ships
screenshot-chooser-replace = استبدال

# What the screen reader says for the button that replaces a folder.
# SAFETY: needs a human review before it ships
screenshot-chooser-replace-spoken = استبدال: المجلد الموجود بالفعل يذهب إلى سلة المهملات

# Name of the main button that merges two folders.
# SAFETY: needs a human review before it ships
screenshot-chooser-merge = دمج

# Words in the question when folders are being pasted.
screenshot-chooser-rest-folders = الجواب نفسه لكل مجلد موجود بالفعل.

# Words in the question when files are being pasted.
screenshot-chooser-rest-files = الجواب نفسه لكل ملف موجود بالفعل.

# Name of the switch that gives the same answer to the rest.
screenshot-chooser-same = افعل الشيء نفسه مع الباقي

# Words under the box after Cut.
screenshot-chooser-was-cut = تم قص { $what }. الصقه في مجلد.

# Words under the box after Copy.
screenshot-chooser-was-copied = تم نسخ { $what }. الصقه في مجلد.

# Words under the box when Paste is pressed and there is nothing to paste.
screenshot-chooser-nothing-cut = لم يُقص أو يُنسخ شيء.

# Words under the box when Paste is pressed while a paste is still going.
screenshot-chooser-paste-busy = لا يزال لصق قيد التنفيذ.

# Red line after some items could not be moved.
screenshot-chooser-not-moved =
    { $n ->
        [zero] لم يتعذّر نقل أي عنصر: { $names }.
        [one] تعذّر نقل عنصر واحد: { $names }.
        [two] تعذّر نقل عنصرين: { $names }.
        [few] تعذّر نقل { $n } عناصر: { $names }.
        [many] تعذّر نقل { $n } عنصرًا: { $names }.
       *[other] تعذّر نقل { $n } عنصر: { $names }.
    }

# Red line after some items could not be copied.
screenshot-chooser-not-copied =
    { $n ->
        [zero] لم يتعذّر نسخ أي عنصر: { $names }.
        [one] تعذّر نسخ عنصر واحد: { $names }.
        [two] تعذّر نسخ عنصرين: { $names }.
        [few] تعذّر نسخ { $n } عناصر: { $names }.
        [many] تعذّر نسخ { $n } عنصرًا: { $names }.
       *[other] تعذّر نسخ { $n } عنصر: { $names }.
    }

# Red line after some items could not be put in the Trash.
screenshot-chooser-not-trashed =
    { $n ->
        [zero] لم يتعذّر نقل أي عنصر إلى سلة المهملات: { $names }.
        [one] تعذّر نقل عنصر واحد إلى سلة المهملات: { $names }.
        [two] تعذّر نقل عنصرين إلى سلة المهملات: { $names }.
        [few] تعذّر نقل { $n } عناصر إلى سلة المهملات: { $names }.
        [many] تعذّر نقل { $n } عنصرًا إلى سلة المهملات: { $names }.
       *[other] تعذّر نقل { $n } عنصر إلى سلة المهملات: { $names }.
    }

# Name of the box where a file or folder is renamed.
screenshot-chooser-new-name = اسم جديد لـ { $old }

# What several things are called in a line such as Cut.
screenshot-chooser-items =
    { $n ->
        [zero] لا عناصر
        [one] عنصر واحد
        [two] عنصران
        [few] { $n } عناصر
        [many] { $n } عنصرًا
       *[other] { $n } عنصر
    }

# Title of the notice when an email could not be started.
screenshot-mail-failed = تعذّر بدء البريد

# Why: no mail program is chosen or installed.
screenshot-mail-no-app = لم يُحدَّد برنامج بريد. اختر واحدًا في الإعدادات، التطبيقات الافتراضية.

# Why: a program asked to attach more files than allowed.
screenshot-mail-too-many = ملفات كثيرة جدًا للإرفاق.

# Why: a file, or all the files together, are too big.
screenshot-mail-too-big = الملفات أكبر من أن تُرفق.

# Why: a file could not be read as a plain file.
screenshot-mail-bad-file = تعذّر إرفاق ملف.

# Why: an address was not a plain address.
screenshot-mail-bad-address = عنوان غير صالح.

# Why: the subject was too long or had odd characters.
screenshot-mail-bad-subject = الموضوع غير صالح.

# Why: the words were too long or had odd characters.
screenshot-mail-bad-words = النص غير صالح.

# Why: no private place to keep the files.
screenshot-mail-no-place = تعذّر حفظ الملفات بأمان.

# Why: the mail program did not start.
screenshot-mail-not-started = لم يبدأ برنامج البريد.

# Line in the menu on a file or folder: delete it for good, after asking.
screenshot-chooser-delete = احذف نهائياً

# Question before deleting for good.
screenshot-chooser-delete-ask = هل تحذف نهائياً؟

# Line in the menu: copy the place of the thing as text.
screenshot-chooser-copy-location = انسخ الموقع

# Line in the menu on a folder: pin it to Folders' side bar.
screenshot-chooser-pin = ثبّت في الشريط الجانبي

# Line in the menu on a folder pinned already.
screenshot-chooser-unpin = ألغِ التثبيت

# Red line after a pin could not be kept.
screenshot-chooser-pin-not-kept = تعذّر حفظ التثبيت.

# Red line after some items could not be deleted.
screenshot-chooser-not-deleted =
    { $n ->
        [zero] لم يُحذف أي عنصر: { $names }.
        [one] تعذّر حذف عنصر واحد: { $names }.
        [two] تعذّر حذف عنصرين: { $names }.
        [few] تعذّر حذف { $n } عناصر: { $names }.
        [many] تعذّر حذف { $n } عنصراً: { $names }.
       *[other] تعذّر حذف { $n } عنصر: { $names }.
    }

