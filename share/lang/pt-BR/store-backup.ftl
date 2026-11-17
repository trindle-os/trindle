
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Nada foi restaurado, porque { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = O Trindle restaurou só parte do backup, porque { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = o backup contém um link ou outra coisa que chegaria a fora da pasta em que ele é colocado

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = já existe algo neste computador onde parte do backup precisa ficar

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = não há onde descompactar o backup

# The reason when the file is not a Trindle backup.
store-because-not-trindle = esse arquivo não é um backup do Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = o backup está danificado e o Trindle não conseguiu ler a lista dele

# The reason when the file is not a Flat backup.
store-because-not-flat = esse arquivo não parece um backup do Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = esse arquivo não contém uma lista de aplicativos do Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = ele foi feito por uma versão do Flat mais nova do que o Trindle consegue ler

# The reason when the file holds no apps.
store-because-no-apps = não há aplicativos nesse arquivo

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = não foi possível ler o backup: { $why }

# The reason when the program gave none.
store-because-none = nenhum motivo informado

# The reason when the system said permission was denied.
store-because-permission = o Trindle não tem permissão para gravar ali

# The reason when the disk is full.
store-because-space = não há mais espaço neste computador

# The reason when a part of the computer is read-only.
store-because-read-only = essa parte deste computador é somente leitura

# The reason when the person's disk quota is used up.
store-because-quota = o espaço permitido para esta conta acabou

# The reason when a file or program is missing.
store-because-missing = falta um arquivo ou programa de que ele precisa

# The reason when no reason could be read from the program's last line.
store-because-failed = falhou

# The reason when the backup has no name to be written under.
store-because-no-name = ele não tem nome

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = as configurações dele no backup chegam a fora da própria pasta

# The reason when something is too big to be read whole.
store-because-too-big = ele é grande demais para ser lido

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = as configurações dele no backup foram danificadas ou alteradas

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Tudo foi restaurado, exceto a foto da sua conta. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } O Trindle também não restaurou a foto da sua conta. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = Não foi possível pedir a senha: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = O Trindle não conseguiu ler isso do backup: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = O Trindle não fez o backup, porque { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = ainda nada

# Beside the size of a part of the backup: about how big it is.
store-backup-about = cerca de { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Restaurando as configurações e os AppImages do Trindle…

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = O Trindle restaurou suas configurações e dados, mas o Flatpak não quis instalar estes aplicativos: { $failed }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = O Trindle restaurou suas configurações e dados, mas não instalou estes aplicativos: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] O Trindle ignorou 1 aplicativo, porque o nome dele não era válido.
        [many] O Trindle ignorou { $n } de aplicativos, porque os nomes deles não eram válidos.
       *[other] O Trindle ignorou { $n } aplicativos, porque os nomes deles não eram válidos.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = a origem { $name } não é criptografada, por isso o Trindle não a usou

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = a origem { $name } não está neste computador, e o backup não diz onde ela fica

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = configurações de { $name } (porque a cópia delas no backup não passou nas verificações)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (porque o nome da origem dele não é válido)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (porque { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (o Flatpak não conseguiu instalá-lo: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = tudo (porque não há onde descompactar o backup)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = permissões de todos os aplicativos

# Said among what could not be done: one app's settings.
store-flat-app-settings = configurações de { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. O que havia antes foi mantido como { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Instalando { $name } ({ $at } de { $total })…

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Restaurando as configurações de { $name } ({ $at } de { $total })…

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] { $n } aplicativo instalado.
        [many] { $n } de aplicativos instalados.
       *[other] { $n } aplicativos instalados.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
        [one] Configurações de um aplicativo restauradas.
       *[other] Configurações de { $n } restauradas.
    }

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] { $n } aplicativo já estava instalado.
       *[other] { $n } aplicativos já estavam instalados.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Ficaram de fora: { $list }

# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }: o Trindle só restaurou as permissões que tiram acesso.
