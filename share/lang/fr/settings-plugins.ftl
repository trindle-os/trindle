# machine translated (French), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Vos modules

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = modules plugins extensions installés supprimer retirer réglages

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = Les modules ajoutent des choses à Trindle. Un module peut ajouter une icône à l'anneau, des réglages ici, des notifications, ou quelque chose qui démarre quand vous ouvrez votre session. Chaque module s'exécute dans sa propre boîte et ne peut rien utiliser tant que vous ne l'autorisez pas dans Autorisations. Le bouton à côté du titre de la page ouvre Autorisations. Pour retirer un module, appuyez deux fois sur la corbeille de sa carte.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Trindle n'a pas pu ouvrir la fenêtre qui demande ce que le module peut utiliser.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Version { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Version { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } ne demande rien, il s'exécute donc dans sa propre boîte, sans rien d'autorisé.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } demande { $list }. Vous avez autorisé { $allowed } sur { $asks }. Il s'exécute dans sa propre boîte, avec seulement ce que vous autorisez.
        [many] { $name } demande { $list }. Vous avez autorisé { $allowed } sur { $asks }. Il s'exécute dans sa propre boîte, avec seulement ce que vous autorisez.
       *[other] { $name } demande { $list }. Vous avez autorisé { $allowed } sur { $asks }. Il s'exécute dans sa propre boîte, avec seulement ce que vous autorisez.
    }

# Heading of the line about a plugin on its card.
settings-plugins-about-it = À propos

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Changer ce qu'il peut utiliser

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Retirer ce module

# Red words when a plugin could not be removed.
settings-plugins-not-removed = Impossible de le retirer.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Écrivez ici

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } n'a pas de réglages propres.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } n'a aucun réglage à montrer pour le moment.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Aucun module n'est installé.
        [one] Un module est installé : { $list }.
        [many] { $n } de modules sont installés : { $list }.
       *[other] { $n } modules sont installés : { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Modules

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Aucun d'installé

# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = Ouvrir Autorisations

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Demande à { $name }…

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } ne répond pas. Son service est peut-être arrêté.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Enregistré.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } ne l'a pas accepté.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } n'a pas pu le faire.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } n'a pas pu le faire : { $why }.
