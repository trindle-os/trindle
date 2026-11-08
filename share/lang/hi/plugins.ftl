
# machine translated (Hindi), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = फ़ोल्डर के शीर्ष पर कोई पढ़ने योग्य { $file } नहीं है ({ $why })।

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } में कोई [Plugin] भाग नहीं है।

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } ऐसा नहीं है जिसे यह Trindle जानता हो: यह Interface={ $known } जानता है।

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] को Interface={ $known } चाहिए।

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] को अक्षरों, अंकों, बिंदुओं और डैश वाला Id चाहिए, जैसे org.example.Weather।

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] को एक Name चाहिए।

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] को एक Version चाहिए।

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] का Summary एक पंक्ति का होता है।

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } एक पंक्ति के सादे शब्द हैं, ज़्यादा से ज़्यादा { $max } अक्षर।
       *[other] [{ $group }] { $key } एक पंक्ति के सादे शब्द हैं, ज़्यादा से ज़्यादा { $max } अक्षर।
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = अनुमति { $name } ऐसी कोई अनुमति नहीं है: उपलब्ध अनुमतियाँ ये हैं { $list }।

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] को एक Icon चाहिए: प्लगइन के फ़ोल्डर में एक SVG या PNG फ़ाइल।

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] का Icon { $icon } को नाम देता है, जो प्लगइन के फ़ोल्डर में नहीं है।

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] को एक Label चाहिए, जो पॉइंटर के ठहरने पर बोला जाता है।

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] को Run चाहिए, यानी क्लिक से क्या शुरू होता है।

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] को Fields चाहिए, यानी उसके फ़ील्ड के नाम।

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = फ़ील्ड का नाम { $name } सिर्फ़ अक्षरों, अंकों, डैश और अंडरस्कोर का हो सकता है।

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = फ़ील्ड { $name } का कोई [{ $group }] भाग नहीं है।

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] को एक Label चाहिए।

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices ज़्यादा से ज़्यादा { $most } हैं, हर एक सादे शब्दों का, ज़्यादा से ज़्यादा { $chars } अक्षर।
       *[other] [{ $group }] Choices ज़्यादा से ज़्यादा { $most } हैं, हर एक सादे शब्दों का, ज़्यादा से ज़्यादा { $chars } अक्षर।
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom प्लगइन के होम में एक फ़ाइल का नाम है: अक्षर, अंक, बिंदु, डैश और अंडरस्कोर, बिंदु से शुरू नहीं, कोई फ़ोल्डर नहीं।

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] एक विकल्प है, इसलिए इसे दो या अधिक Choices चाहिए, या ChoicesFrom में एक फ़ाइल।

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] का Default उसके Choices में से एक नहीं है।

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] एक टिप्पणी है, इसलिए इसे TextFrom चाहिए, यानी प्लगइन के होम में एक फ़ाइल का नाम: अक्षर, अंक, बिंदु, डैश और अंडरस्कोर, बिंदु से शुरू नहीं, कोई फ़ोल्डर नहीं।

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] एक टिप्पणी है, जिसमें कोई सेटिंग नहीं होती, इसलिए इसका कोई Default नहीं है।

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] का Kind { $kind } है: यह switch, text, choice या note होना चाहिए।

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] एक स्विच है, इसलिए इसका Default true या false होता है।

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] को [Plugin] Permissions में background अनुमति चाहिए।

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = प्लगइन किसी चीज़ से नहीं जुड़ता: उसे [Ring], [Settings], [Background], [Shortcut] या सूचनाओं की अनुमति दें।

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } प्लगइन के फ़ोल्डर के बाहर कड़ी बनाता है।

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] का Run आदेश के रूप में पढ़ा नहीं जा सकता।

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] का Run प्लगइन के फ़ोल्डर के अंदर के प्रोग्राम का नाम होना चाहिए, { $program } नहीं।

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] का Run { $program } को नाम देता है, जो प्लगइन के साथ आने वाला प्रोग्राम नहीं है (गुम है, या चलाने योग्य नहीं)।

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] के Menu में { $n } चीज़ है: ज़्यादा से ज़्यादा { $max } की अनुमति है।
       *[other] [Ring] के Menu में { $n } चीज़ें हैं: ज़्यादा से ज़्यादा { $max } की अनुमति है।
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Menu की आईडी { $id } सिर्फ़ अक्षरों, अंकों, डैश और अंडरस्कोर की हो सकती है।

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] का Menu { $id } को दो बार नाम देता है।

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] का Menu { $id } को नाम देता है, जिसका कोई [{ $group }] भाग नहीं है।

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] का Label एक छोटी पंक्ति है, ज़्यादा से ज़्यादा { $max } अक्षर।
       *[other] [{ $group }] का Label एक छोटी पंक्ति है, ज़्यादा से ज़्यादा { $max } अक्षर।
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] को Run चाहिए, यानी इसे चुनने से क्या शुरू होता है।

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] सिर्फ़ Trindle की सूची के Flow के लिए है, जो control अनुमति माँगता है।

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] को Run चाहिए, वह आदेश जिसे PIN दिया जाता है।

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] का ShowWhen ज़्यादा से ज़्यादा { $max } अक्षर का है।
       *[other] [{ $group }] का ShowWhen ज़्यादा से ज़्यादा { $max } अक्षर का है।
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] का ShowWhen field=value के रूप में लिखा जाता है, जैसे ShowWhen=mode=home: एक दूसरा फ़ील्ड और वह एक मान जो उसमें होना चाहिए।

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] का ShowWhen field=value के रूप में लिखा जाता है, और फ़ील्ड का नाम सिर्फ़ अक्षरों, अंकों, डैश और अंडरस्कोर का हो सकता है।

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] के ShowWhen को एक पंक्ति के सादे शब्दों का मान चाहिए, ज़्यादा से ज़्यादा { $max } अक्षर।
       *[other] [{ $group }] के ShowWhen को एक पंक्ति के सादे शब्दों का मान चाहिए, ज़्यादा से ज़्यादा { $max } अक्षर।
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] का ShowWhen अपने ही फ़ील्ड को नाम देता है: उसे कोई दूसरा, पहले वाला फ़ील्ड नाम देना होगा।

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] का ShowWhen फ़ील्ड { $other } को नाम देता है, जो इस प्लगइन के Fields में नहीं है।

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] का ShowWhen फ़ील्ड { $other } को नाम देता है, जो Fields में इसके बाद आता है: यह पहले वाला फ़ील्ड होना चाहिए।

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] का ShowWhen फ़ील्ड { $other } को नाम देता है, जिसका अपना ShowWhen है: कोई फ़ील्ड सिर्फ़ ऐसे फ़ील्ड पर निर्भर हो सकता है जिसका कोई ShowWhen न हो।

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] का ShowWhen स्विच { $other } को नाम देता है, इसलिए उसका मान true या false है।

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] के ShowWhen का मान { $value } है, जो { $other } के Choices में से एक नहीं है।

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] का ShowWhen टिप्पणी { $other } को नाम देता है, जिसमें कोई सेटिंग नहीं होती: कोई फ़ील्ड सिर्फ़ स्विच, विकल्प या टेक्स्ट पर निर्भर हो सकता है।

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [नेटवर्क] सुनना: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = [Network] Listen को [Plugin] Permissions में network अनुमति चाहिए।

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = [Network] Listen को [Plugin] Permissions में listen अनुमति चाहिए।

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = listen अनुमति को [Network] Listen चाहिए, यानी अंदर आने देने वाले पोर्ट।

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = इसकी plugin.ini ऐसे इंटरफ़ेस की नहीं है जिसे यह Trindle जानता हो।

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = यह सुनने के लिए कोई पोर्ट नाम नहीं देता।

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = इसकी plugin.ini पढ़ी नहीं जा सकी।

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = इसे सुनने की अनुमति नहीं दी गई है।

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = जो पोर्ट यह नाम देता है वे वे नहीं हैं जिनकी अनुमति दी गई थी। उनकी अनुमति फिर देनी होगी।

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name } को काम करने के लिए आपकी अनुमति चाहिए: { $n } में से 0 की अनुमति है।

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } के पास जो अनुमति वह माँगता है उसका सिर्फ़ कुछ हिस्सा है: { $n } में से { $a } की अनुमति है।

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] यह कुछ नहीं माँगता।
        [one] जो एक चीज़ यह माँगता है उसकी अनुमति है।
       *[other] जो सभी { $n } चीज़ें यह माँगता है उनकी अनुमति है।
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = अनुमति दें

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = पहले से अलग पोर्ट पर पहुँचा जा सकेगा

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = इंस्टॉल नहीं है।

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = कोई और प्लगइन

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = वह प्लगइन इंस्टॉल नहीं है।

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = इसे सहेजा नहीं जा सका: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = प्लगइन की फ़ाइलें कॉपी नहीं की जा सकीं।

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } इसका पृष्ठभूमि वाला हिस्सा चल नहीं रहा: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = इसे शुरू करने के लिए trindle-plugin नहीं मिला।

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] यह { $n } सेकंड के भीतर शुरू नहीं हुआ।
       *[other] यह { $n } सेकंड के भीतर शुरू नहीं हुआ।
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = इसके जुड़ने के लिए कोई डेस्कटॉप सेशन नहीं चल रहा, इसलिए इसे शुरू नहीं किया गया। पहले लॉग इन करें।

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = यह तय नहीं कर सकता कि किस डेस्कटॉप सेशन से जुड़ना है ({ $sessions }), इसलिए इसे शुरू नहीं किया गया। WAYLAND_DISPLAY को सही वाले पर सेट करके इसे फिर चलाएँ।

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = फ़ाइल की जगह पर एक फ़ोल्डर है

# Error text when a plugin's id is not an id.
plugins-not-an-id = किसी प्लगइन की आईडी नहीं

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] के Shortcuts में { $n } शॉर्टकट है: ज़्यादा से ज़्यादा { $max } की अनुमति है।
       *[other] [Plugin] के Shortcuts में { $n } शॉर्टकट हैं: ज़्यादा से ज़्यादा { $max } की अनुमति है।
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = शॉर्टकट की आईडी { $id } सिर्फ़ अक्षरों, अंकों, डैश और अंडरस्कोर की हो सकती है।

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] के Shortcuts में { $id } दो बार नाम आया है।

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] के Shortcuts में { $id } का नाम है, जिसका कोई [{ $group }] भाग नहीं है।

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] की Keys { $keys } पहले से Trindle का शॉर्टकट है; दूसरी कुंजियाँ चुनें।

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] की Keys { $keys } प्लगइन के किसी दूसरे शॉर्टकट में इस्तेमाल हो रही हैं।

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] को Keys चाहिए, जैसे Ctrl+Alt+S। { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] को Run चाहिए, यानी कुंजियाँ क्या शुरू करती हैं।

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = इसके लिए लिखी कुंजियाँ शॉर्टकट नहीं हैं। { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } पहले से Trindle का शॉर्टकट है।

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } पहले से आपका अपना एक शॉर्टकट है।

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } पहले से { $holder } द्वारा इस्तेमाल हो रहा है।

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = यह किसी इंस्टॉल किए प्लगइन का शॉर्टकट नहीं है।

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = इसे इस्तेमाल में नहीं लाया जा सका ({ $why })।

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = इसे सहेजा नहीं जा सका: कुछ नहीं बदला गया, क्योंकि { $why }।

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = इसे सहेजा नहीं जा सका ({ $why })।



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = सुलभता की अनुमति केवल सूची में मौजूद Trindle के स्क्रीन रीडर के लिए है।

