
# machine translated (Bengali), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = স্ন্যাপশট

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = স্ন্যাপশট ফিরিয়ে আনা পুনরুদ্ধার ব্যাকআপ আপডেট আগের সংস্করণ মুছে যাওয়া ফাইল snapshots snapshot restore undo rollback roll back backup update broke btrfs snapper boot menu grub earlier versions older version previous deleted file put back recover home copies

# Help line at the foot of the Snapshots page.
settings-snap-help = প্রতিটি সফটওয়্যার বদলের আগে ও পরে, আর আপনি যখনই নেন, তখনকার সিস্টেম। একটি ফিরিয়ে আনলে পরের শুরু থেকে পুরো সিস্টেম সেই অবস্থায় ফিরে যায়; হোম ফোল্ডারের জন্য ডিস্কের আলাদা অংশ থাকলে, যেমন Trindle-এর ইনস্টলার বানায়, আপনার নিজের ফাইল যেমন আছে তেমনই থাকে। কম্পিউটার চালু না হলে বুট মেনুতে Trindle snapshots বেছে নিন। আগের সংস্করণ প্রতিদিন সবার নিজের ফাইলের একটি কপি রাখে, সিস্টেমের স্ন্যাপশট থেকে আলাদাভাবে।

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = কোনো কপি নেওয়া হয়নি: ডিস্ক প্রায় ভরে গেছে।

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = আজ { $time }-এ

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = গতকাল { $time }-এ

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date }-এ

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = কম্পিউটারটি পুনরায় চালু হয়নি। বদলে রিং থেকে পুনরায় চালু করুন।

# Said beside the Take a snapshot line while it works.
settings-snap-taking = স্ন্যাপশট নেওয়া হচ্ছে

# Said beside the set aside line while the space is freed.
settings-snap-freeing = খালি করা হচ্ছে...

# Said beside the set aside line once the space is freed.
settings-snap-freed = খালি হয়েছে

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = ফিরিয়ে আনা হচ্ছে

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = পরের শুরু থেকে সিস্টেম হবে স্ন্যাপশট { $n }। তারপর যা বদলেছে তা আলাদা করে রাখা হয়েছে, হারায়নি।

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = মুছে ফেলা হচ্ছে

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = মুছে ফেলা হয়েছে

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = পাসওয়ার্ডের অপেক্ষায়...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = প্রতিদিন একটি কপি নেওয়া হয়

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = আর নেওয়া হবে না

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = রাখা হবে

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = মুছে ফেলা হচ্ছে

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = মুছে ফেলা হয়েছে

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] এখনও কোনো কপি নেই
        [one] একটি কপি
       *[other] { $n }টি কপি
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = প্রথমটি একদিনের মধ্যে নেওয়া হবে।

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = এগুলো { $size } জায়গা নেয়, মাপা হয়েছে { $when }।

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = এদের জায়গা দিনে একবার মাপা হয়।

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = এখনই সবগুলো মুছে ফেলুন

# Heading of the card about earlier versions of files.
settings-snap-earlier = আগের সংস্করণ

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle প্রতিদিনের সবার ফাইলের একটি করে কপি রাখতে পারে, যাতে কোনো ফাইলের পুরোনো সংস্করণ, বা ভুল করে মুছে ফেলা ফাইল, Folders থেকে খোলা বা ফিরিয়ে আনা যায়। প্রত্যেকে কপির ভেতরে শুধু সেটুকুই পড়তে পারে যা সে সেদিন পড়তে পারত, তাই ব্যক্তিগত হোম ব্যক্তিগতই থাকে; তখন অন্যদের জন্য খোলা কোনো ফাইল ওই কপিতে তা না যাওয়া পর্যন্ত তাদের জন্য খোলাই থাকে। কপিগুলো সিস্টেমের স্ন্যাপশট থেকে আলাদা: সিস্টেম ফিরিয়ে আনলে আপনার ফাইল কখনও বদলায় না, আর এগুলো সিস্টেম বদলায় না। খরচ হলো জায়গা: আপনি যে ফাইল মুছে বা বদলে ফেলেন তা শেষ কপিটি না যাওয়া পর্যন্ত জায়গা নেয়, তাই কিছু মুছলে ডিস্ক সঙ্গে সঙ্গে খালি হয় না। কপিগুলো মুছে ফেললে এই কম্পিউটারের সবার জন্য এখনই তা খালি হয়, এবং তা ফেরানো যায় না। ডিস্ক প্রায় ভরা থাকলে কোনো কপি নেওয়া হয় না। এর কিছু বদলাতে প্রশাসকের পাসওয়ার্ড লাগে।

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = আপনার ফাইলের আগের সংস্করণ রাখুন

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = মুছে ফেলা ফাইল জায়গা খালি হওয়ার আগে { $time } থাকে।

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = প্রতিদিনের কপি রাখুন

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = প্রতিদিনের কপি কতদিন রাখা হবে

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = স্ন্যাপশট দেখা হচ্ছে…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = স্ন্যাপশট পড়া যায়নি।

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = স্ন্যাপশট { $n } থেকে চালু হয়েছে, তাই এখন যা করা হবে তা রাখা হবে না।

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] ফিরে যাওয়ার মতো { $n }টি স্ন্যাপশট, সবচেয়ে নতুনটি { $when }-এর।
       *[other] ফিরে যাওয়ার মতো { $n }টি স্ন্যাপশট, সবচেয়ে নতুনটি { $when }-এর।
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = এখনও কোনো স্ন্যাপশট নেই। প্রতিটি সফটওয়্যার বদলের আগে একটি নেওয়া হয়।

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, মুছে না ফেলা পর্যন্ত রাখা হবে

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = সিস্টেমকে এতে ফিরিয়ে আনুন

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = এই স্ন্যাপশট মুছে ফেলুন

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = এখনই পুনরায় চালু করুন

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = এগুলো এখনও সাজানো হয়নি।

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = trindle প্যাকেজটি আবার ইনস্টল করুন।

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = স্ন্যাপশট সাজানো নেই। trindle প্যাকেজটি আবার ইনস্টল করলে তা সাজানো হয়: sudo apt install --reinstall trindle।

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = অন্য এক ধরন

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = এগুলোর জন্য Btrfs লাগে; এই কম্পিউটার { $fs } ব্যবহার করে।

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Debian ইনস্টলের সময় Btrfs বেছে নিন।

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = স্ন্যাপশটের জন্য সিস্টেম Btrfs-এ থাকতে হয়, আর এই কম্পিউটার { $fs } ব্যবহার করে। এগুলো পেতে Debian ইনস্টলের সময় সিস্টেমের জন্য Btrfs বেছে নিন।

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = এই কম্পিউটারে কোনো স্ন্যাপশট নেই।

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = কোনো স্ন্যাপশট নেই

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = এই কম্পিউটারে কোনো স্ন্যাপশট নেই

# Heading of the line saying how to have snapshots.
settings-snap-to-have = এগুলো পেতে

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = দেখা হচ্ছে…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = স্ন্যাপশট পড়া যায়নি

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = এখনও নেই

# Under the None yet heading.
settings-snap-none-line-sub = প্রতিটি সফটওয়্যার বদলের আগে একটি নেওয়া হয়।

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = প্রতিটি স্ন্যাপশট পরের শুরু থেকে পুরো সিস্টেমকে তখনকার অবস্থায় ফিরিয়ে আনতে পারে, অথবা মুছে ফেলা যায়। দুটিতেই দুইবার চাপতে হয় এবং প্রশাসকের পাসওয়ার্ড লাগে। ফিরিয়ে আনার পর এর পাশে পুনরায় চালু করার একটি বোতাম আসে।

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = এখনই একটি স্ন্যাপশট নিন

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = স্ন্যাপশট থেকে চালু থাকার সময় নয়।

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = আপনি মুছে না ফেলা পর্যন্ত রাখা হবে।

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] ফিরিয়ে আনার ফলে আলাদা করে রাখা একটি সিস্টেম
       *[other] ফিরিয়ে আনার ফলে আলাদা করে রাখা { $n }টি সিস্টেম
    }

# Under the set aside heading.
settings-snap-free-space = সব ঠিক থাকলে জায়গা খালি করুন।

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = এদের নেওয়া জায়গা খালি করুন

# Heading of the card for taking a snapshot.
settings-snap-take-one = একটি নিন

# Help text of the Take one card (the information button).
settings-snap-take-about = যা পরে বাতিল করতে চাইতে পারেন এমন কিছু চেষ্টা করার আগে একটি স্ন্যাপশট নিন; আপনার নেওয়া স্ন্যাপশট আপনি মুছে না ফেলা পর্যন্ত রাখা হয়। কম্পিউটার স্ন্যাপশট থেকে চালু থাকার সময় যা নেওয়া হবে তা রাখা হবে না, তাই আগে স্বাভাবিকভাবে পুনরায় চালু করুন। প্রতিটি ফিরিয়ে আনা তার ঠিক আগের সিস্টেমকে আলাদা করে রাখে, তাই কিছুই হারায় না; সব ঠিক থাকলে এদের নেওয়া জায়গা খালি করুন।

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = স্ন্যাপশট { $n } থেকে চালু হয়েছে

# Under the Started from snapshot heading.
settings-snap-nothing-kept = এখন যা করা হবে তা রাখা হবে না।

# Heading of a line on the Before restoring card.
settings-snap-files-too = আপনার ফাইলও স্ন্যাপশটের মধ্যে আছে

# Under the Your files are in the snapshots too heading.
# SAFETY: needs a human review before it ships
settings-snap-files-back = ফিরিয়ে আনলে আপনার ফাইলও আগের অবস্থায় ফিরে যায়।

# Heading of the card about what to know before restoring.
settings-snap-before = ফিরিয়ে আনার আগে

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = কোনো স্ন্যাপশট থেকে চালু হলে এখন যা করা হবে তা রাখা হয় না: সিস্টেম যদি এমনই থাকা উচিত হয়, তবে তালিকায় সেটির ওপর ফিরিয়ে আনুন চাপুন, তারপর পুনরায় চালু করুন। এই কম্পিউটারে হোম ফোল্ডার সিস্টেমের অংশ হতে পারে, আর তখন স্ন্যাপশট ফিরিয়ে আনলে আপনার নিজের ফাইলও তখনকার অবস্থায় ফিরে যায়, তাই নতুন কিছু থাকলে আগে নিরাপদ কোথাও কপি করে রাখুন।


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = সিস্টেম ফাইল তালাবন্ধ

# Under the System files locked switch.
settings-snap-locked-sub = শুধু ইনস্টল ও হালনাগাদ এগুলো খোলে।

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = তালাবন্ধ

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = খোলা

