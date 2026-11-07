# machine translated (Bengali), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }।

# Under the app's name when only the maker is known.
store-app-line-by = নির্মাতা: { $by }।

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, নির্মাতা: { $by }।

# The state beside the dot while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-state-removing = সরানো হচ্ছে

# The state beside the dot while the app is being fetched.
store-app-state-downloading = ডাউনলোড হচ্ছে

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = বসানো হচ্ছে

# The state beside the dot while the app's own update goes in.
store-app-state-updating = আপডেট হচ্ছে

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = ইনস্টল করা, আপডেট আটকে রাখা

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = ইনস্টল করা, সংস্করণ { $version }-এ আটকে রাখা

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = ইনস্টল করা, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = ইনস্টল করা, সংস্করণ { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = ইনস্টল করা, হালনাগাদ

# The same, with the installed version.
store-app-state-current-version = ইনস্টল করা, সংস্করণ { $version }, হালনাগাদ

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = ইনস্টল করা নেই

# The same, with the size of the download.
store-app-state-absent-size = ইনস্টল করা নেই, ডাউনলোড { $size }

# Word beside the spinner while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-busy-removing = সরানো হচ্ছে

# Word beside the spinner while the app is being put in.
store-app-busy-installing = ইনস্টল হচ্ছে

# Tip and spoken name of the round button that gives up a download.
store-app-stop = থামান

# Tip and spoken name of the round button that takes the app off.
# SAFETY: needs a human review before it ships
store-app-remove = সরান

# Round button beside Update or Release that opens the app.
store-app-open-round = খুলুন

# Main button that releases an app whose updates are held back.
store-app-release = ছেড়ে দিন

# Main button that updates the app.
store-app-update = আপডেট করুন

# Main button that opens the app.
store-app-open = খুলুন

# Main button that installs the app.
store-app-install = ইনস্টল করুন

# Button that opens the app's website.
store-app-website = ওয়েবসাইট

# In the picture frame when the app has no pictures.
store-app-no-pictures = এই অ্যাপের কোনো ছবি নেই।

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = এই ছবিটি আনা যায়নি।

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = ছবি আনা হচ্ছে…

# In the picture frame when the computer has no internet.
store-app-picture-offline = এই কম্পিউটার ইন্টারনেটে সংযুক্ত না থাকলে ছবি আনা যায় না।

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = { $n }টির মধ্যে ছবি { $at }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = { $n }টির মধ্যে ছবি { $at }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = আগের ছবি

# Tip of the round button that shows the picture after.
store-app-picture-later = পরের ছবি

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = মালিকানাধীন

# Name of the Facts card.
store-app-facts = তথ্য

# Facts line: the size of the download.
store-app-fact-download = ডাউনলোডের আকার

# Facts line: where the app comes from.
store-app-fact-source = উৎস

# Facts line: the version.
store-app-fact-version = সংস্করণ

# Facts line: the licence of an open source app.
store-app-fact-licence = লাইসেন্স

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = লাইসেন্স, ওপেন সোর্স নয়

# Facts line: who made the app.
store-app-fact-made-by = নির্মাতা

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  { $n ->
        [one] { $count }টি রেটিং থেকে
       *[other] { $count }টি রেটিং থেকে
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = এখনও কোনো রেটিং নেই।

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = দেখানোর মতো রেটিং নেই।

# Under the stars while the ratings are being fetched.
store-app-rating-looking = রেটিং খোঁজা হচ্ছে…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = GNOME-এর খোলা পর্যালোচনা সেবা থেকে।

# Last line of the Rating card when there is no internet.
store-app-rating-offline = এই কম্পিউটার ইন্টারনেটে সংযুক্ত নয়।

# Name of the Rating card.
store-app-rating = রেটিং

# Text of the About card when the app says nothing of itself.
store-app-no-description = এই অ্যাপের কোনো বিবরণ নেই।

# Name of the About card.
store-app-about = পরিচিতি

# Heading of a review that has no title.
store-app-review-no-title = শিরোনাম নেই

# Under What people say when nobody has written.
store-app-people-none = এই অ্যাপ নিয়ে এখনও কেউ লেখেননি।

# Under What people say when there is no internet.
store-app-people-offline = এই কম্পিউটার ইন্টারনেটে সংযুক্ত না থাকলে মানুষের মতামত আনা যায় না।

# Under What people say while reviews are being fetched.
store-app-people-looking = মানুষের মতামত খোঁজা হচ্ছে…

# Name of the card of reviews.
store-app-people = মানুষ যা বলছে

# Facts line: when the app's project was made.
store-app-fact-created = তৈরি

