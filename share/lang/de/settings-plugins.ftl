
# machine translated (German), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Deine Plugins

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = plugins erweiterungen add-ons installiert entfernen einstellungen

# Help line at the foot of the Your plugins page.
settings-plugins-help = Plugins erweitern Trindle: ein Symbol im Ring, Einstellungen hier, Benachrichtigungen oder etwas, das ab der Anmeldung läuft. Jedes läuft abgeschottet und darf nichts, bis du es unter Berechtigungen erlaubst; diese Seite öffnest du mit der Schaltfläche am Ende des Seitentitels. Die Karte jedes Plugins enthält seine eigenen Einstellungen; der Papierkorb auf einer Karte entfernt das Plugin, wenn du ihn zweimal drückst.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Das Fenster, das fragt, was es verwenden darf, konnte nicht geöffnet werden.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Version { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Version { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } fordert nichts an und läuft daher abgeschottet, ohne dass etwas erlaubt ist.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } fordert { $list } an; { $allowed } von { $asks } erlaubt. Es läuft abgeschottet und nur mit dem, was du erlaubst.
       *[other] { $name } fordert { $list } an; { $allowed } von { $asks } erlaubt. Es läuft abgeschottet und nur mit dem, was du erlaubst.
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = Erlauben zeigt jedes angeforderte Ding mit einem Schalter.

# Heading of the line about a plugin on its card.
settings-plugins-about-it = Über das Plugin

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Ändern, was es verwenden darf

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Dieses Plugin entfernen

# Red words when a plugin could not be removed.
settings-plugins-not-removed = Es konnte nicht entfernt werden.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Hier eingeben

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } hat keine eigenen Einstellungen.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } hat im Moment keine Einstellungen zum Anzeigen.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Es sind keine Plugins installiert.
        [one] Ein Plugin ist installiert: { $list }.
       *[other] { $n } Plugins sind installiert: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Plugins

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Keine installiert

# Under the None installed heading.
settings-plugins-none-sub = In Apps gibt es Plugins zum Hinzufügen.

# Tip of the button at the end of the page's title that opens the Permissions page.
# SAFETY: needs a human review before it ships
settings-plugins-open-permissions = Berechtigungen öffnen

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name } wird gefragt.

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } antwortet nicht. Der Dienst ist vielleicht angehalten.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Gespeichert.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } hat das nicht angenommen.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } konnte das nicht tun.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } konnte das nicht tun: { $why }.

