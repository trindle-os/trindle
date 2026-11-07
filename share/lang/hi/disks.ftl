
# machine translated (Hindi), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = ड्राइव

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = इस पन्ने के बारे में

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = इस कंप्यूटर के अंदर या उससे जुड़ी हर ड्राइव, अपने हिस्सों की पट्टी के रूप में। हर हिस्सा उतना ही चौड़ा है जितना वह बड़ा है और जितना इस्तेमाल हुआ है उतना भरा है, इसलिए भरा हुआ हिस्सा आसानी से दिख जाता है।

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = चित्र के नीचे हर ड्राइव का एक कार्ड है। किसी हिस्से को खोलें, माउंट या अनमाउंट करें, या ड्राइव को सुरक्षित रूप से निकालें या बंद करें। हिस्से पर बनी पेंसिल उसके बदलाव खोलती है: उसका नाम, उसका अपना फ़ोल्डर, शुरू होते ही तैयार रहना, उसका आकार, उसे जाँचना और ठीक करना, फ़ॉर्मैट करना या मिटाना। खाली जगह पर प्लस नया हिस्सा बनाता है।

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = जिस ड्राइव से सिस्टम चलता है वह दिखती है, पर उस पर कोई ऐसा बदलाव नहीं सुझाया जाता जो चीज़ें मिटाए या हटाए। किसी भी ड्राइव में हर बदलाव के लिए हर बार प्रशासक का पासवर्ड माँगा जाता है। आँख किसी ड्राइव या हिस्से को Folders और रिंग से छिपा देती है: वह यहाँ धूसर दिखती रहती है और लॉक नहीं होती।

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = हर ड्राइव का हर हिस्सा, जितना इस्तेमाल हुआ है उतना भरा।

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = आपकी ड्राइव

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = ड्राइव अभी पढ़ी नहीं जा सकतीं।

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = फिर कोशिश करें

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = फिर देखें

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = कोई ड्राइव नहीं मिली। कोई लगाएँ तो वह यहाँ दिखेगी।

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = ड्राइव सेवा

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = सिस्टम बस

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } उपलब्ध नहीं है। इसे इंस्टॉल करें: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = ड्राइव देखी जा रही हैं…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } ख़राब हो रही है। जो रखना चाहते हैं उसे अभी इसमें से कॉपी कर लें।

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } को देखने की ज़रूरत है: { $troubles }।

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] ड्राइव और उन पर जगह।
        [one] ड्राइव और उन पर जगह, और एक ड्राइव लगी हुई।
       *[other] ड्राइव और उन पर जगह, और { $plugged } ड्राइव लगी हुई।
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] लगभग भरा: इस कंप्यूटर पर सिर्फ़ { $free } खाली है। जल्दी ही कुछ फ़ाइलें मिटाएँ या हटाएँ।
        [one] लगभग भरा: इस कंप्यूटर पर सिर्फ़ { $free } खाली है, और एक ड्राइव लगी हुई है। जल्दी ही कुछ फ़ाइलें मिटाएँ या हटाएँ।
       *[other] लगभग भरा: इस कंप्यूटर पर सिर्फ़ { $free } खाली है, और { $plugged } ड्राइव लगी हुई हैं। जल्दी ही कुछ फ़ाइलें मिटाएँ या हटाएँ।
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] भरने लगा है: इस कंप्यूटर पर { $free } खाली है।
        [one] भरने लगा है: इस कंप्यूटर पर { $free } खाली है, और एक ड्राइव लगी हुई है।
       *[other] भरने लगा है: इस कंप्यूटर पर { $free } खाली है, और { $plugged } ड्राइव लगी हुई हैं।
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] भरपूर जगह: इस कंप्यूटर पर { $free } खाली है।
        [one] भरपूर जगह: इस कंप्यूटर पर { $free } खाली है, और एक ड्राइव लगी हुई है।
       *[other] भरपूर जगह: इस कंप्यूटर पर { $free } खाली है, और { $plugged } ड्राइव लगी हुई हैं।
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = यह कंप्यूटर

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = आपकी फ़ाइलें

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } अब भी खाली है, जहाँ सिस्टम है।
       *[files] { $free } अब भी खाली है, आपकी अपनी फ़ाइलों के लिए।
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } भरा, { $whole } में से { $free } खाली

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = { $whole } में से

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = कोई रीडिंग नहीं, ड्राइव अपनी हालत नहीं बतातीं

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = स्वस्थ, जैसा ड्राइव खुद बताती है

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] स्वस्थ, जैसा सभी { $n } ड्राइव बताती हैं
       *[other] स्वस्थ, जैसा सभी { $n } ड्राइव बताती हैं
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } को देखने की ज़रूरत है, उसका कार्ड देखें
       *[other] { $n } को देखने की ज़रूरत है, उनके कार्ड देखें
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 ड्राइव, कुल { $total }। { $health }।
       *[other] { $n } ड्राइव, कुल { $total }। { $health }।
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } खाली

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = माउंट नहीं है

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = खाली जगह

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = इसमें कोई डिस्क या कार्ड नहीं है

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = इस पर अभी कुछ नहीं है

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB या निकालने योग्य

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, इस कंप्यूटर के अंदर

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB या निकालने योग्य, छिपी हुई

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, इस कंप्यूटर के अंदर, छिपी हुई

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } भरा, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }।

# Name given to a part that has no label and no device name.
disks-part-name = हिस्सा

# Name given to a drive that gives no maker or model.
disks-drive-name = ड्राइव

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = स्वैप

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = एन्क्रिप्टेड, खुला

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = एन्क्रिप्टेड, बंद

# Kind of a part that holds other parts.
disks-kind-container = दूसरे हिस्से रखता है

# Kind of a part that has no filesystem on it.
disks-kind-none = कोई फ़ाइल सिस्टम नहीं

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, वह हिस्सा जिससे कंप्यूटर शुरू होता है
        [legacy] { $kind }, वह हिस्सा जिससे पुराना कंप्यूटर शुरू होता है
       *[loader] { $kind }, बूट लोडर का अपना हिस्सा
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = कोई भी कंप्यूटर

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = सिर्फ़ Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = पुराने उपकरण

# Trouble a drive reports about its spare space.
disks-trouble-spare = अतिरिक्त जगह कम पड़ रही है

# Trouble a drive reports about its heat.
disks-trouble-temperature = बहुत गर्म है

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = घिस रही है

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = खुद केवल-पढ़ने योग्य हो गई है

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = उसकी मेमोरी का बैकअप विफल हो गया है

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = उसका एक हिस्सा खुद केवल-पढ़ने योग्य हो गया है

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = निर्माता की अपेक्षा से ज़्यादा घिस चुकी है

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } ख़राब जगह
       *[other] { $n } ख़राब जगहें
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] { $hours } घंटा इस्तेमाल हुई
       *[other] { $hours } घंटे इस्तेमाल हुई
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = ख़राब हो रही है: जो रखना चाहते हैं उसे अभी इसमें से कॉपी कर लें

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = ध्यान चाहिए: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = स्वस्थ

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = कोई ड्राइव जवाब नहीं दे रही

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = डिब्बा जवाब देता है, पर उसके अंदर की ड्राइव नहीं। उसे निकालें, दस सेकंड रुकें और अपनी केबल से नीले या USB-C पोर्ट में लगाएँ; अगर वह ऐसी ही रहे, तो अंदर की ड्राइव ढीली हो गई होगी या ख़राब हो गई होगी।

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = सुरक्षित रूप से निकालें

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = बंद करें

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = पूरी ड्राइव मिटाएँ

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] इस ड्राइव को दिखाएँ
       *[part] इस हिस्से को दिखाएँ
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] इस ड्राइव को छिपाएँ। { $lock }
       *[part] इस हिस्से को छिपाएँ। { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Folders और रिंग से छिपी हुई। { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = सिस्टम इसी से चलता है: यहाँ कुछ भी मिटाया नहीं जा सकता।

# Note on the card of a drive that has no partition table.
disks-no-table = कोई पार्टीशन तालिका नहीं: पूरी ड्राइव खाली है।

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = माउंट नहीं है

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = शुरू होते ही { $dir } में माउंट होता है

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = { $dir } में दिखता है

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = { $dir } में

# Tip of the round button that opens the folder a part is mounted in.
disks-open = खोलें

# Tip of the round button that mounts a part.
disks-mount = माउंट करें

# Tip of the round button that unmounts a part.
disks-unmount = अनमाउंट करें

# Tip of the round button on a part that opens the box of changes.
disks-change-it = इसे बदलें

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = नाम

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = नाम (वैकल्पिक)

# Tip of the button that saves a part's new name.
disks-rename-tip = नाम बदलें

# Note under the Name row of a part.
disks-name-note = यहाँ और दूसरे कंप्यूटरों पर इसे इस नाम से पुकारा जाता है।

# Placeholder of the typing box for a part's folder.
disks-folder-name = फ़ोल्डर का नाम

# Tip of the button that saves a part's folder.
disks-use-folder = यह फ़ोल्डर इस्तेमाल करें

# Label of the row where a part's folder is chosen.
disks-folder = फ़ोल्डर

# Note under the Folder row of a part.
disks-folder-note = इसकी फ़ाइलें कहाँ दिखती हैं। खाली छोड़ें तो कंप्यूटर चुन लेगा।

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = शुरू होते ही माउंट करें

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = कंप्यूटर चालू होते ही तैयार। अगर यह न मिले, तब भी कंप्यूटर चालू हो जाता है।

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = GB में आकार

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = { $least } से { $most } तक।

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = { $least } से { $most } तक। कितना छोटा हो सकता है यह देखने के लिए इसे माउंट करें।

# Tip of the button that resizes a part.
disks-resize-tip = आकार बदलें

# Label of the row where a part is resized.
disks-size = आकार

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = इससे { $words } पर सब कुछ मिट जाएगा। इसे वापस नहीं लिया जा सकता।

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = फिर क्लिक करें: { $drive } पर { $name } { $from } से { $to } हो जाएगा

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = नए हिस्से के लिए जगह बनाएँ

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = GB में जगह

# Tip of the button that makes room under a part that is in use.
disks-room-tip = जगह बनाएँ

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = फिर क्लिक करें: स्नैपशॉट के बाद { $drive } पर { $name } से { $size } कम हो जाएगा

# Note under the row that makes room.
disks-room-note = इस्तेमाल के दौरान, स्नैपशॉट के बाद छोटा किया जाता है, और खाली जगह छोड़ी जाती है।

# Tip of the button that checks a part for faults.
disks-check-tip = ख़राबियों के लिए जाँचें

# Tip of the button that repairs a part.
disks-repair-tip = ठीक करें

# Label of the row where a part is checked and repaired.
disks-faults = ख़राबियाँ

# Note under the Faults row.
disks-faults-note = देखता है कि इसकी फ़ाइलें रखने के तरीके में कोई ख़राबी तो नहीं, और उसे ठीक कर देता है। इसके चलने के दौरान यह अनमाउंट रहता है।

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = { $drive } पर { $name } को ठीक करने के लिए फिर क्लिक करें

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = प्रकार

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = फ़ॉर्मैट करें

# Tip of the button that formats a part.
disks-format-tip = फ़ॉर्मैट करें

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = इसे एक हिस्से के रूप में नए सिरे से शुरू करता है, पूरी ड्राइव भरकर।

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = सिर्फ़ इस हिस्से पर सब कुछ मिटा देता है।

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = फिर क्लिक करें: { $words } पर सब कुछ मिट जाएगा

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, { $drive } पर

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = { $drive } पर { $name }, { $size } को मिटाएँ

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = इस हिस्से को मिटाएँ

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = इसकी जगह खाली हो जाती है, नए हिस्से के लिए या इससे पहले वाले हिस्से के बढ़ने के लिए तैयार।

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = यहाँ नया हिस्सा बनाएँ

# Label of the row where a new part is made.
disks-new-part = नया हिस्सा

# Note under the row where a new part is made.
disks-new-part-note = इस खाली जगह में बनाया जाता है, और इस्तेमाल के लिए फ़ॉर्मैट किया जाता है।

# Tip of the button that makes a new part.
disks-make-tip = बनाएँ

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = { $drive } पर { $size } का हिस्सा बनाने के लिए फिर क्लिक करें

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = { $name } को मिटाएँ

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, { $drive } पर

# Title of the box for making a new part in free space.
disks-sheet-new = एक नया हिस्सा

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } खाली, { $drive } पर

# Tip of the round button that closes the box of changes.
disks-close = बंद करें

# Words on a row while a part is being mounted.
disks-mounting = माउंट हो रहा है

# Words on a row when a part has been mounted.
disks-mounted = माउंट हो गया

# Words on a row while a part is being unmounted.
disks-unmounting = अनमाउंट हो रहा है

# Words on a row when a part has been unmounted.
disks-unmounted = अनमाउंट हो गया

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = इसे बंद किया जा रहा है

# Words on a drive's row when it is safe to remove.
disks-safe-remove = निकालना सुरक्षित है

# Words on a drive's row while it is being powered off.
disks-powering-off = बंद हो रहा है

# Words on a drive's row when it has been powered off.
disks-off-safe = बंद: निकालना सुरक्षित है

# Words on a row while a part is being renamed.
disks-renaming = नाम बदला जा रहा है

# Words on a row when a part has been renamed.
disks-renamed = नाम बदल गया

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = कंप्यूटर फिर से इसका फ़ोल्डर चुनता है

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = यह { $dir } में दिखता है

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = शुरू होते ही { $dir } में माउंट होता है

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = शुरू होते समय अकेला छोड़ा गया

# Words on a row while a folder or start-up choice is being saved.
disks-saving = सहेजा जा रहा है

# Words on a row while a part is being resized.
disks-resizing = आकार बदला जा रहा है

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = आकार बदलकर { $size } हुआ

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = फ़ाइलें नहीं मिलीं, इसलिए जगह नहीं बनाई जा सकती।

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = स्नैपशॉट लिया जा रहा है

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = स्नैपशॉट बाद में नहीं मिला।

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } कुछ भी आकार में नहीं बदला।

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = स्नैपशॉट लेते समय चीज़ें बदल गईं, इसलिए कुछ भी आकार में नहीं बदला।

# Words on a row while room is being made.
disks-making-room = जगह बनाई जा रही है

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } जगह बन गई

# Words on a row while a part is being repaired.
disks-repairing = ठीक किया जा रहा है

# Words on a row when a part has been repaired.
disks-repaired = ठीक हो गया

# Red words when some faults could not be put right.
disks-repair-failed = कुछ ख़राबियाँ ठीक नहीं हो सकीं। जो रखना चाहते हैं उसे कॉपी कर लें।

# Words on a row while a part is being checked.
disks-checking = जाँचा जा रहा है

# Words on a row when the check found no faults.
disks-no-faults = कोई ख़राबी नहीं मिली

# Red words when the check found faults.
disks-faults-found = ख़राबियाँ मिलीं: ठीक करने के लिए रिंच दबाएँ।

# Words on a row while a part or drive is being formatted.
disks-formatting = फ़ॉर्मैट हो रहा है

# Words on a row when a part has been formatted.
disks-formatted = फ़ॉर्मैट हो गया

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = पूरी ड्राइव मिटाई जा रही है

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = नया हिस्सा नहीं मिला।

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = ड्राइव सेवा तक पहुँचा नहीं जा सका।

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = नए हिस्से की पहचान पढ़ी नहीं जा सकी।

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = मिटाया जा रहा है

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = मिटा दिया गया

# Words on a row while a new part is being made.
disks-making-it = बनाया जा रहा है

# Words on a row when a new part has been made.
disks-made = बन गया

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = यह रखा नहीं जा सका: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = पहले इसे कोई नाम दें।

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] यह नाम बहुत लंबा है: ज़्यादा से ज़्यादा { $max } अक्षर।
       *[other] यह नाम बहुत लंबा है: ज़्यादा से ज़्यादा { $max } अक्षर।
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = सिर्फ़ अक्षर, अंक, - और _ इस्तेमाल करें (खाली जगह नहीं)।

# Red words under the folder name box when it starts with a dash.
disks-name-start = इसे किसी अक्षर या अंक से शुरू करें।

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } पहले से इस्तेमाल में है।

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] इस प्रकार के लिए यह नाम बहुत लंबा है: ज़्यादा से ज़्यादा { $max } अक्षर।
       *[other] इस प्रकार के लिए यह नाम बहुत लंबा है: ज़्यादा से ज़्यादा { $max } अक्षर।
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT नामों में * ? . , ; : / \ | + = < > [ ] या " नहीं हो सकते।

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] पहले इसे माउंट करें: इस प्रकार को माउंट रहते हुए ही बड़ा किया जा सकता है।
       *[smaller] पहले इसे माउंट करें: इस प्रकार को माउंट रहते हुए ही छोटा किया जा सकता है।
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] इस प्रकार को बड़ा नहीं किया जा सकता।
       *[smaller] इस प्रकार को छोटा नहीं किया जा सकता।
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = पासवर्ड नहीं दिया गया, इसलिए कुछ नहीं बदला।

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = पासवर्ड नहीं दिया गया, इसलिए काम बीच में रुक गया।

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = एक और बदलाव अभी हो रहा है। उसके पूरा होने पर फिर कोशिश करें।

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = यह पक्का नहीं किया जा सका कि पासवर्ड माँगा जाएगा, इसलिए कुछ नहीं बदला।

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = अभी दिया गया पासवर्ड वापस नहीं लिया जा सका, इसलिए कुछ मिनट तक ड्राइव में एक और बदलाव पासवर्ड न माँगे। इसे फिर आज़माया जा रहा है।

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = दिया गया पासवर्ड अब वापस ले लिया गया है: अगला बदलाव पासवर्ड माँगेगा।

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = ड्राइव सेवा तक पहुँचा नहीं जा सका, इसलिए कुछ नहीं बदला।

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = यह बदलाव होते समय Drives बंद कर दिया गया।

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = जब तक यह पूरा नहीं होता Drives खुला रहता है, ताकि वह बता सके कि यह कैसे खत्म हुआ।

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, इसलिए कुछ नहीं बदला।

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, इसलिए काम बीच में रुक गया।

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = ड्राइव सेवा से पूछा नहीं जा सका

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } काम बीच में रुक गया।

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }।

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = हो गया: { $steps }।

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = विफल: { $steps }।

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = नहीं हुआ: { $steps }।

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = पता नहीं: { $steps }।

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = इसके लिए { $dev } को अनमाउंट किया गया था और अब वह माउंट नहीं है।

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = इसके लिए { $dev } को अनमाउंट किया गया था और वह अब भी अनमाउंट है: उसकी पंक्ति से माउंट करें।

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = हो सकता है { $dev } अनमाउंट हो गया हो: उसकी पंक्ति देखें।

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = इसके लिए { $dev } को अनमाउंट किया गया था, और उसे फिर माउंट करने के लिए अलग से पासवर्ड चाहिए: उसकी पंक्ति से माउंट करें।

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = इसके लिए { $dev } को अनमाउंट किया गया था और उसे फिर माउंट नहीं किया जा सका ({ $why }): उसकी पंक्ति से माउंट करें।

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = फ़ॉर्मैट हो गया, पर उसे उसका फ़ोल्डर { $dir } फिर से नहीं दिया गया। { $why } उसकी पंक्ति से फ़ोल्डर फिर चुनें।

# Red words when a drive is in use and cannot be unmounted.
disks-busy = अब भी कोई चीज़ इसे इस्तेमाल कर रही है। इसकी फ़ाइलें दिखाने वाली विंडो या प्रोग्राम बंद करें, फिर कोशिश करें।

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = इस आकार पर इसमें जो कुछ है उसके लिए पर्याप्त जगह नहीं है।

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } को अनमाउंट करना

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } को फिर माउंट करना

# Step of a change.
disks-step-eject = निकाली जाने वाली ड्राइव को बंद करना

# Step of a change.
disks-step-power-off = ड्राइव को बंद करना

# Step of a change. { $dev } is the part.
disks-step-format = { $dev } को फ़ॉर्मैट करना

# Step of a change.
disks-step-create = नया हिस्सा बनाना

# Step of a change. { $dev } is the part.
disks-step-delete = { $dev } को मिटाना

# Step of a change. { $dev } is the part.
disks-step-resize-part = हिस्से { $dev } का आकार बदलना

# Step of a change. { $dev } is the part.
disks-step-resize = { $dev } पर जो है उसका आकार बदलना

# Step of a change. { $dev } is the part.
disks-step-repair = { $dev } को ठीक करना

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } को जाँचना

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } का नाम बदलना

# Step of a change. { $dev } is the part.
disks-step-fstab = { $dev } का फ़ोल्डर और शुरू होने का चुनाव सहेजना

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = { $dev } पर चरण { $method }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = इस { $kind } हिस्से को सिर्फ़ तब छोटा किया जा सकता है जब वह इस्तेमाल में न हो, इसलिए यहाँ से जगह नहीं बनाई जा सकती।

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = इस xfs हिस्से को छोटा नहीं किया जा सकता।

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = इस { $kind } हिस्से को इस्तेमाल के दौरान छोटा नहीं किया जा सकता।

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = स्नैपशॉट सिर्फ़ सिस्टम की अपनी डिस्क का लिया जा सकता है, इसलिए यहाँ जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-not-in-table = यह ड्राइव की तालिका का हिस्सा नहीं है, इसलिए इससे जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-no-online-shrink = ड्राइव सेवा यहाँ Btrfs को इस्तेमाल के दौरान छोटा नहीं कर सकती, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-holds-parts = इसमें दूसरे हिस्से हैं, इसलिए इससे जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-not-gpt = इसकी ड्राइव पर GPT तालिका नहीं है, इसलिए इससे जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-mount-first = पहले इसे माउंट करें: Btrfs माउंट रहते हुए छोटा किया जाता है।

# Red words under the Make room row.
disks-room-mounts-not-one = यह ठीक एक जगह माउंट नहीं है, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-read-only = यह केवल-पढ़ने के लिए माउंट है, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-read-only-unknown = पता नहीं चल सका कि यह केवल-पढ़ने योग्य है या नहीं, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-many-drives = फ़ाइलें एक से ज़्यादा ड्राइव पर फैली हैं, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-drives-unknown = पता नहीं चल सका कि फ़ाइलें कितनी ड्राइव पर फैली हैं, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-swap-file = इस पर एक स्वैप फ़ाइल है, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-swap-unknown = पता नहीं चल सका कि इस पर स्वैप फ़ाइल है या नहीं, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-used-unknown = इसका कितना हिस्सा इस्तेमाल में है यह पढ़ा नहीं जा सका, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-area-unknown = ड्राइव सेवा नहीं बताती कि फ़ाइलों का क्षेत्र कितना बड़ा है, इसलिए जाँच नहीं हो सकी और जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-open-missing = इसकी खुली प्रति नहीं मिली, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-encryption-sizes = एन्क्रिप्शन के लिए दिए गए आकार आपस में मेल नहीं खाते, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row.
disks-room-sizes = इसके लिए दिए गए आकार आपस में मेल नहीं खाते, इसलिए जगह नहीं बनाई जा सकती।

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = इसमें { $used } है और उससे ज़्यादा रखना ज़रूरी है, इसलिए बहुत कम खाली किया जा सकता है।

# Red words under the Make room row.
disks-room-sizes-unread = इसके आकार पढ़े नहीं जा सके।

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = { $least } से { $most } तक जगह बनाई जा सकती है।

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, इसलिए काम रोक दिया गया।

# What was found wrong while making room.
disks-room-check-unmounted = फ़ाइलें अब माउंट नहीं हैं

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = हिस्सा { $is } है और { $should } होना चाहिए

# What was found wrong while making room.
disks-room-check-open = एन्क्रिप्शन की खुली प्रति उस आकार की नहीं है जिसकी होनी चाहिए

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = फ़ाइलों का क्षेत्र { $is } है और { $should } होना चाहिए

# What was found wrong while making room.
disks-room-check-area-small = फ़ाइलों का क्षेत्र इस्तेमाल में आ रही चीज़ों से छोटा है

# What was found wrong while making room.
disks-room-check-fit = जो चीज़ें जिसमें रखी हैं वह अब उनमें समाती नहीं

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] ड्राइव सेवा अब हिस्से को सूचीबद्ध नहीं करती, इसलिए काम रोक दिया गया।
        [copy] ड्राइव सेवा अब खुली प्रति को सूचीबद्ध नहीं करती, इसलिए काम रोक दिया गया।
       *[files] ड्राइव सेवा अब फ़ाइल सिस्टम को सूचीबद्ध नहीं करती, इसलिए काम रोक दिया गया।
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = फ़ाइलों के क्षेत्र को छोटा करना

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = एन्क्रिप्शन की खुली प्रति को छोटा करना

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = हिस्से को छोटा करना

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = आकारों की जाँच करना

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = { $dev } के नीचे जगह बनाने से पहले ({ $at })


# Red words when another change is pressed while one is still being made.
disks-change-going = एक बदलाव किया जा रहा है।


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = फिर क्लिक करें: { $drive } पर { $name } { $from } से { $to } हो जाएगा। पहले उसकी फ़ाइलों का बैकअप लें, क्योंकि छोटा होते समय बिजली जाने पर वे खो सकती हैं।

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name } पर Windows हाइबरनेट था या फ़ास्ट स्टार्ट-अप में छोड़ा गया था, इसलिए उसे छोटा करने पर उसकी फ़ाइलें खो सकती हैं। Windows शुरू करें, रीस्टार्ट चुनें, फिर दोबारा कोशिश करें।

