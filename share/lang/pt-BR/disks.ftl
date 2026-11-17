
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Unidades

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Sobre esta página

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Aqui aparecem todas as unidades dentro do computador ou ligadas a ele, em uma barra dividida em partições. A largura de cada partição mostra o tamanho dela, e o sombreado mostra o quanto ela está cheia, então uma partição cheia é fácil de ver.

# Second paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-2 = Sob a imagem, cada unidade tem um cartão. Você pode abrir uma partição, montá-la ou desmontá-la, remover uma unidade com segurança ou desligá-la. O lápis em uma partição permite renomeá-la, escolher a pasta dela, montá-la ao iniciar, redimensioná-la, verificá-la, repará-la, formatá-la ou excluí-la. O sinal de mais no espaço livre cria uma nova partição.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = Você vê a unidade de onde o sistema roda, mas não pode fazer nela nenhuma alteração que apague ou mova coisas. Toda alteração em qualquer unidade pede a senha de um administrador, a cada vez. O olho esconde uma unidade ou uma partição do Folders e do anel. Ela continua aqui, em cinza, e não fica bloqueada.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Suas unidades

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Não consigo ler as unidades agora.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Tentar de novo

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Olhar de novo

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Nenhuma unidade foi encontrada. Ligue uma e ela vai aparecer aqui.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = O serviço de unidades

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = O barramento do sistema

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } não está disponível. Para instalar, execute: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Olhando as unidades…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } está falhando. Copie agora para outra unidade os arquivos que você quiser guardar.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } precisa de atenção: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] As suas unidades e o espaço nelas.
        [one] As suas unidades e o espaço nelas, com uma unidade ligada.
       *[other] As suas unidades e o espaço nelas, com { $plugged } unidades ligadas.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Este computador está quase cheio, com só { $free } livres. Exclua ou mova alguns arquivos em breve.
        [one] Este computador está quase cheio, com só { $free } livres. Há uma unidade ligada. Exclua ou mova alguns arquivos em breve.
       *[other] Este computador está quase cheio, com só { $free } livres. Há { $plugged } unidades ligadas. Exclua ou mova alguns arquivos em breve.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Este computador está ficando cheio, com { $free } livres.
        [one] Este computador está ficando cheio, com { $free } livres. Há uma unidade ligada.
       *[other] Este computador está ficando cheio, com { $free } livres. Há { $plugged } unidades ligadas.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Há muito espaço neste computador, com { $free } livres.
        [one] Há muito espaço neste computador, com { $free } livres. Há uma unidade ligada.
       *[other] Há muito espaço neste computador, com { $free } livres. Há { $plugged } unidades ligadas.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Este computador

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Seus arquivos

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } ainda livres no disco em que o sistema está.
       *[files] { $free } ainda livres, para os seus próprios arquivos.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } cheio, { $free } livres de { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = de { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = As unidades ainda não enviaram nenhuma leitura de saúde

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Saudável, pela verificação da unidade

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] Saudável, segundo a verificação da própria unidade
       *[other] Todas as { $n } unidades passaram nas próprias verificações
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } precisa de atenção: veja o cartão dela
       *[other] { $n } precisam de atenção: veja os cartões delas
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 unidade, { $total } no total. { $health }.
       *[other] { $n } unidades, { $total } no total. { $health }.
        [many] { $n } unidades, { $total } no total. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } livres

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Não montada

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Espaço livre

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Nenhum disco ou cartão nela

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Ainda não há nada nela

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB ou removível

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, dentro deste computador

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB ou removível, oculta

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, dentro deste computador, oculta

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } cheia, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Partição

# Name given to a drive that gives no maker or model.
disks-drive-name = Unidade

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Swap

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Criptografada, desbloqueada

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Criptografada, bloqueada

# Kind of a part that holds other parts.
disks-kind-container = Contém partições

# Kind of a part that has no filesystem on it.
disks-kind-none = Sem sistema de arquivos

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, a partição de onde um computador inicia
        [legacy] { $kind }, a partição de onde um computador mais antigo inicia
       *[loader] { $kind }, a partição própria de um gerenciador de inicialização
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = qualquer computador

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = só Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = dispositivos antigos

# Trouble a drive reports about its spare space.
disks-trouble-spare = ficando sem espaço de reserva

# Trouble a drive reports about its heat.
disks-trouble-temperature = quente demais

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = se desgastando

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = passou a somente leitura

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = a cópia de segurança da memória falhou

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = parte dela passou a somente leitura

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = desgastada além da vida útil esperada

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } ponto danificado
       *[other] { $n } pontos danificados
        [many] { $n } pontos danificados
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] usada por { $hours } hora
       *[other] usada por { $hours } horas
        [many] usada por { $hours } horas
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Falhando. Copie agora tudo o que quiser guardar dela

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Precisa de atenção: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Saudável

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Não responde

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = A caixa da unidade está conectada, mas a unidade dentro dela não responde. Desconecte, espere dez segundos e ligue em uma porta azul ou USB-C usando o cabo dela. Se continuar assim, a unidade dentro pode ter se soltado ou falhado.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Remover com segurança

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Desligar

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Apagar a unidade inteira

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Mostrar esta unidade
       *[partition] Mostrar esta partição
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Esconder esta unidade. { $lock }
       *[partition] Esconder esta partição. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Escondida do Folders e do anel. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = O sistema roda a partir dela, então nada aqui pode ser apagado.

# Note on the card of a drive that has no partition table.
disks-no-table = Esta unidade não tem tabela de partições, então ela inteira está livre.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = não montada

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = monta ao iniciar em { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = aparece em { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = em { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Abrir

# Tip of the round button that mounts a part.
disks-mount = Montar

# Tip of the round button that unmounts a part.
disks-unmount = Desmontar

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Fazer alterações

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Nome

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Nome (opcional)

# Tip of the button that saves a part's new name.
disks-rename-tip = Renomear

# Note under the Name row of a part.
disks-name-note = Como se chama aqui e em outros computadores.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Nome da pasta

# Tip of the button that saves a part's folder.
disks-use-folder = Usar esta pasta

# Label of the row where a part's folder is chosen.
disks-folder = Pasta

# Note under the Folder row of a part.
disks-folder-note = Onde os arquivos aparecem. Deixe vazio para o computador escolher.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Montar ao iniciar

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Monta a partição sempre que o computador inicia. Se ela não estiver lá, o computador inicia mesmo assim.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Tamanho em GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = De { $least } a { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = De { $least } a { $most }. Monte para ver o quanto pode diminuir.

# Tip of the button that resizes a part.
# SAFETY: needs a human review before it ships
disks-resize-tip = Redimensionar

# Label of the row where a part is resized.
disks-size = Tamanho

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Isto vai apagar tudo em { $words }. Não é possível desfazer.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Clique de novo para alterar { $name } em { $drive } de { $from } para { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Espaço para uma nova partição

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Espaço em GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Abrir espaço

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Clique de novo para tirar { $size } de { $name } em { $drive }. O Trindle cria um instantâneo antes.

# Note under the row that makes room.
disks-room-note = O Trindle cria um instantâneo e depois diminui a partição enquanto ela está em uso. O espaço liberado fica vazio.

# Tip of the button that checks a part for faults.
disks-check-tip = Verificar se há falhas

# Tip of the button that repairs a part.
# SAFETY: needs a human review before it ships
disks-repair-tip = Repará-la

# Label of the row where a part is checked and repaired.
disks-faults = Falhas

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Clique de novo para reparar { $name }, em { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Tipo

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = Formatar

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Formatá-la

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Clique de novo para apagar tudo em { $words }

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, em { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Excluir { $name }, { $size }, em { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Excluir partição

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Criar uma nova partição aqui

# Label of the row where a new part is made.
disks-new-part = Nova partição

# Tip of the button that makes a new part.
disks-make-tip = Criá-la

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = Clique de novo para criar uma partição nova de { $size } em { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Apagar { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, em { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Nova partição

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } livres, em { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Fechar

# Words on a row while a part is being mounted.
disks-mounting = Montando

# Words on a row when a part has been mounted.
disks-mounted = Montada

# Words on a row while a part is being unmounted.
disks-unmounting = Desmontando

# Words on a row when a part has been unmounted.
disks-unmounted = Desmontada

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Fechando

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Seguro para remover

# Words on a drive's row while it is being powered off.
disks-powering-off = Desligando

# Words on a drive's row when it has been powered off.
disks-off-safe = Seguro para desconectar

# Words on a row while a part is being renamed.
disks-renaming = Renomeando

# Words on a row when a part has been renamed.
disks-renamed = Renomeada

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = A pasta agora é escolhida para você

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Aparece em { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Monta ao iniciar em { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Não montada ao iniciar

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Salvando

# Words on a row while a part is being resized.
disks-resizing = Redimensionando

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Redimensionada para { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = O Trindle não encontrou os arquivos, então não consegue abrir espaço.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Criando um instantâneo

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = O Trindle não encontrou o instantâneo depois.

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Nada foi redimensionado.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Algo mudou enquanto o Trindle criava o instantâneo, então nada foi redimensionado.

# Words on a row while room is being made.
disks-making-room = Abrindo espaço

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = Desligando o arquivo de swap

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = Religando o arquivo de swap

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } O arquivo de swap fica desligado até você reiniciar. { $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } de espaço aberto

# Words on a row while a part is being repaired.
disks-repairing = Reparando

# Words on a row when a part has been repaired.
disks-repaired = Reparada

# Red words when some faults could not be put right.
disks-repair-failed = O Trindle não conseguiu reparar algumas falhas. Copie para outra unidade os arquivos que você quiser guardar.

# Words on a row while a part is being checked.
disks-checking = Verificando

# Words on a row when the check found no faults.
disks-no-faults = Nenhuma falha encontrada

# Red words when the check found faults.
disks-faults-found = Falhas encontradas. Pressione a chave inglesa para repará-las.

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = Formatando

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = Formatada

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = apagando a unidade inteira

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Não achei a nova partição.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Não foi possível acessar o serviço de unidades.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = O Trindle não conseguiu ler o identificador da nova partição.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Excluindo

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Excluída

# Words on a row while a new part is being made.
disks-making-it = Criando

# Words on a row when a new part has been made.
disks-made = Criada

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Não foi possível salvar: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Dê um nome primeiro.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] Esse nome é longo demais. Use no máximo { $max } letra.
       *[other] Esse nome é longo demais. Use no máximo { $max } letras.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Use só letras, números, - e _ (sem espaços).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Comece com uma letra ou um número.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } já está em uso.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] Esse nome é longo demais para esta partição. Use no máximo { $max } letra.
       *[other] Esse nome é longo demais para esta partição. Use no máximo { $max } letras.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Nomes FAT não podem conter * ? . , ; : / \ | + = < > [ ] nem ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Monte primeiro. Este tipo de partição só pode ser aumentada enquanto está montada.
       *[smaller] Monte primeiro. Este tipo de partição só pode ser diminuída enquanto está montada.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Este tipo não pode ser aumentado.
       *[smaller] Este tipo não pode ser diminuído.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Você não informou uma senha, então nada mudou.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Você não informou uma senha, então o trabalho parou no meio.

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = Outra alteração ainda está sendo feita. Tente de novo quando terminar.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = O Trindle não conseguiu garantir que pediria a sua senha, então nada mudou.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = O Trindle não conseguiu cancelar a senha que você acabou de informar. Por alguns minutos, outra alteração em uma unidade pode não pedir senha. O Trindle está tentando de novo.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = A senha que você informou foi cancelada, então a próxima alteração vai pedir uma.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = O Trindle não conseguiu alcançar o serviço de unidades, então nada mudou.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Unidades foi fechado enquanto esta alteração era feita.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Unidades continua aberto até isto terminar, para você ver como foi.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, então nada mudou.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, então o trabalho parou no meio.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Não foi possível consultar o serviço de unidades

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } O trabalho parou no meio.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Feito: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Falhou: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Não feito: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Não se sabe: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } foi desmontada para isto e não está montada agora.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } foi desmontada para isto e continua desmontada. Monte-a pela linha dela.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } pode ter sido desmontada. Olhe a linha dela.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } foi desmontada para isto. Monte-a de novo pela linha dela e digite a senha quando for pedida.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } foi desmontada para isto e não pôde ser montada de novo ({ $why }). Monte-a pela linha dela.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = A formatação terminou, mas o Trindle não conseguiu dar a ela de novo a pasta { $dir }. { $why } Escolha a pasta de novo pela linha dela.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Algo ainda a está usando. Feche as janelas ou programas que mostram os arquivos dela e tente de novo.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Esse tamanho é pequeno demais para os arquivos que já estão nela.

# Step of a change. { $dev } is the part.
disks-step-unmount = desmontando { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = montando { $dev } de novo

# Step of a change.
disks-step-eject = preparando a unidade para a remoção

# Step of a change.
disks-step-power-off = desligando a unidade

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = formatando { $dev }

# Step of a change.
disks-step-create = criando uma partição

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = excluindo { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = redimensionando { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = redimensionando o que há em { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = reparando { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = verificando { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = renomeando { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = guardando as configurações de pasta e de inicialização de { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = a etapa { $method } em { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Esta partição { $kind } só pode ser diminuída quando não está em uso, então o Trindle não consegue abrir espaço a partir dela aqui.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Partições xfs não podem ser reduzidas.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Esta partição { $kind } não pode ser diminuída enquanto está em uso.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = O Trindle só pode criar um instantâneo do disco do próprio sistema, então não consegue abrir espaço aqui.

# Red words under the Make room row.
disks-room-not-in-table = Ela não é uma partição na tabela de partições de uma unidade, então o Trindle não consegue abrir espaço a partir dela.

# Red words under the Make room row.
disks-room-no-online-shrink = O serviço de unidades não consegue diminuir o Btrfs enquanto está em uso aqui, então o Trindle não consegue abrir espaço.

# Red words under the Make room row.
disks-room-holds-parts = Ela tem outras partições dentro, então o Trindle não consegue abrir espaço a partir dela.

# Red words under the Make room row.
disks-room-not-gpt = A unidade dela não tem tabela de partições GPT, então o Trindle não consegue abrir espaço a partir dela.

# Red words under the Make room row.
disks-room-mount-first = Monte primeiro: o Btrfs é diminuído enquanto está montado.

# Red words under the Make room row.
disks-room-mounts-not-one = Ela não está montada, então o Trindle não consegue abrir espaço.

# Red words under the Make room row.
disks-room-read-only = Ela está montada como somente leitura, então o Trindle não consegue abrir espaço.

# Red words under the Make room row.
disks-room-read-only-unknown = O Trindle não conseguiu saber se ela é somente leitura, então não consegue abrir espaço.

# Red words under the Make room row.
disks-room-many-drives = Os arquivos estão espalhados por mais de uma unidade, então o Trindle não consegue abrir espaço.

# Red words under the Make room row.
disks-room-drives-unknown = O Trindle não conseguiu saber por quantas unidades os arquivos estão espalhados, então não consegue abrir espaço.

# Red words under the Make room row.
disks-room-swap-unknown = O Trindle não conseguiu saber se há um arquivo de swap nela, então não consegue abrir espaço.

# Red words under the Make room row.
disks-room-used-unknown = O Trindle não conseguiu ler quanto dela está em uso, então não consegue abrir espaço.

# Red words under the Make room row.
disks-room-area-unknown = O serviço de unidades não informa o tamanho do sistema de arquivos, então o Trindle não consegue verificá-lo nem abrir espaço.

# Red words under the Make room row.
disks-room-open-missing = O Trindle não encontrou a cópia desbloqueada dela, então não consegue abrir espaço.

# Red words under the Make room row.
disks-room-encryption-sizes = Os tamanhos dados para a criptografia não coincidem, então o Trindle não consegue abrir espaço.

# Red words under the Make room row.
disks-room-sizes = Os tamanhos dados para ela não coincidem, então o Trindle não consegue abrir espaço.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Ela tem { $used } e precisa manter mais do que isso, então há pouco espaço para liberar.

# Red words under the Make room row.
disks-room-sizes-unread = Não foi possível ler os tamanhos.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Você pode abrir entre { $least } e { $most } de espaço.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, então o Trindle parou.

# What was found wrong while making room.
disks-room-check-unmounted = Os arquivos não estão mais montados

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = A partição está com { $is } e deveria ter { $should }

# What was found wrong while making room.
disks-room-check-open = A cópia desbloqueada da criptografia não tem o tamanho que deveria

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = O sistema de arquivos está com { $is } e deveria ter { $should }

# What was found wrong while making room.
disks-room-check-area-small = O sistema de arquivos é menor do que o que está em uso

# What was found wrong while making room.
disks-room-check-fit = O sistema de arquivos já não cabe

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] A partição não aparece mais na lista do serviço de unidades, então o trabalho foi interrompido.
        [copy] A cópia desbloqueada não aparece mais na lista do serviço de unidades, então o trabalho foi interrompido.
       *[files] O sistema de arquivos não aparece mais na lista do serviço de unidades, então o trabalho foi interrompido.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = diminuindo o sistema de arquivos

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = diminuindo a cópia desbloqueada da criptografia

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = reduzindo a partição

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = verificando os tamanhos

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Antes de abrir espaço em { $dev } ({ $at })

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }


# Red words when another change is pressed while one is still being made.
disks-change-going = Uma alteração está sendo feita.


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Clique de novo para alterar { $name } em { $drive } de { $from } para { $to }. Faça antes um backup dos arquivos dela, porque uma queda de energia durante a redução pode fazê-los se perder.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = O Windows em { $name } foi hibernado ou deixado na inicialização rápida, então reduzi-la pode fazer com que perca seus arquivos. Inicie o Windows, escolha Reiniciar e tente de novo.
