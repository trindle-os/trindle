# machine translated (Arabic), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = هذا ليس اسم برنامج صالحًا.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = لم يُحفظ ذلك.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = اختر مجلدًا موجودًا.

# Red words when the whole home folder is chosen to share.
store-fence-whole-home = يبقى المجلد الرئيسي كله خاصًا.

# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = يحتوي هذا المجلد على مجلدك الرئيسي، لذا يبقى خاصًا.

# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = يمكن مشاركة المجلدات داخل مجلدك الرئيسي فقط.

# Red words when a hidden folder is chosen to share.
store-fence-hidden = المجلدات المخفية تبقى خاصة دائمًا.

# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = يحتوي هذا المجلد على كلمات مرور أو بريد، لذا يبقى خاصًا.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = ليس لهذا البرنامج مجلد إعدادات يمكن استخدامه.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = اكتب اسم موقع، مثل api.example.com، أو *.example.com لكل موقع تحت هذا الاسم.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = اكتب اسم حاسوب، مثل nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = اكتب كل واحد بالشكل NAME=value.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = اكتب كل واحد بالشكل NAME=value، بأحرف كبيرة.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = يطلب برنامج ذكاء اصطناعي استخدام هذا المجلد. لا يستطيع رؤيته الآن.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = يطلب برنامج ذكاء اصطناعي تشغيل هذا بصفة المسؤول. سيعمل خارج السياج وسيستطيع رؤية كل ما تراه أنت.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = برنامج ذكاء اصطناعي

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = اسمح له بقراءة هذا المجلد فقط

# Note under the question about a folder.
store-fence-sees-next = سيرى المجلد عند بدء تشغيل البرنامج في المرة القادمة.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = لا

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = شارك لهذه الجلسة فقط

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = شارك دائمًا

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = لا تسمح

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = اسمح

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = هذا الأمر أطول من أن يُعرض كاملًا، لذلك لم يُنفَّذ.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = هناك طلب آخر ينتظر ردّك.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = لم تسمح بهذا.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } غير متاح داخل السياج.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = تعذّر تشغيله خارج السياج.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = أعطِ المسار الكامل للمجلد.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = لم تشاركه.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = تمت المشاركة. سيكون موجودًا في المرة القادمة التي يبدأ فيها هذا البرنامج.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = اذكر الحاسوب المراد الوصول إليه.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = لا تستطيع برامج الذكاء الاصطناعي داخل السياج الوصول إلى ذلك الحاسوب عبر ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = تعذّر إنشاء المفتاح الخاص ببرامج الذكاء الاصطناعي: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = اذكر أمرًا لتشغيله.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } غير متاح هنا.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } غير متاح داخل السياج. اطلب تشغيله في الخارج.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: خط الطلبات الخاص بالسياج غير موجود.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: لم يُرسل الطلب.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = رفض Landlock: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = لم يبدأ السياج: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = لم يبدأ السياج: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = تحتفظ برامج الذكاء الاصطناعي هنا بتسجيل دخولها الخاص، بمعزل عن تسجيلك. سجّل الدخول داخل البرنامج عند الطلب.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = مجلداتك خاصة ولا تراها برامج الذكاء الاصطناعي. شارك مجلدًا من الإعدادات، برامج الذكاء الاصطناعي، لتعمل فيه.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = لم يبدأ السياج: تعذّر إنشاء مرشّح استدعاءات النظام.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = لم يبدأ السياج: تعذّر إعداد حدّ الشبكة.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = لم يبدأ السياج: رفض bubblewrap العمل: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: غير موجود خارج السياج
