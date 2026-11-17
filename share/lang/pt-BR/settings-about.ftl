# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Sobre este computador

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = nome do host nome do computador nome da rede sobre sistema versão debian trindle memória ram processador cpu disco gráficos tela modelo kernel atividade programas em execução ocupado gerenciador de tarefas monitor do sistema discos unidades usb espaço

# Text behind the info button of the About page.
settings-about-help = Esta página mostra o nome deste computador, como os outros computadores da rede o veem, do que ele é feito e quanto tempo leva para iniciar. Atividade mostra o que o computador está fazendo agora e quais programas usam a memória e o processador. Unidades mostra suas unidades e o espaço nelas. Cada uma abre em uma janela própria. Use as setas em Mais detalhes para abri-las.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Desconhecido

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } thread
        [many] { $model }, { $cores } threads
       *[other] { $model }, { $cores } threads
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Gráficos Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Gráficos AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Gráficos NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Gráficos de máquina virtual

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Gráficos

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } a { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } em uso

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } livres

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = de { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, ligado há { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, ligado há { $on }. O disco está quase cheio.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } segundos

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } minutos

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } minuto
        [many] { $n } minutos
       *[other] { $n } minutos
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } hora
        [many] { $n } horas
       *[other] { $n } horas
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } dia
        [many] { $n } dias
       *[other] { $n } dias
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Use só letras, números e hifens. Não comece nem termine com hífen.

# Words under the computer name box while the password window is open.
settings-about-name-waiting = Aguardando a sua senha…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Não foi possível renomear o computador.

# Words under the computer name box after renaming worked.
settings-about-name-done = Os programas que já estão abertos usarão o novo nome depois que você entrar de novo.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nome do computador

# Tip of the save button beside the computer name box.
settings-about-rename = Renomear este computador

# Bold line of the computer name setting.
settings-about-name = Nome do computador

# Heading of the card with the computer's name.
settings-about-name-group = Nome

# Text behind the info button of the Name card.
settings-about-name-about = Este é o nome do computador na rede e no prompt de um Terminal. Use letras, números e hifens, mas não comece nem termine com hífen. Você precisa digitar sua senha para mudá-lo.

# Heading of the card about the computer's hardware.
settings-about-hardware = O computador

# Text behind the info button of the hardware card.
settings-about-hardware-about = Mostra o fabricante e o modelo do computador, o processador e os gráficos. Mostra também quantas threads o processador executa ao mesmo tempo, e cada tela em uso com seu tamanho e taxa de atualização.

# Heading of the card about the software.
settings-about-software = O software

# Text behind the info button of the software card.
settings-about-software-about = O Trindle é uma área de trabalho que roda sobre o Debian, que cuida dos programas por baixo. O kernel é a parte que trabalha com o hardware. O compositor próprio do Trindle desenha a área de trabalho, usando o Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Compositor próprio do Trindle

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } aplicativo
        [many] { $n } aplicativos
       *[other] { $n } aplicativos
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Mais detalhes

# Text behind the info button of the More detail card.
settings-about-more-about = Atividade mostra o que o computador está fazendo agora e quais programas usam a memória e o processador. Unidades mostra todas as unidades, quão cheias e saudáveis estão, e os pendrives.

# Bold line of the setting that opens Activity.
settings-about-activity = Ver o que está em execução

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Atividade

# Bold line of the setting that opens Drives.
settings-about-drives = Unidades e espaço

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Unidades

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = O Trindle não conseguiu salvar os tempos de inicialização em Downloads.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Data e hora,Segundos

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Tempos de inicialização do Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Em Downloads, como { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Salvar

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Salvar todos os tempos de inicialização em Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Salvar o tempo de cada inicialização

# Quiet line under that setting.
settings-about-times-keep-sub = Salvas em Downloads como uma planilha.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Cada coluna é uma inicialização. A mais recente fica à direita.

# Heading of the card with the last ten start-up times.
settings-about-times = Últimas dez inicializações

# Text behind the info button of the start-up times card.
settings-about-times-about = O Trindle cronometra cada inicialização do botão de energia até a tela de entrada. O tempo gasto digitando a senha do disco não é contado. A mais recente fica no topo.

# Heading of the memory tile.
settings-about-memory = Memória

# Heading of the disk tile.
settings-about-disk = Disco

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Relatórios de problemas

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Os relatórios vão para o Buildmorphic, a comunidade que ajuda a construir o Trindle. Você pode entrar só com um e-mail. Ganha créditos e níveis pelos problemas que for o primeiro a relatar, mesmo que não saiba corrigi-los. Um relatório tem só uma lista fixa de informações, e nada é enviado até você apertar Enviar. Um nome de usuário ou nome do GitHub é opcional, e seu nome de usuário nunca é o seu e-mail.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Oferecer o envio de um relatório quando algo der errado

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Nome de usuário do Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = Nome no GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Sobre a comunidade

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Desligado. { $n } relatório em espera foi apagado.
       *[other] Desligado. { $n } relatórios em espera foram apagados.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Desligado. { $n } relatório em espera não pôde ser apagado.
       *[other] Desligado. { $n } relatórios em espera não puderam ser apagados.
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = Atualizações

# Text behind the info button of the updates card.
settings-about-updates-about = O Trindle procura novas versões de aplicativos, plugins, firmware e dele mesmo logo depois que você entra, e depois a cada poucas horas. Se você desligar isto, o Trindle só procura quando você aperta Verificar atualizações em Aplicativos, ou o botão de atualizações no anel. As correções de segurança do Debian são instaladas todos os dias e têm a própria chave. Se você desligar essa chave, elas deixam de se instalar sozinhas e as falhas de segurança conhecidas continuam sem correção até você atualizar. Mudar qualquer uma das duas chaves exige a senha do administrador, para que o computador inteiro use a sua escolha.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Verificar atualizações

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Aguardando a sua senha…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Atualizações de segurança

# Warning line under the Security updates switch when it is off.
settings-about-security-off = As correções de segurança não se instalam mais sozinhas.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Desligado, mas os relatórios em espera do complemento Reporting não foram apagados.
