
# machine translated (French), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Ouvrir ses réglages

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Échec : { $why }

# SAFETY: needs a human review before it ships
# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = Cette mise à jour demande une nouvelle autorisation, elle attend donc que vous l'autorisiez.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = La version { $new } est prête. Vous avez la version { $mine }.

# SAFETY: needs a human review before it ships
# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = Vous avez la version { $mine }. La version { $new } demande plus d'autorisations : { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Les boutons reviendront quand c'est terminé.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Recherche des modules…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Aucun module n'est installé, et aucun n'est encore disponible.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Aucun module n'est encore installé. 1 est prêt à installer.
       *[other] Aucun module n'est encore installé. { $n } sont prêts à installer.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 module est installé
       *[other] { $n } modules sont installés
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 a une mise à jour
       *[other] { $n } ont une mise à jour
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 mise à jour est retenue
       *[other] { $n } mises à jour sont retenues
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }. { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, tous à jour.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, et { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, et { $have }. { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Il ne demande rien.

# On a plugin's card: what it asks for.
store-plugins-asks = Ce qu'il demande : { $list }.

# Title of the Plugins page.
store-plugins-title = Modules

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Version { $mine } installée. La version { $new } est prête.

# On a plugin's card: its installed version.
store-plugins-version-installed = Version { $version } installée.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Version { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
store-plugins-install = Installer { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = Autoriser les mises à jour de { $name }

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = Mettre à jour vers { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = D'où viennent les modules

# SAFETY: needs a human review before it ships
# Information on the card that says where plugins come from.
store-plugins-come-from = Les modules viennent uniquement de la liste propre à Trindle, que Trindle lit sur trindle.org à chaque ouverture de cette page. Vous pouvez proposer un module pour la liste. Chacun est testé avant d'y apparaître.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Il est installé, mais la partie qui tourne en arrière-plan ne fonctionne pas : { $why }

# SAFETY: needs a human review before it ships
# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Il n'a pas pu être supprimé : { $why }

