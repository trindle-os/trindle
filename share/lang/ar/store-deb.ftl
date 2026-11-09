
# machine translated (Arabic), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = تثبيت حزمة

# Words at the top of the window while the package is read.
store-deb-reading = جارٍ قراءة الحزمة…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = هذا الملف ليس حزمة Debian يمكن تثبيتها. ربما لم يكتمل تنزيله.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } جاهز للتثبيت في مجلده الخاص.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = هذه الحزمة مخصصة لحواسيب { $arch }، فلا يمكن تثبيتها على هذا الحاسوب.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = هذا هو Trindle نفسه. يُحدَّث من صفحة التحديثات في التطبيقات، لا من ملف.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } برنامج تشغيل أو خدمة في الخلفية، ولا يعمل من مجلده الخاص، لذلك لا يمكن تثبيته هنا.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = الحزمة القادمة من خارج Debian وTrindle توضع في مجلدها الخاص، حيث تكون مقيّدة لكن غير معزولة تمامًا. ثبّت فقط الحزم من جهات تثق بها.

# Words at the top of the window once the package is installed.
store-deb-installed = تم تثبيت { $name } في مجلده الخاص.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = جارٍ التحقق من مصدر هذا الملف…

# Heading in the card that says where the file came from.
store-deb-where = مصدره

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = أزِله مع ما حفظه

# Word beside the ring while the package goes in.
store-deb-installing = جارٍ التثبيت

# Main button that installs the package.
store-deb-install = ثبّت

# Main button that installs the same version again.
store-deb-reinstall = أعد التثبيت

# Main button that brings the installed package up to this version.
store-deb-update = حدّث

# Heading of the card for a package not installed yet.
store-deb-goes-in = يوضع في مجلده الخاص، مقيّدًا افتراضيًا

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = لا تلزم كلمة مرور. لا يستطيع الوصول إلى الإنترنت أو الصوت أو ملفاتك حتى تسمح له.

# Heading of the card for a package installed at this version.
store-deb-same-version = الإصدار { $version } مثبّت في مجلده الخاص

# Under that heading.
store-deb-same-version-note = أعد تثبيته هناك إذا توقف عن العمل كما ينبغي.

# Heading of the card for a package installed at another version.
store-deb-other-version = الإصدار { $version } مثبّت في مجلده الخاص

# Under that heading: the version this brings.
store-deb-other-version-note = يرفعه التثبيت هناك إلى الإصدار { $version }، مع الإبقاء على ما حفظه.

# Facts line: how big the package is once installed.
store-deb-fact-size = الحجم بعد التثبيت

# Facts line: the computer kind it is made for.
store-deb-fact-arch = مخصص لـ

# Text of the About card when the package has no description.
store-deb-no-description = ليس لهذه الحزمة وصف.

# Words at the top of the window while the package goes in.
store-deb-installing-note = جارٍ التثبيت في مجلده الخاص. قد يستغرق جلب ما يحتاجه بضع دقائق.

# Words at the top of the window once the package is installed.
store-deb-done = تم التثبيت في مجلده الخاص. ستجده في الحلقة.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = تم التثبيت في مجلده الخاص، لكن تعذّر العثور على هذه الأجزاء، فقد لا يبدأ: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = أُزيل من مجلده الخاص مع كل ما حفظه.

# Red words when a file is not a Debian package.
store-boxed-not-deb = هذا الملف ليس حزمة Debian يمكن تثبيتها.

# Step shown while a package is unpacked.
store-boxed-unpacking = جارٍ فك ضغط { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = جارٍ إضافة { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } ليس حزمة Debian يمكن تثبيتها.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = تعذّر فك ضغط { $path }.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = هذا البرنامج ليس محفوظًا في صندوق.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = الطرفية

# Title of the notice when older programs are off.
store-boxed-x-off = البرامج الأقدم متوقفة

# Body of that notice: where to turn them on.
store-boxed-x-off-where = الإعدادات، الأذونات

# Title of the notice when too many older programs are open.
store-boxed-x-many = برامج قديمة كثيرة جدًا

# Body of that notice.
store-boxed-x-many-close = أغلق واحدًا وحاول مرة أخرى

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = تعذّر فتح { $app }

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = تستطيع الطرفية الوصول إلى جزء أكبر من حاسوبك مما تصل إليه التطبيقات الأخرى، لأن عملها يتطلب ذلك. لا تكتب إلا أوامر تفهمها، ومن أشخاص تثق بهم.

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = الشبكة المنزلية متوقفة

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = لا شبكة لدى { $app }: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = تعذّر العثور على برنامجه الخاص

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare غير مثبّت

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta غير مثبّت

# The same: the network room was not made.
store-boxed-home-why-not-made = لم تُنشأ غرفة شبكته

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = لم تكن غرفة شبكته جديدة

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = تعذّر إنشاء مرشّح استدعاءات النظام

# The same: the room's rules would not load.
store-boxed-home-why-rules = لم تُحمَّل قواعد الغرفة

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = لم يبدأ { $program }: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = قال pasta: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = تعذّر على pasta الانضمام إلى الغرفة



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = فُتح دون إنترنت

# Body of that notice: why, and what the program was given.
store-downloaded-why = لقد نُزِّل، لذا يحصل البرنامج على ذلك الملف فقط، للقراءة فقط.

