
# machine translated (German), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Anzeige

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = Bildschirm Monitor Auflösung Größe Skalierung Zoom größer kleiner drehen Bildwiederholrate Nachtlicht warm Blaulicht Abend Augen screen monitor resolution scale night light

# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = Änderungen gelten sofort. Wenn der Bildschirm nicht mehr stimmt, tu nichts. Nach 15 Sekunden geht er von selbst zurück, es sei denn, du drückst auf „Beibehalten“. Das Nachtlicht macht den Bildschirm am Abend wärmer.

# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = Diese Anordnung beibehalten?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Die Anzeigen werden geprüft …

# Sentence at the top of the page when no display was found.
settings-display-none-found = Keine Anzeige gefunden.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Keine Anzeige in Benutzung

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Eine Anzeige in Benutzung

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] { $n } Anzeige in Benutzung
       *[other] { $n } Anzeigen in Benutzung
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = das Nachtlicht ist aus

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = das Nachtlicht macht den Bildschirm von { $from } bis { $to } wärmer

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = das Nachtlicht macht den Bildschirm vom Sonnenuntergang bis zum Sonnenaufgang wärmer

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normal

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Nach rechts gedreht

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Nach links gedreht

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Auf dem Kopf

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = Rechts von

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = Links von

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Über

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Unter

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } bei { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (empfohlen)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent } %  (Standard)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Auflösung von { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Größe auf { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Drehung von { $display }

# Heading of the resolution line.
settings-display-resolution = Auflösung

# Heading of the size line.
settings-display-size = Größe

# Heading of the rotation line.
settings-display-rotation = Drehung

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Diese Anzeige verwenden

# Small words under that line for the only display in use.
settings-display-use-only = Du kannst die einzige benutzte Anzeige nicht ausschalten.

# Words behind the i of a display's card.
settings-display-card-about = Änderungen gelten sofort. Wenn der Bildschirm nicht mehr stimmt, tu nichts. Nach 15 Sekunden geht er von selbst zurück, es sei denn, du drückst auf „Beibehalten“. Die empfohlene Auflösung ist die, für die der Bildschirm gemacht ist. Die Standardgröße hat Trindle für diesen Bildschirm gewählt.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = { $display } platzieren

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Platzieren

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } im Verhältnis zu { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Anordnung

# Words behind the i of that card.
settings-display-arrangement-about = Lege fest, wo die Anzeigen nebeneinander liegen, damit der Zeiger über die gemeinsame Kante wandern kann.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Du brauchst zwei Anzeigen, um eine neben die andere zu setzen.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Das ließ sich nicht ändern.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] Die Anzeigen haben sich geändert. Sie gehen in { $seconds } Sekunde zurück, es sei denn, du drückst auf „Beibehalten“.
       *[other] Die Anzeigen haben sich geändert. Sie gehen in { $seconds } Sekunden zurück, es sei denn, du drückst auf „Beibehalten“.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Die Änderung ließ sich nicht rückgängig machen. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = Die Anordnung wird jetzt benutzt, aber Trindle konnte sie nicht für die nächste Anmeldung speichern.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Nichts wurde geändert

# Small words under that line.
settings-display-refused-change = Diese Änderung ließ sich nicht durchführen.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Änderungen gelten sofort

# Title of the night light card.
settings-display-night-title = Nachtlicht

# Words behind the i of that card.
settings-display-night-about = Der Bildschirm wird am Abend wärmer, das ist angenehmer für die Augen. Er richtet sich nach Sonnenuntergang und Sonnenaufgang, aus deiner Zeitzone berechnet, oder nach Zeiten, die du selbst festlegst. Wenn Trindle an deiner Zeitzone nicht erkennen kann, wo du bist, werden die Zeiten benutzt, die du festgelegt hast.

# Heading of the night light line, and name of its switch.
settings-display-night = Nachtlicht

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Sonnenuntergang und Sonnenaufgang in { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle nutzt feste Zeiten, weil es den Sonnenuntergang nicht kennt.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Sonnenuntergang bis Sonnenaufgang

# Choice in the When list: between times the person sets.
settings-display-when-times = Feste Zeiten

# Heading of the When line, and name of its list.
settings-display-when = Wann

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = Von

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = Bis

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Wärme

# Small word under the left end of the warmth slider.
settings-display-little = Etwas

# Small word under the right end of the warmth slider.
settings-display-very = Sehr warm

# Spoken description of the strip of colours night light runs through.
settings-display-strip = Von etwas warm bis sehr warm

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Keine Anzeige in Benutzung

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } Anzeige
       *[other] { $n } Anzeigen, wie angeordnet
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Keine Anzeige

# Name of the large picture of the displays.
settings-display-your-displays = Deine Anzeigen

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle hat keine Anzeigen gefunden.

# Title of the card shown when no display was found.
settings-display-none-title = Keine Anzeigen gefunden

# Heading of the line in that card.
settings-display-none-heading = Keine Anzeigen gefunden

# Title of the card about the graphics card.
settings-display-graphics-title = Grafik

# Words behind the i of that card.
settings-display-graphics-about = Dieser Computer hat eine NVIDIA-Grafikkarte, die den offenen Treiber nutzt. Sie funktioniert, aber eine Karte von etwa 2022 an fühlt sich langsam an, weil der offene Treiber die Geschwindigkeit der Karte nicht anheben kann. Du kannst stattdessen den eigenen Treiber von NVIDIA installieren.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = NVIDIA-Karte mit dem offenen Treiber

# Small words under that line.
settings-display-nvidia-about = Auf Karten ab 2022 kann es langsam laufen.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Was man dagegen tun kann
