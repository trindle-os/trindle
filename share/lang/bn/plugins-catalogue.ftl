
# machine translated (Bengali), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = প্লাগইন আপডেট থেমে আছে: প্লাগইনের তালিকা নষ্ট হয়েছে।

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = আবার শুরু করুন

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = রেকর্ডটি সরানো যায়নি: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = একটি নামে nul ছিল।

# Error text when there is no record name to take away.
plugins-start-no-note = সরানোর মতো কোনো নোট নেই।

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = এই Trindle-এর কাছে প্লাগইন তালিকা যাচাইয়ের কোনো কী নেই, তাই এটি কোনো তালিকা মানে না।

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = Trindle-এর প্লাগইন তালিকায় কোনো স্বাক্ষর নেই, তাই এটি ব্যবহার করা হয়নি।

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Trindle-এর প্লাগইন তালিকার স্বাক্ষরের আকার ঠিক নয়, তাই তালিকাটি ব্যবহার করা হয়নি।

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Trindle-এর প্লাগইন তালিকা Trindle-এর কী দিয়ে স্বাক্ষর করা নয়, তাই এটি ব্যবহার করা হয়নি।

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle-এর প্লাগইন তালিকায় এর নম্বর লেখা নেই, তাই এটি ব্যবহার করা হয়নি।

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle-এর প্লাগইন তালিকাটি আগে গ্রহণ করা তালিকার চেয়ে পুরোনো (নম্বর { $is }, আগে { $after }), তাই এটি ব্যবহার করা হয়নি।

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle-এর প্লাগইন তালিকা স্বাক্ষরিত, কিন্তু পড়া যায় না, তাই এটি ব্যবহার করা হয়নি: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } সর্বশেষ ভালো তালিকাটি দেখানো হচ্ছে।

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = এটি লেখা নয়।

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = তালিকাটি এত বড় যে তালিকা হতে পারে না।

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = এটি প্লাগইন তালিকা নয়: { $file }-এ Format সহ কোনো [Catalogue] অংশ নেই।

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = তালিকাটি এই Trindle-এর পড়তে পারা ধরনের চেয়ে নতুন (Format={ $format })।

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } বাদ দেওয়া হয়েছে: { $why ->
        [id] এর id অক্ষর, সংখ্যা, ডট ও ড্যাশ দিয়ে তৈরি নয়
        [needs] এর একটি Name ও একটি Version দরকার
        [archive] এর Archive তালিকার পাশে থাকা কোনো .tar.zst ফাইলের নাম নয়
        [hash] এর SHA256 ৬৪টি হেক্স অঙ্ক নয়
        [permission] এটি এমন অনুমতি চায় যা নেই
       *[icon] এর Icon তালিকার পাশে থাকা কোনো SVG বা PNG ফাইলের নাম নয়
    }।

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar চালু করা যায়নি: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = আর্কাইভটি খোলা যায়নি: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar এটি মানেনি

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = আর্কাইভের একদম ওপরে, বা ওপরের একটি ফোল্ডারে, কোনো { $file } নেই।

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = কোনো অস্থায়ী ফোল্ডার তৈরি করা যায়নি: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl চালু করা যায়নি: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = এটি ডাউনলোড করা যায়নি: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = এর ফাইল তালিকায় থাকা ফাইলটি নয় (SHA-256 আলাদা), তাই কিছু ইনস্টল করা হয়নি।

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = এটি Trindle-এর প্লাগইন যাচাই পাস করেনি, তাই কিছু ইনস্টল করা হয়নি: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = আর্কাইভে { $wanted } নয়, { $found } আছে, তাই কিছু ইনস্টল করা হয়নি।

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = এটি বসানো হয়েছে, কিন্তু এটি যে Trindle-এর তালিকা থেকে এসেছে সেই নোট লেখা যায়নি: { $why }।

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = এটি ইনস্টল করা নেই, তাই আপডেট করার কিছু নেই।

