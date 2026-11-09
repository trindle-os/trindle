
# machine translated (Hindi), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = कीबोर्ड और माउस

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout language typing caps lock on-screen keyboard button ring mouse touchpad trackpad pointer speed tap click scroll wheel slower faster natural left handed plugin shortcut keys remap change swap ctrl escape super alt mac side buttons tapped held made by programs keyd लेआउट भाषा टाइपिंग कीबोर्ड माउस टचपैड पॉइंटर गति टैप क्लिक स्क्रॉल पहिया धीमा तेज़ बाएँ हाथ शॉर्टकट कुंजी बदलें अदला-बदली

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = यहाँ सब कुछ तुरंत बदल जाता है। दूसरा लेआउट Alt और Shift साथ दबाने से बदलता है।

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = { $layout } में टाइपिंग।

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt और Shift से { $layout } पर बदलता है।

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock कुछ नहीं करता।

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock { $key } की तरह काम करता है।

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = दूसरा Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = कीबोर्ड लेआउट की सूची इस कंप्यूटर पर नहीं है।

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = कीबोर्ड

# Heading of the line in that card.
settings-keyboard-no-list-heading = कीबोर्ड लेआउट की सूची उपलब्ध नहीं है

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = इसे इंस्टॉल करें: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] एक कुंजी बदली गई है।
       *[other] { $n } कुंजियाँ और बटन बदले गए हैं।
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = लेआउट

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = लेआउट

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = रूप

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = मानक

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = कोई नहीं

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = इस पर बदलें

# Small words under the second layout line.
settings-keyboard-switch-to-about = Alt और Shift साथ दबाने से दोनों के बीच बदलता है।

# Words behind the i of the layout card.
settings-keyboard-layout-about = हर कुंजी क्या टाइप करती है: लेआउट, और उसका कोई रूप जैसे बिना डेड कीज़ वाला। दूसरा लेआउट Alt और Shift साथ दबाने से बदलता है, और उसी तरह वापस आता है। सब कुछ तुरंत बदलता है।

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = अभी इस्तेमाल में

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = दूसरा Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = कुछ नहीं

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Caps Lock कुंजी इस तरह काम करे

# Small words under that line.
settings-keyboard-caps-acts-as-about = दूसरा Ctrl, Escape वगैरह।

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = रिंग में कीबोर्ड बटन

# Small words under that line.
settings-keyboard-ring-button-about = स्क्रीन पर एक कीबोर्ड लाता है।

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = अन्य कुंजियाँ

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock की जगह कोई दूसरी कुंजी का काम कर सकता है, जैसे दूसरा Ctrl या Escape। कीबोर्ड बटन रिंग के बीच में एक कीबोर्ड है जो स्क्रीन पर कीबोर्ड लाता है, माउस या टच स्क्रीन से टाइप करने के लिए। यह रिंग के अगली बार खुलने पर दिखता है।

# SAFETY: needs a human review before it ships
# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = इसे केवल आपका अपना दबाना बदल सकता है।

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = बदलने के लिए कुंजी या माउस बटन दबाएँ।

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = कुछ दबाया नहीं गया, इसलिए कुछ जोड़ा नहीं गया।

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = लिखें कि वह क्या बनेगी, जैसे Escape, Ctrl+C या Nothing।

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = उस दबाव को दूसरे से अलग नहीं पहचाना जा सका। इसके बजाय सूची से कुंजी चुनें।

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = हर कीबोर्ड और माउस

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = नीचे भी बदला गया है; वही बदलाव इस्तेमाल होता है।

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = { $key } किस कीबोर्ड या माउस पर बदली गई है

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key } क्या बनेगी

# SAFETY: needs a human review before it ships
# Name of the bin button that takes a change away.
settings-keyboard-take-away = यह बदलाव हटाएँ

# Small words under a changed key's line when it changes on every keyboard and mouse.
settings-keyboard-every-device-about = हर कीबोर्ड और माउस पर।

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = पंक्ति { $n } इस्तेमाल में नहीं है: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = बदली गई कुंजियाँ पढ़ी नहीं जा सकीं

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = जब तक वे पढ़ी नहीं जातीं, हर कुंजी वैसी ही है जैसी छपी है: { $why }।

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = बदलने के लिए कुंजी या बटन चुनें

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = बदलने के लिए कुंजी या बटन दबाएँ

# Small words under the change a key line.
settings-keyboard-keys-add-about = उसे दबाएँ, या सूची से चुनें।

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = कुंजी या बटन बदलें

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } नीचे पहले से बदली हुई है।

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = कुंजियाँ और बटन

# SAFETY: needs a human review before it ships
# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = कोई भी कुंजी या माउस बटन कुछ और कर सकता है: दूसरी कुंजी या बटन, साथ दबाई गई कुंजियाँ जैसे Ctrl+C, एक के बाद एक कुंजियाँ जैसे H, I, कुछ भी नहीं, या एक चीज़ टैप करने पर और दूसरी दबाए रखने पर, जैसे टैप करने पर Escape, दबाए रखने पर Ctrl। यह लॉगिन स्क्रीन से ही हर जगह काम करता है। Ctrl, Alt और कोई F कुंजी हमेशा टेक्स्ट कंसोल तक पहुँचती हैं, और वहाँ trindle-settings --keys default लिखने से हर कुंजी वापस पहले जैसी हो जाती है।

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl और Caps Lock की अदला-बदली

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock टैप करने पर Escape, दबाए रखने पर Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape और Caps Lock की अदला-बदली

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Alt और Super की अदला-बदली, जैसे Mac कीबोर्ड पर

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = माउस के बगल के बटन कॉपी और पेस्ट करें

# SAFETY: needs a human review before it ships
# Small words under a keyboard or mouse a program made, when it counts as the person.
settings-keyboard-made-yes = आपके बराबर गिना जाता है, जैसे लगाया गया कीबोर्ड।

# SAFETY: needs a human review before it ships
# Small words under a keyboard or mouse a program made, when it does not count as the person.
settings-keyboard-made-no = टाइप करता और चलाता है। सुरक्षित बटन नहीं दबा सकता।

# SAFETY: needs a human review before it ships
# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = { $name } आपके बराबर गिना जाता है

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = अब तक कोई नहीं

# SAFETY: needs a human review before it ships
# Small words under that line.
settings-keyboard-made-none-about = कोई प्रोग्राम जो बनाए, वह यहाँ दिखता है, ताकि उसे आपके बराबर गिना जा सके।

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = प्रोग्रामों के बनाए कीबोर्ड और माउस

# SAFETY: needs a human review before it ships
# Words behind the i of that card.
settings-keyboard-made-about = कुंजी बदलने वाले टूल और गेम लॉन्चर अपना कीबोर्ड या माउस बना सकते हैं। उनकी कुंजियाँ और क्लिक अब भी टाइप और खेल करते हैं, पर वे सुरक्षित बटन नहीं दबा सकते, जैसे शट डाउन या अनुमति दें, और न ही आपके लिए बने किसी सवाल में टाइप कर सकते हैं। किसी को अपने बराबर गिनने के लिए उसे यहाँ चालू करें।

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = कंट्रोलर { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $title } पर { $name }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title } क्या करता है

# Small words under a controller's changed button; the words Button and what it becomes stand for the two sides of the line typed.
settings-keyboard-pad-type = इसे बटन = वह क्या बनता है, इस तरह लिखें।

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A और B की अदला-बदली

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X और Y की अदला-बदली

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = दाएँ स्टिक का ऊपर-नीचे पलटें

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = प्लगइन शॉर्टकट

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }। इस्तेमाल में नहीं: { $why }

# Small words under a plugin shortcut; { $plugin } is the plugin. The example is keys a person types.
settings-keyboard-plugin-about = { $plugin }। कुंजियाँ लिखें, जैसे Ctrl+Alt+S, या कोई नहीं के लिए Nothing।

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = { $label } के लिए कुंजियाँ

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = सहेजा गया

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = टचपैड के लिए भी।

# Title of the mouse card.
settings-keyboard-mouse-title = माउस

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = पॉइंटर की गति यह है कि माउस के एक हिलाव पर पॉइंटर कितना चलता है, और टचपैड पर भी। स्क्रॉल की गति यह है कि पहिये के हर घुमाव पर पेज कितना चलता है: धीमा, सामान्य या तेज़, या स्लाइडर पर कहीं भी। प्राकृतिक स्क्रॉल में पेज उसी तरह चलता है जैसे पहिया घूमता है, जैसे फ़ोन में। बाएँ हाथ वाला विकल्प बाएँ और दाएँ बटन की अदला-बदली करता है।

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = पॉइंटर की गति

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = स्क्रॉल की गति

# Small words under the wheel's scroll speed.
settings-keyboard-scroll-wheel-about = पहिये के हर घुमाव पर पेज कितना चलता है।

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = प्राकृतिक स्क्रॉल

# Small words under natural scrolling for the mouse wheel.
settings-keyboard-natural-wheel-about = पेज पहिये के साथ चलता है, जैसे फ़ोन में।

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = बाएँ हाथ वाला

# Small words under that switch.
settings-keyboard-left-handed-about = बाएँ और दाएँ बटन की अदला-बदली करता है।

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = क्लिक सहायक

# SAFETY: needs a human review before it ships
# Small words under the clicking helper line.
settings-keyboard-clicker-about = ठहराव या स्विच से क्लिक करने वाले उपकरण के क्लिक आपके अपने गिने जाते हैं।

# Title of the touchpad card.
settings-keyboard-touchpad-title = टचपैड

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = स्क्रॉल की गति यह है कि दो उँगलियों से पेज कितना चलता है। टैप से क्लिक हल्की थपकी को क्लिक गिनता है। प्राकृतिक स्क्रॉल में पेज उसी तरह चलता है जैसे आपकी उँगलियाँ चलती हैं, जैसे फ़ोन में। टाइप करते समय बंद रखने से हथेली छूने पर पॉइंटर टाइप करते समय नहीं हिलता।

# Small words under the touchpad's scroll speed.
settings-keyboard-scroll-touchpad-about = दो उँगलियों से पेज कितना चलता है।

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = टैप से क्लिक

# Small words under that switch.
settings-keyboard-tap-about = हल्की थपकी क्लिक गिनी जाती है।

# Small words under natural scrolling for the touchpad.
settings-keyboard-natural-touchpad-about = पेज आपकी उँगलियों के साथ चलता है, जैसे फ़ोन में।

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = टाइप करते समय बंद

# Small words under that switch.
settings-keyboard-quiet-about = हथेली छूने पर पॉइंटर स्थिर रहता है।

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = धीमा

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = सामान्य

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = तेज़

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB पोर्ट

# Words behind the i of that card.
settings-keyboard-usb-about = जब कोई USB पोर्ट शुरू नहीं हो पाता, तो Trindle उसे दोबारा आज़माने के लिए एक बार बंद करके चालू करता है, और किसी पोर्ट के लिए एक बार से ज़्यादा कभी नहीं।

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error }; और उसका निशान फिर से हटाया नहीं जा सका: { $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = स्क्रीनशॉट और शॉर्टकट सूची

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = स्क्रीनशॉट लें

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = शॉर्टकट की सूची दिखाएँ

# Small words under each of the two shortcuts.
settings-keyboard-shortcuts-about = कुंजियाँ लिखें, या कोई न चाहिए तो खाली छोड़ें।

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = अभी स्क्रीनशॉट लें

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = अभी सूची दिखाएँ

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = डेस्कटॉप रीस्टार्ट करें

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = रिंग और सहायक हिस्से रीस्टार्ट करें

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = सब कुछ रीस्टार्ट करें

# Small words under each of the two restart shortcuts. The example is keys a person types.
settings-keyboard-restart-about = कुंजियाँ लिखें, जैसे Ctrl+Alt+Escape, या कोई न चाहिए तो खाली छोड़ें।

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = { $label } के लिए कुंजियाँ

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = अभी रीस्टार्ट करें। खुली विंडो जैसी हैं वैसी रहती हैं।

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = सब कुछ रीस्टार्ट करें। पहले पूछता है।

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = दोबारा शुरू हुआ

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } पहले से किसी दूसरे शॉर्टकट में उपयोग हो रही है।

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = सहेजा नहीं गया: { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = समस्या बताएँ

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = अभी समस्या बताएँ

