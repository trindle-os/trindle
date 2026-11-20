
# machine translated (Bengali), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = এটি সঠিক লগইন নাম নয়।

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = লগইন নামটি বেশি লম্বা।

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name } নামে কোনো ব্যবহারকারী নেই।

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } একটি সিস্টেম অ্যাকাউন্ট, তাই Trindle এটি বদলায়নি।

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = কোনো পাসওয়ার্ড দেওয়া হয়নি।

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] পাসওয়ার্ডে অন্তত { $n }টি অক্ষর দরকার।
       *[other] পাসওয়ার্ডে অন্তত { $n }টি অক্ষর দরকার।
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = পাসওয়ার্ডে নতুন লাইন থাকতে পারে না।

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } আগে থেকেই আছে।

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } একমাত্র প্রশাসক, তাই তাঁকে প্রশাসকই থাকতে হবে।

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = আপনি নিজের অ্যাকাউন্ট মুছতে পারবেন না।

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = Trindle { $name }-এর অ্যাকাউন্ট মুছে ফেলেছে, কিন্তু লগইন পর্দার ছবিটি সরাতে পারেনি: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = ছবিটি খুব বড়।

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = এটি Trindle-এর তৈরি ছবি নয়।

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle লগইন পর্দাকে ছবির ফোল্ডারে ঢুকতে দিতে পারেনি: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = এটি কোনো ভাষার নাম নয়।

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle Debian-এর ভাষার তালিকা পড়তে পারেনি: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } ভাষাটি Debian চেনে না।

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = কম্পিউটারের নামে শুধু অক্ষর, সংখ্যা ও হাইফেন ব্যবহার করা যায়। নাম সর্বোচ্চ ৬৩টি অক্ষরের হতে পারে এবং হাইফেন দিয়ে শুরু বা শেষ হতে পারে না।

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = চার্জের সীমা আপাতত চালু আছে, কিন্তু কম্পিউটার বন্ধ করলে সেটি হারিয়ে যাবে: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = পুরো চার্জ করা আপাতত চালু আছে, কিন্তু কম্পিউটার পরের বার চালু হলে সীমা ফিরে আসবে: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Trindle শেয়ারিং বন্ধ করতে পারেনি, তাই অন্য কম্পিউটার এখনও যুক্ত হতে পারে: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = এটি প্লাগইন ID নয়।

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = এমন কোনো ব্যক্তি নেই।

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = ওই ব্যক্তির কোনো ফোল্ডার নেই।

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle অনুমতি পাওয়া প্লাগইনের তালিকা সংরক্ষণ করতে পারেনি: { $why }।

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = এমন কোনো প্লাগইন নেই।

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = কিছুই আপডেট হয়নি। Trindle নিজের কোন সংস্করণ ইনস্টল করা আছে তা বুঝতে পারেনি, তাই নিজের আপডেটটি বাদ দিতে পারেনি।

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = আপডেট ইনস্টল হচ্ছে

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = গুছিয়ে রাখা হচ্ছে

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle ইনস্টল হচ্ছে

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = এটি কোনো প্যাকেজের নাম নয়।

# Red words when the package to remove is not installed.
admin-package-not-installed = ওই প্যাকেজ ইনস্টল করা নেই।

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = এটি এমন ডিভাইস নয় যা fwupd চেনে।

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = কে Tailscale চান, Trindle তা বুঝতে পারছে না।

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = ব্লক করা সময়মতো শুরু হয়নি।

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = ব্লক করা পুরোপুরি বন্ধ হয়নি: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle { $path } আগের জায়গায় ফেরাতে পারেনি: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = এটি কোনো সাইটের নাম নয়।

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = এমন কোনো সার্ভার নেই।

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } দরকারি সবকিছু পাওয়ার আগেই থেমে গেছে, তাই কাজটি শেষ হয়নি: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } ব্যর্থ হয়েছে।

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } সরানো যায়নি: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = কিছু বদলানো হয়নি, কারণ Trindle { $path } পড়তে পারেনি: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle সবকিছু আগের অবস্থায় ফেরাতেও পারেনি: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = এটি আগের সংস্করণ কত দিন রাখা হবে তার সঠিক সময় নয়।

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = আগের সংস্করণগুলো পাওয়া যাচ্ছে না।

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = আগের সংস্করণের ফোল্ডারের জায়গায় একটি ফাইল আছে।

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = কে চেয়েছে তা বোঝা যাচ্ছে না।

# Red words when a step that must go through the administrator's password was started another way.
# SAFETY: needs a human review before it ships
admin-pkexec-only = এটি শুধু pkexec দিয়ে চলে।

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = এটি কোনো কম্পিউটারের নাম নয়।

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = এটি কোনো শেয়ার করা ফোল্ডারের নাম নয়।

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = এটি কোনো কম্পিউটারের ঠিকানা নয়।

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = এটি কোনো শেয়ার করা ফোল্ডারের পাথ নয়।

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = এখানে শুধু আপনার নিজের শেয়ার করা ফোল্ডার বন্ধ করা যায়।

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = ওই জায়গায় কোনো শেয়ার করা ফোল্ডার খোলা নেই।

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Trindle সেট আপ করার পর কেউ { $path } বদলে দিয়েছে, তাই সেখানে কিছু সংযুক্ত নেই।

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } Trindle-এর রাখা অবস্থায় নেই।

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = অনেকগুলো শেয়ার করা ফোল্ডার খোলা আছে।

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } ইনস্টল করা নেই। এটি Debian-এর { $package } প্যাকেজের সঙ্গে আসে।

# Red words in Folders when the login for a share could not be read.
# SAFETY: needs a human review before it ships
admin-login-unreadable = লগইন পড়া যায়নি।

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = এই কম্পিউটারে কী মাউন্ট করা আছে তা Trindle পড়তে পারেনি: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = কোনো প্রোগ্রাম এখনও এই ফোল্ডারে কিছু ব্যবহার করছে। প্রোগ্রামটি বন্ধ করে আবার চেষ্টা করুন।

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = ওই কম্পিউটার সাড়া দিচ্ছে না। আপনি লগ আউট করলে শেয়ার করা ফোল্ডারটি বন্ধ হয়ে যাবে।

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = এটি সঠিক ব্যবহারকারী ID নয়।

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare গোষ্ঠী তৈরি করা যায়নি।

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba নেওয়া হচ্ছে

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = শেয়ারিং ঠিক করা হচ্ছে

# Red words in Settings when a sharing password is set before sharing is set up.
# SAFETY: needs a human review before it ships
admin-sharing-not-set-up = শেয়ারিং এখনও ঠিক করা হয়নি। আগে একটি ফোল্ডার শেয়ার করুন।

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = এই কম্পিউটার নেটওয়ার্কে অন্যদের কাছে এখনও দেখা যাচ্ছে: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = এটি কোনো স্ন্যাপশট নম্বর নয়।

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = সেটিংস থেকে নেওয়া

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle বুঝতে পারেনি সিস্টেম কোন ডিস্কে আছে।

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = সিস্টেম নিজস্ব Btrfs সাবভলিউমে নেই, তাই Trindle এটি স্ন্যাপশটের সঙ্গে বদলাতে পারে না।

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle { $url } থেকে স্বাক্ষর কী নিতে পারেনি।

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = ওই ঠিকানা স্বাক্ষর কী ফেরত পাঠায়নি।

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = কিছু বদলানো হয়নি, কারণ Trindle { $path } পড়তে পারেনি: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = এর স্বাক্ষর কী নেওয়া হচ্ছে

# Second line of the progress while a source is added.
admin-step-adding-source = এটি যোগ করে এর তালিকা পড়া হচ্ছে

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = সোর্সটি ব্যবহার করা যায়নি। { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle সোর্সটি ব্যবহার করতে পারেনি, তাই আবার সরিয়ে দিয়েছে। { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } এখনও এটি চালু বা বন্ধ করতে পারেন না: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle { $user }-এর জন্য এটি চালু করতে পারেনি: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Trindle { $user }-কে ভার্চুয়াল মেশিন ব্যবহার করতে দিতে পারেনি, যা Cowork-এর দরকার: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = Trindle { $user }-কে পাসওয়ার্ড ছাড়া এটি ব্যবহার করতে দিতে পারেনি: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } ইনস্টল হয়েছে, কিন্তু Trindle এর সেট-আপ শেষ করতে পারেনি: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = এটি ইনস্টল হচ্ছে

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = এটি তৈরি-করা সোর্সগুলোর কোনোটি নয়।

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } এই ধরনের কম্পিউটারের জন্য তৈরি নয় ({ $arch })।

# Last line of the progress while a program is installed.
admin-step-setting-up = সেট-আপ করা হচ্ছে

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = প্রতিটি ঠিকানা https:// দিয়ে শুরু হওয়া সাধারণ ওয়েব ঠিকানা হতে হবে।

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = suite ও components সাদামাটা শব্দ হতে হবে।

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = এটি কোনো প্যাকেজের নাম নয়।

# First line of the progress while a source and its program are taken away.
admin-step-removing = এটি সরানো হচ্ছে

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = তালিকা আবার পড়া হচ্ছে

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = এখানে শুধু Trindle-এর যোগ করা সোর্স সরানো যায়।

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = সোর্সটি এখনও আছে: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = স্ন্যাপশট { $n } ডিস্কে নেই।

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = কিছু বদলানো হয়নি, কারণ Trindle স্ন্যাপশট প্রস্তুত করতে পারেনি। { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle বর্তমান সিস্টেম একপাশে সরাতে পারেনি: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = বর্তমান সিস্টেমের নাম এখন { $aside }, { $sub } নয়, এবং আবার { $sub } নাম না দেওয়া পর্যন্ত কম্পিউটার চালু হবে না: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle স্ন্যাপশটগুলো সরিয়ে আনতে পারেনি: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle স্ন্যাপশট জায়গামতো বসাতে পারেনি: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle একপাশে সরানো সিস্টেমের তালিকা হালনাগাদ করতে পারেনি: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = { $names } সরানো যায়নি

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = এগুলো আর কিছুর দরকার ছিল না, তাই Trindle সরিয়ে দিয়েছে: { $list }।

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = সব আপডেট ইনস্টল হয়নি। এগুলো আটকে রাখা হয়েছে: { $list }।

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } বন্ধ আছে, কারণ এর ড্রাইভার ({ $driver }, DKMS) এখনও নতুন Linux কার্নেলের জন্য তৈরি হচ্ছে না।

admin-driver-off = ড্রাইভারটি ({ $driver }, DKMS) এখনও নতুন Linux কার্নেলের জন্য তৈরি হচ্ছে না। যেসব সুবিধার জন্য এটি লাগে সেগুলো বন্ধ আছে।

# Said once after the drivers that are off.
admin-driver-off-rest = কার্নেল ও আপডেটের বাকি অংশ ইনস্টল হয়েছে।

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = ভার্চুয়াল ক্যামেরা

admin-language-in-use = { $code } হলো এই কম্পিউটার যে ভাষায় চালু হয়, তাই এটি সরানো যাবে না।

# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = এই কম্পিউটারে বন্ধ করার মতো কোনো স্প্লিট-লক ধীরগতি নেই।

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle ড্রাইভটি খুঁজে পায়নি, অথবা এটি সিস্টেম ড্রাইভ।

# The drive could not be closed where it was open.
admin-games-drive-busy = ড্রাইভটি ব্যবহার হচ্ছে। এতে খোলা ফাইল বা প্রোগ্রাম বন্ধ করে আবার চেষ্টা করুন: { $why }

# Another software change has the read-only system open.
admin-system-busy = আরেকটি সফটওয়্যার পরিবর্তন চলছে। শেষ হলে আবার চেষ্টা করুন।

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = কিছুই বদলানো হয়নি, কারণ Trindle সিস্টেম খুলতে পারেনি: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle সিস্টেম আবার বন্ধ করতে পারেনি। পরের বার চালুর সময় এটি বন্ধ হবে: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle /usr/local লেখার উপযোগী রাখতে পারেনি: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = এটি শুধু root চালাতে পারে, pkexec দিয়ে নয়।

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Btrfs দরকার।

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = স্ন্যাপশট জায়গামতো বসেছে এবং কম্পিউটার সেটি থেকেই চালু হবে। Trindle পুরনো সিস্টেম গুছিয়ে শেষ করতে পারেনি। কম্পিউটার পরের বার চালু হলে সে আবার চেষ্টা করবে: { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = সর্বোচ্চ { $most }টি পোর্ট খোলা যায়।

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle খোলা পোর্টের তালিকা সংরক্ষণ করতে পারেনি: { $why }।

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = ফায়ারওয়াল এখনও চালু আছে: { $why }

# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = এই প্লাগইন এখন আপনার অনুমতি দেওয়া পোর্টের বদলে অন্য নেটওয়ার্ক পোর্ট চাইছে। সেগুলো খুলতে আবার অনুমতি দিন।

# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = যিনি অনুরোধ করেছেন, প্রোগ্রাম শুধু তাঁর জন্যই যোগ করা যায়।

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = একটু আগে আরেকটি পুনরুদ্ধারে একই নাম ব্যবহার হয়েছে। কয়েক সেকেন্ড অপেক্ষা করে আবার চেষ্টা করুন।

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } বিচ্ছিন্ন করা যায়নি: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] ১টি শেয়ার করা ফোল্ডার সাড়া দেয়নি, তাই এটি এখনও সংযুক্ত আছে। কম্পিউটার বন্ধ হলে এটি বিচ্ছিন্ন হবে।
       *[other] { $n }টি শেয়ার করা ফোল্ডার সাড়া দেয়নি, তাই এগুলো এখনও সংযুক্ত আছে। কম্পিউটার বন্ধ হলে এগুলো বিচ্ছিন্ন হবে।
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = সোয়াপ ফাইল { $path } বন্ধ করা যায়নি: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = সোয়াপ ফাইল { $path } আবার চালু করা যায়নি: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } সিস্টেমের সোয়াপ ফাইল নয়।

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = অন্যরা { $why } বদলাতে বা পড়তে পারে, তাই Trindle সোয়াপ ফাইল { $path } ব্যবহার করবে না।

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = সোয়াপ ফাইল এখন কোথায় তা পাওয়া যায়নি, তাই হাইবারনেশন থেকে জাগা নাও চলতে পারে।

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = রিস্টার্ট করার পর হাইবারনেট আবার কাজ করবে।

# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = চিপ আগে থেকেই সেট আপ হচ্ছে। শেষ হওয়া পর্যন্ত অপেক্ষা করুন।

# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = এটি ডিস্কের পাসওয়ার্ড নয়।

# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = অনেকবার ভুল পাসওয়ার্ড। এক মিনিট অপেক্ষা করে আবার চেষ্টা করুন।

# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = শেষবার চালুর সময় চিপই ডিস্কটি খুলেছে, তাই কিছু করার নেই।

# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = কম্পিউটার চালুর পর ডিস্ক বদলেছে। রিস্টার্ট করে আবার চেষ্টা করুন।

# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = নিরাপত্তা চিপ পাওয়া যায়নি। BIOS-এ চালু করে আবার চেষ্টা করুন।

# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = চিপ সেট আপের টুল clevis ইনস্টল করা নেই।

# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = ডিস্কে আরেকটি কী রাখার জায়গা নেই।

# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = চিপ সেট আপ করা যায়নি। ডিস্ক এখনও পাসওয়ার্ডে খোলে।

# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = চিপ আবার কাজ করছে, কিন্তু আগের কী সরানো যায়নি।
