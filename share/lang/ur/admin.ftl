# machine translated (Urdu), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = یہ لاگ ان نام درست نہیں ہے۔

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = یہ لاگ ان نام بہت لمبا ہے۔

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name } نام کا کوئی صارف نہیں ہے۔

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } سسٹم کا اکاؤنٹ ہے اور اسے چھوڑ دیا گیا ہے۔

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

# SAFETY: needs a human review before it ships
# Red words in Settings when a new password has a line break in it.
admin-password-line-break = پاس ورڈ میں نئی سطر نہیں ہو سکتی۔

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } پہلے سے موجود ہے۔

# SAFETY: needs a human review before it ships
# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } واحد منتظم ہے، اس لیے وہ منتظم ہی رہے گا۔

# SAFETY: needs a human review before it ships
# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = آپ اپنا ہی اکاؤنٹ حذف نہیں کر سکتے۔

# SAFETY: needs a human review before it ships
# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = { $name } حذف ہو گیا، لیکن اس کی تصویر لاگ ان اسکرین کے لیے ابھی رکھی ہوئی ہے: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = یہ تصویر بہت بڑی ہے۔

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = یہ Trindle کی بنائی ہوئی تصویر نہیں ہے۔

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = تصاویر کا فولڈر لاگ ان اسکرین کے لیے کھولا نہیں جا سکا: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = یہ زبان کا نام نہیں ہے۔

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Debian کی زبانوں کی فہرست پڑھی نہیں جا سکی: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } ایسی زبان نہیں جسے Debian جانتا ہو۔

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = کمپیوٹر کے نام میں حروف، ہندسے اور ہائفن ہو سکتے ہیں، زیادہ سے زیادہ 63، اور ہائفن سے شروع یا ختم نہیں ہو سکتا۔

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = حد فی الحال لگی ہے، لیکن کمپیوٹر بند ہونے پر ختم ہو جائے گی: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = چارجنگ فی الحال پوری ہے، لیکن اگلی بار شروع ہونے پر حد واپس آ جائے گی: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = دوسرے کمپیوٹر اب بھی جڑ سکتے ہیں، کیونکہ سوئچ بند نہیں ہو سکا: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = یہ پلگ ان کی شناخت نہیں ہے۔

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = یہ شخص معلوم نہیں ہے۔

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = اس شخص کا فولڈر معلوم نہیں ہے۔

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = جن پلگ انز کو اجازت ملی ان کی فہرست محفوظ نہیں ہو سکی: { $why }۔

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = یہ پلگ ان نہیں ملا۔

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = کچھ اپڈیٹ نہیں ہوا: اس کمپیوٹر پر Trindle کا کون سا ورژن ہے یہ معلوم نہیں ہو سکا، اس لیے Trindle کی اپنی اپڈیٹ کو الگ نہیں رکھا جا سکا۔

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = اپڈیٹس انسٹال ہو رہی ہیں

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = صفائی ہو رہی ہے

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle انسٹال ہو رہا ہے

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = یہ پیکیج کا نام نہیں ہے۔

# Red words when the package to remove is not installed.
admin-package-not-installed = کچھ نہیں ہوا: یہ انسٹال ہی نہیں ہے۔

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = یہ ایسا آلہ نہیں جسے fwupd جانتا ہو۔

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }؛ { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = معلوم نہیں ہو سکا کہ Tailscale کس نے مانگا۔

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = بلاکنگ وقت پر شروع نہیں ہوئی۔

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = بلاکنگ پوری طرح بند نہیں ہوئی: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = { $path } واپس نہیں رکھی جا سکی: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = یہ سائٹ کا نام نہیں ہے۔

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = ایسا کوئی سرور نہیں۔

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } سب کچھ ملنے سے پہلے رک گیا، اس لیے اسے مکمل نہیں کہا جا سکتا: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } ناکام ہو گیا۔

# SAFETY: needs a human review before it ships
# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } ہٹائی نہیں جا سکی: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = کچھ نہیں بدلا، کیونکہ { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } چیزوں کو پہلے جیسا کرنا بھی نہیں ہو سکا: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = یہ ایسی مدت نہیں جس تک پرانے ورژن رکھے جا سکیں۔

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = اسنیپ شاٹس کا فولڈر موجود نہیں ہے۔

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = جہاں اسنیپ شاٹس جاتے ہیں وہاں فولڈر کی جگہ کوئی اور چیز ہے۔

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = معلوم نہیں ہو سکا کہ کس نے مانگا۔

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = یہ صرف pkexec کے ذریعے چلایا جاتا ہے۔

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
admin-share-folder-changed = { $path } ویسا نہیں جیسا Trindle نے چھوڑا تھا، اس لیے وہاں کچھ ماؤنٹ نہیں کیا گیا۔

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } ویسا نہیں جیسا Trindle نے چھوڑا تھا۔

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = بہت زیادہ مشترکہ فولڈر کھلے ہیں۔

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } انسٹال نہیں ہے۔ یہ Debian کے { $package } میں آتا ہے۔

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = لاگ ان پڑھا نہیں جا سکا۔

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = اس کمپیوٹر پر جو ماؤنٹ ہے وہ پڑھا نہیں جا سکا: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = کسی پروگرام نے اس پر اب بھی کچھ کھول رکھا ہے۔ پہلے اسے بند کریں۔

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = وہ کمپیوٹر جواب نہیں دے رہا۔ لاگ آؤٹ کرنے پر یہ بند ہو جائے گا۔

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = یہ صارف کی شناخت نہیں ہے۔

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare گروپ بنایا نہیں جا سکا۔

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba حاصل ہو رہا ہے

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = اشتراک ترتیب دیا جا رہا ہے

# SAFETY: needs a human review before it ships
# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = اشتراک ابھی ترتیب نہیں دیا گیا۔ پہلے کوئی فولڈر شیئر کریں۔

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = یہ کمپیوٹر نیٹ ورک پر دوسروں کو اب بھی نظر آ رہا ہے: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = یہ اسنیپ شاٹ کا نمبر نہیں ہے۔

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = ترتیبات میں لیا گیا

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = معلوم نہیں ہو سکا کہ سسٹم کس ڈسک پر ہے۔

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = سسٹم اپنے الگ Btrfs سب والیوم میں نہیں ہے، اس لیے اسے اسنیپ شاٹ سے بدلا نہیں جا سکتا۔

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = { $url } سے دستخطی کلید حاصل نہیں ہو سکی۔

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = اس پتے سے جو ملا وہ دستخطی کلید نہیں ہے۔

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = کچھ نہیں بدلا، کیونکہ { $path } پڑھی نہیں جا سکتی: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = اس کی دستخطی کلید حاصل ہو رہی ہے

# Second line of the progress while a source is added.
admin-step-adding-source = اسے شامل کیا جا رہا ہے اور اس کی فہرست پڑھی جا رہی ہے

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = ذریعہ استعمال نہیں ہو سکا۔ { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = ذریعہ استعمال نہیں ہو سکا، اس لیے اسے دوبارہ ہٹا دیا گیا۔ { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } ابھی اسے چالو یا بند نہیں کر سکتا: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = یہ { $user } کے لیے شروع نہیں ہو سکا: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = { $user } کو ورچوئل مشینیں استعمال کرنے کی اجازت نہیں دی جا سکی، جو Cowork کو چاہییں: { $why }

# SAFETY: needs a human review before it ships
# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = { $user } کو بغیر پاس ورڈ کے اسے استعمال کرنے کی اجازت نہیں دی جا سکی: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } انسٹال ہو گیا ہے، لیکن اس کی ترتیب مکمل نہیں ہوئی: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = انسٹال ہو رہا ہے

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = یہ تیار شدہ ذرائع میں سے نہیں ہے۔

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } اس قسم کے کمپیوٹر ({ $arch }) کے لیے نہیں بنا۔

# Last line of the progress while a program is installed.
admin-step-setting-up = ترتیب دیا جا رہا ہے

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = پتے https:// سے شروع ہونے چاہییں اور ان میں اور کچھ نہیں ہونا چاہیے۔

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
admin-source-not-ours = یہاں صرف وہی ذرائع ہٹائے جاتے ہیں جو Trindle نے شامل کیے۔

# SAFETY: needs a human review before it ships
# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = ذریعہ اب بھی موجود ہے: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = اسنیپ شاٹ { $n } ڈسک پر موجود نہیں ہے۔

# SAFETY: needs a human review before it ships
# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = کچھ نہیں بدلا: اسنیپ شاٹ تیار نہیں ہو سکا۔ { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = موجودہ سسٹم کو الگ نہیں رکھا جا سکا: { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = موجودہ سسٹم کا نام اب { $aside } ہے، { $sub } نہیں، اور جب تک اس کا نام دوبارہ { $sub } نہیں ہوگا کمپیوٹر شروع نہیں ہوگا: { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = اسنیپ شاٹس منتقل نہیں ہو سکے: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }؛ { $second }

# SAFETY: needs a human review before it ships
# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = اسنیپ شاٹ اپنی جگہ پر نہیں رکھا جا سکا: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = الگ رکھے گئے سسٹموں کی فہرست اپڈیٹ نہیں ہو سکی: { $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = ہٹائے نہیں جا سکے: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = ان کی اب کسی کو ضرورت نہیں تھی، اس لیے ہٹا دیے گئے: { $list }۔

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = تمام اپ ڈیٹس انسٹال نہیں ہوئیں۔ روکے گئے: { $list }۔

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } بند ہے: اس کا ڈرائیور ({ $driver }, DKMS) ابھی نئے کرنل پر نہیں بنتا۔

admin-driver-off = ایک ڈرائیور ({ $driver }, DKMS) ابھی نئے کرنل پر نہیں بنتا، اس لیے اس کی سہولت بند ہے۔

# Said once after the drivers that are off.
admin-driver-off-rest = کرنل اور اپ ڈیٹ کا باقی حصہ انسٹال ہو گیا۔

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = ورچوئل کیمرا

admin-language-in-use = { $code } وہ زبان ہے جس میں یہ کمپیوٹر شروع ہوتا ہے، اس لیے یہ رہے گی۔



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = اس کمپیوٹر میں بند کرنے کے لیے split-lock کی سست روی نہیں ہے۔

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = وہ ڈرائیو نہیں ملی، یا اس میں سسٹم موجود ہے۔

# The drive could not be closed where it was open.
admin-games-drive-busy = ڈرائیو استعمال میں ہے۔ اس پر جو کچھ کھلا ہے اسے بند کریں، پھر دوبارہ کوشش کریں: { $why }

# Another software change has the read-only system open.
admin-system-busy = سافٹ ویئر میں کوئی اور تبدیلی ہو رہی ہے۔ اس کے ختم ہونے پر دوبارہ کوشش کریں۔

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = کچھ نہیں بدلا: سسٹم کھولا نہیں جا سکا: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = سسٹم کو دوبارہ بند نہیں کیا جا سکا۔ اگلی بار شروع ہونے پر بند ہو جائے گا: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = /usr/local کو قابلِ تحریر نہیں رکھا جا سکا: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = اسے صرف root چلا سکتا ہے، اور وہ بھی pkexec کے ذریعے نہیں۔

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Btrfs درکار ہے۔


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = اسنیپ شاٹ اپنی جگہ پر ہے اور کمپیوٹر اسی سے شروع ہوگا۔ پرانے سسٹم کی صفائی مکمل نہیں ہوئی اور اگلی بار شروع ہونے پر دوبارہ کی جائے گی: { $why }

