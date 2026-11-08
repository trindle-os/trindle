
# machine translated (Spanish), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Iniciar al entrar

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = inicio automático arranque iniciar entrar sesión automáticamente lanzar retraso

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Aplicaciones que se abren solas al iniciar sesión. Un retraso permite que una aplicación espere antes a la red o a otra aplicación.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = No se inicia nada solo al iniciar sesión.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } se inicia sola al iniciar sesión.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] { $n } aplicación se inicia sola al iniciar sesión.
       *[other] { $n } aplicaciones se inician solas al iniciar sesión.
       [many] { $n } aplicaciones se inician solas al iniciar sesión.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] Se conserva una más, desactivada.
       *[other] Se conservan { $n } más, desactivadas.
       [many] Se conservan { $n } más, desactivadas.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = De inmediato

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Tras un minuto

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Tras 2 minutos

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] Tras { $n } segundo
       *[other] Tras { $n } segundos
       [many] Tras { $n } segundos
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Retraso para { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Iniciarla al entrar

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Dejar de iniciarla al entrar

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = No se pudo quitar, así que sigue iniciándose al entrar.

# Red words under the add card when something could not be added.
settings-startup-not-added = No se pudo añadir.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Buscar una aplicación

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Iniciar esto al entrar

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = No se pudo añadir: la aplicación ya no está instalada.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = p. ej. syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Elegir un programa del disco

# Heading of the line for adding a command.
settings-startup-command = Comando

# Small words under the command line.
settings-startup-command-about = Tal como se escribe en un terminal, o un programa del disco.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Escribe un comando o elige un programa.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Elige un programa para iniciar al entrar

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = No se inicia nada solo

# Small words under that line.
settings-startup-none-about = Añade una aplicación junto a esto.

# Title of the card listing what starts at login.
settings-startup-list-title = Se inicia al entrar

# Words behind the i of the card listing what starts at login.
settings-startup-about = Cada aplicación puede esperar un rato tras iniciar sesión, para que antes estén listas la red u otra aplicación. Su interruptor la mantiene aquí sin iniciarla; la papelera impide del todo que se inicie al entrar.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } ejecuta: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Elegir entre las aplicaciones instaladas

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Escribe un comando o elige un programa

# Heading of the line for adding an installed app.
settings-startup-app-heading = Una aplicación instalada

# Small words under that line.
settings-startup-app-about = Elige una de la lista de aplicaciones.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Un comando o un programa

# Small words under that line.
settings-startup-cmd-about = Escrito, o elegido del disco.

# Words behind the i of the Add an app card.
settings-startup-add-about = Elige entre las aplicaciones instaladas, o da un comando para ejecutar, tal como lo escribirías en un terminal, o un programa elegido del disco. Se inicia en el siguiente inicio de sesión, de inmediato; se puede elegir una espera cuando ya esté en la lista.

# Title of the card for adding an app.
settings-startup-add-title = Añadir una aplicación

