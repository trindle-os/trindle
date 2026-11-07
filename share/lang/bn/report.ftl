# machine translated (Bengali), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = সমস্যা জানান

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = কিছু একটা গোলমাল হয়েছে। রিপোর্ট পাঠাবেন?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = এটি ত্রুটি, কম্পিউটারের সারসংক্ষেপ এবং লগের শেষ কয়েকটি লাইন পাঠায়, যেখান থেকে নাম, ইমেল ঠিকানা, নেটওয়ার্ক ঠিকানা ও Wi-Fi-এর নাম বাদ দেওয়া হয়। লগের লাইনে ফাইলের নাম ও ওয়েব ঠিকানা তবু থাকতে পারে। এটি আপনার ফাইল বা পাসওয়ার্ড পাঠায় না।

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = আপনি কী করছিলেন? (ঐচ্ছিক)

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
report-not-taken = এখনও রিপোর্ট নেওয়া হচ্ছে না।

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = দেখা হচ্ছে।


# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = একসঙ্গে অনেক বেশি রিপোর্ট।

# Written beside the Send button once the report has been sent.
report-already-sent = আগেই পাঠানো হয়েছে।

# Written under the typing box when Save is pressed a second time.
report-already-saved = আগেই সংরক্ষণ করা হয়েছে।

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
report-no-at = ইমেইল ঠিকানা নয়, আপনার Buildmorphic ব্যবহারকারীর নাম দিন, অর্থাৎ @ থাকবে না।

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = ব্যবহারকারীর নাম ২ থেকে ৩১টি ছোট হাতের অক্ষর, সংখ্যা বা ড্যাশ দিয়ে হয়।

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub নামে শুধু অক্ষর, সংখ্যা ও ড্যাশ থাকতে পারে।

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = পরের জন্য রাখা যায়নি: { $reason }।

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = সংরক্ষণ করা যায়নি: { $reason }।

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = অনুমতি নেই

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = ড্রাইভ ভরে গেছে

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = ফোল্ডারটি নেই

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = ড্রাইভ রাজি হয়নি

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl নেই।

# Why a report could not be posted: the post could not be started.
report-not-posted = পাঠানো যায়নি।

# Why a report could not be posted: the computer is offline.
report-no-connection = সংযোগ নেই।

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = ঠিকানাটি { $code } জানিয়েছে।

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = কী ঘটেছিল তা বলতে আরও কয়েকটি শব্দ লিখুন।

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = লগ পড়া হচ্ছে।

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = এর সঙ্গে পাঠানোর মতো কোনো লগ নেই, তাই এটি পাঠানো যাবে না। এর বদলে সংরক্ষণ করুন।



report-sent-as-bug = পাঠানো হয়েছে। বাগ { $number } হিসেবে দেখা হয়েছে। ধন্যবাদ।

