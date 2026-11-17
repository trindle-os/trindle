# Settings, Blocking page: adverts and bad sites kept out for the whole computer, and private name lookups.

## Blocking page (src/settings/panels/blocking.rs)

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = just now
# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] { $n } minute ago
       *[other] { $n } minutes ago
    }
# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] { $n } hour ago
       *[other] { $n } hours ago
    }
# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] { $n } day ago
       *[other] { $n } days ago
    }
# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
settings-blocking-waiting = Waiting for your password…
# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Blocking on
# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Blocking off
# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Saved
# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Type a site's name, such as example.com.
# Headline of the page when blocking is off.
settings-blocking-top-off = Blocking is off. Websites are looked up through your network's own servers, as before.
# Headline of the page while blocking is starting.
settings-blocking-top-starting = Blocking is starting.
# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Adverts and bad sites are blocked, and websites are looked up privately through { $provider }.
# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle can't reach the private server on this network, so it's using your network's own servers for now. Blocking still works.
# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Adverts and bad sites are blocked. Websites are looked up through your network's own servers, which can see each site you visit.
# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Adverts and bad sites are blocked. Websites are looked up through your VPN's own servers.
# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Adverts and bad sites are blocked.
# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Starting
# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Off
# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = blocking is starting
# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = blocking is off
# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Blocked today
# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = lookups blocked today
# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Lists
# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Updating
# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = None yet
# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = the lists of sites
# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = sites listed, updated { $ago }
# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Nothing blocked yet
# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = Always let this site through
# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Recently blocked
# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Sites appear here as they're blocked, newest first. Press the plus to let a site through from now on. Trindle doesn't keep a record of sites it lets through.
# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Switch on blocking to see what it blocks.
# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com
# Heading of the line with the box for adding a site.
settings-blocking-add = Add a site
# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Block this site again
# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = Always let through
# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = These sites, and every address under them, are never blocked, even if they're on the lists. Press the minus to block a site again.
# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Firewall and blocking
# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = block adverts ads trackers malware scams dns private lookups quad9 mullvad cloudflare allow
# Help text at the top of the page.
settings-blocking-help = Blocks adverts, trackers and known bad sites for every browser and app on this computer. It also looks up websites over an encrypted connection, so the network you're on can't see or change which sites you look up.
# Text of the switch that turns blocking on or off.
settings-blocking-switch = Block adverts and bad sites
# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Where lookups go
# Help text (i button) of the Blocking card.
settings-blocking-card-about = Uses HaGeZi's lists of adverts, trackers and known bad sites, updated once a day. While blocking is off, nothing runs. Websites are looked up over an encrypted connection through the private server you choose here. Quad9 keeps no record of who asked. "Your network" still blocks, but it isn't private, because your network's own servers see every site you look up.
# Heading of the line with the private server dropdown.
settings-blocking-private = Private server
# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Your network

# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = Firewall

# Help text (i button) of the Firewall card.
settings-firewall-about = When the firewall is on, other computers can't connect to this one unless this computer started the connection. So nobody on the same Wi-Fi can try to get in. Browsing, updates and printers still work. A program that waits for other computers to connect, such as a game server, needs its port opened below. When the firewall is off, any computer on the network can reach every program that is waiting for connections. You need the administrator's password to switch it on or off.

# Name of the firewall's on and off switch.
settings-firewall-switch = Firewall


# Small line under the switch while the firewall is off.
settings-firewall-off-sub = Other computers can reach programs that wait for them.

# Said beside the switch once the firewall is on.
settings-firewall-done-on = Firewall on

# Said beside the switch once the firewall is off.
settings-firewall-done-off = Firewall off

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = Blocking

# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = The firewall is on but isn't working, because its rules didn't load when the computer started. Switch it off and on again.
