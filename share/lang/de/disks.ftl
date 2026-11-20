# machine translated (German), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Laufwerke

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Über diese Seite

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Hier siehst du jedes Laufwerk, das im Computer steckt oder angesteckt ist, als Balken, der in seine Partitionen geteilt ist. Die Breite einer Partition zeigt ihre Größe, die Schattierung zeigt, wie voll sie ist, sodass eine volle leicht auffällt.

# Second paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-2 = Unter dem Bild hat jedes Laufwerk eine Karte. Du kannst eine Partition öffnen, ein- oder aushängen, ein Laufwerk sicher entfernen oder ausschalten. Mit dem Stift an einer Partition kannst du sie umbenennen, ihren Ordner wählen, sie beim Start einhängen, ihre Größe ändern, sie prüfen, reparieren, formatieren oder löschen. Das Plus im freien Platz erstellt eine neue Partition.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = Du siehst das Laufwerk, von dem das System läuft, kannst daran aber nichts ändern, was etwas löscht oder verschiebt. Jede Änderung an jedem Laufwerk fragt jedes Mal nach dem Passwort eines Administrators. Das Auge blendet ein Laufwerk oder eine Partition in Folders und im Ring aus. Es bleibt hier, ausgegraut, und ist nicht gesperrt.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Deine Laufwerke

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Die Laufwerke lassen sich gerade nicht lesen.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Erneut versuchen

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Erneut suchen

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Es wurden keine Laufwerke gefunden. Stecke eines an, dann erscheint es hier.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Der Laufwerksdienst

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Der Systembus

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } ist nicht verfügbar. Zum Installieren führe aus: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Die Laufwerke werden angesehen…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } fällt aus. Kopiere jetzt alle Dateien, die du behalten willst, auf ein anderes Laufwerk.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } braucht einen Blick: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Deine Laufwerke und der Platz darauf.
        [one] Deine Laufwerke und der Platz darauf, mit einem angesteckten Laufwerk.
       *[other] Deine Laufwerke und der Platz darauf, mit { $plugged } angesteckten Laufwerken.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Dieser Computer ist fast voll, nur noch { $free } sind frei. Lösche oder verschiebe bald einige Dateien.
        [one] Dieser Computer ist fast voll, nur noch { $free } sind frei. Ein Laufwerk ist angesteckt. Lösche oder verschiebe bald einige Dateien.
       *[other] Dieser Computer ist fast voll, nur noch { $free } sind frei. { $plugged } Laufwerke sind angesteckt. Lösche oder verschiebe bald einige Dateien.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Dieser Computer wird voll, { $free } sind noch frei.
        [one] Dieser Computer wird voll, { $free } sind noch frei. Ein Laufwerk ist angesteckt.
       *[other] Dieser Computer wird voll, { $free } sind noch frei. { $plugged } Laufwerke sind angesteckt.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Auf diesem Computer ist viel Platz, { $free } sind frei.
        [one] Auf diesem Computer ist viel Platz, { $free } sind frei. Ein Laufwerk ist angesteckt.
       *[other] Auf diesem Computer ist viel Platz, { $free } sind frei. { $plugged } Laufwerke sind angesteckt.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Dieser Computer

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Deine Dateien

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] Noch { $free } frei auf der Platte, auf der das System liegt.
       *[files] Noch { $free } frei für deine eigenen Dateien.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } voll, { $free } frei von { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = von { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Die Laufwerke haben noch keine Gesundheitswerte gemeldet

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Gesund, laut Laufwerk selbst

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] Gesund, laut eigener Prüfung des Laufwerks
       *[other] Alle { $n } Laufwerke haben ihre eigenen Prüfungen bestanden
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } braucht einen Blick: siehe seine Karte
       *[other] { $n } brauchen einen Blick: siehe ihre Karten
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 Laufwerk, insgesamt { $total }. { $health }.
       *[other] { $n } Laufwerke, insgesamt { $total }. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } frei

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Nicht eingehängt

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Freier Platz

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Keine Disc oder Karte darin

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Noch nichts darauf

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB oder austauschbar

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, in diesem Computer

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB oder austauschbar, ausgeblendet

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, in diesem Computer, ausgeblendet

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } voll, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Partition

# Name given to a drive that gives no maker or model.
disks-drive-name = Laufwerk

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Auslagerung

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Verschlüsselt, entsperrt

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Verschlüsselt, gesperrt

# Kind of a part that holds other parts.
disks-kind-container = Enthält Partitionen

# Kind of a part that has no filesystem on it.
disks-kind-none = Kein Dateisystem

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, die Partition, von der ein Computer startet
        [legacy] { $kind }, die Partition, von der ein älterer Computer startet
       *[loader] { $kind }, die eigene Partition eines Bootloaders
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = jeder Computer

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = nur Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = alte Geräte

# Trouble a drive reports about its spare space.
disks-trouble-spare = der Reserveplatz geht aus

# Trouble a drive reports about its heat.
disks-trouble-temperature = zu heiß

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = verschleißt

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = ist nur noch lesbar

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = seine Zwischenspeichersicherung ist ausgefallen

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = ein Teil davon ist nur noch lesbar

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = über die zu erwartende Lebensdauer hinaus abgenutzt

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } beschädigte Stelle
       *[other] { $n } beschädigte Stellen
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] seit { $hours } Stunde in Gebrauch
       *[other] seit { $hours } Stunden in Gebrauch
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Fällt aus. Kopiere jetzt alles, was du behalten willst, an einen anderen Ort

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Braucht Aufmerksamkeit: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Gesund

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Keine Antwort

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Das Gehäuse des Laufwerks ist verbunden, aber das Laufwerk darin antwortet nicht. Ziehe es ab, warte zehn Sekunden und stecke es mit seinem eigenen Kabel an einen blauen oder USB-C-Anschluss. Bleibt es so, hat sich das Laufwerk darin vielleicht gelöst oder ist ausgefallen.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Sicher entfernen

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Ausschalten

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Das ganze Laufwerk löschen

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Dieses Laufwerk anzeigen
       *[partition] Diese Partition anzeigen
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Dieses Laufwerk ausblenden. { $lock }
       *[partition] Diese Partition ausblenden. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Vor Folders und dem Ring ausgeblendet. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = Das System läuft davon, daher kann hier nichts gelöscht werden.

# Note on the card of a drive that has no partition table.
disks-no-table = Dieses Laufwerk hat keine Partitionstabelle, daher ist es ganz frei.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = nicht eingehängt

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = wird beim Start in { $dir } eingehängt

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = erscheint in { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = in { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Öffnen

# Tip of the round button that mounts a part.
disks-mount = Einhängen

# Tip of the round button that unmounts a part.
disks-unmount = Aushängen

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Ändern

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Name

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Name (optional)

# Tip of the button that saves a part's new name.
disks-rename-tip = Umbenennen

# Note under the Name row of a part.
disks-name-note = Wie sie hier und auf anderen Computern heißt.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Ordnername

# Tip of the button that saves a part's folder.
disks-use-folder = Diesen Ordner verwenden

# Label of the row where a part's folder is chosen.
disks-folder = Ordner

# Note under the Folder row of a part.
disks-folder-note = Wo ihre Dateien erscheinen. Lasse es leer, damit der Computer wählt.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Beim Start einhängen

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Hängt sie ein, sobald der Computer startet. Fehlt sie, startet der Computer trotzdem.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Größe in GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = Von { $least } bis { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = Von { $least } bis { $most }. Hänge sie ein, um zu sehen, wie klein sie werden kann.

# Tip of the button that resizes a part.
# SAFETY: needs a human review before it ships
disks-resize-tip = Größe ändern

# Label of the row where a part is resized.
disks-size = Größe

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Dies löscht alles auf { $words }. Das lässt sich nicht rückgängig machen.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Klicke noch einmal, um die Größe von { $name } auf { $drive } zu ändern, von { $from } auf { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Platz für eine neue Partition

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Platz in GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Platz schaffen

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Klicke noch einmal, um { $name } auf { $drive } um { $size } zu verkleinern. Trindle erstellt vorher einen Schnappschuss.

# Note under the row that makes room.
disks-room-note = Trindle erstellt einen Schnappschuss und verkleinert dann die Partition im Betrieb. Der frei gewordene Platz bleibt leer.

# Tip of the button that checks a part for faults.
disks-check-tip = Auf Fehler prüfen

# Tip of the button that repairs a part.
# SAFETY: needs a human review before it ships
disks-repair-tip = Reparieren

# Label of the row where a part is checked and repaired.
disks-faults = Fehler

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Klicke noch einmal, um { $name } auf { $drive } zu reparieren

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Art

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = Formatieren

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Formatieren

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Klicke noch einmal, um alles auf { $words } zu löschen

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, auf { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = { $name }, { $size }, auf { $drive } löschen

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Partition löschen

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Hier eine neue Partition erstellen

# Label of the row where a new part is made.
disks-new-part = Neue Partition

# Tip of the button that makes a new part.
disks-make-tip = Erstellen

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = Klicke noch einmal, um auf { $drive } eine neue Partition mit { $size } zu erstellen

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = { $name } löschen

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, auf { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Neue Partition

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } frei, auf { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Schließen

# Words on a row while a part is being mounted.
disks-mounting = Wird eingehängt

# Words on a row when a part has been mounted.
disks-mounted = Eingehängt

# Words on a row while a part is being unmounted.
disks-unmounting = Wird ausgehängt

# Words on a row when a part has been unmounted.
disks-unmounted = Ausgehängt

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Wird geschlossen

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Kann sicher entfernt werden

# Words on a drive's row while it is being powered off.
disks-powering-off = Wird ausgeschaltet

# Words on a drive's row when it has been powered off.
disks-off-safe = Kann sicher abgesteckt werden

# Words on a row while a part is being renamed.
disks-renaming = Wird umbenannt

# Words on a row when a part has been renamed.
disks-renamed = Umbenannt

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = Der Ordner wird jetzt für dich gewählt

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Sie erscheint in { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Wird beim Start in { $dir } eingehängt

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Beim Start nicht eingehängt

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Wird gespeichert

# Words on a row while a part is being resized.
disks-resizing = Größe wird geändert

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Auf { $size } geändert

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Trindle hat die Dateien nicht gefunden und kann daher keinen Platz schaffen.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Schnappschuss wird erstellt

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Trindle hat den Schnappschuss danach nicht gefunden.

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Nichts wurde in der Größe geändert.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Während Trindle den Schnappschuss erstellt hat, hat sich etwas geändert, daher wurde nichts in der Größe geändert.

# Words on a row while room is being made.
disks-making-room = Platz wird geschaffen

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = Die Auslagerungsdatei wird ausgeschaltet

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = Die Auslagerungsdatei wird wieder eingeschaltet

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } Die Auslagerungsdatei bleibt bis zum Neustart aus. { $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } Platz geschaffen

# Words on a row while a part is being repaired.
disks-repairing = Wird repariert

# Words on a row when a part has been repaired.
disks-repaired = Repariert

# Red words when some faults could not be put right.
disks-repair-failed = Trindle konnte einige Fehler nicht reparieren. Kopiere alle Dateien, die du behalten willst, auf ein anderes Laufwerk.

# Words on a row while a part is being checked.
disks-checking = Wird geprüft

# Words on a row when the check found no faults.
disks-no-faults = Keine Fehler gefunden

# Red words when the check found faults.
disks-faults-found = Fehler gefunden. Drücke den Schraubenschlüssel, um sie zu beheben.

# Words on a row while a part or drive is being formatted.
disks-formatting = Wird formatiert

# Words on a row when a part has been formatted.
disks-formatted = Formatiert

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = das ganze Laufwerk löschen

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Die neue Partition wurde nicht gefunden.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Der Laufwerksdienst ist nicht erreichbar.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Trindle konnte die Kennung der neuen Partition nicht lesen.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Wird gelöscht

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Gelöscht

# Words on a row while a new part is being made.
disks-making-it = Wird erstellt

# Words on a row when a new part has been made.
disks-made = Erstellt

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Das ließ sich nicht speichern: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
# SAFETY: needs a human review before it ships
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Gib ihm zuerst einen Namen.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] Dieser Name ist zu lang. Verwende höchstens { $max } Zeichen.
       *[other] Dieser Name ist zu lang. Verwende höchstens { $max } Zeichen.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Verwende nur Buchstaben, Ziffern, - und _ (keine Leerzeichen).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Beginne mit einem Buchstaben oder einer Ziffer.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } ist schon in Gebrauch.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] Dieser Name ist für diese Partition zu lang. Verwende höchstens { $max } Zeichen.
       *[other] Dieser Name ist für diese Partition zu lang. Verwende höchstens { $max } Zeichen.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT-Namen dürfen * ? . , ; : / \ | + = < > [ ] oder " nicht enthalten.

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Hänge sie zuerst ein. Diese Art von Partition kann nur vergrößert werden, solange sie eingehängt ist.
       *[smaller] Hänge sie zuerst ein. Diese Art von Partition kann nur verkleinert werden, solange sie eingehängt ist.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Diese Art kann nicht vergrößert werden.
       *[smaller] Diese Art kann nicht verkleinert werden.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Du hast kein Passwort eingegeben, daher hat sich nichts geändert.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Du hast kein Passwort eingegeben, daher wurde die Arbeit auf halbem Weg angehalten.

# Red words when another change is pressed while one is still being made.
disks-change-going = Eine Änderung wird gerade durchgeführt.

disks-one-at-a-time = Eine andere Änderung wird noch durchgeführt. Versuche es erneut, wenn sie fertig ist.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Trindle konnte nicht sicherstellen, dass nach deinem Passwort gefragt wird, daher hat sich nichts geändert.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Trindle konnte das soeben angegebene Passwort nicht zurücknehmen. Einige Minuten lang fragt eine weitere Änderung an einem Laufwerk vielleicht nicht nach einem Passwort. Trindle versucht es erneut.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = Das Passwort, das du angegeben hast, wurde nun zurückgenommen, daher fragt die nächste Änderung danach.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Trindle konnte den Laufwerksdienst nicht erreichen, daher hat sich nichts geändert.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Laufwerke wurde geschlossen, während diese Änderung durchgeführt wurde.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Laufwerke bleibt offen, bis dies fertig ist, damit du siehst, wie es gelaufen ist.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, daher hat sich nichts geändert.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, daher wurde die Arbeit auf halbem Weg angehalten.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Der Laufwerksdienst ließ sich nicht fragen

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } Die Arbeit wurde auf halbem Weg angehalten.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Erledigt: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Fehlgeschlagen: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Nicht erledigt: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Unbekannt: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } wurde dafür ausgehängt und ist jetzt nicht eingehängt.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } wurde dafür ausgehängt und ist weiterhin ausgehängt. Hänge sie über ihre Zeile ein.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } wurde vielleicht ausgehängt. Sieh in ihrer Zeile nach.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-put-back-password = { $dev } wurde dafür ausgehängt. Hänge sie über ihre Zeile wieder ein und gib auf Nachfrage ihr Passwort ein.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } wurde dafür ausgehängt und konnte nicht wieder eingehängt werden ({ $why }). Hänge sie über ihre Zeile ein.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Das Formatieren ist fertig, aber Trindle konnte ihr den Ordner { $dir } nicht wieder zuweisen. { $why } Wähle den Ordner erneut in ihrer Zeile.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Etwas benutzt sie noch. Schließe alle Fenster oder Programme, die ihre Dateien zeigen, und versuche es dann erneut.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Diese Größe ist zu klein für die Dateien, die schon darauf liegen.

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } aushängen

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } wieder einhängen

# Step of a change.
disks-step-eject = das Laufwerk zum Entfernen vorbereiten

# Step of a change.
disks-step-power-off = das Laufwerk ausschalten

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = { $dev } formatieren

# Step of a change.
disks-step-create = eine Partition erstellen

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = { $dev } löschen

# Step of a change. { $dev } is the part.
disks-step-resize-part = { $dev } in der Größe ändern

# Step of a change. { $dev } is the part.
disks-step-resize = das, was auf { $dev } liegt, in der Größe ändern

# Step of a change. { $dev } is the part.
disks-step-repair = { $dev } reparieren

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } prüfen

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } umbenennen

# Step of a change. { $dev } is the part.
disks-step-fstab = den Ordner und die Starteinstellungen von { $dev } speichern

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = der Schritt { $method } auf { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Diese { $kind }-Partition kann nur verkleinert werden, wenn sie nicht in Gebrauch ist, daher kann Trindle hier keinen Platz schaffen.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = xfs-Partitionen lassen sich nicht verkleinern.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Diese { $kind }-Partition kann nicht verkleinert werden, solange sie in Gebrauch ist.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Trindle kann nur von der eigenen Platte des Systems einen Schnappschuss erstellen, daher kann es hier keinen Platz schaffen.

# Red words under the Make room row.
disks-room-not-in-table = Sie ist keine Partition in der Partitionstabelle eines Laufwerks, daher kann Trindle daraus keinen Platz schaffen.

# Red words under the Make room row.
disks-room-no-online-shrink = Der Laufwerksdienst kann Btrfs hier nicht verkleinern, solange es in Gebrauch ist, daher kann Trindle keinen Platz schaffen.

# Red words under the Make room row.
disks-room-holds-parts = Sie enthält andere Partitionen, daher kann Trindle daraus keinen Platz schaffen.

# Red words under the Make room row.
disks-room-not-gpt = Ihr Laufwerk hat keine GPT-Partitionstabelle, daher kann Trindle daraus keinen Platz schaffen.

# Red words under the Make room row.
disks-room-mount-first = Hänge sie zuerst ein: Btrfs wird verkleinert, während es eingehängt ist.

# Red words under the Make room row.
disks-room-mounts-not-one = Sie ist nicht eingehängt, daher kann Trindle keinen Platz schaffen.

# Red words under the Make room row.
disks-room-read-only = Sie ist schreibgeschützt eingehängt, daher kann Trindle keinen Platz schaffen.

# Red words under the Make room row.
disks-room-read-only-unknown = Trindle konnte nicht feststellen, ob sie schreibgeschützt ist, daher kann es keinen Platz schaffen.

# Red words under the Make room row.
disks-room-many-drives = Die Dateien verteilen sich über mehr als ein Laufwerk, daher kann Trindle keinen Platz schaffen.

# Red words under the Make room row.
disks-room-drives-unknown = Trindle konnte nicht feststellen, über wie viele Laufwerke sich die Dateien verteilen, daher kann es keinen Platz schaffen.

# Red words under the Make room row.
disks-room-swap-unknown = Trindle konnte nicht feststellen, ob eine Auslagerungsdatei darauf liegt, daher kann es keinen Platz schaffen.

# Red words under the Make room row.
disks-room-used-unknown = Trindle konnte nicht lesen, wie viel davon belegt ist, daher kann es keinen Platz schaffen.

# Red words under the Make room row.
disks-room-area-unknown = Der Laufwerksdienst sagt nicht, wie groß das Dateisystem ist, daher kann Trindle es nicht prüfen und keinen Platz schaffen.

# Red words under the Make room row.
disks-room-open-missing = Trindle hat die entsperrte Kopie davon nicht gefunden, daher kann es keinen Platz schaffen.

# Red words under the Make room row.
disks-room-encryption-sizes = Die angegebenen Größen für die Verschlüsselung stimmen nicht überein, daher kann Trindle keinen Platz schaffen.

# Red words under the Make room row.
disks-room-sizes = Die dafür angegebenen Größen stimmen nicht überein, daher kann Trindle keinen Platz schaffen.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Sie enthält { $used } und muss mehr als das behalten, daher lässt sich zu wenig Platz freigeben.

# Red words under the Make room row.
disks-room-sizes-unread = Die Größen ließen sich nicht lesen.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Du kannst zwischen { $least } und { $most } Platz schaffen.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, darum hat Trindle angehalten.

# What was found wrong while making room.
disks-room-check-unmounted = Die Dateien sind nicht mehr eingehängt

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = Die Partition ist { $is } groß und sollte { $should } groß sein

# What was found wrong while making room.
disks-room-check-open = Die entsperrte Kopie der Verschlüsselung hat nicht die richtige Größe

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = Das Dateisystem ist { $is } groß und sollte { $should } groß sein

# What was found wrong while making room.
disks-room-check-area-small = Das Dateisystem ist kleiner als das, was belegt ist

# What was found wrong while making room.
disks-room-check-fit = Das Dateisystem passt nicht mehr hinein

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] Die Partition wird vom Laufwerksdienst nicht mehr aufgelistet, daher wurde die Arbeit angehalten.
        [copy] Die entsperrte Kopie wird vom Laufwerksdienst nicht mehr aufgelistet, daher wurde die Arbeit angehalten.
       *[files] Das Dateisystem wird vom Laufwerksdienst nicht mehr aufgelistet, daher wurde die Arbeit angehalten.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = das Dateisystem verkleinern

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = die entsperrte Kopie der Verschlüsselung verkleinern

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = die Partition verkleinern

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = die Größen prüfen

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Vor dem Platzschaffen auf { $dev } ({ $at })

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Klicke noch einmal, um die Größe von { $name } auf { $drive } zu ändern, von { $from } auf { $to }. Sichere vorher die Dateien darauf, denn ein Stromausfall während des Verkleinerns kann sie zerstören.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows auf { $name } wurde in den Ruhezustand versetzt oder im Schnellstart belassen, daher könnte es beim Verkleinern seine Dateien verlieren. Starte Windows, wähle Neu starten und versuche es dann erneut.
