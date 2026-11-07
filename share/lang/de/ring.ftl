# machine translated (German), not yet reviewed by a person.

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = Schließen

# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = Öffnen

# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = Nichts zum Schließen offen

# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = Einstellungen

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = Verwendung:

# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPTION…]

# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = Optionen:

# Help for the ring's --say option, printed in a terminal.
ring-cli-say = Tue, was diese Worte verlangen, als wären sie dem Ring gesprochen worden (um Sprachbefehle ohne Mikrofon auszuprobieren)

# Help for the ring's --background option, printed in a terminal.
ring-cli-background = Ohne Anzeige starten, damit das erste Öffnen sofort geht

# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = Alle Fenster wegräumen, um den Desktop zu zeigen, oder sie zurückholen (Super und D verlangt dies)

# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = Den Ring schon zuhörend öffnen, wie es ein Tastenkürzel kann

# Help for the ring's --all option, printed in a terminal.
ring-cli-all = Den Ring mit allen Apps rundherum öffnen

# Help for the ring's --power option, printed in a terminal.
ring-cli-power = Den Ring bei Abmelden, Neustart und Herunterfahren öffnen

# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = Die Favoritenseite in den Einstellungen öffnen

# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = Ein neues Aussehen (hell oder dunkel, oder eine neue Akzentfarbe) aus den Einstellungen sofort übernehmen

# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = Die Berührungspunkte in den oberen Ecken neu erstellen

# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = Die Tastatur in die Benachrichtigungskreise auf dem Bildschirm verlegen (Super und N verlangt dies)

# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = unbekannte Option { $option }

# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = fehlendes Argument für --say

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = Keine App und kein Favorit heißt so

# Tip of the round button on an app's menu that removes the app.
# SAFETY: needs a human review before it ships
ring-uninstall = Deinstallieren

# Line that replaces the uninstall button while the app is being removed.
# SAFETY: needs a human review before it ships
ring-uninstalling = Wird deinstalliert

# Line that replaces the uninstall button once the app is removed.
# SAFETY: needs a human review before it ships
ring-uninstalled = Deinstalliert

# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = Vom Ring nehmen

# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = Wird entfernt

# Line that replaces that button once the app is off the ring.
ring-taken-off = Entfernt

# Name of the ring's round button that opens Apps.
ring-tool-apps = Apps

# Name of the ring's round button that opens Activity.
ring-tool-activity = Aktivität

# Name of the ring's round button that opens Drives.
ring-tool-drives = Laufwerke

# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = Bildschirmfoto

# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = Zwischenablage

# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = Benachrichtigungen

# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = Lupe

# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = Farbpipette

# Name of the ring's round button that opens the report a problem window.
ring-tool-report = Problem melden

# Line in the ring when the item that was pressed is not there any more.
ring-gone = Das ist nicht mehr da.

# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Nichts konnte das öffnen.

# Line in the ring while it listens to a spoken command.
ring-listening = Hört zu... sprich, es hört auf, wenn du aufhörst

# Line in the ring while it works out what was said.
ring-working-out = Wird herausgefunden...

# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = Nichts zum Schließen

# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = Es ist { $time } am { $date }

# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = Die Helligkeit dieses Bildschirms kann von hier aus nicht geändert werden

# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = Die Bildschirmtastatur ist nicht installiert

# Screen reader name of the ring of open windows.
ring-open-windows = Offene Fenster

# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = Wiederherstellen

# Tip of the button on a window's menu that minimises it.
ring-window-minimise = Minimieren

# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = Größe wiederherstellen

# Tip of the button on a window's menu that maximises it.
ring-window-maximise = Maximieren

# Tip of the button on a window's menu that stops a program that does not answer.
# SAFETY: needs a human review before it ships
ring-window-force-close = Beenden erzwingen

# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = Alle Apps

# Name of the ring's round button that opens the power page.
ring-power = Energie

# Screen reader description of the ring's power button.
ring-power-spoken = Abmelden, neu starten oder herunterfahren

# Screen reader name of the ring's results while something is typed.
ring-finding = { $typed } wird gesucht

# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = Datum und Uhrzeit

# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = Neues Fenster

# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
        [one] Fenster schließen
       *[other] Alle { $n } Fenster schließen
    }

# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = Vom Ring lösen

# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = Aus den Favoriten entfernen

# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = Zu den Favoriten hinzufügen

# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = Nicht erledigt

# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    Die Mitte fasst { $n }.
    Entferne einen, um Platz zu schaffen.

# Choice on the ring's power page: lock the screen.
ring-power-lock = Sperren

# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = Bildschirmschoner

# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = Ruhezustand

# Choice on the ring's power page: hibernate.
ring-power-hibernate = Tiefschlaf

# Choice on the ring's power page: log out.
ring-power-log-out = Abmelden

# Choice on the ring's power page: restart the computer.
ring-power-restart = Neu starten

# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = Herunterfahren

# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = Klicke auf „Alle Apps“, dann mit der rechten Maustaste auf eine App, um sie hier zu behalten

# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = Nur ein Klick bestätigt dies.

# Line in the ring when something other than the person tried to shut the computer down.
# SAFETY: needs a human review before it ships
ring-power-only-own = Nur du kannst diesen Computer herunterfahren.

# Line in the ring when the power page closed because a program pressed a key.
# SAFETY: needs a human review before it ships
ring-power-closed = Die Energieseite wurde geschlossen: Ein Programm hat eine Taste gedrückt, nicht du.

ring-startup-add = Bei der Anmeldung starten

ring-startup-remove = Nicht mehr bei der Anmeldung starten



# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = Alle Tastenkürzel in einer Liste zeigen oder die Liste wegräumen (Super und K, sofern in den Einstellungen nicht geändert)

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = Neu gestartet

# Under the notice above.
ring-restarted-why = Der Ring und die Hilfsteile sind neu gestartet.

