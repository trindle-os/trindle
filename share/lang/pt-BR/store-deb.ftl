
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Instalar um pacote

# Words at the top of the window while the package is read.
store-deb-reading = Lendo o pacote…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Este arquivo não é um pacote Debian que possa ser instalado. Talvez o download não tenha terminado.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } está pronto para ser instalado na própria pasta.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Este pacote é para computadores { $arch }, então não pode ser instalado neste.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Este é o próprio Trindle. Ele se atualiza pela página Atualizações de Aplicativos, não por um arquivo.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } é um driver ou um serviço em segundo plano, que não funciona a partir da própria pasta, então não pode ser instalado aqui.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Um pacote de fora do Debian e do Trindle é instalado na própria pasta, onde fica restrito, mas não totalmente isolado. Instale apenas pacotes de fabricantes em que você confia.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } está instalado na própria pasta.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Verificando de onde isto veio…

# Heading in the card that says where the file came from.
store-deb-where = De onde veio

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Remover o pacote e o que ele guardou

# Word beside the ring while the package goes in.
store-deb-installing = Instalando

# Main button that installs the package.
store-deb-install = Instalar

# Main button that installs the same version again.
store-deb-reinstall = Reinstalar

# Main button that brings the installed package up to this version.
store-deb-update = Atualizar

# Heading of the card for a package not installed yet.
store-deb-goes-in = Vai para a própria pasta, restrito por padrão

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = Não precisa de senha. Ele não acessa a internet, o som nem os seus arquivos até você permitir.

# Heading of the card for a package installed at this version.
store-deb-same-version = A versão { $version } está instalada na própria pasta

# Under that heading.
store-deb-same-version-note = Reinstale-o ali se ele parou de funcionar direito.

# Heading of the card for a package installed at another version.
store-deb-other-version = A versão { $version } está instalada na própria pasta

# Under that heading: the version this brings.
store-deb-other-version-note = Instalar isto ali o leva para a versão { $version }, mantendo o que ele guardou.

# Facts line: how big the package is once installed.
store-deb-fact-size = Tamanho depois de instalado

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Feito para

# Text of the About card when the package has no description.
store-deb-no-description = Este pacote não tem descrição.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Instalando na própria pasta. Buscar o que ele precisa pode levar alguns minutos.

# Words at the top of the window once the package is installed.
store-deb-done = Instalado na própria pasta. Você o encontra no anel.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Instalado na própria pasta, mas estas partes não foram encontradas, então ele pode não iniciar: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Removido da própria pasta, com tudo o que ele guardou.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Este arquivo não é um pacote Debian que possa ser instalado.

# Step shown while a package is unpacked.
store-boxed-unpacking = Descompactando { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = Adicionando { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } não é um pacote Debian que possa ser instalado.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = Não foi possível descompactar { $path }.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Esse programa não é mantido numa caixa.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal

# Title of the notice when older programs are off.
store-boxed-x-off = Programas antigos estão desligados

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Configurações, Permissões

# Title of the notice when too many older programs are open.
store-boxed-x-many = Programas antigos demais

# Body of that notice.
store-boxed-x-many-close = Feche um e tente de novo

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } não pôde abrir

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = O terminal alcança mais do seu computador do que os outros aplicativos, porque precisa disso para o seu trabalho. Digite apenas comandos que você entende, de pessoas em quem confia.

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = A rede doméstica está desligada

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } não tem rede: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = ele não encontrou o próprio programa

# The same: the unshare program is missing.
store-boxed-home-why-unshare = o unshare não está instalado

# The same: the pasta program is missing.
store-boxed-home-why-pasta = o pasta não está instalado

# The same: the network room was not made.
store-boxed-home-why-not-made = a sala de rede dele não foi criada

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = a sala de rede dele não era nova

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = o filtro de chamadas do sistema não pôde ser criado

# The same: the room's rules would not load.
store-boxed-home-why-rules = as regras da sala não carregaram

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } não iniciou: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = o pasta disse: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = o pasta não conseguiu entrar na sala


# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Aberto sem internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Foi baixado, então o programa recebe só esse arquivo, somente leitura.

