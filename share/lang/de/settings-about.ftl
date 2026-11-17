
# machine translated (German), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Über diesen Computer

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computername netzwerkname über system version debian trindle speicher arbeitsspeicher ram prozessor cpu datenträger grafik bildschirm modell kernel rolling release aktivität laufende programme ausgelastet task-manager systemmonitor laufwerke usb platz

# Text behind the info button of the About page.
settings-about-help = Diese Seite zeigt den Namen dieses Computers, wie ihn andere Computer im Netzwerk sehen, woraus er besteht und wie lange er zum Starten braucht. Aktivität zeigt, was der Computer gerade tut und welche Programme Speicher und Prozessor verbrauchen. Laufwerke zeigt deine Laufwerke und ihren Platz. Jedes öffnet sich in einem eigenen Fenster. Öffne sie mit den Pfeilen bei Mehr Details.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Unbekannt

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } Thread
       *[other] { $model }, { $cores } Threads
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel-Grafik

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD-Grafik

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA-Grafik

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Grafik der virtuellen Maschine

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Grafik

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } bei { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } belegt

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } frei

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = von { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, seit { $on } eingeschaltet.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, seit { $on } eingeschaltet. Der Datenträger ist fast voll.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } Sekunden

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } Minuten

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } Minute
       *[other] { $n } Minuten
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } Stunde
       *[other] { $n } Stunden
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } Tag
       *[other] { $n } Tage
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Verwende nur Buchstaben, Zahlen und Bindestriche. Beginne und ende nicht mit einem Bindestrich.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Warten auf dein Passwort…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Der Computer konnte nicht umbenannt werden.

# Words under the computer name box after renaming worked.
settings-about-name-done = Bereits geöffnete Programme verwenden den neuen Namen nach deiner nächsten Anmeldung.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Computername

# Tip of the save button beside the computer name box.
settings-about-rename = Diesen Computer umbenennen

# Bold line of the computer name setting.
settings-about-name = Computername

# Heading of the card with the computer's name.
settings-about-name-group = Name

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Das ist der Name des Computers im Netzwerk und an der Eingabeaufforderung in einem Terminal. Verwende Buchstaben, Zahlen und Bindestriche, aber beginne und ende nicht mit einem Bindestrich. Zum Ändern musst du dein Passwort eingeben.

# Heading of the card about the computer's hardware.
settings-about-hardware = Der Computer

# Text behind the info button of the hardware card.
settings-about-hardware-about = Hier siehst du Hersteller und Modell des Computers, seinen Prozessor und seine Grafik. Außerdem siehst du, wie viele Threads der Prozessor gleichzeitig ausführen kann, und jeden verwendeten Bildschirm mit Größe und Bildwiederholrate.

# Heading of the card about the software.
settings-about-software = Die Software

# Text behind the info button of the software card.
settings-about-software-about = Trindle ist eine Oberfläche, die auf Debian läuft. Debian kümmert sich um die Programme darunter. Der Kernel ist der Teil, der mit der Hardware arbeitet. Trindles eigener Compositor zeichnet die Oberfläche mit Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindles eigener Compositor

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } App
       *[other] { $n } Apps
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Mehr Details

# Text behind the info button of the More detail card.
settings-about-more-about = Aktivität zeigt, was der Computer gerade tut und welche Programme Speicher und Prozessor verbrauchen. Laufwerke zeigt jedes Laufwerk, wie voll und gesund es ist, und alle USB-Sticks.

# Bold line of the setting that opens Activity.
settings-about-activity = Sehen, was läuft

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Aktivität

# Bold line of the setting that opens Drives.
settings-about-drives = Laufwerke und Platz

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Laufwerke

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle konnte die Startzeiten nicht in Downloads speichern.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Datum und Uhrzeit,Sekunden

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle Startzeiten.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = In Downloads, als { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Speichern

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Alle Startzeiten in Downloads speichern

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Jede Startzeit speichern

# Quiet line under that setting.
settings-about-times-keep-sub = In Downloads als Tabelle gespeichert.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Jede Säule ist ein Start. Der neueste ist rechts.

# Heading of the card with the last ten start-up times.
settings-about-times = Die letzten zehn Starts

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle misst jeden Start vom Einschaltknopf bis zum Anmeldebildschirm. Die Zeit für die Eingabe des Passworts für den Datenträger wird nicht mitgezählt. Der neueste steht oben.

# Heading of the memory tile.
settings-about-memory = Speicher

# Heading of the disk tile.
settings-about-disk = Datenträger

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Problemberichte

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Berichte gehen an Buildmorphic, die Community, die beim Bau von Trindle hilft. Du kannst mit nur einer E-Mail-Adresse beitreten. Für Probleme, die du als Erster meldest, bekommst du Punkte und Stufen, auch wenn du sie nicht beheben kannst. Ein Bericht enthält nur eine feste Liste von Angaben, und ohne Druck auf Senden wird nichts abgeschickt. Ein Benutzername oder GitHub-Name ist freiwillig, und dein Benutzername ist nie deine E-Mail-Adresse.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Einen Bericht anbieten, wenn etwas schiefgeht

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic-Benutzername

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub-Name

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Über die Community

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Aus. { $n } wartender Bericht gelöscht.
       *[other] Aus. { $n } wartende Berichte gelöscht.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Aus. { $n } wartender Bericht konnte nicht gelöscht werden.
       *[other] Aus. { $n } wartende Berichte konnten nicht gelöscht werden.
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = Updates

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle sucht kurz nach der Anmeldung und danach alle paar Stunden nach neuen Versionen von Apps, Plugins, Firmware und sich selbst. Wenn du das ausschaltest, sucht Trindle nur, wenn du in Apps auf Nach Updates suchen drückst oder auf die Update-Taste am Ring. Die Sicherheitsfixes von Debian werden jeden Tag installiert und haben einen eigenen Schalter. Wenn du ihn ausschaltest, werden sie nicht mehr von selbst installiert, und bekannte Sicherheitslücken bleiben offen, bis du aktualisierst. Zum Ändern eines der beiden Schalter brauchst du das Passwort des Administrators, damit der ganze Computer deine Wahl verwendet.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Nach Updates suchen

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Warte auf dein Passwort…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Sicherheitsupdates

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Sicherheitsfixes werden nicht mehr von selbst installiert.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Aus, aber die wartenden Berichte des Reporting-Zusatzes wurden nicht gelöscht.
