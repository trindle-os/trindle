
# machine translated (Hindi), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = लॉगिन पर शुरू

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay ऑटोस्टार्ट स्टार्टअप शुरू लॉगिन अपने-आप देरी

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = वे ऐप जो आपके लॉगिन करते ही अपने-आप खुल जाते हैं। देरी लगाने पर ऐप पहले नेटवर्क या किसी दूसरे ऐप के तैयार होने का इंतज़ार करता है।

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = लॉगिन करने पर कुछ भी अपने-आप शुरू नहीं होता।

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = लॉगिन करने पर { $name } अपने-आप शुरू होता है।

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] लॉगिन करने पर { $n } ऐप अपने-आप शुरू होता है।
       *[other] लॉगिन करने पर { $n } ऐप अपने-आप शुरू होते हैं।
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] एक और सूची में रखा है, पर बंद है।
       *[other] { $n } और सूची में रखे हैं, पर बंद हैं।
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = तुरंत

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = एक मिनट बाद

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = 2 मिनट बाद

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] { $n } सेकंड बाद
       *[other] { $n } सेकंड बाद
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name } के लिए देरी

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = लॉगिन पर इसे शुरू करें

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = लॉगिन पर इसे शुरू करना बंद करें

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = इसे हटाया नहीं जा सका, इसलिए यह अब भी लॉगिन पर शुरू होता है।

# Red words under the add card when something could not be added.
settings-startup-not-added = इसे जोड़ा नहीं जा सका।

# Placeholder of the search box above the list of installed apps.
settings-startup-find = कोई ऐप खोजें

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = इसे लॉगिन पर शुरू करें

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = इसे जोड़ा नहीं जा सका: ऐप अब इंस्टॉल नहीं है।

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = जैसे syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = डिस्क पर से कोई प्रोग्राम चुनें

# Heading of the line for adding a command.
settings-startup-command = कमांड

# Small words under the command line.
settings-startup-command-about = जैसा टर्मिनल में लिखा जाता है, या डिस्क पर कोई प्रोग्राम।

# Red words under the command line when nothing was typed.
settings-startup-command-empty = कोई कमांड लिखें, या कोई प्रोग्राम चुनें।

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = लॉगिन पर शुरू करने के लिए कोई प्रोग्राम चुनें

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = कुछ भी अपने-आप शुरू नहीं होता

# Small words under that line.
settings-startup-none-about = इसके पास कोई ऐप जोड़ें।

# Title of the card listing what starts at login.
settings-startup-list-title = लॉगिन पर शुरू होते हैं

# Words behind the i of the card listing what starts at login.
settings-startup-about = लॉगिन के बाद हर ऐप कुछ देर रुक सकता है, ताकि नेटवर्क या कोई दूसरा ऐप पहले तैयार हो जाए। उसका स्विच उसे यहाँ रखता है पर शुरू नहीं करता; कूड़ेदान उसे लॉगिन पर शुरू होने से पूरी तरह रोक देता है।

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } यह चलाता है: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = इंस्टॉल किए हुए ऐप में से चुनें

# Name of the button that opens the box for typing a command.
settings-startup-open-command = कोई कमांड लिखें, या कोई प्रोग्राम चुनें

# Heading of the line for adding an installed app.
settings-startup-app-heading = इंस्टॉल किया हुआ ऐप

# Small words under that line.
settings-startup-app-about = ऐप की सूची में से एक चुनें।

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = कमांड या प्रोग्राम

# Small words under that line.
settings-startup-cmd-about = लिखकर, या डिस्क से चुनकर।

# Words behind the i of the Add an app card.
settings-startup-add-about = इंस्टॉल किए हुए ऐप में से चुनें, या चलाने के लिए कोई कमांड दें, जैसा आप टर्मिनल में लिखते हैं, या डिस्क से चुना हुआ कोई प्रोग्राम। यह अगले लॉगिन पर शुरू होगा, तुरंत नहीं; सूची में आने के बाद इसके लिए इंतज़ार का समय चुना जा सकता है।

# Title of the card for adding an app.
settings-startup-add-title = ऐप जोड़ें

