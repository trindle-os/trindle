
# machine translated (German), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Start bei der Anmeldung

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart start anmeldung login hochfahren automatisch verzögerung starten

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Apps, die sich von selbst öffnen, wenn du dich anmeldest. Mit einer Verzögerung kann eine App zuerst auf das Netzwerk oder eine andere App warten.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = Bei der Anmeldung startet nichts von selbst.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } startet bei der Anmeldung von selbst.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] { $n } App startet bei der Anmeldung von selbst.
       *[other] { $n } Apps starten bei der Anmeldung von selbst.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] Eine weitere ist ausgeschaltet gespeichert.
       *[other] { $n } weitere sind ausgeschaltet gespeichert.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Sofort

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Nach einer Minute

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Nach 2 Minuten

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] Nach { $n } Sekunde
       *[other] Nach { $n } Sekunden
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Verzögerung für { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Bei der Anmeldung starten

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Nicht mehr bei der Anmeldung starten

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Sie konnte nicht entfernt werden und startet deshalb weiterhin bei der Anmeldung.

# Red words under the add card when something could not be added.
settings-startup-not-added = Das konnte nicht hinzugefügt werden.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = App suchen

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Dies bei der Anmeldung starten

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Das konnte nicht hinzugefügt werden: Die App ist nicht mehr installiert.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = z. B. syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Ein Programm auf der Festplatte wählen

# Heading of the line for adding a command.
settings-startup-command = Befehl

# Small words under the command line.
settings-startup-command-about = So eingegeben wie in einem Terminal, oder ein Programm auf der Festplatte.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Gib einen Befehl ein oder wähle ein Programm.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Programm für den Start bei der Anmeldung wählen

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Nichts startet von selbst

# Small words under that line.
settings-startup-none-about = Füge daneben eine App hinzu.

# Title of the card listing what starts at login.
settings-startup-list-title = Startet bei der Anmeldung

# Words behind the i of the card listing what starts at login.
settings-startup-about = Jede App kann nach der Anmeldung eine Weile warten, bis das Netzwerk oder eine andere App bereit ist. Ihr Schalter hält sie hier, ohne sie zu starten; der Papierkorb verhindert den Start bei der Anmeldung ganz.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } führt aus: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Aus den installierten Apps wählen

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Befehl eingeben oder Programm wählen

# Heading of the line for adding an installed app.
settings-startup-app-heading = Eine installierte App

# Small words under that line.
settings-startup-app-about = Wähle eine aus der Liste der Apps.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Ein Befehl oder ein Programm

# Small words under that line.
settings-startup-cmd-about = Eingegeben oder von der Festplatte gewählt.

# Words behind the i of the Add an app card.
settings-startup-add-about = Wähle aus den installierten Apps oder gib einen Befehl an, so wie du ihn in einem Terminal eingeben würdest, oder wähle ein Programm von der Festplatte. Es startet bei der nächsten Anmeldung, und zwar sofort; eine Wartezeit kannst du festlegen, sobald es in der Liste steht.

# Title of the card for adding an app.
settings-startup-add-title = App hinzufügen
