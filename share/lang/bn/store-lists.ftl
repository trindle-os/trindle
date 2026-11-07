# machine translated (Bengali), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = একটি Flatpak ফাইল

# On an installed thing's line: it came from a .deb file.
store-from-deb = একটি .deb ফাইল

# On an installed thing's line: it is a plugin.
store-from-plugin = প্লাগইন

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = রিলিজ ক্যান্ডিডেট

# Tip of the round button on an app's tile while it is being removed.
# SAFETY: needs a human review before it ships
store-list-removing = { $name } সরানো হচ্ছে

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = { $name } ইনস্টল হচ্ছে

# Tip of the round button on an app's tile that opens the app.
store-list-open = { $name } খুলুন

# Tip of the round button on an app's tile that installs the app.
store-list-install = { $name } ইনস্টল করুন

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = ইনস্টল করা আছে

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = দেখান

# How long is left when it is a minute or less.
store-left-minute = প্রায় এক মিনিট বাকি

# How long is left, in minutes.
store-left-minutes = প্রায় { $n } মিনিট বাকি

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] প্রায় ১ ঘণ্টা বাকি
       *[other] প্রায় { $n } ঘণ্টা বাকি
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = HaGeZi-র Threat Intelligence তালিকার সঙ্গে মিলিয়ে দেখা হয়েছে

# Where a download came from: the site and the page that led to it.
# SAFETY: needs a human review before it ships
store-origin-both = { $site } থেকে নামানো, { $referrer } থেকে পৌঁছানো।

# Where a download came from: the site.
# SAFETY: needs a human review before it ships
store-origin-site = { $site } থেকে নামানো।

# Where a download came from: the page that led to it.
# SAFETY: needs a human review before it ships
store-origin-referrer = { $referrer } থেকে পৌঁছানো।

# Said when nothing tells where a download came from.
# SAFETY: needs a human review before it ships
store-origin-unknown = এটি কোথা থেকে এসেছে Trindle বলতে পারে না।

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } HaGeZi-র Threat Intelligence তালিকায় আছে, যা ক্ষতিকর ডাউনলোড ও প্রতারণার জন্য পরিচিত সাইটের তালিকা। কোথা থেকে এসেছে সে বিষয়ে নিশ্চিত হলেই কেবল এটি ইনস্টল করুন।

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } সাইটটি HaGeZi-র Threat Intelligence তালিকায় নেই, কিন্তু তালিকাটি সম্পূর্ণ নয়, তাই এটি নিরাপদ বলে প্রমাণ হয় না।

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } সাইটটি যাচাই করা যায়নি, কারণ HaGeZi-র Threat Intelligence তালিকা আনা যায়নি।

# Name shown for a review whose writer gave none.
store-review-someone = কেউ একজন

# Red words when the keyring could not be reached.
store-key-unreached = কীরিং পাওয়া যায়নি: { $why }।

# Red words when the key kept in the keyring is not text.
store-key-not-text = রাখা কী-টি লেখা নয়।

# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path } সরানো যায়নি: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = { $path } পড়া যায়নি: { $why }
