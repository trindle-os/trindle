
# machine translated (Hindi), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = { $name } इंस्टॉल करें

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = स्वागत

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = डिस्क

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = गोपनीयता

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = पासवर्ड

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = आप

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = तैयार

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = इंस्टॉल हो रहा है

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = पूरा

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = तैयारी हो रही है

# Red line at the foot when the installer's helper could not be started.
installer-no-start = इंस्टॉलर शुरू नहीं हो सका: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = योजना पढ़ी नहीं जा सकी।

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = योजना पढ़ी नहीं जा सकी: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = इंस्टॉलेशन रुक गया। इसके बाद कुछ और नहीं लिखा गया है।

# Name of the round button at the foot that goes back a page.
installer-back = पीछे

# Name of the main button at the foot that goes to the next page.
installer-next = आगे

# Name of the main button at the foot of the Ready page that starts the install.
# SAFETY: needs a human review before it ships
installer-install = इंस्टॉल करें

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = अभी रीस्टार्ट करें

# Beside the keyboard test box on the first page.
installer-try-keyboard = कीबोर्ड आज़माएँ

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = कीबोर्ड आज़माने के लिए यहाँ लिखें

# Note beside Install after one press, when the second press writes beside another system.
# SAFETY: needs a human review before it ships
installer-arm = इंस्टॉल करने के लिए फिर क्लिक करें

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = मिटाकर इंस्टॉल करने के लिए फिर क्लिक करें

# Note beside Next on the first page while no language is chosen.
installer-missing-language = भाषा चुनें

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = कीबोर्ड चुनें

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = समय चुनें

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = चुनें कि Trindle कहाँ जाए

# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = डिस्क का पासवर्ड दो बार लिखें, दोनों बार एक जैसा

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = साइन-इन नाम चाहिए

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = इस कंप्यूटर को एक नाम दें

# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = अपना पासवर्ड दो बार लिखें, दोनों बार एक जैसा

# Title of the first page.
installer-welcome-title = { $name } में आपका स्वागत है

# Sentence under the title of the first page.
installer-welcome-help = अपनी भाषा चुनें; कीबोर्ड और समय उसके साथ चलते हैं, और आप दोनों बदल सकते हैं।

# Head of the card for choosing the language, and the name of its search box.
installer-language = भाषा

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = कीबोर्ड

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = समय

# Under a card's head while nothing has been chosen in its list.
installer-select = चुनें

# Title of the disk page.
installer-disks-title = { $name } कहाँ जाएगा

# Sentence under the title of the disk page.
installer-disks-help = आपकी चुनी डिस्क पर जो कुछ है वह बदल दिया जाता है, जब तक आप वहाँ पहले से मौजूद चीज़ों के बगल में जगह न बना लें।

# Shown on the disk page when no disk is big enough.
installer-disks-none = इतनी बड़ी कोई डिस्क नहीं मिली। Trindle को 20 GB चाहिए।

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = पूरी डिस्क इस्तेमाल करें, उस पर सब कुछ बदलकर

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = { $what } को रखें और डिस्क साझा करें

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = अभी जो है उसके लिए { $kept } बचा, Trindle के लिए { $mine }

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = खाली जगह

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } और { $last }

# Under a disk's name: what is on it.
installer-disk-has = इस पर { $list } है

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = इस पर { $list } है, एन्क्रिप्टेड

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = इस पर एक एन्क्रिप्टेड सिस्टम है

# Under a disk's name: there is nothing on it.
installer-disk-empty = खाली

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = इस पर फ़ाइलें हैं, पर कोई ऐसा सिस्टम नहीं जिसे Trindle पहचानता हो

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = { $gb } GB की स्वैप फ़ाइल

# Name of the switch that makes a swap file.
installer-swap-switch = स्वैप फ़ाइल बनाएँ

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot चालू है, इसलिए यह कंप्यूटर हाइबरनेट नहीं कर सकता, पर मेमोरी कम पड़ने पर स्वैप फ़ाइल फिर भी काम आती है। यह तभी बनती है जब डिस्क पर बहुत जगह बची हो।

# Under the swap file's head when the computer can hibernate.
installer-swap-note = हाइबरनेट करने की जगह, यानी बिजली बंद होने पर भी सब कुछ खुला रखना, और मेमोरी कम पड़ने पर काम आने की जगह। इसके बिना कंप्यूटर हाइबरनेट नहीं कर सकता। यह तभी बनती है जब डिस्क पर बहुत जगह बची हो।

# Title of the page about encrypting the disk.
installer-privacy-title = डिस्क को एन्क्रिप्ट करना

# Sentence under the title of that page.
installer-privacy-help = एन्क्रिप्टेड डिस्क को वह कोई नहीं पढ़ सकता जो उसे इस कंप्यूटर से निकाल ले।

# Head of the card with the three choices.
installer-privacy-locked = डिस्क कैसे लॉक होती है

# Name and words of the choice not to encrypt the disk.
installer-privacy-none = डिस्क को एन्क्रिप्ट न करें

# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = इसे एन्क्रिप्ट करें, और कंप्यूटर चालू होने पर पासवर्ड माँगें

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = इसे एन्क्रिप्ट करें, और इस कंप्यूटर की TPM चिप से खुलने दें

# Head of the card about the TPM chip.
installer-privacy-about = चिप के बारे में

# In the card about the chip, on a computer that has one.
installer-privacy-chip = इस कंप्यूटर में TPM चिप है। इंस्टॉल के बाद पहली बार चालू करने पर एक बार पासवर्ड माँगा जाता है, जब चिप इस कंप्यूटर को सीखती है; उसके बाद वह डिस्क खुद खोल देती है और कुछ लिखना नहीं पड़ता। पासवर्ड संभालकर रखें: चिप बदली जाने या डिस्क दूसरे कंप्यूटर में ले जाने पर अंदर आने का यही रास्ता है।

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = इस कंप्यूटर में Trindle के इस्तेमाल लायक कोई TPM चिप नहीं है, इसलिए हर बार चालू होने पर पासवर्ड माँगा जाएगा। जिस कंप्यूटर में चिप हो, उसमें चिप डिस्क खुद खोल सकती है।

# Title of the disk password page.
installer-disk-title = डिस्क का पासवर्ड

# Sentence under the title of the disk password page.
installer-disk-help = कंप्यूटर चालू होने पर यह सबसे पहले लिखा जाता है।

# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = डिस्क का पासवर्ड

# Name of the round button that makes up a passphrase.
installer-disk-suggest = पासफ़्रेज़ सुझाएँ

# Label and name of the second box of a password, where it is typed again.
installer-same-again = वही फिर से

# Red line under the password boxes when the two do not match.
installer-mismatch = दोनों पासवर्ड मेल नहीं खाते।

# Head of the card that tells the person to write the password down.
installer-disk-write-down = इसे लिख लें

# Words in that card.
installer-disk-write-down-text = इस पासवर्ड को कागज़ पर लिख लें और कंप्यूटर से दूर किसी सुरक्षित जगह रखें। इसके बिना डिस्क कभी नहीं खोली जा सकती, न आपसे, न किसी और से। वापस अंदर आने का कोई रास्ता नहीं है। चार आम शब्द एक ऐसा पासफ़्रेज़ बनाते हैं जिसे लिखना आसान और अंदाज़ा लगाना मुश्किल है।

# Title of the account page.
installer-you-title = आपके बारे में

# Sentence under the title of the account page.
installer-you-help = यह वह खाता है जिससे आप साइन इन करते हैं।

# Head of the card with the name boxes.
installer-account = आपका खाता

# Label and name of the box for the person's name.
installer-your-name = आपका नाम

# Label and name of the box for the sign-in name.
installer-user-name = साइन-इन नाम

# Label and name of the box for the computer's name.
installer-host-name = इस कंप्यूटर का नाम

# Head of the card with the password boxes.
installer-your-password = आपका पासवर्ड

# Label and name of the first password box.
installer-password = पासवर्ड

# Label of the strength meter and the name a screen reader gives it.
installer-strength = अंदाज़ा लगाना कितना कठिन है

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = साइन-इन नाम चाहिए।

# Red line under the account boxes: a name is too long.
installer-name-too-long = वह नाम बहुत लंबा है।

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = छोटे अक्षर से शुरू करें।

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = केवल छोटे अक्षर, अंक, - और _।

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = वह नाम सिस्टम पहले से इस्तेमाल करता है।

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = इस कंप्यूटर को एक नाम दें।

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = केवल अक्षर, अंक और -।

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = कंप्यूटर इसका अंदाज़ा तुरंत लगा लेगा।

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] कंप्यूटर इसका अंदाज़ा लगभग { $n } मिनट में लगा लेगा।
       *[other] कंप्यूटर इसका अंदाज़ा लगभग { $n } मिनट में लगा लेगा।
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] कंप्यूटर इसका अंदाज़ा लगभग { $n } घंटे में लगा लेगा।
       *[other] कंप्यूटर इसका अंदाज़ा लगभग { $n } घंटे में लगा लेगा।
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] कंप्यूटर इसका अंदाज़ा लगभग { $n } दिन में लगा लेगा।
       *[other] कंप्यूटर इसका अंदाज़ा लगभग { $n } दिन में लगा लेगा।
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] कंप्यूटर को इसका अंदाज़ा लगाने में लगभग { $n } साल लगेंगे।
       *[other] कंप्यूटर को इसका अंदाज़ा लगाने में लगभग { $n } साल लगेंगे।
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = कंप्यूटर को इसका अंदाज़ा लगाने में दर्ज इतिहास से भी ज़्यादा समय लगेगा।

# Title of the Ready page.
installer-ready-title = इंस्टॉल के लिए तैयार

# Head of the card that lists what will happen.
installer-ready-nothing-changed = अभी कुछ भी बदला नहीं गया है

# Head of the card about security updates.
installer-ready-security = सुरक्षा अपडेट

# In the card about security updates.
installer-ready-security-text = { $name } Debian के सुरक्षा सुधार हर दिन अपने-आप इंस्टॉल करता है। इसे बंद नहीं किया जा सकता।

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = { $disk } पर सब कुछ बदल दिया जाएगा।

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = { $part } पर सब कुछ रखा जाएगा, पर पहले उसका बैकअप ले लें: बिजली जाने पर उसे छोटा करने से फ़ाइलें खो सकती हैं।

# Sentence under the title when no disk has been chosen.
installer-ready-none = अभी कोई डिस्क नहीं चुनी गई।

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = पूरी { $disk }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = उस पर सब कुछ { $name } से बदल दिया जाता है।

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = { $part } के बगल में

# Words of that row.
installer-ready-beside-text = { $part } को छोटा करके { $size } तक किया जाता है, और { $name } खाली हुई जगह में जाता है।

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = कोई डिस्क नहीं

# Words of that row.
installer-ready-no-disk-text = कोई डिस्क नहीं चुनी गई।

# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = एन्क्रिप्टेड नहीं

# Words of that row.
installer-ready-plain-text = जो भी डिस्क निकाल ले, वह उसे पढ़ सकता है।

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = एन्क्रिप्टेड

# Words of that row when a password is typed at each start.
installer-ready-locked-password = कंप्यूटर हर बार चालू होने पर आप उसका पासवर्ड लिखते हैं।

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = एक बार पासवर्ड, फिर इस कंप्यूटर की TPM चिप इसे खोल देती है।

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = कीबोर्ड { $keyboard }, समय { $zone }।

# Title of the page shown while the install runs.
installer-installing-title = { $name } इंस्टॉल हो रहा है

# Head of the card that holds the short tour.
installer-tour = एक छोटा दौरा

# Title of the last page.
installer-done-title = { $name } इंस्टॉल हो गया

# Sentence under the title of the last page.
installer-done-help = USB स्टिक निकालें, फिर रीस्टार्ट करें।

# Head of the card that repeats the warning about the disk password.
installer-done-keep = डिस्क का पासवर्ड सुरक्षित रखें

# In that card when the TPM chip unlocks the disk.
installer-keep-chip = डिस्क एन्क्रिप्टेड है। पहली बार चालू करने पर एक बार डिस्क का पासवर्ड माँगा जाता है, जब चिप इस कंप्यूटर को सीखती है; उसके बाद वह अपने-आप खुलती है। उस पासवर्ड को कागज़ पर लिख लें और इस कंप्यूटर से दूर किसी सुरक्षित जगह रखें। इसके बिना डिस्क कभी नहीं खोली जा सकती, न आपसे, न किसी और से।

# In that card when the password is typed at each start.
installer-keep-password = डिस्क एन्क्रिप्टेड है। अगर आपने अभी तक नहीं लिखा, तो डिस्क का पासवर्ड अभी कागज़ पर लिख लें और इस कंप्यूटर से दूर किसी सुरक्षित जगह रखें। इसके बिना डिस्क कभी नहीं खोली जा सकती, न आपसे, न किसी और से।

# Title of the first slide.
installer-slide-ring = सब कुछ एक घेरे में

# Words of the first slide.
installer-slide-ring-text = पॉइंटर को स्क्रीन के किनारे ले जाकर डबल-क्लिक करें, और रिंग खुल जाती है: बाहर की ओर आपके प्रोग्राम, बीच में समय और स्विच।

# Title of the second slide.
installer-slide-snapshots = अपडेट ने जो बिगाड़ा उसे वापस लाएँ

# Words of the second slide.
installer-slide-snapshots-text = हर सॉफ़्टवेयर बदलाव से पहले और बाद में सिस्टम का स्नैपशॉट लिया जाता है। सेटिंग्स पूरे सिस्टम को उनमें से किसी पर भी लौटा देती है, और कंप्यूटर शुरू न होने पर बूट मेनू भी यह कर सकता है।

# Title of the third slide.
installer-slide-apps = ऐप्स जो रास्ते में नहीं आते

# Words of the third slide.
installer-slide-apps-text = Flathub के ऐप, Debian के अपने, AppImage और सॉफ़्टवेयर स्रोत एक जगह, ऐसे बैकअप के साथ जिन्हें आप दूसरे कंप्यूटर पर ले जा सकते हैं।

# Title of the fourth slide.
installer-slide-yours = आपका अपना, जल्दी

# Words of the fourth slide.
installer-slide-yours-text = हल्का या गहरा रूप, आपकी अपनी ध्वनियाँ, और एक क्लिपबोर्ड जो याद रखता है कि आपने क्या कॉपी किया।

# Title of the fifth slide.
installer-slide-small = छोटा और शांत

# Words of the fifth slide.
installer-slide-small-text = डेस्कटॉप मेमोरी पर हल्का है और कुछ सेकंड में शुरू हो जाता है, इसलिए पुराना लैपटॉप फिर से नया लगता है।

# Title of the tour slide about the ring.
installer-tour-ring = रिंग ही पूरा डेस्कटॉप है

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = पॉइंटर को इस कोने में रखें और रिंग खुल जाती है

# Note on the ring picture, at the time.
installer-tour-ring-time = बीच में समय और तारीख़

# Note on the ring picture, at the programs.
installer-tour-ring-programs = आपके प्रोग्राम बाहर की ओर रहते हैं

# Note on the ring picture, at the switches.
installer-tour-ring-switches = आवाज़, स्क्रीन, आपके सभी ऐप्स, जगाए रखना, और बंद करना

# Title of the tour slide about Settings.
installer-tour-settings = सेटिंग्स, सीधे शब्दों में

# Note on the Settings picture, at the search box.
installer-tour-settings-search = कोई सेटिंग ढूँढे बिना पाने के लिए यहाँ लिखें

# Note on the Settings picture, at Network.
installer-tour-settings-network = नेटवर्क वह जगह है जहाँ आप वाई-फ़ाई नेटवर्क से जुड़ते हैं

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = हल्का या गहरा, जब चाहें

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = बीस रंग, या अपना खुद का लिखें

# Title of the tour slide about Apps.
installer-tour-apps = ऐप्स, बाकी सब के लिए

# Note on the Apps picture, at the search box.
installer-tour-apps-search = किसी प्रोग्राम को उसके नाम या काम से खोजें

# Note on the Apps picture, at the list.
installer-tour-apps-many = हज़ारों प्रोग्राम, मुफ़्त, Flathub और Debian से

# Note on the Apps picture, at updates.
installer-tour-apps-updates = सबके अपडेट, Trindle के भी

# Title of the tour slide about voice control.
installer-tour-voice = आप इससे बोलकर भी काम ले सकते हैं

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = आवाज़ से नियंत्रण यहाँ है: सेटिंग्स, फिर रिंग

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = एक बार "आवाज़ जोड़ें" दबाएँ। यह लगभग 150 MB है और कुछ मिनट लेता है

# Note on the voice picture, about what to say.
installer-tour-voice-say = फिर रिंग के बीच में टैप करें और किसी ऐप का नाम, “तेज़”, या “वाई-फ़ाई सेटिंग्स” बोलें

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = आप जो कहते हैं वह इसी कंप्यूटर पर समझा जाता है और कहीं और नहीं जाता

# Title of the tour slide about snapshots.
installer-tour-snapshots = कुछ बदलने से पहले, लौटने का एक रास्ता

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = कंप्यूटर हर अपडेट से पहले स्नैपशॉट लेता है

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = अगर कुछ बिगड़े, तो बूट मेनू से वैसे ही लौट जाएँ जैसा पहले था

# Name of the picture of the ring, and the head of its card.
installer-live-caption = रिंग, जहाँ सब कुछ रहता है

# Sentence under the title of the welcome from a stick.
installer-live-help = आप स्टिक से चला रहे हैं, इसलिए इस कंप्यूटर पर कुछ भी बदला नहीं गया है।

# Head of the card with the three first things to do.
installer-live-begin = शुरू करने के लिए

# Name of the first thing to do.
installer-live-open = रिंग खोलें

# Line under it.
installer-live-open-line = स्क्रीन के किनारे पर डबल-क्लिक करें, या Super दबाएँ।

# Name of the second thing to do.
installer-live-speak = इससे बोलें

# Line under it when voice control is there.
installer-live-speak-line = रिंग के बीच में टैप करें और बताएँ कि आप क्या चाहते हैं।

# Line under it when voice control has to be added.
installer-live-speak-add = सेटिंग्स, रिंग में आवाज़ जोड़ें, फिर बीच में टैप करके बोलें।

# Name of the third thing to do.
installer-live-tab = सब कुछ रिंग में

# Line under it.
installer-live-tab-line = जो खुला है, आपकी सूचनाएँ और क्लिपबोर्ड, सब यहीं हैं।

# The choice of the dark look.
installer-live-dark = गहरा

# The choice of the light look.
installer-live-light = हल्का

# Name the screen reader gives to the choice between dark and light.
installer-live-look = पूरे डेस्कटॉप के लिए हल्का या गहरा

# Name of the round button that closes this window and opens the ring.
installer-live-around = पहले इधर-उधर देखें

# Name of the main button that opens the installer.
installer-live-install = { $name } इंस्टॉल करें

# Foot of the install page while room is made on the disk.
installer-step-room = जगह बनाई जा रही है

# Foot of the install page while the disk is prepared.
installer-step-prepare = डिस्क तैयार की जा रही है

# Foot of the install page while a place is made beside another system.
installer-step-place = Trindle के लिए जगह बनाई जा रही है

# Foot of the install page while the disk is looked at.
installer-step-check = देखा जा रहा है कि डिस्क पर पहले से क्या है

# Foot of the install page while another system is made smaller.
installer-step-smaller = दूसरे सिस्टम को छोटा किया जा रहा है

# Foot of the install page while the disk is encrypted.
installer-step-lock = डिस्क लॉक की जा रही है

# Foot of the install page while the TPM chip is set up.
installer-step-chip = कंप्यूटर की चिप सेट की जा रही है

# Foot of the install page while the system is copied.
installer-step-copy = Trindle को डिस्क पर कॉपी किया जा रहा है

# Foot of the install page while settings are written.
installer-step-settings = सेटिंग्स लिखी जा रही हैं

# Foot of the install page while the person's account is made.
installer-step-account = आपका खाता बनाया जा रहा है

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = लाइव हिस्से निकाले जा रहे हैं

# Foot of the install page while the start-up files are made.
installer-step-startup = शुरू होने की फ़ाइलें बनाई जा रही हैं

# Foot of the install page while the boot menu is set up.
installer-step-boot = बूट मेनू सेट किया जा रहा है

# Foot of the install page while snapshots are set up.
installer-step-snapshots = स्नैपशॉट सेट किए जा रहे हैं

# Foot of the install page at the very end.
installer-step-finishing = पूरा किया जा रहा है

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } शुरू नहीं हो सका: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } पूरा नहीं हुआ। { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = { $path } लिखा नहीं जा सका: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } डिस्क नहीं है।

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } इस कंप्यूटर की कोई डिस्क नहीं है।

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = कोई डिस्क नहीं चुनी गई।

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = वह साइन-इन नाम Trindle इस्तेमाल नहीं कर सकता।

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = वह कंप्यूटर का नाम Trindle इस्तेमाल नहीं कर सकता।

# Red line at the foot: the language cannot be used.
installer-bad-language = यह भाषा Trindle इस्तेमाल नहीं कर सकता।

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = यह कीबोर्ड Trindle इस्तेमाल नहीं कर सकता।

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = यह समय क्षेत्र Trindle इस्तेमाल नहीं कर सकता।

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = उस नाम में कुछ ऐसा है जिसे Trindle लिख नहीं सकता।

# Red line at the foot: a password has a line break in it.
installer-bad-password = पासवर्ड में लाइन ब्रेक नहीं हो सकता।

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = इंस्टॉलर का दूसरा आधा हिस्सा root के रूप में चलना चाहिए।

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = इस डिस्क में साझा करने के लिए बूट पार्टीशन नहीं है, इसलिए Trindle पहले से मौजूद चीज़ों के बगल में नहीं जा सकता।

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = डिस्क का नया हिस्सा नहीं मिला।

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = { $part } का आकार पढ़ा नहीं जा सका।

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle { $fs } पार्टीशन को छोटा नहीं कर सकता।

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle अज्ञात पार्टीशन को छोटा नहीं कर सकता।

# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = डिस्क को एन्क्रिप्ट करने के लिए पासवर्ड चाहिए।

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = सिस्टम कॉपी नहीं किया जा सका: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = सिस्टम कॉपी नहीं किया जा सका। { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = स्वैप फ़ाइल के लिए जगह नहीं है।

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = डिस्क पर स्वैप फ़ाइल की जगह पढ़ी नहीं जा सकी।

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = नए पार्टीशन पहचाने नहीं जा सके।

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = समय क्षेत्र सेट नहीं किया जा सका: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = इस कंप्यूटर पर बूट मेनू इंस्टॉल नहीं किया जा सका।

# Name of that button after one press, when a second press goes ahead.
installer-sure = पक्का?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = क्या खुला है, क्लिपबोर्ड, और स्क्रीन साफ़ करने वाला एक बटन

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } वह स्टिक है जिससे Trindle चल रहा है।

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } उपयोग में है। जो उसे उपयोग कर रहा है उसे बंद करें, फिर दोबारा कोशिश करें।

# Said beside how far the install has got, once it has started.
installer-started = इंस्टॉल शुरू हुआ

# The button that ends an install under way.
installer-halt = रोकें

# Beside Stop after its first press.
installer-halt-sure = इंस्टॉल रोकें? डिस्क की पुरानी सामग्री पहले ही जा चुकी है।

# While an install is being stopped.
installer-halting = रोका जा रहा है

# When Stop could not let go of the disk.
installer-halt-stuck = डिस्क को छोड़ा नहीं जा सका। दोबारा कोशिश करने के लिए कंप्यूटर को रीस्टार्ट करें।

# The copy is done and the person is still answering.
installer-step-waiting = कॉपी हो गया। आपके उत्तरों का इंतज़ार है।



installer-already-running = इस कंप्यूटर पर पहले से एक और इंस्टॉलर चल रहा है।


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle को कम से कम 1 GB मेमोरी चाहिए, और इस कंप्यूटर में इससे कम है।

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = डिस्क को एन्क्रिप्ट केवल उसी कंप्यूटर पर किया जा सकता है जो UEFI से शुरू होता है, और यह कंप्यूटर ऐसा नहीं है।

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = { $part } पर Windows हाइबरनेट था या फ़ास्ट स्टार्ट-अप में छोड़ा गया था, इसलिए उसे छोटा करने पर उसकी फ़ाइलें खो सकती हैं। Windows शुरू करें, रीस्टार्ट चुनें, फिर दोबारा कोशिश करें।

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle इंस्टॉल हो गया है, लेकिन स्नैपशॉट सेट अप नहीं किए जा सके: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle इंस्टॉल हो गया है, लेकिन बूट मेनू में स्नैपशॉट की सूची नहीं बनाई जा सकी: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = सब कुछ सेट अप नहीं हो सका

