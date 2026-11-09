# machine translated (Hindi), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = सहेजा गया

# Said beside a button that worked, when nothing more is known.
settings-common-done = हो गया

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = नहीं हुआ

# Tip of the information button at the top of every Settings page.
settings-common-about-page = इस पेज के बारे में

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = { $name } के बारे में

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } पहले से मौजूद है

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] बाकी सब के साथ भी यही करें ({ $n } और)
       *[other] बाकी सब के साथ भी यही करें ({ $n } और)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = बाकी सब के साथ भी यही करें

# Tip of the button that keeps the one already there.
# SAFETY: needs a human review before it ships
settings-common-skip = छोड़ें: जो पहले से है उसे रखें

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = बदलें

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = जो पहले से है उसकी जगह लाए गए को रखें

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = जो लाया जा रहा है

# Heading of the line that shows the one already there.
settings-common-already-line = जो पहले से है

# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = जो पहले से है उसे बदलें?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] यह { $left } सेकंड में वापस हो जाएगा।
       *[other] यह { $left } सेकंड में वापस हो जाएगा।
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] यह { $left } सेकंड में वापस हो जाएगा। { $more }
       *[other] यह { $left } सेकंड में वापस हो जाएगा। { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = अभी वापस जाएँ

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = रखें

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } उपलब्ध नहीं है

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = इसे इस कमांड से इंस्टॉल करें: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = वेब पेज खोला नहीं जा सका: कोई वेब ब्राउज़र शुरू नहीं हो सका।

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } खोला नहीं जा सका।

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = इसे सहेजा नहीं जा सका।

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = नहीं हुआ: सिस्टम की मैसेज बस तक पहुँचा नहीं जा सका। कंप्यूटर फिर से चालू करें, और दोबारा कोशिश करें।

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = नहीं हुआ: इस लॉगिन की मैसेज बस तक पहुँचा नहीं जा सका। लॉग आउट करके फिर लॉग इन करें, और दोबारा कोशिश करें।

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = चुनने वाली विंडो खोली नहीं जा सकी। लॉग आउट करके फिर लॉग इन करें, और दोबारा कोशिश करें।

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = डिस्क भरी हुई है।

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = डिस्क पर अभी लिखा नहीं जा सकता।

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = सिस्टम का जो हिस्सा यह करता है वह चल नहीं रहा।

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = जवाब देने में बहुत देर लगी। फिर कोशिश करें।

# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = पासवर्ड नहीं दिया गया।

# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = इस खाते को इसकी अनुमति नहीं है।

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = इस कंप्यूटर में इसके लिए ज़रूरी कुछ मौजूद नहीं है।

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } और { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = रिंग स्क्रीन के किनारे पर क्लिक करने से खुलती है।

# Word drawn in the middle of that picture.
settings-common-ring-name = रिंग

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = लैपटॉप के फ़र्मवेयर तक पहुँचा नहीं जा सका: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = लैपटॉप के फ़र्मवेयर ने चार्ज की सीमा नहीं मानी।

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = यह कंप्यूटर अपनी चार्जिंग को बीच में रोकने नहीं देता।

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path } सेट नहीं किया जा सका: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = वर्चुअल मशीन में Trindle आज़माना

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle { $machine } में चल रहा है। वहाँ यह अच्छे से काम करता है, पर वर्चुअल मशीन असली कंप्यूटर जितनी तेज़ या सहज कभी नहीं होती, इसलिए इसकी गति असली हार्डवेयर पर परखें।

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle { $machine } में 3D ग्राफ़िक्स के बिना चल रहा है, इसलिए प्रोसेसर को वह सब बनाना पड़ता है जो आम तौर पर ग्राफ़िक्स चिप बनाती है। विंडो, वीडियो और वेब पेज असली कंप्यूटर से धीमे लगेंगे, और मदद के लिए Trindle अपने एनिमेशन छोड़ देता है। अगर जिस कंप्यूटर पर यह चल रहा है वह कर सके, तो वर्चुअल मशीन की डिस्प्ले सेटिंग्स में 3D एक्सेलरेशन चालू करने से बहुत फ़र्क पड़ता है। असली कंप्यूटर पर इंस्टॉल करने पर इनमें से कुछ भी लागू नहीं होता।
