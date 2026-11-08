
# machine translated (German), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Willkommen bei Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Den Ring öffnen

# Title of the third page, where the first apps are picked.
welcome-page-apps = Wähle deine Apps

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Mehr zur Auswahl

# Title of the fifth page, about plugins.
welcome-page-plugins = Plugins bringen mehr

# Title of the sixth page, about crash reports.
welcome-page-reports = Hilf, Trindle zu verbessern

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Die Punkte

# Title of the last page, with a few things worth knowing.
welcome-page-know = Gut zu wissen

# Sentence under the title of the first page.
welcome-sentence-hello = Alles, was du brauchst, ist in einem Kreis, dem Ring.

# Sentence under the title of the second page.
welcome-sentence-ring = Doppelklicke ganz an den Bildschirmrand oder drücke die Super-Taste.

# Sentence under the title of the third page.
welcome-sentence-apps = Es ist fast nichts installiert, also wähle die Apps, die du willst.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Wähle mehr aus und installiere dann alles Gewählte mit einem Klick.

# Sentence under the title of the fifth page.
# SAFETY: needs a human review before it ships
welcome-sentence-plugins = Plugins sind kleine Extras aus Apps, und jedes tut nur das, was du erlaubst.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Sende einen Bericht, wenn etwas schiefgeht, falls du das möchtest.

# Sentence under the title of the dots page.
welcome-sentence-dots = Ein Punkt bedeutet, dass ein Programm etwas von dir verwendet.

# Sentence under the title of the last page.
welcome-sentence-know = Ein paar Dinge, die Trindle besonders machen.

# Top right of every page: where the person is in the tour.
welcome-step = Schritt { $n } von { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Schritt { $n } von { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
# SAFETY: needs a human review before it ships
welcome-done-file-note = Das Willkommen wurde gesehen. Es ist unter Alle Apps als Willkommen zu finden. Lösche diese Datei, um es bei der nächsten Anmeldung wieder zu sehen.

# Name of the round button at the foot that closes the tour.
welcome-close = Schließen

# Name of the round button at the foot that goes to the page before.
welcome-back = Zurück

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Weiter

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Gewählte Apps installieren

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Trindle verwenden

# Foot of the apps pages while the computer is offline.
welcome-offline = Verbinde dich mit dem Internet, um Apps zu installieren.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Noch einmal versuchen

# Foot while apps are going in: their names.
welcome-installing = { $names } wird installiert.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = { $names } wird installiert, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } ist jetzt im Ring.
       *[other] { $names } sind jetzt im Ring.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Die Apps konnten nicht installiert werden. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak hat abgelehnt.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Gewählt: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } und { $more } weitere

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = Der Ring

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Was darin ist

# Name of the first thing the ring holds.
welcome-held-programs = Deine Programme

# Line under it.
welcome-held-programs-line = Sitzen außen herum.

# Name of the second thing the ring holds.
welcome-held-time = Uhrzeit und Datum

# Line under it.
welcome-held-time-line = Sitzen in der Mitte.

# Name of the third thing the ring holds.
welcome-held-switches = Die Schalter

# Line under it.
welcome-held-switches-line = Ton, Bildschirm, Wachhalten und Energie.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Mit dem Zeiger

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Mit der Tastatur

# Note under the picture of the Super key.
welcome-ring-super-note = Die Super-Taste hat oft ein Windows-Logo.

# Head of a card of apps: web browsers.
welcome-kind-browser = Webbrowser

# Head of a card of apps: email programs.
welcome-kind-email = E-Mail

# Head of a card of apps: chat programs.
welcome-kind-chat = Chat

# Head of a card of apps: office programs.
welcome-kind-office = Büro

# Head of a card of apps: music and video players.
welcome-kind-media = Musik und Video

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Bilder und Zeichnen

# Head of a card of apps: notes and writing.
welcome-kind-notes = Notizen und Passwörter

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF und Spiele

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Im Moment ist keine davon auf Flathub.

# Beside an app that has been installed.
welcome-app-installed = Installiert

# Beside an app that is going in now.
welcome-app-installing = Wird installiert

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, gewählt

# Name of the round button on an app's row that picks it.
welcome-app-pick = { $name } wählen

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = Im Ring

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Die Plugins

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = In Apps, unter Plugins.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Plugins in Apps öffnen

# Said in the plugins card while there are none to show.
welcome-plugins-none = Die angebotenen Plugins sind in Apps aufgelistet.

# Head of the card on the crash reports page.
welcome-reports-card = Absturzberichte

# The one question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-question = Trindle verbessern helfen, indem ein Bericht gesendet wird, wenn etwas schiefgeht? Du siehst ihn immer zuerst.

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = Ohne deinen Klick wird nichts gesendet. Du siehst zuerst den ganzen Bericht, und er enthält keine Dateien, Passwörter oder Wi-Fi-Namen.

# Second note under the question about crash reports.
welcome-reports-settings = Du kannst das jederzeit in den Einstellungen unter Absturzberichte ändern.

# The answer that turns crash reports off.
# SAFETY: needs a human review before it ships
welcome-reports-no = Nein

# The answer that turns crash reports on.
# SAFETY: needs a human review before it ships
welcome-reports-yes = Ja, vorher fragen

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Einen Bericht senden, wenn etwas schiefgeht

# Head of a card on the last page.
# SAFETY: needs a human review before it ships
welcome-know-boxes = Apps leben in ihren eigenen Boxen

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Eine App fragt, bevor sie auf deine Dateien, Kamera oder dein Mikrofon zugreift.

# Head of a card on the last page.
welcome-know-updates = Sicherheitsupdates installieren sich selbst

# Line in that card.
welcome-know-updates-line = Jeden Tag, ohne zu fragen. Andere Updates warten auf dich.

# Head of a card on the last page.
welcome-know-ring = Einstellungen und Apps sind im Ring

# Line in that card.
welcome-know-ring-line = In seiner Mitte, über der Uhrzeit.

# Head of a card on the last page.
welcome-know-help = Hilfe gibt es in den Einstellungen

# Line in that card.
welcome-know-help-line = Dieser Rundgang ist unter Alle Apps als Willkommen, wann immer du ihn wieder möchtest.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Ein kleiner Bildschirm mit nichts darauf außer dem Ring in der Mitte: die Uhrzeit in seinem Zentrum, Lieblings-Apps rundherum.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = Der Ring auf einem kleinen Bildschirm, geöffnet, mit den Symbolen einiger Plugins in einer Reihe in seiner Mitte, unter der Uhrzeit, wo das Symbol eines Plugins nach der Installation sitzt.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Zwei kleine Zeichnungen, wo die Punkte erscheinen: auf dem Reiter eines Fensters im Ring und oben in der Mitte des Bildschirms.

welcome-dots-where-tab = Am Reiter eines Fensters

welcome-dots-where-corner = Oben in der Mitte des Bildschirms

welcome-picture-ring = Ein kleiner Bildschirm. Der Zeiger bewegt sich ganz an den oberen Rand und doppelklickt, und der Ring öffnet sich in der Mitte.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Die Super-Taste wird gedrückt, wodurch sich der Ring öffnet.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Wie viel weißt du schon?

# Sentence under that title.
welcome-sentence-level = Wähle das Nächstliegende, dann lässt die Tour aus, was du schon kennst.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Neu bei Linux

# Line in that answer's card.
welcome-level-new-line = Jede Seite, auch was jeder Linux-Desktop kann.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Linux schon benutzt

# Line in that answer's card.
welcome-level-linux-line = Nur, was zu Trindle gehört. Die Linux-Grundlagen werden übersprungen.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Trindle schon benutzt

# Line in that answer's card.
welcome-level-trindle-line = Beendet die Tour jetzt und zeigt sie nicht mehr an.

# Title of the page about files and folders.
welcome-page-files = Deine Dateien

# Sentence under that title.
welcome-sentence-files = Alles, was dir gehört, liegt in deinem Home-Ordner.

# Title of the page about getting software.
welcome-page-software = Software bekommen

# Sentence under that title.
welcome-sentence-software = Apps kommen aus „Apps“, nicht aus heruntergeladenen Setup-Dateien.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = Das Administratorpasswort

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Änderungen am ganzen Computer verlangen zuerst ein Passwort.

# Title of the page about snapshots.
welcome-page-snapshots = Snapshots

# Sentence under that title.
welcome-sentence-snapshots = Trindle speichert Kopien des Systems, damit ein Update rückgängig gemacht werden kann.

# Title of the page about the person's own words.
welcome-page-words = Deine eigenen Wörter

# Sentence under that title.
welcome-sentence-words = Ändere jedes Wort, das Trindle zeigt, oder füge eine eigene Sprache hinzu.

# Head of a card on the files page.
welcome-files-home = Ein Home-Ordner

# Line in that card.
welcome-files-home-line = Dokumente, Downloads und Bilder liegen darin. „Ordner“ öffnet sich dort.

# Head of a card on the files page.
welcome-files-drives = Keine Laufwerksbuchstaben

# Line in that card.
welcome-files-drives-line = Ein USB-Stick oder ein anderer Datenträger erscheint in „Ordner“, sobald du ihn einsteckst.

# Head of a card on the files page.
welcome-files-hidden = Versteckte Namen

# Line in that card.
welcome-files-hidden-line = Ein Name, der mit einem Punkt beginnt, etwa .config, ist versteckt. Darin liegen die Einstellungen einer App.

# Head of a card on the files page.
welcome-files-bin = Der Papierkorb

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Eine gelöschte Datei bleibt im Papierkorb, bis du ihn leerst.

# Head of a card on the software page.
welcome-software-apps = Ein Ort für Apps

# Line in that card.
welcome-software-apps-line = Öffne „Apps“ über den Ring, um Programme zu finden, zu installieren und zu entfernen.

# Head of a card on the software page.
welcome-software-exe = Keine Setup-Dateien

# Line in that card.
welcome-software-exe-line = Eine Windows-Setup-Datei mit der Endung .exe läuft hier nicht. Suche die App in „Apps“.

# Head of a card on the software page.
welcome-software-updates = Updates an einem Ort

# Line in that card.
welcome-software-updates-line = „Apps“ und „Einstellungen“ halten alles aktuell, daher braucht keine App ihren eigenen Updater.

# Head of a card on the software page.
welcome-software-remove = Sauber entfernen

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Beim Entfernen einer App verschwindet alles davon, und nichts bleibt zurück.

# Head of a card on the administrator password page.
welcome-admin-why = Warum gefragt wird

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Eine Installation für alle oder eine Änderung an einem Datenträger betrifft jeden Benutzer, deshalb fragt der Computer zuerst.

# Head of a card on the administrator password page.
welcome-admin-who = Wessen Passwort

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = Das eines Administrators, meist dessen, mit dem du dich anmeldest.

# Head of a card on the administrator password page.
welcome-admin-not = Wann nicht gefragt wird

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Apps, die du nur für dich installierst, und deine eigenen Dateien brauchen es nie.

# Head of a card on the snapshots page.
welcome-snap-taken = Für dich erstellt

# Line in that card.
welcome-snap-taken-line = Auf einem Btrfs-System wird vor und nach jeder Softwareänderung einer erstellt.

# Head of a card on the snapshots page.
welcome-snap-list = In den Einstellungen aufgelistet

# Line in that card.
welcome-snap-list-line = „Einstellungen“, „Snapshots“ listet sie auf. Die Kamera erstellt jetzt einen.

# Head of a card on the snapshots page.
welcome-snap-boot = Nutzbar, wenn das System nicht startet

# Line in that card.
welcome-snap-boot-line = Wähle im Startmenü „Trindle snapshots“, dann einen Snapshot.

# Head of a card on the your own words page.
welcome-words-change = Ein Wort ändern

# Line in that card.
welcome-words-change-line = Finde ein beliebiges Wort, das Trindle zeigt, und tippe dein eigenes ein. „Open it, brew“ kann „Click here to open the file“ ersetzen.

# Head of a card on the your own words page.
welcome-words-share = Ein Blatt teilen

# Line in that card.
welcome-words-share-line = Exportiere deine Wörter als Datei oder importiere eine, die dir ein Freund geschickt hat. Sie wird geprüft, bevor sie verwendet wird.

# Head of a card on the your own words page.
welcome-words-all = Jedes Programm auf einmal

# Line in that card.
welcome-words-all-line = Ganz Trindle liest seine Wörter von einer Stelle, daher zeigt sich eine Änderung überall.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Es steht in „Einstellungen“, „Region und Sprache“. Die Hilfe erklärt es Schritt für Schritt.

# Name of the round button that opens that Settings page.
welcome-words-open = „Region und Sprache“ öffnen
