
# machine translated (French), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Ouvrir ses réglages

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Non fait : { $why }

# SAFETY: needs a human review before it ships
# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = Cette mise à jour demande une nouvelle permission. Elle attend votre réponse.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Version { $mine } actuellement, la { $new } est prête

# SAFETY: needs a human review before it ships
# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = Version { $mine } actuellement, la { $new } est prête. Elle demande plus qu'avant : { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Les boutons ici reviennent quand c'est terminé.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Recherche des extensions…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Aucune extension n'est installée, et aucune n'est encore proposée.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Aucune extension n'est encore installée ; 1 est prête à installer.
       *[other] Aucune extension n'est encore installée ; { $n } sont prêtes à installer.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 extension est installée
       *[other] { $n } extensions sont installées
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
store-plugins-line-held = { $installed } ; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, toutes à jour.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, et { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, et { $have } ; { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Elle ne demande rien.

# On a plugin's card: what it asks for.
store-plugins-asks = Ce qu'elle demande : { $list }.

# Title of the Plugins page.
store-plugins-title = Extensions

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Version { $mine } installée ; la { $new } est prête.

# On a plugin's card: its installed version.
store-plugins-version-installed = Version { $version }, installée.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Version { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
store-plugins-install = Installer { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = Libérer { $name }

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = Mettre à jour vers { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = D'où viennent les extensions

# SAFETY: needs a human review before it ships
# Information on the card that says where plugins come from.
store-plugins-come-from = Les extensions viennent uniquement de la liste propre à Trindle, lue sur trindle.org à chaque ouverture de cette page. Pour en proposer une, soumettez-la : chacune est testée avant d'être mise en ligne.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Elle est installée, mais sa partie en arrière-plan ne tourne pas : { $why }

# SAFETY: needs a human review before it ships
# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Elle n'a pas pu être supprimée : { $why }

