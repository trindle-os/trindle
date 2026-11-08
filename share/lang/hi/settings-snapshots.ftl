
# machine translated (Hindi), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = स्नैपशॉट

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = स्नैपशॉट पुनर्स्थापित पूर्ववत वापस बैकअप अपडेट पुराने संस्करण मिटाई फ़ाइल प्रतियाँ snapshots snapshot restore undo rollback roll back backup update broke btrfs snapper boot menu grub earlier versions older version previous deleted file put back recover home copies

# Help line at the foot of the Snapshots page.
settings-snap-help = हर सॉफ़्टवेयर बदलाव से पहले और बाद में, और जब भी आप एक लें, सिस्टम जैसा था वैसा। एक को पुनर्स्थापित करने पर पूरा सिस्टम अगली शुरुआत से उसी पर लौट आता है; आपकी अपनी फ़ाइलें जैसी हैं वैसी रहती हैं, अगर होम फ़ोल्डर के लिए डिस्क का अपना हिस्सा है, जैसा Trindle का इंस्टॉलर बनाता है। अगर कंप्यूटर शुरू नहीं होता, तो बूट मेनू में Trindle स्नैपशॉट चुनें। पुराने संस्करण हर दिन सबकी अपनी फ़ाइलों की एक प्रति रखते हैं, जो सिस्टम के स्नैपशॉट से अलग है।

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = कोई प्रति नहीं ली गई: डिस्क लगभग भर चुकी है।

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = आज { $time } बजे

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = कल { $time } बजे

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date } को

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = कंप्यूटर रीस्टार्ट नहीं हुआ। इसके बजाय रिंग से रीस्टार्ट करें।

# Said beside the Take a snapshot line while it works.
settings-snap-taking = स्नैपशॉट लिया जा रहा है

# Said beside the set aside line while the space is freed.
settings-snap-freeing = जगह खाली हो रही है...

# Said beside the set aside line once the space is freed.
settings-snap-freed = खाली हो गई

# Said beside a snapshot while it is restored.
settings-snap-restoring = पुनर्स्थापित हो रहा है

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = अगली शुरुआत से सिस्टम स्नैपशॉट { $n } पर होगा। तब से जो कुछ बदला है वह अलग रख दिया गया है, खोया नहीं है।

# Said beside a snapshot while it is deleted.
settings-snap-deleting = मिटाया जा रहा है

# Said beside a snapshot once it is deleted.
settings-snap-deleted = मिटा दिया गया

# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = पासवर्ड का इंतज़ार है...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = हर दिन एक प्रति ली जाती है

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = और नहीं ली जाएँगी

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = रखी जाती हैं

# Said beside the earlier copies while they are cleared.
settings-snap-clearing = साफ़ हो रही हैं

# Said beside the earlier copies once they are cleared.
settings-snap-cleared = साफ़ हो गईं

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] अभी कोई प्रति नहीं
        [one] एक प्रति
       *[other] { $n } प्रतियाँ
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = पहली एक दिन के भीतर ली जाती है।

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = ये { $size } लेती हैं, { $when } नापा गया।

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = इनकी जगह दिन में एक बार नापी जाती है।

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = सब अभी साफ़ करें

# Heading of the card about earlier versions of files.
settings-snap-earlier = पुराने संस्करण

# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle हर दिन सबकी फ़ाइलों की उस समय की एक प्रति रख सकता है, ताकि किसी फ़ाइल का पुराना संस्करण, या गलती से मिटाई गई फ़ाइल, फ़ोल्डर से खोली या वापस रखी जा सके। हर व्यक्ति प्रति में केवल वही पढ़ सकता है जो वह उस दिन पढ़ सकता था, इसलिए निजी होम निजी रहता है; जो फ़ाइल तब दूसरों के लिए खुली थी, वह उस प्रति में उनके लिए खुली रहती है जब तक प्रति हट न जाए। ये प्रतियाँ सिस्टम के स्नैपशॉट से अलग हैं: सिस्टम को पुनर्स्थापित करना आपकी फ़ाइलें कभी नहीं बदलता, और ये सिस्टम को कभी नहीं बदलतीं। कीमत जगह की है: जो फ़ाइल आप मिटाते या बदलते हैं वह अपनी जगह तब तक घेरती है जब तक उसे रखने वाली आखिरी प्रति न हट जाए, इसलिए चीज़ें मिटाने से डिस्क तुरंत खाली नहीं होती। प्रतियाँ साफ़ करने से वह अभी खाली हो जाती है, इस कंप्यूटर के सभी लोगों के लिए, और इसे पूर्ववत नहीं किया जा सकता। डिस्क लगभग भरी होने पर कोई प्रति नहीं ली जाती। इनमें से कुछ भी बदलने के लिए एडमिनिस्ट्रेटर का पासवर्ड चाहिए।

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = अपनी फ़ाइलों के पुराने संस्करण रखें

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = मिटाई गई फ़ाइलें अपनी जगह खाली होने से पहले { $time } तक रहती हैं।

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = हर दिन की प्रति रखें

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = हर दिन की प्रति कितनी देर रखी जाती है

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = स्नैपशॉट देखे जा रहे हैं…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = स्नैपशॉट पढ़े नहीं जा सके।

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = स्नैपशॉट { $n } से शुरू हुआ, इसलिए अभी जो कुछ किया जाए वह रखा नहीं जाएगा।

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] लौटने के लिए { $n } स्नैपशॉट, सबसे नया { $when } का।
       *[other] लौटने के लिए { $n } स्नैपशॉट, सबसे नया { $when } का।
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = अभी कोई स्नैपशॉट नहीं। हर सॉफ़्टवेयर बदलाव से पहले एक लिया जाता है।

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, मिटाए जाने तक रखा जाता है

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = सिस्टम को इस पर पुनर्स्थापित करें

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = यह स्नैपशॉट मिटाएँ

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = अभी रीस्टार्ट करें

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = ये अभी सेट नहीं हैं।

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = trindle पैकेज दोबारा इंस्टॉल करें।

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = स्नैपशॉट सेट नहीं हैं। trindle पैकेज दोबारा इंस्टॉल करने से वे सेट हो जाते हैं: sudo apt install --reinstall trindle।

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = कोई और प्रकार

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = इनके लिए Btrfs चाहिए; यह कंप्यूटर { $fs } इस्तेमाल करता है।

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Debian इंस्टॉल करते समय Btrfs चुनें।

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = स्नैपशॉट के लिए सिस्टम Btrfs पर होना चाहिए, और यह कंप्यूटर { $fs } इस्तेमाल करता है। इन्हें पाने के लिए Debian इंस्टॉल करते समय सिस्टम के लिए Btrfs चुनें।

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = इस कंप्यूटर पर कोई स्नैपशॉट नहीं है।

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = कोई स्नैपशॉट नहीं

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = इस कंप्यूटर पर कोई स्नैपशॉट नहीं

# Heading of the line saying how to have snapshots.
settings-snap-to-have = इन्हें पाने के लिए

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = देख रहा है…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = स्नैपशॉट पढ़े नहीं जा सके

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = अभी कोई नहीं

# Under the None yet heading.
settings-snap-none-line-sub = हर सॉफ़्टवेयर बदलाव से पहले एक लिया जाता है।

# Help text of the Snapshots card (the information button).
settings-snap-list-about = हर स्नैपशॉट पूरे सिस्टम को अगली शुरुआत से वैसा कर सकता है जैसा वह तब था, या मिटाया जा सकता है। दोनों के लिए दो बार दबाना पड़ता है और एडमिनिस्ट्रेटर का पासवर्ड चाहिए। पुनर्स्थापित करने के बाद उसके पास रीस्टार्ट का बटन दिखता है।

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = अभी स्नैपशॉट लें

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = स्नैपशॉट से शुरू होने पर नहीं।

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = आपके मिटाने तक रखा जाता है।

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] पुनर्स्थापित करने पर एक सिस्टम अलग रखा गया
       *[other] पुनर्स्थापित करने पर { $n } सिस्टम अलग रखे गए
    }

# Under the set aside heading.
settings-snap-free-space = सब ठीक होने पर जगह खाली करें।

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = उनकी घेरी जगह खाली करें

# Heading of the card for taking a snapshot.
settings-snap-take-one = एक लें

# Help text of the Take one card (the information button).
settings-snap-take-about = कुछ आज़माने से पहले स्नैपशॉट लें जिसे आप पूर्ववत करना चाहें; आपका लिया हुआ मिटाए जाने तक रखा जाता है। जब कंप्यूटर स्नैपशॉट से शुरू हुआ हो, तो लिया गया कुछ भी रखा नहीं जाएगा, इसलिए पहले सामान्य रूप से रीस्टार्ट करें। हर पुनर्स्थापना सिस्टम को ठीक उससे पहले जैसा था वैसा अलग रख देती है, इसलिए कुछ खोता नहीं; सब ठीक होने पर उनकी घेरी जगह खाली करें।

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = स्नैपशॉट { $n } से शुरू हुआ

# Under the Started from snapshot heading.
settings-snap-nothing-kept = अभी जो कुछ किया जाए वह रखा नहीं जाएगा।

# Heading of a line on the Before restoring card.
settings-snap-files-too = आपकी फ़ाइलें भी स्नैपशॉट में हैं

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = पुनर्स्थापित करने से आपकी फ़ाइलें भी वापस आ जाती हैं।

# Heading of the card about what to know before restoring.
settings-snap-before = पुनर्स्थापित करने से पहले

# Help text of the Before restoring card (the information button).
settings-snap-before-about = स्नैपशॉट से शुरू होने पर अभी जो कुछ किया जाए वह रखा नहीं जाता: अगर सिस्टम ऐसा ही रहना चाहिए, तो सूची में उस पर पुनर्स्थापित दबाएँ, फिर रीस्टार्ट करें। इस कंप्यूटर पर होम फ़ोल्डर सिस्टम का हिस्सा हो सकता है, और तब स्नैपशॉट पुनर्स्थापित करने से आपकी अपनी फ़ाइलें भी वैसी हो जाती हैं जैसी तब थीं, इसलिए पहले कुछ भी नया हो तो उसे कहीं सुरक्षित कॉपी कर लें।



# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = सिस्टम फ़ाइलें लॉक हैं

# Under the System files locked switch.
settings-snap-locked-sub = केवल इंस्टॉल और अपडेट उन्हें अनलॉक करते हैं।

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = लॉक है

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = अनलॉक है

