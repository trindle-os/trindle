
# machine translated (French), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = à l'instant

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] il y a { $n } minute
        [many] il y a { $n } minutes
       *[other] il y a { $n } minutes
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] il y a { $n } heure
        [many] il y a { $n } heures
       *[other] il y a { $n } heures
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] il y a { $n } jour
        [many] il y a { $n } jours
       *[other] il y a { $n } jours
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
settings-blocking-waiting = En attente du mot de passe...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Blocage activé

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Blocage désactivé

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Enregistré

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Saisissez le nom d'un site, par exemple example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Le blocage est désactivé. Les recherches de noms passent par les serveurs de votre réseau, comme avant.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Le blocage démarre.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Les publicités et les sites dangereux sont bloqués, et les recherches de noms passent en privé par { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Le serveur privé est inaccessible sur ce réseau : les serveurs de votre réseau sont donc utilisés pour le moment. Le blocage fonctionne toujours.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Les publicités et les sites dangereux sont bloqués. Les recherches de noms passent par les serveurs de votre réseau, qui peuvent les voir.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Les publicités et les sites dangereux sont bloqués. Les recherches de noms passent par les serveurs de votre VPN, à travers le VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Les publicités et les sites dangereux sont bloqués.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Démarrage

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Désactivé

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = le blocage démarre

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = le blocage est désactivé

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Bloqués aujourd'hui

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = recherches bloquées aujourd'hui

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Listes

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Téléchargement

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Aucune pour l'instant

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = les listes de sites

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = sites listés, téléchargés { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Rien de bloqué pour l'instant

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Les sites apparaissent ici, les plus récents en premier.

# SAFETY: needs a human review before it ships
# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = Toujours laisser passer ce site

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Récemment bloqués

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Les sites apparaissent ici au moment où ils sont bloqués, les plus récents en premier. Le plus laisse passer un site à partir de maintenant. Seuls les sites bloqués sont affichés ici. Les sites autorisés ne sont jamais conservés.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Activez le blocage pour voir ce qu'il bloque.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = p. ex. example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Ajouter un site

# SAFETY: needs a human review before it ships
# Small line under "Add a site".
settings-blocking-add-sub = Lui et tous ses sous-domaines ne sont jamais bloqués.

# SAFETY: needs a human review before it ships
# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Bloquer à nouveau ce site

# SAFETY: needs a human review before it ships
# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = Toujours autorisés

# SAFETY: needs a human review before it ships
# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = Les sites listés ici, et tous leurs sous-domaines, ne sont jamais bloqués, même s'ils figurent sur les listes. Le moins en bloque un à nouveau.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Blocage

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = block adverts ads trackers malware scams dns private lookups quad9 mullvad cloudflare allow blocage publicités pubs traqueurs logiciels malveillants arnaques recherches privées autoriser

# Help text at the top of the page.
settings-blocking-help = Bloque les publicités, les traqueurs et les sites dangereux connus pour tous les navigateurs et toutes les applications de cet ordinateur, et envoie ses recherches de noms par une connexion chiffrée, afin que le réseau où vous vous trouvez ne puisse ni les lire ni les modifier.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Bloquer les publicités et les sites dangereux

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Destination des recherches

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Utilise les listes HaGeZi de publicités, de traqueurs et de sites dangereux connus, téléchargées une fois par jour. Quand cette option est désactivée, rien ne tourne. Les recherches de noms passent par une connexion chiffrée vers le serveur privé choisi ici. Quad9 ne garde aucune trace de qui a demandé. « Votre réseau » bloque toujours, mais n'est pas privé : les serveurs de votre réseau voient chaque recherche.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Listes HaGeZi, téléchargées une fois par jour.

# Heading of the line with the private server dropdown.
settings-blocking-private = Serveur privé

# Small line under Private server.
settings-blocking-private-sub = Quad9 ne garde aucune trace de qui a demandé.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Votre réseau
