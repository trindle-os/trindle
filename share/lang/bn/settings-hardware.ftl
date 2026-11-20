# machine translated (Bengali), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = হার্ডওয়্যার

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = হার্ডওয়্যার ডিভাইস ড্রাইভার ফার্মওয়্যার গ্রাফিক্স শব্দ নেটওয়ার্ক ব্লুটুথ স্টোরেজ ক্যামেরা কিবোর্ড মাউস অনুপস্থিত সমস্যা চিপ hardware devices drivers driver firmware pci usb graphics sound network bluetooth storage camera keyboard mouse missing problem chip

# Text behind the info button of the Hardware page.
settings-hardware-help = এই কম্পিউটারের যেসব অংশ Trindle দেখতে পায়, কাজ অনুযায়ী ভাগ করে দেখানো হয়েছে। যে অংশের ড্রাইভার নেই, বা ড্রাইভারের একটি ফার্মওয়্যার ফাইল নেই, সেটি সবার আগে আসে। ড্রাইভার সিস্টেমের সঙ্গেই আসে, তাই এখানে কিছু ডাউনলোড করার নেই।

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] আপনার { $n }টি ডিভাইসের ড্রাইভার ঠিকমতো কাজ করছে।
       *[other] আপনার সব { $n }টি ডিভাইসের ড্রাইভার ঠিকমতো কাজ করছে।
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n }টি ডিভাইসে নজর দেওয়া দরকার।
       *[other] { $n }টি ডিভাইসে নজর দেওয়া দরকার।
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = গ্রাফিক্স

settings-hardware-kind-sound = শব্দ

settings-hardware-kind-network = নেটওয়ার্ক

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = স্টোরেজ

settings-hardware-kind-input = কিবোর্ড ও মাউস

settings-hardware-kind-camera = ক্যামেরা

settings-hardware-kind-other = অন্যান্য

# Under a device that works; name is the driver's name.
settings-hardware-driver = কাজ করছে। ড্রাইভার: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = কাজ করছে

# Under a device that has no driver.
settings-hardware-no-driver = ড্রাইভার নেই

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = ফার্মওয়্যার নেই

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = কোনো ড্রাইভার ব্যবহার হচ্ছে না

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = অজানা ডিভাইস { $id }

# Card name.
settings-hardware-chip = নিরাপত্তা চিপ

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = চালুর সময় চিপ ডিস্কটি খোলেনি, তাই পাসওয়ার্ড চাওয়া হয়েছে। BIOS আপডেটে এমন হতে পারে।

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = চিপ আবার ব্যবহার করুন

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = ডিস্কের পাসওয়ার্ড

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = বাতিল করুন

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = আগে ডিস্কের পাসওয়ার্ড লিখুন।

# Beside the line while it works.
settings-hardware-chip-waiting = চিপ সেট আপ হচ্ছে

# Once it worked.
settings-hardware-chip-done = পরের বার চালু থেকে চিপ ডিস্কটি খুলবে।
