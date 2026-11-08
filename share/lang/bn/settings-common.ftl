
# machine translated (Bengali), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = সংরক্ষিত

# Said beside a button that worked, when nothing more is known.
settings-common-done = হয়েছে

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = হয়নি

# Tip of the information button at the top of every Settings page.
settings-common-about-page = এই পাতা সম্পর্কে

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = { $name } সম্পর্কে

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } আগে থেকেই আছে

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] বাকিগুলোর জন্য একই করুন (আরও { $n }টি)
       *[other] বাকিগুলোর জন্য একই করুন (আরও { $n }টি)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = বাকিগুলোর জন্য একই করুন

# Tip of the button that keeps the one already there.
settings-common-skip = বাদ দিন: আগে থেকে থাকাটি রাখুন

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = বদলে দিন

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = আগে থেকে থাকাটিকে যেটি আনা হচ্ছে সেটি দিয়ে বদলে দিন

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = যেটি আনা হচ্ছে

# Heading of the line that shows the one already there.
settings-common-already-line = যেটি আগে থেকে আছে

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = আগে থেকে থাকাটি বদলে দেবেন?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] { $left } সেকেন্ডে আগের অবস্থায় ফিরবে।
       *[other] { $left } সেকেন্ডে আগের অবস্থায় ফিরবে।
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] { $left } সেকেন্ডে আগের অবস্থায় ফিরবে। { $more }
       *[other] { $left } সেকেন্ডে আগের অবস্থায় ফিরবে। { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = এখনই ফিরে যান

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = রাখুন

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } পাওয়া যায় না

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = এটি ইনস্টল করুন: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = ওয়েব পাতা খোলা যায়নি: কোনো ওয়েব ব্রাউজার চালু করা যায়নি।

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } খোলা যায়নি।

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = এটি সংরক্ষণ করা যায়নি।

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = হয়নি: সিস্টেমের মেসেজ বাসে পৌঁছানো যায়নি। কম্পিউটার রিস্টার্ট করে আবার চেষ্টা করুন।

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = হয়নি: এই লগইনের মেসেজ বাসে পৌঁছানো যায়নি। লগ আউট করে আবার ঢুকে চেষ্টা করুন।

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = বাছাইয়ের জানালা খোলা যায়নি। লগ আউট করে আবার ঢুকে চেষ্টা করুন।

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = ডিস্ক ভরা।

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = ডিস্কে এখন লেখা যাচ্ছে না।

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = সিস্টেমের যে অংশ এটি করে তা চলছে না।

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = সাড়া দিতে অনেক সময় লেগেছে। আবার চেষ্টা করুন।

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = পাসওয়ার্ড দেওয়া হয়নি।

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = এই অ্যাকাউন্টের এটি করার অনুমতি নেই।

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = এই কম্পিউটারে এর দরকারি কিছু নেই।

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } ও { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = স্ক্রিনের কিনারায় ক্লিক করলে রিং খোলে।

# Word drawn in the middle of that picture.
settings-common-ring-name = রিং

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = ল্যাপটপের ফার্মওয়্যারে পৌঁছানো যায়নি: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = ল্যাপটপের ফার্মওয়্যার চার্জের সীমা নেয়নি।

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = এই কম্পিউটারে আগেভাগে চার্জ থামানো যায় না।

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path } ঠিক করা যায়নি: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = ভার্চুয়াল মেশিনে Trindle পরখ করা

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle { $machine }-এ চলছে। সেখানে এটি ভালো চলে, কিন্তু ভার্চুয়াল মেশিন আসল কম্পিউটারের মতো দ্রুত বা মসৃণ হয় না, তাই এর গতি আসল যন্ত্রে যাচাই করুন।

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle { $machine }-এ ৩ডি গ্রাফিক্স ছাড়া চলছে, তাই গ্রাফিক্স চিপ সাধারণত যা আঁকে তা প্রসেসরকে আঁকতে হচ্ছে। জানালা, ভিডিও ও ওয়েব পাতা আসল কম্পিউটারের চেয়ে ধীর লাগবে, এবং Trindle সাহায্যের জন্য অ্যানিমেশন বাদ দেয়। যে কম্পিউটারে এটি চলে সে পারলে, ভার্চুয়াল মেশিনের ডিসপ্লে সেটিংসে ৩ডি অ্যাক্সেলারেশন চালু করলে অনেক তফাত হয়। আসল কম্পিউটারে ইনস্টল করলে এর কিছুই প্রযোজ্য নয়।

