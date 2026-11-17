# machine translated (Hindi), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = इस कंप्यूटर के बारे में

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = होस्टनाम कंप्यूटर का नाम नेटवर्क नाम जानकारी सिस्टम संस्करण मेमोरी प्रोसेसर डिस्क ग्राफ़िक्स स्क्रीन मॉडल कर्नेल गतिविधि चल रहे प्रोग्राम टास्क मैनेजर ड्राइव यूएसबी जगह hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space

# Text behind the info button of the About page.
settings-about-help = यह पेज इस कंप्यूटर का नाम दिखाता है, जैसा नेटवर्क के दूसरे कंप्यूटरों को दिखता है, यह किससे बना है, और इसे चालू होने में कितना समय लगता है। गतिविधि दिखाती है कि कंप्यूटर अभी क्या कर रहा है और कौन-से प्रोग्राम मेमोरी और प्रोसेसर इस्तेमाल कर रहे हैं। ड्राइव आपकी ड्राइव और उनकी जगह दिखाता है। हर एक अपनी अलग विंडो में खुलता है। उन्हें खोलने के लिए "और ब्योरा" के तीर इस्तेमाल करें।

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = अज्ञात

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } थ्रेड
       *[other] { $model }, { $cores } थ्रेड
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel ग्राफ़िक्स

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD ग्राफ़िक्स

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA ग्राफ़िक्स

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = वर्चुअल मशीन ग्राफ़िक्स

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = ग्राफ़िक्स

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }, { $hz } Hz पर

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } इस्तेमाल में

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } खाली

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = कुल { $amount } में से

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, { $on } से चालू है।

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, { $on } से चालू है। डिस्क लगभग भर चुकी है।

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } सेकंड

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } मिनट

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } मिनट
       *[other] { $n } मिनट
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } घंटा
       *[other] { $n } घंटे
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } दिन
       *[other] { $n } दिन
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = केवल अक्षर, अंक और हाइफ़न इस्तेमाल करें। नाम हाइफ़न से शुरू या खत्म न करें।

# Words under the computer name box while the password window is open.
settings-about-name-waiting = आपके पासवर्ड का इंतज़ार है…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = कंप्यूटर का नाम बदला नहीं जा सका।

# Words under the computer name box after renaming worked.
settings-about-name-done = पहले से खुले प्रोग्राम आपके अगली बार लॉग इन करने के बाद नया नाम इस्तेमाल करेंगे।

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = कंप्यूटर का नाम

# Tip of the save button beside the computer name box.
settings-about-rename = इस कंप्यूटर का नाम बदलें

# Bold line of the computer name setting.
settings-about-name = कंप्यूटर का नाम

# Heading of the card with the computer's name.
settings-about-name-group = नाम

# Text behind the info button of the Name card.
settings-about-name-about = नेटवर्क पर और टर्मिनल के प्रॉम्प्ट में यह इस कंप्यूटर का नाम है। केवल अक्षर, अंक और हाइफ़न इस्तेमाल करें, पर हाइफ़न से शुरू या खत्म न करें। इसे बदलने के लिए आपको अपना पासवर्ड डालना होगा।

# Heading of the card about the computer's hardware.
settings-about-hardware = कंप्यूटर

# Text behind the info button of the hardware card.
settings-about-hardware-about = यह कंप्यूटर बनाने वाली कंपनी और मॉडल, प्रोसेसर और ग्राफ़िक्स दिखाता है। यह भी दिखाता है कि प्रोसेसर एक साथ कितने थ्रेड चला सकता है, और इस्तेमाल में हर स्क्रीन का आकार और रिफ़्रेश दर।

# Heading of the card about the software.
settings-about-software = सॉफ़्टवेयर

# Text behind the info button of the software card.
settings-about-software-about = Trindle एक डेस्कटॉप है जो Debian के ऊपर चलता है, और Debian नीचे के प्रोग्रामों की देखभाल करता है। कर्नेल वह हिस्सा है जो हार्डवेयर के साथ काम करता है। Trindle का अपना कंपोज़िटर Wayland का इस्तेमाल करके डेस्कटॉप बनाता है।

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle का अपना कंपोज़िटर

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } ऐप
       *[other] { $n } ऐप्स
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = और ब्योरा

# Text behind the info button of the More detail card.
settings-about-more-about = गतिविधि दिखाती है कि कंप्यूटर अभी क्या कर रहा है और कौन-से प्रोग्राम मेमोरी और प्रोसेसर इस्तेमाल कर रहे हैं। ड्राइव हर ड्राइव, वह कितनी भरी और कितनी ठीक है, और कोई भी USB स्टिक दिखाता है।

# Bold line of the setting that opens Activity.
settings-about-activity = देखें क्या चल रहा है

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = गतिविधि

# Bold line of the setting that opens Drives.
settings-about-drives = ड्राइव और जगह

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ड्राइव

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle चालू होने का समय Downloads में सहेज नहीं सका।

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = तारीख और समय,सेकंड

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle शुरू होने के समय.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Downloads में, { $file } के नाम से

# Label of the word button that saves every start-up time.
settings-about-times-save = सहेजें

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = चालू होने का हर समय Downloads में सहेजें

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = हर बार चालू होने का समय सहेजें

# Quiet line under that setting.
settings-about-times-keep-sub = स्प्रेडशीट के रूप में Downloads में सहेजा जाता है।

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = हर स्तंभ एक बार चालू होना है। सबसे नया दाईं ओर है।

# Heading of the card with the last ten start-up times.
settings-about-times = पिछले दस बार चालू होना

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle हर बार का समय पावर बटन से लॉगिन स्क्रीन तक गिनता है। डिस्क का पासवर्ड लिखने में लगा समय नहीं गिना जाता। सबसे नया सबसे ऊपर है।

# Heading of the memory tile.
settings-about-memory = मेमोरी

# Heading of the disk tile.
settings-about-disk = डिस्क

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = समस्या की रिपोर्ट

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = रिपोर्ट Buildmorphic को जाती हैं, वह समुदाय जो Trindle बनाने में मदद करता है। आप सिर्फ़ एक ईमेल पते से जुड़ सकते हैं। जिन समस्याओं की रिपोर्ट आप सबसे पहले करते हैं, उनके लिए आपको क्रेडिट और स्तर मिलते हैं, भले ही आप उन्हें ठीक न कर सकें। रिपोर्ट में सिर्फ़ तय की हुई सूची की जानकारी होती है, और भेजें दबाए बिना कुछ नहीं भेजा जाता। यूज़रनेम या GitHub नाम देना वैकल्पिक है, और आपका यूज़रनेम कभी आपका ईमेल पता नहीं होता।

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = कुछ गड़बड़ होने पर रिपोर्ट भेजने की पेशकश करें

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic यूज़रनेम

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub नाम

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = समुदाय के बारे में

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] बंद। { $n } इंतज़ार करती रिपोर्ट मिटाई गई।
       *[other] बंद। { $n } इंतज़ार करती रिपोर्ट मिटाई गईं।
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] बंद। { $n } इंतज़ार करती रिपोर्ट मिटाई नहीं जा सकी।
       *[other] बंद। { $n } इंतज़ार करती रिपोर्ट मिटाई नहीं जा सकीं।
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = अपडेट

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle लॉग इन करने के कुछ देर बाद ऐप्स, प्लगइन, फ़र्मवेयर और खुद के नए संस्करण जाँचता है, फिर हर कुछ घंटों में। इसे बंद करने पर Trindle केवल तब जाँचता है जब आप ऐप्स में अपडेट जाँचें दबाते हैं, या रिंग पर अपडेट का बटन। Debian के सुरक्षा सुधार हर दिन इंस्टॉल होते हैं और उनका अपना स्विच है। उसे बंद करने पर वे अपने आप इंस्टॉल नहीं होते और ज्ञात सुरक्षा खामियाँ आपके अपडेट करने तक ठीक नहीं होतीं। किसी भी स्विच को बदलने के लिए आपके एडमिनिस्ट्रेटर का पासवर्ड चाहिए, ताकि पूरा कंप्यूटर आपका चुनाव इस्तेमाल करे।

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = अपडेट जाँचें

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = आपके पासवर्ड का इंतज़ार है…


# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = सुरक्षा अपडेट

# Warning line under the Security updates switch when it is off.
settings-about-security-off = सुरक्षा सुधार अब अपने आप नहीं लगते।

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = बंद, पर Reporting ऐड-ऑन की इंतज़ार करती रिपोर्ट मिटाई नहीं गईं।
