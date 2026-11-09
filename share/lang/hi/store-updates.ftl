
# machine translated (Hindi), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian के पैकेज

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = संस्करण { $have } पर रोका हुआ; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = रोका हुआ; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = संस्करण { $have } पर रोका हुआ

# On a thing held back with no update waiting.
store-upd-held = रोका हुआ

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = संस्करण { $have } पर रोका हुआ; अभी कुछ नया नहीं

# On the Held back card: nothing newer waits.
store-upd-held-nothing = रोका हुआ; अभी कुछ नया नहीं

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 अपडेट
       *[other] { $n } अपडेट
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 अपडेट तैयार है
       *[other] { $n } अपडेट तैयार हैं
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, कुल { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 रोका हुआ है
       *[other] { $n } रोके हुए हैं
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }।

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = फ़र्मवेयर को पूरा करने के लिए रीस्टार्ट चाहिए।

# Headline while the check runs.
store-upd-checking = अपडेट खोजे जा रहे हैं।

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = { $name } अपडेट हो रहा है। इसके लगते समय आप काम करते रह सकते हैं।

# Headline while all the updates go in.
store-upd-installing-all = अपडेट इंस्टॉल हो रहे हैं। इनके लगते समय आप काम करते रह सकते हैं।

# Headline when the updates are in and the computer must start again.
store-upd-restart = पूरा करने के लिए रीस्टार्ट करें: अपडेट लग गए हैं, और कंप्यूटर के फिर शुरू होने पर उपयोग में आएँगे।

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = जो लग चुका है उसे पूरा करने के लिए रीस्टार्ट चाहिए।

# Added to the headline with no internet.
store-upd-no-more = यह कंप्यूटर इंटरनेट से जुड़ा नहीं है, इसलिए और नहीं खोजा जा सकता।

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = कोई अपडेट तैयार नहीं है

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = इस कंप्यूटर की जानकारी में कोई अपडेट प्रतीक्षा में नहीं है। इंटरनेट से जुड़े न होने पर यह और नहीं खोज सकता।

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = ऐप्स

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle और Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = फ़र्मवेयर

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = { $names } के अपडेट खोजे नहीं जा सके।

# Headline when everything is up to date.
store-upd-current = सब कुछ अद्यतन है।

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = आखिरी बार { $time } पर जाँचा गया।

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = इंटरनेट के बिना जाँचा नहीं जा सकता

# Red words under a card when a check took too long.
store-upd-too-long = जवाब देने में बहुत समय लगा।

# Tip of the round button that checks again.
store-upd-check-again = फिर जाँचें

# Main button while the check runs.
store-upd-checking-main = जाँच रहा है

# Main button while the updates go in.
store-upd-installing-main = इंस्टॉल हो रहा है

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] 1 अपडेट इंस्टॉल करें
       *[other] { $n } अपडेट इंस्टॉल करें
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = अभी रीस्टार्ट करें

# Main button that checks for updates.
store-upd-check-for = अपडेट खोजें

# Tip of the round button that releases a thing held back.
store-upd-release = { $name } को छोड़ें

# Tip of the round button that holds a thing's updates back.
store-upd-hold = { $name } को रोककर रखें

# A card's state while it is being checked.
store-upd-status-checking = जाँच रहा है

# A card's state while its updates go in.
store-upd-status-installing = इंस्टॉल हो रहा है

# A card's state when its check failed; the reason follows.
store-upd-status-failed = जाँचा नहीं जा सका। { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } रोके हुए

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } रोके हुए

# A card's state when nothing waits.
store-upd-up-to-date = अद्यतन है

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = { $from } से, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = { $from } से

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = एक साझा हिस्सा, { $from } से

# Note under the Apps card.
store-upd-yours-alone = सिर्फ़ आपके, इसलिए ये बिना पासवर्ड के अपडेट होते हैं। कोई ऐप अपने नए संस्करण के लिए ज़रूरी साझा हिस्से साथ लाता है।

# Note under the Apps card: what holding back does.
store-upd-hold-why = रोका हुआ अपडेट उसी संस्करण पर रहता है जो उसके पास है, और यहाँ तथा रिंग में "सब इंस्टॉल करें" से बाहर रहता है, जब तक आप उसे छोड़ न दें।

# Heading of the Apps card.
store-upd-card-apps = ऐप्स

# Plugins card: every plugin is up to date.
store-upd-plugins-current = हर प्लगइन अद्यतन है।

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] 1 प्लगइन का अपडेट रोका हुआ है।
       *[other] { $n } प्लगइनों के अपडेट रोके हुए हैं।
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 प्लगइन का अपडेट है
       *[other] { $n } प्लगइनों के अपडेट हैं
    }

# Heading of the Plugins card.
store-upd-card-plugins = प्लगइन

# Held back card: nothing is held back.
store-upd-nothing-held = कुछ भी रोका हुआ नहीं है।

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = छोड़ा गया

# Note under the Held back card: what releasing does.
store-upd-release-why = छोड़ी गई चीज़ फिर बाकी के साथ लगती है, यहाँ और रिंग में "सब इंस्टॉल करें" से।

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = किसी उपकरण का फ़र्मवेयर

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = उपकरण { $id } का फ़र्मवेयर

# Heading of the Held back card.
store-upd-card-held = रोका हुआ

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } से { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 अपडेट, जो अपने आप लगता है
       *[other] { $n } अपडेट, जो साथ में लगते हैं
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Debian से

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Debian के { $total } अपडेट में से { $shown } दिखाए गए हैं।

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = इस कंप्यूटर पर Trindle { $version } है।

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle और Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Debian के पैकेज एक-दूसरे के साथ बैठने के लिए बनाए गए हैं, और अकेला लगाया गया एक पैकेज दूसरों को साथ ला सकता है या उन्हें अलग छोड़ सकता है। इसलिए वे एक बटन से साथ में लगते हैं।

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Trindle और Debian के अपडेट पूरे कंप्यूटर के लिए सुरक्षा सुधार लाते हैं, इसलिए उन्हें रोका नहीं जा सकता।

# Heading of the card about the daily security updates.
store-upd-card-security = सुरक्षा अपडेट

# On the security card: its state.
store-upd-security-on = चालू, हर दिन

# On the security card.
store-upd-security-note = Debian के सुधार बिना पूछे हर दिन लग जाते हैं।

# On the security card.
store-upd-security-part = Trindle का हिस्सा; इसे बंद नहीं किया जा सकता।

# On the security card: that holding back never stops these.
store-upd-never-held = किसी अपडेट को रोकने से ये कभी नहीं रुकते।

# Firmware card: the computer has no updater.
store-upd-fw-none = इस कंप्यूटर में फ़र्मवेयर अपडेटर नहीं है।

# Firmware card: restart to finish.
# SAFETY: needs a human review before it ships
store-upd-fw-restart = पूरा करने के लिए रीस्टार्ट करें

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = संस्करण { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = नया फ़र्मवेयर कंप्यूटर के रीस्टार्ट होने पर लगता है।

# Note on the firmware card when the updater is missing.
store-upd-fw-install = फ़र्मवेयर को अद्यतन रखने के लिए fwupd इंस्टॉल करें।

# Heading of the Firmware card.
store-upd-card-firmware = फ़र्मवेयर

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd नई सूची ला नहीं सका।

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub तक पहुँचा नहीं जा सका।

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Debian की सूचियाँ लाई नहीं जा सकीं।

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Debian की सूचियाँ लाई नहीं जा सकीं। { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = रोका नहीं गया। रोकी गई चीज़ों की सूची सहेजी नहीं जा सकी: { $why }।

# The same, for a release.
store-upd-not-released-list = छोड़ा नहीं गया। रोकी गई चीज़ों की सूची सहेजी नहीं जा सकी: { $why }।

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Apps में रोका गया, पर Flatpak को खुद नहीं बताया जा सका, इसलिए ऐप अपडेट करने वाली कोई और चीज़ इसे फिर भी लगा सकती है।

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Apps में छोड़ा गया, पर Flatpak को खुद नहीं बताया जा सका और वह इसे अब भी छोड़ सकता है। इसे रोकें और फिर छोड़ें।

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak ने कहा: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle



# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = अपने आप अपडेट जाँचना बंद है। देखने के लिए अपडेट जाँचें दबाएँ।

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = जाँचा नहीं गया

# Tip of the round button that ends an update that is running.
store-upd-stop = रोकें

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = रोका जा रहा है

# Under a row whose update was stopped.
store-upd-stopped = रोका गया। जारी रखने के लिए अपडेट दबाएँ।

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = रोका गया। जारी रखने के लिए इंस्टॉल दबाएँ।

# Note under the Apps card.
store-upd-steam-restarts = कोई गेम न चल रहा हो, तब Steam रीस्टार्ट होता है।

