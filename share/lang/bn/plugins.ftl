
# machine translated (Bengali), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle ফোল্ডারের ওপরে { $file } পড়তে পারছে না ({ $why })।

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file }-এ কোনো [Plugin] অংশ নেই।

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Trindle-এর এই সংস্করণ Interface={ $known } সমর্থন করে, Interface={ $n } নয়।

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
        [one] [{ $group }] { $key } এক লাইনের সাদামাটা লেখা হতে হবে, { $max }টির বেশি অক্ষর চলবে না।
       *[other] [{ $group }] { $key } এক লাইনের সাদামাটা লেখা হতে হবে, { $max }টির বেশি অক্ষর চলবে না।
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = { $name } নামে কোনো অনুমতি নেই। অনুমতিগুলো হলো { $list }।

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring]-এ একটি Icon দরকার: প্লাগইনের ফোল্ডারে একটি SVG বা PNG ফাইল।

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon-এ { $icon } লেখা, কিন্তু প্লাগইনের ফোল্ডারে সেটি নেই।

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring]-এ একটি Label দরকার, পয়েন্টার এর ওপর থাকলে যা দেখানো হয়।

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings]-এ Fields দরকার, অর্থাৎ ঘরগুলোর নাম।

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Field নাম { $name }-এ শুধু অক্ষর, সংখ্যা, হাইফেন ও আন্ডারস্কোর থাকতে পারে।

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Field { $name }-এর কোনো [{ $group }] অংশ নেই।

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }]-এ একটি Label দরকার।

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }]-এ সর্বোচ্চ { $most }টি Choices থাকতে পারে। প্রতিটি সাদামাটা লেখা হতে হবে, { $chars }টির বেশি অক্ষর চলবে না।
       *[other] [{ $group }]-এ সর্বোচ্চ { $most }টি Choices থাকতে পারে। প্রতিটি সাদামাটা লেখা হতে হবে, { $chars }টির বেশি অক্ষর চলবে না।
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom-এ প্লাগইনের ফোল্ডারে থাকা একটি ফাইলের নাম দিতে হবে। শুধু অক্ষর, সংখ্যা, ডট, ড্যাশ ও আন্ডারস্কোর ব্যবহার করুন। নামের শুরুতে ডট বা ফোল্ডারের পথ দেবেন না।

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] একটি পছন্দ, তাই এতে দুই বা তার বেশি বিকল্পসহ Choices, অথবা ফাইল হিসেবে ChoicesFrom দরকার।

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default তার Choices-এর মধ্যে নেই।

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] একটি নোট, তাই এতে TextFrom দরকার, যাতে প্লাগইনের ফোল্ডারে থাকা একটি ফাইলের নাম দিতে হবে। শুধু অক্ষর, সংখ্যা, ডট, ড্যাশ ও আন্ডারস্কোর ব্যবহার করুন। নামের শুরুতে ডট বা ফোল্ডারের পথ দেবেন না।

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] একটি নোট, তাই এটি কোনো সেটিং রাখে না এবং এর Default থাকতে পারে না।

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind হলো { $kind }, কিন্তু এটি switch, text, choice বা note হতে হবে।

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] একটি সুইচ, তাই এর Default অবশ্যই true বা false হতে হবে।

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background]-এর জন্য [Plugin] Permissions-এ background অনুমতি দরকার।

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = প্লাগইনটি কোনো কিছুর সঙ্গে যুক্ত নয়। এতে [Ring], [Settings], [Background], [Shortcut] বা notifications অনুমতি দিন।

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } প্লাগইনের ফোল্ডারের বাইরে যাওয়া একটি লিঙ্ক।

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run কমান্ড হিসেবে পড়া যায় না।

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run-এ প্লাগইনের ফোল্ডারের ভেতরের প্রোগ্রামের নাম থাকতে হবে, { $program } নয়।

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run-এ { $program } লেখা, কিন্তু প্লাগইনে সেই প্রোগ্রাম নেই (ফাইলটি নেই বা চালানো যায় না)।

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu-তে { $n }টি আইটেম আছে। সর্বোচ্চ { $max }টি চলবে।
       *[other] [Ring] Menu-তে { $n }টি আইটেম আছে। সর্বোচ্চ { $max }টি চলবে।
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Menu id { $id }-এ শুধু অক্ষর, সংখ্যা, হাইফেন ও আন্ডারস্কোর থাকতে পারে।

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu-তে { $id } দুইবার আছে।

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu-তে { $id } আছে, যার কোনো [{ $group }] অংশ নেই।

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label এক লাইনের ছোট লেখা হতে হবে, { $max }টির বেশি অক্ষর চলবে না।
       *[other] [{ $group }] Label এক লাইনের ছোট লেখা হতে হবে, { $max }টির বেশি অক্ষর চলবে না।
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] শুধু Trindle-এর তালিকার Flow-এর জন্য, যা control অনুমতি চায়।

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }]-এ Run দরকার, যে কমান্ডকে PIN দেওয়া হয়।

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen সর্বোচ্চ { $max }টি অক্ষরের হতে পারে।
       *[other] [{ $group }] ShowWhen সর্বোচ্চ { $max }টি অক্ষরের হতে পারে।
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen লিখতে হবে field=value আকারে, যেমন ShowWhen=mode=home। অন্য একটি ঘরের নাম দিন, আর সেই ঘরে যে মান থাকতে হবে তা দিন।

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen field=value আকারে লিখতে হবে, এবং ঘরের নামে শুধু অক্ষর, সংখ্যা, ড্যাশ ও আন্ডারস্কোর থাকতে পারে।

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen-এ এক লাইনের সাদামাটা লেখার একটি মান দরকার, { $max }টির বেশি অক্ষর চলবে না।
       *[other] [{ $group }] ShowWhen-এ এক লাইনের সাদামাটা লেখার একটি মান দরকার, { $max }টির বেশি অক্ষর চলবে না।
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen নিজের ঘরের নাম দিয়েছে। এতে অন্য, আগের একটি ঘরের নাম দিতে হবে।

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen { $other } ঘরের নাম দিয়েছে, যা এই প্লাগইনের Fields-এ নেই।

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen { $other } ঘরের নাম দিয়েছে, যা Fields-এ এর পরে আসে। এতে আগের কোনো ঘরের নাম দিতে হবে।

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen { $other } ঘরের নাম দিয়েছে, যার নিজেরও একটি ShowWhen আছে। একটি ঘর শুধু এমন ঘরের ওপর নির্ভর করতে পারে যার ShowWhen নেই।

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen { $other } সুইচের নাম দিয়েছে, তাই এর মান অবশ্যই true বা false হতে হবে।

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen-এর মান { $value }, যা { $other }-এর Choices-এর একটি নয়।

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen { $other } নোটের কথা বলেছে, যা কোনো সেটিং রাখে না। এর বদলে সুইচ, পছন্দ বা লেখার ঘর ব্যবহার করুন।

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen-এর জন্য [Plugin] Permissions-এ network অনুমতি দরকার।

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = [Network] Listen-এর জন্য [Plugin] Permissions-এ listen অনুমতি দরকার।

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = listen অনুমতির জন্য [Network] Listen দরকার, যাতে প্লাগইন যেসব পোর্টে সংযোগ নিতে পারবে সেগুলোর তালিকা দিতে হবে।

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = এর plugin.ini এমন একটি interface ব্যবহার করে যা Trindle-এর এই সংস্করণ সমর্থন করে না।

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = এতে শোনার জন্য কোনো পোর্টের তালিকা নেই।

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = এর plugin.ini পড়া যাচ্ছে না।

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = এটিকে এখনও শোনার অনুমতি দেওয়া হয়নি।

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = তালিকার পোর্টগুলো বদলে গেছে। সেগুলোতে আপনাকে আবার সংযোগের অনুমতি দিতে হবে।

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none =
    { $n ->
        [one] { $name }-কে কাজ করার আগে আপনার অনুমতি দিতে হবে। সে যে একটি অনুমতি চায় সেটি আপনি দেননি।
       *[other] { $name }-কে কাজ করার আগে আপনার অনুমতি দিতে হবে। সে যে { $n }টি অনুমতি চায় তার একটিও আপনি দেননি।
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } যে { $n }টি অনুমতি চায় তার মধ্যে আপনি শুধু { $a }টি দিয়েছেন।

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] এটি কিছুই চায় না।
        [one] এর যে অনুমতি দরকার তা দেওয়া আছে।
       *[other] এটি যে সব { $n }টি অনুমতি চায় তা দেওয়া আছে।
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = অনুমতি দিন

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = অন্য পোর্টে যোগাযোগ পাওয়া

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
plugins-swap-not-running = { $why } প্লাগইনটি ব্যাকগ্রাউন্ডে চলছে না: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle চালু করার জন্য trindle-plugin খুঁজে পায়নি।

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] এটি { $n } সেকেন্ডের মধ্যে চালু হয়নি।
       *[other] এটি { $n } সেকেন্ডের মধ্যে চালু হয়নি।
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = যোগ দেওয়ার মতো কোনো ডেস্কটপ সেশন চলছে না, তাই এটি চালু করা হয়নি। আগে লগ ইন করুন।

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = কোন ডেস্কটপ সেশনে যোগ দেবে তা বোঝা যাচ্ছে না ({ $sessions }), তাই এটি চালু করা হয়নি। WAYLAND_DISPLAY-তে সঠিক সেশনটি দিয়ে আবার চালান।

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = ফাইলের জায়গায় একটি ফোল্ডার আছে

# Error text when a plugin's id is not an id.
plugins-not-an-id = বৈধ id নয়

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts-এ { $n }টি শর্টকাট আছে। সর্বোচ্চ { $max }টি চলবে।
       *[other] [Plugin] Shortcuts-এ { $n }টি শর্টকাট আছে। সর্বোচ্চ { $max }টি চলবে।
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Shortcut id { $id }-এ শুধু অক্ষর, সংখ্যা, হাইফেন ও আন্ডারস্কোর থাকতে পারে।

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts-এ { $id } দুইবার আছে।

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts-এ { $id } আছে, যার কোনো [{ $group }] অংশ নেই।

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } আগে থেকেই Trindle-এর একটি শর্টকাট। অন্য কী বেছে নিন।

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } এই প্লাগইনের আরেকটি শর্টকাট আগেই ব্যবহার করছে।

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }]-এ Keys দরকার, যেমন Ctrl+Alt+S। { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = ওই কী সমন্বয় বৈধ শর্টকাট নয়। { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } আগে থেকেই Trindle-এর একটি শর্টকাট।

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } আগে থেকেই আপনার নিজের একটি শর্টকাট।

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } আগে থেকেই { $holder } ব্যবহার করছে।

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = এটি ইনস্টল করা কোনো প্লাগইনের শর্টকাট নয়।

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle এটি ব্যবহার করতে পারেনি ({ $why })।

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle সংরক্ষণ করতে পারেনি, কারণ { $why }। কিছুই বদলায়নি।

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = এটি সংরক্ষণ করা যায়নি: { $why }।

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = এর পোর্ট বন্ধ করা যায়নি: { $why }।

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle এর পোর্ট খুলতে পারেনি, তাই শোনা আবার বন্ধ করা হয়েছে: { $why }।

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = এর পোর্ট খোলা হয়নি ({ $why }), আর শোনাও আবার বন্ধ করা যায়নি ({ $kept })। কিছুই খোলা নেই। সেটিংসের অনুমতি অংশে এটি বন্ধ করুন।



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = অ্যাক্সেসিবিলিটির অনুমতি শুধু তালিকার Trindle স্ক্রিন রিডার ব্যবহার করতে পারে।

# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = { $name } অনুমতিটির নাম দুইবার দেওয়া হয়েছে।

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = { $home } অথবা network চান, দুটোই একসঙ্গে নয়: network ইন্টারনেটের পাশাপাশি স্থানীয় নেটওয়ার্কেও পৌঁছাতে পারে।

# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }]-এ Run দরকার, অর্থাৎ যে কমান্ড চালাতে হবে।

# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } পুরোনো সংস্করণটিও আগের জায়গায় ফেরানো যায়নি: { $back } দুটোই প্লাগইনের ফোল্ডারে রাখা আছে।
