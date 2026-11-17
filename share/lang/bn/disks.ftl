# machine translated (Bengali), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = ড্রাইভ

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = এই পাতা সম্পর্কে

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = এখানে এই কম্পিউটারের ভেতরের বা এতে লাগানো প্রতিটি ড্রাইভ দেখানো হয়, পার্টিশন অনুযায়ী ভাগ করা একটি বার হিসেবে। প্রতিটি পার্টিশনের প্রস্থ তার আকার দেখায়, আর রঙের গাঢ়তা দেখায় সেটি কতটা ভরা, তাই ভরে যাওয়া পার্টিশন সহজেই চোখে পড়ে।

# Second paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-2 = ছবির নিচে প্রতিটি ড্রাইভের একটি কার্ড আছে। আপনি একটি পার্টিশন খুলতে, মাউন্ট বা আনমাউন্ট করতে, ড্রাইভ নিরাপদে খুলে নিতে, অথবা বন্ধ করতে পারেন। পার্টিশনের পেন্সিল দিয়ে আপনি তার নাম বদলাতে, ফোল্ডার বাছতে, চালুর সময় মাউন্ট করাতে, আকার বদলাতে, পরীক্ষা, মেরামত, ফরম্যাট বা মুছে ফেলতে পারেন। খালি জায়গার প্লাস নতুন পার্টিশন বানায়।

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = যে ড্রাইভ থেকে সিস্টেম চলছে সেটি আপনি দেখতে পান, কিন্তু কিছু মুছে বা সরিয়ে দেয় এমন কোনো পরিবর্তন তাতে করা যায় না। যেকোনো ড্রাইভে প্রতিটি পরিবর্তনের জন্য প্রতিবার প্রশাসকের পাসওয়ার্ড চাওয়া হয়। চোখ চিহ্নটি একটি ড্রাইভ বা পার্টিশনকে Folders ও রিং থেকে লুকিয়ে রাখে। সেটি এখানে ধূসর হয়ে থাকে, আর তালাবদ্ধ হয় না।

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = আপনার ড্রাইভ

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = এখন ড্রাইভগুলো পড়া যাচ্ছে না।

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = আবার চেষ্টা করুন

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = আবার দেখুন

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = কোনো ড্রাইভ পাওয়া যায়নি। একটি লাগালে এখানে দেখা যাবে।

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = ড্রাইভ সেবা

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = সিস্টেম বাস

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } পাওয়া যাচ্ছে না। এটি ইনস্টল করতে চালান: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = ড্রাইভগুলো দেখা হচ্ছে…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } নষ্ট হয়ে যাচ্ছে। যে ফাইল রাখতে চান তা এখনই অন্য ড্রাইভে কপি করে নিন।

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } একবার দেখা দরকার: { $troubles }।

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] আপনার ড্রাইভ ও তাদের জায়গা।
        [one] আপনার ড্রাইভ ও তাদের জায়গা, সঙ্গে একটি ড্রাইভ লাগানো।
       *[other] আপনার ড্রাইভ ও তাদের জায়গা, সঙ্গে { $plugged }টি ড্রাইভ লাগানো।
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] এই কম্পিউটার প্রায় ভরে গেছে, মাত্র { $free } খালি। শিগগির কিছু ফাইল মুছুন বা সরান।
        [one] এই কম্পিউটার প্রায় ভরে গেছে, মাত্র { $free } খালি। একটি ড্রাইভ লাগানো আছে। শিগগির কিছু ফাইল মুছুন বা সরান।
       *[other] এই কম্পিউটার প্রায় ভরে গেছে, মাত্র { $free } খালি। { $plugged }টি ড্রাইভ লাগানো আছে। শিগগির কিছু ফাইল মুছুন বা সরান।
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] এই কম্পিউটার ভরে আসছে, { $free } খালি আছে।
        [one] এই কম্পিউটার ভরে আসছে, { $free } খালি আছে। একটি ড্রাইভ লাগানো আছে।
       *[other] এই কম্পিউটার ভরে আসছে, { $free } খালি আছে। { $plugged }টি ড্রাইভ লাগানো আছে।
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] এই কম্পিউটারে যথেষ্ট জায়গা আছে, { $free } খালি।
        [one] এই কম্পিউটারে যথেষ্ট জায়গা আছে, { $free } খালি। একটি ড্রাইভ লাগানো আছে।
       *[other] এই কম্পিউটারে যথেষ্ট জায়গা আছে, { $free } খালি। { $plugged }টি ড্রাইভ লাগানো আছে।
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = এই কম্পিউটার

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = আপনার ফাইল

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] সিস্টেম যে ডিস্কে আছে তাতে এখনও { $free } খালি।
       *[files] আপনার নিজের ফাইলের জন্য এখনও { $free } খালি।
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } ভরা, { $whole }-এর মধ্যে { $free } খালি

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = { $whole }-এর মধ্যে

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = ড্রাইভগুলো এখনও কোনো স্বাস্থ্যের রিডিং পাঠায়নি

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = সুস্থ, ড্রাইভের নিজের পরীক্ষায়

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] সুস্থ, ড্রাইভের নিজের পরীক্ষা অনুযায়ী
       *[other] সব { $n }টি ড্রাইভ নিজেদের পরীক্ষায় পাস করেছে
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n }টি একবার দেখা দরকার: তার কার্ড দেখুন
       *[other] { $n }টি একবার দেখা দরকার: তাদের কার্ড দেখুন
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] ১টি ড্রাইভ, মোট { $total }। { $health }।
       *[other] { $n }টি ড্রাইভ, মোট { $total }। { $health }।
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } খালি

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = মাউন্ট করা নেই

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = খালি জায়গা

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = ডিস্ক বা কার্ড নেই

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = এখনও কিছু নেই

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB বা খুলে নেওয়া যায়

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, এই কম্পিউটারের ভেতরে

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB বা খুলে নেওয়া যায়, লুকানো

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, এই কম্পিউটারের ভেতরে, লুকানো

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } ভরা, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }।

# Name given to a part that has no label and no device name.
disks-part-name = পার্টিশন

# Name given to a drive that gives no maker or model.
disks-drive-name = ড্রাইভ

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = সোয়াপ

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = এনক্রিপ্টেড, খোলা

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = এনক্রিপ্টেড, তালাবদ্ধ

# Kind of a part that holds other parts.
disks-kind-container = পার্টিশন ধরে রাখে

# Kind of a part that has no filesystem on it.
disks-kind-none = কোনো ফাইল সিস্টেম নেই

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, যে পার্টিশন থেকে কম্পিউটার চালু হয়
        [legacy] { $kind }, যে পার্টিশন থেকে পুরনো কম্পিউটার চালু হয়
       *[loader] { $kind }, বুট লোডারের নিজস্ব পার্টিশন
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = যেকোনো কম্পিউটার

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = শুধু Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = পুরনো ডিভাইস

# Trouble a drive reports about its spare space.
disks-trouble-spare = বাড়তি জায়গা ফুরিয়ে আসছে

# Trouble a drive reports about its heat.
disks-trouble-temperature = অতিরিক্ত গরম

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = ক্ষয়ে যাচ্ছে

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = শুধু-পড়া হয়ে গেছে

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = তার মেমরি ব্যাকআপ নষ্ট হয়েছে

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = এর একটি অংশ শুধু-পড়া হয়ে গেছে

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = প্রত্যাশিত আয়ুর চেয়ে বেশি ক্ষয়ে গেছে

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n }টি নষ্ট জায়গা
       *[other] { $n }টি নষ্ট জায়গা
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] { $hours } ঘণ্টা ব্যবহৃত
       *[other] { $hours } ঘণ্টা ব্যবহৃত
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = নষ্ট হচ্ছে। যা রাখতে চান তা এখনই এটি থেকে কপি করে নিন

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = নজর দরকার: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = সুস্থ

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = সাড়া দিচ্ছে না

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = ড্রাইভের বাক্স যুক্ত আছে, কিন্তু ভেতরের ড্রাইভ সাড়া দিচ্ছে না। এটি খুলে নিন, দশ সেকেন্ড অপেক্ষা করুন, তারপর নিজস্ব তার দিয়ে একটি নীল বা USB-C পোর্টে লাগান। এরপরও একই থাকলে ভেতরের ড্রাইভ আলগা হয়ে বা নষ্ট হয়ে থাকতে পারে।

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = নিরাপদে খুলুন

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = বন্ধ করুন

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = পুরো ড্রাইভ মুছে ফেলুন

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] এই ড্রাইভ দেখান
       *[partition] এই পার্টিশন দেখান
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] এই ড্রাইভ লুকান। { $lock }
       *[partition] এই পার্টিশন লুকান। { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Folders ও রিং থেকে লুকানো। { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = সিস্টেম এটি থেকে চলছে, তাই এখানে কিছুই মোছা যাবে না।

# Note on the card of a drive that has no partition table.
disks-no-table = এই ড্রাইভে কোনো পার্টিশন টেবিল নেই, তাই পুরোটাই খালি।

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = মাউন্ট করা নেই

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = চালুর সময় { $dir }-এ মাউন্ট হয়

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = { $dir }-এ দেখা যায়

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = { $dir }-এ

# Tip of the round button that opens the folder a part is mounted in.
disks-open = খুলুন

# Tip of the round button that mounts a part.
disks-mount = মাউন্ট করুন

# Tip of the round button that unmounts a part.
disks-unmount = আনমাউন্ট করুন

# Tip of the round button on a part that opens the box of changes.
disks-change-it = পরিবর্তন করুন

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = নাম

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = নাম (ঐচ্ছিক)

# Tip of the button that saves a part's new name.
disks-rename-tip = নাম বদলান

# Note under the Name row of a part.
disks-name-note = এখানে ও অন্য কম্পিউটারে এটি যে নামে ডাকা হয়।

# Placeholder of the typing box for a part's folder.
disks-folder-name = ফোল্ডারের নাম

# Tip of the button that saves a part's folder.
disks-use-folder = এই ফোল্ডার ব্যবহার করুন

# Label of the row where a part's folder is chosen.
disks-folder = ফোল্ডার

# Note under the Folder row of a part.
disks-folder-note = এর ফাইলগুলো যেখানে দেখা যায়। কম্পিউটারকে বাছতে দিতে ফাঁকা রাখুন।

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = চালুর সময় মাউন্ট করুন

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = কম্পিউটার চালু হলেই এটি মাউন্ট করা হয়। এটি না থাকলেও কম্পিউটার ঠিকই চালু হয়।

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = GB-তে আকার

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = { $least } থেকে { $most }।

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = { $least } থেকে { $most }। কত ছোট করা যায় দেখতে মাউন্ট করুন।

# Tip of the button that resizes a part.
# SAFETY: needs a human review before it ships
disks-resize-tip = আকার বদলান

# Label of the row where a part is resized.
disks-size = আকার

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = এতে { $words }-এর সবকিছু মুছে যাবে। এটি আর ফেরানো যাবে না।

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = { $drive }-এর { $name } { $from } থেকে { $to } করতে আবার ক্লিক করুন

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = নতুন পার্টিশনের জায়গা

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = GB-তে জায়গা

# Tip of the button that makes room under a part that is in use.
disks-room-tip = জায়গা করুন

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = { $drive }-এর { $name } থেকে { $size } নিতে আবার ক্লিক করুন। Trindle আগে একটি স্ন্যাপশট নেয়।

# Note under the row that makes room.
disks-room-note = Trindle একটি স্ন্যাপশট নেয়, তারপর ব্যবহারে থাকা অবস্থাতেই পার্টিশনটি ছোট করে। যে জায়গা খালি হয় তা ফাঁকা রাখা হয়।

# Tip of the button that checks a part for faults.
disks-check-tip = ত্রুটি পরীক্ষা করুন

# Tip of the button that repairs a part.
# SAFETY: needs a human review before it ships
disks-repair-tip = মেরামত করুন

# Label of the row where a part is checked and repaired.
disks-faults = ত্রুটি

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = { $drive }-এর { $name } মেরামত করতে আবার ক্লিক করুন

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = ধরন

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = ফরম্যাট

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = ফরম্যাট করুন

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = { $words }-এর সবকিছু মুছতে আবার ক্লিক করুন

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $drive }-এর { $name }, { $size }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = { $drive }-এর { $name }, { $size } মুছে ফেলুন

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = পার্টিশন মুছুন

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = এখানে নতুন পার্টিশন বানান

# Label of the row where a new part is made.
disks-new-part = নতুন পার্টিশন

# Tip of the button that makes a new part.
disks-make-tip = বানান

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = { $drive }-এ { $size }-এর একটি নতুন পার্টিশন বানাতে আবার ক্লিক করুন

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = { $name } মুছে ফেলুন

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $drive }-এ, { $size }

# Title of the box for making a new part in free space.
disks-sheet-new = নতুন পার্টিশন

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $drive }-এ, { $size } খালি

# Tip of the round button that closes the box of changes.
disks-close = বন্ধ করুন

# Words on a row while a part is being mounted.
disks-mounting = মাউন্ট হচ্ছে

# Words on a row when a part has been mounted.
disks-mounted = মাউন্ট হয়েছে

# Words on a row while a part is being unmounted.
disks-unmounting = আনমাউন্ট হচ্ছে

# Words on a row when a part has been unmounted.
disks-unmounted = আনমাউন্ট হয়েছে

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = বন্ধ করা হচ্ছে

# Words on a drive's row when it is safe to remove.
disks-safe-remove = নিরাপদে খোলা যাবে

# Words on a drive's row while it is being powered off.
disks-powering-off = বন্ধ হচ্ছে

# Words on a drive's row when it has been powered off.
disks-off-safe = খুলে নেওয়া নিরাপদ

# Words on a row while a part is being renamed.
disks-renaming = নাম বদলানো হচ্ছে

# Words on a row when a part has been renamed.
disks-renamed = নাম বদলানো হয়েছে

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = ফোল্ডার এখন নিজে থেকে বাছা হচ্ছে

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = এটি { $dir }-এ দেখা যায়

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = চালুর সময় { $dir }-এ মাউন্ট হয়

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = চালুর সময় মাউন্ট করা হয় না

# Words on a row while a folder or start-up choice is being saved.
disks-saving = সংরক্ষণ হচ্ছে

# Words on a row while a part is being resized.
# SAFETY: needs a human review before it ships
disks-resizing = আকার বদলানো হচ্ছে

# Words on a row when a part has been resized. { $size } is its new size.
# SAFETY: needs a human review before it ships
disks-resized = আকার বদলে { $size } হয়েছে

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Trindle ফাইলগুলো খুঁজে পায়নি, তাই জায়গা করতে পারছে না।

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = স্ন্যাপশট নেওয়া হচ্ছে

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Trindle পরে স্ন্যাপশটটি খুঁজে পায়নি।

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } কিছুরই আকার বদলানো হয়নি।

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Trindle যখন স্ন্যাপশট নিচ্ছিল তখন কিছু বদলে গেছে, তাই কিছুরই আকার বদলানো হয়নি।

# Words on a row while room is being made.
disks-making-room = জায়গা করা হচ্ছে

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = সোয়াপ ফাইল বন্ধ করা হচ্ছে

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = সোয়াপ ফাইল আবার চালু করা হচ্ছে

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } রিস্টার্ট না করা পর্যন্ত সোয়াপ ফাইল বন্ধ থাকবে। { $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } জায়গা করা হয়েছে

# Words on a row while a part is being repaired.
# SAFETY: needs a human review before it ships
disks-repairing = মেরামত হচ্ছে

# Words on a row when a part has been repaired.
# SAFETY: needs a human review before it ships
disks-repaired = মেরামত হয়েছে

# Red words when some faults could not be put right.
# SAFETY: needs a human review before it ships
disks-repair-failed = Trindle কিছু ত্রুটি সারাতে পারেনি। যে ফাইল রাখতে চান তা অন্য ড্রাইভে কপি করে নিন।

# Words on a row while a part is being checked.
disks-checking = পরীক্ষা হচ্ছে

# Words on a row when the check found no faults.
disks-no-faults = কোনো ত্রুটি পাওয়া যায়নি

# Red words when the check found faults.
disks-faults-found = ত্রুটি পাওয়া গেছে। মেরামত করতে রেঞ্চ চাপুন।

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = ফরম্যাট হচ্ছে

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = ফরম্যাট হয়েছে

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = পুরো ড্রাইভ মুছে ফেলা

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = নতুন পার্টিশন পাওয়া যাচ্ছে না।

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = ড্রাইভ সেবা পাওয়া যায়নি।

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Trindle নতুন পার্টিশনের শনাক্তকারী পড়তে পারেনি।

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = মুছে ফেলা হচ্ছে

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = মুছে ফেলা হয়েছে

# Words on a row while a new part is being made.
disks-making-it = বানানো হচ্ছে

# Words on a row when a new part has been made.
disks-made = বানানো হয়েছে

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = এটি সংরক্ষণ করা যায়নি: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
# SAFETY: needs a human review before it ships
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = আগে একটি নাম দিন।

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] নামটি অনেক লম্বা। সর্বোচ্চ { $max }টি অক্ষর রাখুন।
       *[other] নামটি অনেক লম্বা। সর্বোচ্চ { $max }টি অক্ষর রাখুন।
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = শুধু অক্ষর, সংখ্যা, - ও _ ব্যবহার করুন (ফাঁকা জায়গা নয়)।

# Red words under the folder name box when it starts with a dash.
disks-name-start = একটি অক্ষর বা সংখ্যা দিয়ে শুরু করুন।

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } ইতিমধ্যে ব্যবহার হচ্ছে।

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] এই পার্টিশনের জন্য নামটি অনেক লম্বা। সর্বোচ্চ { $max }টি অক্ষর রাখুন।
       *[other] এই পার্টিশনের জন্য নামটি অনেক লম্বা। সর্বোচ্চ { $max }টি অক্ষর রাখুন।
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT নামে * ? . , ; : / \ | + = < > [ ] বা " থাকতে পারে না।

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] আগে মাউন্ট করুন। এই ধরনের পার্টিশন শুধু মাউন্ট থাকা অবস্থায় বড় করা যায়।
       *[smaller] আগে মাউন্ট করুন। এই ধরনের পার্টিশন শুধু মাউন্ট থাকা অবস্থায় ছোট করা যায়।
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] এই ধরন বড় করা যায় না।
       *[smaller] এই ধরন ছোট করা যায় না।
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = আপনি পাসওয়ার্ড দেননি, তাই কিছুই বদলায়নি।

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = আপনি পাসওয়ার্ড দেননি, তাই কাজ মাঝপথে থেমে গেছে।

# Red words when another change is pressed while one is still being made.
disks-change-going = একটি পরিবর্তন চলছে।

disks-one-at-a-time = আরেকটি পরিবর্তন এখনও চলছে। শেষ হলে আবার চেষ্টা করুন।

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Trindle নিশ্চিত হতে পারেনি যে সে আপনার পাসওয়ার্ড চাইবে, তাই কিছুই বদলায়নি।

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Trindle আপনার এইমাত্র দেওয়া পাসওয়ার্ড বাতিল করতে পারেনি। কয়েক মিনিট ড্রাইভের আরেকটি পরিবর্তনে পাসওয়ার্ড নাও চাওয়া হতে পারে। Trindle আবার চেষ্টা করছে।

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = আপনার দেওয়া পাসওয়ার্ড এখন বাতিল করা হয়েছে, তাই পরের পরিবর্তনে পাসওয়ার্ড চাওয়া হবে।

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Trindle ড্রাইভ সেবায় পৌঁছাতে পারেনি, তাই কিছুই বদলায়নি।

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = এই পরিবর্তন চলার সময় ড্রাইভ বন্ধ হয়ে গিয়েছিল।

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = এটি শেষ না হওয়া পর্যন্ত ড্রাইভ খোলা থাকবে, যাতে আপনি দেখতে পারেন কেমন হলো।

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, তাই কিছুই বদলায়নি।

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, তাই কাজ মাঝপথে থেমে গেছে।

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = ড্রাইভ সেবাকে জিজ্ঞাসা করা যায়নি

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } কাজ মাঝপথে থেমে গেছে।

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }।

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = হয়েছে: { $steps }।

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = ব্যর্থ: { $steps }।

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = হয়নি: { $steps }।

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = অজানা: { $steps }।

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } এর জন্য আনমাউন্ট করা হয়েছিল এবং এখন মাউন্ট করা নেই।

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } এর জন্য আনমাউন্ট করা হয়েছিল এবং এখনও আনমাউন্ট আছে। তার সারি থেকে মাউন্ট করুন।

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } আনমাউন্ট হয়ে থাকতে পারে। তার সারি দেখুন।

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } এর জন্য আনমাউন্ট করা হয়েছিল। তার সারি থেকে আবার মাউন্ট করুন এবং জিজ্ঞাসা করলে তার পাসওয়ার্ড লিখুন।

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } এর জন্য আনমাউন্ট করা হয়েছিল এবং আবার মাউন্ট করা যায়নি ({ $why })। তার সারি থেকে মাউন্ট করুন।

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = ফরম্যাট শেষ, কিন্তু Trindle এটিকে আবার তার ফোল্ডার { $dir } দিতে পারেনি। { $why } তার সারি থেকে আবার ফোল্ডার বাছুন।

# Red words when a drive is in use and cannot be unmounted.
disks-busy = কিছু একটি এখনও এটি ব্যবহার করছে। এর ফাইল দেখাচ্ছে এমন উইন্ডো বা প্রোগ্রাম বন্ধ করে আবার চেষ্টা করুন।

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = ওই আকার এতে থাকা ফাইলগুলোর জন্য খুব ছোট।

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } আনমাউন্ট করা

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } আবার মাউন্ট করা

# Step of a change.
disks-step-eject = খুলে নেওয়ার জন্য ড্রাইভ প্রস্তুত করা

# Step of a change.
disks-step-power-off = ড্রাইভ বন্ধ করা

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = { $dev } ফরম্যাট করা

# Step of a change.
disks-step-create = পার্টিশন বানানো

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = { $dev } মুছে ফেলা

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-resize-part = { $dev }-এর আকার বদলানো

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-resize = { $dev }-এর ভেতরের জিনিসের আকার বদলানো

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-repair = { $dev } মেরামত করা

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } পরীক্ষা করা

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } এর নাম বদলানো

# Step of a change. { $dev } is the part.
disks-step-fstab = { $dev } এর ফোল্ডার ও চালুর সেটিংস সংরক্ষণ করা

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = { $dev }-এ { $method } ধাপ

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = এই { $kind } পার্টিশন ব্যবহারে না থাকলেই কেবল ছোট করা যায়, তাই Trindle এখান থেকে জায়গা করতে পারছে না।

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = xfs পার্টিশন ছোট করা যায় না।

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = ব্যবহারে থাকা অবস্থায় এই { $kind } পার্টিশন ছোট করা যায় না।

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Trindle শুধু সিস্টেমের নিজস্ব ডিস্কের স্ন্যাপশট নিতে পারে, তাই এখানে জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-not-in-table = এটি ড্রাইভের পার্টিশন টেবিলের কোনো পার্টিশন নয়, তাই Trindle এখান থেকে জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-no-online-shrink = ড্রাইভ সেবা এখানে ব্যবহারে থাকা অবস্থায় Btrfs ছোট করতে পারে না, তাই Trindle জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-holds-parts = এর ভেতরে অন্য পার্টিশন আছে, তাই Trindle এখান থেকে জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-not-gpt = এর ড্রাইভে GPT পার্টিশন টেবিল নেই, তাই Trindle এখান থেকে জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-mount-first = আগে মাউন্ট করুন: Btrfs মাউন্ট থাকা অবস্থায় ছোট করা হয়।

# Red words under the Make room row.
disks-room-mounts-not-one = এটি মাউন্ট করা নেই, তাই Trindle জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-read-only = এটি শুধু-পড়া হিসেবে মাউন্ট করা, তাই Trindle জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-read-only-unknown = Trindle বুঝতে পারেনি এটি শুধু-পড়া কি না, তাই জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-many-drives = ফাইলগুলো একাধিক ড্রাইভে ছড়িয়ে আছে, তাই Trindle জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-drives-unknown = Trindle বুঝতে পারেনি ফাইলগুলো কতগুলো ড্রাইভে ছড়িয়ে আছে, তাই জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-swap-unknown = Trindle বুঝতে পারেনি এতে সোয়াপ ফাইল আছে কি না, তাই জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-used-unknown = Trindle পড়তে পারেনি এর কতটা ব্যবহৃত, তাই জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-area-unknown = ড্রাইভ সেবা বলে না ফাইল সিস্টেম কত বড়, তাই Trindle তা যাচাই করতে বা জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-open-missing = Trindle এর আনলক করা কপি খুঁজে পায়নি, তাই জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-encryption-sizes = এনক্রিপশনের জন্য দেওয়া আকারগুলো মেলে না, তাই Trindle জায়গা করতে পারছে না।

# Red words under the Make room row.
disks-room-sizes = এর জন্য দেওয়া আকারগুলো মেলে না, তাই Trindle জায়গা করতে পারছে না।

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = এতে { $used } আছে এবং তার চেয়ে বেশি রাখা দরকার, তাই খালি করার মতো জায়গা খুব কম।

# Red words under the Make room row.
disks-room-sizes-unread = এর আকার পড়া যায়নি।

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = আপনি { $least } থেকে { $most } পর্যন্ত জায়গা করতে পারেন।

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, তাই Trindle থেমে গেছে।

# What was found wrong while making room.
disks-room-check-unmounted = ফাইলগুলো আর মাউন্ট করা নেই

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = পার্টিশন { $is } কিন্তু { $should } হওয়া উচিত

# What was found wrong while making room.
disks-room-check-open = এনক্রিপশনের আনলক করা কপি যে আকারের হওয়া উচিত সেই আকারের নয়

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = ফাইল সিস্টেম { $is } কিন্তু { $should } হওয়া উচিত

# What was found wrong while making room.
disks-room-check-area-small = ফাইল সিস্টেম ব্যবহৃত অংশের চেয়ে ছোট

# What was found wrong while making room.
disks-room-check-fit = ফাইল সিস্টেম আর ধরছে না

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] ড্রাইভ সেবা আর পার্টিশনটি তালিকায় দেখাচ্ছে না, তাই কাজ থামানো হয়েছে।
        [copy] ড্রাইভ সেবা আর আনলক করা কপিটি তালিকায় দেখাচ্ছে না, তাই কাজ থামানো হয়েছে।
       *[files] ড্রাইভ সেবা আর ফাইল সিস্টেমটি তালিকায় দেখাচ্ছে না, তাই কাজ থামানো হয়েছে।
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = ফাইল সিস্টেম ছোট করা

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = এনক্রিপশনের আনলক করা কপি ছোট করা

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = পার্টিশন ছোট করা

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = আকার পরীক্ষা করা

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = { $dev } এ জায়গা করার আগে ({ $at })

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = { $drive }-এর { $name } { $from } থেকে { $to } করতে আবার ক্লিক করুন। আগে এর ফাইলের ব্যাকআপ নিন, কারণ ছোট করার সময় বিদ্যুৎ চলে গেলে ফাইল হারিয়ে যেতে পারে।

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name }-এর Windows হাইবারনেট অবস্থায় ছিল বা ফাস্ট স্টার্ট-আপে রাখা ছিল, তাই এটি ছোট করলে এর ফাইল হারিয়ে যেতে পারে। Windows চালু করে পুনরায় চালু করা বেছে নিন, তারপর আবার চেষ্টা করুন।
