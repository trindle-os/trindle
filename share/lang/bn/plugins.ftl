
# machine translated (Bengali), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = ফোল্ডারের ওপরে পড়ার মতো কোনো { $file } নেই ({ $why })।

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file }-এ কোনো [Plugin] অংশ নেই।

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } এই Trindle চেনে না: এটি Interface={ $known } চেনে।

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin]-এ Interface={ $known } দরকার।

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin]-এ অক্ষর, সংখ্যা, ডট ও ড্যাশের একটি Id দরকার, যেমন org.example.Weather।

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin]-এ একটি Name দরকার।

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin]-এ একটি Version দরকার।

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary হবে এক লাইনের।

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } এক লাইনের সাদামাটা লেখা, সর্বোচ্চ { $max }টি অক্ষর।
       *[other] [{ $group }] { $key } এক লাইনের সাদামাটা লেখা, সর্বোচ্চ { $max }টি অক্ষর।
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = { $name } নামে কোনো অনুমতি নেই: আছে { $list }।

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring]-এ একটি Icon দরকার: প্লাগইনের ফোল্ডারে একটি SVG বা PNG ফাইল।

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon-এ { $icon } লেখা, যা প্লাগইনের ফোল্ডারে নেই।

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring]-এ একটি Label দরকার, পয়েন্টার এর ওপর থাকলে যা বলা হয়।

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring]-এ Run দরকার, ক্লিক করলে যা চালু হয়।

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings]-এ Fields দরকার, অর্থাৎ ঘরগুলোর নাম।

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Field নাম { $name }-এ শুধু অক্ষর, সংখ্যা, ড্যাশ ও আন্ডারস্কোর থাকতে পারে।

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Field { $name }-এর কোনো [{ $group }] অংশ নেই।

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }]-এ একটি Label দরকার।

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices সর্বোচ্চ { $most }টি, প্রতিটি সর্বোচ্চ { $chars }টি অক্ষরের সাদামাটা লেখা।
       *[other] [{ $group }] Choices সর্বোচ্চ { $most }টি, প্রতিটি সর্বোচ্চ { $chars }টি অক্ষরের সাদামাটা লেখা।
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom হলো প্লাগইনের হোমে থাকা একটি ফাইলের নাম: অক্ষর, সংখ্যা, ডট, ড্যাশ ও আন্ডারস্কোর, ডট দিয়ে শুরু নয়, কোনো ফোল্ডার নয়।

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] একটি পছন্দ, তাই এতে দুই বা তার বেশি Choices, অথবা একটি ফাইলের ChoicesFrom দরকার।

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default এর Choices-এর একটি নয়।

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] একটি নোট, তাই এতে TextFrom দরকার, যা প্লাগইনের হোমে থাকা একটি ফাইলের নাম: অক্ষর, সংখ্যা, ডট, ড্যাশ ও আন্ডারস্কোর, ডট দিয়ে শুরু নয়, কোনো ফোল্ডার নয়।

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] একটি নোট, যাতে কোনো সেটিং থাকে না, তাই এর Default নেই।

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind হলো { $kind }: এটি switch, text, choice বা note হতে হবে।

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] একটি সুইচ, তাই এর Default হবে true বা false।

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background]-এর জন্য [Plugin] Permissions-এ background অনুমতি দরকার।

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = প্লাগইনটি কোনো কিছুতেই যুক্ত হয়নি: এতে [Ring], [Settings], [Background], [Shortcut] বা notifications অনুমতি দিন।

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } প্লাগইনের ফোল্ডারের বাইরে লিঙ্ক করা।

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run কমান্ড হিসেবে পড়া যায় না।

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run-এ প্লাগইনের ফোল্ডারের ভেতরের প্রোগ্রামের নাম থাকতে হবে, { $program } নয়।

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run-এ { $program } লেখা, যা প্লাগইনের সঙ্গে থাকা কোনো প্রোগ্রাম নয় (নেই, বা চালানো যায় না)।

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu-তে { $n }টি আইটেম: সর্বোচ্চ { $max }টি চলে।
       *[other] [Ring] Menu-তে { $n }টি আইটেম: সর্বোচ্চ { $max }টি চলে।
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Menu id { $id }-এ শুধু অক্ষর, সংখ্যা, ড্যাশ ও আন্ডারস্কোর থাকতে পারে।

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu-তে { $id } দুইবার আছে।

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu-তে { $id } আছে, যার কোনো [{ $group }] অংশ নেই।

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label এক লাইনের ছোট লেখা, সর্বোচ্চ { $max }টি অক্ষর।
       *[other] [{ $group }] Label এক লাইনের ছোট লেখা, সর্বোচ্চ { $max }টি অক্ষর।
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }]-এ Run দরকার, এটি বাছলে যা চালু হয়।

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] শুধু Trindle-এর তালিকার Flow-এর জন্য, যা control অনুমতি চায়।

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }]-এ Run দরকার, যে কমান্ডকে PIN দেওয়া হয়।

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen সর্বোচ্চ { $max }টি অক্ষরের।
       *[other] [{ $group }] ShowWhen সর্বোচ্চ { $max }টি অক্ষরের।
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen লিখতে হয় field=value আকারে, যেমন ShowWhen=mode=home: অন্য একটি ঘর এবং তার একটি মান।

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen লিখতে হয় field=value আকারে, এবং ঘরের নামে শুধু অক্ষর, সংখ্যা, ড্যাশ ও আন্ডারস্কোর থাকতে পারে।

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen-এ এক লাইনের সাদামাটা লেখার একটি মান দরকার, সর্বোচ্চ { $max }টি অক্ষর।
       *[other] [{ $group }] ShowWhen-এ এক লাইনের সাদামাটা লেখার একটি মান দরকার, সর্বোচ্চ { $max }টি অক্ষর।
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen নিজের ঘরের নাম দিয়েছে: এতে অন্য, আগের একটি ঘরের নাম দিতে হবে।

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen { $other } ঘরের নাম দিয়েছে, যা এই প্লাগইনের Fields-এ নেই।

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen { $other } ঘরের নাম দিয়েছে, যা Fields-এ এর পরে আসে: এটি আগের কোনো ঘর হতে হবে।

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen { $other } ঘরের নাম দিয়েছে, যার নিজেরও একটি ShowWhen আছে: একটি ঘর শুধু এমন ঘরের ওপর নির্ভর করতে পারে যার ShowWhen নেই।

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen { $other } সুইচের নাম দিয়েছে, তাই এর মান হবে true বা false।

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen-এর মান { $value }, যা { $other }-এর Choices-এর একটি নয়।

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen { $other } নোটের নাম দিয়েছে, যাতে কোনো সেটিং নেই: একটি ঘর শুধু সুইচ, পছন্দ বা লেখার ওপর নির্ভর করতে পারে।

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [নেটওয়ার্ক] শোনা: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen-এর জন্য [Plugin] Permissions-এ network অনুমতি দরকার।

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = [Network] Listen-এর জন্য [Plugin] Permissions-এ listen অনুমতি দরকার।

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = listen অনুমতির জন্য [Network] Listen দরকার, অর্থাৎ যে পোর্টগুলো খুলে দিতে হবে।

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = এর plugin.ini এই Trindle-এর চেনা কোনো interface-এর জন্য নয়।

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = এটি শোনার জন্য কোনো পোর্টের নাম দেয়নি।

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = এর plugin.ini পড়া যায়নি।

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = এটিকে শোনার অনুমতি দেওয়া হয়নি।

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = এটি যে পোর্টের নাম দিয়েছে সেগুলো অনুমতি পাওয়া পোর্ট নয়। আবার অনুমতি দিতে হবে।

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name }-এর কাজ করতে আপনার অনুমতি দরকার: { $n }টির মধ্যে ০টি অনুমোদিত।

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } যে অনুমতি চায় তার কিছুটাই পেয়েছে: { $n }টির মধ্যে { $a }টি অনুমোদিত।

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] এটি কিছুই চায় না।
        [one] এটি যে একটি জিনিস চায় তা অনুমোদিত।
       *[other] এটি যে সব { $n }টি জিনিস চায় তা অনুমোদিত।
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = অনুমতি দিন

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = আগের চেয়ে অন্য পোর্টে যোগাযোগ করা যাক

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = ইনস্টল করা নেই।

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = অন্য একটি প্লাগইন

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = সেই প্লাগইন ইনস্টল করা নেই।

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = এটি সংরক্ষণ করা যায়নি: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = প্লাগইনের ফাইল কপি করা যায়নি।

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } এর ব্যাকগ্রাউন্ড অংশ চলছে না: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = এটি চালু করতে trindle-plugin পাওয়া যায়নি।

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] এটি { $n } সেকেন্ডের মধ্যে চালু হয়নি।
       *[other] এটি { $n } সেকেন্ডের মধ্যে চালু হয়নি।
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = যোগ দেওয়ার মতো কোনো ডেস্কটপ সেশন চলছে না, তাই এটি চালু করা হয়নি। আগে লগ ইন করুন।

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = কোন ডেস্কটপ সেশনে যোগ দেবে তা বোঝা যাচ্ছে না ({ $sessions }), তাই এটি চালু করা হয়নি। WAYLAND_DISPLAY ঠিক সেশনে ঠিক করে আবার চালান।

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = ফাইলের জায়গায় একটি ফোল্ডার আছে

# Error text when a plugin's id is not an id.
plugins-not-an-id = প্লাগইনের id নয়

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts-এ { $n }টি শর্টকাট: সর্বোচ্চ { $max }টি চলে।
       *[other] [Plugin] Shortcuts-এ { $n }টি শর্টকাট: সর্বোচ্চ { $max }টি চলে।
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Shortcut id { $id }-এ শুধু অক্ষর, সংখ্যা, ড্যাশ ও আন্ডারস্কোর থাকতে পারে।

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts-এ { $id } দুইবার আছে।

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts-এ { $id } আছে, যার কোনো [{ $group }] অংশ নেই।

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } আগে থেকেই Trindle-এর শর্টকাট; অন্য কী বেছে নিন।

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } প্লাগইনের আরেকটি শর্টকাট ব্যবহার করছে।

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }]-এ Keys দরকার, যেমন Ctrl+Alt+S। { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }]-এ Run দরকার, কী চাপলে যা চালু হয়।

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = এর জন্য লেখা কী শর্টকাট নয়। { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } আগে থেকেই Trindle-এর শর্টকাট।

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } আগে থেকেই আপনার নিজের একটি শর্টকাট।

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } আগে থেকেই { $holder } ব্যবহার করছে।

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = এটি ইনস্টল করা কোনো প্লাগইনের শর্টকাট নয়।

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = এটি চালু করা যায়নি ({ $why })।

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = এটি সংরক্ষণ করা যায়নি: কিছু বদলায়নি, কারণ { $why }।

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = এটি সংরক্ষণ করা যায়নি ({ $why })।



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = অ্যাক্সেসিবিলিটির অনুমতি শুধু তালিকার Trindle স্ক্রিন রিডারের জন্য।

