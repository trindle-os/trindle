
# machine translated (Bengali), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = কিবোর্ড ও মাউস

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = বিন্যাস ভাষা টাইপ ক্যাপস-লক স্ক্রিন-কিবোর্ড বোতাম রিং মাউস টাচপ্যাড পয়েন্টার গতি ট্যাপ ক্লিক স্ক্রল চাকা ধীর দ্রুত বাঁহাতি শর্টকাট কী বদল layout language typing caps lock on-screen keyboard button ring mouse touchpad trackpad pointer speed tap click scroll wheel slower faster natural left handed plugin shortcut keys remap change swap ctrl escape super alt mac side buttons tapped held made by programs keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = এখানকার সবকিছু সঙ্গে সঙ্গে বদলে যায়। দ্বিতীয় বিন্যাসটি Alt ও Shift একসঙ্গে চেপে বদলানো যায়।

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = { $layout } বিন্যাসে টাইপ হচ্ছে।

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt ও Shift চাপলে { $layout } হবে।

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock কিছুই করে না।

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock এখন { $key }।

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = আরেকটি Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = কিবোর্ড বিন্যাসের তালিকা এই কম্পিউটারে নেই।

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = কিবোর্ড

# Heading of the line in that card.
settings-keyboard-no-list-heading = কিবোর্ড বিন্যাসের তালিকা পাওয়া যাচ্ছে না

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = ইনস্টল করতে এটি চালান: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] আপনি একটি কী বা বোতাম বদলেছেন।
       *[other] আপনি { $n }টি কী ও বোতাম বদলেছেন।
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = বিন্যাস

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = বিন্যাস

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = ধরন

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = সাধারণ

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = কিছু নয়

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = বদলে যান

# Words behind the i of the layout card.
settings-keyboard-layout-about = এটি ঠিক করে প্রতিটি কী কী টাইপ করবে। আপনি একটি বিন্যাস ও তার একটি ধরন বেছে নিতে পারেন। দ্বিতীয় বিন্যাসে যেতে Alt ও Shift একসঙ্গে চাপুন, আর ফিরতেও আবার একই ভাবে চাপুন। পরিবর্তন সঙ্গে সঙ্গে হয়।

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = এখন ব্যবহার হচ্ছে

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = আরেকটি Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = কিছুই না

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Caps Lock হবে

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = রিংয়ে কিবোর্ডের বোতাম

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = অন্যান্য কী

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock এর বদলে অন্য একটি কী হিসেবে কাজ করতে পারে, যেমন দ্বিতীয় একটি Ctrl বা Escape। কিবোর্ডের বোতামটি রিংয়ের মাঝখানে থাকে এবং পর্দায় একটি কিবোর্ড খোলে। মাউস বা টাচ স্ক্রিন দিয়ে টাইপ করতে এটি ব্যবহার করতে পারেন। রিং পরের বার খুললে এটি দেখা যাবে।

# Words under a line when a press that was not the person's own changed nothing.
# SAFETY: needs a human review before it ships
settings-keyboard-only-yours = কোনো প্রোগ্রাম এটি বদলাতে পারে না।

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = বদলাতে কী বা মাউসের বোতামটি চাপুন।

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = আপনি কিছু চাপেননি, তাই কিছু যোগ হয়নি।

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = বদলে যে কী ব্যবহার করবেন তা লিখুন, যেমন Escape বা Ctrl+C, অথবা Nothing লিখুন।

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle বুঝতে পারেনি সেটি কোন কী। বদলে তালিকা থেকে কীটি বেছে নিন।

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = সব কিবোর্ড ও মাউস

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = আপনি নিচে এই কীটি নিজে বদলেছেন, আর ওই বদলটিই ব্যবহার হয়।

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = এই ডিভাইসে { $key } বদলান

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key } বদলে হবে

# Name of the bin button that takes a change away.
settings-keyboard-take-away = এই বদলটি সরিয়ে দিন

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = { $n } নম্বর লাইন উপেক্ষা করা হয়েছে: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = বদলানো কী পড়া যায়নি

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Trindle বদলগুলো পড়তে না পারা পর্যন্ত আপনার কী স্বাভাবিকভাবে কাজ করবে। { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = বদলানোর কী বা বোতাম বেছে নিন

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = বদলানোর কী বা বোতামটি চাপুন

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = একটি কী বা বোতাম বদলান

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = আপনি { $key } আগেই বদলেছেন।

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = কী ও বোতাম

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = যেকোনো কী বা মাউসের বোতাম অন্য কিছু করতে পারে। সেটি অন্য একটি কী বা বোতাম হতে পারে, বা একসঙ্গে চাপা কী যেমন Ctrl+C, বা পরপর চাপা কয়েকটি কী যেমন H, I। এটি কিছুই না করতে পারে, অথবা ছুঁলে এক কাজ আর চেপে ধরলে আরেক কাজ করতে পারে, যেমন ছুঁলে Escape, চেপে ধরলে Ctrl। এটি লগইন পর্দা থেকে শুরু করে সর্বত্র কাজ করে। Ctrl, Alt ও একটি F কী সবসময় টেক্সট কনসোল খোলে, আর সেখানে trindle-settings --keys default লিখলে প্রতিটি কী আগের মতো হয়ে যায়।

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl ও Caps Lock অদলবদল করুন

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: ছুঁলে Escape, চেপে ধরলে Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape ও Caps Lock অদলবদল করুন

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Alt ও Super অদলবদল করুন, যেমন Mac কিবোর্ডে

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = মাউসের পাশের বোতাম কপি ও পেস্ট করে

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = { $name }-কে বিশ্বাস করুন

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = এখনও কিছু নেই

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = প্রোগ্রামের বানানো কিবোর্ড ও মাউস

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = কী বদলানোর টুল ও গেম লঞ্চার নিজেদের কিবোর্ড বা মাউস বানাতে পারে। এদের কী ও ক্লিক তবুও টাইপ করতে ও খেলতে পারে, কিন্তু বন্ধ করা বা অনুমতি দেওয়ার মতো সুরক্ষিত বোতাম চাপতে পারে না, বা আপনার জন্য রাখা প্রশ্নের উত্তর দিতে পারে না। কোনোটিকে আপনার হিসেবে গণ্য করতে এখানে চালু করুন।

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = কন্ট্রোলার { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $title }-এ { $name }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title } যা করে

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A ও B অদলবদল করুন

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X ও Y অদলবদল করুন

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = ডান স্টিকের ওপর ও নিচ উল্টে দিন

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = প্লাগইনের শর্টকাট

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } ব্যবহার হয় না: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = { $label }-এর কী

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = সংরক্ষিত

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = টাচপ্যাডের জন্যও।

# Title of the mouse card.
settings-keyboard-mouse-title = মাউস

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = পয়েন্টারের গতি হলো মাউস নাড়ালে পয়েন্টার কতটা যায়, টাচপ্যাডেরও একই। স্ক্রলের গতি হলো চাকার প্রতিটি ঘূর্ণনে পাতা কতটা সরে: ধীর, সাধারণ বা দ্রুত, অথবা স্লাইডারের যেকোনো জায়গায়। স্বাভাবিক স্ক্রলে চাকা যেদিকে ঘোরে পাতা সেদিকে সরে, ফোনের মতো। বাঁহাতি চালু করলে বাঁ ও ডান বোতাম অদলবদল হয়।

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = পয়েন্টারের গতি

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = স্ক্রলের গতি

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = স্বাভাবিক স্ক্রল

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = বাঁহাতি

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = ক্লিক সহায়ক

# Small words under the clicking helper line.
# SAFETY: needs a human review before it ships
settings-keyboard-clicker-about = স্থির থাকা বা সুইচ ক্লিকারের ক্লিক আপনার ক্লিক হিসেবে গণ্য হয়।

# Title of the touchpad card.
settings-keyboard-touchpad-title = টাচপ্যাড

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = স্ক্রলের গতি হলো দুই আঙুলে পাতা কতটা সরে। ট্যাপ করে ক্লিক চালু করলে হালকা ট্যাপ ক্লিক হিসেবে গণ্য হয়। স্বাভাবিক স্ক্রলে আঙুল যেদিকে যায় পাতা সেদিকে সরে, ফোনের মতো। টাইপের সময় বন্ধ চালু থাকলে টাইপ করার সময় হাতের তালু টাচপ্যাড ছুঁলে পয়েন্টার নড়ে না।

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = ট্যাপ করে ক্লিক

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = টাইপের সময় বন্ধ

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = ধীর

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = সাধারণ

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = দ্রুত

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB পোর্ট

# Words behind the i of that card.
settings-keyboard-usb-about = কোনো USB পোর্ট চালু না হলে Trindle সেটি একবার বন্ধ করে আবার চালু করে দেখে, এবং একটি পোর্টের জন্য একবারের বেশি করে না।

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle বদলটি ফিরিয়ে নিতেও পারেনি: { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = স্ক্রিনশট, রিপোর্ট ও শর্টকাট

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = স্ক্রিনশট নিন

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = শর্টকাটের তালিকা দেখান

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = এখনই স্ক্রিনশট নিন

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = এখনই তালিকা দেখান

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = ডেস্কটপ আবার চালু করুন

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = রিং ও সহায়ক অংশ আবার চালু করুন

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = সবকিছু আবার চালু করুন

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = { $label }-এর কী

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = এখনই আবার চালু করুন। খোলা উইন্ডো যেমন আছে তেমনই থাকে।

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = আগে জিজ্ঞেস করে, তারপর সবকিছু আবার চালু করুন

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = আবার চালু হয়েছে

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } অন্য একটি শর্টকাট আগেই ব্যবহার করছে।

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = সংরক্ষণ হয়নি। { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = সমস্যা জানান

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = এখনই সমস্যা জানান


# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = সংরক্ষিত হয়নি। বদলানো কীয়ের ফাইলটি পড়া যায়নি, তাই সেটি যেমন ছিল তেমনই রাখা হয়েছে।
