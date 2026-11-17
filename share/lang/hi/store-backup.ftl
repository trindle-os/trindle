
# machine translated (Hindi), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = कुछ भी वापस नहीं रखा गया, क्योंकि { $why }।

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Trindle ने बैकअप का केवल एक हिस्सा वापस रखा, क्योंकि { $why }।

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = बैकअप में कोई लिंक या कुछ और है जो उस फ़ोल्डर के बाहर पहुँचेगा जिसमें वह जाता है

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = इस कंप्यूटर पर वहाँ पहले से कुछ है जहाँ बैकअप के एक हिस्से को जाना है

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = बैकअप खोलने के लिए कोई जगह नहीं है

# The reason when the file is not a Trindle backup.
store-because-not-trindle = वह फ़ाइल Trindle का बैकअप नहीं है

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = बैकअप खराब है और Trindle उसकी सूची पढ़ नहीं सका

# The reason when the file is not a Flat backup.
store-because-not-flat = वह फ़ाइल Flat के बैकअप जैसी नहीं दिखती

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = उस फ़ाइल में Flat की ऐप सूची नहीं है

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = यह Flat के ऐसे नए संस्करण ने बनाया है जिसे Trindle पढ़ नहीं सकता

# The reason when the file holds no apps.
store-because-no-apps = उस फ़ाइल में कोई ऐप नहीं है

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = बैकअप पढ़ा नहीं जा सका: { $why }

# The reason when the program gave none.
store-because-none = कोई कारण नहीं बताया गया

# The reason when the system said permission was denied.
store-because-permission = Trindle को वहाँ लिखने की अनुमति नहीं है

# The reason when the disk is full.
store-because-space = इस कंप्यूटर पर जगह नहीं बची है

# The reason when a part of the computer is read-only.
store-because-read-only = इस कंप्यूटर का वह हिस्सा केवल पढ़ने के लिए है

# The reason when the person's disk quota is used up.
store-because-quota = इस खाते के लिए अनुमत जगह पूरी हो चुकी है

# The reason when a file or program is missing.
store-because-missing = इसके लिए ज़रूरी कोई फ़ाइल या प्रोग्राम मौजूद नहीं है

# The reason when no reason could be read from the program's last line.
store-because-failed = यह विफल रहा

# The reason when the backup has no name to be written under.
store-because-no-name = इसका कोई नाम नहीं है

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = बैकअप में इसकी सेटिंग्स इसके अपने फ़ोल्डर के बाहर पहुँचती हैं

# The reason when something is too big to be read whole.
store-because-too-big = यह इतना बड़ा है कि पढ़ा नहीं जा सकता

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = बैकअप में इसकी सेटिंग्स खराब हो गई हैं या बदल दी गई हैं

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = आपके खाते की तस्वीर को छोड़कर सब कुछ वापस रख दिया गया। { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Trindle ने आपके खाते की तस्वीर भी वापस नहीं रखी। { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = पासवर्ड माँगा नहीं जा सका: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Trindle इसे बैकअप से पढ़ नहीं सका: { $why }।

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Trindle ने बैकअप नहीं बनाया, क्योंकि { $why }।

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = अभी कुछ नहीं

# Beside the size of a part of the backup: about how big it is.
store-backup-about = लगभग { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindle की सेटिंग्स और AppImage वापस रखे जा रहे हैं…

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Trindle ने आपकी सेटिंग्स और डेटा वापस रख दिए, पर Flatpak ये ऐप्स इंस्टॉल नहीं कर सका: { $failed }।

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Trindle ने आपकी सेटिंग्स और डेटा वापस रख दिए, पर उसने ये ऐप्स इंस्टॉल नहीं किए: { $refused }।

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] Trindle ने 1 ऐप छोड़ दिया, क्योंकि उसका नाम सही नहीं था।
       *[other] Trindle ने { $n } ऐप छोड़ दिए, क्योंकि उनके नाम सही नहीं थे।
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = { $name } स्रोत एन्क्रिप्टेड नहीं है, इसलिए Trindle ने उसे इस्तेमाल नहीं किया

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } स्रोत इस कंप्यूटर पर नहीं है, और बैकअप यह नहीं बताता कि वह कहाँ है

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name } की सेटिंग्स (क्योंकि बैकअप में उनकी प्रति जाँच में पास नहीं हुई)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (क्योंकि इसके स्रोत का नाम सही नहीं है)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (क्योंकि { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (Flatpak इसे इंस्टॉल नहीं कर सका: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = सब कुछ (क्योंकि बैकअप खोलने के लिए कोई जगह नहीं है)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = हर ऐप की अनुमतियाँ

# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name } की सेटिंग्स

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }। जो पहले वहाँ था वह { $path } के रूप में रखा गया है: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } इंस्टॉल हो रहा है ({ $total } में से { $at })…

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = { $name } की सेटिंग्स वापस रखी जा रही हैं ({ $total } में से { $at })…

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } ऐप इंस्टॉल हुआ।
       *[other] { $n } ऐप्स इंस्टॉल हुए।
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
        [one] एक ऐप की सेटिंग वापस रखी गई।
       *[other] { $n } ऐप की सेटिंग वापस रखी गईं।
    }

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } ऐप पहले से इंस्टॉल था।
       *[other] { $n } ऐप पहले से इंस्टॉल थे।
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = छोड़े गए: { $list }

# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }: Trindle ने सिर्फ़ वे अनुमतियाँ वापस रखीं जो पहुँच कम करती हैं।
