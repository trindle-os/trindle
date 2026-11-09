
# machine translated (Hindi), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = अभी कोई प्रिंटर नहीं। आपके नेटवर्क पर मिले प्रिंटर नीचे दिखाए गए हैं।

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } रुका हुआ है, इसलिए अभी उस पर कुछ प्रिंट नहीं होगा।

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } प्रिंट कर रहा है।

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = { $name } पर प्रिंट करने के लिए तैयार।

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } { $state } है।

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } सेट है; पहले प्रिंट करने के लिए कुछ चुना नहीं गया है।

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } प्रिंटर सेट हैं; पहले प्रिंट करने के लिए कोई चुना नहीं गया है।

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = यह ऐसा काम नहीं है जिसे CUPS जानता हो।

# Shown beside a print job after it was cancelled.
# SAFETY: needs a human review before it ships
settings-printers-cancelled = रद्द किया गया

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat विफल रहा

# Words in the ring while the network is searched for printers.
settings-printers-searching = खोज रहा है

# Words in the ring when the search is finished.
settings-printers-search-done = हो गया

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = अब डिफ़ॉल्ट

# Shown beside a printer after a test page was sent.
settings-printers-sent = भेजा गया

# Shown beside a printer after it was removed.
settings-printers-removed = हटाया गया

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = पहले प्रिंटर का पता लिखें।

# Shown under the By its address line while the printer is added.
settings-printers-adding = जोड़ा जा रहा है...

# Shown beside a printer or the address line after it was added.
settings-printers-added = जोड़ा गया

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = इसका ड्राइवर खोजा जा रहा है...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = { $printer } पर काम { $number }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = यह काम रद्द करें

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = प्रिंट के काम

# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = प्रिंट होने की प्रतीक्षा में हर काम, किसी भी प्रिंटर पर, जैसा अभी है। यह पन्ना खुला रहने तक सूची हर कुछ सेकंड में फिर पढ़ी जाती है। क्रॉस किसी काम को रद्द करता है; जो कागज़ पहले ही छप चुका है वह छपा रहता है।

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS जवाब नहीं दे रहा

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = कतार पढ़ी नहीं जा सकती।

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = खोज रहा है…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = कतार।

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = कुछ प्रतीक्षा में नहीं

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = प्रिंट होते समय काम यहाँ दिखते हैं।

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = रुका हुआ

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = खाली

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = प्रिंट हो रहा है

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = डिफ़ॉल्ट, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = पहले इसी पर प्रिंट करें

# Tip of the button that prints a test page.
settings-printers-test-page = टेस्ट पन्ना प्रिंट करें

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = इस प्रिंटर को हटाएँ

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = खोजने के लिए cups-filters चाहिए

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = इसे इंस्टॉल करें: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = यह प्रिंटर जोड़ें

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = कुछ नया नहीं मिला

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = क्या यह चालू है, और उसी नेटवर्क पर है?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = पहले घोषणा करने वाले, फिर एक-एक करके हर मशीन।

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = जोड़ें

# Heading of the line where a printer's address is typed.
settings-printers-by-address = उसके पते से

# Small line under "By its address".
settings-printers-by-address-sub = जैसे 192.168.1.50, उसकी अपनी स्क्रीन पर।

# Heading of the card for adding printers.
settings-printers-add-card = प्रिंटर जोड़ें

# Help text (i button) of the Add a printer card.
settings-printers-add-about = जो प्रिंटर अपनी घोषणा करते हैं उन्हें पहले खोजा जाता है, फिर नेटवर्क की हर मशीन से एक-एक करके पूछा जाता है। प्रिंटर चालू होना चाहिए और उसी नेटवर्क पर। कुछ प्रिंटर अपनी घोषणा नहीं करते, और कुछ नेटवर्क घोषणा आगे नहीं पहुँचाते: तब उसे उसके पते से जोड़ें, जैसे 192.168.1.50। यह आपको प्रिंटर की अपनी स्क्रीन पर, नेटवर्क सेटिंग्स में मिलेगा।

# Name of the page in the list of settings pages.
settings-printers-title = प्रिंटर

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = प्रिंटर प्रिंट स्कैनर printer print scanner

# Help text at the top of the page.
settings-printers-help = आपके नेटवर्क के प्रिंटर अपने आप मिल जाते हैं। इस्तेमाल करने के लिए उसके पास जोड़ें दबाएँ।

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = इस कंप्यूटर पर प्रिंटिंग इंस्टॉल नहीं है।

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = खोज रहा है…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = प्रिंटिंग सेवा जवाब नहीं दे रही, इसलिए अभी कुछ प्रिंट नहीं हो सकता।

# Heading of the card shown when printing is not installed.
settings-printers-printing = प्रिंटिंग

# Help text (i button) of that card.
settings-printers-printing-about = प्रिंटिंग के लिए CUPS चाहिए, जो इस कंप्यूटर पर नहीं है। इंस्टॉल होते ही आपके नेटवर्क के प्रिंटर अपने आप मिल जाएँगे।

# Line in that card.
settings-printers-unavailable = प्रिंटिंग उपलब्ध नहीं है

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = इसे इंस्टॉल करें: sudo apt install cups

# Line in that card.
settings-printers-then-found = फिर प्रिंटर मिल जाते हैं

# Small line under it.
settings-printers-then-found-sub = आपके नेटवर्क के, अपने आप।

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = प्रिंटिंग सेवा।

# Line in the Printers card when there are none.
settings-printers-none = अभी कोई प्रिंटर नहीं

# Small line under "No printers yet".
settings-printers-none-sub = इसके पास एक जोड़ें।

# Heading of the card that lists the printers.
settings-printers-card = प्रिंटर

# Help text (i button) of the Printers card.
# SAFETY: needs a human review before it ships
settings-printers-card-about = इस कंप्यूटर पर सेट हर प्रिंटर। तारा किसी को पहले इस्तेमाल होने वाला प्रिंटर बनाता है; पन्ने का आइकन उस पर टेस्ट पन्ना छापता है; कूड़ेदान उसे हटाता है, दो बार पूछकर। बदलाव करने के लिए आपका खाता lpadmin समूह में होना चाहिए।

# Red words under a page range box that cannot be used.
printers-bad-range = पन्ने जैसे 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = प्रतियाँ: 1 से 99

# Name of the choice to print on one side of the paper.
printers-sides-one = एक तरफ़

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = दोनों, लंबे किनारे से

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = दोनों, छोटे किनारे से

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = खड़ा

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = आड़ा

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = अनुमति नहीं है। आपका खाता lpadmin समूह में होना चाहिए; ऐसा होने के बाद एक बार लॉग आउट करके फिर लॉग इन करें।

# Red words when CUPS refuses and gives no reason.
# SAFETY: needs a human review before it ships
printers-refused = CUPS ने मना कर दिया।

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = उस प्रिंटर का नाम ऐसा नहीं है जो CUPS मानता हो।

# Red words when printing is not installed on the computer.
printers-not-installed = इस कंप्यूटर पर प्रिंटिंग इंस्टॉल नहीं है।

# Red words when the printer chosen has been removed since.
printers-gone = { $name } अब सेट नहीं है।

# Red words when the printer chosen is paused.
printers-paused = { $name } रुका हुआ है।

# Red words when sending the document to CUPS took too long.
printers-too-slow = दस्तावेज़ भेजने में बहुत समय लगा।

