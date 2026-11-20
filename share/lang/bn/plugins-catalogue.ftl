
# machine translated (Bengali), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = প্লাগইনের তালিকা নষ্ট হয়েছে, তাই প্লাগইন আপডেট থেমে আছে।

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = আবার শুরু করুন

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle রেকর্ডটি সরাতে পারেনি: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = একটি নামে null বাইট আছে।

# Error text when there is no record name to take away.
plugins-start-no-note = সরানোর মতো কোনো রেকর্ড নেই।

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = যাচাইয়ের কী না থাকায় Trindle প্লাগইন তালিকা যাচাই করতে পারছে না।

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = Trindle-এর প্লাগইন তালিকায় কোনো স্বাক্ষর নেই, তাই Trindle এটি ব্যবহার করেনি।

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = Trindle-এর প্লাগইন তালিকার স্বাক্ষরের আকার ঠিক নয়, তাই Trindle তালিকাটি ব্যবহার করেনি।

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = Trindle-এর প্লাগইন তালিকা Trindle-এর কী দিয়ে স্বাক্ষর করা নয়, তাই Trindle এটি ব্যবহার করেনি।

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle-এর প্লাগইন তালিকায় কোনো সংস্করণ নম্বর নেই, তাই Trindle এটি ব্যবহার করেনি।

# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
plugins-refused-older = এই প্লাগইন তালিকা আগে ব্যবহার করা তালিকার চেয়ে পুরোনো, তাই Trindle এটি ব্যবহার করেনি।

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle-এর প্লাগইন তালিকা স্বাক্ষরিত, কিন্তু পড়া যায় না, তাই Trindle এটি ব্যবহার করেনি: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } সর্বশেষ ভালো তালিকাটি দেখানো হচ্ছে।

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = এটি লেখা নয়।

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = তালিকাটি খুব বড়।

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = এটি প্লাগইন তালিকা নয়, কারণ { $file }-এ Format সহ কোনো [Catalogue] অংশ নেই।

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = এই তালিকা Trindle-এর এই সংস্করণের পড়তে পারা ফরম্যাটের চেয়ে নতুন ফরম্যাট ব্যবহার করে। Trindle Format={ $format } সমর্থন করে।

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
plugins-unpack-tar = tar চালু হয়নি: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle আর্কাইভটি খুলতে পারেনি: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = কারণ জানা নেই

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = আর্কাইভের সবচেয়ে ওপরের স্তরে, বা ওপরের স্তরের একটি ফোল্ডারে, কোনো { $file } নেই।

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle একটি অস্থায়ী ফোল্ডার তৈরি করতে পারেনি: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl চালু হয়নি: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = এটি ডাউনলোড করা যায়নি: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = ডাউনলোড করা ফাইল তালিকার চেকসামের (SHA-256) সঙ্গে মেলেনি, তাই Trindle কিছু ইনস্টল করেনি।

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = প্লাগইনটি যাচাইয়ে ব্যর্থ হয়েছে, তাই Trindle কিছু ইনস্টল করেনি: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = আর্কাইভে { $wanted } নয়, { $found } আছে, তাই Trindle কিছু ইনস্টল করেনি।

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle প্লাগইনটি ইনস্টল করেছে, কিন্তু এটি যে Trindle-এর তালিকা থেকে এসেছে তা নথিভুক্ত করতে পারেনি: { $why }।

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = এটি ইনস্টল করা নেই, তাই আপডেট করার কিছু নেই।
