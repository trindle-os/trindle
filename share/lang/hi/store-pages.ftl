# machine translated (Hindi), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } ऐप
       *[other] { $count } ऐप्स
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } अपडेट प्रतीक्षा में है
       *[other] { $count } अपडेट प्रतीक्षा में हैं
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub पर चुनने के लिए { $apps }; { $more }।

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = आपका सब कुछ अद्यतन है

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }।

# Line under the title of a search that found nothing.
store-grid-none-found = “{ $text }” से कोई ऐप मेल नहीं खाता।

# Line under the title of an empty page of apps.
store-grid-none-here = यहाँ अभी कोई ऐप नहीं है।

# Advice in the empty page of a search.
store-grid-try-shorter = छोटा शब्द आज़माएँ, या बाईं ओर कोई श्रेणी देखें।

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = ऊपर नाम से खोजें, या बाईं ओर कोई दूसरी श्रेणी देखें।

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $apps } में से { $shown } दिखाए जा रहे हैं।

# Link under a page of apps that shows more of them.
store-grid-show-more = { $next } और दिखाएँ

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Flathub से ऐप्स की सूची लाई जा रही है। पहली बार में एक मिनट लगता है।

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = ऐप्स Flathub तक पहुँच नहीं सका।

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = यह कंप्यूटर इंटरनेट से जुड़ा नहीं है। जुड़ते ही ऐप्स अपने आप फिर कोशिश करेगा।

# In the waiting page when the computer is not connected.
store-wait-comes-from = चुनने के लिए ऐप्स इंटरनेट पर Flathub से आते हैं।

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } और ऐप्स की सेटिंग, बाईं ओर, इसके बिना भी काम करते हैं।

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } और ऐप्स की सेटिंग, बाईं ओर, Flathub के बिना भी काम करते हैं।

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } और ऐप्स की सेटिंग, बाईं ओर, इंटरनेट के बिना भी काम करते हैं।

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = फिर कोशिश करें

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = नतीजे

# Red words under the search box when no web browser opens a link.
store-no-browser = { $url } खोलने के लिए कोई वेब ब्राउज़र नहीं है। पहले ऐप्स में से एक ले लें।

# Said in the card or under the title while the lists are being read.
store-looking = खोज रहा है…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = यह अब इंस्टॉल किए हुए में है, और रिंग में भी

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = हटा दिया गया

# The word on an app's dial when installing or removing did not work.
store-done-not = नहीं हुआ

# The word on an app's dial when it was installed.
store-done-installed = इंस्टॉल हो गया

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak ने मना कर दिया।

# Tab of the Installed page that shows everything.
store-tab-all = सभी

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak फ़ाइलें

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb फ़ाइलें

# Tab of the Installed page that shows plugins.
store-tab-plugins = प्लगइन

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = अभी कुछ भी इंस्टॉल नहीं है। खोजें में आज़माने के लिए बहुत कुछ है।

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 प्लगइन इंस्टॉल है
       *[other] { $n } प्लगइन इंस्टॉल हैं
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 प्रोग्राम इंस्टॉल है
       *[other] { $n } प्रोग्राम इंस्टॉल हैं
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } चीज़ें इंस्टॉल हैं

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, Flathub से
       *[other] { $start }, सब Flathub से
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, एक Flatpak फ़ाइल से
       *[other] { $start }, सब Flatpak फ़ाइलों से
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, एक AppImage
       *[other] { $start }, सब AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, एक .deb फ़ाइल से
       *[other] { $start }, सब .deb फ़ाइलों से
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } Flathub से

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 Flatpak फ़ाइल से
       *[other] { $n } Flatpak फ़ाइलों से
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
       *[other] { $n } AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 ऐप
       *[other] { $n } ऐप्स
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 .deb फ़ाइल से
       *[other] { $n } .deb फ़ाइलों से
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 प्लगइन
       *[other] { $n } प्लगइन
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }। { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = यह अद्यतन है।

# End of the Installed line when everything is up to date.
store-installed-state-all = सब अद्यतन हैं।

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 का अपडेट प्रतीक्षा में है।
       *[other] { $n } के अपडेट प्रतीक्षा में हैं।
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $total } प्रोग्रामों में से { $shown } “{ $find }” से मेल खाते हैं।

# The same when plugins are among the things.
store-installed-match-things = { $total } चीज़ों में से { $shown } “{ $find }” से मेल खाती हैं।

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = AppImage जोड़ें

# Placeholder of the typing box on the Installed page.
store-installed-search = इंस्टॉल की हुई चीज़ें खोजें

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage जोड़ा नहीं जा सका।

# In the card of the Installed page when nothing is installed.
store-installed-empty = खोजें से, GitHub से या AppImage के रूप में इंस्टॉल किए ऐप्स, .deb फ़ाइलों के प्रोग्राम और प्लगइन यहाँ दिखते हैं।

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = इंस्टॉल की हुई कोई चीज़ “{ $find }” से मेल नहीं खाती।

# SAFETY: needs a human review before it ships
# Note at the foot of the Installed list.
store-installed-remove-twice = हटाएँ दो बार पूछता है: तीन सेकंड के अंदर इसे फिर दबाएँ।

# On an installed thing's line: its version.
store-installed-version = संस्करण { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = रोका गया

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = रोका गया; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } अपडेट हो रहा है

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } अपडेट करें

# Tip of the round button that shows an installed app's page.
store-installed-about = जानकारी

# Under an installed thing's row while it is being taken off.
store-installed-removing = हटाया जा रहा है...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = कौन सी फ़ाइल, यह पूछने के लिए कोई सत्र बस नहीं है।

# Title of the window that asks which AppImage to add.
store-add-choose-title = AppImage चुनें

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = फ़ाइल चुनने वाले से पूछा नहीं जा सका।

# Words in the Installed card while an AppImage is being added.
store-add-adding = जोड़ा जा रहा है...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } पहले से है: इंस्टॉल किए हुए में देखें।

# Words once an AppImage is in the ring.
store-add-in-ring = जोड़ दिया गया। अब यह आपकी सूची में है।

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = यह ऐसा Flatpak संदर्भ नहीं है जिसे यह कंप्यूटर इस्तेमाल कर सके।

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = { $host } से { $name } इंस्टॉल हो रहा है...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } इंस्टॉल हो गया।

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak ने इसे इंस्टॉल नहीं किया।

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } इंस्टॉल नहीं हो सका। { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = बदला जा रहा है...

# SAFETY: needs a human review before it ships
# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } पहले से है

# Small heading over the AppImage being brought.
store-replace-new = जो लाया जा रहा है

# Small heading over the AppImage already there.
store-replace-old = जो पहले से है

# SAFETY: needs a human review before it ships
# Tip of the round button that keeps the AppImage already there.
store-replace-skip = छोड़ें: जो पहले से है उसे रखें

# SAFETY: needs a human review before it ships
# Button that replaces the AppImage already there with the new one.
store-replace-do = बदलें

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = यह पूरा नहीं हुआ, और कोई कारण नहीं बताया।

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = दिखाएँ कि समस्या कहाँ है

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = GitHub खोजें: लिखें, फिर Enter दबाएँ

# Heading of the card of matches from AppImageHub.
store-gh-hub = AppImageHub पर ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = पूरे GitHub में भी खोजने के लिए Enter दबाएँ।

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub से पूछा नहीं गया: पूछने के लिए इंटरनेट नहीं है।

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub से पूछा जा रहा है...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub पर और कुछ नहीं।

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] { $n } प्रोजेक्ट में AppImage और Flatpak फ़ाइलें खोजी जा रही हैं...
       *[other] { $n } प्रोजेक्टों में AppImage और Flatpak फ़ाइलें खोजी जा रही हैं...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub पर मिले किसी भी नतीजे में इस कंप्यूटर के लिए AppImage या Flatpak फ़ाइल नहीं है।

# Heading of the card of matches from GitHub.
store-gh-on = GitHub पर

# Heading of the card that holds the search box.
store-gh-search-card = GitHub खोजें

# SAFETY: needs a human review before it ships
# Line under the title of the GitHub page.
store-gh-line = GitHub पर AppImage या Flatpak फ़ाइलों के रूप में ऐप्स। इन्हें कोई नहीं जाँचता: केवल उन्हीं से इंस्टॉल करें जिन पर आपको भरोसा है।

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = जब तक यह कंप्यूटर इंटरनेट से जुड़ा नहीं है, GitHub में खोजा नहीं जा सकता।

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak और AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = फ़ाइलें देखें

# Button that installs a GitHub project or file.
store-gh-install = इंस्टॉल करें

# Link to a project's page on GitHub.
store-gh-open = GitHub पर खोलें

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, रिलीज़ { $tag }

# Name of the GitHub key's button and of its box.
store-gh-key = GitHub की

# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub की: एक रखी है

# In the GitHub key's box: what the key is for.
store-gh-key-words = आपको लगभग निश्चित रूप से इसकी ज़रूरत नहीं है। बिना की के GitHub इस कंप्यूटर से एक घंटे में 60 सवालों का जवाब देता है; की के साथ 5,000 का। की केवल यह सीमा बढ़ाती है। इसे सिर्फ़ सार्वजनिक फ़ाइलें देखने के लिए इस्तेमाल किया जाता है, और कुछ नहीं: यह Trindle को आपके खाते में जाने का कोई रास्ता नहीं देती, और इसे बनाते समय आप कोई अनुमति नहीं चुनते। यह इस कंप्यूटर पर, आपकी कीरिंग में रखी जाती है, और केवल GitHub को भेजी जाती है।

# In the GitHub key's box: how to make a key.
store-gh-key-how = GitHub पर एक की बनाएँ, उसे कोई भी नाम दें, हर अनुमति को जैसा है वैसा रहने दें, Generate token दबाएँ, फिर जो दिखे उसे यहाँ चिपकाएँ।

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = इसे हटाया नहीं जा सका: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = इसे रखा नहीं जा सका: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = इसे रखने के लिए खाली छोड़ें

# Placeholder of the GitHub key's box.
store-gh-key-paste = की चिपकाएँ

# Tip of the round button that removes the stored GitHub key.
store-gh-key-remove = की हटाएँ

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = GitHub पर की बनाएँ

# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = की रखें

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = इसकी सबसे नई रिलीज़ में यहाँ इंस्टॉल होने लायक कुछ नहीं है।

# Word on the file's dial and under a GitHub file when it was added.
store-added = जोड़ा गया

# Line under the title of the Move your apps page.
store-bk-line = आपके ऐप्स और सेटिंग, एक फ़ाइल में जिसे आप कहीं भी ले जा सकते हैं।

# Heading of the card that makes a backup.
store-bk-card = बैकअप लें

# Tip of the round button while the backup is being made.
store-bk-making-tip = बैकअप बन रहा है

# Main button that makes the backup.
store-bk-make = बैकअप बनाएँ

# Tip of the round button that shows the backup in Folders.
store-bk-show = इसे फ़ोल्डर में दिखाएँ

# Name of the switch for the list of apps in a backup.
store-bk-apps = ऐप्स

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] आपके 1 Flatpak ऐप की सूची।
       *[other] आपके { $n } Flatpak ऐप्स की सूची।
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } दूसरा कंप्यूटर इन्हें Flathub से नया इंस्टॉल करता है, इसलिए फ़ाइल छोटी रहती है।

# Name of the switch for the apps' own data in a backup.
store-bk-data = ऐप्स का अपना डेटा

# Under the Apps' own data switch.
store-bk-data-sub = हर ऐप की सेटिंग और सहेजा हुआ काम, उसके कैश के बिना।

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = पूरे रखे जाते हैं, क्योंकि ये अकेली फ़ाइलें हैं।

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle की सेटिंग

# Under the Trindle settings switch.
store-bk-settings-sub = रिंग, वॉलपेपर का चुनाव, रंग, कीबोर्ड, पावर, डिफ़ॉल्ट ऐप्स और आपकी तस्वीर।

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } अभी { $size }।

# Name of the row that says where the backup goes.
store-bk-where = यह कहाँ जाता है

# Under that row: the folder and the file's name.
store-bk-where-sub = आपका होम फ़ोल्डर, { $name } के रूप में

# Heading of the card that puts a backup back.
store-bk-back-card = बैकअप वापस लगाएँ

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = बैकअप फ़ाइल चुनें

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Trindle का बैकअप, या { $flat } का बनाया हुआ।

# Note in the card that puts a backup back.
store-bk-back-note = इसके ऐप्स इंस्टॉल हो जाते हैं और इसका डेटा और सेटिंग वापस रख दी जाती हैं। आपके दस्तावेज़ और तस्वीरें इसमें नहीं हैं: उन्हें खुद कॉपी करें।

# Said while a backup is being made.
store-bk-making = बैकअप बन रहा है... इसमें कुछ मिनट लग सकते हैं।

# Said when a backup is made: the file and its size.
store-bk-made = यह { $path } है ({ $size })। इसे साथ ले जाने के लिए USB स्टिक या क्लाउड स्टोरेज में कॉपी करें।

# Red words when no connection to the file chooser exists.
store-bk-no-bus = कोई बैकअप नहीं चुना गया, क्योंकि Trindle का फ़ाइल चुनने वाले से कनेक्शन नहीं है।

# Title of the window that asks which backup to use.
store-bk-choose-title = बैकअप चुनें

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle और Flat बैकअप

# Red words when the file chooser gave no answer.
store-bk-no-answer = कोई बैकअप नहीं चुना गया, क्योंकि फ़ाइल चुनने वाले ने जवाब नहीं दिया।

# Said while a backup file is read.
store-bk-reading = बैकअप पढ़ा जा रहा है...

# Said when a backup was put back.
store-bk-logout = हर सेटिंग देखने के लिए लॉग आउट करके फिर लॉग इन करें।

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } हर सेटिंग देखने के लिए लॉग आउट करके फिर लॉग इन करें।

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 ऐप, { $with } अपनी सेटिंग के साथ। शुरू हो रहा है...
       *[other] { $apps } ऐप्स, { $with } अपनी सेटिंग के साथ। शुरू हो रहा है...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] निर्माताओं के अपने स्रोत अभी जोड़े नहीं गए हैं।
        [one] निर्माता का 1 स्रोत जोड़ा गया है, जो Debian के अपडेट के साथ अद्यतन रखा जाता है।
       *[other] निर्माताओं के { $n } स्रोत जोड़े गए हैं, जो Debian के अपडेट के साथ अद्यतन रखे जाते हैं।
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }। जोड़ा गया, और अद्यतन रखा जाता है।

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }। स्रोत जोड़ा गया है, पर प्रोग्राम इंस्टॉल नहीं है।

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }। दूसरे तरीके से इंस्टॉल किया गया।

# SAFETY: needs a human review before it ships
# Tip of the round button that removes a maker's source and its program.
store-src-remove-both = इसे और इसका स्रोत हटाएँ

# Link to a maker's page, naming the program.
store-src-open = { $name } खोलें

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Tailscale में साइन इन करें

# SAFETY: needs a human review before it ships
# Tip of the round button that adds a maker's source, naming the program.
store-src-add = { $name } जोड़ें

# Heading of the card of makers' sources that can be added.
store-src-ready = जोड़ने के लिए तैयार

# Name of the field for the address of a source.
store-src-address = पता

# Under the address field.
store-src-address-sub = रिपॉज़िटरी का पता, उसके निर्माता के इंस्टॉल निर्देशों से।

# Name of the field for the suite of a source.
store-src-suite = सुइट

# Under the suite field.
store-src-suite-sub = आमतौर पर trixie या stable।

# Name of the field for the components of a source.
store-src-components = घटक

# Under the components field.
store-src-components-sub = आमतौर पर main।

# Name of the field for a source's signing key.
store-src-key = हस्ताक्षर की

# Under the signing key field.
store-src-key-sub = उसकी हस्ताक्षर की का पता।

# Name of the field for the program to install from a source.
store-src-program = इंस्टॉल करने का प्रोग्राम

# Under the program field.
store-src-program-sub = उसका पैकेज नाम, अगर आप उसे अभी इंस्टॉल कराना चाहते हैं।

# Placeholder of the program field.
store-src-program-hint = वैकल्पिक

# SAFETY: needs a human review before it ships
# Tip of the round button that adds the source typed.
store-src-add-this = यह स्रोत जोड़ें

# SAFETY: needs a human review before it ships
# Heading of the card that adds another source.
store-src-another = एक और स्रोत जोड़ें

# SAFETY: needs a human review before it ships
# Note in that card.
store-src-another-note = कोई स्रोत पूरे कंप्यूटर के लिए अपडेट इंस्टॉल कर सकता है, इसलिए केवल उन्हीं निर्माताओं को जोड़ें जिन पर आपको भरोसा है। हर एक के लिए एडमिनिस्ट्रेटर का पासवर्ड माँगा जाता है।

# SAFETY: needs a human review before it ships
# Tip of the round button that removes a source that is already on the computer.
store-src-remove-this = यह स्रोत हटाएँ

# Heading of the card of sources already on the computer.
store-src-already = इस कंप्यूटर पर पहले से है

# Red words when the addresses of a new source are not secure.
store-src-bad-address = दोनों पते https:// से शुरू होने चाहिए

# Red words when the suite or components are not plain words.
store-src-bad-words = सुइट और घटक सादे शब्द होने चाहिए।

# Red words when the program is not a package name.
store-src-bad-package = यह पैकेज का नाम नहीं है।

# Word on a source's dial while its job starts.
store-src-starting = शुरू हो रहा है

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Tailscale का साइन-इन पन्ना खुल रहा है...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = जो पन्ना खुला है उस पर साइन इन करें; यह अपने आप पूरा हो जाएगा।

# Said under Tailscale's row when it is connected.
store-src-connected = जुड़ गया

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = ऐप्स का लिंक एन्क्रिप्टेड नहीं है।

# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = एन्क्रिप्टेड नहीं

# SAFETY: needs a human review before it ships
# Words of that card.
store-bk-http-card = यह पता एन्क्रिप्टेड नहीं है। रास्ते में कोई भी भेजी जा रही चीज़ देख सकता है, और बैकअप को किसी दूसरे से बदल भी सकता है।

# SAFETY: needs a human review before it ships
# Label of the tick box on that card; the restore button works only once it is on.
store-bk-http-tick = मैं यह जोखिम समझता हूँ और स्वीकार करता हूँ

# SAFETY: needs a human review before it ships
# Button on that card that restores the backup.
store-bk-http-go = बहाल करें

# SAFETY: needs a human review before it ships
# Button on that card that gives the restore up.
store-bk-http-cancel = रद्द करें

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = { $name } हटाएँ?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = इसका सहेजा हुआ डेटा { $size } लेता है।

# Under that heading when the app has saved no data.
store-flatpak-ask-none = इसमें कोई सहेजा हुआ डेटा नहीं है।

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = इसका सहेजा हुआ डेटा गिना जा रहा है...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = मिटाया हुआ डेटा वापस नहीं लाया जा सकता।

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = हटाएँ, डेटा रखें

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = हटाएँ और इसका डेटा मिटाएँ

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = केवल इसका डेटा मिटाएँ

# Why Delete its data only is greyed.
store-flatpak-data-only-none = मिटाने के लिए इसमें कोई सहेजा हुआ डेटा नहीं है।

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = रद्द करें: दोनों रखें

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = इसका सहेजा हुआ डेटा मिटा दिया गया है।

# Words while only the data is being deleted.
store-flatpak-data-going = इसका सहेजा हुआ डेटा मिटाया जा रहा है...

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = इसका सहेजा हुआ डेटा गिना नहीं जा सका।

store-hub-warning = AppImageHub केवल प्रोजेक्ट सूचीबद्ध करता है: कोई उन्हें जाँचता या स्कैन नहीं करता, और उनका हस्ताक्षरित होना ज़रूरी नहीं। Trindle हर AppImage को निजी होम फ़ोल्डर वाले डिब्बे में चलाता है: शुरू में वह इंटरनेट और आवाज़ इस्तेमाल कर सकता है, और आपकी कोई फ़ाइल नहीं देखता।

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = AppImage कहाँ से आया है, यह एक संकेत है, जाँच नहीं।

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = GitHub पर: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = GitHub पर: { $repo }, { $stars } स्टार

# Where an AppImage comes from: a website, by its name.
store-from-site = GitHub पर नहीं: एक वेबसाइट से, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = स्रोत अज्ञात

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = इसकी वेबसाइट खोलें

# When an app was last updated.
store-age-updated = आख़िरी अपडेट: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = आख़िरी अपडेट: { $date }। छह महीने से ज़्यादा से अपडेट नहीं हुआ, इसलिए इसमें अनसुलझी सुरक्षा खामियाँ हो सकती हैं।

# When the source does not say when an app was last updated.
store-age-updated-unknown = आख़िरी अपडेट: पता नहीं

# When an app's project was made.
store-age-created = बनाया गया: { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = बनाया गया: पता नहीं

# A date the source does not give, on a line of Facts.
store-age-not-known = पता नहीं

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = एक जोड़ने से पहले

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] AppImageHub पर { $count } AppImage सूचीबद्ध है।
       *[other] AppImageHub पर { $count } AppImage सूचीबद्ध हैं।
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = AppImageHub की सूची लाई जा रही है…

# The same when the list could not be fetched.
store-browse-failed = AppImageHub की सूची नहीं लाई जा सकी।

# The same with no internet and no copy kept.
store-browse-offline = जब तक यह कंप्यूटर इंटरनेट से जुड़ा नहीं है, AppImageHub की सूची नहीं लाई जा सकती।

# In the list when the kind chosen has none.
store-browse-none = AppImageHub में इस तरह का कोई नहीं है।

# Under the rows: how many are shown of how many.
store-browse-showing = { $n } में से { $shown } दिख रहे हैं।

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Trindle के ऐप्स में जोड़ें

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = विवरण और फ़ाइलें देखें



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = अपडेट की जाँच बंद है

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = OpenAI अपने डाउनलोड पर हस्ताक्षर कर दे, तब ChatGPT लौट आएगा।



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Flathub और AppImageHub पर चुनने के लिए { $apps }; { $more }।

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = AppImageHub पर चुनने के लिए { $apps }।

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = दोनों

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage की जाँच नहीं होती। कोई उन्हें न तो देखता है और न स्कैन करता है।

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle हर AppImage को एक अलग डिब्बे में चलाता है जिसका होम फ़ोल्डर निजी होता है: शुरू में वह इंटरनेट और आवाज़ इस्तेमाल कर सकता है, और आपकी कोई फ़ाइल नहीं देख सकता।

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = दोबारा न दिखाएँ

# Button on that card that goes on with the install.
store-appimage-warn-ok = ठीक है

# Button on that card that gives the install up.
store-appimage-warn-cancel = रद्द करें

