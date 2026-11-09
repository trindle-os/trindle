# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Seus plugins

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = plugins complementos extensões instalados remover configurações

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = Plugins acrescentam coisas ao Trindle: um ícone no anel, configurações aqui, notificações ou algo que roda desde o login. Cada um roda isolado, sem permissão para nada até você permitir em Permissões, que o botão no fim do título desta página abre. O cartão de cada plugin aqui guarda suas próprias configurações; a lixeira em um cartão, pressionada duas vezes, remove esse plugin.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Não foi possível abrir a janela que pergunta o que ele pode usar.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Versão { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Versão { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } não pede nada, então roda isolado, sem nada permitido.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } pede { $list }; { $allowed } de { $asks } permitidos. Roda isolado, só com o que você permitir.
        [many] { $name } pede { $list }; { $allowed } de { $asks } permitidos. Roda isolado, só com o que você permitir.
       *[other] { $name } pede { $list }; { $allowed } de { $asks } permitidos. Roda isolado, só com o que você permitir.
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = Permitir mostra cada coisa que ele pede, com um interruptor.

# Heading of the line about a plugin on its card.
settings-plugins-about-it = Sobre ele

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Mudar o que ele pode usar

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Remover este plugin

# Red words when a plugin could not be removed.
# SAFETY: needs a human review before it ships
settings-plugins-not-removed = Não foi possível removê-lo.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Digite aqui

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } não tem configurações próprias.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } não tem configurações a mostrar no momento.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Nenhum plugin está instalado.
        [one] Um plugin está instalado: { $list }.
        [many] { $n } de plugins estão instalados: { $list }.
       *[other] { $n } plugins estão instalados: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Plugins

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Nenhum instalado

# Under the None installed heading.
settings-plugins-none-sub = Aplicativos tem plugins para adicionar.

# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = Abrir Permissões

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Perguntando a { $name }.

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } não está respondendo. O serviço dele pode estar parado.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Salvo.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } não aceitou isso.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } não conseguiu fazer isso.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } não conseguiu fazer isso: { $why }.

