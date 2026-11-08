# machine translated (Hindi), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = इस कंप्यूटर के बारे में

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = होस्टनाम कंप्यूटर का नाम नेटवर्क नाम जानकारी सिस्टम संस्करण मेमोरी प्रोसेसर डिस्क ग्राफ़िक्स स्क्रीन मॉडल कर्नेल गतिविधि चल रहे प्रोग्राम टास्क मैनेजर ड्राइव यूएसबी जगह hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space

# Text behind the info button of the About page.
settings-about-help = इस कंप्यूटर का नाम, जैसा नेटवर्क के दूसरे कंप्यूटरों को दिखता है, यह किससे बना है, और इसे चालू होने में कितना समय लगता है। यह अभी क्या कर रहा है, और कौन-से प्रोग्राम मेमोरी और प्रोसेसर इस्तेमाल कर रहे हैं, यह गतिविधि में है; ड्राइव और उनकी जगह ड्राइव में है। ये दोनों अलग विंडो हैं: "और ब्योरा" के तीर इन्हें खोलते हैं।

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
settings-about-name-bad = केवल अक्षर, अंक और हाइफ़न; नाम हाइफ़न से शुरू या खत्म नहीं हो सकता।

# Words under the computer name box while the password window is open.
settings-about-name-waiting = पासवर्ड का इंतज़ार है...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = कंप्यूटर का नाम बदला नहीं जा सका।

# Words under the computer name box after renaming worked.
settings-about-name-done = पहले से खुले प्रोग्राम अगले लॉगिन के बाद नया नाम देखेंगे।

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = कंप्यूटर का नाम

# Tip of the save button beside the computer name box.
settings-about-rename = इस कंप्यूटर का नाम बदलें

# Bold line of the computer name setting.
settings-about-name = कंप्यूटर का नाम

# Quiet line under the computer name setting.
settings-about-name-sub = अक्षर, अंक और हाइफ़न।

# Heading of the card with the computer's name.
settings-about-name-group = नाम

# Text behind the info button of the Name card.
settings-about-name-about = नेटवर्क पर और टर्मिनल के प्रॉम्प्ट में यह कंप्यूटर कैसे दिखता है। अक्षर, अंक और हाइफ़न; हाइफ़न से शुरू या खत्म नहीं हो सकता। इसे बदलने के लिए आपका पासवर्ड माँगा जाता है।

# Heading of the card about the computer's hardware.
settings-about-hardware = कंप्यूटर

# Text behind the info button of the hardware card.
settings-about-hardware-about = बनाने वाली कंपनी और मॉडल, प्रोसेसर और वह एक साथ कितने थ्रेड चलाता है, ग्राफ़िक्स, और इस्तेमाल में हर स्क्रीन का आकार और रिफ़्रेश दर।

# Quiet line under the maker and model.
settings-about-hw-computer = कंप्यूटर

# Quiet line under the processor.
settings-about-hw-processor = प्रोसेसर

# Quiet line under the graphics.
settings-about-hw-graphics = तस्वीरें बनाता है

# Quiet line under the screens.
settings-about-hw-screen = स्क्रीन

# Heading of the card about the software.
settings-about-software = सॉफ़्टवेयर

# Text behind the info button of the software card.
settings-about-software-about = Trindle, Debian के ऊपर चलने वाला डेस्कटॉप है, और Debian नीचे के प्रोग्रामों की देखभाल करता है। कर्नेल वह हिस्सा है जो हार्डवेयर से बात करता है, और Trindle का अपना कंपोज़िटर Wayland पर डेस्कटॉप बनाता है।

# Quiet line under the Debian version.
settings-about-sw-debian = नीचे चलता है

# Quiet line under the kernel version.
settings-about-sw-kernel = कर्नेल

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle का अपना कंपोज़िटर

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Wayland पर डेस्कटॉप बनाता है

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } ऐप
       *[other] { $n } ऐप्स
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = इंस्टॉल हैं, रिंग में तैयार

# Heading of the card with the ways to Activity and Drives.
settings-about-more = और ब्योरा

# Text behind the info button of the More detail card.
settings-about-more-about = यह अभी क्या कर रहा है, और कौन-से प्रोग्राम मेमोरी और प्रोसेसर इस्तेमाल कर रहे हैं, यह गतिविधि में है। हर ड्राइव, वह कितनी भरी और कितनी ठीक है, और USB स्टिक ड्राइव में हैं।

# Bold line of the setting that opens Activity.
settings-about-activity = देखें क्या चल रहा है

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = मेमोरी और प्रोसेसर इस्तेमाल करने वाले प्रोग्राम।

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = गतिविधि

# Bold line of the setting that opens Drives.
settings-about-drives = ड्राइव और जगह

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = हर ड्राइव, कितनी भरी और कितनी ठीक।

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ड्राइव

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = चालू होने का समय Downloads में सहेजा नहीं जा सका।

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Downloads में, { $file } के नाम से

# Label of the word button that saves every start-up time.
settings-about-times-save = सहेजें

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = चालू होने का हर समय Downloads में सहेजें

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = हर बार चालू होना, रखने के लिए

# Quiet line under that setting.
settings-about-times-keep-sub = स्प्रेडशीट के रूप में Downloads में सहेजा जाता है।

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = हर स्तंभ एक बार चालू होना है, सबसे नया दाईं ओर।

# Heading of the card with the last ten start-up times.
settings-about-times = पिछले दस बार चालू होना

# Text behind the info button of the start-up times card.
settings-about-times-about = हर बार का समय पावर बटन से लेकर लॉगिन स्क्रीन तक गिना जाता है; डिस्क का पासवर्ड लिखने में लगा समय छोड़ दिया जाता है। सबसे नया सबसे ऊपर है।

# Heading of the memory tile.
settings-about-memory = मेमोरी

# Heading of the disk tile.
settings-about-disk = डिस्क


# Heading of the card with the switch for looking for updates.
settings-about-updates = अपडेट

# Text behind the info button of the updates card.
settings-about-updates-about = लॉग इन करने के कुछ देर बाद और फिर हर कुछ घंटों में Trindle ऐप्स, प्लगइन, फ़र्मवेयर और खुद के नए संस्करण खोजता है, और Debian के सुरक्षा सुधार हर दिन लगते हैं। बंद करने पर Trindle केवल तब देखता है जब आप ऐप्स में अपडेट जाँचें दबाते हैं, या रिंग पर अपडेट का बटन। Debian के सुरक्षा सुधारों का अपना स्विच है; बंद करने पर वे अपने आप नहीं लगते और ज्ञात खामियाँ आपके अपडेट करने तक खुली रहती हैं। किसी भी स्विच को बदलने पर आपके एडमिनिस्ट्रेटर का पासवर्ड माँगा जाता है, ताकि कंप्यूटर की अपनी सेवाएँ भी यह चुनाव देख सकें।

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = अपडेट जाँचें

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = पासवर्ड का इंतज़ार है...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = बंद: कहने तक कुछ नहीं देखा जाता।



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = सुरक्षा अपडेट

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = सुरक्षा सुधार हर दिन अपने आप लगते हैं।

# Warning line under the Security updates switch when it is off.
settings-about-security-off = सुरक्षा सुधार अब अपने आप नहीं लगते।

