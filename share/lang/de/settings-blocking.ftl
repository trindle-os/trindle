
# machine translated (German), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = gerade eben

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] vor { $n } Minute
       *[other] vor { $n } Minuten
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] vor { $n } Stunde
       *[other] vor { $n } Stunden
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] vor { $n } Tag
       *[other] vor { $n } Tagen
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Warten auf das Passwort ...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Blockieren ist an

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Blockieren ist aus

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Gespeichert

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Gib den Namen einer Website ein, zum Beispiel example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Blockieren ist aus. Abfragen gehen wie bisher an die eigenen Server deines Netzwerks.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Blockieren wird gestartet.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Werbung und schlechte Websites werden blockiert, und Abfragen gehen privat an { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Der private Server ist in diesem Netzwerk nicht erreichbar, daher werden vorerst die eigenen Server deines Netzwerks verwendet. Das Blockieren funktioniert weiterhin.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Werbung und schlechte Websites werden blockiert. Abfragen gehen an die eigenen Server deines Netzwerks, die sie sehen können.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Werbung und schlechte Websites werden blockiert. Abfragen gehen durch das VPN an die eigenen Server deines VPNs.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Werbung und schlechte Websites werden blockiert.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Startet

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Aus

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = Blockieren wird gestartet

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = Blockieren ist aus

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Heute blockiert

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = heute blockierte Abfragen

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Listen

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Wird geladen

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Noch keine

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = die Listen der Websites

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = Websites gelistet, geladen { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Noch nichts blockiert

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Websites erscheinen hier, die neuesten zuerst.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Diese Website immer durchlassen

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Zuletzt blockiert

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Websites erscheinen hier, sobald sie blockiert werden, die neuesten zuerst. Das Plus lässt eine ab jetzt durch. Hier werden nur blockierte Websites gezeigt. Durchgelassene Websites werden nie gespeichert.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Schalte das Blockieren ein, um zu sehen, was es blockiert.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = z. B. example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Eine Website hinzufügen

# Small line under "Add a site".
# SAFETY: needs a human review before it ships
settings-blocking-add-sub = Sie und alle ihre Subdomains werden nie blockiert.

# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Diese Website wieder blockieren

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Immer durchlassen

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Hier aufgeführte Websites und alle ihre Subdomains werden nie blockiert, auch wenn sie auf den Listen stehen. Das Minus blockiert eine wieder.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Blockieren

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = blockieren werbung ads tracker malware betrug dns privat abfragen quad9 mullvad cloudflare erlauben

# Help text at the top of the page.
settings-blocking-help = Blockiert Werbung, Tracker und bekannte schlechte Websites für jeden Browser und jede App auf diesem Computer und sendet seine Namensabfragen über eine verschlüsselte Verbindung, damit das Netzwerk, in dem du bist, sie weder lesen noch ändern kann.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Werbung und schlechte Websites blockieren

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Wohin Abfragen gehen

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Verwendet die Listen von HaGeZi mit Werbung, Trackern und bekannten schlechten Websites, einmal am Tag geladen. Solange das ausgeschaltet ist, läuft nichts. Namensabfragen gehen über eine verschlüsselte Verbindung an den hier gewählten privaten Server. Quad9 speichert nicht, wer gefragt hat. „Dein Netzwerk“ blockiert weiterhin, ist aber nicht privat: Die eigenen Server deines Netzwerks sehen jede Abfrage.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Die Listen von HaGeZi, einmal am Tag geladen.

# Heading of the line with the private server dropdown.
settings-blocking-private = Privater Server

# Small line under Private server.
settings-blocking-private-sub = Quad9 speichert nicht, wer gefragt hat.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Dein Netzwerk
