
# machine translated (Hindi), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle फ़ोल्डर के शीर्ष पर मौजूद { $file } नहीं पढ़ सकता ({ $why })।

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } में कोई [Plugin] भाग नहीं है।

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Trindle का यह संस्करण Interface={ $known } को समर्थन देता है, Interface={ $n } को नहीं।

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
        [one] [{ $group }] { $key } एक पंक्ति का सादा टेक्स्ट होना चाहिए, जो { $max } अक्षर से ज़्यादा लंबा न हो।
       *[other] [{ $group }] { $key } एक पंक्ति का सादा टेक्स्ट होना चाहिए, जो { $max } अक्षर से ज़्यादा लंबा न हो।
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = अनुमति { $name } मौजूद नहीं है। उपलब्ध अनुमतियाँ ये हैं: { $list }।

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] को एक Icon चाहिए: प्लगइन के फ़ोल्डर में एक SVG या PNG फ़ाइल।

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] का Icon { $icon } को नाम देता है, पर वह प्लगइन के फ़ोल्डर में नहीं है।

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] को एक Label चाहिए, जो पॉइंटर के ठहरने पर दिखाया जाता है।

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] को Fields चाहिए, यानी उसके फ़ील्ड के नाम।

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = फ़ील्ड के नाम { $name } में केवल अक्षर, अंक, डैश और अंडरस्कोर हो सकते हैं।

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = फ़ील्ड { $name } का कोई [{ $group }] भाग नहीं है।

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] को एक Label चाहिए।

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] में ज़्यादा से ज़्यादा { $most } Choices हो सकते हैं। हर एक सादा टेक्स्ट होना चाहिए, जो { $chars } अक्षर से ज़्यादा लंबा न हो।
       *[other] [{ $group }] में ज़्यादा से ज़्यादा { $most } Choices हो सकते हैं। हर एक सादा टेक्स्ट होना चाहिए, जो { $chars } अक्षर से ज़्यादा लंबा न हो।
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom को प्लगइन के फ़ोल्डर की किसी फ़ाइल का नाम देना होगा। केवल अक्षर, अंक, बिंदु, डैश और अंडरस्कोर इस्तेमाल करें। नाम बिंदु से शुरू न करें और उसमें फ़ोल्डर का पाथ न रखें।

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] एक विकल्प है, इसलिए इसे दो या अधिक विकल्पों के साथ Choices चाहिए, या ChoicesFrom, यानी एक फ़ाइल।

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] का Default उसके Choices में से एक नहीं है।

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] एक टिप्पणी है, इसलिए TextFrom में प्लगइन के फ़ोल्डर की किसी फ़ाइल का नाम देना होगा। केवल अक्षर, अंक, बिंदु, डैश और अंडरस्कोर इस्तेमाल करें। नाम बिंदु से शुरू न करें और उसमें फ़ोल्डर का पाथ न रखें।

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] एक टिप्पणी है, इसलिए यह कोई सेटिंग नहीं सहेजती और इसका Default नहीं हो सकता।

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] का Kind { $kind } है, पर यह switch, text, choice या note होना चाहिए।

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] एक स्विच है, इसलिए इसका Default true या false होना चाहिए।

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] को [Plugin] Permissions में background अनुमति चाहिए।

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = प्लगइन किसी चीज़ से नहीं जुड़ता। उसे [Ring], [Settings], [Background], [Shortcut] या सूचनाओं की अनुमति दें।

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } एक लिंक है जो प्लगइन के फ़ोल्डर के बाहर ले जाता है।

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] का Run आदेश के रूप में पढ़ा नहीं जा सकता।

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] का Run प्लगइन के फ़ोल्डर के अंदर के प्रोग्राम का नाम होना चाहिए, { $program } नहीं।

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] का Run { $program } को नाम देता है, पर प्लगइन में वह प्रोग्राम नहीं है (वह गुम है या चलाने योग्य नहीं)।

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] के Menu में { $n } चीज़ है। ज़्यादा से ज़्यादा { $max } की अनुमति है।
       *[other] [Ring] के Menu में { $n } चीज़ें हैं। ज़्यादा से ज़्यादा { $max } की अनुमति है।
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Menu की आईडी { $id } में केवल अक्षर, अंक, डैश और अंडरस्कोर हो सकते हैं।

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] का Menu { $id } को दो बार नाम देता है।

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] का Menu { $id } को नाम देता है, जिसका कोई [{ $group }] भाग नहीं है।

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] का Label एक छोटी पंक्ति होना चाहिए, ज़्यादा से ज़्यादा { $max } अक्षर की।
       *[other] [{ $group }] का Label एक छोटी पंक्ति होना चाहिए, ज़्यादा से ज़्यादा { $max } अक्षर की।
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] सिर्फ़ Trindle की सूची के Flow के लिए है, जो control अनुमति माँगता है।

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] को Run चाहिए, वह आदेश जिसे PIN दिया जाता है।

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] का ShowWhen ज़्यादा से ज़्यादा { $max } अक्षर का हो सकता है।
       *[other] [{ $group }] का ShowWhen ज़्यादा से ज़्यादा { $max } अक्षर का हो सकता है।
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] के ShowWhen में field=value लिखना होगा, जैसे ShowWhen=mode=home। कोई दूसरा फ़ील्ड और वह मान बताएँ जो उसमें होना चाहिए।

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] का ShowWhen field=value के रूप में लिखना होगा, और फ़ील्ड के नाम में केवल अक्षर, अंक, डैश और अंडरस्कोर हो सकते हैं।

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] के ShowWhen को एक पंक्ति का सादा टेक्स्ट मान चाहिए, जो { $max } अक्षर से ज़्यादा लंबा न हो।
       *[other] [{ $group }] के ShowWhen को एक पंक्ति का सादा टेक्स्ट मान चाहिए, जो { $max } अक्षर से ज़्यादा लंबा न हो।
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] का ShowWhen अपने ही फ़ील्ड को नाम देता है। उसे कोई दूसरा, पहले वाला फ़ील्ड नाम देना होगा।

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] का ShowWhen फ़ील्ड { $other } को नाम देता है, जो इस प्लगइन के Fields में नहीं है।

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] का ShowWhen फ़ील्ड { $other } को नाम देता है, जो Fields में इसके बाद आता है। यह पहले वाला फ़ील्ड होना चाहिए।

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] का ShowWhen फ़ील्ड { $other } को नाम देता है, जिसका अपना ShowWhen है। कोई फ़ील्ड सिर्फ़ ऐसे फ़ील्ड पर निर्भर हो सकता है जिसका कोई ShowWhen न हो।

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] का ShowWhen स्विच { $other } को नाम देता है, इसलिए उसका मान true या false होना चाहिए।

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] के ShowWhen का मान { $value } है, जो { $other } के Choices में से एक नहीं है।

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] का ShowWhen टिप्पणी { $other } का हवाला देता है, जो कोई सेटिंग नहीं सहेजती। इसकी जगह स्विच, विकल्प या टेक्स्ट फ़ील्ड इस्तेमाल करें।

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = [Network] Listen को [Plugin] Permissions में network अनुमति चाहिए।

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = [Network] Listen को [Plugin] Permissions में listen अनुमति चाहिए।

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = listen अनुमति को [Network] Listen चाहिए, जिसमें उन पोर्ट की सूची हो जिन पर प्लगइन कनेक्शन स्वीकार कर सकता है।

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = इसकी plugin.ini ऐसा इंटरफ़ेस इस्तेमाल करती है जिसे Trindle का यह संस्करण समर्थन नहीं देता।

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = इसकी सूची में सुनने के लिए कोई पोर्ट नहीं है।

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = इसकी plugin.ini पढ़ी नहीं जा सकती।

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = इसे सुनने की अनुमति नहीं दी गई है।

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = सूची के पोर्ट बदल गए हैं। आपको उन पर कनेक्शन की अनुमति फिर देनी होगी।

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none =
    { $n ->
        [one] { $name } को काम करने से पहले आपकी अनुमति चाहिए। उसकी माँगी हुई एक अनुमति आपने नहीं दी है।
       *[other] { $name } को काम करने से पहले आपकी अनुमति चाहिए। उसकी माँगी हुई { $n } अनुमतियों में से आपने कोई नहीं दी है।
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } की माँगी हुई { $n } अनुमतियों में से आपने केवल { $a } दी हैं।

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] यह कुछ नहीं माँगता।
        [one] जो अनुमति इसे चाहिए, वह दी गई है।
       *[other] जो सभी { $n } अनुमतियाँ यह माँगता है, वे दी गई हैं।
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = अनुमति दें

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = अलग पोर्ट पर पहुँचा जाना

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = इंस्टॉल नहीं है।

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = कोई और प्लगइन

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = वह प्लगइन इंस्टॉल नहीं है।

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = यह सहेजा नहीं जा सका: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = प्लगइन की फ़ाइलें कॉपी नहीं हो सकीं।

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } प्लगइन पृष्ठभूमि में नहीं चल रहा: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle को इसे शुरू करने के लिए trindle-plugin नहीं मिला।

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
plugins-not-an-id = मान्य आईडी नहीं

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] के Shortcuts में { $n } शॉर्टकट है। ज़्यादा से ज़्यादा { $max } की अनुमति है।
       *[other] [Plugin] के Shortcuts में { $n } शॉर्टकट हैं। ज़्यादा से ज़्यादा { $max } की अनुमति है।
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = शॉर्टकट की आईडी { $id } में केवल अक्षर, अंक, डैश और अंडरस्कोर हो सकते हैं।

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] के Shortcuts में { $id } दो बार नाम आया है।

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] के Shortcuts में { $id } का नाम है, जिसका कोई [{ $group }] भाग नहीं है।

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] की Keys { $keys } पहले से Trindle के शॉर्टकट में से एक है। दूसरी कुंजियाँ चुनें।

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] की Keys { $keys } इस प्लगइन के किसी दूसरे शॉर्टकट में पहले से इस्तेमाल हो रही हैं।

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] को Keys चाहिए, जैसे Ctrl+Alt+S। { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = यह कुंजी संयोजन मान्य शॉर्टकट नहीं है। { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } पहले से Trindle के शॉर्टकट में से एक है।

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } पहले से आपका अपना एक शॉर्टकट है।

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } पहले से { $holder } द्वारा इस्तेमाल हो रहा है।

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = यह किसी इंस्टॉल किए प्लगइन का शॉर्टकट नहीं है।

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle इसे इस्तेमाल नहीं कर सका ({ $why })।

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle इसे सहेज नहीं सका, क्योंकि { $why }। कुछ नहीं बदला है।

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = यह सहेजा नहीं जा सका: { $why }।

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = इसके पोर्ट बंद नहीं किए जा सके: { $why }।

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle इसके पोर्ट नहीं खोल सका, इसलिए सुनना फिर बंद कर दिया गया: { $why }।

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = इसके पोर्ट खोले नहीं गए ({ $why }), और सुनना फिर बंद भी नहीं किया जा सका ({ $kept })। कुछ भी खुला नहीं है। इसे Settings, अनुमतियाँ में बंद करें।


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = सुलभता की अनुमति का इस्तेमाल केवल सूची में मौजूद Trindle का स्क्रीन रीडर कर सकता है।

plugins-permission-twice = अनुमति { $name } का नाम दो बार लिखा गया है।

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = { $home } या network में से एक माँगें, दोनों नहीं: network स्थानीय नेटवर्क के साथ इंटरनेट तक भी पहुँचता है।

plugins-needs-run = [{ $group }] को Run चाहिए, यानी चलाने का आदेश।

plugins-swap-not-back = { $why } पुराना संस्करण भी वापस नहीं रखा जा सका: { $back } दोनों प्लगइन के फ़ोल्डर में रखे हैं।
