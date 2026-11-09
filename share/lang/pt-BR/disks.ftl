
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Unidades

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Sobre esta página

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Cada unidade dentro do computador ou ligada a ele, desenhada como uma barra com as suas partições. Cada partição é tão larga quanto é grande e preenchida até onde está em uso, então uma partição cheia é fácil de ver.

# Second paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-2 = Sob a imagem, cada unidade tem um cartão. Abra uma partição, monte-a ou desmonte-a, ou retire uma unidade com segurança ou desligue-a. O lápis em uma partição abre as alterações dela: o nome, uma pasta própria e se fica pronta ao iniciar, o tamanho, verificá-la e repará-la, formatá-la ou excluí-la. O sinal de mais no espaço livre cria uma nova partição.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = A unidade de onde o sistema roda aparece, mas nunca é oferecida para nenhuma alteração que apague ou mova coisas. Toda alteração em qualquer unidade pede a senha de um administrador, a cada vez. O olho esconde uma unidade ou uma partição das Pastas e do anel: ela continua aqui, em cinza, e não fica bloqueada.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Cada partição de cada unidade, preenchida até onde está em uso.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Suas unidades

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Não foi possível ler as unidades agora.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Tentar de novo

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Olhar de novo

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Nenhuma unidade foi encontrada. Ligue uma e ela aparece aqui.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = O serviço de unidades

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = O barramento do sistema

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } não está disponível. Instale com: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Olhando as unidades…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } está falhando. Copie agora tudo o que quiser guardar dele.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } precisa de atenção: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] As unidades e o espaço nelas.
        [one] As unidades e o espaço nelas, e uma unidade ligada.
       *[other] As unidades e o espaço nelas, e { $plugged } unidades ligadas.
        [many] As unidades e o espaço nelas, e { $plugged } unidades ligadas.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Quase cheio: só { $free } livres neste computador. Exclua ou mova alguns arquivos em breve.
        [one] Quase cheio: só { $free } livres neste computador, e uma unidade ligada. Exclua ou mova alguns arquivos em breve.
       *[other] Quase cheio: só { $free } livres neste computador, e { $plugged } unidades ligadas. Exclua ou mova alguns arquivos em breve.
        [many] Quase cheio: só { $free } livres neste computador, e { $plugged } unidades ligadas. Exclua ou mova alguns arquivos em breve.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Ficando cheio: { $free } livres neste computador.
        [one] Ficando cheio: { $free } livres neste computador, e uma unidade ligada.
       *[other] Ficando cheio: { $free } livres neste computador, e { $plugged } unidades ligadas.
        [many] Ficando cheio: { $free } livres neste computador, e { $plugged } unidades ligadas.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Muito espaço: { $free } livres neste computador.
        [one] Muito espaço: { $free } livres neste computador, e uma unidade ligada.
       *[other] Muito espaço: { $free } livres neste computador, e { $plugged } unidades ligadas.
        [many] Muito espaço: { $free } livres neste computador, e { $plugged } unidades ligadas.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Este computador

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Seus arquivos

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] { $free } ainda livres, onde está o sistema.
       *[files] { $free } ainda livres, para os seus próprios arquivos.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } cheio, { $free } livres de { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = de { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Sem leituras, as unidades não dizem como estão

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Saudável, como a própria unidade informa

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] Saudáveis, como todas as { $n } unidades informam
       *[other] Saudáveis, como todas as { $n } unidades informam
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } precisa de atenção, veja o cartão dela
       *[other] { $n } precisam de atenção, veja os cartões delas
        [many] { $n } precisam de atenção, veja os cartões delas
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
disks-kind-container = Contém outras partições

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
disks-trouble-readonly = passou sozinha para somente leitura

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = a cópia de segurança da memória falhou

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = parte dela passou sozinha para somente leitura

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = desgastada além do que o fabricante espera

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
disks-health-failing = Falhando: copie agora tudo o que quiser guardar dela

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Precisa de atenção: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Saudável

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Nenhuma unidade respondendo

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = A caixa responde, mas a unidade dentro dela não. Desconecte, espere dez segundos e ligue em uma porta azul ou USB-C com o cabo dela; se continuar assim, a unidade dentro pode ter se soltado ou falhado.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Remover com segurança

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Desligá-la

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Apagar a unidade inteira

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Mostrar esta unidade
       *[part] Mostrar esta partição
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Esconder esta unidade. { $lock }
       *[part] Esconder esta partição. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Escondida das Pastas e do anel. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = O sistema roda a partir dela: nada aqui pode ser apagado.

# Note on the card of a drive that has no partition table.
disks-no-table = Sem tabela de partições: a unidade inteira está livre.

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
disks-change-it = Alterar

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
disks-startup-note = Pronta sempre que o computador inicia. Se não estiver lá, o computador inicia mesmo assim.

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
disks-format-erases = Isto vai apagar tudo em { $words }. Não pode ser desfeito.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Clique de novo: { $name } em { $drive } vai de { $from } para { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Abrir espaço para uma nova partição

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Espaço em GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Abrir espaço

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Clique de novo: { $name } em { $drive } perde { $size }, depois de um instantâneo

# Note under the row that makes room.
disks-room-note = Diminuída em uso, depois de um instantâneo, deixando espaço livre.

# Tip of the button that checks a part for faults.
disks-check-tip = Verificar se há falhas

# Tip of the button that repairs a part.
# SAFETY: needs a human review before it ships
disks-repair-tip = Repará-la

# Label of the row where a part is checked and repaired.
disks-faults = Falhas

# Note under the Faults row.
# SAFETY: needs a human review before it ships
disks-faults-note = Procura falhas na forma como os arquivos são guardados e as corrige. Ela é desmontada enquanto isto roda.

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

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = Recomeça como uma só partição, ocupando a unidade.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = Apaga tudo só nesta partição.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Clique de novo: tudo em { $words } será apagado

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, em { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Excluir { $name }, { $size }, em { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Excluir esta partição

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = O espaço dela fica livre, pronto para uma nova partição ou para a anterior crescer.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Criar uma nova partição aqui

# Label of the row where a new part is made.
disks-new-part = Nova partição

# Note under the row where a new part is made.
disks-new-part-note = Criada neste espaço livre e formatada, pronta para usar.

# Tip of the button that makes a new part.
disks-make-tip = Criá-la

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = Clique de novo para criar uma partição de { $size } em { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Apagar { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, em { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Uma nova partição

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
disks-off-safe = Desligada: seguro desconectar

# Words on a row while a part is being renamed.
disks-renaming = Renomeando

# Words on a row when a part has been renamed.
disks-renamed = Renomeada

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = O computador escolhe a pasta de novo

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Aparece em { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Monta ao iniciar em { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Deixada em paz ao iniciar

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Salvando

# Words on a row while a part is being resized.
disks-resizing = Redimensionando

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Redimensionada para { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Os arquivos não foram encontrados, então não é possível abrir espaço.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Criando um instantâneo

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = O instantâneo não foi encontrado depois.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Nada foi redimensionado.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = As coisas mudaram enquanto o instantâneo era criado, então nada foi redimensionado.

# Words on a row while room is being made.
disks-making-room = Abrindo espaço

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } de espaço aberto

# Words on a row while a part is being repaired.
disks-repairing = Reparando

# Words on a row when a part has been repaired.
disks-repaired = Reparada

# Red words when some faults could not be put right.
disks-repair-failed = Algumas falhas não puderam ser corrigidas. Copie o que quiser guardar.

# Words on a row while a part is being checked.
disks-checking = Verificando

# Words on a row when the check found no faults.
disks-no-faults = Nenhuma falha encontrada

# Red words when the check found faults.
disks-faults-found = Falhas encontradas: pressione a chave inglesa para repará-las.

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
disks-new-part-missing = A nova partição não foi encontrada.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Não foi possível alcançar o serviço de unidades.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Não foi possível ler o identificador da nova partição.

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
disks-hide-failed = Isso não pôde ser guardado: { $why }

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
        [one] Esse nome é longo demais: no máximo { $max } letra.
       *[other] Esse nome é longo demais: no máximo { $max } letras.
        [many] Esse nome é longo demais: no máximo { $max } letras.
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
        [one] Esse nome é longo demais para este tipo: no máximo { $max } letra.
       *[other] Esse nome é longo demais para este tipo: no máximo { $max } letras.
        [many] Esse nome é longo demais para este tipo: no máximo { $max } letras.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Nomes FAT não podem conter * ? . , ; : / \ | + = < > [ ] nem ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Monte primeiro: este tipo só pode ser aumentado quando montado.
       *[smaller] Monte primeiro: este tipo só pode ser diminuído quando montado.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Este tipo não pode ser aumentado.
       *[smaller] Este tipo não pode ser diminuído.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Nenhuma senha foi informada, então nada mudou.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Nenhuma senha foi informada, então o trabalho parou no meio.

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = Outra alteração ainda está sendo feita. Tente de novo quando terminar.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Não foi possível ter certeza de que a senha seria pedida, então nada mudou.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = A senha que acabou de ser dada não pôde ser retirada, então por alguns minutos outra alteração em uma unidade pode não pedir senha. Está sendo tentado de novo.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = A senha dada foi retirada: a próxima alteração vai pedir uma.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Não foi possível alcançar o serviço de unidades, então nada mudou.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Unidades foi fechado enquanto esta alteração era feita.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Unidades continua aberto até isto terminar, para poder dizer como acabou.

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
disks-still-unmounted = { $dev } foi desmontada para isto e continua desmontada: monte-a pela linha dela.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } pode ter sido desmontada: olhe a linha dela.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } foi desmontada para isto, e montá-la de novo pede uma senha própria: monte-a pela linha dela.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } foi desmontada para isto e não pôde ser montada de novo ({ $why }): monte-a pela linha dela.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Formatada, mas não recebeu de novo a pasta { $dir }. { $why } Escolha a pasta de novo pela linha dela.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Algo ainda a está usando. Feche as janelas ou programas que mostram os arquivos dela e tente de novo.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Não há espaço suficiente para o que há nela nesse tamanho.

# Step of a change. { $dev } is the part.
disks-step-unmount = desmontando { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = montando { $dev } de novo

# Step of a change.
disks-step-eject = fechando a unidade para ser removida

# Step of a change.
disks-step-power-off = desligando a unidade

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = formatando { $dev }

# Step of a change.
disks-step-create = criando a nova partição

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = excluindo { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = redimensionando a partição { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = redimensionando o que há em { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = reparando { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = verificando { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = renomeando { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = guardando a pasta e a escolha de inicialização de { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = a etapa { $method } em { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Esta partição { $kind } só pode ser diminuída quando não está em uso, então não é possível abrir espaço a partir dela aqui.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Esta partição xfs não pode ser diminuída.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Esta partição { $kind } não pode ser diminuída enquanto está em uso.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Só é possível criar um instantâneo do disco do próprio sistema, então não é possível abrir espaço aqui.

# Red words under the Make room row.
disks-room-not-in-table = Ela não é uma partição da tabela de uma unidade, então não é possível abrir espaço a partir dela.

# Red words under the Make room row.
disks-room-no-online-shrink = O serviço de unidades não consegue diminuir o Btrfs enquanto está em uso aqui, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-holds-parts = Ela contém outras partições, então não é possível abrir espaço a partir dela.

# Red words under the Make room row.
disks-room-not-gpt = A unidade dela não tem tabela GPT, então não é possível abrir espaço a partir dela.

# Red words under the Make room row.
disks-room-mount-first = Monte primeiro: o Btrfs é diminuído enquanto está montado.

# Red words under the Make room row.
disks-room-mounts-not-one = Ela não está montada em exatamente um lugar, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-read-only = Ela está montada como somente leitura, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-read-only-unknown = Não foi possível saber se ela é somente leitura, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-many-drives = Os arquivos se espalham por mais de uma unidade, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-drives-unknown = Não foi possível saber por quantas unidades os arquivos se espalham, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-swap-file = Há um arquivo de swap nela, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-swap-unknown = Não foi possível saber se há um arquivo de swap nela, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-used-unknown = Não foi possível ler quanto dela está em uso, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-area-unknown = O serviço de unidades não diz o tamanho da área dos arquivos, então não foi possível verificar e não é possível abrir espaço.

# Red words under the Make room row.
disks-room-open-missing = A cópia aberta dela não foi encontrada, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-encryption-sizes = Os tamanhos dados para a criptografia não concordam, então não é possível abrir espaço.

# Red words under the Make room row.
disks-room-sizes = Os tamanhos dados para ela não concordam, então não é possível abrir espaço.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Ela contém { $used } e precisa que mais do que isso seja mantido, então pouco pode ser liberado.

# Red words under the Make room row.
disks-room-sizes-unread = Não foi possível ler os tamanhos dela.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = É possível abrir de { $least } a { $most } de espaço.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, então o trabalho foi interrompido.

# What was found wrong while making room.
disks-room-check-unmounted = Os arquivos não estão mais montados

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = A partição está com { $is } e deveria ter { $should }

# What was found wrong while making room.
disks-room-check-open = A cópia aberta da criptografia não tem o tamanho que deveria

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = A área dos arquivos está com { $is } e deveria ter { $should }

# What was found wrong while making room.
disks-room-check-area-small = A área dos arquivos é menor do que o que está em uso

# What was found wrong while making room.
disks-room-check-fit = O que contém o conteúdo já não cabe

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] A partição não aparece mais na lista do serviço de unidades, então o trabalho foi interrompido.
        [copy] A cópia aberta não aparece mais na lista do serviço de unidades, então o trabalho foi interrompido.
       *[files] O sistema de arquivos não aparece mais na lista do serviço de unidades, então o trabalho foi interrompido.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = diminuindo a área dos arquivos

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = diminuindo a cópia aberta da criptografia

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = diminuindo a partição

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = verificando os tamanhos

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Antes de abrir espaço em { $dev } ({ $at })


# Red words when another change is pressed while one is still being made.
disks-change-going = Uma alteração está sendo feita.


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Clique de novo: { $name } em { $drive } vai de { $from } para { $to }. Faça antes um backup dos arquivos dela, pois uma queda de energia durante a redução pode fazê-los se perder.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = O Windows em { $name } foi hibernado ou deixado na inicialização rápida, então reduzi-la pode fazer com que perca seus arquivos. Inicie o Windows, escolha Reiniciar e tente de novo.

