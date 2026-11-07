
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Permissões

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = permissões permitir privacidade flatpak flathub sandbox internet rede arquivos som microfone câmera bluetooth notificações plugins deb x11 xwayland programas antigos steam wine java tela terminal computador thunderbolt usb4 dock dispositivos

# SAFETY: needs a human review before it ships
# Help line at the foot of the Permissions page.
settings-permissions-help = O que cada aplicativo pode usar. Aplicativos do Flathub, programas instalados na própria pasta e plugins ficam cada um na sua pasta, com restrições por padrão. Um ícone preenchido está permitido; clique nele para mudar. Uma mudança em um aplicativo do Flathub vale na próxima vez que ele iniciar. Dispositivos Thunderbolt e USB4 permitidos sempre se conectam sem perguntar; esqueça um para ser perguntado de novo.

# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = Não foi possível mudar.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Salvo, mas o plugin não pôde ser iniciado de novo. A mudança vale na próxima vez que ele iniciar.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Salvo

# SAFETY: needs a human review before it ships
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = Cada AppImage roda em uma caixa própria, com um início privado. Um ícone preenchido está permitido; clique nele para mudar. A mudança vale na próxima vez que o AppImage iniciar.

# SAFETY: needs a human review before it ships
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Cada programa iniciado por Pastas roda em uma caixa própria, com um início privado. Um ícone preenchido está permitido; clique nele para mudar. A mudança vale na próxima vez que o programa iniciar.

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }

# SAFETY: needs a human review before it ships
# Under the Allow older programs switch.
settings-permissions-older-why = Cada um fica separado dos demais. Desligado, eles não abrem.

# SAFETY: needs a human review before it ships
# Heading of the card with the switch for older programs.
settings-permissions-older = Programas antigos

# SAFETY: needs a human review before it ships
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = Permitir programas antigos

# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = Estas portas só levam telas, então nenhum dispositivo alcança a memória deste computador e não há nada a perguntar.

# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Estas portas só levam USB e telas, então nenhum dispositivo alcança a memória deste computador e não há nada a perguntar.

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = Estas portas deixam todo dispositivo se conectar por conta própria, então não há nada a perguntar. Isso é definido no firmware do computador.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Estas portas só levam telas.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Estas portas só levam USB e telas.

# SAFETY: needs a human review before it ships
# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = O firmware deixa todo dispositivo se conectar.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Versão { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Versão { $v }. Não pede nada.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = Não foi possível esquecer.

# SAFETY: needs a human review before it ships
# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt e USB4

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = Dispositivos Thunderbolt e USB4 permitidos sempre se conectam sem perguntar; esqueça um para ser perguntado de novo. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = As portas

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Verificando o que elas deixam entrar…

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Dispositivos permitidos

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Lendo…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Memória mantida separada

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = Nenhuma porta Thunderbolt ou USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Então não há nada a perguntar.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Nenhum: nada se conecta deste modo.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Nada a perguntar

# SAFETY: needs a human review before it ships
# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = Todos, pelas próprias portas.

# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = Este computador também mantém os dispositivos longe da memória que não lhes foi dada, então o risco é menor. Esquecer um dispositivo o deixa conectado até ser desconectado.

# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = Esquecer um dispositivo o deixa conectado até ser desconectado; da próxima vez, será perguntado de novo.

# SAFETY: needs a human review before it ships
# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = Perguntado antes de um novo dispositivo se conectar

# Under the heading above.
settings-permissions-forget-one = Esqueça um para ser perguntado de novo.

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Não foi possível ler os dispositivos permitidos: { $why }

# SAFETY: needs a human review before it ships
# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = Nenhum dispositivo pode se conectar sem perguntar ainda.

# SAFETY: needs a human review before it ships
# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
settings-permissions-forget-device = Esquecer este dispositivo

# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = Um dispositivo Thunderbolt

# SAFETY: needs a human review before it ships
# Under an allowed device's name when its maker is not known.
settings-permissions-may-always = Pode sempre se conectar

# SAFETY: needs a human review before it ships
# Under an allowed device's name: the maker is { $vendor }.
settings-permissions-made-by = Fabricado por { $vendor }. Pode sempre se conectar.

# Headline while the list of apps is being read.
settings-permissions-reading-apps = Verificando o que cada aplicativo pode usar…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Verificando o que cada um dos { $n } pode usar…
        [many] Verificando o que cada um dos { $n } pode usar…
       *[other] Verificando o que cada um dos { $n } pode usar…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = Nenhum aplicativo ou plugin mantido em caixa própria está instalado.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Um aplicativo é mantido em caixa própria e usa só o que é permitido aqui.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } aplicativo ou plugin é mantido em caixa própria, usando só o que é permitido aqui.
        [many] { $n } aplicativos e plugins são mantidos cada um em caixa própria, usando só o que é permitido aqui.
       *[other] { $n } aplicativos e plugins são mantidos cada um em caixa própria, usando só o que é permitido aqui.
    }

# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Aplicativos do Flathub

# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = Na própria pasta

# Heading of the card for plugins.
settings-permissions-plugins = Plugins

# Heading of the card for AppImages.
settings-permissions-appimages = AppImages

# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Iniciados por Pastas

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = Nenhum instalado.

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = Nenhum executado ainda.

# SAFETY: needs a human review before it ships
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Cada aplicativo do Flathub é mantido em uma caixa própria e só pode usar o que é permitido aqui. Um ícone preenchido está permitido; clique nele para mudar. A mudança vale na próxima vez que o aplicativo iniciar.

# SAFETY: needs a human review before it ships
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = Programas instalados na própria pasta têm restrições por padrão. Um ícone preenchido está permitido; clique nele para mudar.

# SAFETY: needs a human review before it ships
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = Plugins têm restrições por padrão e só podem receber permissão para o que pedem. Um plugin que roda em segundo plano inicia de novo na hora com a mudança.

# Typing codes card: every short code Trindle swaps for words as you type.
# SAFETY: needs a human review before it ships
settings-permissions-codes = Códigos de digitação

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Códigos curtos que o Trindle troca por palavras enquanto você digita, cada um com o plugin que é dono dele. Desative os códigos de um plugin no ícone redondo dele.

settings-permissions-codes-none = Nenhum código está ativo

# SAFETY: needs a human review before it ships
settings-permissions-codes-none-sub = Um plugin com permissão para expandir códigos aparece aqui.

# Button on a plugin's row: stops that plugin's codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-off = Desativar os códigos de { $name }

settings-permissions-codes-count =
    { $n ->
        [one] Um código, { $id }
       *[other] { $n } códigos, { $id }
        [many] { $n } códigos, { $id }
    }

# A code's second line: what it types and who owns it.
settings-permissions-code-types = Digita "{ $words }" para { $name }

settings-permissions-codes-more =
    { $n ->
        [one] Mais um código está ativo
       *[other] Mais { $n } códigos estão ativos
        [many] Mais { $n } códigos estão ativos
    }

# SAFETY: needs a human review before it ships
# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
settings-permissions-codes-unread-line = Não foi possível ler a pasta de plugins

# SAFETY: needs a human review before it ships
# Red words under that line.
settings-permissions-codes-unread = Não foi possível ler a pasta de plugins, então alguns códigos podem estar faltando.

settings-permissions-not-added = Executado a partir de Folders, não adicionado a Apps

settings-permissions-kind-tip = Tipo de app



# Heading of the card for programs that are not in a box.
settings-outside-heading = Programas fora de uma caixa

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = O Trindle olha a cada poucos segundos e avisa você. Ele não pode impedir estes programas e pode deixar passar um que seja rápido. Um programa em uma caixa fica preso pela caixa.

# Label of the switch for the notice about the network.
settings-outside-network = Avisar quando um programa acessar a rede

# Label of the switch for the notice about the person's files.
settings-outside-files = Avisar quando um programa estiver com seus arquivos abertos

# Under those two switches.
settings-outside-once = Uma vez para cada programa.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Avisar de novo para { $name }

# Under a program marked fine for the network.
settings-outside-fine-network = Acessar a rede é permitido.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Ter os seus arquivos abertos é permitido.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Nenhum programa está marcado como permitido.

# Heading of the card for downloaded files.
settings-downloads-heading = Arquivos baixados

# Help text of that card (the information button).
settings-downloads-about = Um arquivo da internet é um que está em Downloads ou que o navegador marcou. O programa que o abre recebe só esse arquivo, somente leitura, sem internet e sem nenhuma das suas pastas. Isto não vale para o botão Abrir do próprio navegador, que inicia o programa por conta própria.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Abrir arquivos baixados em uma caixa

# Under that switch.
settings-downloads-box-sub = Sem internet, e só aquele arquivo, somente leitura.

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Abrir arquivos { $kind } normalmente

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Arquivos { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Ligado: abre normalmente, com a internet.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Nenhum tipo ainda. Um aparece quando um arquivo baixado abre em uma caixa.

