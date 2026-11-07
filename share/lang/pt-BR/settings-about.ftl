# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Sobre este computador

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = nome do host nome do computador nome da rede sobre sistema versão debian trindle memória ram processador cpu disco gráficos tela modelo kernel atividade programas em execução ocupado gerenciador de tarefas monitor do sistema discos unidades usb espaço

# Text behind the info button of the About page.
settings-about-help = O nome deste computador, como os outros computadores da rede o veem, do que ele é feito e quanto tempo leva para iniciar. O que ele está fazendo agora, e quais programas usam a memória e o processador, fica em Atividade, e as unidades e o espaço nelas em Unidades, cada uma em sua própria janela: as setas em Mais detalhes as abrem.

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
settings-about-name-bad = Só letras, números e hifens, sem começar nem terminar com hífen.

# Words under the computer name box while the password window is open.
settings-about-name-waiting = Aguardando a senha...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Não foi possível renomear o computador.

# Words under the computer name box after renaming worked.
settings-about-name-done = Os programas já abertos veem o novo nome depois da próxima entrada.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nome do computador

# Tip of the save button beside the computer name box.
settings-about-rename = Renomear este computador

# Bold line of the computer name setting.
settings-about-name = Nome do computador

# Quiet line under the computer name setting.
settings-about-name-sub = Letras, números e hifens.

# Heading of the card with the computer's name.
settings-about-name-group = Nome

# Text behind the info button of the Name card.
settings-about-name-about = Como este computador aparece na rede e no prompt de um Terminal. Letras, números e hifens, sem começar nem terminar com hífen. Alterá-lo pede sua senha.

# Heading of the card about the computer's hardware.
settings-about-hardware = O computador

# Text behind the info button of the hardware card.
settings-about-hardware-about = O fabricante e o modelo, o processador e quantas threads ele executa ao mesmo tempo, os gráficos, e cada tela em uso com seu tamanho e taxa de atualização.

# Quiet line under the maker and model.
settings-about-hw-computer = O computador

# Quiet line under the processor.
settings-about-hw-processor = O processador

# Quiet line under the graphics.
settings-about-hw-graphics = Desenha as imagens

# Quiet line under the screens.
settings-about-hw-screen = A tela

# Heading of the card about the software.
settings-about-software = O software

# Text behind the info button of the software card.
settings-about-software-about = O Trindle é uma área de trabalho sobre o Debian, que cuida dos programas por baixo. O kernel é a parte que fala com o hardware, e o compositor próprio do Trindle desenha a área de trabalho, no Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Funciona por baixo

# Quiet line under the kernel version.
settings-about-sw-kernel = O kernel

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Compositor próprio do Trindle

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Desenha a área de trabalho, no Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } aplicativo
        [many] { $n } aplicativos
       *[other] { $n } aplicativos
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Instalados, prontos no anel

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Mais detalhes

# Text behind the info button of the More detail card.
settings-about-more-about = O que ele está fazendo agora, e quais programas usam a memória e o processador, fica em Atividade. Cada unidade, quão cheia e saudável ela está, e os pendrives, ficam em Unidades.

# Bold line of the setting that opens Activity.
settings-about-activity = Ver o que está em execução

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Programas que usam a memória e o processador.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Atividade

# Bold line of the setting that opens Drives.
settings-about-drives = Unidades e espaço

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Cada unidade, quão cheia e saudável ela está.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Unidades

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Não foi possível salvar os tempos de inicialização em Downloads.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Em Downloads, como { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Salvar

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Salvar todos os tempos de inicialização em Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Todas as inicializações, para guardar

# Quiet line under that setting.
settings-about-times-keep-sub = Salvas em Downloads como uma planilha.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Cada coluna é uma inicialização, a mais recente à direita.

# Heading of the card with the last ten start-up times.
settings-about-times = As últimas dez inicializações

# Text behind the info button of the start-up times card.
settings-about-times-about = Cada inicialização é cronometrada do botão de energia até a tela de entrada, sem contar o tempo gasto digitando a senha do disco. A mais recente fica no topo.

# Heading of the memory tile.
settings-about-memory = Memória

# Heading of the disk tile.
settings-about-disk = Disco


# Heading of the card with the switch for looking for updates.
settings-about-updates = Atualizações

# Text behind the info button of the updates card.
settings-about-updates-about = O Trindle procura novas versões de aplicativos, plugins, firmware e dele mesmo logo depois que você entra e a cada poucas horas, e as correções de segurança do Debian entram todos os dias. Desligado, ele só procura quando você aperta Verificar atualizações em Aplicativos, ou o botão de atualizações no anel. As correções de segurança do Debian têm a própria chave; desligada, elas deixam de se instalar sozinhas e as falhas conhecidas continuam abertas até você atualizar. Mudar qualquer uma das duas chaves pede a senha do administrador, para que os serviços do próprio computador também vejam a escolha.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Verificar atualizações

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Aguardando a senha...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Desligado, nada procura a menos que você peça.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Atualizações de segurança

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = As correções de segurança se instalam sozinhas todos os dias.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = As correções de segurança não se instalam mais sozinhas.

