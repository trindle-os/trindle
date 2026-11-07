# machine translated (French), not yet reviewed by a person.

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Date et heure

# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = fuseau horaire horloge heure date

# Help line at the foot of the Date and time page.
# SAFETY: needs a human review before it ships
settings-datetime-help = Le fuseau horaire et l'heure automatique demandent le mot de passe d'un administrateur.

# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = L'heure actuelle

# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 heures

# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 heures

# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 heures, avec les secondes

# Name of the box for the day when the date is set by hand.
settings-datetime-day = Jour

# Name of the box for the month when the date is set by hand.
settings-datetime-month = Mois

# Name of the box for the year when the date is set by hand.
settings-datetime-year = Année

# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Heure

# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Minute

# Red words when the time could not be set.
settings-datetime-no-time = L'heure n'a pas pu être réglée.

# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = L'horloge se met à l'heure toute seule pour le moment. Désactivez cela plus haut, puis réglez l'heure à la main.

# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Le bus du système est injoignable.

# Red words when the time service does not answer.
settings-datetime-not-answering = Le service de l'heure ne répond pas.

# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = L'horloge n'a pas pu être réglée pour se mettre à l'heure toute seule.

# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = L'heure automatique n'a pas pu être désactivée.

# Red words when the time zone could not be changed.
settings-datetime-zone-failed = Le fuseau horaire n'a pas pu être changé.

# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = Cette date n'existe pas.

# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = Il est { $time } le { $date }.

# Headline of the page when the time service is not answering.
settings-datetime-service-down = Le service de l'heure ne répond pas, l'horloge ne peut donc pas être modifiée ici.

# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = Il est { $time } le { $date }, et l'horloge se met à l'heure toute seule.

# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = Il est { $time } le { $date }. L'horloge se vérifie dès qu'elle atteint internet.

# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = Il est { $time } le { $date }. L'horloge est réglée à la main.

# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }

# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }

# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = Il est { $time } le { $date }, { $zone }.

# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = L'horloge au milieu de l'anneau s'écrit en 24 ou 12 heures, avec ou sans secondes. Seule l'horloge de l'anneau suit ce choix ; les dates et heures dans les programmes suivent les formats choisis dans Région et langue.

# Heading of the card with the clock settings.
settings-datetime-the-clock = L'horloge

# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = Le service de l'heure ne répond pas

# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = Depuis internet, quand l'ordinateur est connecté.

# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = Demande que systemd-timesyncd soit installé.

# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = Régler l'heure automatiquement

# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Fuseau horaire

# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Régler la date et l'heure

# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Régler l'heure à la main

# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = Désactivez d'abord l'heure automatique, comme maintenant.

# Help text of the clock card (the information button) when the time service can keep the time from the internet.
# SAFETY: needs a human review before it ships
settings-datetime-about = Réglée automatiquement, l'horloge est comparée aux serveurs de temps sur internet chaque fois que l'ordinateur est connecté, et reste à l'heure. Désactivée, la date et l'heure se saisissent à la main. Le fuseau horaire indique aussi à la lumière de nuit quand le soleil se couche. Les deux demandent le mot de passe d'un administrateur. L'horloge au milieu de l'anneau s'écrit en 24 ou 12 heures, avec ou sans secondes. Seule l'horloge de l'anneau suit ce choix ; les dates et heures dans les programmes suivent les formats choisis dans Région et langue.

# Help text of the clock card (the information button) when the time service is not installed.
# SAFETY: needs a human review before it ships
settings-datetime-about-no-ntp = Réglée automatiquement, l'horloge est comparée aux serveurs de temps sur internet chaque fois que l'ordinateur est connecté, et reste à l'heure. Désactivée, la date et l'heure se saisissent à la main. Le fuseau horaire indique aussi à la lumière de nuit quand le soleil se couche. Les deux demandent le mot de passe d'un administrateur. Garder l'heure depuis internet demande systemd-timesyncd : sudo apt install systemd-timesyncd L'horloge au milieu de l'anneau s'écrit en 24 ou 12 heures, avec ou sans secondes. Seule l'horloge de l'anneau suit ce choix ; les dates et heures dans les programmes suivent les formats choisis dans Région et langue.

# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })

# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Afficher comme

# Heading of the line for the ring's clock.
settings-datetime-rings-clock = L'horloge de l'anneau

# Under The ring's clock heading.
settings-datetime-rings-clock-sub = La façon dont l'anneau écrit l'heure.
