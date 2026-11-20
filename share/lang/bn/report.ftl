# machine translated (Bengali), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = সমস্যা জানান

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = কিছু একটা গোলমাল হয়েছে। রিপোর্ট পাঠাবেন?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = এটি আপনার লেখা, কম্পিউটারের তথ্য, এই বুটের ত্রুটি এবং Trindle-এর লগ পাঠায়। Trindle যা জোগাড় করে তা থেকে নাম ও ঠিকানা (ইমেল, নেটওয়ার্ক, Wi-Fi) বাদ দেওয়া হয়। লগের লাইনে ফাইলের নাম ও ওয়েব ঠিকানা তবু থাকতে পারে। এটি আপনার ফাইল বা পাসওয়ার্ড পাঠায় না।

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = কী হয়েছিল?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = কোন অ্যাপ?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = নিশ্চিত নই

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = অন্য প্রোগ্রাম

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = ডেস্কটপ

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = পাঠান

# Button at the bottom of the window when the person opened it themselves.
# SAFETY: needs a human review before it ships
report-cancel = বাতিল করুন

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
# SAFETY: needs a human review before it ships
report-dont-send = পাঠাবেন না

# Button that opens the exact text of the report.
report-show-details = বিস্তারিত দেখান

# Button that closes the exact text of the report.
report-hide-details = বিস্তারিত লুকান

# Button under the shown report text that copies it.
report-copy = কপি করুন

# Button under the shown report text that saves it as a file in Downloads.
report-save = সংরক্ষণ করুন

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = এখনও রিপোর্ট পাঠানো যাচ্ছে না।

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = দেখা হচ্ছে।

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = একসঙ্গে অনেক বেশি রিপোর্ট।

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = সংযোগের অপেক্ষায়।

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = সংযোগের অপেক্ষায়। { $why }

# Under the typing box after the report was sent.
report-sent = পাঠানো হয়েছে। ধন্যবাদ।

# Under the typing box while the report is being posted.
report-sending = পাঠানো হচ্ছে।

# Under the typing box after Save. $name is the file's name.
report-saved-as = Downloads/{ $name } নামে সংরক্ষণ করা হয়েছে।

# Red words when the Buildmorphic username holds an email address.
report-no-at = ইমেইল ঠিকানা নয়, আপনার Buildmorphic ব্যবহারকারীর নাম লিখুন। @ ব্যবহার করবেন না।

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = ব্যবহারকারীর নাম ২ থেকে ৩১ অক্ষরের হতে হবে, আর তাতে শুধু ছোট হাতের অক্ষর, সংখ্যা বা ড্যাশ থাকতে পারবে।

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub নামে শুধু অক্ষর, সংখ্যা ও ড্যাশ থাকতে পারে।

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle রিপোর্টটি পরের জন্য সংরক্ষণ করতে পারেনি: { $reason }।

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = সংরক্ষণ করা যায়নি: { $reason }।

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = অনুমতি নেই

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = ড্রাইভ ভরে গেছে

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = ফোল্ডারটি নেই

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = ড্রাইভের ত্রুটি

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl নেই।

# Why a report could not be posted: the post could not be started.
report-not-posted = পাঠানো যায়নি।

# Why a report could not be posted: the computer is offline.
report-no-connection = সংযোগ নেই।

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = সার্ভার { $code } পাঠিয়েছে।

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = কী ঘটেছিল তা বলতে আরও কয়েকটি শব্দ লিখুন।

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = লগ পড়া হচ্ছে।

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = এর সঙ্গে পাঠানোর মতো কোনো লগ নেই, তাই এটি পাঠানো যাবে না। এর বদলে সংরক্ষণ করুন।

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = ঐচ্ছিক ছবি। শুধু প্রজেক্টের মালিক এটি দেখতে পান।

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = একটি PNG ছবি বাছুন

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = স্ক্রিনশট নিন

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = ছবিটি সরান

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = আপনার ছবি, { $width } বাই { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = একটি PNG ছবি বাছুন

report-picture-filter = PNG ছবি

# Under the picture while it is being read.
report-picture-reading = পড়া হচ্ছে।

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = স্ক্রিনশট নেওয়া যায়নি।

report-picture-no-chooser = Trindle ফাইল বাছাইয়ের জানালা খুলতে পারেনি।

# Red words under the picture, each the reason it is refused.
report-picture-not-png = এটি PNG ছবি নয়।

report-picture-too-big = এটি { $megabytes } MB-এর বেশি।

report-picture-too-large = ছবিটি খুব বড়। এটি সর্বোচ্চ { $side } পিক্সেল চওড়া ও { $side } পিক্সেল উঁচু হতে পারে।

report-picture-damaged = ছবিটি নষ্ট।

report-picture-unreadable = পড়া যায়নি।

report-sent-as-bug = পাঠানো হয়েছে। আপনার বাগ রিপোর্টের নম্বর { $number }। ধন্যবাদ।

# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle এখন বিটায় আছে। রিপোর্ট বেনামি: নাম, ইমেইল ঠিকানা ও হোম ফোল্ডার সরিয়ে দেওয়া হয়। ব্যবহারকারীর নাম ঐচ্ছিক।

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic ব্যবহারকারীর নাম (ঐচ্ছিক)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle রিপোর্টটি পাঠাতে পারেনি। { $why } আবার চেষ্টা করতে পাঠান চাপুন।
