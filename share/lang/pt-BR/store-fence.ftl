
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Esse não é um nome de programa válido.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Isso não foi salvo.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Escolha uma pasta que exista.

# Red words when the whole home folder is chosen to share.
store-fence-whole-home = A pasta pessoal inteira continua privada.

# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = Esta pasta contém a sua pasta pessoal, então continua privada.

# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = Só pastas dentro da sua pasta pessoal podem ser compartilhadas.

# Red words when a hidden folder is chosen to share.
store-fence-hidden = Pastas ocultas sempre continuam privadas.

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = Esta pasta guarda senhas ou e-mails, então continua privada.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Esse programa não tem pasta de configurações para usar.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Digite o nome de um site, como api.example.com, ou *.example.com para todos os sites sob esse nome.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Digite o nome de um computador, como nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Escreva cada uma como NOME=valor.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Escreva cada uma como NOME=valor, com letras maiúsculas.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = Um programa de IA está pedindo para usar esta pasta. Por enquanto ele não consegue vê-la.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = Um programa de IA está pedindo para executar isto como administrador. Ele rodaria fora da cerca e poderia ver tudo o que você vê.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Programa de IA

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = Deixar só ler esta pasta

# Note under the question about a folder.
store-fence-sees-next = Ele verá a pasta na próxima vez que o programa iniciar.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = Não

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = Compartilhar só nesta sessão

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = Sempre compartilhar

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = Não permitir

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = Permitir

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Esse comando é longo demais para ser mostrado inteiro, então não foi executado.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Outro pedido está esperando que você responda.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Você não permitiu isto.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } não está disponível dentro da cerca.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Não foi possível iniciá-lo fora da cerca.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Informe o caminho completo da pasta.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Você não compartilhou a pasta.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Compartilhada. Ela estará lá na próxima vez que este programa iniciar.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Informe o nome do computador a acessar.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Programas de IA dentro da cerca não podem acessar esse computador com ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = Não foi possível criar a chave para programas de IA: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Informe o comando a executar.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } não está disponível aqui.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } não está disponível dentro da cerca. Peça para executá-lo fora.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: a linha de pedidos da cerca não está lá.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: o pedido não foi enviado.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = O Landlock recusou: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = A cerca não iniciou: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = A cerca não iniciou: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Os programas de IA guardam aqui o próprio login, separado do seu. Entre dentro do programa quando ele pedir.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Suas pastas são privadas para os programas de IA. Compartilhe uma em Configurações, Programas de IA, para trabalhar nela.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = A cerca não iniciou: não foi possível criar o filtro de chamadas do sistema.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = A cerca não iniciou: não foi possível configurar o limite de rede.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = A cerca não iniciou: o bubblewrap não quis executar: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: não encontrado fora da cerca
