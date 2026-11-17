# machine translated (Bengali), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = এই কম্পিউটার সম্পর্কে

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = কম্পিউটারের নাম নেটওয়ার্কের নাম পরিচিতি সিস্টেম সংস্করণ মেমরি প্রসেসর ডিস্ক গ্রাফিক্স পর্দা মডেল কার্নেল চলমান প্রোগ্রাম ব্যস্ত টাস্ক ম্যানেজার ড্রাইভ জায়গা hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space

# Text behind the info button of the About page.
settings-about-help = এই পাতায় আছে নেটওয়ার্কের অন্য কম্পিউটার এই কম্পিউটারকে যে নামে দেখে, এটি কী দিয়ে তৈরি এবং চালু হতে কতক্ষণ লাগে। "কাজকর্ম" দেখায় কম্পিউটার এখন কী করছে এবং কোন প্রোগ্রাম কতটা মেমরি ও প্রসেসর ব্যবহার করছে। "ড্রাইভ" দেখায় আপনার ড্রাইভ আর তাদের জায়গা। প্রতিটি আলাদা উইন্ডোতে খোলে। খুলতে "আরও বিস্তারিত"-এর তীর ব্যবহার করুন।

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = অজানা

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores }টি থ্রেড
       *[other] { $model }, { $cores }টি থ্রেড
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel গ্রাফিক্স

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD গ্রাফিক্স

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA গ্রাফিক্স

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = ভার্চুয়াল মেশিনের গ্রাফিক্স

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = গ্রাফিক্স

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }, { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } ব্যবহৃত

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } ফাঁকা

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = মোট { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, চালু আছে { $on } ধরে।

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, চালু আছে { $on } ধরে। ডিস্ক প্রায় ভরে গেছে।

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } সেকেন্ড

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } মিনিট

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } মিনিট
       *[other] { $n } মিনিট
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } ঘণ্টা
       *[other] { $n } ঘণ্টা
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } দিন
       *[other] { $n } দিন
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = শুধু অক্ষর, সংখ্যা ও হাইফেন ব্যবহার করুন। হাইফেন দিয়ে শুরু বা শেষ করবেন না।

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = আপনার পাসওয়ার্ডের অপেক্ষায়…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = কম্পিউটারের নাম বদলানো যায়নি।

# Words under the computer name box after renaming worked.
settings-about-name-done = ইতিমধ্যে খোলা প্রোগ্রামগুলো আপনি পরের বার লগ ইন করার পর নতুন নাম ব্যবহার করবে।

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = কম্পিউটারের নাম

# Tip of the save button beside the computer name box.
settings-about-rename = এই কম্পিউটারের নাম বদলান

# Bold line of the computer name setting.
settings-about-name = কম্পিউটারের নাম

# Heading of the card with the computer's name.
settings-about-name-group = নাম

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = নেটওয়ার্কে এবং টার্মিনালের প্রম্পটে এটিই এই কম্পিউটারের নাম। অক্ষর, সংখ্যা ও হাইফেন ব্যবহার করুন, তবে হাইফেন দিয়ে শুরু বা শেষ করবেন না। বদলাতে আপনাকে পাসওয়ার্ড দিতে হবে।

# Heading of the card about the computer's hardware.
settings-about-hardware = কম্পিউটার

# Text behind the info button of the hardware card.
settings-about-hardware-about = এখানে কম্পিউটারের নির্মাতা ও মডেল, প্রসেসর এবং গ্রাফিক্স দেখা যায়। আরও দেখা যায় প্রসেসর একসঙ্গে কয়টি থ্রেড চালাতে পারে, এবং ব্যবহৃত প্রতিটি পর্দার আকার ও রিফ্রেশ হার।

# Heading of the card about the software.
settings-about-software = সফটওয়্যার

# Text behind the info button of the software card.
settings-about-software-about = Trindle হলো Debian-এর ওপরে চলা একটি ডেস্কটপ, যা নিচের প্রোগ্রামগুলো দেখাশোনা করে। কার্নেল হলো যে অংশ হার্ডওয়্যারের সঙ্গে কাজ করে। Trindle-এর নিজস্ব কম্পোজিটর Wayland ব্যবহার করে ডেস্কটপ আঁকে।

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle-এর নিজস্ব কম্পোজিটর

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n }টি অ্যাপ
       *[other] { $n }টি অ্যাপ
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = আরও বিস্তারিত

# Text behind the info button of the More detail card.
settings-about-more-about = "কাজকর্ম" দেখায় কম্পিউটার এখন কী করছে এবং কোন প্রোগ্রাম কতটা মেমরি ও প্রসেসর ব্যবহার করছে। "ড্রাইভ" দেখায় প্রতিটি ড্রাইভ, তা কতটা ভরা ও কেমন আছে, এবং ইউএসবি স্টিক।

# Bold line of the setting that opens Activity.
settings-about-activity = কী চলছে দেখুন

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = কাজকর্ম

# Bold line of the setting that opens Drives.
settings-about-drives = ড্রাইভ ও জায়গা

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ড্রাইভ

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle চালু হওয়ার সময়গুলো ডাউনলোডসে সংরক্ষণ করতে পারেনি।

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = তারিখ ও সময়,সেকেন্ড

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle চালু হওয়ার সময়.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = ডাউনলোডসে, { $file } নামে

# Label of the word button that saves every start-up time.
settings-about-times-save = সংরক্ষণ করুন

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = চালু হওয়ার সব সময় ডাউনলোডসে সংরক্ষণ করুন

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = প্রতিবার চালু হওয়ার সময় সংরক্ষণ করুন

# Quiet line under that setting.
settings-about-times-keep-sub = স্প্রেডশিট হিসেবে ডাউনলোডসে সংরক্ষিত।

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = প্রতিটি স্তম্ভ একবার চালু হওয়া। নতুনটি ডানদিকে।

# Heading of the card with the last ten start-up times.
settings-about-times = শেষ দশবার চালু হওয়া

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle পাওয়ার বোতাম থেকে লগইন পর্দা পর্যন্ত প্রতিবার চালু হওয়ার সময় মাপে। ডিস্কের পাসওয়ার্ড লিখতে যে সময় গেছে তা ধরা হয় না। নতুনটি ওপরে।

# Heading of the memory tile.
settings-about-memory = মেমরি

# Heading of the disk tile.
settings-about-disk = ডিস্ক

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = সমস্যার রিপোর্ট

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = রিপোর্ট Buildmorphic-এ যায়, যে কমিউনিটি Trindle বানাতে সাহায্য করে। শুধু একটি ইমেল ঠিকানা দিয়েই যোগ দিতে পারেন। যে সমস্যা আপনি প্রথম জানান, নিজে ঠিক করতে না পারলেও, তার জন্য ক্রেডিট ও লেভেল পাবেন। একটি রিপোর্টে শুধু একটি নির্দিষ্ট তালিকার তথ্য থাকে, আর পাঠান চাপার আগে কিছুই পাঠানো হয় না। ব্যবহারকারীর নাম বা GitHub নাম ঐচ্ছিক, এবং আপনার ব্যবহারকারীর নাম কখনো ইমেল ঠিকানা হয় না।

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = কিছু ভুল হলে রিপোর্ট পাঠানোর প্রস্তাব দিন

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic ব্যবহারকারীর নাম

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub নাম

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = কমিউনিটি সম্পর্কে

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] বন্ধ। অপেক্ষায় থাকা { $n }টি রিপোর্ট মুছে ফেলা হয়েছে।
       *[other] বন্ধ। অপেক্ষায় থাকা { $n }টি রিপোর্ট মুছে ফেলা হয়েছে।
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] বন্ধ। অপেক্ষায় থাকা { $n }টি রিপোর্ট মোছা যায়নি।
       *[other] বন্ধ। অপেক্ষায় থাকা { $n }টি রিপোর্ট মোছা যায়নি।
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = হালনাগাদ

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle লগ ইনের কিছু পরে, তারপর প্রতি কয়েক ঘণ্টায় অ্যাপ, প্লাগইন, ফার্মওয়্যার ও নিজের নতুন সংস্করণ আছে কি না দেখে। এটি বন্ধ করলে Trindle শুধু তখনই দেখে যখন আপনি অ্যাপসে "হালনাগাদ খুঁজুন" চাপেন, বা রিংয়ের হালনাগাদ বোতাম চাপেন। Debian-এর নিরাপত্তা সংশোধন প্রতিদিন ইনস্টল হয় এবং এর নিজস্ব সুইচ আছে। সেটি বন্ধ করলে সংশোধনগুলো আর নিজে থেকে ইনস্টল হয় না, আর জানা নিরাপত্তা ত্রুটি আপনি হালনাগাদ না করা পর্যন্ত সারানো হয় না। যেকোনো সুইচ বদলাতে প্রশাসকের পাসওয়ার্ড লাগে, যাতে পুরো কম্পিউটার আপনার পছন্দ মেনে চলে।

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = হালনাগাদ খুঁজুন

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = আপনার পাসওয়ার্ডের অপেক্ষায়…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = নিরাপত্তা হালনাগাদ

# Warning line under the Security updates switch when it is off.
settings-about-security-off = নিরাপত্তা সংশোধন আর নিজে থেকে বসে না।

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = বন্ধ, কিন্তু Reporting অ্যাড-অনের অপেক্ষায় থাকা রিপোর্ট মোছা হয়নি।
