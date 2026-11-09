# machine translated (Bengali), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } কম্পিউটারের নিজের একটি সুইচ বা কী দিয়ে বন্ধ করা আছে। সেটি চালু করে আবার চেষ্টা করুন।

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } সিস্টেম আটকে রেখেছে, এবং সেই আটকানো সরানো যায়নি।

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = পাসওয়ার্ডটি নেওয়া হয়নি। আবার চেষ্টা করুন।

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = এই নেটওয়ার্কের জন্য রাখা পাসওয়ার্ডটি নেওয়া হয়নি। ঝুড়ি দিয়ে নেটওয়ার্কটি ভুলে যান, তারপর আবার যুক্ত হোন।

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = নেটওয়ার্কটি পাওয়া যায়নি। এর নাম ঠিক আছে কিনা এবং এটি নাগালের মধ্যে আছে কিনা দেখুন।

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = নেটওয়ার্ক এই কম্পিউটারকে ঢুকতে দিয়েছে কিন্তু কোনো ঠিকানা দেয়নি। আবার চেষ্টা করুন, অথবা রাউটার আবার চালু করুন।

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = নেটওয়ার্ক এই কম্পিউটারকে যুক্ত হতে দেয়নি। এর কাছে সরে এসে আবার চেষ্টা করুন।

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = এমন পাসওয়ার্ড Wi-Fi নেটওয়ার্কের হতে পারে না। দেখে নিয়ে আবার চেষ্টা করুন।

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = নেটওয়ার্কে যুক্ত হওয়া যায়নি।

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = আপনার পাসওয়ার্ড দিয়ে খোলা যায়

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = আপনার পাসওয়ার্ড দিয়ে খোলা ও বদলানো যায়

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = বাড়ির যে কেউ খুলতে পারে

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = বাড়ির যে কেউ বদলাতে পারে

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = { $name } ভাগ করা বন্ধ করুন

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = পরিচালিত নয়

# State of a network device that cannot be used.
settings-network-state-unavailable = পাওয়া যাচ্ছে না

# State of a network device that is not connected.
settings-network-state-disconnected = সংযোগ বিচ্ছিন্ন

# State of a network device that is getting ready to connect.
settings-network-state-preparing = তৈরি হচ্ছে

# State of a network device that is being set up.
settings-network-state-configuring = সাজানো হচ্ছে

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = পাসওয়ার্ড লাগবে

# State of a network device that is getting an address.
settings-network-state-address = ঠিকানা নেওয়া হচ্ছে

# State of a network device whose connection is being checked.
settings-network-state-checking = যাচাই করা হচ্ছে

# State of a network device that is waiting.
settings-network-state-waiting = অপেক্ষায়

# State of a network device that is connected.
settings-network-state-connected = সংযুক্ত

# State of a network device that is disconnecting.
settings-network-state-disconnecting = সংযোগ বিচ্ছিন্ন হচ্ছে

# State of a network device that failed.
settings-network-state-failed = ব্যর্থ

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = তারের মাধ্যমে এবং Wi-Fi দিয়ে { $name } এর সঙ্গে সংযুক্ত।

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] { $name } এর সঙ্গে সংযুক্ত, সিগন্যাল শক্তিশালী।
        [good] { $name } এর সঙ্গে সংযুক্ত, সিগন্যাল ভালো।
        [fair] { $name } এর সঙ্গে সংযুক্ত, সিগন্যাল মোটামুটি।
       *[other] { $name } এর সঙ্গে সংযুক্ত, সিগন্যাল দুর্বল।
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = তারের মাধ্যমে সংযুক্ত।

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi বন্ধ, তাই এই কম্পিউটার সংযুক্ত নয়।

# Headline of the page when not connected.
settings-network-top-none = সংযুক্ত নয়। নিচ থেকে একটি নেটওয়ার্ক বেছে নিন।

# Headline of the page while the networks are looked for.
settings-network-top-looking = নেটওয়ার্ক খোঁজা হচ্ছে…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = নেটওয়ার্ক দেখা যাচ্ছে না: NetworkManager চলছে না।

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = সিস্টেম বাস

# Name of the page in the list of settings pages and its heading.
settings-network-title = নেটওয়ার্ক

# Words the settings search looks for on this page (not shown).
settings-network-keywords = ওয়াইফাই ইন্টারনেট ইথারনেট তার পাসওয়ার্ড সংযোগ ফায়ারওয়াল ভাগ প্রিন্টার ফোল্ডার wifi wi-fi wireless internet ethernet wired password connect firewall sharing share printer folder

# Help text at the top of the page.
settings-network-help = Wi-Fi চালু বা বন্ধ করুন, নেটওয়ার্কে যুক্ত হোন, যে নেটওয়ার্ক আর চান না তা ভুলে যান, অথবা অন্য কম্পিউটারকে যুক্ত হতে দিন।

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] { $name } থেকে শক্তিশালী সিগন্যাল।
        [good] { $name } থেকে ভালো সিগন্যাল।
        [fair] { $name } থেকে মোটামুটি সিগন্যাল।
       *[other] { $name } থেকে দুর্বল সিগন্যাল।
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = এর চিপের জন্য নতুন Linux দরকার।

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = এই কম্পিউটারে কাজ করার মতো Wi-Fi নেই।

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = বন্ধ। নেটওয়ার্কে যুক্ত হতে চালু করুন।

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = চালু, কোনো নেটওয়ার্কে যুক্ত নয়।

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi ব্যবহার করুন

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi ও তার

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi তার ছাড়াই এই কম্পিউটারকে নেটওয়ার্কে যুক্ত করে। বন্ধ করলে এটি নেটওয়ার্ক খোঁজা থামায় এবং সামান্য ব্যাটারি বাঁচে। যে নেটওয়ার্ক নিজের নাম প্রচার করে না, তা নাগালের মধ্যের তালিকায় আসে না: এখানে তার নাম বড় হাতের অক্ষরসহ ঠিকঠাক লিখে যুক্ত হোন। এই কম্পিউটারের Wi-Fi চিপ যদি এমন হয় যা এর Linux-এর কোনো ড্রাইভার এখনও নেয়নি, তাহলে Trindle যে Linux-এ চলে তার চেয়ে নতুন Linux দরকার। নেটওয়ার্ক তারের জন্য আলাদা একটি লাইন আছে, সংযোগ করা বা বিচ্ছিন্ন করার বোতামসহ।

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = অন্য কম্পিউটারকে যুক্ত হতে দিন

# Small line under that switch.
settings-network-share-switch-sub = বাড়িতে ফোল্ডার ও প্রিন্টার ভাগ করতে।

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = ভাগ করা ফোল্ডার খোঁজা হচ্ছে…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = ভাগ করা ফোল্ডার পড়া যায়নি

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = এখনও নাগালের বাইরে

# Small line under it.
settings-network-not-reachable-sub = এটি বন্ধ থাকলে অন্যরা এগুলো খুলতে পারে না।

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = কোনো ফোল্ডার ভাগ করা নেই

# Small line under it.
settings-network-no-shared-sub = ভাগ করতে ফোল্ডারে একটিতে ডান-ক্লিক করুন।

# Heading of the Sharing card.
settings-network-sharing = ভাগ করা

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = "অন্য কম্পিউটারকে যুক্ত হতে দিন" বন্ধ থাকলে, নেটওয়ার্কের কিছুই এই কম্পিউটারে পৌঁছাতে পারে না যদি না এটি আগে ডেকে থাকে, তাই একই Wi-Fi-র কেউ ভাগ্য পরীক্ষা করতে পারে না। বাড়ির অন্য কম্পিউটারের সঙ্গে ফোল্ডার বা প্রিন্টার ভাগ করতে এটি চালু করুন। ফোল্ডার ভাগ করতে, ফোল্ডারে সেটিতে ডান-ক্লিক করে "নেটওয়ার্কে ভাগ করুন" বেছে নিন। প্রতিটি বলে কে কী করতে পারবে। আপনার পাসওয়ার্ড দিয়ে: অন্যরা আপনার ভাগ করার পাসওয়ার্ড দিয়ে ফাইল খুলতে, অথবা খুলতে ও বদলাতে পারে। বাড়ির যে কেউ: নেটওয়ার্কের যে কেউ পাসওয়ার্ড ছাড়াই ফাইল খুলতে, অথবা খুলতে ও বদলাতে পারে। সুইচ বন্ধ থাকলে বাড়ির অন্য কম্পিউটার এগুলোর কোনোটিই খুলতে পারে না।

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = বন্ধ করা হচ্ছে…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = পাসওয়ার্ডের অপেক্ষায়...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = ভাগ করার অনুমতি দেওয়া হয়েছে

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = ঢুকতে দেওয়া হয়নি

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi চালু করা যায়নি।

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi বন্ধ করা যায়নি।

# Name of a wired network device that has no name of its own.
settings-network-ethernet = ইথারনেট

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = সংযোগ বিচ্ছিন্ন করুন

# Tip of the button that connects a network or a cable.
settings-network-connect = সংযুক্ত করুন

# Shown under a network or cable while it is being connected.
settings-network-connecting = সংযুক্ত হচ্ছে...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = নাগালের মধ্যে কোনো নেটওয়ার্ক নেই

# Small line under it.
settings-network-none-in-range-sub = এই পাতা খোলার সময় প্রতিবার আবার খোঁজা হয়।

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = নাগালের মধ্যে নেটওয়ার্ক

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = এই কম্পিউটার যতগুলো Wi-Fi নেটওয়ার্ক শুনতে পায় সবগুলো। পাতা খোলার সময় যেটি ব্যবহার হচ্ছে সেটি প্রথমে, তারপর এখানে সংরক্ষিতগুলো, তারপর বাকিগুলো সিগন্যালের জোর অনুযায়ী; পাতা খোলা থাকলে কোনো লাইন নিজের জায়গায় থাকে। একটিতে যুক্ত হতে তীরে চাপুন; নিরাপদ নেটওয়ার্ক প্রথমবার তার পাসওয়ার্ড চায়। ঝুড়ি একটি সংরক্ষিত নেটওয়ার্ক ও তার পাসওয়ার্ড ভুলে যায়। খোলা নেটওয়ার্ক এনক্রিপ্ট করা নয়, তাই কাছের অন্যরা এর মাধ্যমে যা পাঠানো হয় তা দেখতে পারে।

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = নাগালের মধ্যের নেটওয়ার্ক দেখতে Wi-Fi চালু করুন।

# Part of a network's small line: it is saved on this computer.
settings-network-saved = সংরক্ষিত

# Part of a network's small line: it needs a password.
settings-network-secured = নিরাপদ

# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = খোলা, এনক্রিপ্ট করা নয়

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
settings-network-forget = এই নেটওয়ার্ক ভুলে যান

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = নেটওয়ার্কটি ভুলে যাওয়া যায়নি।

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = পাসওয়ার্ড

# Tip of the button that joins the network.
settings-network-join = যুক্ত হোন

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = বাতিল করুন

# Shown under a network while it is being joined.
settings-network-joining = যুক্ত হচ্ছে...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi পাসওয়ার্ড কমপক্ষে ৮ অক্ষরের হয়।

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = লুকানো নেটওয়ার্কে যুক্ত হোন

# Small line under it.
settings-network-hidden-sub = নামটি বড় হাতের অক্ষরসহ ঠিকঠাক লিখুন।

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = নেটওয়ার্কের নাম

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = পাসওয়ার্ড, যদি থাকে

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = নেটওয়ার্কের নাম দরকার।

# Shown under a network after it was joined.
settings-network-joined = যুক্ত হয়েছে

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } তবুও এটি সংরক্ষিত হয়ে গেছে: আবার যুক্ত হওয়ার আগে ঝুড়ি দিয়ে এটি ভুলে যান।

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = এটি বিচ্ছিন্ন হয়নি।
