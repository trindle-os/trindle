# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Iniciar no login

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = inicialização iniciar login boot automaticamente executar atraso

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Aplicativos que abrem sozinhos quando você entra. Você pode adicionar um atraso para o aplicativo esperar antes pela rede ou por outro aplicativo.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = Nenhum aplicativo inicia sozinho quando você entra.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } inicia sozinho quando você entra.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] { $n } aplicativo inicia sozinho quando você entra.
        [many] { $n } de aplicativos iniciam sozinhos quando você entra.
       *[other] { $n } aplicativos iniciam sozinhos quando você entra.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] Mais um está desligado.
       *[other] Mais { $n } estão desligados.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Na hora

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Após um minuto

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Após 2 minutos

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] Após { $n } segundo
        [many] Após { $n } de segundos
       *[other] Após { $n } segundos
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Atraso para { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Iniciar no login

# Name of the bin button on an app's line that stops it starting at login.
# SAFETY: needs a human review before it ships
settings-startup-stop = Não iniciar no login

# Red words under an app's line when it could not be taken off the list.
# SAFETY: needs a human review before it ships
settings-startup-not-removed = O Trindle não conseguiu removê-lo, então ele ainda inicia no login.

# Under the name of a launcher that comes with the system, which can only be switched on or off; { $command } is what it runs, cut short.
settings-startup-system = Vem com o sistema: { $command }

# Red words under the add card when something could not be added.
settings-startup-not-added = Não foi possível adicioná-lo.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Procurar um aplicativo

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Iniciar isto no login

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = O Trindle não conseguiu adicioná-lo, porque o aplicativo não está mais instalado.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Escolher um programa

# Heading of the line for adding a command.
settings-startup-command = Comando

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Digite um comando ou escolha um programa.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Escolher um programa para iniciar no login

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Nada inicia sozinho

# Title of the card listing what starts at login.
settings-startup-list-title = Inicia no login

# Words behind the i of the card listing what starts at login.
settings-startup-about = Cada aplicativo pode esperar um pouco depois que você entra, para que a rede ou outro aplicativo fique pronto antes.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } executa: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Escolher entre os aplicativos instalados

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Digitar um comando ou escolher um programa

# Heading of the line for adding an installed app.
settings-startup-app-heading = Um aplicativo instalado

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Um comando ou um programa

# Words behind the i of the Add an app card.
settings-startup-add-about = Escolha um dos seus aplicativos instalados, ou digite um comando para executar, como faria em um Terminal. Você também pode escolher um programa no disco. Ele inicia no próximo login. Depois que estiver na lista, você pode escolher um atraso para ele.

# Title of the card for adding an app.
settings-startup-add-title = Adicionar um aplicativo
