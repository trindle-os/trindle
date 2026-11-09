
# machine translated (Bengali), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = এখনও কোনো প্রিন্টার নেই। আপনার নেটওয়ার্কে যা পাওয়া যায় তা নিচে দেখানো হয়েছে।

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } থামানো আছে, তাই আপাতত এতে কিছু ছাপা হবে না।

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } ছাপছে।

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = { $name }-এ ছাপার জন্য তৈরি।

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } এখন { $state }।

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } সাজানো আছে; আগে কোনটিতে ছাপা হবে তা বাছা হয়নি।

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n }টি প্রিন্টার সাজানো আছে; আগে কোনটিতে ছাপা হবে তা বাছা হয়নি।

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = এটি CUPS-এর জানা কোনো কাজ নয়।

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = বাতিল হয়েছে

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat ব্যর্থ হয়েছে

# Words in the ring while the network is searched for printers.
settings-printers-searching = খোঁজা হচ্ছে

# Words in the ring when the search is finished.
settings-printers-search-done = শেষ

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = এখন ডিফল্ট

# Shown beside a printer after a test page was sent.
settings-printers-sent = পাঠানো হয়েছে

# Shown beside a printer after it was removed.
settings-printers-removed = সরানো হয়েছে

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = আগে প্রিন্টারের ঠিকানা লিখুন।

# Shown under the By its address line while the printer is added.
settings-printers-adding = যোগ করা হচ্ছে...

# Shown beside a printer or the address line after it was added.
settings-printers-added = যোগ করা হয়েছে

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = এর ড্রাইভার খোঁজা হচ্ছে...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = { $printer }-এ কাজ { $number }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = এই কাজটি বাতিল করুন

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = ছাপার কাজ

# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = যেকোনো প্রিন্টারে ছাপার অপেক্ষায় থাকা প্রতিটি কাজ, এখন যেমন আছে। এই পাতা খোলা থাকলে তালিকা কয়েক সেকেন্ড পরপর আবার পড়া হয়। ক্রস চিহ্ন একটি কাজ বাতিল করে; যে কাগজ আগেই ছাপা হয়ে গেছে তা ছাপাই থাকে।

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS সাড়া দিচ্ছে না

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = সারি পড়া যাচ্ছে না।

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = দেখা হচ্ছে…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = সারি।

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = কিছু অপেক্ষায় নেই

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = ছাপার সময় কাজগুলো এখানে দেখা যায়।

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = থামানো

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = অলস

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = ছাপছে

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = ডিফল্ট, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = আগে এটিতে ছাপুন

# Tip of the button that prints a test page.
settings-printers-test-page = একটি পরীক্ষার পাতা ছাপুন

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = এই প্রিন্টারটি সরিয়ে দিন

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = খোঁজার জন্য cups-filters লাগে

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = এটি ইনস্টল করুন এই কমান্ডে: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = এই প্রিন্টারটি যোগ করুন

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = নতুন কিছু পাওয়া যায়নি

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = এটি কি চালু, আর একই নেটওয়ার্কে আছে?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = আগে যারা নিজেদের জানান দেয়, তারপর একে একে প্রতিটি যন্ত্র।

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = যোগ করুন

# Heading of the line where a printer's address is typed.
settings-printers-by-address = ঠিকানা দিয়ে

# Small line under "By its address".
settings-printers-by-address-sub = যেমন 192.168.1.50, যা প্রিন্টারের নিজের পর্দায় থাকে।

# Heading of the card for adding printers.
settings-printers-add-card = একটি প্রিন্টার যোগ করুন

# Help text (i button) of the Add a printer card.
settings-printers-add-about = যে প্রিন্টার নিজেদের জানান দেয় সেগুলো আগে খোঁজা হয়, তারপর নেটওয়ার্কের প্রতিটি যন্ত্রকে একে একে জিজ্ঞাসা করা হয়। প্রিন্টারটি চালু ও একই নেটওয়ার্কে থাকতে হবে। কিছু প্রিন্টার নিজেদের জানান দেয় না, আর কিছু নেটওয়ার্ক সেই ঘোষণা এগিয়ে দেয় না: তখন ঠিকানা দিয়ে যোগ করুন, যেমন 192.168.1.50। এটি প্রিন্টারের নিজের পর্দায়, নেটওয়ার্ক সেটিংসের ভেতরে পাবেন।

# Name of the page in the list of settings pages.
settings-printers-title = প্রিন্টার

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = প্রিন্টার ছাপা স্ক্যানার printer print scanner

# Help text at the top of the page.
settings-printers-help = আপনার নেটওয়ার্কের প্রিন্টার নিজে থেকেই পাওয়া যায়। ব্যবহার করতে একটির পাশে যোগ করুন চাপুন।

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = এই কম্পিউটারে ছাপার ব্যবস্থা ইনস্টল করা নেই।

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = দেখা হচ্ছে…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = ছাপার সেবা সাড়া দিচ্ছে না, তাই এখন কিছু ছাপা যাবে না।

# Heading of the card shown when printing is not installed.
settings-printers-printing = ছাপা

# Help text (i button) of that card.
settings-printers-printing-about = ছাপার জন্য CUPS লাগে, যা এই কম্পিউটারে নেই। এটি ইনস্টল হলে আপনার নেটওয়ার্কের প্রিন্টার নিজে থেকেই পাওয়া যাবে।

# Line in that card.
settings-printers-unavailable = ছাপা পাওয়া যাচ্ছে না

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = এটি ইনস্টল করুন এই কমান্ডে: sudo apt install cups

# Line in that card.
settings-printers-then-found = তারপর প্রিন্টার পাওয়া যাবে

# Small line under it.
settings-printers-then-found-sub = আপনার নেটওয়ার্কেরগুলো, নিজে থেকেই।

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = ছাপার সেবা।

# Line in the Printers card when there are none.
settings-printers-none = এখনও কোনো প্রিন্টার নেই

# Small line under "No printers yet".
settings-printers-none-sub = এর পাশে একটি যোগ করুন।

# Heading of the card that lists the printers.
settings-printers-card = প্রিন্টার

# Help text (i button) of the Printers card.
settings-printers-card-about = এই কম্পিউটারে সাজানো প্রতিটি প্রিন্টার। তারা চিহ্ন একটিকে আগে ব্যবহারের প্রিন্টার করে; পাতার আইকন তাতে একটি পরীক্ষার পাতা ছাপে; ঝুড়ি এটি সরিয়ে দেয়, দুইবার জিজ্ঞাসা করে। বদলাতে আপনার অ্যাকাউন্ট lpadmin গ্রুপে থাকতে হবে।

# Red words under a page range box that cannot be used.
printers-bad-range = পৃষ্ঠা যেমন 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = কপি: ১ থেকে ৯৯

# Name of the choice to print on one side of the paper.
printers-sides-one = এক পাশে

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = দুই পাশে, লম্বা ধারে

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = দুই পাশে, ছোট ধারে

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = খাড়া

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = আড়াআড়ি

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = অনুমতি নেই। আপনার অ্যাকাউন্ট lpadmin গ্রুপে থাকতে হবে; তা হলে লগ আউট করে আবার লগ ইন করুন।

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS রাজি হয়নি।

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = ওই প্রিন্টারের নাম CUPS মেনে নেয় না।

# Red words when printing is not installed on the computer.
printers-not-installed = এই কম্পিউটারে ছাপার ব্যবস্থা ইনস্টল করা নেই।

# Red words when the printer chosen has been removed since.
printers-gone = { $name } আর সাজানো নেই।

# Red words when the printer chosen is paused.
printers-paused = { $name } থামানো আছে।

# Red words when sending the document to CUPS took too long.
printers-too-slow = নথিটি পাঠাতে অনেক বেশি সময় লেগেছে।
