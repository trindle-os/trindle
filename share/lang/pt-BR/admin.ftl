
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Esse não é um nome de login válido.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Esse nome de login é longo demais.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Não há usuário chamado { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } é uma conta do sistema, então o Trindle não a alterou.

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = Nenhuma senha foi informada.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] A senha precisa ter pelo menos { $n } caractere.
       *[other] A senha precisa ter pelo menos { $n } caracteres.
        [many] A senha precisa ter pelo menos { $n } caracteres.
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = Uma senha não pode ter quebra de linha.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } já existe.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } é o único administrador, então precisa continuar sendo administrador.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Você não pode excluir a sua própria conta.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = O Trindle excluiu a conta de { $name }, mas não conseguiu remover a foto dessa conta na tela de login: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Essa imagem é grande demais.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Essa não é uma imagem feita pelo Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = O Trindle não conseguiu dar à tela de login acesso à pasta das imagens: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Esse não é um nome de idioma.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = O Trindle não conseguiu ler a lista de idiomas do Debian: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = O Debian não reconhece o idioma { $code }.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = O nome de um computador só pode ter letras, números e hifens. Pode ter até 63 caracteres e não pode começar nem terminar com hífen.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = O limite de carga está ativo por enquanto, mas será perdido quando o computador for desligado: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = A carga total está ativada por enquanto, mas o limite volta na próxima vez que o computador iniciar: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Outros computadores ainda podem se conectar, porque o Trindle não conseguiu desativar o compartilhamento: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Esse não é um ID de plugin.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Essa pessoa não existe.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Não há pasta para essa pessoa.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = O Trindle não conseguiu salvar a lista de plugins permitidos: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Esse plugin não existe.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Nada foi atualizado. O Trindle não conseguiu saber qual versão dele mesmo está instalada, então não pôde deixar de fora a própria atualização.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Instalando as atualizações

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Arrumando

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Instalando o Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Esse não é o nome de um pacote.

# Red words when the package to remove is not installed.
admin-package-not-installed = Esse pacote não está instalado.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Esse não é um dispositivo que o fwupd conhece.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = O Trindle não sabe quem quer o Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = O bloqueio não começou a tempo.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = O bloqueio não está totalmente desligado: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = O Trindle não conseguiu colocar { $path } de volta: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Esse não é um nome de site.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Servidor não encontrado.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } parou antes de ter tudo de que precisava, então o trabalho não terminou: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } falhou.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = Não foi possível remover { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Nada foi alterado, porque o Trindle não conseguiu ler { $path }: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } O Trindle também não conseguiu voltar as coisas ao que eram: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Esse não é um período válido para manter versões anteriores.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Não achei as versões anteriores.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Há um arquivo onde deveria estar a pasta das versões anteriores.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Não sei quem pediu.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = Isto só roda pelo pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Esse não é o nome de um computador.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Esse não é o nome de uma pasta compartilhada.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Esse não é o endereço de um computador.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Esse não é o caminho de uma pasta compartilhada.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Só as suas próprias pastas compartilhadas podem ser fechadas aqui.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Nenhuma pasta compartilhada está aberta ali.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Algo alterou { $path } desde que o Trindle o configurou, então nada está conectado ali.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } não está como o Trindle deixou.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Há pastas compartilhadas abertas demais.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } não está instalado. Ele vem com o pacote { $package } do Debian.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Não foi possível ler o login.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = O Trindle não conseguiu ler o que está montado neste computador: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Um programa ainda está usando algo nesta pasta. Feche o programa e tente de novo.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Esse computador não está respondendo. A pasta compartilhada será fechada quando você sair da sessão.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Esse não é um ID de usuário.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Não foi possível criar o grupo sambashare.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Baixando o Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Configurando o compartilhamento

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = O compartilhamento ainda não está configurado. Compartilhe uma pasta primeiro.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Este computador ainda está visível para os outros na rede: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Esse não é um número de instantâneo.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Criado nas Configurações

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = O Trindle não conseguiu saber em qual disco o sistema está.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = O sistema não está em um subvolume Btrfs próprio, então o Trindle não pode trocá-lo por um instantâneo.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = O Trindle não conseguiu baixar a chave de assinatura de { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Esse endereço não devolveu uma chave de assinatura.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Nada foi alterado, porque o Trindle não conseguiu ler { $path }: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Baixando a chave de assinatura

# Second line of the progress while a source is added.
admin-step-adding-source = Adicionando e lendo a lista

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Não foi possível usar a fonte. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = O Trindle não conseguiu usar a fonte, então a removeu de novo. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } ainda não pode ligar nem desligar: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = O Trindle não conseguiu iniciá-lo para { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = O Trindle não conseguiu dar a { $user } permissão para usar máquinas virtuais, de que o Cowork precisa: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = O Trindle não conseguiu dar a { $user } permissão para usá-lo sem senha: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } está instalado, mas o Trindle não conseguiu terminar a configuração: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Instalando

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Essa não é uma das fontes prontas.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } não é feito para este tipo de computador ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Configurando

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Cada endereço deve ser um endereço web simples que comece com https://.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = A suíte e os componentes devem ser palavras simples.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Esse não é um nome de pacote.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Removendo

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Lendo as listas de novo

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Só as fontes que o Trindle adicionou são removidas aqui.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = A fonte ainda está lá: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = O instantâneo { $n } não está no disco.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Nada foi alterado, porque o Trindle não conseguiu preparar o instantâneo. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = O Trindle não conseguiu pôr de lado o sistema atual: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = O sistema atual agora se chama { $aside } em vez de { $sub }, e o computador não vai iniciar até que ele se chame { $sub } de novo: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = O Trindle não conseguiu mover os instantâneos: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = O Trindle não conseguiu colocar o instantâneo no lugar: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = O Trindle não conseguiu atualizar a lista de sistemas postos de lado: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Não foi possível remover { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = O Trindle removeu estes itens porque nada mais precisava deles: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Nem todas as atualizações foram instaladas. Estas ficaram retidas: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature }: desativado, porque o driver ({ $driver }, DKMS) ainda não compila para o novo kernel do Linux.

admin-driver-off = O driver ({ $driver }, DKMS) ainda não compila para o novo kernel do Linux. Os recursos que dependem dele estão desativados.

# Said once after the drivers that are off.
admin-driver-off-rest = O kernel e o restante da atualização foram instalados.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = câmera virtual

admin-language-in-use = { $code } é o idioma em que este computador inicia, então não pode ser removido.

# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Este computador não tem lentidão por split-lock para desligar.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = O Trindle não encontrou essa unidade, ou ela é a unidade do sistema.

# The drive could not be closed where it was open.
admin-games-drive-busy = A unidade está em uso. Feche os arquivos ou programas abertos nela e tente de novo: { $why }

# Another software change has the read-only system open.
admin-system-busy = Outra alteração de software está em andamento. Tente de novo quando terminar.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Nada foi alterado, porque o Trindle não conseguiu destravar o sistema: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = O Trindle não conseguiu travar o sistema de novo. Ele será travado na próxima inicialização: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = O Trindle não conseguiu manter /usr/local gravável: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Só o root pode executar isto, e não pelo pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Requer Btrfs.

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = O instantâneo está no lugar, e o computador vai iniciar a partir dele. O Trindle não conseguiu terminar a limpeza do sistema antigo. Ele tentará de novo na próxima vez que o computador iniciar: { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = Você pode abrir no máximo { $most } portas.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = O Trindle não conseguiu salvar a lista de portas abertas: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = O firewall continua ligado: { $why }

# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = Este plugin agora pede portas de rede diferentes das que você permitiu. Permita-o de novo para abri-las.

# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = Um programa só pode ser adicionado para a pessoa que o pede.

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = Outra restauração usou o mesmo nome há pouco. Espere alguns segundos e tente de novo.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = Não foi possível desconectar { $path }: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 pasta compartilhada não respondeu, então continua conectada. Ela será desconectada quando o computador desligar.
       *[other] { $n } pastas compartilhadas não responderam, então continuam conectadas. Elas serão desconectadas quando o computador desligar.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = Não foi possível desligar o arquivo de swap { $path }: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = Não foi possível religar o arquivo de swap { $path }: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } não é um arquivo de swap do sistema.

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = O Trindle não usará o arquivo de swap { $path }, porque outros poderiam alterar ou ler { $why }.

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = Não achei onde o arquivo de swap está agora, então sair da hibernação pode falhar.

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = A hibernação voltará a funcionar depois de reiniciar.

# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = O chip já está sendo configurado. Espere terminar.

# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = Essa não é a senha do disco.

# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = Senhas erradas demais. Espere um minuto e tente de novo.

# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = O chip destravou o disco na última inicialização. Nada a fazer.

# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = O disco mudou desde a inicialização. Reinicie e tente de novo.

# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = Chip de segurança não encontrado. Ligue-o na BIOS e tente de novo.

# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = A ferramenta que configura o chip, clevis, não está instalada.

# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = O disco não tem espaço para outra chave.

# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = Não foi possível configurar o chip. O disco ainda abre com a senha.

# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = O chip funciona de novo, mas a chave antiga não pôde ser removida.
