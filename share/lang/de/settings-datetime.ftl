
# machine translated (German), not yet reviewed by a person.

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Datum und Uhrzeit

# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = Zeitzone Uhr Uhrzeit Datum time zone clock date

# Help line at the foot of the Date and time page.
# SAFETY: needs a human review before it ships
settings-datetime-help = Für die Zeitzone und die automatische Zeit wird das Passwort eines Administrators gebraucht.

# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = Die Uhrzeit jetzt

# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 Stunden

# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 Stunden

# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 Stunden, mit Sekunden

# Name of the box for the day when the date is set by hand.
settings-datetime-day = Tag

# Name of the box for the month when the date is set by hand.
settings-datetime-month = Monat

# Name of the box for the year when the date is set by hand.
settings-datetime-year = Jahr

# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Stunde

# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Minute

# Red words when the time could not be set.
settings-datetime-no-time = Die Uhrzeit konnte nicht gesetzt werden.

# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = Die Uhr stellt sich gerade selbst richtig. Schalte das oben aus und stelle dann die Uhrzeit von Hand ein.

# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Der Systembus war nicht erreichbar.

# Red words when the time service does not answer.
settings-datetime-not-answering = Der Zeitdienst antwortet nicht.

# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = Die Uhr konnte nicht angewiesen werden, sich selbst richtig zu stellen.

# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = Die automatische Zeit konnte nicht ausgeschaltet werden.

# Red words when the time zone could not be changed.
settings-datetime-zone-failed = Die Zeitzone konnte nicht geändert werden.

# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = Das ist kein gültiges Datum.

# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = Es ist { $time } am { $date }.

# Headline of the page when the time service is not answering.
settings-datetime-service-down = Der Zeitdienst antwortet nicht, deshalb kann die Uhr hier nicht geändert werden.

# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = Es ist { $time } am { $date }, und die Uhr stellt sich selbst richtig.

# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = Es ist { $time } am { $date }. Die Uhr gleicht sich ab, sobald sie das Internet erreicht.

# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = Es ist { $time } am { $date }. Die Uhr ist von Hand gestellt.

# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }

# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }

# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = Es ist { $time } am { $date }, { $zone }.

# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = Die Uhr in der Mitte des Rings wird in 24 oder 12 Stunden geschrieben, mit oder ohne Sekunden. Nur die Uhr des Rings folgt dem; Datum und Uhrzeit in Programmen folgen den Formaten, die unter „Region und Sprache“ gewählt sind.

# Heading of the card with the clock settings.
settings-datetime-the-clock = Die Uhr

# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = Der Zeitdienst antwortet nicht

# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = Aus dem Internet, wenn verbunden.

# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = Braucht ein installiertes systemd-timesyncd.

# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = Uhrzeit automatisch einstellen

# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Zeitzone

# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Datum und Uhrzeit einstellen

# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Uhrzeit von Hand einstellen

# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = Schalte zuerst die automatische Zeit aus, wie sie jetzt ist.

# Help text of the clock card (the information button) when the time service can keep the time from the internet.
# SAFETY: needs a human review before it ships
settings-datetime-about = Wenn die Zeit automatisch eingestellt wird, gleicht sich die Uhr mit Zeitservern im Internet ab, sobald der Computer verbunden ist, und stellt sich selbst richtig. Ist das aus, werden Datum und Uhrzeit von Hand eingegeben. Die Zeitzone sagt dem Nachtlicht auch, wann die Sonne untergeht. Beides braucht das Passwort eines Administrators. Die Uhr in der Mitte des Rings wird in 24 oder 12 Stunden geschrieben, mit oder ohne Sekunden. Nur die Uhr des Rings folgt dem; Datum und Uhrzeit in Programmen folgen den Formaten, die unter „Region und Sprache“ gewählt sind.

# Help text of the clock card (the information button) when the time service is not installed.
# SAFETY: needs a human review before it ships
settings-datetime-about-no-ntp = Wenn die Zeit automatisch eingestellt wird, gleicht sich die Uhr mit Zeitservern im Internet ab, sobald der Computer verbunden ist, und stellt sich selbst richtig. Ist das aus, werden Datum und Uhrzeit von Hand eingegeben. Die Zeitzone sagt dem Nachtlicht auch, wann die Sonne untergeht. Beides braucht das Passwort eines Administrators. Die Zeit aus dem Internet zu halten braucht systemd-timesyncd: sudo apt install systemd-timesyncd Die Uhr in der Mitte des Rings wird in 24 oder 12 Stunden geschrieben, mit oder ohne Sekunden. Nur die Uhr des Rings folgt dem; Datum und Uhrzeit in Programmen folgen den Formaten, die unter „Region und Sprache“ gewählt sind.

# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })

# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Anzeigen als

# Heading of the line for the ring's clock.
settings-datetime-rings-clock = Die Uhr des Rings

# Under The ring's clock heading.
settings-datetime-rings-clock-sub = Wie der Ring die Uhrzeit schreibt.

