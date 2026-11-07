
# machine translated (German), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = { $name } installieren

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Willkommen

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Datenträger

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Datenschutz

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Passwort

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Du

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Bereit

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Installation

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Fertig

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Vorbereitung

# Red line at the foot when the installer's helper could not be started.
installer-no-start = Das Installationsprogramm konnte nicht starten: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Der Plan konnte nicht gelesen werden.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Der Plan konnte nicht gelesen werden: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = Die Installation wurde angehalten. Es wurde nichts weiter geschrieben.

# Name of the round button at the foot that goes back a page.
installer-back = Zurück

# Name of the main button at the foot that goes to the next page.
installer-next = Weiter

# Name of the main button at the foot of the Ready page that starts the install.
# SAFETY: needs a human review before it ships
installer-install = Installieren

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Jetzt neu starten

# Beside the keyboard test box on the first page.
installer-try-keyboard = Tastatur ausprobieren

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Tippe hier, um die Tastatur auszuprobieren

# Note beside Install after one press, when the second press writes beside another system.
# SAFETY: needs a human review before it ships
installer-arm = Zum Installieren noch einmal klicken

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Zum Löschen und Installieren noch einmal klicken

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Wähle eine Sprache

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Wähle eine Tastatur

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Wähle eine Zeit

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Wähle, wohin Trindle soll

# Note beside Next on the disk password page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-disk-password = Gib das Passwort für den Datenträger zweimal gleich ein

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Ein Anmeldename wird benötigt

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Gib diesem Computer einen Namen

# Note beside Next on the account page while no password has been typed twice.
# SAFETY: needs a human review before it ships
installer-missing-password = Gib dein Passwort zweimal gleich ein

# Title of the first page.
installer-welcome-title = Willkommen bei { $name }

# Sentence under the title of the first page.
installer-welcome-help = Wähle deine Sprache. Tastatur und Zeit richten sich danach, und du kannst beides ändern.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Sprache

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Tastatur

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Zeit

# Under a card's head while nothing has been chosen in its list.
installer-select = Auswählen

# Title of the disk page.
installer-disks-title = Wohin { $name } kommt

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Alles auf dem gewählten Datenträger wird ersetzt, außer du schaffst Platz neben dem, was dort ist.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Es wurde kein ausreichend großer Datenträger gefunden. Trindle braucht 20 GB.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Den ganzen Datenträger verwenden und alles darauf ersetzen

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = { $what } behalten und den Datenträger teilen

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } bleiben für das, was jetzt dort ist, { $mine } für Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Freier Platz

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } und { $last }

# Under a disk's name: what is on it.
installer-disk-has = Enthält { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Enthält { $list }, verschlüsselt

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Enthält ein verschlüsseltes System

# Under a disk's name: there is nothing on it.
installer-disk-empty = Leer

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Enthält Dateien, aber kein System, das Trindle erkennt

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Eine Auslagerungsdatei von { $gb } GB

# Name of the switch that makes a swap file.
installer-swap-switch = Auslagerungsdatei anlegen

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot ist eingeschaltet, daher kann dieser Computer nicht in den Ruhezustand wechseln. Die Auslagerungsdatei hilft aber trotzdem, wenn der Speicher knapp wird. Sie wird nur angelegt, wenn auf dem Datenträger reichlich Platz übrig ist.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Platz für den Ruhezustand, bei dem alles Geöffnete ohne Strom erhalten bleibt, und eine Reserve, wenn der Speicher knapp wird. Ohne sie kann der Computer nicht in den Ruhezustand wechseln. Sie wird nur angelegt, wenn auf dem Datenträger reichlich Platz übrig ist.

# Title of the page about encrypting the disk.
installer-privacy-title = Den Datenträger verschlüsseln

# Sentence under the title of that page.
installer-privacy-help = Ein verschlüsselter Datenträger kann von niemandem gelesen werden, der ihn aus diesem Computer ausbaut.

# Head of the card with the three choices.
installer-privacy-locked = Wie der Datenträger gesperrt wird

# Name and words of the choice not to encrypt the disk.
installer-privacy-none = Den Datenträger nicht verschlüsseln

# Name and words of the choice to encrypt the disk with a password typed at start-up.
# SAFETY: needs a human review before it ships
installer-privacy-password = Verschlüsseln und beim Start des Computers nach dem Passwort fragen

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
# SAFETY: needs a human review before it ships
installer-privacy-chip-choice = Verschlüsseln und vom TPM-Chip dieses Computers entsperren lassen

# Head of the card about the TPM chip.
installer-privacy-about = Über den Chip

# In the card about the chip, on a computer that has one.
installer-privacy-chip = Dieser Computer hat einen TPM-Chip. Beim ersten Start nach der Installation wird einmal nach dem Passwort gefragt, während der Chip diesen Computer kennenlernt. Danach entsperrt er den Datenträger selbst, und es muss nichts eingetippt werden. Bewahre das Passwort gut auf: Es ist der Weg hinein, wenn der Chip ersetzt oder der Datenträger in einen anderen Computer eingebaut wird.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Dieser Computer hat keinen TPM-Chip, den Trindle nutzen kann. Daher wird bei jedem Start nach dem Passwort gefragt. Auf einem Computer mit Chip kann dieser den Datenträger selbst entsperren.

# Title of the disk password page.
installer-disk-title = Das Passwort für den Datenträger

# Sentence under the title of the disk password page.
installer-disk-help = Es wird beim Start des Computers vor allem anderen eingegeben.

# Head of the card with the disk password boxes, and the name of the first box.
# SAFETY: needs a human review before it ships
installer-disk-password = Passwort für den Datenträger

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Passphrase vorschlagen

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Noch einmal dasselbe

# Red line under the password boxes when the two do not match.
# SAFETY: needs a human review before it ships
installer-mismatch = Die beiden Passwörter stimmen nicht überein.

# Head of the card that tells the person to write the password down.
installer-disk-write-down = Schreib es auf

# Words in that card.
# SAFETY: needs a human review before it ships
installer-disk-write-down-text = Schreibe dieses Passwort auf Papier und bewahre es sicher auf, getrennt vom Computer. Ohne das Passwort kann der Datenträger nie geöffnet werden, weder von dir noch von sonst jemandem. Es gibt keinen anderen Weg hinein. Vier gewöhnliche Wörter ergeben eine Passphrase, die sich leicht tippen und schwer erraten lässt.

# Title of the account page.
installer-you-title = Über dich

# Sentence under the title of the account page.
installer-you-help = Das ist das Konto, mit dem du dich anmeldest.

# Head of the card with the name boxes.
installer-account = Dein Konto

# Label and name of the box for the person's name.
installer-your-name = Dein Name

# Label and name of the box for the sign-in name.
installer-user-name = Anmeldename

# Label and name of the box for the computer's name.
installer-host-name = Name dieses Computers

# Head of the card with the password boxes.
# SAFETY: needs a human review before it ships
installer-your-password = Dein Passwort

# Label and name of the first password box.
# SAFETY: needs a human review before it ships
installer-password = Passwort

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Wie schwer es zu erraten ist

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Ein Anmeldename wird benötigt.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Dieser Name ist zu lang.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Beginne mit einem Kleinbuchstaben.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Nur Kleinbuchstaben, Ziffern, - und _.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Dieser Name wird schon vom System verwendet.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Gib diesem Computer einen Namen.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Nur Buchstaben, Ziffern und -.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Ein Computer würde das sofort erraten.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Ein Computer würde das in etwa { $n } Minute erraten.
       *[other] Ein Computer würde das in etwa { $n } Minuten erraten.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Ein Computer würde das in etwa { $n } Stunde erraten.
       *[other] Ein Computer würde das in etwa { $n } Stunden erraten.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Ein Computer würde das in etwa { $n } Tag erraten.
       *[other] Ein Computer würde das in etwa { $n } Tagen erraten.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Ein Computer bräuchte etwa { $n } Jahr, um das zu erraten.
       *[other] Ein Computer bräuchte etwa { $n } Jahre, um das zu erraten.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Ein Computer bräuchte länger als die gesamte überlieferte Geschichte, um das zu erraten.

# Title of the Ready page.
installer-ready-title = Bereit zur Installation

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Bisher wurde nichts geändert

# Head of the card about security updates.
installer-ready-security = Sicherheitsupdates

# In the card about security updates.
installer-ready-security-text = { $name } installiert die Sicherheitskorrekturen von Debian jeden Tag selbstständig. Das lässt sich nicht abschalten.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = Alles auf { $disk } wird ersetzt.

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Alles auf { $part } bleibt erhalten, aber sichere es vorher: Beim Verkleinern können Dateien verloren gehen, wenn der Strom ausfällt.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Es wurde noch kein Datenträger gewählt.

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = Ganz { $disk }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Alles darauf wird durch { $name } ersetzt.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Neben { $part }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-beside-text = { $part } wird auf { $size } verkleinert, und { $name } kommt in den freigewordenen Platz.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Kein Datenträger

# Words of that row.
installer-ready-no-disk-text = Kein Datenträger gewählt.

# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Nicht verschlüsselt

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-plain-text = Jeder, der den Datenträger ausbaut, kann ihn lesen.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Verschlüsselt

# Words of that row when a password is typed at each start.
# SAFETY: needs a human review before it ships
installer-ready-locked-password = Du gibst bei jedem Start des Computers das Passwort ein.

# Words of that row when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-ready-locked-chip = Das Passwort einmal, danach entsperrt der TPM-Chip dieses Computers den Datenträger.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Tastatur { $keyboard }, Zeit { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = { $name } wird installiert

# Head of the card that holds the short tour.
installer-tour = Ein kurzer Rundgang

# Title of the last page.
installer-done-title = { $name } ist installiert

# Sentence under the title of the last page.
installer-done-help = Ziehe den USB-Stick ab und starte dann neu.

# Head of the card that repeats the warning about the disk password.
# SAFETY: needs a human review before it ships
installer-done-keep = Bewahre das Passwort für den Datenträger sicher auf

# In that card when the TPM chip unlocks the disk.
# SAFETY: needs a human review before it ships
installer-keep-chip = Der Datenträger ist verschlüsselt. Beim ersten Start wird einmal nach dem Passwort gefragt, während der Chip diesen Computer kennenlernt. Danach öffnet er sich von selbst. Schreibe dieses Passwort auf Papier und bewahre es sicher auf, getrennt von diesem Computer. Ohne das Passwort kann der Datenträger nie geöffnet werden, weder von dir noch von sonst jemandem.

# In that card when the password is typed at each start.
# SAFETY: needs a human review before it ships
installer-keep-password = Der Datenträger ist verschlüsselt. Schreibe das Passwort jetzt auf Papier, falls du das noch nicht getan hast, und bewahre es sicher auf, getrennt von diesem Computer. Ohne das Passwort kann der Datenträger nie geöffnet werden, weder von dir noch von sonst jemandem.

# Title of the first slide.
installer-slide-ring = Alles in einem Kreis

# Words of the first slide.
installer-slide-ring-text = Bewege den Zeiger an den Bildschirmrand und doppelklicke, dann öffnet sich der Ring: außen deine Programme, in der Mitte die Uhrzeit und die Schalter.

# Title of the second slide.
installer-slide-snapshots = Zurückholen, was ein Update kaputt gemacht hat

# Words of the second slide.
installer-slide-snapshots-text = Vor und nach jeder Softwareänderung wird ein Schnappschuss des Systems gemacht. In den Einstellungen setzt du das ganze System auf jeden davon zurück, und das Startmenü kann es auch, wenn der Computer nicht mehr startet.

# Title of the third slide.
installer-slide-apps = Apps, die nicht im Weg sind

# Words of the third slide.
installer-slide-apps-text = Flathub-Apps, die von Debian, AppImages und Softwarequellen an einem Ort, mit Sicherungen, die du auf einen anderen Computer mitnehmen kannst.

# Title of the fourth slide.
installer-slide-yours = Deins, und zwar schnell

# Words of the fourth slide.
installer-slide-yours-text = Hell oder dunkel, eigene Töne und eine Zwischenablage, die sich merkt, was du kopiert hast.

# Title of the fifth slide.
installer-slide-small = Klein und leise

# Words of the fifth slide.
installer-slide-small-text = Die Oberfläche braucht wenig Speicher und startet in wenigen Sekunden, sodass sich ein älteres Notebook wieder wie neu anfühlt.

# Title of the tour slide about the ring.
installer-tour-ring = Der Ring ist die ganze Oberfläche

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Setze den Zeiger in diese Ecke, dann öffnet sich der Ring

# Note on the ring picture, at the time.
installer-tour-ring-time = Uhrzeit und Datum in der Mitte

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Deine Programme sitzen außen herum

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Ton, Bildschirm, alle deine Apps, Wachhalten und Ausschalten

# Title of the tour slide about Settings.
installer-tour-settings = Einstellungen in einfachen Worten

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Tippe hier, um eine Einstellung zu finden, ohne lange zu suchen

# Note on the Settings picture, at Network.
installer-tour-settings-network = Unter Netzwerk verbindest du dich mit einem Wi-Fi-Netzwerk

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Hell oder dunkel, ganz wie du magst

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Zwanzig Farben, oder tippe deine eigene ein

# Title of the tour slide about Apps.
installer-tour-apps = Apps für alles andere

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Suche ein Programm nach Name oder danach, was es kann

# Note on the Apps picture, at the list.
installer-tour-apps-many = Tausende Programme, kostenlos, von Flathub und Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Updates für alles, auch für Trindle selbst

# Title of the tour slide about voice control.
installer-tour-voice = Du kannst auch mit ihm sprechen

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Die Sprachsteuerung findest du hier: Einstellungen, dann Ring

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Drücke einmal auf Sprache hinzufügen. Sie ist etwa 150 MB groß und braucht ein paar Minuten

# Note on the voice picture, about what to say.
installer-tour-voice-say = Tippe dann auf die Mitte des Rings und sage den Namen einer App, „lauter“ oder „Wi-Fi-Einstellungen“

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Was du sagst, wird auf diesem Computer verstanden und geht nirgendwohin sonst

# Title of the tour slide about snapshots.
installer-tour-snapshots = Ein Weg zurück, bevor sich etwas ändert

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Der Computer macht vor jedem Update einen Schnappschuss

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Wenn etwas kaputtgeht, kehrst du über das Startmenü zum früheren Stand zurück

# Name of the picture of the ring, and the head of its card.
installer-live-caption = Der Ring, in dem alles zu Hause ist

# Sentence under the title of the welcome from a stick.
installer-live-help = Du arbeitest vom Stick aus, daher wurde auf diesem Computer nichts geändert.

# Head of the card with the three first things to do.
installer-live-begin = Zum Anfang

# Name of the first thing to do.
installer-live-open = Den Ring öffnen

# Line under it.
installer-live-open-line = Doppelklicke an den Bildschirmrand oder drücke Super.

# Name of the second thing to do.
installer-live-speak = Mit ihm sprechen

# Line under it when voice control is there.
installer-live-speak-line = Tippe auf die Mitte des Rings und sage, was du möchtest.

# Line under it when voice control has to be added.
installer-live-speak-add = Füge die Sprache in den Einstellungen unter Ring hinzu, tippe dann auf die Mitte und sprich.

# Name of the third thing to do.
installer-live-tab = Alles im Ring

# Line under it.
installer-live-tab-line = Was geöffnet ist, deine Benachrichtigungen und die Zwischenablage findest du alle dort.

# The choice of the dark look.
installer-live-dark = Dunkel

# The choice of the light look.
installer-live-light = Hell

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Hell oder dunkel, für die ganze Oberfläche

# Name of the round button that closes this window and opens the ring.
installer-live-around = Erst umsehen

# Name of the main button that opens the installer.
installer-live-install = { $name } installieren

# Foot of the install page while room is made on the disk.
installer-step-room = Platz schaffen

# Foot of the install page while the disk is prepared.
installer-step-prepare = Datenträger vorbereiten

# Foot of the install page while a place is made beside another system.
installer-step-place = Platz für Trindle schaffen

# Foot of the install page while the disk is looked at.
installer-step-check = Prüfen, was schon auf dem Datenträger ist

# Foot of the install page while another system is made smaller.
installer-step-smaller = Das andere System verkleinern

# Foot of the install page while the disk is encrypted.
installer-step-lock = Datenträger verschlüsseln

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Chip des Computers einrichten

# Foot of the install page while the system is copied.
installer-step-copy = Trindle auf den Datenträger kopieren

# Foot of the install page while settings are written.
installer-step-settings = Einstellungen schreiben

# Foot of the install page while the person's account is made.
installer-step-account = Dein Konto anlegen

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Die Teile des Live-Systems entfernen

# Foot of the install page while the start-up files are made.
installer-step-startup = Startdateien erstellen

# Foot of the install page while the boot menu is set up.
installer-step-boot = Startmenü einrichten

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Schnappschüsse einrichten

# Foot of the install page at the very end.
installer-step-finishing = Abschließen

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } konnte nicht starten: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } wurde nicht beendet. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = { $path } konnte nicht geschrieben werden: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } ist kein Datenträger.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } ist kein Datenträger in diesem Computer.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Es wurde kein Datenträger gewählt.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Diesen Anmeldenamen kann Trindle nicht verwenden.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Diesen Computernamen kann Trindle nicht verwenden.

# Red line at the foot: the language cannot be used.
installer-bad-language = Diese Sprache kann Trindle nicht verwenden.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Diese Tastatur kann Trindle nicht verwenden.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Diese Zeitzone kann Trindle nicht verwenden.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Dieser Name enthält etwas, das Trindle nicht schreiben kann.

# Red line at the foot: a password has a line break in it.
# SAFETY: needs a human review before it ships
installer-bad-password = Ein Passwort darf keinen Zeilenumbruch enthalten.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = Die zweite Hälfte des Installationsprogramms muss als root laufen.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Dieser Datenträger hat keine Boot-Partition zum Mitnutzen, daher kann Trindle nicht neben dem Vorhandenen installiert werden.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Der neue Teil des Datenträgers wurde nicht gefunden.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = Die Größe von { $part } konnte nicht gelesen werden.

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle kann eine { $fs }-Partition nicht verkleinern.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle kann eine unbekannte Partition nicht verkleinern.

# Red line at the foot: encrypting needs a password and there is none.
# SAFETY: needs a human review before it ships
installer-disk-password-needed = Zum Verschlüsseln des Datenträgers wird ein Passwort benötigt.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Das System konnte nicht kopiert werden: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Das System konnte nicht kopiert werden. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Kein Platz für eine Auslagerungsdatei.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Der Ort der Auslagerungsdatei auf dem Datenträger konnte nicht gelesen werden.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Die neuen Partitionen konnten nicht erkannt werden.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Die Zeitzone konnte nicht eingestellt werden: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Das Startmenü konnte auf diesem Computer nicht installiert werden.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Sicher?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Was geöffnet ist, die Zwischenablage und eine Taste, die den Bildschirm freiräumt

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } ist der Stick, von dem Trindle gerade läuft.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } wird gerade benutzt. Schließe, was es benutzt, und versuche es dann erneut.

# Said beside how far the install has got, once it has started.
installer-started = Installation gestartet

# The button that ends an install under way.
installer-halt = Anhalten

# Beside Stop after its first press.
installer-halt-sure = Installation anhalten? Der alte Inhalt des Datenträgers ist bereits weg.

# While an install is being stopped.
installer-halting = Wird angehalten

# When Stop could not let go of the disk.
installer-halt-stuck = Der Datenträger konnte nicht freigegeben werden. Starte den Computer neu und versuche es erneut.

# The copy is done and the person is still answering.
installer-step-waiting = Kopiert. Es wird auf deine Antworten gewartet.



installer-already-running = Auf diesem Computer läuft bereits ein anderes Installationsprogramm.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle braucht mindestens 1 GB Speicher, und dieser Computer hat weniger.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Ein Datenträger lässt sich nur auf einem Computer verschlüsseln, der per UEFI startet, und dieser tut es nicht.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows auf { $part } wurde in den Ruhezustand versetzt oder im Schnellstart belassen, daher könnte es beim Verkleinern seine Dateien verlieren. Starte Windows, wähle Neu starten und versuche es dann erneut.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle ist installiert, aber die Schnappschüsse konnten nicht eingerichtet werden: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle ist installiert, aber das Startmenü konnte nicht dazu gebracht werden, die Schnappschüsse aufzulisten: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Nicht alles wurde eingerichtet

