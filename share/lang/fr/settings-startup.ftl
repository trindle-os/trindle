# machine translated (French), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Démarrage à la connexion

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = démarrage automatique autostart connexion session lancer lancement amorçage retard délai

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Les applications qui s'ouvrent d'elles-mêmes quand vous ouvrez votre session. Un délai permet à une application d'attendre d'abord le réseau ou une autre application.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = Rien ne démarre tout seul quand vous ouvrez votre session.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } démarre tout seul quand vous ouvrez votre session.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] { $n } application démarre toute seule quand vous ouvrez votre session.
       *[other] { $n } applications démarrent toutes seules quand vous ouvrez votre session.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] Une de plus est gardée, désactivée.
       *[other] { $n } de plus sont gardées, désactivées.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Tout de suite

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Après une minute

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Après 2 minutes

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] Après { $n } seconde
       *[other] Après { $n } secondes
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Délai pour { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = La démarrer à la connexion

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Ne plus la démarrer à la connexion

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Elle n'a pas pu être retirée, elle démarre donc toujours à la connexion.

# Red words under the add card when something could not be added.
settings-startup-not-added = Elle n'a pas pu être ajoutée.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Chercher une application

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Démarrer ceci à la connexion

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Elle n'a pas pu être ajoutée : l'application n'est plus installée.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = p. ex. syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Choisir un programme sur le disque

# Heading of the line for adding a command.
settings-startup-command = Commande

# Small words under the command line.
settings-startup-command-about = Telle que saisie dans un Terminal, ou un programme du disque.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Saisissez une commande, ou choisissez un programme.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Choisir un programme à démarrer à la connexion

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Rien ne démarre tout seul

# Small words under that line.
settings-startup-none-about = Ajoutez une application à côté.

# Title of the card listing what starts at login.
settings-startup-list-title = Démarre à la connexion

# Words behind the i of the card listing what starts at login.
settings-startup-about = Chaque application peut attendre un moment après votre connexion, que le réseau ou une autre application soit prêt. Son interrupteur la garde ici sans la démarrer ; la corbeille l'empêche tout à fait de démarrer à la connexion.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } exécute : { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Choisir parmi les applications installées

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Saisir une commande, ou choisir un programme

# Heading of the line for adding an installed app.
settings-startup-app-heading = Une application installée

# Small words under that line.
settings-startup-app-about = Choisissez-en une dans la liste des applications.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Une commande ou un programme

# Small words under that line.
settings-startup-cmd-about = Saisi, ou choisi sur le disque.

# Words behind the i of the Add an app card.
settings-startup-add-about = Choisissez parmi les applications installées, ou donnez une commande à exécuter, telle que vous la saisiriez dans un Terminal, ou un programme choisi sur le disque. Elle démarre à la prochaine connexion, tout de suite ; un délai peut être choisi une fois qu'elle figure dans la liste.

# Title of the card for adding an app.
settings-startup-add-title = Ajouter une application
