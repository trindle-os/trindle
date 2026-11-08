
# machine translated (Hindi), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }।

# Under the app's name when only the maker is known.
store-app-line-by = { $by } द्वारा।

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, { $by } द्वारा।

# The state beside the dot while the app is being taken off.
store-app-state-removing = हटाया जा रहा है

# The state beside the dot while the app is being fetched.
store-app-state-downloading = डाउनलोड हो रहा है

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = लगाया जा रहा है

# The state beside the dot while the app's own update goes in.
store-app-state-updating = अपडेट हो रहा है

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = इंस्टॉल है, रोका हुआ

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = इंस्टॉल है, संस्करण { $version } पर रोका हुआ

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = इंस्टॉल है, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = इंस्टॉल है, संस्करण { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = इंस्टॉल है, अद्यतन है

# The same, with the installed version.
store-app-state-current-version = इंस्टॉल है, संस्करण { $version }, अद्यतन है

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = इंस्टॉल नहीं है

# The same, with the size of the download.
store-app-state-absent-size = इंस्टॉल नहीं है, { $size } डाउनलोड करना होगा

# Word beside the spinner while the app is being taken off.
store-app-busy-removing = हटाया जा रहा है

# Word beside the spinner while the app is being put in.
store-app-busy-installing = इंस्टॉल हो रहा है

# Tip and spoken name of the round button that gives up a download.
store-app-stop = रोकें

# Tip and spoken name of the round button that takes the app off.
# SAFETY: needs a human review before it ships
store-app-remove = हटाएँ

# Round button beside Update or Release that opens the app.
store-app-open-round = खोलें

# Main button that releases an app whose updates are held back.
store-app-release = रिलीज़

# Main button that updates the app.
store-app-update = अपडेट करें

# Main button that opens the app.
store-app-open = खोलें

# Main button that installs the app.
store-app-install = इंस्टॉल करें

# Button that opens the app's website.
store-app-website = वेबसाइट

# In the picture frame when the app has no pictures.
store-app-no-pictures = इस ऐप के कोई चित्र नहीं हैं।

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = यह चित्र लाया नहीं जा सका।

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = चित्र लाया जा रहा है…

# In the picture frame when the computer has no internet.
store-app-picture-offline = जब तक यह कंप्यूटर इंटरनेट से जुड़ा नहीं है, चित्र नहीं लाए जा सकते।

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = { $n } में से चित्र { $at }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = { $n } में से चित्र { $at }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = पिछला चित्र

# Tip of the round button that shows the picture after.
store-app-picture-later = अगला चित्र

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = स्वामित्व वाला

# Name of the Facts card.
store-app-facts = जानकारी

# Facts line: the size of the download.
store-app-fact-download = डाउनलोड का आकार

# Facts line: where the app comes from.
store-app-fact-source = स्रोत

# Facts line: the version.
store-app-fact-version = संस्करण

# Facts line: the licence of an open source app.
store-app-fact-licence = लाइसेंस

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = लाइसेंस, ओपन सोर्स नहीं

# Facts line: who made the app.
store-app-fact-made-by = बनाने वाले

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  { $n ->
        [one] { $count } रेटिंग से
       *[other] { $count } रेटिंग से
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = अभी कोई रेटिंग नहीं।

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = दिखाने के लिए कोई रेटिंग नहीं।

# Under the stars while the ratings are being fetched.
store-app-rating-looking = रेटिंग खोजी जा रही हैं…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = GNOME की खुली समीक्षा सेवा से।

# Last line of the Rating card when there is no internet.
store-app-rating-offline = यह कंप्यूटर इंटरनेट से जुड़ा नहीं है।

# Name of the Rating card.
store-app-rating = रेटिंग

# Text of the About card when the app says nothing of itself.
store-app-no-description = इस ऐप का कोई विवरण नहीं है।

# Name of the About card.
store-app-about = परिचय

# Heading of a review that has no title.
store-app-review-no-title = कोई शीर्षक नहीं

# Under What people say when nobody has written.
store-app-people-none = अभी तक किसी ने इस ऐप के बारे में नहीं लिखा।

# Under What people say when there is no internet.
store-app-people-offline = जब तक यह कंप्यूटर इंटरनेट से जुड़ा नहीं है, लोगों की राय नहीं लाई जा सकती।

# Under What people say while reviews are being fetched.
store-app-people-looking = लोगों की राय खोजी जा रही है…

# Name of the card of reviews.
store-app-people = लोग क्या कहते हैं

# Facts line: when the app's project was made.
store-app-fact-created = बनाया गया

