
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Boas-vindas ao Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Abra o anel

# Title of the third page, where the first apps are picked.
welcome-page-apps = Escolha seus aplicativos

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Mais opções

# Title of the fifth page, about plugins.
welcome-page-plugins = Plugins acrescentam mais

# Title of the sixth page, about crash reports.
welcome-page-reports = Ajude a corrigir o Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Os pontos

# Title of the last page, with a few things worth knowing.
welcome-page-know = Bom saber

# Sentence under the title of the first page.
welcome-sentence-hello = Tudo o que você precisa fica em um só círculo, o anel.

# Sentence under the title of the second page.
welcome-sentence-ring = Dê um clique duplo na borda da tela ou pressione a tecla Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Quase nada está instalado ainda, então escolha os aplicativos que quiser.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Escolha mais e instale tudo o que escolheu com um clique.

# Sentence under the title of the fifth page.
welcome-sentence-plugins = Plugins são pequenos extras de Aplicativos, e cada um faz só o que você permitir.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Envie um relatório quando algo der errado, se quiser.

# Sentence under the title of the dots page.
welcome-sentence-dots = Um ponto significa que um programa está usando algo seu.

# Sentence under the title of the last page.
welcome-sentence-know = Algumas coisas que tornam o Trindle diferente.

# Top right of every page: where the person is in the tour.
welcome-step = Passo { $n } de { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Passo { $n } de { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = As boas-vindas já foram vistas. Elas estão em Todos os aplicativos como Boas-vindas; apague este arquivo para vê-las de novo no próximo login.

# Name of the round button at the foot that closes the tour.
welcome-close = Fechar

# Name of the round button at the foot that goes to the page before.
welcome-back = Voltar

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Avançar

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Instalar os aplicativos escolhidos

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Começar a usar o Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Conecte-se à internet para instalar aplicativos.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Tentar de novo

# Foot while apps are going in: their names.
welcome-installing = Instalando { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Instalando { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } já está no anel.
        [many] { $names } já estão no anel.
       *[other] { $names } já estão no anel.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Não foi possível instalar os aplicativos. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = O Flatpak recusou.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Escolhidos: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } e mais { $more }

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = O anel

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = O que há nele

# Name of the first thing the ring holds.
welcome-held-programs = Seus programas

# Line under it.
welcome-held-programs-line = Ficam ao redor, na parte externa.

# Name of the second thing the ring holds.
welcome-held-time = A hora e a data

# Line under it.
welcome-held-time-line = Ficam no meio.

# Name of the third thing the ring holds.
welcome-held-switches = Os botões

# Line under it.
welcome-held-switches-line = Som, tela, manter ativo e energia.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Com o ponteiro

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Com o teclado

# Note under the picture of the Super key.
welcome-ring-super-note = A tecla Super costuma ter o logotipo do Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Navegador

# Head of a card of apps: email programs.
welcome-kind-email = E-mail

# Head of a card of apps: chat programs.
welcome-kind-chat = Bate-papo

# Head of a card of apps: office programs.
welcome-kind-office = Escritório

# Head of a card of apps: music and video players.
welcome-kind-media = Música e vídeo

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Imagens e desenho

# Head of a card of apps: notes and writing.
welcome-kind-notes = Notas e escrita

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF e documentos

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Nenhum deles está no Flathub no momento.

# Beside an app that has been installed.
welcome-app-installed = Instalado

# Beside an app that is going in now.
welcome-app-installing = Instalando

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, escolhido

# Name of the round button on an app's row that picks it.
welcome-app-pick = Escolher { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = No anel

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Os plugins

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = Em Aplicativos, em Plugins.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Abrir plugins em Aplicativos

# Said in the plugins card while there are none to show.
welcome-plugins-none = Os plugins disponíveis estão listados em Aplicativos.

# Head of the card on the crash reports page.
welcome-reports-card = Relatórios de falhas

# The one question about crash reports.
welcome-reports-question = Ajudar a corrigir o Trindle enviando um relatório quando algo der errado? Você sempre o vê antes.

# Note under the question about crash reports.
welcome-reports-note = Nada é enviado sem o seu clique. Você vê o relatório inteiro antes, e ele não contém arquivos, senhas nem nomes de redes Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = Você pode mudar isso a qualquer momento em Configurações, Relatórios de falhas.

# The answer that turns crash reports off.
welcome-reports-no = Não

# The answer that turns crash reports on.
welcome-reports-yes = Sim, perguntar

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Enviar um relatório quando algo der errado

# Head of a card on the last page.
welcome-know-boxes = Os aplicativos vivem em caixas próprias

# Line in that card.
welcome-know-boxes-line = Um aplicativo pergunta antes de acessar seus arquivos, câmera ou microfone.

# Head of a card on the last page.
welcome-know-updates = As atualizações de segurança se instalam sozinhas

# Line in that card.
welcome-know-updates-line = Todos os dias, sem perguntar. As outras atualizações esperam por você.

# Head of a card on the last page.
welcome-know-ring = Configurações e Aplicativos estão no anel

# Line in that card.
welcome-know-ring-line = No meio dele, acima da hora.

# Head of a card on the last page.
welcome-know-help = A ajuda está em Configurações

# Line in that card.
welcome-know-help-line = Este tour está em Todos os aplicativos como Boas-vindas, para quando quiser vê-lo de novo.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Uma tela pequena sem nada além do anel no meio: a hora no centro e os aplicativos favoritos ao redor.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = O anel aberto em uma tela pequena, com os ícones de alguns plugins em fila no meio, abaixo da hora, onde fica o ícone de um plugin depois de instalado.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Dois pequenos desenhos de onde os pontos aparecem: na aba de uma janela no anel e no centro superior da tela.

welcome-dots-where-tab = Na aba de uma janela

welcome-dots-where-corner = Centro superior da tela

welcome-picture-ring = Uma tela pequena. O ponteiro vai até a borda superior e dá um clique duplo, e o anel se abre no meio.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = A tecla Super sendo pressionada, o que abre o anel.



# Title of the first page, which asks how much the person knows.
welcome-page-level = Quanto você conhece?

# Sentence under that title.
welcome-sentence-level = Escolha o mais próximo, e o tour pula o que você já sabe.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Novo no Linux

# Line in that answer's card.
welcome-level-new-line = Todas as páginas, inclusive o que qualquer desktop Linux faz.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Já usei Linux

# Line in that answer's card.
welcome-level-linux-line = Só o que é próprio do Trindle. O básico do Linux é pulado.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Já usei o Trindle

# Line in that answer's card.
welcome-level-trindle-line = Fecha o tour agora e não o mostra de novo.

# Title of the page about files and folders.
welcome-page-files = Seus arquivos

# Sentence under that title.
welcome-sentence-files = Tudo o que é seu fica na sua pasta pessoal.

# Title of the page about getting software.
welcome-page-software = Como obter programas

# Sentence under that title.
welcome-sentence-software = Os aplicativos vêm de Aplicativos, não de arquivos de instalação baixados.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = A senha do administrador

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Mudanças no computador inteiro pedem uma senha antes.

# Title of the page about snapshots.
welcome-page-snapshots = Instantâneos

# Sentence under that title.
welcome-sentence-snapshots = O Trindle guarda cópias do sistema, para que uma atualização possa ser desfeita.

# Title of the page about the person's own words.
welcome-page-words = Suas próprias palavras

# Sentence under that title.
welcome-sentence-words = Mude qualquer palavra que o Trindle mostra, ou adicione um idioma seu.

# Head of a card on the files page.
welcome-files-home = Uma pasta pessoal

# Line in that card.
welcome-files-home-line = Documentos, Downloads e Imagens ficam dentro dela. As Pastas abrem ali.

# Head of a card on the files page.
welcome-files-drives = Sem letras de unidade

# Line in that card.
welcome-files-drives-line = Um pen drive USB ou outro disco aparece em Pastas quando você o conecta.

# Head of a card on the files page.
welcome-files-hidden = Nomes ocultos

# Line in that card.
welcome-files-hidden-line = Um nome que começa com ponto, como .config, fica oculto. Ele guarda as configurações de um aplicativo.

# Head of a card on the files page.
welcome-files-bin = A lixeira

# Line in that card.
welcome-files-bin-line = Um arquivo excluído espera na lixeira até você esvaziá-la.

# Head of a card on the software page.
welcome-software-apps = Um só lugar para aplicativos

# Line in that card.
welcome-software-apps-line = Abra Aplicativos no anel para encontrar, instalar e remover programas.

# Head of a card on the software page.
welcome-software-exe = Sem arquivos de instalação

# Line in that card.
welcome-software-exe-line = Um arquivo de instalação do Windows, terminado em .exe, não roda aqui. Procure o aplicativo em Aplicativos.

# Head of a card on the software page.
welcome-software-updates = Atualizações em um só lugar

# Line in that card.
welcome-software-updates-line = Aplicativos e Configurações mantêm tudo em dia, então nenhum aplicativo precisa de atualizador próprio.

# Head of a card on the software page.
welcome-software-remove = Remover é limpo

# Line in that card.
welcome-software-remove-line = Remover um aplicativo tira tudo dele e não deixa nada para trás.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Por que ela pergunta

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Instalar para todos ou mexer em um disco afeta todos os usuários, então o computador pergunta antes.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = De quem é a senha

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = A de um administrador, normalmente a que você usa para entrar.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Quando ela não pergunta

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Aplicativos que você instala só para você e seus próprios arquivos nunca precisam dela.

# Head of a card on the snapshots page.
welcome-snap-taken = Feito para você

# Line in that card.
welcome-snap-taken-line = Em um sistema Btrfs, um é feito antes e depois de cada mudança de programas.

# Head of a card on the snapshots page.
welcome-snap-list = Listado em Configurações

# Line in that card.
welcome-snap-list-line = Configurações, Instantâneos os lista. A câmera faz um agora.

# Head of a card on the snapshots page.
welcome-snap-boot = Usado se o sistema não iniciar

# Line in that card.
welcome-snap-boot-line = Escolha Trindle snapshots no menu de inicialização e depois um instantâneo.

# Head of a card on the your own words page.
welcome-words-change = Mudar uma palavra

# Line in that card.
welcome-words-change-line = Encontre qualquer palavra que o Trindle mostra e digite a sua. "Abrir, pronto" pode substituir "Clique aqui para abrir o arquivo".

# Head of a card on the your own words page.
# SAFETY: needs a human review before it ships
welcome-words-share = Compartilhar uma folha

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-words-share-line = Exporte suas palavras como um arquivo, ou importe um que um amigo enviou. Ele é verificado antes de ser usado.

# Head of a card on the your own words page.
welcome-words-all = Todos os programas de uma vez

# Line in that card.
welcome-words-all-line = Todo o Trindle lê suas palavras de um só lugar, então uma mudança aparece em tudo.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Fica em Configurações, Região e idioma. A Ajuda explica passo a passo.

# Name of the round button that opens that Settings page.
welcome-words-open = Abrir Região e idioma

