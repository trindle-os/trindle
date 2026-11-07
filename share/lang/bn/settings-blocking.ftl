# machine translated (Bengali), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = এইমাত্র

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] { $n } মিনিট আগে
       *[other] { $n } মিনিট আগে
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] { $n } ঘণ্টা আগে
       *[other] { $n } ঘণ্টা আগে
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] { $n } দিন আগে
       *[other] { $n } দিন আগে
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = পাসওয়ার্ডের অপেক্ষায়...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = ব্লক করা চালু

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = ব্লক করা বন্ধ

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = সংরক্ষিত

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = একটি সাইটের নাম লিখুন, যেমন example.com।

# Headline of the page when blocking is off.
settings-blocking-top-off = ব্লক করা বন্ধ আছে। আগের মতোই খোঁজ আপনার নেটওয়ার্কের নিজস্ব সার্ভারে যায়।

# Headline of the page while blocking is starting.
settings-blocking-top-starting = ব্লক করা চালু হচ্ছে।

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = বিজ্ঞাপন ও খারাপ সাইট ব্লক করা হচ্ছে, এবং খোঁজ গোপনে { $provider }-এ যাচ্ছে।

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = এই নেটওয়ার্কে গোপন সার্ভারে পৌঁছানো যাচ্ছে না, তাই আপাতত আপনার নেটওয়ার্কের নিজস্ব সার্ভার ব্যবহার হচ্ছে। ব্লক করা এখনও কাজ করছে।

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = বিজ্ঞাপন ও খারাপ সাইট ব্লক করা হচ্ছে। খোঁজ আপনার নেটওয়ার্কের নিজস্ব সার্ভারে যায়, যারা তা দেখতে পায়।

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = বিজ্ঞাপন ও খারাপ সাইট ব্লক করা হচ্ছে। খোঁজ ভিপিএনের মাধ্যমে ভিপিএনের নিজস্ব সার্ভারে যায়।

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = বিজ্ঞাপন ও খারাপ সাইট ব্লক করা হচ্ছে।

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = চালু হচ্ছে

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = বন্ধ

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = ব্লক করা চালু হচ্ছে

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = ব্লক করা বন্ধ

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = আজ ব্লক হয়েছে

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = আজ ব্লক হওয়া খোঁজ

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = তালিকা

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = আনা হচ্ছে

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = এখনও নেই

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = সাইটের তালিকা

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = সাইট তালিকাভুক্ত, আনা হয়েছে { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = এখনও কিছু ব্লক হয়নি

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = সাইট এখানে দেখা যাবে, নতুনটি আগে।

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = এই সাইটকে সবসময় ঢুকতে দিন

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = সম্প্রতি ব্লক হয়েছে

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = সাইট ব্লক হওয়ার সঙ্গে সঙ্গে এখানে দেখা যায়, নতুনটি আগে। প্লাস চিহ্ন দিলে সেটি এখন থেকে ঢুকতে পারে। এখানে শুধু ব্লক হওয়া সাইট দেখানো হয়। যেসব সাইট ঢুকতে দেওয়া হয়েছে তা কখনও রাখা হয় না।

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = কী ব্লক হচ্ছে দেখতে ব্লক করা চালু করুন।

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = যেমন example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = একটি সাইট যোগ করুন

# Small line under "Add a site".
settings-blocking-add-sub = এটি ও এর সব সাবডোমেইন কখনও ব্লক হবে না।

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = এই সাইট আবার ব্লক করুন

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = সবসময় ঢুকতে দেওয়া হয়

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = এখানে তালিকাভুক্ত সাইট ও তাদের সব সাবডোমেইন কখনও ব্লক হয় না, এমনকি তালিকায় থাকলেও। মাইনাস চিহ্ন একটিকে আবার ব্লক করে।

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = ব্লক করা

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = ব্লক বিজ্ঞাপন ট্র্যাকার ম্যালওয়্যার প্রতারণা গোপন খোঁজ অনুমতি block adverts ads trackers malware scams dns private lookups quad9 mullvad cloudflare allow

# Help text at the top of the page.
settings-blocking-help = এই কম্পিউটারের প্রতিটি ব্রাউজার ও অ্যাপের জন্য বিজ্ঞাপন, ট্র্যাকার ও পরিচিত খারাপ সাইট ব্লক করে, এবং নামের খোঁজ এনক্রিপ্ট করা সংযোগে পাঠায়, যাতে আপনি যে নেটওয়ার্কে আছেন সেটি তা পড়তে বা বদলাতে না পারে।

# Text of the switch that turns blocking on or off.
settings-blocking-switch = বিজ্ঞাপন ও খারাপ সাইট ব্লক করুন

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = খোঁজ কোথায় যায়

# Help text (i button) of the Blocking card.
settings-blocking-card-about = HaGeZi-র বিজ্ঞাপন, ট্র্যাকার ও পরিচিত খারাপ সাইটের তালিকা ব্যবহার করে, দিনে একবার আনা হয়। এটি বন্ধ থাকলে কিছুই চলে না। নামের খোঁজ এখানে বাছাই করা গোপন সার্ভারে এনক্রিপ্ট করা সংযোগে যায়। Quad9 কে চেয়েছিল তার কোনো হিসাব রাখে না। "আপনার নেটওয়ার্ক" ব্লক করে ঠিকই, কিন্তু গোপন নয়: আপনার নেটওয়ার্কের নিজস্ব সার্ভার প্রতিটি খোঁজ দেখতে পায়।

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = HaGeZi-র তালিকা, দিনে একবার আনা হয়।

# Heading of the line with the private server dropdown.
settings-blocking-private = গোপন সার্ভার

# Small line under Private server.
settings-blocking-private-sub = Quad9 কে চেয়েছিল তার কোনো হিসাব রাখে না।

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = আপনার নেটওয়ার্ক
