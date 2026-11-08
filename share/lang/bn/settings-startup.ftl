
# machine translated (Bengali), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = লগইনের সময় চালু

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay স্বয়ংক্রিয় চালু লগইন দেরি

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = লগইন করার সময় যেসব অ্যাপ নিজে থেকে খোলে। দেরি ঠিক করলে অ্যাপ আগে নেটওয়ার্ক বা অন্য অ্যাপের জন্য অপেক্ষা করে।

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = লগইন করার সময় কিছুই নিজে থেকে চালু হয় না।

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = লগইন করার সময় { $name } নিজে থেকে চালু হয়।

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] লগইন করার সময় { $n }টি অ্যাপ নিজে থেকে চালু হয়।
       *[other] লগইন করার সময় { $n }টি অ্যাপ নিজে থেকে চালু হয়।
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] আরও একটি তালিকায় আছে, বন্ধ করা অবস্থায়।
       *[other] আরও { $n }টি তালিকায় আছে, বন্ধ করা অবস্থায়।
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = সঙ্গে সঙ্গে

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = এক মিনিট পরে

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = ২ মিনিট পরে

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] { $n } সেকেন্ড পরে
       *[other] { $n } সেকেন্ড পরে
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name }-এর দেরি

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = লগইনের সময় চালু করুন

# Name of the bin button on an app's line that stops it starting at login.
# SAFETY: needs a human review before it ships
settings-startup-stop = লগইনের সময় চালু হওয়া বন্ধ করুন

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = এটি সরানো যায়নি, তাই লগইনের সময় এখনও চালু হবে।

# Red words under the add card when something could not be added.
settings-startup-not-added = এটি যোগ করা যায়নি।

# Placeholder of the search box above the list of installed apps.
settings-startup-find = অ্যাপ খুঁজুন

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = লগইনের সময় এটি চালু করুন

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = এটি যোগ করা যায়নি: অ্যাপটি আর ইনস্টল করা নেই।

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = যেমন syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = ডিস্ক থেকে একটি প্রোগ্রাম বেছে নিন

# Heading of the line for adding a command.
settings-startup-command = কমান্ড

# Small words under the command line.
settings-startup-command-about = টার্মিনালে যেভাবে লেখা হয় সেভাবে, অথবা ডিস্কের একটি প্রোগ্রাম।

# Red words under the command line when nothing was typed.
settings-startup-command-empty = একটি কমান্ড লিখুন, অথবা একটি প্রোগ্রাম বেছে নিন।

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = লগইনের সময় চালানোর প্রোগ্রাম বেছে নিন

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = কিছুই নিজে থেকে চালু হয় না

# Small words under that line.
settings-startup-none-about = এর পাশে একটি অ্যাপ যোগ করুন।

# Title of the card listing what starts at login.
settings-startup-list-title = লগইনের সময় চালু হয়

# Words behind the i of the card listing what starts at login.
settings-startup-about = লগইনের পর প্রতিটি অ্যাপ কিছুক্ষণ অপেক্ষা করতে পারে, যাতে নেটওয়ার্ক বা অন্য অ্যাপ আগে তৈরি হয়ে যায়। এর সুইচ এটিকে চালু না করেই তালিকায় রাখে; বিন বোতাম লগইনের সময় চালু হওয়া পুরোপুরি বন্ধ করে।

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } চালায়: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = ইনস্টল করা অ্যাপ থেকে বেছে নিন

# Name of the button that opens the box for typing a command.
settings-startup-open-command = একটি কমান্ড লিখুন, অথবা একটি প্রোগ্রাম বেছে নিন

# Heading of the line for adding an installed app.
settings-startup-app-heading = ইনস্টল করা একটি অ্যাপ

# Small words under that line.
settings-startup-app-about = অ্যাপের তালিকা থেকে একটি বেছে নিন।

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = একটি কমান্ড বা প্রোগ্রাম

# Small words under that line.
settings-startup-cmd-about = লিখে দিন, অথবা ডিস্ক থেকে বেছে নিন।

# Words behind the i of the Add an app card.
settings-startup-add-about = ইনস্টল করা অ্যাপ থেকে বেছে নিন, অথবা টার্মিনালে যেভাবে লেখেন সেভাবে একটি কমান্ড দিন, কিংবা ডিস্ক থেকে একটি প্রোগ্রাম বেছে নিন। এটি পরের লগইন থেকেই, সঙ্গে সঙ্গে চালু হবে; তালিকায় আসার পর এর জন্য অপেক্ষার সময় ঠিক করা যায়।

# Title of the card for adding an app.
settings-startup-add-title = অ্যাপ যোগ করুন
