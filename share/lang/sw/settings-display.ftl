# machine translated (Swahili), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Skrini

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = skrini kifuatiliaji azimio ukubwa kipimo kuza kubwa ndogo zungusha mzunguko mwanga wa usiku joto mwanga wa buluu jioni macho

# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = Mabadiliko hutumika mara moja. Skrini ikiharibika, usifanye kitu. Itarudi yenyewe baada ya sekunde 15 isipokuwa ubonyeze Weka. Mwanga wa usiku hufanya skrini iwe na joto zaidi jioni.

# Question along the bottom of the page while a change of displays waits to be kept.
# SAFETY: needs a human review before it ships
settings-display-foot-keep = Weka mpangilio huu?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Inakagua skrini…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Skrini haijapatikana.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Hakuna skrini inayotumika

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Skrini moja inatumika

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] Skrini { $n } inatumika
       *[other] Skrini { $n } zinatumika
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = mwanga wa usiku umezimwa

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = mwanga wa usiku hupasha skrini joto kuanzia { $from } hadi { $to }

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = mwanga wa usiku hupasha skrini joto kuanzia jua linapozama hadi linapochomoza

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Kawaida

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Imezungushwa kulia

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Imezungushwa kushoto

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Juu chini

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = Kulia kwa

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = Kushoto kwa

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Juu ya

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Chini ya

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } kwa { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (inapendekezwa)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (kawaida)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Azimio la { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Ukubwa kwenye { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Mzunguko wa { $display }

# Heading of the resolution line.
settings-display-resolution = Azimio

# Heading of the size line.
settings-display-size = Ukubwa

# Heading of the rotation line.
settings-display-rotation = Mzunguko

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Tumia skrini hii

# Small words under that line for the only display in use.
settings-display-use-only = Huwezi kuzima skrini pekee inayotumika.

# Words behind the i of a display's card.
settings-display-card-about = Mabadiliko hutumika mara moja. Skrini ikiharibika, usifanye kitu. Itarudi yenyewe baada ya sekunde 15 isipokuwa ubonyeze Weka. Azimio linalopendekezwa ni lile ambalo skrini imetengenezewa. Ukubwa wa kawaida ni ule ambao Trindle ilichagua kwa skrini hii.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Weka { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Iweke

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } kulingana na { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Mpangilio

# Words behind the i of that card.
settings-display-arrangement-about = Weka mahali skrini zinapokaa kando ya nyingine, ili kielekezi kiweze kuvuka ukingo wanaoshiriki.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Unahitaji skrini mbili ili kuweka moja kando ya nyingine.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Haikuweza kubadilishwa.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
# SAFETY: needs a human review before it ships
settings-display-aloud-changed =
    { $seconds ->
        [one] Skrini zimebadilika. Zitarudi baada ya sekunde { $seconds } isipokuwa ubonyeze Weka.
       *[other] Skrini zimebadilika. Zitarudi baada ya sekunde { $seconds } isipokuwa ubonyeze Weka.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Mabadiliko hayakuweza kutenguliwa. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = Mpangilio unatumika sasa, lakini Trindle haikuweza kuuhifadhi kwa kuingia kunakofuata.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Hakuna kilichobadilishwa

# Small words under that line.
settings-display-refused-change = Haikuweza kufanya mabadiliko hayo.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Mabadiliko hutumika mara moja

# Title of the night light card.
settings-display-night-title = Mwanga wa usiku

# Words behind the i of that card.
settings-display-night-about = Skrini inakuwa na joto zaidi jioni, jambo linalopunguza uchovu wa macho. Hufuata jua kuzama na kuchomoza, kulingana na eneo lako la saa, au saa unazoweka mwenyewe. Trindle isipoweza kujua uko wapi kupitia eneo lako la saa, hutumia saa ulizoweka.

# Heading of the night light line, and name of its switch.
settings-display-night = Mwanga wa usiku

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Jua kuzama na kuchomoza katika { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle inatumia saa zilizowekwa kwa sababu haijui jua linazama saa ngapi.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Jua kuzama hadi kuchomoza

# Choice in the When list: between times the person sets.
settings-display-when-times = Saa zilizowekwa

# Heading of the When line, and name of its list.
settings-display-when = Lini

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = Kuanzia

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = Hadi

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Joto

# Small word under the left end of the warmth slider.
settings-display-little = Kidogo

# Small word under the right end of the warmth slider.
settings-display-very = Joto sana

# Spoken description of the strip of colours night light runs through.
settings-display-strip = Kutoka joto kidogo hadi joto sana

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Hakuna skrini inayotumika

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] Skrini { $n }
       *[other] Skrini { $n }, kama zilivyopangwa
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Hakuna skrini

# Name of the large picture of the displays.
settings-display-your-displays = Skrini zako

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle haikupata skrini yoyote.

# Title of the card shown when no display was found.
settings-display-none-title = Hakuna skrini zilizopatikana

# Heading of the line in that card.
settings-display-none-heading = Hakuna skrini iliyopatikana

# Title of the card about the graphics card.
settings-display-graphics-title = Michoro

# Words behind the i of that card.
settings-display-graphics-about = Kompyuta hii ina kadi ya michoro ya NVIDIA inayotumia dereva huria. Inafanya kazi, lakini kadi ya kuanzia mwaka 2022 hivi itahisi polepole, kwa sababu dereva huria hawezi kuongeza kasi ya kadi. Unaweza kusakinisha dereva wa NVIDIA mwenyewe badala yake.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Kadi ya NVIDIA yenye dereva huria

# Small words under that line.
settings-display-nvidia-about = Kadi za kuanzia 2022 huenda zikawa polepole.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Cha kufanya kuhusu hili
