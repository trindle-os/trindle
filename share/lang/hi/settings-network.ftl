
# machine translated (Hindi), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } कंप्यूटर के ही किसी स्विच या कुंजी से बंद है। उसे चालू करें, फिर दोबारा कोशिश करें।

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } को सिस्टम ने रोक रखा है, और वह रोक हटाई नहीं जा सकी।

# SAFETY: needs a human review before it ships
# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = वह पासवर्ड स्वीकार नहीं हुआ। फिर से आज़माएँ।

# SAFETY: needs a human review before it ships
# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = इस नेटवर्क के लिए सहेजा गया पासवर्ड स्वीकार नहीं हुआ। कूड़ेदान से नेटवर्क को भुलाएँ, फिर उससे दोबारा जुड़ें।

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = वह नेटवर्क नहीं मिला। उसका नाम जाँचें, और देखें कि वह पहुँच में है।

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = नेटवर्क ने इस कंप्यूटर को अंदर आने दिया पर कोई पता नहीं दिया। फिर कोशिश करें, या राउटर को दोबारा चालू करें।

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = नेटवर्क ने इस कंप्यूटर को जुड़ने नहीं दिया। उसके पास जाएँ, फिर दोबारा कोशिश करें।

# SAFETY: needs a human review before it ships
# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = वह पासवर्ड Wi-Fi नेटवर्क का पासवर्ड नहीं हो सकता। उसे जाँचें, फिर दोबारा कोशिश करें।

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = नेटवर्क से जुड़ा नहीं जा सका।

# SAFETY: needs a human review before it ships
# Small line of a shared folder: who may open it with a password.
settings-network-access-open = आपके पासवर्ड से खोलें

# SAFETY: needs a human review before it ships
# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = आपके पासवर्ड से खोलें और बदलें

# SAFETY: needs a human review before it ships
# Small line of a shared folder: anyone at home may open it.
settings-network-access-guests-open = घर में कोई भी खोल सकता है

# SAFETY: needs a human review before it ships
# Small line of a shared folder: anyone at home may change it.
settings-network-access-guests-change = घर में कोई भी बदल सकता है

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# SAFETY: needs a human review before it ships
# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = { $name } को साझा करना बंद करें

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = प्रबंधित नहीं

# State of a network device that cannot be used.
settings-network-state-unavailable = उपलब्ध नहीं

# State of a network device that is not connected.
settings-network-state-disconnected = जुड़ा नहीं है

# State of a network device that is getting ready to connect.
settings-network-state-preparing = तैयारी हो रही है

# State of a network device that is being set up.
settings-network-state-configuring = सेट किया जा रहा है

# State of a network device that needs a password.
settings-network-state-needs-password = पासवर्ड चाहिए

# State of a network device that is getting an address.
settings-network-state-address = पता लिया जा रहा है

# State of a network device whose connection is being checked.
settings-network-state-checking = जाँच हो रही है

# State of a network device that is waiting.
settings-network-state-waiting = प्रतीक्षा में

# State of a network device that is connected.
settings-network-state-connected = जुड़ा है

# State of a network device that is disconnecting.
settings-network-state-disconnecting = जुड़ाव हटाया जा रहा है

# State of a network device that failed.
settings-network-state-failed = विफल

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = केबल से जुड़े हैं, और Wi-Fi पर { $name } से भी।

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] { $name } से जुड़े हैं, सिग्नल मज़बूत।
        [good] { $name } से जुड़े हैं, सिग्नल अच्छा।
        [fair] { $name } से जुड़े हैं, सिग्नल ठीक-ठाक।
       *[other] { $name } से जुड़े हैं, सिग्नल कमज़ोर।
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = केबल से जुड़े हैं।

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi बंद है, इसलिए यह कंप्यूटर कहीं जुड़ा नहीं है।

# Headline of the page when not connected.
settings-network-top-none = जुड़े नहीं हैं। नीचे से कोई नेटवर्क चुनें।

# Headline of the page while the networks are looked for.
settings-network-top-looking = नेटवर्क ढूँढे जा रहे हैं…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = नेटवर्क देखे नहीं जा सकते: NetworkManager चल नहीं रहा।

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = सिस्टम बस

# Name of the page in the list of settings pages and its heading.
settings-network-title = नेटवर्क

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wireless internet ethernet wired password connect firewall sharing share printer folder वाई-फ़ाई वायरलेस इंटरनेट ईथरनेट पासवर्ड जुड़ें फ़ायरवॉल साझा प्रिंटर फ़ोल्डर नेटवर्क

# Help text at the top of the page.
settings-network-help = Wi-Fi चालू या बंद करें, किसी नेटवर्क से जुड़ें, जो नेटवर्क अब नहीं चाहिए उसे भुलाएँ, या दूसरे कंप्यूटरों को जुड़ने दें।

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] { $name } से मज़बूत सिग्नल।
        [good] { $name } से अच्छा सिग्नल।
        [fair] { $name } से ठीक-ठाक सिग्नल।
       *[other] { $name } से कमज़ोर सिग्नल।
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = इसकी चिप के लिए नया Linux चाहिए।

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = इस कंप्यूटर में काम करने वाला Wi-Fi नहीं है।

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = बंद है। नेटवर्क से जुड़ने के लिए इसे चालू करें।

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = चालू है, और किसी नेटवर्क से जुड़ा नहीं है।

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi इस्तेमाल करें

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi और केबल

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi इस कंप्यूटर को बिना केबल के नेटवर्क से जोड़ता है। बंद करने पर वह नेटवर्क ढूँढना बंद कर देता है और थोड़ी बैटरी बचती है। जो नेटवर्क अपना नाम प्रसारित नहीं करता, वह पहुँच वाले नेटवर्कों की सूची में कभी नहीं दिखता: उससे यहाँ जुड़ने के लिए उसका नाम बिल्कुल सही लिखें, बड़े अक्षरों समेत। अगर इस कंप्यूटर की Wi-Fi चिप ऐसी है जिसे इसके Linux का कोई ड्राइवर अभी नहीं सँभालता, तो Trindle जिस Linux पर चलता है उससे नया Linux चाहिए। नेटवर्क केबल की अपनी अलग पंक्ति है, जिसमें जोड़ने या हटाने का बटन है।

# SAFETY: needs a human review before it ships
# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = दूसरे कंप्यूटरों को जुड़ने दें

# SAFETY: needs a human review before it ships
# Small line under that switch.
settings-network-share-switch-sub = घर में फ़ोल्डर और प्रिंटर साझा करने के लिए।

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = साझा फ़ोल्डर ढूँढे जा रहे हैं…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = साझा फ़ोल्डर पढ़े नहीं जा सके

# SAFETY: needs a human review before it ships
# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = अभी पहुँच में नहीं

# Small line under it.
settings-network-not-reachable-sub = यह बंद होने पर दूसरे इन्हें नहीं खोल सकते।

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = कोई फ़ोल्डर साझा नहीं है

# Small line under it.
settings-network-no-shared-sub = साझा करने के लिए Folders में किसी पर राइट-क्लिक करें।

# Heading of the Sharing card.
settings-network-sharing = साझा करना

# SAFETY: needs a human review before it ships
# Help text (i button) of the Sharing card.
settings-network-sharing-about = जब तक दूसरे कंप्यूटरों को जुड़ने दें बंद है, नेटवर्क पर कोई भी चीज़ इस कंप्यूटर तक नहीं पहुँच सकती जब तक इसने पहले न पूछा हो, इसलिए उसी Wi-Fi पर कोई किस्मत नहीं आज़मा सकता। घर के दूसरे कंप्यूटरों के साथ फ़ोल्डर या प्रिंटर साझा करने के लिए इसे चालू करें। फ़ोल्डर साझा करने के लिए Folders में उस पर राइट-क्लिक करें और नेटवर्क पर साझा करें चुनें। हर एक में तय होता है कि कौन क्या कर सकता है। आपके पासवर्ड से: दूसरे आपके साझा करने के पासवर्ड से फ़ाइलें खोल सकते हैं, या खोल और बदल सकते हैं। घर में कोई भी: नेटवर्क पर कोई भी बिना पासवर्ड के फ़ाइलें खोल सकता है, या खोल और बदल सकता है। स्विच बंद होने पर घर के दूसरे कंप्यूटर इनमें से कोई भी नहीं खोल सकते।

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = बंद किया जा रहा है…

# SAFETY: needs a human review before it ships
# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = पासवर्ड की प्रतीक्षा है...

# SAFETY: needs a human review before it ships
# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = साझा करने की अनुमति है

# SAFETY: needs a human review before it ships
# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = बाहर रखा गया

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi चालू नहीं हो सका।

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi बंद नहीं हो सका।

# Name of a wired network device that has no name of its own.
settings-network-ethernet = ईथरनेट

# SAFETY: needs a human review before it ships
# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = जुड़ाव हटाएँ

# Tip of the button that connects a network or a cable.
settings-network-connect = जुड़ें

# Shown under a network or cable while it is being connected.
settings-network-connecting = जुड़ रहा है...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = पहुँच में कोई नेटवर्क नहीं

# Small line under it.
settings-network-none-in-range-sub = यह पेज खुलने पर हर बार फिर ढूँढा जाता है।

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = पहुँच वाले नेटवर्क

# SAFETY: needs a human review before it ships
# Help text (i button) of the Networks in range card.
settings-network-in-range-about = हर वह Wi-Fi नेटवर्क जो यह कंप्यूटर सुन सकता है। पेज खुलते समय पहले इस्तेमाल वाला दिखता है, फिर यहाँ सहेजे हुए, फिर बाकी सिग्नल की ताकत के क्रम में; पेज खुला रहने तक कोई पंक्ति अपनी जगह रहती है। किसी से जुड़ने के लिए तीर दबाएँ; सुरक्षित नेटवर्क पहली बार पासवर्ड माँगता है। कूड़ेदान सहेजे हुए नेटवर्क और उसके पासवर्ड को भुला देता है। खुला नेटवर्क एन्क्रिप्टेड नहीं होता, इसलिए पास के लोग उस पर भेजा गया देख सकते हैं।

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = पहुँच वाले नेटवर्क देखने के लिए Wi-Fi चालू करें।

# Part of a network's small line: it is saved on this computer.
settings-network-saved = सहेजा हुआ

# Part of a network's small line: it needs a password.
settings-network-secured = सुरक्षित

# SAFETY: needs a human review before it ships
# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = खुला, एन्क्रिप्टेड नहीं

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# SAFETY: needs a human review before it ships
# Tip of the bin button that forgets a saved network.
settings-network-forget = इस नेटवर्क को भुलाएँ

# SAFETY: needs a human review before it ships
# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = नेटवर्क भुलाया नहीं जा सका।

# Placeholder of the box where a network's password is typed.
settings-network-password = पासवर्ड

# Tip of the button that joins the network.
settings-network-join = जुड़ें

# SAFETY: needs a human review before it ships
# Tip of the button that closes the box without joining.
settings-network-cancel = रद्द करें

# Shown under a network while it is being joined.
settings-network-joining = जुड़ रहा है...

# SAFETY: needs a human review before it ships
# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Wi-Fi पासवर्ड कम से कम 8 अक्षरों के होते हैं।

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = छिपे हुए नेटवर्क से जुड़ें

# Small line under it.
settings-network-hidden-sub = उसका नाम बिल्कुल सही लिखें, बड़े अक्षर भी।

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = नेटवर्क का नाम

# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = पासवर्ड, अगर हो

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = नेटवर्क का नाम चाहिए।

# Shown under a network after it was joined.
settings-network-joined = जुड़ गया

# SAFETY: needs a human review before it ships
# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } फिर भी यह सहेजा गया: दोबारा जुड़ने से पहले इसे कूड़ेदान से भुलाएँ।

# SAFETY: needs a human review before it ships
# Said in red when a network would not disconnect.
settings-network-disconnect-failed = यह जुड़ाव हटाया नहीं जा सका।

