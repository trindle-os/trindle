
# machine translated (German), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Das ist kein gültiger Anmeldename.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Dieser Anmeldename ist zu lang.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Es gibt keinen Benutzer namens { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } ist ein Systemkonto, deshalb hat Trindle es nicht geändert.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Es wurde kein Passwort angegeben.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] Das Passwort braucht mindestens { $n } Zeichen.
       *[other] Das Passwort braucht mindestens { $n } Zeichen.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Ein Passwort darf keinen Zeilenumbruch enthalten.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } gibt es bereits.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } ist der einzige Administrator und muss es bleiben.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Du kannst dein eigenes Konto nicht löschen.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = Trindle hat das Konto von { $name } gelöscht, konnte aber das Bild für den Anmeldebildschirm nicht entfernen: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Dieses Bild ist zu groß.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Das ist kein Bild, das Trindle erstellt hat.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle konnte dem Anmeldebildschirm keinen Zugriff auf den Bilderordner geben: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Das ist kein Sprachname.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle konnte die Sprachliste von Debian nicht lesen: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } ist keine Sprache, die Debian kennt.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Ein Computername darf nur Buchstaben, Ziffern und Bindestriche enthalten. Er darf höchstens 63 Zeichen lang sein und nicht mit einem Bindestrich beginnen oder enden.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = Die Ladegrenze ist vorerst aktiv, geht aber beim Ausschalten des Computers verloren: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Es wird vorerst voll geladen, aber die Grenze kommt beim nächsten Start des Computers zurück: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Andere Computer können sich weiterhin verbinden, weil Trindle die Freigabe nicht ausschalten konnte: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Das ist keine Plugin-ID.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Diese Person gibt es nicht.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Für diese Person gibt es keinen Ordner.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle konnte die Liste der erlaubten Plugins nicht speichern: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Dieses Plugin gibt es nicht.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Nichts wurde aktualisiert. Trindle konnte nicht feststellen, welche Version von Trindle installiert ist, und konnte deshalb das eigene Update nicht ausnehmen.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Updates werden installiert

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Es wird aufgeräumt

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle wird installiert

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Das ist kein Paketname.

# Red words when the package to remove is not installed.
admin-package-not-installed = Dieses Paket ist nicht installiert.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Das ist kein Gerät, das fwupd kennt.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle weiß nicht, wer Tailscale will.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Die Sperre wurde nicht rechtzeitig gestartet.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Die Sperre ist nicht ganz aus: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle konnte { $path } nicht zurücksetzen: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Das ist kein Seitenname.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Diesen Server gibt es nicht.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } wurde beendet, bevor es alles hatte, was es braucht, daher ist die Aufgabe nicht abgeschlossen: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } ist fehlgeschlagen.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } ließ sich nicht entfernen: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Nichts wurde geändert, weil Trindle { $path } nicht lesen konnte: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle konnte auch den früheren Stand nicht wiederherstellen: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Das ist keine gültige Zeitspanne für das Aufbewahren früherer Versionen.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Die früheren Versionen wurden nicht gefunden.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Dort, wo der Ordner für frühere Versionen sein sollte, liegt eine Datei.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Es ist unklar, wer gefragt hat.

# Red words when a step that must go through the administrator's password was started another way.
# SAFETY: needs a human review before it ships
admin-pkexec-only = Das läuft nur über pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Das ist kein Computername.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Das ist kein Name eines freigegebenen Ordners.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Das ist keine Computeradresse.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Das ist kein Pfad eines freigegebenen Ordners.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Hier können nur deine eigenen freigegebenen Ordner geschlossen werden.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Dort ist kein freigegebener Ordner geöffnet.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } wurde verändert, seit Trindle es eingerichtet hat, daher ist dort nichts verbunden.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } ist nicht mehr so, wie Trindle es hinterlassen hat.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Es sind zu viele freigegebene Ordner geöffnet.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } ist nicht installiert. Es gehört zum Debian-Paket { $package }.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Die Anmeldung ließ sich nicht lesen.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle konnte nicht lesen, was auf diesem Computer eingehängt ist: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Ein Programm verwendet noch etwas in diesem Ordner. Schließe das Programm und versuche es dann erneut.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Dieser Computer antwortet nicht. Der freigegebene Ordner wird beim Abmelden geschlossen.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Das ist keine Benutzer-ID.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Die Gruppe sambashare ließ sich nicht anlegen.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba wird geholt

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Die Freigabe wird eingerichtet

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = Die Freigabe ist noch nicht eingerichtet. Gib zuerst einen Ordner frei.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Dieser Computer ist für andere im Netzwerk weiterhin sichtbar: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Das ist keine Schnappschussnummer.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = In den Einstellungen erstellt

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle konnte nicht feststellen, auf welchem Datenträger das System liegt.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = Das System liegt nicht in einem eigenen Btrfs-Subvolume, daher kann Trindle es nicht gegen einen Schnappschuss tauschen.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle konnte den Signaturschlüssel nicht von { $url } holen.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Diese Adresse hat keinen Signaturschlüssel zurückgeschickt.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Nichts wurde geändert, weil Trindle { $path } nicht lesen konnte: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Der Signaturschlüssel wird geholt

# Second line of the progress while a source is added.
admin-step-adding-source = Sie wird hinzugefügt und ihre Liste gelesen

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Die Quelle ließ sich nicht verwenden. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle konnte die Quelle nicht verwenden und hat sie wieder entfernt. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } kann es noch nicht ein- oder ausschalten: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle konnte es für { $user } nicht starten: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = Trindle konnte { $user } nicht erlauben, virtuelle Maschinen zu nutzen, die Cowork braucht: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = Trindle konnte { $user } nicht erlauben, es ohne Passwort zu nutzen: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } ist installiert, aber Trindle konnte die Einrichtung nicht abschließen: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Es wird installiert

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Das ist keine der vorgefertigten Quellen.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } ist nicht für diese Art von Computer gemacht ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Es wird eingerichtet

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Jede Adresse muss eine einfache Webadresse sein, die mit https:// beginnt.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = Suite und Komponenten müssen einfache Wörter sein.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Das ist kein Paketname.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Es wird entfernt

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Die Listen werden neu gelesen

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Hier können nur Quellen entfernt werden, die Trindle hinzugefügt hat.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = Die Quelle ist noch vorhanden: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = Schnappschuss { $n } ist nicht auf dem Datenträger.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Nichts wurde geändert, weil Trindle den Schnappschuss nicht vorbereiten konnte. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle konnte das aktuelle System nicht beiseitelegen: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = Das aktuelle System heißt jetzt { $aside } statt { $sub }, und der Computer startet erst wieder, wenn es erneut { $sub } heißt: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle konnte die Schnappschüsse nicht übertragen: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle konnte den Schnappschuss nicht einspielen: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle konnte die Liste der beiseitegelegten Systeme nicht aktualisieren: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Nicht entfernt: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Trindle hat diese entfernt, weil sie nicht mehr gebraucht wurden: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Nicht alle Updates wurden installiert. Diese wurden zurückgehalten: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } ist aus, weil sich der Treiber ({ $driver }, DKMS) noch nicht für den neuen Linux-Kernel bauen lässt.

admin-driver-off = Der Treiber ({ $driver }, DKMS) lässt sich noch nicht für den neuen Linux-Kernel bauen. Funktionen, die ihn brauchen, sind aus.

# Said once after the drivers that are off.
admin-driver-off-rest = Der Kernel und der Rest des Updates wurden eingespielt.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = Die virtuelle Kamera

admin-language-in-use = { $code } ist die Sprache, in der dieser Computer startet, daher kann sie nicht entfernt werden.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Dieser Computer hat keine Split-Lock-Verlangsamung, die sich abschalten ließe.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle hat dieses Laufwerk nicht gefunden, oder es ist das Systemlaufwerk.

# The drive could not be closed where it was open.
admin-games-drive-busy = Das Laufwerk wird gerade benutzt. Schließe alle Dateien und Programme darauf und versuche es dann erneut: { $why }

# Another software change has the read-only system open.
admin-system-busy = Es läuft gerade eine andere Softwareänderung. Versuche es erneut, wenn sie abgeschlossen ist.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Es wurde nichts geändert, weil Trindle das System nicht entsperren konnte: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle konnte das System nicht wieder sperren. Es wird beim nächsten Start gesperrt: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle konnte /usr/local nicht beschreibbar halten: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Das kann nur root ausführen, und nicht über pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Dafür wird Btrfs benötigt.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = Der Schnappschuss ist eingespielt, und der Computer startet davon. Trindle konnte das Aufräumen des alten Systems nicht abschließen. Es versucht es beim nächsten Start des Computers erneut: { $why }


# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = Du kannst höchstens { $most } Ports öffnen.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle konnte die Liste der offenen Ports nicht speichern: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = Die Firewall ist noch eingeschaltet: { $why }

admin-plugin-ports-changed = Dieses Plugin fordert jetzt andere Netzwerk-Ports an als die, die du erlaubt hast. Erlaube es noch einmal, damit es sie öffnen darf.

admin-source-not-you = Ein Programm kann nur für die Person hinzugefügt werden, die darum bittet.

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = Eine andere Wiederherstellung hat vor einem Moment denselben Namen benutzt. Warte ein paar Sekunden und versuche es noch einmal.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } konnte nicht getrennt werden: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 freigegebener Ordner hat nicht geantwortet und ist deshalb noch verbunden. Er wird getrennt, wenn der Computer herunterfährt.
       *[other] { $n } freigegebene Ordner haben nicht geantwortet und sind deshalb noch verbunden. Sie werden getrennt, wenn der Computer herunterfährt.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = Die Auslagerungsdatei { $path } ließ sich nicht ausschalten: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = Die Auslagerungsdatei { $path } ließ sich nicht wieder einschalten: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } gehört nicht zu den Auslagerungsdateien des Systems.

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = Trindle nutzt die Auslagerungsdatei { $path } nicht, weil andere { $why } ändern oder lesen könnten.

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = Der Ort der Auslagerungsdatei wurde nicht gefunden. Das Aufwachen aus dem Ruhezustand klappt vielleicht nicht.

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = Der Ruhezustand funktioniert nach einem Neustart wieder.
