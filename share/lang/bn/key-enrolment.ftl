
# machine translated (Bengali), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
# SAFETY: needs a human review before it ships
key-enrol-title = রিস্টার্টের সময় ড্রাইভারের কী অনুমোদন করা

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen নীল পর্দা পাসওয়ার্ড ড্রাইভার

# First paragraph before the steps when the driver is NVIDIA's own.
# SAFETY: needs a human review before it ships
key-enrol-why-nvidia = NVIDIA-র নিজস্ব ড্রাইভার এই কম্পিউটারেই তৈরি হয়, তাই আপনি এর কী অনুমোদন না করা পর্যন্ত Secure Boot এটি চালু করবে না। রিস্টার্টের আগে এটি করুন, নাহলে পর্দা কালো থেকে যেতে পারে।

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
# SAFETY: needs a human review before it ships
key-enrol-why-addon = অতিরিক্ত ড্রাইভার এই কম্পিউটারেই তৈরি হয়, তাই আপনি এর কী অনুমোদন না করা পর্যন্ত Secure Boot এটি চালু করবে না। রিস্টার্টের আগে এটি করুন।

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = শুধু অক্ষর ও অঙ্ক দিয়ে একটি পাসওয়ার্ড বানান। নীল পর্দায় আমেরিকান কিবোর্ড থাকে, তাই চিহ্নগুলো অন্য কী-তে থাকতে পারে। এটি লিখে রাখুন; মাত্র একবার লাগবে।

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = রিস্টার্ট করুন। Trindle চালু হওয়ার আগে Enroll MOK লেখা একটি নীল পর্দা আসবে। প্রায় ১০ সেকেন্ড পরে এটি হাল ছেড়ে দেয়, তাই একটি কী চাপতে তৈরি থাকুন।

# Step 3: the first two screens.
key-enrol-step3 = Enroll MOK বেছে নিন, তারপর Continue।

# Step 4: the third screen and the password.
# SAFETY: needs a human review before it ships
key-enrol-step4 = Yes বেছে নিন, তারপর আপনার বানানো পাসওয়ার্ডটি লিখুন।

# Step 5: the last screen.
key-enrol-step5 = Reboot বেছে নিন। কম্পিউটার ফিরে এলে ড্রাইভার চালু হবে।

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = নীল পর্দা ফসকে গেলে কম্পিউটার আগের মতোই ড্রাইভার ছাড়া চালু হয়। কিছু ভাঙেনি; আবার জিজ্ঞাসা পেতে sudo mokutil --import /var/lib/dkms/mok.pub চালান এবং রিস্টার্ট করুন।
