
# machine translated (French), not yet reviewed by a person.

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = Fermer

# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = Ouvrir

# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = Paramètres

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = Utilisation :

# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPTION…]

# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = Options :

# Help for the ring's --say option, printed in a terminal.
ring-cli-say = Faire ce que demandent ces mots, comme s'ils étaient prononcés devant l'anneau (pour essayer les commandes vocales sans microphone)

# Help for the ring's --background option, printed in a terminal.
ring-cli-background = Démarrer sans afficher, pour que la première ouverture soit instantanée

# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = Ranger toutes les fenêtres pour afficher le bureau, ou les rétablir (Super et D le demandent)

# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = Ouvrir l'anneau déjà à l'écoute, comme le peut un raccourci clavier

# Help for the ring's --all option, printed in a terminal.
ring-cli-all = Ouvrir l'anneau avec toutes les applications affichées autour

# Help for the ring's --power option, printed in a terminal.
ring-cli-power = Ouvrir l'anneau pour fermer la session, redémarrer et éteindre

# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = Ouvrir la page des favoris dans les paramètres

# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = Appliquer tout de suite une nouvelle apparence (clair ou sombre, ou un nouvel accent) depuis les paramètres

# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = Recréer les zones tactiles des coins du haut

# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = Déplacer le clavier dans les cercles de notification à l'écran (Super et N le demandent)

# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = option inconnue { $option }

# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = argument manquant pour --say

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = Aucune application ni aucun favori ne porte ce nom

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's menu that removes the app.
ring-uninstall = Désinstaller

# SAFETY: needs a human review before it ships
# Line that replaces the uninstall button while the app is being removed.
ring-uninstalling = Désinstallation en cours

# SAFETY: needs a human review before it ships
# Line that replaces the uninstall button once the app is removed.
ring-uninstalled = Désinstallée

# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = Retirer de l'anneau

# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = Retrait en cours

# Line that replaces that button once the app is off the ring.
ring-taken-off = Retirée

# Name of the ring's round button that opens Apps.
ring-tool-apps = Applications

# Name of the ring's round button that opens Activity.
ring-tool-activity = Activité

# Name of the ring's round button that opens Drives.
ring-tool-drives = Disques

# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = Capture d'écran

# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = Presse-papiers

# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = Notifications

# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = Loupe

# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = Pipette

# Name of the ring's round button that opens the report a problem window.
ring-tool-report = Signaler un problème

# Line in the ring when the item that was pressed is not there any more.
ring-gone = Cet élément n'existe plus.

# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Rien n'a pu l'ouvrir.

# Line in the ring while it listens to a spoken command.
ring-listening = À l'écoute... parlez, l'écoute s'arrête quand vous vous arrêtez

# Line in the ring while it works out what was said.
ring-working-out = Analyse en cours...

# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = Rien à fermer

# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = Il est { $time }, le { $date }

# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = La luminosité de cet écran ne peut pas être modifiée ici

# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = Le clavier à l'écran n'est pas installé

# Screen reader name of the ring of open windows.
ring-open-windows = Fenêtres ouvertes

# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = Restaurer

# Tip of the button on a window's menu that minimises it.
ring-window-minimise = Réduire

# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = Restaurer la taille

# Tip of the button on a window's menu that maximises it.
ring-window-maximise = Agrandir

# SAFETY: needs a human review before it ships
# Tip of the button on a window's menu that stops a program that does not answer.
ring-window-force-close = Forcer la fermeture

# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = Toutes les applications

# Name of the ring's round button that opens the power page.
ring-power = Alimentation

# Screen reader description of the ring's power button.
ring-power-spoken = Fermer la session, redémarrer ou éteindre

# Screen reader name of the ring's results while something is typed.
ring-finding = Recherche de { $typed }

# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = Date et heure

# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = Nouvelle fenêtre

# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
        [one] Fermer la fenêtre
        [many] Fermer les { $n } fenêtres
       *[other] Fermer les { $n } fenêtres
    }

# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = Détacher de l'anneau

# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = Retirer des favoris

# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = Ajouter aux favoris

# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = Non effectué

# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    Le centre peut en contenir { $n }.
    Retirez-en un pour faire de la place.

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: lock the screen.
ring-power-lock = Verrouiller

# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = Économiseur d'écran

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = Mettre en veille

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: hibernate.
ring-power-hibernate = Mettre en veille prolongée

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: log out.
ring-power-log-out = Fermer la session

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: restart the computer.
ring-power-restart = Redémarrer

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = Éteindre

# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = Cliquez sur Toutes les applications, puis faites un clic droit sur une application pour la garder ici

# SAFETY: needs a human review before it ships
# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = Seul un clic confirme ceci.

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = Vous seul pouvez éteindre cet ordinateur.

# SAFETY: needs a human review before it ships
# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = La page d'alimentation s'est fermée : un programme a appuyé sur une touche, et non vous.

# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = Rien d’ouvert à fermer

ring-startup-add = Lancer à l'ouverture de session

ring-startup-remove = Ne plus lancer à l'ouverture de session



# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = Affiche tous les raccourcis clavier dans une liste, ou range la liste (Super et K, sauf changement dans les Réglages)

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = Redémarré

# Under the notice above.
ring-restarted-why = L'anneau et les éléments d'aide ont redémarré.

