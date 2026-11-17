
# machine translated (Arabic), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = اسم الدخول هذا غير صالح.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = اسم الدخول هذا طويل جداً.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = لا يوجد مستخدم باسم { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } حساب نظام، لذلك لم يغيّره Trindle.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = لم تُعطَ كلمة مرور.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [zero] يجب ألا تقل كلمة المرور عن { $n } حرف.
        [one] يجب ألا تقل كلمة المرور عن حرف واحد.
        [two] يجب ألا تقل كلمة المرور عن حرفين.
        [few] يجب ألا تقل كلمة المرور عن { $n } أحرف.
        [many] يجب ألا تقل كلمة المرور عن { $n } حرفاً.
       *[other] يجب ألا تقل كلمة المرور عن { $n } حرف.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = لا يمكن أن تحتوي كلمة المرور على فاصل أسطر.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } موجود بالفعل.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } هو المسؤول الوحيد، لذلك يجب أن يبقى مسؤولاً.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = لا يمكنك حذف حسابك أنت.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = حذف Trindle حساب { $name }، لكنه لم يتمكن من إزالة صورته على شاشة الدخول: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = هذه الصورة كبيرة جداً.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = هذه ليست صورة صنعها Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = لم يتمكن Trindle من إعطاء شاشة الدخول إذن الوصول إلى مجلد الصور: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = هذا ليس اسم لغة.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = لم يتمكن Trindle من قراءة قائمة اللغات في Debian: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian لا يتعرف على اللغة { $code }.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = اسم الحاسوب لا يستخدم إلا الأحرف والأرقام والشرطات. ويمكن أن يصل طوله إلى 63 حرفاً، ولا يجوز أن يبدأ بشرطة أو ينتهي بها.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = حد الشحن مفعّل الآن، لكنه سيضيع عند إطفاء الحاسوب: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = الشحن حتى الامتلاء مفعّل الآن، لكن الحد سيعود عند التشغيل التالي للحاسوب: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = لا يزال بإمكان الحواسيب الأخرى الاتصال، لأن Trindle لم يتمكن من إيقاف المشاركة: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = هذا ليس معرّف إضافة.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = لا يوجد هذا الشخص.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = لا يوجد مجلد لهذا الشخص.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = لم يتمكن Trindle من حفظ قائمة الإضافات المسموح بها: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = لا توجد هذه الإضافة.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = لم يتم تحديث شيء. لم يتمكن Trindle من معرفة الإصدار المثبّت منه، فلم يستطع استثناء تحديثه هو.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = تثبيت التحديثات

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = ترتيب ما بعد التثبيت

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = تثبيت Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = هذا ليس اسم حزمة.

# Red words when the package to remove is not installed.
admin-package-not-installed = هذه الحزمة غير مثبّتة.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = هذا جهاز لا يعرفه fwupd.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }؛ { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = لا يعرف Trindle من طلب Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = لم يبدأ الحجب في الوقت المناسب.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = الحجب ليس متوقفاً تماماً: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = لم يتمكن Trindle من إعادة { $path }: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = هذا ليس اسم موقع.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = لا يوجد هذا الخادم.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = توقف { $tool } قبل أن يحصل على كل ما يحتاجه، لذلك لم ينتهِ العمل: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = فشل { $tool }.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = تعذّرت إزالة { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = لم يتغير شيء، لأن Trindle لم يتمكن من قراءة { $path }: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } ولم يتمكن Trindle كذلك من إعادة الأشياء إلى ما كانت عليه: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = هذه ليست مدة صالحة لحفظ النسخ السابقة.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = لا توجد النسخ السابقة.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = يوجد ملف في المكان الذي يجب أن يكون فيه مجلد النسخ السابقة.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = لا يُعرف من طلب ذلك.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = يعمل هذا عبر pkexec فقط.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = هذا ليس اسم حاسوب.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = هذا ليس اسم مجلد مشترك.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = هذا ليس عنوان حاسوب.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = هذا ليس مسار مجلد مشترك.

# Red words in Folders when the shared folder to close is not one of the person's own.
# SAFETY: needs a human review before it ships
admin-share-not-yours = لا يمكن هنا إغلاق إلا مجلداتك المشتركة أنت.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = لا يوجد مجلد مشترك مفتوح هناك.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = غيّر شيء ما { $path } منذ أن أعدّه Trindle، لذلك لا شيء متصل هناك.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } تغيّر عمّا تركه Trindle.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = عدد المجلدات المشتركة المفتوحة كبير جداً.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } غير مثبّت. وهو يأتي مع حزمة { $package } من Debian.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = تعذّرت قراءة بيانات الدخول.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = لم يتمكن Trindle من قراءة ما هو مركّب على هذا الحاسوب: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = لا يزال أحد البرامج يستخدم شيئاً في هذا المجلد. أغلق البرنامج ثم حاول مرة أخرى.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = لا يستجيب ذلك الحاسوب. سيُغلق المجلد المشترك عند تسجيل خروجك.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = هذا ليس معرّف مستخدم.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = تعذّر إنشاء مجموعة sambashare.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = الحصول على Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = إعداد المشاركة

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = المشاركة غير مُعدّة بعد. شارك مجلداً أولاً.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = لا يزال هذا الحاسوب ظاهراً للآخرين على الشبكة: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = هذا ليس رقم لقطة.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = أُخذت من الإعدادات

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = لم يتمكن Trindle من معرفة القرص الذي عليه النظام.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
# SAFETY: needs a human review before it ships
admin-system-subvolume = النظام ليس في وحدة Btrfs فرعية خاصة به، لذلك لا يستطيع Trindle استبداله بلقطة.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = لم يتمكن Trindle من الحصول على مفتاح التوقيع من { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = لم يرسل ذلك العنوان مفتاح توقيع.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = لم يتغير شيء، لأن Trindle لم يتمكن من قراءة { $path }: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = الحصول على مفتاح توقيعه

# Second line of the progress while a source is added.
admin-step-adding-source = إضافته وقراءة قائمته

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = تعذّر استخدام المصدر. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = لم يتمكن Trindle من استخدام المصدر، لذلك أزاله من جديد. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = لا يستطيع { $user } تشغيله أو إيقافه بعد: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = لم يتمكن Trindle من تشغيله لـ{ $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = لم يتمكن Trindle من السماح لـ{ $user } باستخدام الأجهزة الافتراضية التي يحتاجها Cowork: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = لم يتمكن Trindle من السماح لـ{ $user } باستخدامه دون كلمة مرور: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = تم تثبيت { $name }، لكن Trindle لم يتمكن من إنهاء إعداده: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = تثبيته

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = هذا ليس واحداً من المصادر الجاهزة.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } غير مصنوع لهذا النوع من الحواسيب ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = إعداده

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = يجب أن يكون كل عنوان عنوان ويب عادياً يبدأ بـ https://.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = يجب أن يكون الإصدار (suite) والمكوّنات كلمات عادية.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = هذا ليس اسم حزمة.

# First line of the progress while a source and its program are taken away.
admin-step-removing = إزالته

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = قراءة القوائم من جديد

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = لا تُزال هنا إلا المصادر التي أضافها Trindle.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = المصدر لا يزال موجوداً: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = اللقطة { $n } غير موجودة على القرص.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-not-ready = لم يتغير شيء، لأن Trindle لم يتمكن من تجهيز اللقطة. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-aside = لم يتمكن Trindle من وضع النظام الحالي جانباً: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = النظام الحالي اسمه الآن { $aside } بدلاً من { $sub }، ولن يبدأ الحاسوب حتى يُسمّى { $sub } من جديد: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = لم يتمكن Trindle من نقل اللقطات: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }؛ { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-in-place = لم يتمكن Trindle من وضع اللقطة في مكانها: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = لم يتمكن Trindle من تحديث قائمة الأنظمة التي وُضعت جانباً: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = تعذّرت إزالة { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = أزال Trindle هذه لأن شيئاً لم يعد يحتاج إليها: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = لم تُثبَّت كل التحديثات. المؤجَّل منها: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } متوقف لأن برنامج تشغيله ({ $driver }، DKMS) لا يمكن بناؤه لنواة Linux الجديدة بعد.

admin-driver-off = لا يمكن بناء برنامج التشغيل ({ $driver }، DKMS) لنواة Linux الجديدة بعد. الميزات التي تحتاج إليه متوقفة.

# Said once after the drivers that are off.
admin-driver-off-rest = تم تثبيت النواة وبقية التحديث.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = الكاميرا الافتراضية

admin-language-in-use = { $code } هي اللغة التي يبدأ بها هذا الحاسوب، لذلك لا يمكن إزالتها.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = ليس في هذا الحاسوب إبطاء القفل المنقسم ليتم إيقافه.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = لم يتمكن Trindle من العثور على هذا القرص، أو أنه قرص النظام.

# The drive could not be closed where it was open.
admin-games-drive-busy = القرص قيد الاستخدام. أغلق أي ملفات أو برامج مفتوحة عليه ثم حاول مرة أخرى: { $why }

# Another software change has the read-only system open.
admin-system-busy = هناك تغيير آخر في البرامج قيد التنفيذ. حاول مرة أخرى عند انتهائه.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = لم يتغير شيء، لأن Trindle لم يتمكن من فتح قفل النظام: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = لم يتمكن Trindle من قفل النظام مرة أخرى. سيُقفل عند التشغيل التالي: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = لم يتمكن Trindle من إبقاء /usr/local قابلاً للكتابة: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = لا يمكن تشغيل هذا إلا بحساب الجذر، وليس عبر pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = يحتاج هذا إلى Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = اللقطة في مكانها، وسيبدأ الحاسوب منها. لم يتمكن Trindle من إنهاء تنظيف النظام القديم. وسيحاول مرة أخرى عند التشغيل التالي للحاسوب: { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = يمكن فتح { $most } منافذ على الأكثر.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = لم يتمكن Trindle من حفظ قائمة المنافذ المفتوحة: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = جدار الحماية ما زال يعمل: { $why }

admin-plugin-ports-changed = تطلب هذه الإضافة الآن منافذ شبكة غير التي سمحت بها. اسمح لها من جديد لفتحها.

admin-source-not-you = لا يمكن إضافة برنامج إلا للشخص الذي يطلب ذلك.

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = استُخدم الاسم نفسه في استرجاع آخر قبل لحظات. انتظر بضع ثوانٍ ثم حاول مرة أخرى.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = تعذّر فصل { $path }: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [zero] لم يبقَ أي مجلد مشترك متصلًا.
        [one] لم يستجب مجلد مشترك واحد، لذلك لا يزال متصلًا. سيُفصل عند إيقاف الحاسوب.
        [two] لم يستجب مجلدان مشتركان، لذلك لا يزالان متصلين. سيُفصلان عند إيقاف الحاسوب.
        [few] لم تستجب { $n } مجلدات مشتركة، لذلك لا تزال متصلة. ستُفصل عند إيقاف الحاسوب.
        [many] لم يستجب { $n } مجلدًا مشتركًا، لذلك لا تزال متصلة. ستُفصل عند إيقاف الحاسوب.
       *[other] لم يستجب { $n } مجلد مشترك، لذلك لا تزال متصلة. ستُفصل عند إيقاف الحاسوب.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = تعذّر إيقاف ملف التبديل { $path }: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = تعذّرت إعادة تشغيل ملف التبديل { $path }: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } ليس من ملفات التبديل في النظام.

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = لن يستخدم Trindle ملف التبديل { $path }، لأن غير المسؤول يستطيع تغيير { $why } أو قراءته.

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = لم يُعرف مكان ملف التبديل الآن، وقد لا يعمل الاستئناف من السبات.

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = سيعمل السبات من جديد بعد إعادة التشغيل.
