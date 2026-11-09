
# machine translated (German), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Über diesen Computer

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computername netzwerkname über system version debian trindle speicher arbeitsspeicher ram prozessor cpu datenträger grafik bildschirm modell kernel rolling release aktivität laufende programme ausgelastet task-manager systemmonitor laufwerke usb platz

# Text behind the info button of the About page.
settings-about-help = Der Name dieses Computers, wie ihn andere Computer im Netzwerk sehen, woraus er besteht und wie lange er zum Starten braucht. Was er gerade tut und welche Programme Speicher und Prozessor verbrauchen, steht in Aktivität, die Laufwerke und ihr Platz in Laufwerke, jeweils in einem eigenen Fenster: Die Pfeile bei Mehr Details öffnen sie.

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
settings-about-name-bad = Nur Buchstaben, Zahlen und Bindestriche, ohne Bindestrich am Anfang oder Ende.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Warten auf das Passwort ...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Der Computer konnte nicht umbenannt werden.

# Words under the computer name box after renaming worked.
settings-about-name-done = Bereits geöffnete Programme sehen den neuen Namen nach der nächsten Anmeldung.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Computername

# Tip of the save button beside the computer name box.
settings-about-rename = Diesen Computer umbenennen

# Bold line of the computer name setting.
settings-about-name = Computername

# Quiet line under the computer name setting.
settings-about-name-sub = Buchstaben, Zahlen und Bindestriche.

# Heading of the card with the computer's name.
settings-about-name-group = Name

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Wie dieser Computer im Netzwerk und an der Eingabeaufforderung in einem Terminal erscheint. Buchstaben, Zahlen und Bindestriche, ohne Bindestrich am Anfang oder Ende. Zum Ändern wird nach deinem Passwort gefragt.

# Heading of the card about the computer's hardware.
settings-about-hardware = Der Computer

# Text behind the info button of the hardware card.
settings-about-hardware-about = Hersteller und Modell, der Prozessor und wie viele Threads er gleichzeitig ausführt, die Grafik und jeder verwendete Bildschirm mit Größe und Bildwiederholrate.

# Quiet line under the maker and model.
settings-about-hw-computer = Der Computer

# Quiet line under the processor.
settings-about-hw-processor = Der Prozessor

# Quiet line under the graphics.
settings-about-hw-graphics = Zeichnet die Bilder

# Quiet line under the screens.
settings-about-hw-screen = Der Bildschirm

# Heading of the card about the software.
settings-about-software = Die Software

# Text behind the info button of the software card.
settings-about-software-about = Trindle ist eine Oberfläche auf Debian, das sich um die Programme darunter kümmert. Der Kernel ist der Teil, der mit der Hardware spricht, und Trindles eigener Compositor zeichnet die Oberfläche, auf Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Läuft darunter

# Quiet line under the kernel version.
settings-about-sw-kernel = Der Kernel

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindles eigener Compositor

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Zeichnet die Oberfläche, auf Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } App
       *[other] { $n } Apps
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Installiert, bereit im Ring

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Mehr Details

# Text behind the info button of the More detail card.
settings-about-more-about = Was er gerade tut und welche Programme Speicher und Prozessor verbrauchen, steht in Aktivität. Jedes Laufwerk, wie voll und wie gesund es ist, und USB-Sticks stehen in Laufwerke.

# Bold line of the setting that opens Activity.
settings-about-activity = Sehen, was läuft

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Programme, die Speicher und Prozessor verbrauchen.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Aktivität

# Bold line of the setting that opens Drives.
settings-about-drives = Laufwerke und Platz

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Jedes Laufwerk, wie voll und gesund es ist.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Laufwerke

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Die Startzeiten konnten nicht in Downloads gespeichert werden.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = In Downloads, als { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Speichern

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Alle Startzeiten in Downloads speichern

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Jeder Start, zum Aufbewahren

# Quiet line under that setting.
settings-about-times-keep-sub = In Downloads als Tabelle gespeichert.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Jede Säule ist ein Start, der neueste rechts.

# Heading of the card with the last ten start-up times.
settings-about-times = Die letzten zehn Starts

# Text behind the info button of the start-up times card.
settings-about-times-about = Jeder Start wird vom Einschaltknopf bis zum Anmeldebildschirm gemessen, ohne die Zeit für die Eingabe des Passworts für den Datenträger. Der neueste steht oben.

# Heading of the memory tile.
settings-about-memory = Speicher

# Heading of the disk tile.
settings-about-disk = Datenträger


# Heading of the card with the switch for looking for updates.
settings-about-updates = Updates

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle sucht kurz nach der Anmeldung und danach alle paar Stunden nach neuen Versionen von Apps, Plugins, Firmware und sich selbst, und die Sicherheitsfixes von Debian werden jeden Tag eingespielt. Ist das ausgeschaltet, wird nur gesucht, wenn du in Apps auf Nach Updates suchen drückst oder auf die Update-Taste am Ring. Die Sicherheitsfixes von Debian haben einen eigenen Schalter; ist er aus, werden sie nicht mehr von selbst installiert und bekannte Fehler bleiben offen, bis du aktualisierst. Zum Ändern eines der beiden Schalter wird das Passwort des Administrators verlangt, damit auch die eigenen Dienste des Computers die Wahl sehen.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Nach Updates suchen

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Warte auf das Passwort ...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Aus: Nichts sucht, wenn du nicht darum bittest.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Sicherheitsupdates

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Sicherheitsfixes werden jeden Tag von selbst installiert.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Sicherheitsfixes werden nicht mehr von selbst installiert.

