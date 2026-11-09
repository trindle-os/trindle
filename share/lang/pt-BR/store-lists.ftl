# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Um arquivo Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Um arquivo .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Plugin

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = candidata a lançamento

# Tip of the round button on an app's tile while it is being removed.
# SAFETY: needs a human review before it ships
store-list-removing = Removendo { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Instalando { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Abrir { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Instalar { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Instalado

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Mostrar

# How long is left when it is a minute or less.
store-left-minute = falta cerca de um minuto

# How long is left, in minutes.
store-left-minutes = faltam cerca de { $n } minutos

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] falta cerca de 1 hora
        [many] faltam cerca de { $n } horas
       *[other] faltam cerca de { $n } horas
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Verificado na lista Threat Intelligence do HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Baixado de { $site }, a partir de { $referrer }.

# Where a download came from: the site.
store-origin-site = Baixado de { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Acessado a partir de { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = O Trindle não consegue saber de onde isto veio.

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } está na lista Threat Intelligence do HaGeZi de sites conhecidos por downloads nocivos e golpes. Só instale isto se tiver certeza de onde veio.

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } O site não está na lista Threat Intelligence do HaGeZi, mas a lista não é completa, então isso não prova que seja seguro.

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } Não foi possível verificar o site, pois a lista Threat Intelligence do HaGeZi não pôde ser obtida.

# Name shown for a review whose writer gave none.
store-review-someone = Alguém

# Red words when the keyring could not be reached.
store-key-unreached = Não foi possível acessar o chaveiro: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = A chave guardada não é texto.

# Red words when a file that held the key could not be removed.
store-key-not-removed = Não foi possível remover { $path }: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = Não foi possível ler { $path }: { $why }
