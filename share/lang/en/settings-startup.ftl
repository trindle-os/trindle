# Settings page "Start at login": apps that open by themselves when someone logs in.

## Page (src/settings/panels/startup.rs)

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Start at login
# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay
# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Apps that open by themselves when you log in. You can add a delay so an app waits for the network or for another app first.

## Top sentence

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = No apps start by themselves when you log in.
# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } starts by itself when you log in.
# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] { $n } app starts by itself when you log in.
       *[other] { $n } apps start by themselves when you log in.
    }
# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] One more is switched off.
       *[other] { $n } more are switched off.
    }

## Delays

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = At once
# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = After a minute
# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = After 2 minutes
# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] After { $n } second
       *[other] After { $n } seconds
    }

## Each app's line

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Delay for { $name }
# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Start it at login
# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = Don't start it at login
# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Trindle couldn't remove it, so it still starts at login.

# Under the name of a launcher that comes with the system, which can only be switched on or off; { $command } is what it runs, cut short.
settings-startup-system = Comes with the system: { $command }

## Adding

# Red words under the add card when something could not be added.
settings-startup-not-added = Couldn't add it.
# Placeholder of the search box above the list of installed apps.
settings-startup-find = Find an app
# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Start this at login
# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Trindle couldn't add it, because the app is no longer installed.
# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = syncthing --no-browser
# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Choose a program
# Heading of the line for adding a command.
settings-startup-command = Command
# Red words under the command line when nothing was typed.
settings-startup-command-empty = Type a command, or choose a program.
# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Choose a program to start at login

## Cards

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Nothing starts by itself
# Title of the card listing what starts at login.
settings-startup-list-title = Starts at login
# Words behind the i of the card listing what starts at login.
settings-startup-about = Each app can wait a while after you log in, so the network or another app is ready first.
# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } runs: { $command }
# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Choose from the installed apps
# Name of the button that opens the box for typing a command.
settings-startup-open-command = Type a command, or choose a program
# Heading of the line for adding an installed app.
settings-startup-app-heading = An installed app
# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = A command or a program
# Words behind the i of the Add an app card.
settings-startup-add-about = Choose one of your installed apps, or type a command to run, as you would in a terminal. You can also choose a program on the disk. It starts at your next login. Once it is in the list, you can choose a delay for it.
# Title of the card for adding an app.
settings-startup-add-title = Add an app
