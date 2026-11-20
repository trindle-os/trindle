# machine translated (Swahili), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Ripoti tatizo

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Kuna kitu kimeharibika. Tuma ripoti?

# Under the title: what a report holds, and what it never holds.
report-summary = Hii hutuma maneno yako, maelezo ya kompyuta, makosa ya kuwasha huku na kumbukumbu ya Trindle. Majina na anwani (barua pepe, mtandao, Wi-Fi) huondolewa kwenye kile Trindle inachokusanya. Majina ya faili na anwani za wavuti bado zinaweza kuonekana kwenye mistari ya kumbukumbu. Haitumi faili zako wala nywila zako.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = Nini kilitokea?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Programu gani?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Sina uhakika

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Programu nyingine

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = Eneo-kazi

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Tuma

# Button at the bottom of the window when the person opened it themselves.
report-cancel = Ghairi

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = Usitume

# Button that opens the exact text of the report.
report-show-details = Onyesha maelezo

# Button that closes the exact text of the report.
report-hide-details = Ficha maelezo

# Button under the shown report text that copies it.
report-copy = Nakili

# Button under the shown report text that saves it as a file in Downloads.
report-save = Hifadhi

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Ripoti bado haziwezi kutumwa.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Inakagua.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Ripoti nyingi mno kwa wakati mmoja.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Inasubiri muunganisho.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Inasubiri muunganisho. { $why }

# Under the typing box after the report was sent.
report-sent = Imetumwa. Asante.

# Under the typing box while the report is being posted.
report-sending = Inatuma.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Imehifadhiwa kama Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Andika jina lako la mtumiaji la Buildmorphic, si anwani ya barua pepe. Usitumie @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Jina la mtumiaji lazima liwe na urefu wa herufi 2 hadi 31, zikiwa herufi ndogo, nambari au vistari.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Jina la GitHub linaweza kuwa na herufi, namba na vistari tu.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle haikuweza kuhifadhi ripoti kwa baadaye: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Haikuweza kuhifadhiwa: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = hakuna ruhusa

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = diski imejaa

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = folda haipo

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = hitilafu ya diski

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl haipo.

# Why a report could not be posted: the post could not be started.
report-not-posted = Haikuweza kutumwa.

# Why a report could not be posted: the computer is offline.
report-no-connection = Hakuna muunganisho.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Seva ilijibu { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Andika maneno machache zaidi kueleza kilichotokea.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Inasoma kumbukumbu.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Hakuna kumbukumbu ya kutuma nayo, kwa hiyo haiwezi kutumwa. Ihifadhi badala yake.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Picha ni hiari. Ni mmiliki wa mradi tu anayeiona.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Chagua picha ya PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Piga picha ya skrini

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Ondoa picha

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Picha yako, { $width } kwa { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Chagua picha ya PNG

report-picture-filter = Picha za PNG

# Under the picture while it is being read.
report-picture-reading = Inasomwa.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Imeshindwa kupiga picha ya skrini.

report-picture-no-chooser = Trindle haikuweza kufungua kichagua faili.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Si picha ya PNG.

report-picture-too-big = Ni zaidi ya MB { $megabytes }.

report-picture-too-large = Picha ni kubwa mno. Inaweza kuwa hadi pikseli { $side } kwa upana na pikseli { $side } kwa urefu.

report-picture-damaged = Picha imeharibika.

report-picture-unreadable = Imeshindwa kuisoma.

report-sent-as-bug = Imetumwa. Nambari ya ripoti yako ya hitilafu ni { $number }. Asante.

# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle iko katika hatua ya beta. Ripoti hazitambulishi mtu: majina, anwani za barua pepe na folda za nyumbani huondolewa. Jina la mtumiaji ni la hiari.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Jina la mtumiaji la Buildmorphic (la hiari)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle haikuweza kutuma ripoti. { $why } Bonyeza Tuma ujaribu tena.
