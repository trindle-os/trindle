
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Pacotes do Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Retido na versão { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Retido; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Retido na versão { $have }

# On a thing held back with no update waiting.
store-upd-held = Retido

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Retido na versão { $have }; nada mais novo ainda

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Retido; nada mais novo ainda

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 atualização
        [many] { $n } atualizações
       *[other] { $n } atualizações
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 atualização está pronta
        [many] { $n } atualizações estão prontas
       *[other] { $n } atualizações estão prontas
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, { $size } no total

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 está retida
        [many] { $n } estão retidas
       *[other] { $n } estão retidas
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = O firmware precisa de uma reinicialização para terminar.

# Headline while the check runs.
store-upd-checking = Verificando atualizações.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Atualizando { $name }. Você pode continuar trabalhando enquanto ele é instalado.

# Headline while all the updates go in.
store-upd-installing-all = Instalando as atualizações. Você pode continuar trabalhando enquanto elas são instaladas.

# SAFETY: needs a human review before it ships
# Headline when the updates are in and the computer must start again.
store-upd-restart = Reinicie para terminar: as atualizações foram instaladas e passam a valer quando o computador iniciar de novo.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = O que foi instalado precisa de uma reinicialização para terminar.

# Added to the headline with no internet.
store-upd-no-more = Este computador não está conectado à internet, então não é possível procurar mais.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Nenhuma atualização está pronta

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Nenhuma atualização está esperando, até onde este computador sabe. Ele não pode procurar mais enquanto não estiver conectado à internet.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = os aplicativos

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = o Trindle e o Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = o firmware

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Não foi possível verificar atualizações para { $names }.

# Headline when everything is up to date.
store-upd-current = Tudo está atualizado.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Última verificação às { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Não pode ser verificado sem a internet

# Red words under a card when a check took too long.
store-upd-too-long = A resposta demorou demais.

# Tip of the round button that checks again.
store-upd-check-again = Verificar de novo

# Main button while the check runs.
store-upd-checking-main = Verificando

# Main button while the updates go in.
store-upd-installing-main = Instalando

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] Instalar 1 atualização
        [many] Instalar { $n } atualizações
       *[other] Instalar { $n } atualizações
    }

# SAFETY: needs a human review before it ships
# Main button that restarts the computer.
store-upd-restart-now = Reiniciar agora

# Main button that checks for updates.
store-upd-check-for = Verificar atualizações

# Tip of the round button that releases a thing held back.
store-upd-release = Liberar { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = Reter { $name }

# A card's state while it is being checked.
store-upd-status-checking = Verificando

# A card's state while its updates go in.
store-upd-status-installing = Instalando

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Não foi possível verificar. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } retidas

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } retidas

# A card's state when nothing waits.
store-upd-up-to-date = Atualizado

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = De { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = De { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Uma parte compartilhada, de { $from }

# Note under the Apps card.
store-upd-yours-alone = Só seus, então são atualizados sem senha. Um aplicativo traz as partes compartilhadas de que a nova versão precisa.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Uma atualização retida fica na versão que tem, e fora de Instalar tudo aqui e no anel, até você liberá-la.

# Heading of the Apps card.
store-upd-card-apps = Aplicativos

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Todos os plugins estão atualizados.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] A atualização de 1 plugin está retida.
        [many] As atualizações de { $n } plugins estão retidas.
       *[other] As atualizações de { $n } plugins estão retidas.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 plugin tem uma atualização
        [many] { $n } plugins têm uma atualização
       *[other] { $n } plugins têm uma atualização
    }

# Heading of the Plugins card.
store-upd-card-plugins = Plugins

# Held back card: nothing is held back.
store-upd-nothing-held = Nada está retido.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Liberado

# Note under the Held back card: what releasing does.
store-upd-release-why = O que for liberado volta a ser instalado com o resto, por Instalar tudo aqui e no anel.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = O firmware de um dispositivo

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = o firmware do dispositivo { $id }

# Heading of the Held back card.
store-upd-card-held = Retido

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } para { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 atualização, que é instalada sozinha
        [many] { $n } atualizações, que são instaladas juntas
       *[other] { $n } atualizações, que são instaladas juntas
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Do Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Mostrando { $shown } de { $total } atualizações do Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Este computador tem o Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle e Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Os pacotes do Debian são feitos para combinar entre si, e um instalado sozinho pode trazer outros junto ou deixá-los fora de compasso. Por isso são instalados juntos, por um só botão.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = As atualizações do Trindle e do Debian trazem correções de segurança para o computador inteiro, por isso não podem ser retidas.

# Heading of the card about the daily security updates.
store-upd-card-security = Atualizações de segurança

# On the security card: its state.
store-upd-security-on = Ligado, todo dia

# On the security card.
store-upd-security-note = As correções do Debian são instaladas todo dia, sem perguntar.

# On the security card.
store-upd-security-part = Parte do Trindle; não pode ser desligado.

# On the security card: that holding back never stops these.
store-upd-never-held = Reter uma atualização nunca impede estas.

# Firmware card: the computer has no updater.
store-upd-fw-none = Este computador não tem atualizador de firmware.

# Firmware card: restart to finish.
store-upd-fw-restart = Reinicie para terminar

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Versão { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = O firmware novo é instalado quando o computador reinicia.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Instale o fwupd para manter o firmware atualizado.

# Heading of the Firmware card.
store-upd-card-firmware = Firmware

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = O fwupd não conseguiu baixar a lista mais recente.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Não foi possível acessar o Flathub.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Não foi possível baixar as listas do Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Não foi possível baixar as listas do Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Não retido. A lista do que está retido não pôde ser guardada: { $why }.

# The same, for a release.
store-upd-not-released-list = Não liberado. A lista do que está retido não pôde ser guardada: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Retido em Aplicativos, mas o próprio Flatpak não pôde ser avisado, então outra coisa que atualiza aplicativos ainda poderia instalá-lo.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Liberado em Aplicativos, mas o próprio Flatpak não pôde ser avisado e pode continuar deixando-o de fora. Retenha-o e libere-o de novo.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } O Flatpak disse: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle



# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = A verificação automática de atualizações está desligada. Aperte Verificar atualizações para procurar.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Não verificado

# Tip of the round button that ends an update that is running.
store-upd-stop = Parar

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Parando

# Under a row whose update was stopped.
store-upd-stopped = Parado. Aperte Atualizar para continuar.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Parado. Aperte Instalar para continuar.

# Note under the Apps card.
store-upd-steam-restarts = A Steam reinicia quando nenhum jogo estiver rodando.

