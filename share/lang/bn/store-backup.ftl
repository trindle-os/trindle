# machine translated (Bengali), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = কিছুই ফিরিয়ে আনা হয়নি, কারণ { $why }।

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = ব্যাকআপের একটি অংশ ফিরিয়ে আনা হয়নি, কারণ { $why }।

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = ব্যাকআপে এমন লিংক বা অন্য কিছু আছে যা যে ফোল্ডারে যাবে তার বাইরে পৌঁছাবে

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = ব্যাকআপের একটি অংশ যেখানে যাবে সেখানে এই কম্পিউটারের কিছু আগে থেকেই আছে

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = ব্যাকআপ খোলার মতো কোনো জায়গা নেই

# The reason when the backup file cannot be read.
store-because-unreadable = ব্যাকআপ ফাইলটি পড়া যায়নি

# The reason when the file is not a Trindle backup.
store-because-not-trindle = ওই ফাইলটি Trindle-এর ব্যাকআপ নয়

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = ব্যাকআপটি নষ্ট এবং এর তালিকা পড়া যায়নি

# The reason when the file is not a Flat backup.
store-because-not-flat = ওই ফাইলটি Flat ব্যাকআপ বলে মনে হচ্ছে না

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = ওই ফাইলে Flat অ্যাপের তালিকা নেই

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = এটি এমন নতুন Flat দিয়ে বানানো যা Trindle পড়তে পারে না

# The reason when the file holds no apps.
store-because-no-apps = ওই ফাইলে কোনো অ্যাপ নেই

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = ব্যাকআপ পড়া যায়নি: { $why }

# The reason when the program gave none.
store-because-none = কোনো কারণ জানায়নি

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = এই কম্পিউটার Trindle-কে সেখানে লিখতে দেয়নি

# The reason when the disk is full.
store-because-space = এই কম্পিউটারে আর জায়গা নেই

# The reason when a part of the computer is read-only.
store-because-read-only = এই কম্পিউটারের ওই অংশ শুধু পড়ার জন্য

# The reason when the person's disk quota is used up.
store-because-quota = এই অ্যাকাউন্টের জন্য অনুমোদিত জায়গা শেষ হয়ে গেছে

# The reason when a file or program is missing.
store-because-missing = প্রয়োজনীয় একটি ফাইল বা প্রোগ্রাম নেই

# The reason when no reason could be read from the program's last line.
store-because-failed = ব্যর্থ হয়েছে

# The reason when the backup has no name to be written under.
store-because-no-name = এর কোনো নাম নেই

# The reason when a backup's settings for an app reach outside the app's own folder.
# SAFETY: needs a human review before it ships
store-because-outside-own = ব্যাকআপে এর সেটিংস নিজের ফোল্ডারের বাইরে পৌঁছায়

# The reason when something is too big to be read whole.
store-because-too-big = এটি পুরোটা পড়ার জন্য অনেক বড়

# The reason when an app's settings in the backup do not match their checksum.
# SAFETY: needs a human review before it ships
store-because-checksum = ব্যাকআপে এর সেটিংস চেকসামের সঙ্গে মেলে না

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = ফিরিয়ে আনা হয়েছে, শুধু আপনার অ্যাকাউন্টের ছবি বাদে। { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } আপনার অ্যাকাউন্টের ছবিও ফিরিয়ে আনা হয়নি। { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = pkexec চালু করা যায়নি: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = ব্যাকআপ থেকে এটি পড়া যায়নি: { $why }।

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = ব্যাকআপ বানানো হয়নি, কারণ { $why }।

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = এখনও কিছু নেই

# Beside the size of a part of the backup: about how big it is.
store-backup-about = প্রায় { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindle-এর সেটিংস ও AppImage ফিরিয়ে আনা হচ্ছে...

# Step shown while the settings and data are put back.
store-backup-step-data = সেটিংস ও ডেটা ফিরিয়ে আনা হচ্ছে...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Flathub থেকে { $count }টি অ্যাপ ইনস্টল হচ্ছে... এতে কিছুক্ষণ লাগতে পারে।
       *[other] Flathub থেকে { $count }টি অ্যাপ ইনস্টল হচ্ছে... এতে কিছুক্ষণ লাগতে পারে।
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = সেটিংস ও ডেটা ফিরিয়ে আনা হয়েছে, কিন্তু এই অ্যাপগুলো ইনস্টল হয়নি, কারণ Flatpak সেগুলো প্রত্যাখ্যান করেছে: { $failed }।

# The same, when other apps were also left out for another reason.
store-not-installed-both = সেটিংস ও ডেটা ফিরিয়ে আনা হয়েছে, কিন্তু এই অ্যাপগুলো ইনস্টল হয়নি, কারণ Flatpak সেগুলো প্রত্যাখ্যান করেছে: { $failed }। এগুলোও ইনস্টল হয়নি: { $refused }।

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = সেটিংস ও ডেটা ফিরিয়ে আনা হয়েছে, কিন্তু এই অ্যাপগুলো ইনস্টল হয়নি: { $refused }।

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] ১টি অ্যাপ বাদ দেওয়া হয়েছে: এর নাম সঠিক ছিল না।
       *[other] { $n }টি অ্যাপ বাদ দেওয়া হয়েছে: এদের নাম সঠিক ছিল না।
    }

# Said beside an app whose source is not secure: the source's name is given.
# SAFETY: needs a human review before it ships
store-flat-not-secure = { $name } রিমোট এনক্রিপ্টেড নয় এবং গ্রহণ করা হয়নি, তাই ব্যবহার করা হয়নি

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } রিমোট এই কম্পিউটারে নেই এবং ব্যাকআপে এটি কোথায় তা বলা নেই

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name }-এর সেটিংস (কারণ ব্যাকআপের কপি যাচাই পাস করেনি)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (কারণ এর উৎসের নাম সঠিক নয়)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (কারণ { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (কারণ Flatpak এটি প্রত্যাখ্যান করেছে: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = সবকিছু (কারণ ব্যাকআপ খোলার মতো জায়গা নেই)

# Said among what could not be done: the settings that apply to every app.
# SAFETY: needs a human review before it ships
store-flat-every-app = প্রতিটি অ্যাপকে যা করতে দেওয়া হয়

# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name }-এর সেটিংস

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }। আগে যা ছিল তা { $path } হিসেবে রাখা হয়েছে: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } ইনস্টল হচ্ছে ({ $total }টির মধ্যে { $at })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = { $name }-এর সেটিংস ফিরিয়ে আনা হচ্ছে ({ $total }টির মধ্যে { $at })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] ১টি অ্যাপ ইনস্টল হয়েছে।
       *[other] { $n }টি অ্যাপ ইনস্টল হয়েছে।
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = { $n }টির সেটিংস ফিরিয়ে আনা হয়েছে।

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n }টি আগে থেকেই আছে, যেমন ছিল তেমন রাখা হয়েছে।
       *[other] { $n }টি আগে থেকেই আছে, যেমন ছিল তেমন রাখা হয়েছে।
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = ফিরিয়ে আনা হয়নি: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })
