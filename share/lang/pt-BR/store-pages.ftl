
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } aplicativo
        [many] { $count } de aplicativos
       *[other] { $count } aplicativos
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } atualização está esperando
        [many] { $count } de atualizações estão esperando
       *[other] { $count } atualizações estão esperando
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } para escolher no Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = tudo o que é seu está atualizado

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Nenhum aplicativo corresponde a “{ $text }”.

# Line under the title of an empty page of apps.
store-grid-none-here = Ainda não há aplicativos aqui.

# Advice in the empty page of a search.
store-grid-try-shorter = Tente uma palavra mais curta ou escolha uma categoria à esquerda.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Busque pelo nome acima ou escolha outra categoria à esquerda.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Mostrando { $shown } de { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = Mostrar mais { $next }

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Buscando a lista de aplicativos no Flathub. Da primeira vez, leva um minuto.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Aplicativos não conseguiu acessar o Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Este computador não está conectado à internet. Aplicativos tentará de novo quando estiver.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Os aplicativos para escolher vêm do Flathub, pela internet.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } e as configurações de aplicativos, à esquerda, funcionam sem ele.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } e as configurações de aplicativos, à esquerda, funcionam sem o Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } e as configurações de aplicativos, à esquerda, funcionam sem a internet.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Tentar de novo

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Resultados

# Red words under the search box when no web browser opens a link.
store-no-browser = Não há navegador para abrir { $url }. Instale um em Aplicativos primeiro.

# Said in the card or under the title while the lists are being read.
store-looking = Procurando…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Agora ele está em Instalados e no anel

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Removido

# The word on an app's dial when installing or removing did not work.
store-done-not = Falhou

# The word on an app's dial when it was installed.
store-done-installed = Instalado

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = O Flatpak falhou.

# Tab of the Installed page that shows everything.
store-tab-all = Todos

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Arquivos Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImages

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Arquivos deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Plugins

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Nada está instalado ainda. Você encontra aplicativos para experimentar em Descobrir.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 plugin está instalado
        [many] { $n } de plugins estão instalados
       *[other] { $n } plugins estão instalados
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 programa está instalado
        [many] { $n } de programas estão instalados
       *[other] { $n } programas estão instalados
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } itens estão instalados

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, do Flathub
        [many] { $start }, todos do Flathub
       *[other] { $start }, todos do Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, de um arquivo Flatpak
        [many] { $start }, todos de arquivos Flatpak
       *[other] { $start }, todos de arquivos Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, um AppImage
        [many] { $start }, todos AppImages
       *[other] { $start }, todos AppImages
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, de um arquivo .deb
        [many] { $start }, todos de arquivos .deb
       *[other] { $start }, todos de arquivos .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } do Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 de um arquivo Flatpak
        [many] { $n } de arquivos Flatpak
       *[other] { $n } de arquivos Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
        [many] { $n } de AppImages
       *[other] { $n } AppImages
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 aplicativo
        [many] { $n } de aplicativos
       *[other] { $n } aplicativos
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 de um arquivo .deb
        [many] { $n } de arquivos .deb
       *[other] { $n } de arquivos .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 plugin
        [many] { $n } de plugins
       *[other] { $n } plugins
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Está atualizado.

# End of the Installed line when everything is up to date.
store-installed-state-all = Todos estão atualizados.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 tem uma atualização esperando.
        [many] { $n } de itens têm atualizações esperando.
       *[other] { $n } têm atualizações esperando.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } de { $total } programas correspondem a “{ $find }”.

# The same when plugins are among the things.
store-installed-match-things = { $shown } de { $total } itens correspondem a “{ $find }”.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Adicionar um AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Buscar entre os instalados

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = Não foi possível adicionar o AppImage.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Tudo o que você instala aparece aqui: aplicativos do Descobrir ou do GitHub, AppImages, programas de arquivos .deb e plugins.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Nada instalado corresponde a “{ $find }”.

# On an installed thing's line: its version.
store-installed-version = versão { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = retido

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = retido; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Atualizando { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Atualizar { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = Sobre

# Under an installed thing's row while it is being taken off.
store-installed-removing = Removendo…

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = O Trindle não consegue abrir o seletor de arquivos agora.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Escolher um AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImages

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = O Trindle não consegue abrir o seletor de arquivos.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Adicionando…

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } já está instalado. Você o encontra em Instalados.

# Words once an AppImage is in the ring.
store-add-in-ring = Adicionado. Agora está na sua lista.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Essa não é uma referência Flatpak que este computador possa usar.

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } está instalado.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = O Flatpak não conseguiu instalá-lo.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = O Trindle não conseguiu instalar { $name }. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Substituindo…

# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } já está aqui

# Small heading over the AppImage being brought.
store-replace-new = O novo

# Small heading over the AppImage already there.
store-replace-old = O que já está aqui

# Tip of the round button that keeps the AppImage already there.
store-replace-skip = Pular e manter o antigo

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Substituir

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Não terminou. Nenhum motivo foi informado.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Mostrar onde está o problema

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Buscar aplicativos (pressione Enter para buscar também no GitHub)

# Heading of the card of matches from AppImageHub.
store-gh-hub = No AppImageHub ({ $n })

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = O Trindle não conseguiu pesquisar no GitHub, porque este computador não está conectado à internet.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Consultando o GitHub…

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Não há mais nada no GitHub.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] Procurando arquivos AppImage e Flatpak em { $n } projeto…
        [many] Procurando arquivos AppImage e Flatpak em { $n } de projetos…
       *[other] Procurando arquivos AppImage e Flatpak em { $n } projetos…
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Nenhum dos resultados no GitHub tem um arquivo AppImage ou Flatpak para este computador.

# Heading of the card of matches from GitHub.
store-gh-on = No GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Buscar no GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Aplicativos no GitHub, como arquivos AppImage ou Flatpak. Ninguém verifica estes, então instale apenas de pessoas em quem você confia.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = Não é possível buscar no GitHub enquanto este computador não está conectado à internet.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak e AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Ver os arquivos

# Button that installs a GitHub project or file.
store-gh-install = Instalar

# Link to a project's page on GitHub.
store-gh-open = Abrir no GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, versão { $tag }

# Name of the GitHub key's button and of its box.
store-gh-key = Chave do GitHub

# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = Uma chave do GitHub está salva

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Quase certamente você não precisa de uma. Sem chave, o GitHub permite 60 consultas por hora deste computador. Com uma, permite 5.000. A chave só aumenta esse limite. Ela serve para ver arquivos públicos e nada mais. Não dá ao Trindle nenhum acesso à sua conta, e você não marca nenhuma permissão ao criá-la. Ela fica salva neste computador, no seu chaveiro, e é enviada somente ao GitHub.

# In the GitHub key's box: how to make a key.
store-gh-key-how = Crie uma chave no GitHub, dê a ela qualquer nome e deixe todas as permissões como estão. Toque em Generate token e cole aqui o que aparecer.

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = Ela não foi removida: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = Ela não foi salva: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Deixe vazio para manter

# Placeholder of the GitHub key's box.
store-gh-key-paste = Cole a chave

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Remover a chave

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Criar uma chave no GitHub

# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = Salvar a chave

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Nada na versão mais nova dele pode ser instalado neste computador.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Adicionado

# Line under the title of the Move your apps page.
store-bk-line = Seus aplicativos e configurações, num só arquivo para levar para qualquer lugar.

# Heading of the card that makes a backup.
store-bk-card = Fazer backup

# Tip of the round button while the backup is being made.
store-bk-making-tip = Fazendo o backup

# Main button that makes the backup.
store-bk-make = Fazer o backup

# Tip of the round button that shows the backup in Folders.
store-bk-show = Mostrar no Folders

# Name of the switch for the list of apps in a backup.
store-bk-apps = Aplicativos

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] A lista do seu 1 aplicativo Flatpak.
        [many] A lista dos seus { $n } de aplicativos Flatpak.
       *[other] A lista dos seus { $n } aplicativos Flatpak.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list }

# Name of the switch for the apps' own data in a backup.
store-bk-data = Dados próprios dos aplicativos

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImages

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Configurações do Trindle

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Agora { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Onde ele fica

# Under that row: the folder and the file's name.
store-bk-where-sub = Sua pasta pessoal, como { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = Restaurar um backup

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Escolher um arquivo de backup

# Note in the card that puts a backup back.
store-bk-back-note = Restaurar instala os aplicativos dele e restaura os dados e as configurações. Seus documentos e imagens não estão no backup, então copie-os você mesmo.

# Said while a backup is being made.
store-bk-making = Fazendo o backup. Isso pode levar alguns minutos.

# Said when a backup is made: the file and its size.
store-bk-made = O backup está em { $path } ({ $size }). Copie-o para um pen drive ou para o armazenamento na nuvem para levá-lo com você.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Nenhum backup foi escolhido, porque o Trindle não consegue se conectar ao seletor de arquivos.

# Title of the window that asks which backup to use.
store-bk-choose-title = Escolher um backup

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Backups do Trindle e do Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = Nenhum backup foi escolhido, porque o seletor de arquivos não respondeu.

# Said while a backup file is read.
store-bk-reading = Lendo o backup…

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Saia da sessão e entre de novo para ver todas as configurações.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 aplicativo encontrado, { $with } com configurações. Começando…
        [many] { $apps } de aplicativos encontrados, { $with } com configurações. Começando…
       *[other] { $apps } aplicativos encontrados, { $with } com configurações. Começando…
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Nenhuma origem de software adicionada ainda.
        [one] 1 origem de software adicionada, mantida em dia com as atualizações do Debian.
        [many] { $n } de origens de software adicionadas, mantidas em dia com as atualizações do Debian.
       *[other] { $n } origens de software adicionadas, mantidas em dia com as atualizações do Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what } foi adicionada e é mantida em dia.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. A origem foi adicionada, mas o programa não está instalado.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Instalado de outra forma.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Remover o programa e a origem dele

# Link to a maker's page, naming the program.
store-src-open = Abrir { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Entrar no Tailscale

# Tip of the round button that adds a maker's source, naming the program.
store-src-add = Adicionar { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Prontas para adicionar

# Name of the field for the address of a source.
store-src-address = Endereço

# Name of the field for the suite of a source.
store-src-suite = Suíte

# Name of the field for the components of a source.
store-src-components = Componentes

# Name of the field for a source's signing key.
store-src-key = Chave de assinatura

# Name of the field for the program to install from a source.
store-src-program = Programa a instalar

# Placeholder of the program field.
store-src-program-hint = opcional

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = Adicionar esta origem

# Heading of the card that adds another source.
store-src-another = Adicionar outra origem

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Uma origem pode instalar atualizações para o computador inteiro, então adicione apenas fabricantes em que você confia. Cada uma pede a senha de um administrador.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Remover esta origem

# Heading of the card of sources already on the computer.
store-src-already = Já neste computador

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Os dois endereços precisam começar com https://

# Red words when the suite or components are not plain words.
store-src-bad-words = A suíte e os componentes precisam ser palavras simples.

# Red words when the program is not a package name.
store-src-bad-package = Esse não é um nome de pacote.

# Word on a source's dial while its job starts.
store-src-starting = Iniciando

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Abrindo a página de entrada do Tailscale…

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Entre na página que abriu. O Trindle continua sozinho assim que você entrar.

# Said under Tailscale's row when it is connected.
store-src-connected = Conectado

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = O link para os aplicativos não é criptografado.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = Não criptografado

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Esta conexão não é criptografada. Alguém poderia ver o que é enviado pelo caminho, ou trocar o backup por outro.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Eu entendo e aceito este risco

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Restaurar

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Cancelar

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = Remover { $name }?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Os dados salvos dele ocupam { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Ele não tem dados salvos.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Medindo os dados salvos dele…

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Dados apagados não podem ser recuperados.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Remover e manter os dados

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Remover e apagar os dados

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Apagar só os dados

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Ele não tem dados salvos para apagar.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Cancelar

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Os dados salvos dele foram apagados.

# Words while only the data is being deleted.
store-flatpak-data-going = Apagando os dados salvos dele…

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Não foi possível medir os dados salvos dele.

store-hub-warning = O AppImageHub apenas lista projetos. Ninguém os revisa nem os analisa, e eles não precisam ser assinados. O Trindle executa cada AppImage em uma caixa com a própria pasta pessoal privada. No começo ele pode usar a internet e o som, mas não vê nenhum dos seus arquivos.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = De onde um AppImage diz que vem é só uma referência. O Trindle não verifica isso.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = No GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = No GitHub: { $repo }, { $stars } estrelas

# Where an AppImage comes from: a website, by its name.
store-from-site = Fora do GitHub. Vem de { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Origem desconhecida

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Abrir o site dele

# When an app was last updated.
store-age-updated = Última atualização: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Última atualização: { $date }. Não é atualizado há mais de seis meses, então pode ter problemas de segurança sem correção.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Última atualização: desconhecida

# When an app's project was made.
store-age-created = Criado em { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Criado: desconhecido

# A date the source does not give, on a line of Facts.
store-age-not-known = Desconhecido

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Antes de adicionar um

# Title of the card that lists the AppImages.
store-browse-list = AppImages

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] { $count } AppImage está listado no AppImageHub.
        [many] { $count } AppImages estão listados no AppImageHub.
       *[other] { $count } AppImages estão listados no AppImageHub.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Buscando a lista do AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = O Trindle não conseguiu obter a lista do AppImageHub.

# The same with no internet and no copy kept.
store-browse-offline = A lista do AppImageHub não pode ser buscada enquanto este computador não está conectado à internet.

# In the list when the kind chosen has none.
store-browse-none = O AppImageHub não tem nenhum destes.

# Under the rows: how many are shown of how many.
store-browse-showing = Mostrando { $shown } de { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Adicionar aos apps da Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Ver detalhes e arquivos



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = a verificação de atualizações está desligada

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = O ChatGPT voltará quando a OpenAI assinar o download dele.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } para escolher no Flathub e no AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } para escolher no AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Ambos

# The same: only Flatpaks.
store-show-flatpaks = Flatpaks

# The same: only AppImages.
store-show-appimages = AppImages

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = Os AppImages não são verificados. Ninguém os revisa nem os analisa.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = O Trindle executa cada AppImage em uma caixa com a própria pasta pessoal privada. No começo ele pode usar a internet e o som, mas não vê nenhum dos seus arquivos.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Não mostrar novamente

# Button on that card that goes on with the install.
store-appimage-warn-ok = OK

# Button on that card that gives the install up.
store-appimage-warn-cancel = Cancelar

# Printed by the command line after a backup. { $size } is how big the file is.
store-cli-backed-up = backup feito ({ $size })

# Printed by the command line after Trindle's own settings were put back.
store-cli-settings-back = configurações do Trindle restauradas


# Tick box under the Search page's box: look on Flathub, for Flatpaks.
store-gh-place-flathub = Flathub (Flatpaks)

# Tick box: look on AppImageHub, for AppImages.
store-gh-place-hub = AppImageHub (AppImages)

# Tick box: look on GitHub, which needs Enter.
store-gh-place-github = GitHub
