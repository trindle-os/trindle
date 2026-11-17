
# machine translated (German), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Das ist kein gültiger Programmname.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Das wurde nicht gespeichert.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Wähle einen Ordner, der existiert.

# Red words when the whole home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-whole-home = Der gesamte persönliche Ordner bleibt privat.

# Red words when a folder that holds the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-holds-home = Dieser Ordner enthält deinen persönlichen Ordner und bleibt deshalb privat.

# Red words when a folder outside the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-outside-home = Nur Ordner innerhalb deines persönlichen Ordners können geteilt werden.

# Red words when a hidden folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-hidden = Versteckte Ordner bleiben immer privat.

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = Dieser Ordner enthält Passwörter oder E-Mails und bleibt deshalb privat.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Dieses Programm hat keinen Einstellungsordner, den es nutzen könnte.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Gib den Namen einer Website ein, etwa api.example.com, oder *.example.com für alle Websites unter diesem Namen.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Gib den Namen eines Computers ein, etwa nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Schreibe jede als NAME=Wert.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Schreibe jede als NAME=Wert, mit Großbuchstaben.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = Ein KI-Programm möchte diesen Ordner verwenden. Es kann ihn im Moment nicht sehen.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = Ein KI-Programm möchte dies als Administrator ausführen. Es würde außerhalb der Absperrung laufen und könnte alles sehen, was du siehst.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = KI-Programm

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = Nur Lesen in diesem Ordner erlauben

# Note under the question about a folder.
store-fence-sees-next = Es sieht den Ordner beim nächsten Start des Programms.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = Nein

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = Nur für diese Sitzung teilen

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = Immer teilen

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = Nicht erlauben

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = Erlauben

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Dieser Befehl ist zu lang, um ganz angezeigt zu werden. Deshalb wurde er nicht ausgeführt.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Eine andere Anfrage wartet darauf, dass du antwortest.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Du hast das nicht erlaubt.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } ist innerhalb der Absperrung nicht verfügbar.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Es konnte nicht außerhalb der Absperrung gestartet werden.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Gib den vollständigen Pfad des Ordners an.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Du hast ihn nicht geteilt.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Freigegeben. Er ist beim nächsten Start dieses Programms da.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Nenne den Computer, zu dem verbunden werden soll.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = KI-Programme im Zaun können diesen Computer nicht per ssh erreichen.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = Der Schlüssel für KI-Programme konnte nicht erstellt werden: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Nenne einen Befehl zum Ausführen.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } ist hier nicht verfügbar.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } ist innerhalb der Absperrung nicht verfügbar. Bitte darum, es außerhalb auszuführen.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: Die Anfrageleitung der Absperrung ist nicht da.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: Die Anfrage wurde nicht gesendet.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock hat abgelehnt: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = Die Absperrung wurde nicht gestartet: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = Die Absperrung wurde nicht gestartet: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = KI-Programme behalten hier ihre eigene Anmeldung, getrennt von deiner. Melde dich im Programm an, wenn du gefragt wirst.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Deine Ordner sind für KI-Programme privat. Teile einen in den Einstellungen unter KI-Programme, um darin zu arbeiten.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = Die Absperrung wurde nicht gestartet: Der Systemaufruf-Filter konnte nicht erstellt werden.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = Die Absperrung wurde nicht gestartet: Die Netzwerkgrenze konnte nicht eingerichtet werden.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = Die Absperrung wurde nicht gestartet: bubblewrap wollte nicht laufen: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: außerhalb der Absperrung nicht gefunden
