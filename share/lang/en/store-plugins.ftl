# Apps: the Plugins page (src/store/plugin_page.rs).

## The Plugins page (src/store/plugin_page.rs)

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Open its settings
# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Not done: { $why }
# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = This update needs a new permission. It is waiting for you.
# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Version { $mine } now, { $new } is ready
# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = Version { $mine } now, { $new } is ready. It asks for more than before: { $more }. { $waits }
# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = The buttons here come back when it has finished.
# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Looking for plugins…
# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = No plugins are installed, and none are on offer yet.
# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] No plugins are installed yet; 1 is ready to install.
       *[other] No plugins are installed yet; { $n } are ready to install.
    }
# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 plugin is installed
       *[other] { $n } plugins are installed
    }
# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 has an update
       *[other] { $n } have an update
    }
# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 update is held back
       *[other] { $n } updates are held back
    }
# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.
# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.
# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, all up to date.
# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, and { $have }.
# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, and { $have }; { $held }.
# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = It asks for nothing.
# On a plugin's card: what it asks for.
store-plugins-asks = What it asks for: { $list }.
# Title of the Plugins page.
store-plugins-title = Plugins
# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Version { $mine } installed; { $new } is ready.
# On a plugin's card: its installed version.
store-plugins-version-installed = Version { $version }, installed.
# On a plugin's card: the version on offer.
store-plugins-version-offered = Version { $version }.
# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.
# Tip of the round button that installs a plugin.
store-plugins-install = Install { $name }
# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = Release { $name }
# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = Update to { $version }
# Heading of the card that says where plugins come from.
store-plugins-where = Where plugins come from
# Information on the card that says where plugins come from.
store-plugins-come-from = Plugins come only from Trindle's own list, read from trindle.org each time this page opens. To offer one, submit it: each is tested before it goes live.
# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = It is installed, but its background part is not running: { $why }
# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = It could not be removed: { $why }
