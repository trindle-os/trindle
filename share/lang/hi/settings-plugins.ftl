# machine translated (Hindi), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = आपके प्लगइन

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = plugins add-ons extensions installed remove settings प्लगइन एक्सटेंशन इंस्टॉल हटाएँ सेटिंग

# Help line at the foot of the Your plugins page.
settings-plugins-help = प्लगइन Trindle में कुछ जोड़ते हैं: रिंग में एक आइकन, यहाँ सेटिंग, सूचनाएँ, या कुछ ऐसा जो लॉगिन से चलता है। हर प्लगइन अलग घेरे में चलता है और जब तक आप अनुमतियाँ में अनुमति न दें, उसे कुछ भी करने की अनुमति नहीं होती; अनुमतियाँ इस पेज के शीर्षक के अंत वाले बटन से खुलती हैं। यहाँ हर प्लगइन के कार्ड में उसकी अपनी सेटिंग हैं; कार्ड पर कूड़ेदान को दो बार दबाने से वह प्लगइन हट जाता है।

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = वह विंडो नहीं खुल सकी जो पूछती है कि प्लगइन क्या इस्तेमाल कर सकता है।

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = संस्करण { $v }।

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = संस्करण { $v }। { $settled }

# SAFETY: needs a human review before it ships
# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } कुछ नहीं माँगता, इसलिए वह अलग घेरे में चलता है और उसे कुछ भी करने की अनुमति नहीं है।

# SAFETY: needs a human review before it ships
# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } { $list } माँगता है; { $asks } में से { $allowed } की अनुमति है। वह अलग घेरे में चलता है, केवल उसी के साथ जिसकी आप अनुमति दें।
       *[other] { $name } { $list } माँगता है; { $asks } में से { $allowed } की अनुमति है। वह अलग घेरे में चलता है, केवल उसी के साथ जिसकी आप अनुमति दें।
    }

# SAFETY: needs a human review before it ships
# Under the line that tells a plugin needs more allowed, beside its Allow button.
settings-plugins-needs-more = अनुमति दें दिखाता है कि वह क्या-क्या माँगता है, हर एक के लिए एक स्विच के साथ।

# Heading of the line about a plugin on its card.
settings-plugins-about-it = इसके बारे में

# SAFETY: needs a human review before it ships
# Tip of the button that changes what a plugin may use.
settings-plugins-change = बदलें कि वह क्या इस्तेमाल कर सकता है

# SAFETY: needs a human review before it ships
# Tip of the button that removes a plugin.
settings-plugins-remove = इस प्लगइन को हटाएँ

# SAFETY: needs a human review before it ships
# Red words when a plugin could not be removed.
settings-plugins-not-removed = इसे हटाया नहीं जा सका।

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = यहाँ लिखें

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } की अपनी कोई सेटिंग नहीं है।

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } की अभी दिखाने लायक कोई सेटिंग नहीं है।

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] कोई प्लगइन इंस्टॉल नहीं है।
        [one] एक प्लगइन इंस्टॉल है: { $list }।
       *[other] { $n } प्लगइन इंस्टॉल हैं: { $list }।
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = प्लगइन

# Heading of the line shown when no plugin is installed.
settings-plugins-none = कोई इंस्टॉल नहीं है

# Under the None installed heading.
settings-plugins-none-sub = ऐप्स में जोड़ने के लिए प्लगइन हैं।

# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = अनुमतियाँ खोलें

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name } से पूछा जा रहा है।

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } जवाब नहीं दे रहा। हो सकता है उसकी सेवा रुकी हो।

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = सहेजा गया।

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } ने इसे स्वीकार नहीं किया।

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } यह नहीं कर सका।

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } यह नहीं कर सका: { $why }।

