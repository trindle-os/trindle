# machine translated (Arabic), not yet reviewed by a person.

# Name of the Settings page about default apps, in the list and as its heading.
settings-default-apps-title = التطبيقات الافتراضية

# Words a person might type into the Settings search to find the default apps page; separate them with spaces.
settings-default-apps-keywords = browser mail email files pictures music video pdf calendar open with terminal متصفح بريد ملفات صور موسيقى فيديو تقويم فتح باستخدام طرفية

# Introduction under the heading of the default apps page, and the words behind the page i.
settings-default-apps-help = أي برنامج يفتح روابط الويب والبريد والصور والتقويم وغيرها. تُختار الطرفية في ~/.config/xdg-terminals.list؛ وإن لم يُختر شيء، يستخدم Trindle أول واحدة مثبتة.

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-web = الويب

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-mail = البريد

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-folders = المجلدات

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-text = النصوص

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pictures = الصور

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-music = الموسيقى

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-video = الفيديو

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pdf = مستندات PDF

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-calendar = التقويم

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-web-in-sentence = الويب

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-mail-in-sentence = البريد

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-folders-in-sentence = المجلدات

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-text-in-sentence = النصوص

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pictures-in-sentence = الصور

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-music-in-sentence = الموسيقى

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-video-in-sentence = الفيديو

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pdf-in-sentence = مستندات PDF

# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-calendar-in-sentence = التقويم

# Title of a card on the default apps page.
settings-default-apps-card-internet = الإنترنت

# Words behind the i of a card on the default apps page.
settings-default-apps-card-internet-about = البرنامج الذي يفتح رابط ويب يُنقر عليه في أي مكان، والذي يكتب بريدًا عند النقر على عنوان، والذي يفتح دعوة تقويم.

# Title of a card on the default apps page.
settings-default-apps-card-files = الملفات والمستندات

# Words behind the i of a card on the default apps page.
settings-default-apps-card-files-about = البرنامج الذي يفتح مجلدًا وملف نص عاديًا ومستند PDF عندما تفتح واحدًا من المجلدات أو سطح المكتب أو تطبيق آخر.

# Title of a card on the default apps page.
settings-default-apps-card-media = الصور والصوت

# Words behind the i of a card on the default apps page.
settings-default-apps-card-media-about = البرنامج الذي يفتح صورة أو مقطوعة موسيقية أو فيديو عندما تفتح واحدًا منها.

# Sentence at the top of the page when every kind has a program; { $web } is the program that opens web links.
settings-default-apps-all-set-web = لكل شيء برنامج يفتحه؛ و{ $web } يفتح روابط الويب.

# Sentence at the top of the page when every kind has a program and the one for web links is not known.
settings-default-apps-all-set = لكل شيء برنامج يفتحه.

# Sentence at the top of the page when one kind has no program chosen; { $kind } is its name in lower case.
settings-default-apps-one-missing = لم يُختر بعد برنامج لفتح { $kind }.

# Sentence at the top of the page when several kinds have no program chosen; { $kinds } is their names as a list joined with "or".
settings-default-apps-many-missing = لم يُختر بعد برنامج لفتح { $kinds }.

# Small words under a kind that no installed program can open.
settings-default-apps-none-installed = لا شيء مثبت يستطيع فتح هذه بعد.

# First choice in a list when no program is chosen for that kind yet.
settings-default-apps-not-chosen = لم يُختر بعد

# Small words under a kind with a program list; { $n } is how many programs can open it.
settings-default-apps-can-open =
    { $n ->
        [zero] لا برنامج يستطيع فتحها.
        [one] البرنامج الوحيد القادر على ذلك.
        [two] برنامجان يستطيعان فتحها.
        [few] { $n } برامج تستطيع فتحها.
        [many] { $n } برنامجًا يستطيع فتحها.
       *[other] { $n } برنامج يستطيع فتحها.
    }

# Said in green under a kind's list after the choice was kept.
settings-default-apps-saved = تم الحفظ

