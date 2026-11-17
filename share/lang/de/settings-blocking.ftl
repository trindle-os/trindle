
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
settings-blocking-waiting = Warten auf dein Passwort …

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Blockieren ist an

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Blockieren ist aus

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Gespeichert

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Gib den Namen einer Website ein, zum Beispiel example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Blockieren ist aus. Websites werden wie bisher über die eigenen Server deines Netzwerks nachgeschlagen.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Blockieren wird gestartet.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Werbung und schlechte Websites werden blockiert, und Websites werden privat über { $provider } nachgeschlagen.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle kann den privaten Server in diesem Netzwerk nicht erreichen und verwendet deshalb vorerst die eigenen Server deines Netzwerks. Das Blockieren funktioniert weiterhin.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Werbung und schlechte Websites werden blockiert. Websites werden über die eigenen Server deines Netzwerks nachgeschlagen, die jede Website sehen können, die du besuchst.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Werbung und schlechte Websites werden blockiert. Websites werden über die eigenen Server deines VPNs nachgeschlagen.

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
settings-blocking-tile-fetching = Wird aktualisiert

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Noch keine

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = die Listen der Websites

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = Websites gelistet, aktualisiert { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Noch nichts blockiert

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Diese Website immer durchlassen

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Zuletzt blockiert

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Websites erscheinen hier, sobald sie blockiert werden, die neuesten zuerst. Drücke auf das Plus, um eine Website ab jetzt durchzulassen. Trindle speichert nicht, welche Websites es durchlässt.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Schalte das Blockieren ein, um zu sehen, was es blockiert.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Eine Website hinzufügen

# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Diese Website wieder blockieren

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Immer durchlassen

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Diese Websites und jede Adresse darunter werden nie blockiert, auch wenn sie auf den Listen stehen. Drücke auf das Minus, um eine Website wieder zu blockieren.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Blockieren

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = blockieren werbung ads tracker malware betrug dns privat abfragen quad9 mullvad cloudflare erlauben

# Help text at the top of the page.
settings-blocking-help = Blockiert Werbung, Tracker und bekannte schlechte Websites für jeden Browser und jede App auf diesem Computer. Außerdem schlägt es Websites über eine verschlüsselte Verbindung nach, damit das Netzwerk, in dem du bist, nicht sehen oder ändern kann, welche Websites du aufrufst.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Werbung und schlechte Websites blockieren

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Wohin Abfragen gehen

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Verwendet die Listen von HaGeZi mit Werbung, Trackern und bekannten schlechten Websites, einmal am Tag aktualisiert. Solange das Blockieren ausgeschaltet ist, läuft nichts. Websites werden über eine verschlüsselte Verbindung beim hier gewählten privaten Server nachgeschlagen. Quad9 speichert nicht, wer gefragt hat. „Dein Netzwerk“ blockiert weiterhin, ist aber nicht privat, weil die eigenen Server deines Netzwerks jede Website sehen, die du nachschlägst.

# Heading of the line with the private server dropdown.
settings-blocking-private = Privater Server

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Dein Netzwerk

# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = Firewall

# Help text (i button) of the Firewall card.
settings-firewall-about = Wenn die Firewall eingeschaltet ist, können sich andere Computer nicht mit diesem verbinden, es sei denn, dieser Computer hat die Verbindung selbst begonnen. So kann niemand im selben WLAN versuchen, hineinzukommen. Surfen, Updates und Drucker funktionieren weiterhin. Ein Programm, das auf Verbindungen von anderen Computern wartet, etwa ein Spieleserver, braucht seinen Port unten geöffnet. Wenn die Firewall ausgeschaltet ist, kann jeder Computer im Netzwerk jedes Programm erreichen, das auf Verbindungen wartet. Zum Ein- oder Ausschalten brauchst du das Passwort des Administrators.

# Name of the firewall's on and off switch.
settings-firewall-switch = Firewall

# Small line under the switch while the firewall is off.
settings-firewall-off-sub = Andere Computer können Programme erreichen, die auf Verbindungen warten.

# Said beside the switch once the firewall is on.
settings-firewall-done-on = Firewall an

# Said beside the switch once the firewall is off.
settings-firewall-done-off = Firewall aus

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = Blockieren



# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = Die Firewall ist eingeschaltet, wirkt aber nicht, weil ihre Regeln beim Start des Computers nicht geladen wurden. Schalte sie aus und wieder ein.
