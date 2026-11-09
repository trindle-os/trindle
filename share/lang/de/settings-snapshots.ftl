
# machine translated (German), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Schnappschüsse

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = schnappschüsse schnappschuss snapshot snapshots wiederherstellen rückgängig zurücksetzen zurückrollen sicherung backup update kaputt btrfs snapper bootmenü grub frühere versionen ältere version vorherige gelöschte datei zurückholen home kopien

# Help line at the foot of the Snapshots page.
settings-snap-help = Das System, wie es vor und nach jeder Softwareänderung war, und wann immer du einen machst. Beim Wiederherstellen wird das ganze System ab dem nächsten Start auf diesen Stand zurückgesetzt; deine eigenen Dateien bleiben unverändert, wenn der Home-Ordner einen eigenen Teil der Festplatte hat, wie ihn der Installer von Trindle anlegt. Wenn der Computer nicht startet, wähle im Bootmenü Trindle-Schnappschüsse. Frühere Versionen hält jeden Tag eine Kopie der eigenen Dateien aller Personen vor, getrennt von den Schnappschüssen des Systems.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Es wurde keine Kopie gemacht: Die Festplatte ist fast voll.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = heute um { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = gestern um { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = am { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = Der Computer wollte nicht neu starten. Starte ihn stattdessen über den Ring neu.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Ein Schnappschuss wird gemacht

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Wird freigegeben ...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Freigegeben

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Wird wiederhergestellt

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Das System ist ab dem nächsten Start Schnappschuss { $n }. Was seitdem geändert wurde, wird beiseitegelegt, nicht verloren.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Wird gelöscht

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Gelöscht

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Warte auf das Passwort ...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Jeden Tag wird eine Kopie gemacht

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Es werden keine mehr gemacht

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Behalten

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = Wird geleert

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = Geleert

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Noch keine Kopien
        [one] Eine Kopie
       *[other] { $n } Kopien
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = Die erste wird innerhalb eines Tages gemacht.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Sie belegen { $size }, gemessen { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Ihr Platzbedarf wird einmal am Tag gemessen.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Alle jetzt löschen

# Heading of the card about earlier versions of files.
settings-snap-earlier = Frühere Versionen

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle kann jeden Tag eine Kopie der Dateien aller Personen behalten, wie sie an dem Tag waren. So lässt sich eine ältere Version einer Datei oder eine versehentlich gelöschte in Ordner öffnen oder zurückholen. Jede Person kann in einer Kopie nur lesen, was sie an dem Tag lesen konnte, ein privates Home bleibt also privat; eine Datei, die damals für andere offen war, bleibt in dieser Kopie für sie offen, bis sie verschwindet. Die Kopien sind getrennt von den Schnappschüssen des Systems: Das Wiederherstellen des Systems ändert nie deine Dateien, und diese ändern nie das System. Der Preis ist Platz: Eine Datei, die du löschst oder änderst, belegt ihren Platz weiter, bis die letzte Kopie, die sie enthält, verschwunden ist, darum wird die Festplatte durch Löschen nicht sofort frei. Das Leeren der Kopien gibt den Platz jetzt frei, für alle auf diesem Computer, und lässt sich nicht rückgängig machen. Solange die Festplatte fast voll ist, wird keine Kopie gemacht. Für Änderungen daran ist das Passwort eines Administrators nötig.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Frühere Versionen deiner Dateien behalten

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Gelöschte Dateien bleiben { $time }, bevor ihr Platz freigegeben wird.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Die Kopie jedes Tages behalten für

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Wie lange die Kopie jedes Tages behalten wird

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Die Schnappschüsse werden angesehen …

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Die Schnappschüsse konnten nicht gelesen werden.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Von Schnappschuss { $n } gestartet, darum wird nichts, was jetzt getan wird, behalten.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] { $n } Schnappschuss zum Zurückgehen, der neueste von { $when }.
       *[other] { $n } Schnappschüsse zum Zurückgehen, der neueste von { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Noch keine Schnappschüsse. Vor jeder Softwareänderung wird einer gemacht.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, behalten bis zum Löschen

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Das System auf diesen Stand zurücksetzen

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Diesen Schnappschuss löschen

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Jetzt neu starten

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Sie sind noch nicht eingerichtet.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Installiere das Paket trindle neu.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Schnappschüsse sind nicht eingerichtet. Wenn du das Paket trindle neu installierst, werden sie eingerichtet: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = eine andere Art

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Sie brauchen Btrfs; dieser Computer nutzt { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Wähle beim Installieren von Debian Btrfs.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Schnappschüsse brauchen das System auf Btrfs, und dieser Computer nutzt { $fs }. Wähle beim Installieren von Debian Btrfs für das System, um sie zu haben.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Auf diesem Computer gibt es keine Schnappschüsse.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Keine Schnappschüsse

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Keine Schnappschüsse auf diesem Computer

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Um sie zu haben

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Suche läuft …

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Die Schnappschüsse konnten nicht gelesen werden

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Noch keine

# Under the None yet heading.
settings-snap-none-line-sub = Vor jeder Softwareänderung wird einer gemacht.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Mit jedem Schnappschuss kann das ganze System ab dem nächsten Start auf den damaligen Stand zurückgesetzt oder er kann gelöscht werden. Beides muss zweimal gedrückt werden und braucht das Passwort eines Administrators. Nach einer Wiederherstellung erscheint daneben eine Schaltfläche zum Neustart.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Jetzt einen Schnappschuss machen

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Nicht, solange von einem Schnappschuss gestartet wurde.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Wird behalten, bis du ihn löschst.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Ein System durch Wiederherstellen beiseitegelegt
       *[other] { $n } Systeme durch Wiederherstellen beiseitegelegt
    }

# Under the set aside heading.
settings-snap-free-space = Gib den Platz frei, wenn alles in Ordnung ist.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Den Platz freigeben, den sie belegen

# Heading of the card for taking a snapshot.
settings-snap-take-one = Einen machen

# Help text of the Take one card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-take-about = Mach einen Schnappschuss, bevor du etwas ausprobierst, das du vielleicht rückgängig machen willst; einer, den du machst, wird behalten, bis du ihn löschst. Solange der Computer von einem Schnappschuss gestartet ist, würde nichts Gemachtes behalten, starte darum zuerst normal neu. Jede Wiederherstellung legt beiseite, wie das System kurz davor war, darum geht nichts verloren; gib den Platz frei, den sie belegen, wenn alles in Ordnung ist.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Von Schnappschuss { $n } gestartet

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Nichts, was jetzt getan wird, bleibt erhalten.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Deine Dateien sind auch in den Schnappschüssen

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Beim Wiederherstellen kommen auch deine Dateien zurück.

# Heading of the card about what to know before restoring.
settings-snap-before = Vor dem Wiederherstellen

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Wenn von einem Schnappschuss gestartet wurde, bleibt nichts, was jetzt getan wird: Wenn das System so sein soll, drücke in der Liste bei ihm auf Wiederherstellen und starte dann neu. Auf diesem Computer kann der Home-Ordner Teil des Systems sein, und dann setzt das Wiederherstellen eines Schnappschusses auch deine eigenen Dateien auf den damaligen Stand zurück, kopiere darum zuerst alles Neue an einen sicheren Ort.



# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Systemdateien gesperrt

# Under the System files locked switch.
settings-snap-locked-sub = Nur Installationen und Updates entsperren sie.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Gesperrt

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Entsperrt

