# machine translated (Hindi), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = इसकी सेटिंग खोलें

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = नहीं हुआ: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = इस अपडेट को एक नई अनुमति चाहिए। यह आपकी प्रतीक्षा में है।

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = अभी संस्करण { $mine } है, { $new } तैयार है

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = अभी संस्करण { $mine } है, { $new } तैयार है। यह पहले से ज़्यादा माँगता है: { $more }। { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = यहाँ के बटन इसके पूरा होने पर लौट आएँगे।

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = प्लगइन खोजे जा रहे हैं…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = कोई प्लगइन इंस्टॉल नहीं है, और अभी कोई उपलब्ध भी नहीं है।

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] अभी कोई प्लगइन इंस्टॉल नहीं है; 1 इंस्टॉल के लिए तैयार है।
       *[other] अभी कोई प्लगइन इंस्टॉल नहीं है; { $n } इंस्टॉल के लिए तैयार हैं।
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 प्लगइन इंस्टॉल है
       *[other] { $n } प्लगइन इंस्टॉल हैं
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 का अपडेट आया है
       *[other] { $n } के अपडेट आए हैं
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 अपडेट रोका गया है
       *[other] { $n } अपडेट रोके गए हैं
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }।

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }।

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, सब अद्यतन हैं।

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, और { $have }।

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, और { $have }; { $held }।

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = यह कुछ नहीं माँगता।

# On a plugin's card: what it asks for.
store-plugins-asks = यह जो माँगता है: { $list }।

# Title of the Plugins page.
store-plugins-title = प्लगइन

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = संस्करण { $mine } इंस्टॉल है; { $new } तैयार है।

# On a plugin's card: its installed version.
store-plugins-version-installed = संस्करण { $version }, इंस्टॉल है।

# On a plugin's card: the version on offer.
store-plugins-version-offered = संस्करण { $version }।

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }।

# Tip of the round button that installs a plugin.
store-plugins-install = { $name } इंस्टॉल करें

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = { $name } जारी करें

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = { $version } पर अपडेट करें

# Heading of the card that says where plugins come from.
store-plugins-where = प्लगइन कहाँ से आते हैं

# Information on the card that says where plugins come from.
store-plugins-come-from = प्लगइन केवल Trindle की अपनी सूची से आते हैं, जो यह पन्ना खुलने पर हर बार trindle.org से पढ़ी जाती है। कोई प्लगइन देने के लिए उसे जमा करें: हर एक को चालू होने से पहले परखा जाता है।

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = यह इंस्टॉल है, पर इसका बैकग्राउंड हिस्सा चल नहीं रहा: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = इसे हटाया नहीं जा सका: { $why }
