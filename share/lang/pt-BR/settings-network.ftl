# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = O { $radio } está desligado por uma chave ou tecla no próprio computador. Ligue-o e tente de novo.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = O sistema está bloqueando o { $radio }, e o Trindle não conseguiu remover o bloqueio.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Essa senha não foi aceita. Tente de novo.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = A senha salva para esta rede não foi aceita. Use o botão da lixeira para esquecer a rede e depois entre nela de novo.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = O Trindle não encontrou essa rede. Confira o nome dela e se ela está ao alcance.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Este computador entrou na rede, mas não recebeu um endereço. Tente de novo ou reinicie o roteador.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Este computador não conseguiu entrar na rede. Chegue mais perto do roteador e tente de novo.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Isso não pode ser uma senha de Wi-Fi. Confira e tente de novo.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Não foi possível entrar na rede.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = abre com a sua senha

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = abre e altera com a sua senha

# Small line of a shared folder: anyone on the local network may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = qualquer pessoa na rede local pode abrir

# Small line of a shared folder: anyone on the local network may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = qualquer pessoa na rede local pode alterar

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = Parar de compartilhar { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Não gerenciado

# State of a network device that cannot be used.
settings-network-state-unavailable = Indisponível

# State of a network device that is not connected.
settings-network-state-disconnected = Desconectado

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Preparando

# State of a network device that is being set up.
settings-network-state-configuring = Configurando

# State of a network device that needs a password.
settings-network-state-needs-password = Precisa de senha

# State of a network device that is getting an address.
settings-network-state-address = Obtendo um endereço

# State of a network device whose connection is being checked.
settings-network-state-checking = Verificando

# State of a network device that is waiting.
settings-network-state-waiting = Aguardando

# State of a network device that is connected.
settings-network-state-connected = Conectado

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Desconectando

# State of a network device that failed.
settings-network-state-failed = Falhou

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Conectado por cabo e a { $name } por Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Conectado a { $name }. O sinal é forte.
        [good] Conectado a { $name }. O sinal é bom.
        [fair] Conectado a { $name }. O sinal é razoável.
       *[other] Conectado a { $name }. O sinal é fraco.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Conectado por cabo.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = O Wi-Fi está desligado, então este computador não está conectado.

# Headline of the page when not connected.
settings-network-top-none = Não conectado. Escolha uma rede abaixo.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Procurando redes…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = O Trindle não consegue mostrar as redes porque o NetworkManager não está em execução.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = O barramento do sistema

# Name of the page in the list of settings pages and its heading.
settings-network-title = Rede

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi sem fio internet ethernet cabo senha conectar firewall compartilhamento compartilhar impressora pasta

# Help text at the top of the page.
settings-network-help = Ligue ou desligue o Wi-Fi, entre em uma rede, esqueça uma rede que você não quer mais, ou deixe outros computadores se conectarem.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Sinal forte de { $name }.
        [good] Sinal bom de { $name }.
        [fair] Sinal razoável de { $name }.
       *[other] Sinal fraco de { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = O Wi-Fi precisa de um Linux mais novo.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Este computador não tem Wi-Fi funcionando.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Desligado. Ligue-o para entrar em uma rede.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Ligado, mas fora de uma rede.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Usar Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi e cabo

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = O Wi-Fi conecta este computador a uma rede sem cabo. Se você o desligar, ele para de procurar redes e economiza um pouco de bateria. Uma rede que esconde o nome nunca aparece na lista de redes ao alcance. Para entrar em uma, digite o nome exatamente, com as letras maiúsculas. Alguns chips Wi-Fi precisam de um driver que a versão do Linux usada pelo Trindle ainda não tem. Para esses, você precisa de uma versão mais nova do Linux. Um cabo de rede tem uma linha própria, com um botão para conectar ou desconectar.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Deixar outros computadores se conectarem

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Procurando pastas compartilhadas…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Não foi possível ler as pastas compartilhadas

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Ainda não acessível

# Small line under it.
settings-network-not-reachable-sub = Outros computadores não podem abri-las enquanto o compartilhamento estiver desligado.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Nenhuma pasta está compartilhada

# Heading of the Sharing card.
settings-network-sharing = Compartilhamento

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Enquanto Deixar outros computadores se conectarem estiver desligado, os outros computadores da rede só alcançam este se ele os tiver contatado antes. Isso impede que qualquer pessoa no mesmo Wi-Fi tente entrar. Ligue essa opção para compartilhar uma pasta ou impressora com outros computadores em casa. Para compartilhar uma pasta, clique nela com o botão direito no Folders e escolha Compartilhar na rede. Cada pasta compartilhada mostra quem pode fazer o quê. Com a sua senha: outros podem abrir os arquivos, ou abrir e alterá-los, se tiverem a sua senha de compartilhamento. Qualquer pessoa em casa: qualquer um na rede pode abrir os arquivos, ou abrir e alterá-los, sem senha. Enquanto a chave estiver desligada, os outros computadores em casa não podem abrir nenhuma delas.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Parando…

# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = Aguardando a sua senha…

# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = O compartilhamento está ligado

# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Bloqueados

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Não foi possível ligar o Wi-Fi.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Não foi possível desligar o Wi-Fi.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Desconectar

# Tip of the button that connects a network or a cable.
settings-network-connect = Conectar

# Shown under a network or cable while it is being connected.
settings-network-connecting = Conectando…

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Nenhuma rede ao alcance

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Redes ao alcance

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Todas as redes Wi-Fi que este computador consegue captar. Quando você abre a página, a rede que você está usando vem primeiro, depois as redes salvas, depois as demais pela força do sinal. A ordem não muda enquanto a página estiver aberta. Pressione a seta para entrar em uma rede. Uma rede protegida pede a senha na primeira vez. O botão da lixeira esquece uma rede salva e a senha dela. Uma rede aberta não é criptografada, então pessoas por perto poderiam ver o que você envia por ela.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Ligue o Wi-Fi para ver as redes ao alcance.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Salva

# Part of a network's small line: it needs a password.
settings-network-secured = Protegida

# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = Aberta, não criptografada

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Esquecer esta rede

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Não foi possível esquecer a rede.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Senha

# Tip of the button that joins the network.
settings-network-join = Entrar

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Cancelar

# Shown under a network while it is being joined.
settings-network-joining = Entrando…

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = As senhas de Wi-Fi têm pelo menos 8 caracteres.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Entrar em uma rede oculta

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Nome da rede

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Senha, se tiver

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Digite o nome da rede.

# Shown under a network after it was joined.
settings-network-joined = Conectado

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } A rede foi salva mesmo assim. Esqueça-a com o botão da lixeira antes de entrar de novo.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Não foi possível desconectar.

# Heading of the card that lists the ports opened by hand.
settings-network-ports = Portas abertas

# Help text (i button) of the Open ports card.
settings-network-ports-about = Toda porta fica fechada, a menos que este computador tenha contatado o outro antes. Um programa que espera conexões de outros, como um servidor de jogo ou uma ferramenta de sincronização, não pode ser alcançado até você abrir a porta dele aqui. Uma porta aberta deixa entrar qualquer computador da mesma rede local que este, mas nunca a internet toda. Em uma rede de café, hotel ou escritório, isso inclui todas as outras pessoas que estão lá. Para abrir ou fechar uma porta, você precisa da senha do administrador.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Nenhuma porta aberta

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = Fechar { $port }

# Heading of the line that opens a port.
settings-network-port-add = Abrir uma porta

# Placeholder in the box where the port number is typed.
settings-network-port-number = Número da porta

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Protocolo

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Ambos

# Button that opens the typed port.
settings-network-port-open = Abrir

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Digite um número de 1 a 65535.

# Red words when the port is already open.
settings-network-port-already = Já está aberta.

# Said under the line once the port is open.
settings-network-port-opened = Aberta

# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = Precisa de um nome de usuário

# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = Esta rede precisa de um nome de usuário além da senha. O Trindle ainda não consegue entrar nessas redes.
