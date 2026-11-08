# machine translated (Arabic), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = البدء عند تسجيل الدخول

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay بدء تلقائي تسجيل الدخول إقلاع تشغيل تأخير

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = تطبيقات تفتح من تلقاء نفسها عند تسجيل الدخول. يتيح التأخير للتطبيق أن ينتظر الشبكة أو تطبيقًا آخر أولًا.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = لا يبدأ شيء من تلقاء نفسه عند تسجيل الدخول.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = يبدأ { $name } من تلقاء نفسه عند تسجيل الدخول.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [zero] لا يبدأ أي تطبيق من تلقاء نفسه عند تسجيل الدخول.
        [one] يبدأ تطبيق واحد من تلقاء نفسه عند تسجيل الدخول.
        [two] يبدأ تطبيقان من تلقاء نفسيهما عند تسجيل الدخول.
        [few] تبدأ { $n } تطبيقات من تلقاء نفسها عند تسجيل الدخول.
        [many] يبدأ { $n } تطبيقًا من تلقاء نفسها عند تسجيل الدخول.
       *[other] يبدأ { $n } تطبيق من تلقاء نفسها عند تسجيل الدخول.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [zero] لا يوجد غيرها محفوظًا ومعطلًا.
        [one] وهناك تطبيق آخر محفوظ ومعطَّل.
        [two] وهناك تطبيقان آخران محفوظان ومعطَّلان.
        [few] وهناك { $n } تطبيقات أخرى محفوظة ومعطَّلة.
        [many] وهناك { $n } تطبيقًا آخر محفوظة ومعطَّلة.
       *[other] وهناك { $n } تطبيق آخر محفوظة ومعطَّلة.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = فورًا

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = بعد دقيقة

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = بعد دقيقتين

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [zero] فورًا
        [one] بعد ثانية واحدة
        [two] بعد ثانيتين
        [few] بعد { $n } ثوانٍ
        [many] بعد { $n } ثانية
       *[other] بعد { $n } ثانية
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = تأخير { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = ابدأه عند تسجيل الدخول

# Name of the bin button on an app's line that stops it starting at login.
# SAFETY: needs a human review before it ships
settings-startup-stop = أوقف بدءه عند تسجيل الدخول

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = تعذّرت إزالته، لذا لا يزال يبدأ عند تسجيل الدخول.

# Red words under the add card when something could not be added.
settings-startup-not-added = تعذّرت إضافته.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = ابحث عن تطبيق

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = ابدأ هذا عند تسجيل الدخول

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = تعذّرت إضافته: لم يعد التطبيق مثبتًا.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = مثال: syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = اختر برنامجًا من القرص

# Heading of the line for adding a command.
settings-startup-command = أمر

# Small words under the command line.
settings-startup-command-about = كما يُكتب في الطرفية، أو برنامج على القرص.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = اكتب أمرًا، أو اختر برنامجًا.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = اختر برنامجًا يبدأ عند تسجيل الدخول

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = لا شيء يبدأ من تلقاء نفسه

# Small words under that line.
settings-startup-none-about = أضف تطبيقًا بجانب هذا.

# Title of the card listing what starts at login.
settings-startup-list-title = يبدأ عند تسجيل الدخول

# Words behind the i of the card listing what starts at login.
settings-startup-about = يمكن لكل تطبيق أن ينتظر قليلًا بعد تسجيل دخولك، حتى تصبح الشبكة أو تطبيق آخر جاهزًا أولًا. يبقيه مفتاحه هنا دون أن يبدأه؛ وسلة المهملات توقف بدءه عند تسجيل الدخول نهائيًا.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = يشغّل { $name }: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = اختر من التطبيقات المثبتة

# Name of the button that opens the box for typing a command.
settings-startup-open-command = اكتب أمرًا، أو اختر برنامجًا

# Heading of the line for adding an installed app.
settings-startup-app-heading = تطبيق مثبت

# Small words under that line.
settings-startup-app-about = اختر واحدًا من قائمة التطبيقات.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = أمر أو برنامج

# Small words under that line.
settings-startup-cmd-about = يُكتب، أو يُختار من القرص.

# Words behind the i of the Add an app card.
settings-startup-add-about = اختر من التطبيقات المثبتة، أو أعطِ أمرًا للتشغيل كما تكتبه في الطرفية، أو برنامجًا تختاره من القرص. يبدأ عند تسجيل الدخول التالي، فورًا؛ ويمكن اختيار مدة انتظار له بعد دخوله إلى القائمة.

# Title of the card for adding an app.
settings-startup-add-title = أضف تطبيقًا

