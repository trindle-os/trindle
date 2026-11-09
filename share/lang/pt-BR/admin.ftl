
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Esse não é um nome de login válido.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Esse nome de login é longo demais.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Não existe um usuário chamado { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } é uma conta do sistema e não será alterada.

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
admin-password-line-break = Uma senha não pode conter quebra de linha.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } já existe.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } é o único administrador, então continua sendo.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Você não pode excluir a sua própria conta.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = { $name } foi excluído, mas a foto continua guardada para a tela de login: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Essa imagem é grande demais.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Essa não é uma imagem feita pelo Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = A pasta das imagens não pôde ser aberta para a tela de login: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Esse não é um nome de idioma.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = A lista de idiomas do Debian não pôde ser lida: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } não é um idioma que o Debian conhece.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = O nome de um computador tem letras, números e hifens, até 63, sem começar nem terminar com hífen.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = O limite está ativo por enquanto, mas será perdido quando o computador for desligado: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = A carga está completa por enquanto, mas o limite voltará na próxima inicialização: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Outros computadores ainda podem se conectar, pois a chave não pôde ser desligada: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Esse não é o id de um plugin.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Essa pessoa não é conhecida.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = A pasta dessa pessoa não é conhecida.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = A lista de plugins permitidos não pôde ser guardada: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Esse plugin não foi encontrado.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Nada foi atualizado: não foi possível saber qual versão do Trindle este computador tem, então a atualização do próprio Trindle não pôde ser deixada de fora.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Instalando as atualizações

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Arrumando

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Instalando o Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Esse não é o nome de um pacote.

# Red words when the package to remove is not installed.
admin-package-not-installed = Não feito: não está instalado.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Esse não é um dispositivo que o fwupd conhece.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Não foi possível saber quem pediu o Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = O bloqueio não começou a tempo.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = O bloqueio não está totalmente desligado: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = { $path } não pôde ser colocado de volta: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Esse não é um nome de site.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Esse servidor não existe.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } parou antes de receber tudo, então isto não pode ser dado como concluído: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } falhou.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } não pôde ser removido: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Nada foi alterado, porque { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Voltar as coisas ao que eram também não funcionou: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Esse não é um período pelo qual versões anteriores podem ser mantidas.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = A pasta dos instantâneos não está lá.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Algo que não é uma pasta está onde os instantâneos ficam.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Não foi possível saber quem pediu.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = Isto só é executado por meio do pkexec.

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
admin-share-folder-changed = { $path } não está como o Trindle deixou, então nada é montado ali.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } não está como o Trindle deixou.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Há pastas compartilhadas abertas demais.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } não está instalado. Ele vem no pacote { $package } do Debian.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Não foi possível ler o login.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Não foi possível ler o que está montado neste computador: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Um programa ainda tem algo aberto nela. Feche isso primeiro.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Esse computador não está respondendo. Ele é fechado quando você sai da sessão.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Esse não é um id de usuário.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = O grupo sambashare não pôde ser criado.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Baixando o Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Configurando o compartilhamento

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = O compartilhamento ainda não está configurado. Compartilhe uma pasta primeiro.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Este computador ainda aparece para os outros na rede: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Esse não é um número de instantâneo.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Criado nas Configurações

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Não foi possível saber em qual disco o sistema está.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = O sistema não está em um subvolume Btrfs próprio, então não pode ser trocado por um instantâneo.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Não foi possível baixar a chave de assinatura de { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = O que esse endereço devolveu não é uma chave de assinatura.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Nada foi alterado, porque { $path } não pode ser lido: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Baixando a chave de assinatura

# Second line of the progress while a source is added.
admin-step-adding-source = Adicionando e lendo a lista

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = A fonte não pôde ser usada. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = A fonte não pôde ser usada, então foi removida de novo. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } ainda não pode ligar nem desligar: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Não foi possível iniciá-lo para { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = { $user } não pôde receber permissão para usar máquinas virtuais, que o Cowork precisa: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = { $user } não pôde receber permissão para usá-lo sem senha: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } está instalado, mas a configuração não terminou: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Instalando

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Essa não é uma das fontes prontas.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } não é feito para este tipo de computador ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Configurando

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Os endereços devem começar com https:// e não ter mais nada.

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
admin-restore-not-ready = Nada foi alterado: o instantâneo não pôde ser preparado. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Não foi possível pôr de lado o sistema atual: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = o sistema atual agora se chama { $aside } e não { $sub }, e o computador não vai iniciar até que ele se chame { $sub } de novo: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Não foi possível mover os instantâneos: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Não foi possível colocar o instantâneo no lugar: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = A lista do que foi posto de lado não pôde ser atualizada: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Não foi possível remover: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Nada mais precisava destes, então foram removidos: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Nem todas as atualizações foram instaladas. Retidas: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature }: desativado. O driver ({ $driver }, DKMS) ainda não compila no novo kernel.

admin-driver-off = Um driver ({ $driver }, DKMS) ainda não compila no novo kernel, então o que ele oferece está desativado.

# Said once after the drivers that are off.
admin-driver-off-rest = O kernel e o restante da atualização foram instalados.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = câmera virtual

admin-language-in-use = { $code } é o idioma em que este computador inicia, então ele fica.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Este computador não tem lentidão por split-lock para desligar.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Essa unidade não foi encontrada ou contém o sistema.

# The drive could not be closed where it was open.
admin-games-drive-busy = A unidade está em uso. Feche o que estiver aberto nela e tente de novo: { $why }

# Another software change has the read-only system open.
admin-system-busy = Outra alteração de software está em andamento. Tente de novo quando terminar.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Nada foi alterado: não foi possível destravar o sistema: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Não foi possível travar o sistema de novo. Ele será travado na próxima inicialização: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Não foi possível manter /usr/local gravável: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Só o root pode executar isto, e não pelo pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Requer Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = O instantâneo está no lugar e o computador vai iniciar a partir dele. A limpeza do sistema antigo não terminou e será tentada de novo na próxima inicialização: { $why }

