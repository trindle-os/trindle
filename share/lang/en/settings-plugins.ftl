# Settings: the Your plugins page (src/settings/panels/plugins.rs).

## The Your plugins page (src/settings/panels/plugins.rs)

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Your plugins
# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = plugins add-ons extensions installed remove settings
# Help line at the foot of the Your plugins page.
settings-plugins-help = Plugins add to Trindle: an icon in the ring, settings here, notifications or something that runs from login. Each runs walled off, allowed nothing until you allow it in Permissions, which the button at the end of this page's title opens. Each plugin's card here holds its own settings; the bin on a card, pressed twice, removes that plugin.
# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = The window that asks what it may use could not be opened.
# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Version { $v }.
# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Version { $v }. { $settled }
# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } asks for nothing, so it runs walled off with nothing allowed.
# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } asks for { $list }; { $allowed } of { $asks } allowed. It runs walled off, with only what you allow.
       *[other] { $name } asks for { $list }; { $allowed } of { $asks } allowed. It runs walled off, with only what you allow.
    }
# Under the line that tells a plugin needs more allowed, beside its Allow button.
settings-plugins-needs-more = Allow shows each thing it asks for, with a switch.
# Heading of the line about a plugin on its card.
settings-plugins-about-it = About it
# Tip of the button that changes what a plugin may use.
settings-plugins-change = Change what it may use
# Tip of the button that removes a plugin.
settings-plugins-remove = Remove this plugin
# Red words when a plugin could not be removed.
settings-plugins-not-removed = It could not be removed.
# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Type here
# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } has no settings of its own.
# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } has no settings to show as things stand.
# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] No plugins are installed.
        [one] One plugin is installed: { $list }.
       *[other] { $n } plugins are installed: { $list }.
    }
# Heading of the card shown when no plugin is installed.
settings-plugins-group = Plugins
# Heading of the line shown when no plugin is installed.
settings-plugins-none = None installed
# Under the None installed heading.
settings-plugins-none-sub = Apps has plugins to add.
# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = Open Permissions

## Cards drawn from a plugin's rows file (src/settings/panels/plugin_rows.rs)

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Asking { $name }.
# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } is not answering. Its service may be stopped.
# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Saved.
# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } did not accept that.
# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } could not do that.
# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } could not do that: { $why }.
