
# machine translated (German), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Einstellungen öffnen

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Nicht erledigt: { $why }

# Line of the built-in Pocketkey row.

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = Dieses Update braucht eine neue Berechtigung. Es wartet auf dich.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Jetzt Version { $mine }, { $new } ist bereit

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = Jetzt Version { $mine }, { $new } ist bereit. Sie verlangt mehr als zuvor: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Die Schaltflächen hier kommen zurück, wenn es fertig ist.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Plugins werden gesucht…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Es sind keine Plugins installiert, und noch keine werden angeboten.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Es sind noch keine Plugins installiert; 1 ist bereit zur Installation.
       *[other] Es sind noch keine Plugins installiert; { $n } sind bereit zur Installation.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 Plugin ist installiert
       *[other] { $n } Plugins sind installiert
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 hat ein Update
       *[other] { $n } haben ein Update
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 Update wird zurückgehalten
       *[other] { $n } Updates werden zurückgehalten
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, alle auf dem neuesten Stand.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, und { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, und { $have }; { $held }.

# On a plugin's card when it asks for nothing.
# SAFETY: needs a human review before it ships
store-plugins-asks-nothing = Es verlangt nichts.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = Was es verlangt: { $list }.

# Title of the Plugins page.
store-plugins-title = Plugins

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Version { $mine } installiert; { $new } ist bereit.

# On a plugin's card: its installed version.
store-plugins-version-installed = Version { $version }, installiert.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Version { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
# SAFETY: needs a human review before it ships
store-plugins-install = { $name } installieren

# Tip of the round button that releases a plugin whose updates are held back.
# SAFETY: needs a human review before it ships
store-plugins-release = { $name } freigeben

# Tip of the round button that updates a plugin to a version.
# SAFETY: needs a human review before it ships
store-plugins-update-to = Auf { $version } aktualisieren

# Heading of the card that says where plugins come from.
store-plugins-where = Woher Plugins kommen

# Information on the card that says where plugins come from.
store-plugins-come-from = Plugins kommen nur aus Trindles eigener Liste, die bei jedem Öffnen dieser Seite von trindle.org gelesen wird. Um eines anzubieten, reiche es ein: Jedes wird getestet, bevor es freigeschaltet wird.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Es ist installiert, aber sein Hintergrundteil läuft nicht: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Es konnte nicht entfernt werden: { $why }
