# machine translated (Bengali), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = এর সেটিংস খুলুন

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = করা যায়নি: { $why }

# Line of the built-in Pocketkey row.

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = এই আপডেট একটি নতুন অনুমতি চায়। এটি আপনার জন্য অপেক্ষা করছে।

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = এখন সংস্করণ { $mine }, { $new } তৈরি

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = এখন সংস্করণ { $mine }, { $new } তৈরি। এটি আগের চেয়ে বেশি কিছু চায়: { $more }। { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = শেষ হলে এখানের বোতামগুলো ফিরে আসবে।

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = প্লাগইন খোঁজা হচ্ছে…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = কোনো প্লাগইন ইনস্টল করা নেই, আর এখনও কোনোটি পাওয়াও যাচ্ছে না।

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] এখনও কোনো প্লাগইন ইনস্টল করা নেই; ১টি ইনস্টলের জন্য তৈরি।
       *[other] এখনও কোনো প্লাগইন ইনস্টল করা নেই; { $n }টি ইনস্টলের জন্য তৈরি।
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] ১টি প্লাগইন ইনস্টল করা আছে
       *[other] { $n }টি প্লাগইন ইনস্টল করা আছে
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] ১টির আপডেট আছে
       *[other] { $n }টির আপডেট আছে
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] ১টি আপডেট আটকে রাখা আছে
       *[other] { $n }টি আপডেট আটকে রাখা আছে
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }।

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }।

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, সব হালনাগাদ।

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, এবং { $have }।

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, এবং { $have }; { $held }।

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = এটি কিছুই চায় না।

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = এটি যা চায়: { $list }।

# Title of the Plugins page.
store-plugins-title = প্লাগইন

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = সংস্করণ { $mine } ইনস্টল করা; { $new } তৈরি।

# On a plugin's card: its installed version.
store-plugins-version-installed = সংস্করণ { $version }, ইনস্টল করা।

# On a plugin's card: the version on offer.
store-plugins-version-offered = সংস্করণ { $version }।

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }।

# Tip of the round button that installs a plugin.
# SAFETY: needs a human review before it ships
store-plugins-install = { $name } ইনস্টল করুন

# Tip of the round button that releases a plugin whose updates are held back.
# SAFETY: needs a human review before it ships
store-plugins-release = { $name }-এর আটকে রাখা আপডেট ছাড়ুন

# Tip of the round button that updates a plugin to a version.
# SAFETY: needs a human review before it ships
store-plugins-update-to = { $version } সংস্করণে আপডেট করুন

# Heading of the card that says where plugins come from.
store-plugins-where = প্লাগইন কোথা থেকে আসে

# Information on the card that says where plugins come from.
# SAFETY: needs a human review before it ships
store-plugins-come-from = প্লাগইন শুধু Trindle-এর নিজস্ব তালিকা থেকে আসে, যা এই পাতা খোলার প্রতিবার trindle.org থেকে পড়া হয়। একটি দিতে চাইলে জমা দিন: চালু হওয়ার আগে প্রতিটি পরীক্ষা করা হয়।

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = এটি ইনস্টল হয়েছে, কিন্তু এর ব্যাকগ্রাউন্ড অংশ চলছে না: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = এটি সরানো যায়নি: { $why }
