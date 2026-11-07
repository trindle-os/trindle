# machine translated (Bengali), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian-এর প্যাকেজ

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = সংস্করণ { $have }-এ আটকে রাখা; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = আটকে রাখা; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = সংস্করণ { $have }-এ আটকে রাখা

# On a thing held back with no update waiting.
store-upd-held = আটকে রাখা

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = সংস্করণ { $have }-এ আটকে রাখা; আরও নতুন কিছু এখনও নেই

# On the Held back card: nothing newer waits.
store-upd-held-nothing = আটকে রাখা; আরও নতুন কিছু এখনও নেই

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] ১টি আপডেট
       *[other] { $n }টি আপডেট
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] ১টি আপডেট তৈরি
       *[other] { $n }টি আপডেট তৈরি
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, মোট { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] ১টি আটকে রাখা আছে
       *[other] { $n }টি আটকে রাখা আছে
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }।

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
# SAFETY: needs a human review before it ships
store-upd-firmware-restart = ফার্মওয়্যার শেষ করতে রিস্টার্ট লাগবে।

# Headline while the check runs.
store-upd-checking = আপডেট খোঁজা হচ্ছে।

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = { $name } আপডেট হচ্ছে। এটি ঢোকার সময় আপনি কাজ চালিয়ে যেতে পারেন।

# Headline while all the updates go in.
store-upd-installing-all = আপডেট ইনস্টল হচ্ছে। এগুলো ঢোকার সময় আপনি কাজ চালিয়ে যেতে পারেন।

# Headline when the updates are in and the computer must start again.
# SAFETY: needs a human review before it ships
store-upd-restart = শেষ করতে রিস্টার্ট করুন: আপডেট ঢুকে গেছে, কম্পিউটার আবার চালু হলে কাজে লাগবে।

# Added to the headline when what went in needs a restart.
# SAFETY: needs a human review before it ships
store-upd-gone-in-restart = যা ঢুকেছে তা শেষ করতে রিস্টার্ট লাগবে।

# Added to the headline with no internet.
store-upd-no-more = এই কম্পিউটার ইন্টারনেটে যুক্ত নেই, তাই আর খোঁজা যাচ্ছে না।

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = কোনো আপডেট তৈরি নেই

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = এই কম্পিউটারের জানা কোনো আপডেট অপেক্ষায় নেই। ইন্টারনেটে যুক্ত না থাকলে আরও খোঁজা যায় না।

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = অ্যাপ

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle ও Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = ফার্মওয়্যার

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = { $names }-এর আপডেট খোঁজা যায়নি।

# Headline when everything is up to date.
store-upd-current = সবকিছু হালনাগাদ আছে।

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = শেষবার দেখা হয়েছে { $time }-এ।

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = ইন্টারনেট ছাড়া খোঁজা যায় না

# Red words under a card when a check took too long.
store-upd-too-long = সাড়া দিতে অনেক বেশি সময় লেগেছে।

# Tip of the round button that checks again.
store-upd-check-again = আবার খুঁজুন

# Main button while the check runs.
store-upd-checking-main = খোঁজা হচ্ছে

# Main button while the updates go in.
store-upd-installing-main = ইনস্টল হচ্ছে

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] ১টি আপডেট ইনস্টল করুন
       *[other] { $n }টি আপডেট ইনস্টল করুন
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = এখনই রিস্টার্ট করুন

# Main button that checks for updates.
store-upd-check-for = আপডেট খুঁজুন

# Tip of the round button that releases a thing held back.
# SAFETY: needs a human review before it ships
store-upd-release = { $name } ছাড়ুন

# Tip of the round button that holds a thing's updates back.
# SAFETY: needs a human review before it ships
store-upd-hold = { $name } আটকে রাখুন

# A card's state while it is being checked.
store-upd-status-checking = খোঁজা হচ্ছে

# A card's state while its updates go in.
store-upd-status-installing = ইনস্টল হচ্ছে

# A card's state when its check failed; the reason follows.
store-upd-status-failed = খোঁজা যায়নি। { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held }টি আটকে রাখা

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n }টি আটকে রাখা

# A card's state when nothing waits.
store-upd-up-to-date = হালনাগাদ

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = { $from } থেকে, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = { $from } থেকে

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = ভাগ করা অংশ, { $from } থেকে

# Note under the Apps card.
store-upd-yours-alone = শুধু আপনার, তাই পাসওয়ার্ড ছাড়াই আপডেট হয়। অ্যাপ তার নতুন সংস্করণের দরকারি ভাগ করা অংশগুলো সাথে আনে।

# Note under the Apps card: what holding back does.
# SAFETY: needs a human review before it ships
store-upd-hold-why = আটকে রাখা আপডেট আপনি না ছাড়া পর্যন্ত যে সংস্করণে আছে সেখানেই থাকে, এবং এখানে ও রিংয়ের "সব ইনস্টল"-এর বাইরে থাকে।

# Heading of the Apps card.
store-upd-card-apps = অ্যাপ

# Plugins card: every plugin is up to date.
store-upd-plugins-current = প্রতিটি প্লাগইন হালনাগাদ আছে।

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] ১টি প্লাগইনের আপডেট আটকে রাখা আছে।
       *[other] { $n }টি প্লাগইনের আপডেট আটকে রাখা আছে।
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] ১টি প্লাগইনের আপডেট আছে
       *[other] { $n }টি প্লাগইনের আপডেট আছে
    }

# Heading of the Plugins card.
store-upd-card-plugins = প্লাগইন

# Held back card: nothing is held back.
store-upd-nothing-held = কিছুই আটকে রাখা নেই।

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = ছাড়া হয়েছে

# Note under the Held back card: what releasing does.
# SAFETY: needs a human review before it ships
store-upd-release-why = ছাড়া জিনিস আবার বাকিদের সাথে, এখানের ও রিংয়ের "সব ইনস্টল" থেকে ঢোকে।

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = একটি ডিভাইসের ফার্মওয়্যার

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = { $id } ডিভাইসের ফার্মওয়্যার

# Heading of the Held back card.
store-upd-card-held = আটকে রাখা

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } থেকে { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] ১টি আপডেট, যা একাই ঢোকে
       *[other] { $n }টি আপডেট, যা একসাথে ঢোকে
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Debian থেকে

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Debian-এর { $total }টি আপডেটের মধ্যে { $shown }টি দেখানো হচ্ছে।

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = এই কম্পিউটারে Trindle { $version } আছে।

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle ও Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Debian-এর প্যাকেজগুলো একটি আরেকটির সাথে মানানসই করে তৈরি, তাই একটি একা ঢোকালে অন্যগুলো সাথে আসতে পারে বা তালগোল পাকিয়ে যেতে পারে। তাই সেগুলো একটি বোতাম থেকে একসাথে ঢোকে।

# Note on the Trindle and Debian card: why no row has a Hold back button.
# SAFETY: needs a human review before it ships
store-upd-not-held = Trindle ও Debian-এর আপডেট পুরো কম্পিউটারের নিরাপত্তা সংশোধন আনে, তাই সেগুলো আটকে রাখা যায় না।

# Heading of the card about the daily security updates.
# SAFETY: needs a human review before it ships
store-upd-card-security = নিরাপত্তা আপডেট

# On the security card: its state.
# SAFETY: needs a human review before it ships
store-upd-security-on = চালু, প্রতিদিন

# On the security card.
# SAFETY: needs a human review before it ships
store-upd-security-note = Debian-এর সংশোধন প্রতিদিন না জিজ্ঞেস করেই ঢোকে।

# On the security card.
# SAFETY: needs a human review before it ships
store-upd-security-part = Trindle-এর অংশ; বন্ধ করা যায় না।

# On the security card: that holding back never stops these.
# SAFETY: needs a human review before it ships
store-upd-never-held = কোনো আপডেট আটকে রাখলেও এগুলো থামে না।

# Firmware card: the computer has no updater.
store-upd-fw-none = এই কম্পিউটারে ফার্মওয়্যার আপডেটার নেই।

# Firmware card: restart to finish.
# SAFETY: needs a human review before it ships
store-upd-fw-restart = শেষ করতে রিস্টার্ট করুন

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = সংস্করণ { $version }

# Note on the firmware card when the updater is there.
# SAFETY: needs a human review before it ships
store-upd-fw-note = কম্পিউটার রিস্টার্ট হলে নতুন ফার্মওয়্যার ঢোকে।

# Note on the firmware card when the updater is missing.
store-upd-fw-install = ফার্মওয়্যার হালনাগাদ রাখতে fwupd ইনস্টল করুন।

# Heading of the Firmware card.
store-upd-card-firmware = ফার্মওয়্যার

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd সবচেয়ে নতুন তালিকা আনতে পারেনি।

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub-এ পৌঁছানো যায়নি।

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Debian-এর তালিকা আনা যায়নি।

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Debian-এর তালিকা আনা যায়নি। { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = আটকে রাখা হয়নি। আটকে রাখা জিনিসের তালিকা রাখা যায়নি: { $why }।

# The same, for a release.
store-upd-not-released-list = ছাড়া হয়নি। আটকে রাখা জিনিসের তালিকা রাখা যায়নি: { $why }।

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = অ্যাপে আটকে রাখা হয়েছে, কিন্তু Flatpak নিজেকে জানানো যায়নি, তাই অ্যাপ আপডেট করে এমন অন্য কিছু এটি ঢুকিয়ে দিতে পারে।

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = অ্যাপে ছাড়া হয়েছে, কিন্তু Flatpak নিজেকে জানানো যায়নি এবং এটি এখনও বাদ রাখতে পারে। আবার আটকে রেখে ছেড়ে দিন।

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak বলেছে: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = নিজে থেকে হালনাগাদ খোঁজা বন্ধ আছে। দেখতে "হালনাগাদ খুঁজুন" চাপুন।

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = খোঁজা হয়নি

# Tip of the round button that ends an update that is running.
store-upd-stop = থামান

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = থামানো হচ্ছে

# Under a row whose update was stopped.
store-upd-stopped = থামানো হয়েছে। চালিয়ে যেতে "হালনাগাদ করুন" চাপুন।

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = থামানো হয়েছে। চালিয়ে যেতে "ইনস্টল করুন" চাপুন।

# Note under the Apps card.
store-upd-steam-restarts = কোনো গেম না চললে Steam আবার চালু হবে।

