# machine translated (Bengali), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count }টি অ্যাপ
       *[other] { $count }টি অ্যাপ
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count }টি আপডেট অপেক্ষায় আছে
       *[other] { $count }টি আপডেট অপেক্ষায় আছে
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub-এ বেছে নেওয়ার জন্য { $apps }; { $more }।

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = আপনার সবকিছু হালনাগাদ আছে

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }।

# Line under the title of a search that found nothing.
store-grid-none-found = “{ $text }”-এর সাথে কোনো অ্যাপ মেলেনি।

# Line under the title of an empty page of apps.
store-grid-none-here = এখানে এখনও কোনো অ্যাপ নেই।

# Advice in the empty page of a search.
store-grid-try-shorter = ছোট শব্দ লিখে দেখুন, বা বাঁ দিকের কোনো ধরন ঘেঁটে দেখুন।

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = ওপরে নাম দিয়ে খুঁজুন, বা বাঁ দিকের অন্য কোনো ধরন ঘেঁটে দেখুন।

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $apps }-এর মধ্যে { $shown }টি দেখানো হচ্ছে।

# Link under a page of apps that shows more of them.
store-grid-show-more = আরও { $next }টি দেখান

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Flathub থেকে অ্যাপের তালিকা আনা হচ্ছে। প্রথমবার এক মিনিট লাগে।

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = অ্যাপস Flathub-এ পৌঁছাতে পারেনি।

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = এই কম্পিউটার ইন্টারনেটে যুক্ত নেই। যুক্ত হওয়া মাত্র অ্যাপস নিজে থেকে আবার চেষ্টা করবে।

# In the waiting page when the computer is not connected.
store-wait-comes-from = বেছে নেওয়ার অ্যাপগুলো ইন্টারনেটে Flathub থেকে আসে।

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } এবং বাঁ দিকের অ্যাপের সেটিংস এটি ছাড়াই কাজ করে।

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } এবং বাঁ দিকের অ্যাপের সেটিংস Flathub ছাড়াই কাজ করে।

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } এবং বাঁ দিকের অ্যাপের সেটিংস ইন্টারনেট ছাড়াই কাজ করে।

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = আবার চেষ্টা করুন

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = ফলাফল

# Red words under the search box when no web browser opens a link.
store-no-browser = { $url } খোলার মতো কোনো ওয়েব ব্রাউজার নেই। আগে অ্যাপসে একটি নিন।

# Said in the card or under the title while the lists are being read.
store-looking = খোঁজা হচ্ছে…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = এটি এখন "ইনস্টল করা"-তে এবং রিংয়ে আছে

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = সরানো হয়েছে

# The word on an app's dial when installing or removing did not work.
store-done-not = করা যায়নি

# The word on an app's dial when it was installed.
store-done-installed = ইনস্টল করা হয়েছে

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak মানেনি।

# Tab of the Installed page that shows everything.
store-tab-all = সব

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak ফাইল

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb ফাইল

# Tab of the Installed page that shows plugins.
store-tab-plugins = প্লাগইন

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = এখনও কিছু ইনস্টল করা নেই। "আবিষ্কার"-এ চেষ্টা করার মতো অনেক কিছু আছে।

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] ১টি প্লাগইন ইনস্টল করা আছে
       *[other] { $n }টি প্লাগইন ইনস্টল করা আছে
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] ১টি প্রোগ্রাম ইনস্টল করা আছে
       *[other] { $n }টি প্রোগ্রাম ইনস্টল করা আছে
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n }টি জিনিস ইনস্টল করা আছে

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, Flathub থেকে
       *[other] { $start }, সব Flathub থেকে
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, একটি Flatpak ফাইল থেকে
       *[other] { $start }, সব Flatpak ফাইল থেকে
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, একটি AppImage
       *[other] { $start }, সব AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, একটি .deb ফাইল থেকে
       *[other] { $start }, সব .deb ফাইল থেকে
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = Flathub থেকে { $n }টি

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] একটি Flatpak ফাইল থেকে ১টি
       *[other] Flatpak ফাইল থেকে { $n }টি
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] ১টি AppImage
       *[other] { $n }টি AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] ১টি অ্যাপ
       *[other] { $n }টি অ্যাপ
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] একটি .deb ফাইল থেকে ১টি
       *[other] .deb ফাইল থেকে { $n }টি
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] ১টি প্লাগইন
       *[other] { $n }টি প্লাগইন
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }। { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = এটি হালনাগাদ আছে।

# End of the Installed line when everything is up to date.
store-installed-state-all = সব হালনাগাদ আছে।

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] ১টির আপডেট অপেক্ষায় আছে।
       *[other] { $n }টির আপডেট অপেক্ষায় আছে।
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $total }টি প্রোগ্রামের মধ্যে { $shown }টি “{ $find }”-এর সাথে মিলেছে।

# The same when plugins are among the things.
store-installed-match-things = { $total }টি জিনিসের মধ্যে { $shown }টি “{ $find }”-এর সাথে মিলেছে।

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = একটি AppImage যোগ করুন

# Placeholder of the typing box on the Installed page.
store-installed-search = ইনস্টল করা জিনিস খুঁজুন

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage যোগ করা যায়নি।

# In the card of the Installed page when nothing is installed.
store-installed-empty = "আবিষ্কার" থেকে, GitHub থেকে বা AppImage হিসেবে ইনস্টল করা অ্যাপ, .deb ফাইলের প্রোগ্রাম এবং প্লাগইন এখানে দেখা যায়।

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = “{ $find }”-এর সাথে ইনস্টল করা কিছুই মেলেনি।

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = সরানো দুবার জিজ্ঞাসা করে: তিন সেকেন্ডের মধ্যে আবার চাপুন।

# On an installed thing's line: its version.
store-installed-version = সংস্করণ { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = আটকে রাখা

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = আটকে রাখা; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } আপডেট হচ্ছে

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } আপডেট করুন

# Tip of the round button that shows an installed app's page.
store-installed-about = সম্পর্কে

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = সরানো হচ্ছে...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = কোন ফাইল তা জিজ্ঞাসা করার মতো সেশন বাস নেই।

# Title of the window that asks which AppImage to add.
store-add-choose-title = একটি AppImage বেছে নিন

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = ফাইল বাছাইকারীকে জিজ্ঞাসা করা যায়নি।

# Words in the Installed card while an AppImage is being added.
store-add-adding = যোগ করা হচ্ছে...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } আগেই আছে: "ইনস্টল করা" দেখুন।

# Words once an AppImage is in the ring.
store-add-in-ring = যোগ করা হয়েছে। এটি এখন আপনার তালিকায় আছে।

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = এটি এই কম্পিউটারের ব্যবহারযোগ্য Flatpak রেফারেন্স নয়।

# Words under the search box while a Flatpak reference is installed; the source is given.
# SAFETY: needs a human review before it ships
store-add-installing-ref = { $host } থেকে { $name } ইনস্টল হচ্ছে...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } ইনস্টল করা হয়েছে।

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak এটি ইনস্টল করতে চায়নি।

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } ইনস্টল করা যায়নি। { $why }

# Words in the Installed card while an AppImage replaces another.
# SAFETY: needs a human review before it ships
store-add-replacing = বদলানো হচ্ছে...

# Heading of the question when an AppImage has the name of one already there.
# SAFETY: needs a human review before it ships
store-replace-head = { $name } আগেই আছে

# Small heading over the AppImage being brought.
store-replace-new = যেটি আনা হচ্ছে

# Small heading over the AppImage already there.
store-replace-old = যেটি আগে থেকে আছে

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = বাদ দিন: আগে থেকে থাকাটি রাখুন

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = প্রতিস্থাপন করুন

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = { $name } সরাবেন?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = এর সংরক্ষিত তথ্য { $size } জায়গা নেয়।

# Under that heading when the app has saved no data.
store-flatpak-ask-none = এর কোনো সংরক্ষিত তথ্য নেই।

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = এর সংরক্ষিত তথ্য গোনা হচ্ছে...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = মুছে ফেলা তথ্য আর ফেরত আনা যায় না।

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = সরান, তথ্য রাখুন

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = সরান ও এর তথ্য মুছুন

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = শুধু এর তথ্য মুছুন

# Why Delete its data only is greyed.
store-flatpak-data-only-none = মোছার মতো কোনো সংরক্ষিত তথ্য নেই।

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = বাতিল: দুটোই রাখুন

# Said under the card or the row once only the data has been deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-gone = এর সংরক্ষিত তথ্য মুছে ফেলা হয়েছে।

# Words while only the data is being deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-going = এর সংরক্ষিত তথ্য মুছে ফেলা হচ্ছে...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = এটি শেষ হয়নি, আর কারণও জানায়নি।

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = সমস্যা কোথায় দেখান

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = GitHub-এ খুঁজুন: লিখে Enter চাপুন

# Heading of the card of matches from AppImageHub.
store-gh-hub = AppImageHub-এ ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = পুরো GitHub-এও খুঁজতে Enter চাপুন।

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub-কে জিজ্ঞাসা করা হয়নি: জিজ্ঞাসা করার মতো ইন্টারনেট নেই।

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub-কে জিজ্ঞাসা করা হচ্ছে...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub-এ আর কিছু নেই।

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] ১টি প্রজেক্টে AppImage ও Flatpak ফাইল খোঁজা হচ্ছে...
       *[other] { $n }টি প্রজেক্টে AppImage ও Flatpak ফাইল খোঁজা হচ্ছে...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub-এর কোনো মিলে এই কম্পিউটারের জন্য AppImage বা Flatpak ফাইল নেই।

# Heading of the card of matches from GitHub.
store-gh-on = GitHub-এ

# Heading of the card that holds the search box.
store-gh-search-card = GitHub-এ খুঁজুন

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = GitHub-এর অ্যাপ AppImage বা Flatpak ফাইল হিসেবে। এগুলো কেউ যাচাই করে না: শুধু বিশ্বাসযোগ্য মানুষের কাছ থেকে ইনস্টল করুন।

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = এই কম্পিউটার ইন্টারনেটে যুক্ত না থাকলে GitHub-এ খোঁজা যায় না।

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak ও AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = ফাইলগুলো দেখুন

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = ইনস্টল করুন

# Link to a project's page on GitHub.
store-gh-open = GitHub-এ খুলুন

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, রিলিজ { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = GitHub কী

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = GitHub কী: একটি জমা আছে

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = আপনার সম্ভবত এটি লাগবে না। কী ছাড়া GitHub এই কম্পিউটার থেকে ঘণ্টায় ৬০টি প্রশ্নের উত্তর দেয়; কী থাকলে ৫,০০০টির। কী শুধু এই সীমা বাড়ায়। এটি শুধু প্রকাশ্য ফাইল দেখতে ব্যবহার হয়, আর কিছুর জন্য নয়: এটি Trindle-কে আপনার অ্যাকাউন্টে ঢোকার কোনো পথ দেয় না, আর বানানোর সময় আপনি কোনো অনুমতি টিক দেন না। এটি এই কম্পিউটারে আপনার কীরিংয়ে রাখা হয় এবং শুধু GitHub-কে পাঠানো হয়।

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = GitHub-এ একটি কী বানান, যেকোনো নাম দিন, সব অনুমতি যেমন আছে তেমন রাখুন, Generate token চাপুন, তারপর যা দেখায় তা এখানে পেস্ট করুন।

# Red words under the GitHub key's box when the key could not be removed.
# SAFETY: needs a human review before it ships
store-gh-key-not-removed = এটি সরানো যায়নি: { $why }

# Red words under the GitHub key's box when the key could not be kept.
# SAFETY: needs a human review before it ships
store-gh-key-not-kept = এটি রাখা যায়নি: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
# SAFETY: needs a human review before it ships
store-gh-key-keep-empty = রাখতে খালি রাখুন

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = কী পেস্ট করুন

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = কী সরান

# Link in the GitHub key's box to the page where a key is made.
# SAFETY: needs a human review before it ships
store-gh-key-make = GitHub-এ একটি কী বানান

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = কী রাখুন

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = এর সবচেয়ে নতুন রিলিজের কিছুই এখানে ইনস্টল হয় না।

# Word on the file's dial and under a GitHub file when it was added.
store-added = যোগ করা হয়েছে

# Line under the title of the Move your apps page.
store-bk-line = আপনার অ্যাপ ও সেটিংস, যেকোনো জায়গায় নিয়ে যাওয়ার মতো একটি ফাইলে।

# Heading of the card that makes a backup.
store-bk-card = ব্যাকআপ নিন

# Tip of the round button while the backup is being made.
store-bk-making-tip = ব্যাকআপ তৈরি হচ্ছে

# Main button that makes the backup.
store-bk-make = ব্যাকআপ তৈরি করুন

# Tip of the round button that shows the backup in Folders.
store-bk-show = ফোল্ডারে দেখান

# Name of the switch for the list of apps in a backup.
store-bk-apps = অ্যাপ

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] আপনার ১টি Flatpak অ্যাপের তালিকা।
       *[other] আপনার { $n }টি Flatpak অ্যাপের তালিকা।
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } অন্য কম্পিউটার এগুলো Flathub থেকে নতুন করে ইনস্টল করে, তাই ফাইল ছোট থাকে।

# Name of the switch for the apps' own data in a backup.
store-bk-data = অ্যাপের নিজস্ব তথ্য

# Under the Apps' own data switch.
store-bk-data-sub = প্রতিটি অ্যাপের সেটিংস ও সংরক্ষিত কাজ, ক্যাশ ছাড়া।

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = পুরোটা রাখা হয়, কারণ এগুলো একক ফাইল।

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle সেটিংস

# Under the Trindle settings switch.
store-bk-settings-sub = রিং, ওয়ালপেপার পছন্দ, রং, কীবোর্ড, পাওয়ার, ডিফল্ট অ্যাপ ও আপনার ছবি।

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } এখন { $size }।

# Name of the row that says where the backup goes.
store-bk-where = কোথায় যাবে

# Under that row: the folder and the file's name.
store-bk-where-sub = আপনার হোম ফোল্ডারে, { $name } নামে

# Heading of the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-card = ব্যাকআপ ফিরিয়ে আনুন

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = একটি ব্যাকআপ ফাইল বেছে নিন

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = একটি Trindle ব্যাকআপ, বা { $flat } দিয়ে তৈরি একটি।

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = এর অ্যাপগুলো ইনস্টল হয় এবং এর তথ্য ও সেটিংস ফিরিয়ে আনা হয়। আপনার নথি ও ছবি এতে নেই: সেগুলো নিজে কপি করে নিন।

# Said while a backup is being made.
store-bk-making = ব্যাকআপ নেওয়া হচ্ছে... কয়েক মিনিট লাগতে পারে।

# Said when a backup is made: the file and its size.
store-bk-made = এটি { $path } ({ $size })। সাথে নিয়ে যেতে একটি USB স্টিকে বা ক্লাউড স্টোরেজে কপি করুন।

# Red words when no connection to the file chooser exists.
store-bk-no-bus = কোনো ব্যাকআপ বাছা হয়নি, কারণ Trindle-এর ফাইল বাছাইকারীর সাথে সংযোগ নেই।

# Title of the window that asks which backup to use.
store-bk-choose-title = একটি ব্যাকআপ বেছে নিন

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle ও Flat ব্যাকআপ

# Red words when the file chooser gave no answer.
store-bk-no-answer = কোনো ব্যাকআপ বাছা হয়নি, কারণ ফাইল বাছাইকারী সাড়া দেয়নি।

# Said while a backup file is read.
store-bk-reading = ব্যাকআপ পড়া হচ্ছে...

# Said when a backup was put back.
store-bk-logout = সব সেটিংস দেখতে লগ আউট করে আবার লগ ইন করুন।

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } সব সেটিংস দেখতে লগ আউট করে আবার লগ ইন করুন।

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] ১টি অ্যাপ, তার মধ্যে { $with }টির সেটিংস আছে। শুরু হচ্ছে...
       *[other] { $apps }টি অ্যাপ, তার মধ্যে { $with }টির সেটিংস আছে। শুরু হচ্ছে...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] নির্মাতাদের নিজস্ব কোনো উৎস এখনও যোগ করা হয়নি।
        [one] নির্মাতার ১টি উৎস যোগ করা হয়েছে, Debian-এর আপডেটের সাথে হালনাগাদ রাখা হয়।
       *[other] নির্মাতাদের { $n }টি উৎস যোগ করা হয়েছে, Debian-এর আপডেটের সাথে হালনাগাদ রাখা হয়।
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }। যোগ করা হয়েছে এবং হালনাগাদ রাখা হয়।

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }। উৎস যোগ করা হয়েছে, কিন্তু প্রোগ্রাম ইনস্টল করা নেই।

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }। অন্যভাবে ইনস্টল করা।

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = এটি ও এর উৎস সরান

# Link to a maker's page, naming the program.
store-src-open = { $name } খুলুন

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Tailscale-এ সাইন ইন করুন

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = { $name } যোগ করুন

# Heading of the card of makers' sources that can be added.
store-src-ready = যোগ করার জন্য তৈরি

# Name of the field for the address of a source.
store-src-address = ঠিকানা

# Under the address field.
store-src-address-sub = রিপোজিটরির ঠিকানা, নির্মাতার ইনস্টল নির্দেশনা থেকে।

# Name of the field for the suite of a source.
store-src-suite = স্যুট

# Under the suite field.
store-src-suite-sub = সাধারণত trixie বা stable।

# Name of the field for the components of a source.
store-src-components = কম্পোনেন্ট

# Under the components field.
store-src-components-sub = সাধারণত main।

# Name of the field for a source's signing key.
# SAFETY: needs a human review before it ships
store-src-key = সাইনিং কী

# Under the signing key field.
# SAFETY: needs a human review before it ships
store-src-key-sub = এর সাইনিং কীর ঠিকানা।

# Name of the field for the program to install from a source.
store-src-program = ইনস্টল করার প্রোগ্রাম

# Under the program field.
store-src-program-sub = এর প্যাকেজের নাম, এখনই ইনস্টল করতে চাইলে।

# Placeholder of the program field.
store-src-program-hint = ঐচ্ছিক

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = এই উৎস যোগ করুন

# Heading of the card that adds another source.
# SAFETY: needs a human review before it ships
store-src-another = আরেকটি উৎস যোগ করুন

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = একটি উৎস পুরো কম্পিউটারের আপডেট ইনস্টল করতে পারে, তাই শুধু যাদের বিশ্বাস করেন সেই নির্মাতাদের যোগ করুন। প্রতিটির জন্য অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড লাগে।

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = এই উৎস সরান

# Heading of the card of sources already on the computer.
store-src-already = এই কম্পিউটারে আগেই আছে

# Red words when the addresses of a new source are not secure.
# SAFETY: needs a human review before it ships
store-src-bad-address = দুটি ঠিকানাই https:// দিয়ে শুরু হতে হবে

# Red words when the suite or components are not plain words.
store-src-bad-words = স্যুট ও কম্পোনেন্ট সাধারণ শব্দ হতে হবে।

# Red words when the program is not a package name.
store-src-bad-package = এটি প্যাকেজের নাম নয়।

# Word on a source's dial while its job starts.
store-src-starting = শুরু হচ্ছে

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Tailscale-এর সাইন-ইন পাতা খোলা হচ্ছে...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = যে পাতা খুলেছে সেখানে সাইন ইন করুন; এটি নিজে থেকেই শেষ হবে।

# Said under Tailscale's row when it is connected.
store-src-connected = যুক্ত

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = অ্যাপের লিংকটি এনক্রিপ্ট করা নয়।

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = এনক্রিপ্ট করা নয়

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = এই ঠিকানা এনক্রিপ্ট করা নয়। পথে কেউ যা পাঠানো হচ্ছে তা দেখতে পারে, এবং ব্যাকআপটি বদলে অন্য একটি দিয়ে দিতে পারে।

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = আমি বুঝেছি এবং এই ঝুঁকি মেনে নিচ্ছি

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = ফিরিয়ে আনুন

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = বাতিল করুন

# machine translated (Bengali)

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = এর সংরক্ষিত ডেটা গণনা করা যায়নি।

store-hub-warning = AppImageHub শুধু প্রকল্পের তালিকা দেয়: কেউ সেগুলো পর্যালোচনা বা স্ক্যান করে না, আর সেগুলো স্বাক্ষরিত হতে হবে এমনও নয়। Trindle প্রতিটি AppImage একটি ব্যক্তিগত হোম ফোল্ডারওয়ালা বাক্সে চালায়: শুরুতে সেটি ইন্টারনেট ও শব্দ ব্যবহার করতে পারে, আপনার কোনো ফাইল দেখে না।

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = একটি AppImage কোথা থেকে এসেছে তা একটি ইঙ্গিত, যাচাই নয়।

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = GitHub-এ: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = GitHub-এ: { $repo }, { $stars }টি স্টার

# Where an AppImage comes from: a website, by its name.
store-from-site = GitHub-এ নয়: একটি ওয়েবসাইট থেকে, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = উৎস অজানা

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = এর ওয়েবসাইট খুলুন

# When an app was last updated.
store-age-updated = সর্বশেষ আপডেট: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = সর্বশেষ আপডেট: { $date }। ছয় মাসের বেশি আপডেট হয়নি, তাই অমীমাংসিত নিরাপত্তা ত্রুটি থাকতে পারে।

# When the source does not say when an app was last updated.
store-age-updated-unknown = সর্বশেষ আপডেট: জানা নেই

# When an app's project was made.
store-age-created = তৈরি { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = তৈরি: জানা নেই

# A date the source does not give, on a line of Facts.
store-age-not-known = জানা নেই

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = একটি যোগ করার আগে

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] AppImageHub-এ { $count }টি AppImage তালিকাভুক্ত আছে।
       *[other] AppImageHub-এ { $count }টি AppImage তালিকাভুক্ত আছে।
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = AppImageHub-এর তালিকা আনা হচ্ছে…

# The same when the list could not be fetched.
store-browse-failed = AppImageHub-এর তালিকা আনা যায়নি।

# The same with no internet and no copy kept.
store-browse-offline = এই কম্পিউটার ইন্টারনেটে যুক্ত না থাকলে AppImageHub-এর তালিকা আনা যায় না।

# In the list when the kind chosen has none.
store-browse-none = AppImageHub-এ এই ধরনের কিছু নেই।

# Under the rows: how many are shown of how many.
store-browse-showing = { $n }-এর মধ্যে { $shown }টি দেখানো হচ্ছে।

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Trindle-এর অ্যাপে যোগ করুন

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = বিবরণ ও ফাইল দেখুন



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = হালনাগাদ খোঁজা বন্ধ আছে

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = OpenAI তার ডাউনলোডে স্বাক্ষর করলে ChatGPT ফিরে আসবে।



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Flathub ও AppImageHub-এ বেছে নেওয়ার জন্য { $apps }; { $more }।

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = AppImageHub-এ বেছে নেওয়ার জন্য { $apps }।

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = দুটোই

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage যাচাই করা হয় না। কেউ এগুলো পর্যালোচনা বা স্ক্যান করে না।

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle প্রতিটি AppImage একটি আলাদা বাক্সে চালায়, যার হোম ফোল্ডার ব্যক্তিগত: শুরুতে এটি ইন্টারনেট ও শব্দ ব্যবহার করতে পারে, কিন্তু আপনার কোনো ফাইল দেখতে পায় না।

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = আর দেখাবেন না

# Button on that card that goes on with the install.
store-appimage-warn-ok = ঠিক আছে

# Button on that card that gives the install up.
store-appimage-warn-cancel = বাতিল করুন

