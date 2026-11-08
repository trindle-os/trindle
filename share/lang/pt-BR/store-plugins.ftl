
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Abrir as configurações dele

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Não foi feito: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = Esta atualização precisa de uma permissão nova. Ela está esperando por você.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Versão { $mine } agora, a { $new } está pronta

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = Versão { $mine } agora, a { $new } está pronta. Ela pede mais do que antes: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Os botões daqui voltam quando terminar.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Procurando plugins…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Nenhum plugin está instalado, e ainda não há nenhum disponível.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Nenhum plugin está instalado ainda; 1 está pronto para instalar.
        [many] Nenhum plugin está instalado ainda; { $n } de plugins estão prontos para instalar.
       *[other] Nenhum plugin está instalado ainda; { $n } estão prontos para instalar.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 plugin está instalado
        [many] { $n } de plugins estão instalados
       *[other] { $n } plugins estão instalados
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 tem atualização
        [many] { $n } de plugins têm atualização
       *[other] { $n } têm atualização
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 atualização está retida
        [many] { $n } de atualizações estão retidas
       *[other] { $n } atualizações estão retidas
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, todos atualizados.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, e { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, e { $have }; { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Ele não pede nada.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = O que ele pede: { $list }.

# Title of the Plugins page.
store-plugins-title = Plugins

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Versão { $mine } instalada; a { $new } está pronta.

# On a plugin's card: its installed version.
store-plugins-version-installed = Versão { $version }, instalada.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Versão { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
store-plugins-install = Instalar { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = Liberar { $name }

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = Atualizar para { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = De onde vêm os plugins

# Information on the card that says where plugins come from.
store-plugins-come-from = Os plugins vêm apenas da lista do próprio Trindle, lida em trindle.org toda vez que esta página abre. Para oferecer um, envie-o: cada um é testado antes de entrar no ar.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Ele está instalado, mas a parte dele que roda em segundo plano não está funcionando: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Não foi possível removê-lo: { $why }
