# machine translated (Bengali), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = এই কম্পিউটার সম্পর্কে

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = কম্পিউটারের নাম নেটওয়ার্কের নাম পরিচিতি সিস্টেম সংস্করণ মেমরি প্রসেসর ডিস্ক গ্রাফিক্স পর্দা মডেল কার্নেল চলমান প্রোগ্রাম ব্যস্ত টাস্ক ম্যানেজার ড্রাইভ জায়গা hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space

# Text behind the info button of the About page.
settings-about-help = নেটওয়ার্কের অন্য কম্পিউটার এই কম্পিউটারকে যে নামে দেখে, এটি কী দিয়ে তৈরি এবং চালু হতে কতক্ষণ লাগে, তা এখানে আছে। এটি এখন কী করছে এবং কোন প্রোগ্রাম কতটা মেমরি ও প্রসেসর ব্যবহার করছে তা "কাজকর্ম"-এ, আর ড্রাইভ ও তাদের জায়গা "ড্রাইভ"-এ আছে; প্রতিটি আলাদা উইন্ডো, "আরও বিস্তারিত"-এর তীর দিয়ে খোলে।

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
settings-about-name-bad = শুধু অক্ষর, সংখ্যা ও হাইফেন চলবে; হাইফেন দিয়ে শুরু বা শেষ করা যাবে না।

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = পাসওয়ার্ডের অপেক্ষায়...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = কম্পিউটারের নাম বদলানো যায়নি।

# Words under the computer name box after renaming worked.
settings-about-name-done = ইতিমধ্যে খোলা প্রোগ্রামগুলো পরের লগইনের পরে নতুন নাম দেখবে।

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = কম্পিউটারের নাম

# Tip of the save button beside the computer name box.
settings-about-rename = এই কম্পিউটারের নাম বদলান

# Bold line of the computer name setting.
settings-about-name = কম্পিউটারের নাম

# Quiet line under the computer name setting.
settings-about-name-sub = অক্ষর, সংখ্যা ও হাইফেন।

# Heading of the card with the computer's name.
settings-about-name-group = নাম

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = নেটওয়ার্কে এবং টার্মিনালের প্রম্পটে এই কম্পিউটার যেভাবে দেখা যায়। অক্ষর, সংখ্যা ও হাইফেন চলবে; হাইফেন দিয়ে শুরু বা শেষ করা যাবে না। বদলাতে আপনার পাসওয়ার্ড চাওয়া হবে।

# Heading of the card about the computer's hardware.
settings-about-hardware = কম্পিউটার

# Text behind the info button of the hardware card.
settings-about-hardware-about = নির্মাতা ও মডেল, প্রসেসর ও এটি একসঙ্গে কয়টি থ্রেড চালায়, গ্রাফিক্স, এবং ব্যবহৃত প্রতিটি পর্দার আকার ও রিফ্রেশ হার।

# Quiet line under the maker and model.
settings-about-hw-computer = কম্পিউটার

# Quiet line under the processor.
settings-about-hw-processor = প্রসেসর

# Quiet line under the graphics.
settings-about-hw-graphics = ছবি আঁকে

# Quiet line under the screens.
settings-about-hw-screen = পর্দা

# Heading of the card about the software.
settings-about-software = সফটওয়্যার

# Text behind the info button of the software card.
settings-about-software-about = Trindle হলো Debian-এর ওপরে বসানো একটি ডেস্কটপ, যা নিচের প্রোগ্রামগুলো দেখাশোনা করে। কার্নেল হলো যে অংশ হার্ডওয়্যারের সঙ্গে কথা বলে, আর Trindle-এর নিজস্ব কম্পোজিটর Wayland-এ ডেস্কটপ আঁকে।

# Quiet line under the Debian version.
settings-about-sw-debian = নিচে চলে

# Quiet line under the kernel version.
settings-about-sw-kernel = কার্নেল

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle-এর নিজস্ব কম্পোজিটর

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Wayland-এ ডেস্কটপ আঁকে

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n }টি অ্যাপ
       *[other] { $n }টি অ্যাপ
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = ইনস্টল করা, রিংয়ে তৈরি

# Heading of the card with the ways to Activity and Drives.
settings-about-more = আরও বিস্তারিত

# Text behind the info button of the More detail card.
settings-about-more-about = এটি এখন কী করছে এবং কোন প্রোগ্রাম কতটা মেমরি ও প্রসেসর ব্যবহার করছে তা কাজকর্মে আছে। প্রতিটি ড্রাইভ, তা কতটা ভরা ও কেমন আছে, এবং ইউএসবি স্টিক ড্রাইভে আছে।

# Bold line of the setting that opens Activity.
settings-about-activity = কী চলছে দেখুন

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = যেসব প্রোগ্রাম মেমরি ও প্রসেসর ব্যবহার করছে।

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = কাজকর্ম

# Bold line of the setting that opens Drives.
settings-about-drives = ড্রাইভ ও জায়গা

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = প্রতিটি ড্রাইভ, কতটা ভরা ও কেমন আছে।

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ড্রাইভ

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = চালু হওয়ার সময়গুলো ডাউনলোডসে সংরক্ষণ করা যায়নি।

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = ডাউনলোডসে, { $file } নামে

# Label of the word button that saves every start-up time.
settings-about-times-save = সংরক্ষণ করুন

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = চালু হওয়ার সব সময় ডাউনলোডসে সংরক্ষণ করুন

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = প্রতিটি চালু হওয়া, রেখে দিতে

# Quiet line under that setting.
settings-about-times-keep-sub = স্প্রেডশিট হিসেবে ডাউনলোডসে সংরক্ষিত।

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = প্রতিটি স্তম্ভ একটি চালু হওয়া, নতুনটি ডানদিকে।

# Heading of the card with the last ten start-up times.
settings-about-times = শেষ দশবার চালু হওয়া

# Text behind the info button of the start-up times card.
settings-about-times-about = পাওয়ার বোতাম থেকে লগইন পর্দা পর্যন্ত প্রতিবারের সময় মাপা হয়, ডিস্কের পাসওয়ার্ড লিখতে যে সময় গেছে তা বাদ দিয়ে। নতুনটি ওপরে।

# Heading of the memory tile.
settings-about-memory = মেমরি

# Heading of the disk tile.
settings-about-disk = ডিস্ক


# Heading of the card with the switch for looking for updates.
settings-about-updates = হালনাগাদ

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle লগ ইনের কিছু পরে এবং প্রতি কয়েক ঘণ্টায় অ্যাপ, প্লাগইন, ফার্মওয়্যার ও নিজের নতুন সংস্করণ খোঁজে, আর Debian-এর নিরাপত্তা সংশোধন প্রতিদিন আসে। বন্ধ করলে Trindle শুধু অ্যাপসে "হালনাগাদ খুঁজুন" চাপলে, বা রিংয়ের হালনাগাদ বোতাম চাপলে খোঁজে। Debian-এর নিরাপত্তা সংশোধনের নিজস্ব সুইচ আছে; বন্ধ থাকলে সেগুলো আর নিজে থেকে বসে না এবং জানা ত্রুটি আপনি হালনাগাদ না করা পর্যন্ত খোলা থাকে। যেকোনো সুইচ বদলালে প্রশাসকের পাসওয়ার্ড চাওয়া হয়, যাতে কম্পিউটারের নিজের সেবাগুলোও এই পছন্দ দেখতে পায়।

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = হালনাগাদ খুঁজুন

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = পাসওয়ার্ডের অপেক্ষায়...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = বন্ধ থাকলে আপনি না বললে কিছুই খোঁজা হয় না।



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = নিরাপত্তা হালনাগাদ

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = নিরাপত্তা সংশোধন প্রতিদিন নিজে থেকে বসে।

# Warning line under the Security updates switch when it is off.
settings-about-security-off = নিরাপত্তা সংশোধন আর নিজে থেকে বসে না।

