# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Instalar { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Boas-vindas

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Disco

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Privacidade

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Senha

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Você

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Pronto

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Instalando

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Concluído

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Preparando

# Red line at the foot when the installer's helper could not be started.
installer-no-start = O instalador não conseguiu iniciar: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Não foi possível ler o plano.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Não foi possível ler o plano: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = A instalação parou. Nada mais foi gravado.

# Name of the round button at the foot that goes back a page.
installer-back = Voltar

# Name of the main button at the foot that goes to the next page.
installer-next = Avançar

# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Instalar

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Reiniciar agora

# Beside the keyboard test box on the first page.
installer-try-keyboard = Teste o teclado

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Digite aqui para testar o teclado

# Note beside Install after one press, when the second press writes beside another system.
installer-arm = Clique de novo para instalar

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Clique de novo para apagar e instalar

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Escolha um idioma

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Escolha um teclado

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Escolha um fuso horário

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Escolha onde o Trindle vai

# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = Digite a senha do disco duas vezes, igual nas duas

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = É preciso um nome de usuário

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Dê um nome a este computador

# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = Digite sua senha duas vezes, igual nas duas

# Title of the first page.
installer-welcome-title = Boas-vindas ao { $name }

# Sentence under the title of the first page.
installer-welcome-help = Escolha seu idioma; o teclado e o horário o acompanham, e você pode mudar qualquer um.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Idioma

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Teclado

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Horário

# Under a card's head while nothing has been chosen in its list.
installer-select = Selecione

# Title of the disk page.
installer-disks-title = Onde o { $name } vai

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Tudo no disco que você escolher é substituído, a menos que você abra espaço ao lado do que já está lá.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Nenhum disco grande o bastante foi encontrado. O Trindle precisa de 20 GB.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Usar o disco inteiro, substituindo tudo nele

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Manter { $what } e compartilhar o disco

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } ficam para o que existe agora, { $mine } para o Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Espaço vazio

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } e { $last }

# Under a disk's name: what is on it.
installer-disk-has = Contém { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Contém { $list }, criptografado

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Contém um sistema criptografado

# Under a disk's name: there is nothing on it.
installer-disk-empty = Vazio

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Contém arquivos, mas nenhum sistema que o Trindle reconheça

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Um arquivo de swap de { $gb } GB

# Name of the switch that makes a swap file.
installer-swap-switch = Criar um arquivo de swap

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = O Secure Boot está ligado, então este computador não pode hibernar, mas o arquivo de swap ainda ajuda quando a memória acaba. Ele só é criado se o disco tiver bastante espaço sobrando.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Espaço para hibernar, mantendo tudo aberto com o computador desligado, e um recurso para quando a memória acabar. Sem ele, o computador não pode hibernar. Ele só é criado se o disco tiver bastante espaço sobrando.

# Title of the page about encrypting the disk.
installer-privacy-title = Criptografar o disco

# Sentence under the title of that page.
installer-privacy-help = Um disco criptografado não pode ser lido por quem o tirar deste computador.

# Head of the card with the three choices.
installer-privacy-locked = Como o disco é trancado

# Name and words of the choice not to encrypt the disk.
installer-privacy-none = Não criptografar o disco

# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Criptografar e pedir a senha quando o computador iniciar

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Criptografar e deixar o chip TPM deste computador destrancá-lo

# Head of the card about the TPM chip.
installer-privacy-about = Sobre o chip

# In the card about the chip, on a computer that has one.
installer-privacy-chip = Este computador tem um chip TPM. A primeira inicialização após a instalação pede a senha uma vez, enquanto o chip aprende este computador; depois disso ele destranca o disco sozinho e nada é digitado. Guarde a senha: ela é a forma de entrar se o chip for trocado ou o disco for movido para outro computador.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Este computador não tem um chip TPM que o Trindle possa usar, então a senha será pedida toda vez que ele iniciar. Em um computador que tenha um, o chip pode destrancar o disco sozinho.

# Title of the disk password page.
installer-disk-title = A senha do disco

# Sentence under the title of the disk password page.
installer-disk-help = Ela é digitada antes de qualquer outra coisa quando o computador inicia.

# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Senha do disco

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Sugerir uma frase secreta

# Label and name of the second box of a password, where it is typed again.
installer-same-again = A mesma de novo

# Red line under the password boxes when the two do not match.
installer-mismatch = As duas senhas não correspondem.

# Head of the card that tells the person to write the password down.
# SAFETY: needs a human review before it ships
installer-disk-write-down = Anote-a

# Words in that card.
# SAFETY: needs a human review before it ships
installer-disk-write-down-text = Anote esta senha no papel e guarde-a em um lugar seguro, longe do computador. Sem ela, o disco não pode ser aberto, nem por você nem por mais ninguém, nunca. Não há como entrar de novo. Quatro palavras comuns formam uma frase secreta fácil de digitar e difícil de adivinhar.

# Title of the account page.
installer-you-title = Sobre você

# Sentence under the title of the account page.
installer-you-help = Esta é a conta com a qual você entra.

# Head of the card with the name boxes.
installer-account = Sua conta

# Label and name of the box for the person's name.
installer-your-name = Seu nome

# Label and name of the box for the sign-in name.
installer-user-name = Nome de usuário

# Label and name of the box for the computer's name.
installer-host-name = Nome deste computador

# Head of the card with the password boxes.
installer-your-password = Sua senha

# Label and name of the first password box.
installer-password = Senha

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Quão difícil é de adivinhar

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = É preciso um nome de usuário.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Esse nome é longo demais.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Comece com uma letra minúscula.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Só letras minúsculas, números, - e _.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Esse nome já é usado pelo sistema.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Dê um nome a este computador.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Só letras, números e -.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Um computador adivinharia isto na hora.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Um computador adivinharia isto em cerca de { $n } minuto.
        [many] Um computador adivinharia isto em cerca de { $n } minutos.
       *[other] Um computador adivinharia isto em cerca de { $n } minutos.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Um computador adivinharia isto em cerca de { $n } hora.
        [many] Um computador adivinharia isto em cerca de { $n } horas.
       *[other] Um computador adivinharia isto em cerca de { $n } horas.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Um computador adivinharia isto em cerca de { $n } dia.
        [many] Um computador adivinharia isto em cerca de { $n } dias.
       *[other] Um computador adivinharia isto em cerca de { $n } dias.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Um computador precisaria de cerca de { $n } ano para adivinhar isto.
        [many] Um computador precisaria de cerca de { $n } anos para adivinhar isto.
       *[other] Um computador precisaria de cerca de { $n } anos para adivinhar isto.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Um computador precisaria de mais tempo do que a história registrada para adivinhar isto.

# Title of the Ready page.
installer-ready-title = Pronto para instalar

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Nada foi alterado ainda

# Head of the card about security updates.
installer-ready-security = Atualizações de segurança

# In the card about security updates.
installer-ready-security-text = O { $name } instala as correções de segurança do Debian sozinho todo dia. Isso não pode ser desativado.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = Tudo em { $disk } será substituído.

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Tudo em { $part } é mantido, mas faça um backup antes: reduzi-la pode perder arquivos se a energia acabar.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Nenhum disco foi escolhido ainda.

# Name of a row in the list: the whole disk will be used.
# SAFETY: needs a human review before it ships
installer-ready-whole = Todo o { $disk }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Tudo nele é substituído pelo { $name }.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Ao lado de { $part }

# Words of that row.
installer-ready-beside-text = { $part } é reduzida, até { $size }, e o { $name } vai no espaço liberado.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Nenhum disco

# Words of that row.
installer-ready-no-disk-text = Nenhum disco escolhido.

# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Não criptografado

# Words of that row.
installer-ready-plain-text = Quem tirar o disco pode lê-lo.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Criptografado

# Words of that row when a password is typed at each start.
installer-ready-locked-password = Você digita a senha dele toda vez que o computador inicia.

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = A senha uma vez, e depois o chip TPM deste computador o destranca.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Teclado { $keyboard }, horário { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Instalando o { $name }

# Head of the card that holds the short tour.
installer-tour = Um tour rápido

# Title of the last page.
installer-done-title = O { $name } está instalado

# Sentence under the title of the last page.
installer-done-help = Retire o pendrive e reinicie.

# Head of the card that repeats the warning about the disk password.
installer-done-keep = Guarde bem a senha do disco

# In that card when the TPM chip unlocks the disk.
installer-keep-chip = O disco está criptografado. A primeira inicialização pede a senha do disco uma vez, enquanto o chip aprende este computador; depois disso ele abre sozinho. Anote essa senha no papel e guarde-a em um lugar seguro, longe deste computador. Sem ela, o disco não pode ser aberto, nem por você nem por mais ninguém, nunca.

# In that card when the password is typed at each start.
installer-keep-password = O disco está criptografado. Anote agora a senha do disco no papel, se ainda não o fez, e guarde-a em um lugar seguro, longe deste computador. Sem ela, o disco não pode ser aberto, nem por você nem por mais ninguém, nunca.

# Title of the first slide.
installer-slide-ring = Tudo em um círculo

# Words of the first slide.
installer-slide-ring-text = Mova o ponteiro até a borda da tela e dê um clique duplo, e o anel abre: seus programas ao redor, a hora e as chaves no meio.

# Title of the second slide.
installer-slide-snapshots = Desfaça o que uma atualização estragou

# Words of the second slide.
installer-slide-snapshots-text = Um instantâneo do sistema é feito antes e depois de cada mudança de software. As Configurações restauram o sistema inteiro para qualquer um deles, e o menu de inicialização pode fazer isso quando o computador não inicia.

# Title of the third slide.
installer-slide-apps = Aplicativos que não atrapalham

# Words of the third slide.
installer-slide-apps-text = Aplicativos do Flathub, os próprios do Debian, AppImages e fontes de software em um só lugar, com backups que você pode levar para outro computador.

# Title of the fourth slide.
installer-slide-yours = Do seu jeito, rapidamente

# Words of the fourth slide.
installer-slide-yours-text = Claro ou escuro, seus próprios sons e uma área de transferência que lembra o que você copiou.

# Title of the fifth slide.
installer-slide-small = Pequeno e silencioso

# Words of the fifth slide.
installer-slide-small-text = A área de trabalho usa pouca memória e inicia em poucos segundos, então um laptop mais antigo parece novo de novo.

# Title of the tour slide about the ring.
installer-tour-ring = O anel é toda a área de trabalho

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Ponha o ponteiro neste canto e o anel abre

# Note on the ring picture, at the time.
installer-tour-ring-time = A hora e a data, no meio

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Seus programas ficam ao redor

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Som, tela, todos os seus aplicativos, manter ativo e desligar

# Title of the tour slide about Settings.
installer-tour-settings = Configurações, em palavras simples

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Digite aqui para achar uma configuração sem procurar

# Note on the Settings picture, at Network.
installer-tour-settings-network = Em Rede você entra em uma rede Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Claro ou escuro, quando quiser

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Vinte cores, ou digite a sua

# Title of the tour slide about Apps.
installer-tour-apps = Aplicativos, para todo o resto

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Procure um programa pelo nome ou pelo que ele faz

# Note on the Apps picture, at the list.
installer-tour-apps-many = Milhares de programas, gratuitos, do Flathub e do Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Atualizações para tudo, inclusive o próprio Trindle

# Title of the tour slide about voice control.
installer-tour-voice = Você pode falar com ele

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = O controle por voz fica aqui: Configurações, depois Anel

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Pressione Adicionar voz uma vez. Ocupa cerca de 150 MB e leva alguns minutos

# Note on the voice picture, about what to say.
installer-tour-voice-say = Depois toque no meio do anel e diga o nome de um aplicativo, “mais alto” ou “configurações de Wi-Fi”

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = O que você diz é entendido neste computador e não vai para nenhum outro lugar

# Title of the tour slide about snapshots.
installer-tour-snapshots = Um caminho de volta, antes de qualquer mudança

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = O computador faz um instantâneo antes de cada atualização

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Se algo quebrar, volte a como era, pelo menu de inicialização

# Name of the picture of the ring, and the head of its card.
installer-live-caption = O anel, onde tudo mora

# Sentence under the title of the welcome from a stick.
installer-live-help = Você está executando a partir do pendrive, então nada neste computador foi alterado.

# Head of the card with the three first things to do.
installer-live-begin = Para começar

# Name of the first thing to do.
installer-live-open = Abra o anel

# Line under it.
installer-live-open-line = Dê um clique duplo na borda da tela, ou pressione Super.

# Name of the second thing to do.
installer-live-speak = Fale com ele

# Line under it when voice control is there.
installer-live-speak-line = Toque no meio do anel e diga o que você quer.

# Line under it when voice control has to be added.
installer-live-speak-add = Adicione a voz em Configurações, Anel, depois toque no meio e fale.

# Name of the third thing to do.
installer-live-tab = Tudo no anel

# Line under it.
installer-live-tab-line = O que está aberto, suas notificações e a área de transferência estão todos ali.

# The choice of the dark look.
installer-live-dark = Escuro

# The choice of the light look.
installer-live-light = Claro

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Claro ou escuro, para toda a área de trabalho

# Name of the round button that closes this window and opens the ring.
installer-live-around = Dar uma olhada primeiro

# Name of the main button that opens the installer.
installer-live-install = Instalar o { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Abrindo espaço

# Foot of the install page while the disk is prepared.
installer-step-prepare = Preparando o disco

# Foot of the install page while a place is made beside another system.
installer-step-place = Criando um lugar para o Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Verificando o que já está no disco

# Foot of the install page while another system is made smaller.
installer-step-smaller = Reduzindo o outro sistema

# Foot of the install page while the disk is encrypted.
installer-step-lock = Trancando o disco

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Configurando o chip do computador

# Foot of the install page while the system is copied.
installer-step-copy = Copiando o Trindle para o disco

# Foot of the install page while settings are written.
installer-step-settings = Gravando as configurações

# Foot of the install page while the person's account is made.
installer-step-account = Criando sua conta

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Retirando as partes do pendrive

# Foot of the install page while the start-up files are made.
installer-step-startup = Criando os arquivos de inicialização

# Foot of the install page while the boot menu is set up.
installer-step-boot = Configurando o menu de inicialização

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Configurando os instantâneos

# Foot of the install page at the very end.
installer-step-finishing = Finalizando

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = O { $program } não conseguiu iniciar: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = O { $program } não terminou. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = Não foi possível gravar { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } não é um disco.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } não é um disco neste computador.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Nenhum disco foi escolhido.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Esse nome de usuário não pode ser usado pelo Trindle.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Esse nome de computador não pode ser usado pelo Trindle.

# Red line at the foot: the language cannot be used.
installer-bad-language = O Trindle não pode usar esse idioma.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = O Trindle não pode usar esse teclado.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = O Trindle não pode usar esse fuso horário.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Esse nome contém algo que o Trindle não consegue gravar.

# Red line at the foot: a password has a line break in it.
installer-bad-password = Uma senha não pode conter uma quebra de linha.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = A segunda metade do instalador precisa ser executada como root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Este disco não tem uma partição de inicialização para compartilhar, então o Trindle não pode ir ao lado do que já está aqui.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Não foi possível encontrar a nova parte do disco.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = Não foi possível ler o tamanho de { $part }.

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = O Trindle não consegue reduzir uma partição { $fs }.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = O Trindle não consegue reduzir uma partição desconhecida.

# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = É preciso uma senha para criptografar o disco.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Não foi possível copiar o sistema: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Não foi possível copiar o sistema. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Não há espaço para um arquivo de swap.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Não foi possível ler o lugar do arquivo de swap no disco.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Não foi possível identificar as novas partições.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Não foi possível definir o fuso horário: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Não foi possível instalar o menu de inicialização neste computador.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Tem certeza?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = O que está aberto, a área de transferência e um botão que limpa a tela

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } é o pendrive de onde o Trindle está rodando.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } está em uso. Feche o que o estiver usando e tente de novo.

# Said beside how far the install has got, once it has started.
installer-started = Instalação iniciada

# The button that ends an install under way.
installer-halt = Parar

# Beside Stop after its first press.
installer-halt-sure = Parar a instalação? O conteúdo antigo do disco já foi apagado.

# While an install is being stopped.
installer-halting = Parando

# When Stop could not let go of the disk.
installer-halt-stuck = Não foi possível liberar o disco. Reinicie o computador para tentar de novo.

# The copy is done and the person is still answering.
installer-step-waiting = Copiado. Aguardando as suas respostas.



installer-already-running = Outro instalador já está em execução neste computador.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = O Trindle precisa de pelo menos 1 GB de memória, e este computador tem menos.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Um disco só pode ser criptografado em um computador que inicia por UEFI, e este não inicia.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = O Windows em { $part } foi hibernado ou deixado na inicialização rápida, então reduzi-la pode fazer com que perca seus arquivos. Inicie o Windows, escolha Reiniciar e tente de novo.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = O Trindle está instalado, mas não foi possível configurar os instantâneos: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = O Trindle está instalado, mas não foi possível fazer o menu de inicialização listar os instantâneos: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Nem tudo foi configurado

