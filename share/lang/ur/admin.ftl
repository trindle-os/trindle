# machine translated (Urdu), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = یہ لاگ ان نام درست نہیں ہے۔

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = یہ لاگ ان نام بہت لمبا ہے۔

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name } نام کا کوئی صارف نہیں۔

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } سسٹم کا اکاؤنٹ ہے، اس لیے Trindle نے اسے تبدیل نہیں کیا۔

# SAFETY: needs a human review before it ships
# Red words in Settings when no password reached the helper that sets it.
admin-password-none = کوئی پاس ورڈ نہیں دیا گیا۔

# SAFETY: needs a human review before it ships
# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
        [one] پاس ورڈ میں کم از کم { $n } حرف ہونا چاہیے۔
       *[other] پاس ورڈ میں کم از کم { $n } حروف ہونے چاہییں۔
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = پاس ورڈ میں نئی سطر نہیں ہو سکتی۔

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } پہلے سے موجود ہے۔

# SAFETY: needs a human review before it ships
# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } واحد منتظم ہے، اس لیے اسے منتظم ہی رہنا ہوگا۔

# SAFETY: needs a human review before it ships
# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = آپ اپنا ہی اکاؤنٹ حذف نہیں کر سکتے۔

# SAFETY: needs a human review before it ships
# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = Trindle نے { $name } کا اکاؤنٹ حذف کر دیا، لیکن اس کی لاگ ان اسکرین کی تصویر نہیں ہٹا سکا: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = یہ تصویر بہت بڑی ہے۔

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = یہ Trindle کی بنائی ہوئی تصویر نہیں ہے۔

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle لاگ ان اسکرین کو تصاویر کے فولڈر تک رسائی نہیں دے سکا: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = یہ زبان کا نام نہیں ہے۔

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle، Debian کی زبانوں کی فہرست نہیں پڑھ سکا: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian زبان { $code } کو نہیں پہچانتا۔

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = کمپیوٹر کے نام میں صرف حروف، ہندسے اور ہائفن ہو سکتے ہیں۔ وہ زیادہ سے زیادہ 63 حروف کا ہو سکتا ہے اور ہائفن سے شروع یا ختم نہیں ہو سکتا۔

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = چارجنگ کی حد فی الحال لگی ہے، لیکن کمپیوٹر بند ہونے پر ختم ہو جائے گی: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = پوری چارجنگ فی الحال چالو ہے، لیکن اگلی بار کمپیوٹر شروع ہونے پر حد واپس آ جائے گی: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = دوسرے کمپیوٹر اب بھی جڑ سکتے ہیں، کیونکہ Trindle شیئرنگ بند نہیں کر سکا: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = یہ پلگ ان آئی ڈی نہیں ہے۔

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = ایسا کوئی شخص نہیں۔

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = اس شخص کا کوئی فولڈر نہیں۔

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle اجازت یافتہ پلگ انز کی فہرست محفوظ نہیں کر سکا: { $why }۔

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = ایسا کوئی پلگ ان نہیں۔

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = کچھ اپڈیٹ نہیں ہوا۔ Trindle یہ معلوم نہیں کر سکا کہ اس کا اپنا کون سا ورژن انسٹال ہے، اس لیے وہ اپنی اپڈیٹ کو الگ نہیں رکھ سکا۔

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = اپڈیٹس انسٹال ہو رہی ہیں

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = صفائی ہو رہی ہے

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle انسٹال ہو رہا ہے

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = یہ پیکیج کا نام نہیں ہے۔

# Red words when the package to remove is not installed.
admin-package-not-installed = وہ پیکیج انسٹال نہیں ہے۔

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = یہ ایسا آلہ نہیں جسے fwupd جانتا ہو۔

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }؛ { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle نہیں جان سکتا کہ Tailscale کون چاہتا ہے۔

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = بلاکنگ وقت پر شروع نہیں ہوئی۔

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = بلاکنگ پوری طرح بند نہیں ہوئی: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle { $path } کو واپس نہیں رکھ سکا: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = یہ سائٹ کا نام نہیں ہے۔

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = ایسا کوئی سرور نہیں۔

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } کو جو کچھ درکار تھا وہ ملنے سے پہلے رک گیا، اس لیے کام مکمل نہیں ہوا: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } ناکام ہو گیا۔

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-file-not-removed = { $path } ہٹائی نہیں جا سکی: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = کچھ نہیں بدلا، کیونکہ Trindle { $path } کو پڑھ نہیں سکا: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle چیزوں کو پہلے جیسا بھی نہیں کر سکا: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = یہ پرانے ورژن رکھنے کی درست مدت نہیں ہے۔

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = پرانے ورژن نہیں ملے۔

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = جہاں پرانے ورژنز کا فولڈر ہونا چاہیے وہاں ایک فائل ہے۔

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = پتا نہیں کس نے کہا۔

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = یہ صرف pkexec سے چلتا ہے۔

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = یہ کمپیوٹر کا نام نہیں ہے۔

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = یہ مشترکہ فولڈر کا نام نہیں ہے۔

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = یہ کمپیوٹر کا پتہ نہیں ہے۔

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = یہ مشترکہ فولڈر کا راستہ نہیں ہے۔

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = یہاں صرف آپ کے اپنے مشترکہ فولڈر بند کیے جا سکتے ہیں۔

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = وہاں کوئی مشترکہ فولڈر کھلا نہیں ہے۔

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Trindle کے ترتیب دینے کے بعد کسی نے { $path } کو بدل دیا ہے، اس لیے وہاں کچھ جڑا ہوا نہیں ہے۔

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } ویسا نہیں جیسا Trindle نے چھوڑا تھا۔

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = بہت زیادہ مشترکہ فولڈر کھلے ہیں۔

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } انسٹال نہیں ہے۔ یہ Debian کے { $package } پیکیج کے ساتھ آتا ہے۔

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = لاگ ان پڑھا نہیں جا سکا۔

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle یہ نہیں پڑھ سکا کہ اس کمپیوٹر پر کیا ماؤنٹ ہے: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = کوئی پروگرام اب بھی اس فولڈر میں کچھ استعمال کر رہا ہے۔ پروگرام بند کریں، پھر دوبارہ کوشش کریں۔

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = وہ کمپیوٹر جواب نہیں دے رہا۔ لاگ آؤٹ کرنے پر مشترکہ فولڈر بند ہو جائے گا۔

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = یہ صارف کی شناخت نہیں ہے۔

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare گروپ نہیں بن سکا۔

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba حاصل کیا جا رہا ہے

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = اشتراک ترتیب دیا جا رہا ہے

# SAFETY: needs a human review before it ships
# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = اشتراک ابھی ترتیب نہیں دیا گیا۔ پہلے کوئی فولڈر شیئر کریں۔

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = یہ کمپیوٹر نیٹ ورک پر دوسروں کو اب بھی نظر آتا ہے: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = یہ اسنیپ شاٹ کا نمبر نہیں ہے۔

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = ترتیبات میں لیا گیا

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle یہ معلوم نہیں کر سکا کہ سسٹم کس ڈسک پر ہے۔

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = سسٹم اپنے الگ Btrfs سب والیوم میں نہیں ہے، اس لیے Trindle اسے اسنیپ شاٹ سے بدل نہیں سکتا۔

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle { $url } سے دستخطی کلید حاصل نہیں کر سکا۔

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = اس پتے نے دستخطی کلید واپس نہیں بھیجی۔

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = کچھ نہیں بدلا، کیونکہ Trindle { $path } کو پڑھ نہیں سکا: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = اس کی دستخطی کلید حاصل کی جا رہی ہے

# Second line of the progress while a source is added.
admin-step-adding-source = اسے شامل کیا جا رہا ہے اور اس کی فہرست پڑھی جا رہی ہے

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = ذریعہ استعمال نہیں ہو سکا۔ { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle ذریعہ استعمال نہیں کر سکا، اس لیے اسے دوبارہ ہٹا دیا۔ { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } ابھی اسے چالو یا بند نہیں کر سکتا: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle اسے { $user } کے لیے شروع نہیں کر سکا: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Trindle { $user } کو ورچوئل مشینیں استعمال کرنے کی اجازت نہیں دے سکا، جو Cowork کو چاہییں: { $why }

# SAFETY: needs a human review before it ships
# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = Trindle { $user } کو بغیر پاس ورڈ کے اسے استعمال کرنے کی اجازت نہیں دے سکا: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } انسٹال ہو گیا ہے، لیکن Trindle اسے ترتیب دینا مکمل نہیں کر سکا: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = انسٹال ہو رہا ہے

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = یہ تیار شدہ ذرائع میں سے نہیں ہے۔

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } اس قسم کے کمپیوٹر ({ $arch }) کے لیے نہیں بنا۔

# Last line of the progress while a program is installed.
admin-step-setting-up = ترتیب دیا جا رہا ہے

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = ہر پتہ سادہ ویب پتہ ہونا چاہیے جو https:// سے شروع ہو۔

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = سوئٹ اور اجزا سادہ الفاظ ہونے چاہییں۔

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = یہ پیکیج کا نام نہیں ہے۔

# First line of the progress while a source and its program are taken away.
admin-step-removing = ہٹایا جا رہا ہے

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = فہرستیں دوبارہ پڑھی جا رہی ہیں

# SAFETY: needs a human review before it ships
# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = یہاں صرف وہی ذرائع ہٹائے جا سکتے ہیں جو Trindle نے شامل کیے۔

# SAFETY: needs a human review before it ships
# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = ذریعہ اب بھی موجود ہے: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = اسنیپ شاٹ { $n } ڈسک پر موجود نہیں ہے۔

# SAFETY: needs a human review before it ships
# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = کچھ نہیں بدلا، کیونکہ Trindle اسنیپ شاٹ تیار نہیں کر سکا۔ { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle موجودہ سسٹم کو الگ نہیں رکھ سکا: { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = موجودہ سسٹم کا نام اب { $aside } ہے، { $sub } نہیں، اور جب تک اس کا نام دوبارہ { $sub } نہیں ہوگا کمپیوٹر شروع نہیں ہوگا: { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle اسنیپ شاٹس منتقل نہیں کر سکا: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }؛ { $second }

# SAFETY: needs a human review before it ships
# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle اسنیپ شاٹ کو اس کی جگہ پر نہیں رکھ سکا: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle الگ رکھے گئے سسٹموں کی فہرست اپڈیٹ نہیں کر سکا: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
# SAFETY: needs a human review before it ships
admin-restore-not-removed = { $names } ہٹائے نہیں جا سکے

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Trindle نے انہیں ہٹا دیا کیونکہ اب ان کی کسی کو ضرورت نہیں تھی: { $list }۔

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = تمام اپ ڈیٹس انسٹال نہیں ہوئیں۔ یہ روکی گئی ہیں: { $list }۔

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } اس لیے بند ہے کہ اس کا ڈرائیور ({ $driver }, DKMS) ابھی نئے Linux کرنل کے لیے نہیں بنے گا۔

admin-driver-off = ڈرائیور ({ $driver }, DKMS) ابھی نئے Linux کرنل کے لیے نہیں بنے گا۔ جن خصوصیات کو اس کی ضرورت ہے وہ بند ہیں۔

# Said once after the drivers that are off.
admin-driver-off-rest = کرنل اور اپ ڈیٹ کا باقی حصہ انسٹال ہو گیا۔

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = ورچوئل کیمرا

admin-language-in-use = { $code } وہ زبان ہے جس میں یہ کمپیوٹر شروع ہوتا ہے، اس لیے اسے ہٹایا نہیں جا سکتا۔



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = اس کمپیوٹر میں بند کرنے کے لیے split-lock کی سست روی نہیں ہے۔

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle کو وہ ڈرائیو نہیں ملی، یا وہ سسٹم ڈرائیو ہے۔

# The drive could not be closed where it was open.
admin-games-drive-busy = ڈرائیو استعمال میں ہے۔ اس پر کھلی کوئی بھی فائل یا پروگرام بند کریں، پھر دوبارہ کوشش کریں: { $why }

# Another software change has the read-only system open.
admin-system-busy = سافٹ ویئر میں کوئی اور تبدیلی ہو رہی ہے۔ اس کے ختم ہونے پر دوبارہ کوشش کریں۔

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = کچھ نہیں بدلا، کیونکہ Trindle سسٹم کو کھول نہیں سکا: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle سسٹم کو دوبارہ بند نہیں کر سکا۔ اگلی بار شروع ہونے پر یہ بند ہو جائے گا: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle /usr/local کو قابلِ تحریر نہیں رکھ سکا: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = اسے صرف root چلا سکتا ہے، اور وہ بھی pkexec کے ذریعے نہیں۔

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = اس کے لیے Btrfs درکار ہے۔


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = اسنیپ شاٹ اپنی جگہ پر ہے، اور کمپیوٹر اسی سے شروع ہوگا۔ Trindle پرانے سسٹم کی صفائی مکمل نہیں کر سکا۔ وہ کمپیوٹر کے اگلی بار شروع ہونے پر دوبارہ کوشش کرے گا: { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = زیادہ سے زیادہ { $most } پورٹ کھولے جا سکتے ہیں۔

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle کھلے پورٹوں کی فہرست محفوظ نہیں کر سکا: { $why }۔

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = فائر وال اب بھی چل رہی ہے: { $why }

# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = یہ پلگ ان اب ان پورٹس کے علاوہ دوسرے نیٹ ورک پورٹس مانگتا ہے جن کی آپ نے اجازت دی تھی۔ انہیں کھولنے کے لیے دوبارہ اجازت دیں۔

# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = پروگرام صرف اسی شخص کے لیے شامل کیا جا سکتا ہے جو اسے مانگے۔

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = ایک لمحہ پہلے ایک اور بحالی نے یہی نام استعمال کیا۔ چند سیکنڈ رکیں، پھر دوبارہ کوشش کریں۔

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } کو منقطع نہیں کیا جا سکا: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 شیئر کیے گئے فولڈر نے جواب نہیں دیا، اس لیے وہ ابھی جڑا ہوا ہے۔ کمپیوٹر بند ہونے پر وہ الگ ہو جائے گا۔
       *[other] { $n } شیئر کیے گئے فولڈروں نے جواب نہیں دیا، اس لیے وہ ابھی جڑے ہوئے ہیں۔ کمپیوٹر بند ہونے پر وہ الگ ہو جائیں گے۔
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = سواپ فائل { $path } بند نہیں ہو سکی: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = سواپ فائل { $path } دوبارہ چالو نہیں ہو سکی: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } سسٹم کی سواپ فائلوں میں شامل نہیں۔

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = Trindle سواپ فائل { $path } استعمال نہیں کرے گا، کیونکہ دوسرے { $why } کو بدل یا پڑھ سکتے ہیں۔

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = سواپ فائل کی نئی جگہ نہیں ملی، اس لیے ہائبرنیشن سے جاگنا شاید نہ چلے۔

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = ری سٹارٹ کے بعد ہائبرنیشن دوبارہ کام کرے گا۔
