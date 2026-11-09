
# machine translated (German), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } App
       *[other] { $count } Apps
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } Update wartet
       *[other] { $count } Updates warten
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } zur Auswahl auf Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = alles von dir ist auf dem neuesten Stand

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Keine Apps passen zu „{ $text }“.

# Line under the title of an empty page of apps.
store-grid-none-here = Hier gibt es noch keine Apps.

# Advice in the empty page of a search.
store-grid-try-shorter = Versuche ein kürzeres Wort oder stöbere links in einer Kategorie.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Suche oben nach dem Namen oder stöbere links in einer anderen Kategorie.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $shown } von { $apps } werden angezeigt.

# Link under a page of apps that shows more of them.
store-grid-show-more = { $next } weitere anzeigen

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Die App-Liste wird von Flathub abgerufen. Beim ersten Mal dauert das eine Minute.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Apps konnte Flathub nicht erreichen.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Dieser Computer ist nicht mit dem Internet verbunden. Apps versucht es von selbst erneut, sobald er es ist.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Die Apps zur Auswahl kommen über das Internet von Flathub.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } und die Einstellungen für Apps, links, funktionieren auch ohne.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } und die Einstellungen für Apps, links, funktionieren auch ohne Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } und die Einstellungen für Apps, links, funktionieren auch ohne Internet.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Erneut versuchen

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Ergebnisse

# Red words under the search box when no web browser opens a link.
store-no-browser = Es gibt keinen Webbrowser, der { $url } öffnen könnte. Hole dir zuerst einen in Apps.

# Said in the card or under the title while the lists are being read.
store-looking = Es wird nachgesehen…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Sie ist jetzt unter Installiert und im Ring

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Entfernt

# The word on an app's dial when installing or removing did not work.
store-done-not = Nicht erledigt

# The word on an app's dial when it was installed.
store-done-installed = Installiert

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak hat abgelehnt.

# Tab of the Installed page that shows everything.
store-tab-all = Alle

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak-Dateien

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImages

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb-Dateien

# Tab of the Installed page that shows plugins.
store-tab-plugins = Plugins

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Es ist noch nichts installiert. Unter Entdecken gibt es viel zum Ausprobieren.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 Plugin ist installiert
       *[other] { $n } Plugins sind installiert
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 Programm ist installiert
       *[other] { $n } Programme sind installiert
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } Dinge sind installiert

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, von Flathub
       *[other] { $start }, alle von Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, aus einer Flatpak-Datei
       *[other] { $start }, alle aus Flatpak-Dateien
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, ein AppImage
       *[other] { $start }, alle AppImages
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, aus einer .deb-Datei
       *[other] { $start }, alle aus .deb-Dateien
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } von Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 aus einer Flatpak-Datei
       *[other] { $n } aus Flatpak-Dateien
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
       *[other] { $n } AppImages
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 App
       *[other] { $n } Apps
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 aus einer .deb-Datei
       *[other] { $n } aus .deb-Dateien
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 Plugin
       *[other] { $n } Plugins
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Es ist auf dem neuesten Stand.

# End of the Installed line when everything is up to date.
store-installed-state-all = Alle sind auf dem neuesten Stand.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 hat ein wartendes Update.
       *[other] { $n } haben wartende Updates.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } von { $total } Programmen passen zu „{ $find }“.

# The same when plugins are among the things.
store-installed-match-things = { $shown } von { $total } Dingen passen zu „{ $find }“.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = AppImage hinzufügen

# Placeholder of the typing box on the Installed page.
store-installed-search = Installiertes durchsuchen

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = Das AppImage konnte nicht hinzugefügt werden.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Apps, die du über Entdecken, von GitHub oder als AppImages installierst, Programme aus .deb-Dateien und Plugins erscheinen hier.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Nichts Installiertes passt zu „{ $find }“.

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = Entfernen fragt zweimal: Drücke es innerhalb von drei Sekunden noch einmal.

# On an installed thing's line: its version.
store-installed-version = Version { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = zurückgehalten

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = zurückgehalten; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } wird aktualisiert

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } aktualisieren

# Tip of the round button that shows an installed app's page.
store-installed-about = Info

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = Wird entfernt...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Es gibt keinen Sitzungsbus, um nach der Datei zu fragen.

# Title of the window that asks which AppImage to add.
store-add-choose-title = AppImage wählen

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImages

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Die Dateiauswahl konnte nicht aufgerufen werden.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Wird hinzugefügt...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } ist schon da: siehe Installiert.

# Words once an AppImage is in the ring.
store-add-in-ring = Hinzugefügt. Es steht jetzt in Ihrer Liste.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Das ist keine Flatpak-Referenz, die dieser Computer verwenden kann.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = { $name } wird von { $host } installiert...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } ist installiert.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak wollte es nicht installieren.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = { $name } konnte nicht installiert werden. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Wird ersetzt...

# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } ist schon da

# Small heading over the AppImage being brought.
store-replace-new = Das neue

# Small heading over the AppImage already there.
store-replace-old = Das vorhandene

# Tip of the round button that keeps the AppImage already there.
store-replace-skip = Überspringen: das vorhandene behalten

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Ersetzen

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = { $name } entfernen?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Ihre gespeicherten Daten belegen { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Sie hat keine gespeicherten Daten.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Ihre gespeicherten Daten werden gezählt...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Gelöschte Daten lassen sich nicht wiederherstellen.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Entfernen, Daten behalten

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Entfernen und Daten löschen

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Nur die Daten löschen

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Es gibt keine gespeicherten Daten zum Löschen.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Abbrechen: beides behalten

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Ihre gespeicherten Daten sind gelöscht.

# Words while only the data is being deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-going = Gespeicherte Daten werden gelöscht...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Es wurde nicht fertig und nannte keinen Grund.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Zeigen, wo das Problem liegt

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = GitHub durchsuchen: eintippen, dann Eingabetaste drücken

# Heading of the card of matches from AppImageHub.
store-gh-hub = Auf AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Drücke die Eingabetaste, um auch ganz GitHub zu durchsuchen.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = GitHub wurde nicht gefragt: Es gibt kein Internet dafür.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub wird gefragt...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = Auf GitHub gibt es nichts weiter.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] In { $n } Projekt wird nach AppImage- und Flatpak-Dateien gesucht...
       *[other] In { $n } Projekten wird nach AppImage- und Flatpak-Dateien gesucht...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Keiner der Treffer auf GitHub hat eine AppImage- oder Flatpak-Datei für diesen Computer.

# Heading of the card of matches from GitHub.
store-gh-on = Auf GitHub

# Heading of the card that holds the search box.
store-gh-search-card = GitHub durchsuchen

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Apps auf GitHub als AppImage- oder Flatpak-Dateien. Niemand prüft diese: Installiere nur von Leuten, denen du vertraust.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = GitHub kann nicht durchsucht werden, solange dieser Computer nicht mit dem Internet verbunden ist.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak und AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Dateien ansehen

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = Installieren

# Link to a project's page on GitHub.
store-gh-open = Auf GitHub öffnen

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, Version { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = GitHub-Schlüssel

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = GitHub-Schlüssel: einer ist gespeichert

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Du brauchst mit ziemlicher Sicherheit keinen. Ohne Schlüssel beantwortet GitHub 60 Anfragen pro Stunde von diesem Computer, mit einem 5.000. Der Schlüssel hebt nur dieses Limit an. Er wird nur verwendet, um öffentliche Dateien anzusehen, und für nichts sonst: Er gibt Trindle keinen Zugang zu deinem Konto, und beim Erstellen setzt du keine Berechtigungen. Er wird auf diesem Computer in deinem Schlüsselbund aufbewahrt und nur an GitHub gesendet.

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = Erstelle auf GitHub einen Schlüssel, gib ihm einen beliebigen Namen, lass alle Berechtigungen unverändert, drücke Generate token und füge dann hier ein, was angezeigt wird.

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = Er konnte nicht entfernt werden: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = Er konnte nicht gespeichert werden: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Leer lassen, um ihn zu behalten

# Placeholder of the GitHub key's box.
store-gh-key-paste = Schlüssel einfügen

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Schlüssel entfernen

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Schlüssel auf GitHub erstellen

# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = Schlüssel speichern

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Nichts in der neuesten Version lässt sich hier installieren.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Hinzugefügt

# Line under the title of the Move your apps page.
store-bk-line = Deine Apps und Einstellungen, in einer Datei zum Mitnehmen.

# Heading of the card that makes a backup.
store-bk-card = Sichern

# Tip of the round button while the backup is being made.
store-bk-making-tip = Die Sicherung wird erstellt

# Main button that makes the backup.
store-bk-make = Sicherung erstellen

# Tip of the round button that shows the backup in Folders.
store-bk-show = In Ordner anzeigen

# Name of the switch for the list of apps in a backup.
store-bk-apps = Apps

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] Die Liste deiner 1 Flatpak-App.
       *[other] Die Liste deiner { $n } Flatpak-Apps.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } Der andere Computer installiert sie frisch von Flathub, deshalb bleibt die Datei klein.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Eigene Daten der Apps

# Under the Apps' own data switch.
store-bk-data-sub = Einstellungen und gespeicherte Arbeit jeder App, ohne ihre Zwischenspeicher.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImages

# Under the AppImages switch.
store-bk-appimages-sub = Vollständig gesichert, da es einzelne Dateien sind.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle-Einstellungen

# Under the Trindle settings switch.
store-bk-settings-sub = Der Ring, die Hintergrundwahl, Farben, Tastatur, Energie, Standard-Apps und dein Bild.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Jetzt { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Wohin sie kommt

# Under that row: the folder and the file's name.
store-bk-where-sub = In deinen Home-Ordner, als { $name }

# Heading of the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-card = Eine Sicherung zurückspielen

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Sicherungsdatei wählen

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Eine Trindle-Sicherung oder eine von { $flat }.

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = Ihre Apps werden installiert und ihre Daten und Einstellungen zurückgespielt. Deine Dokumente und Bilder sind nicht enthalten: Kopiere diese selbst hinüber.

# Said while a backup is being made.
store-bk-making = Sicherung läuft... das kann einige Minuten dauern.

# Said when a backup is made: the file and its size.
store-bk-made = Es ist { $path } ({ $size }). Kopiere sie auf einen USB-Stick oder in einen Cloud-Speicher, um sie mitzunehmen.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Es wurde keine Sicherung gewählt, weil Trindle keine Verbindung zur Dateiauswahl hat.

# Title of the window that asks which backup to use.
store-bk-choose-title = Sicherung wählen

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle- und Flat-Sicherungen

# Red words when the file chooser gave no answer.
store-bk-no-answer = Es wurde keine Sicherung gewählt, weil die Dateiauswahl nicht geantwortet hat.

# Said while a backup file is read.
store-bk-reading = Die Sicherung wird gelesen...

# Said when a backup was put back.
store-bk-logout = Melde dich ab und wieder an, um alle Einstellungen zu sehen.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Melde dich ab und wieder an, um alle Einstellungen zu sehen.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 App, { $with } mit ihren Einstellungen. Start läuft...
       *[other] { $apps } Apps, { $with } mit ihren Einstellungen. Start läuft...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Noch keine eigenen Quellen von Herstellern hinzugefügt.
        [one] 1 Quelle eines Herstellers hinzugefügt, mit den Updates von Debian aktuell gehalten.
       *[other] { $n } Quellen von Herstellern hinzugefügt, mit den Updates von Debian aktuell gehalten.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Hinzugefügt und aktuell gehalten.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. Die Quelle ist hinzugefügt, aber das Programm ist nicht installiert.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Auf anderem Weg installiert.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Es und seine Quelle entfernen

# Link to a maker's page, naming the program.
store-src-open = { $name } öffnen

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Bei Tailscale anmelden

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = { $name } hinzufügen

# Heading of the card of makers' sources that can be added.
store-src-ready = Bereit zum Hinzufügen

# Name of the field for the address of a source.
store-src-address = Adresse

# Under the address field.
store-src-address-sub = Die Adresse des Repositorys, aus der Installationsanleitung des Herstellers.

# Name of the field for the suite of a source.
store-src-suite = Suite

# Under the suite field.
store-src-suite-sub = Meist trixie oder stable.

# Name of the field for the components of a source.
store-src-components = Komponenten

# Under the components field.
store-src-components-sub = Meist main.

# Name of the field for a source's signing key.
# SAFETY: needs a human review before it ships
store-src-key = Signaturschlüssel

# Under the signing key field.
# SAFETY: needs a human review before it ships
store-src-key-sub = Die Adresse seines Signaturschlüssels.

# Name of the field for the program to install from a source.
store-src-program = Zu installierendes Programm

# Under the program field.
store-src-program-sub = Sein Paketname, wenn du es jetzt installieren willst.

# Placeholder of the program field.
store-src-program-hint = optional

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = Diese Quelle hinzufügen

# Heading of the card that adds another source.
# SAFETY: needs a human review before it ships
store-src-another = Weitere Quelle hinzufügen

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Eine Quelle kann Updates für den ganzen Computer installieren, daher füge nur Hersteller hinzu, denen du vertraust. Jede fragt nach dem Passwort eines Administrators.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Diese Quelle entfernen

# Heading of the card of sources already on the computer.
store-src-already = Schon auf diesem Computer

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Beide Adressen müssen mit https:// beginnen

# Red words when the suite or components are not plain words.
store-src-bad-words = Suite und Komponenten müssen einfache Wörter sein.

# Red words when the program is not a package name.
store-src-bad-package = Das ist kein Paketname.

# Word on a source's dial while its job starts.
store-src-starting = Start

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Die Anmeldeseite von Tailscale wird geöffnet...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Melde dich auf der geöffneten Seite an; das erledigt sich von selbst.

# Said under Tailscale's row when it is connected.
store-src-connected = Verbunden

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = Die Verbindung zu den Apps ist nicht verschlüsselt.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = Nicht verschlüsselt

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Diese Adresse ist nicht verschlüsselt. Jemand auf dem Weg könnte sehen, was gesendet wird, und die Sicherung durch eine andere ersetzen.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Ich verstehe und akzeptiere dieses Risiko

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Zurückspielen

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Abbrechen

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Ihre gespeicherten Daten konnten nicht gezählt werden.

store-hub-warning = AppImageHub listet nur Projekte auf: Niemand prüft oder scannt sie, und sie müssen nicht signiert sein. Trindle führt jedes AppImage in einer Box mit privatem Home-Ordner aus: Anfangs darf es das Internet und den Ton nutzen und sieht keine deiner Dateien.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = Woher ein AppImage kommt, ist ein Hinweis, keine Prüfung.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = Auf GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = Auf GitHub: { $repo }, { $stars } Sterne

# Where an AppImage comes from: a website, by its name.
store-from-site = Nicht auf GitHub: von einer Website, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Herkunft unbekannt

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Seine Website öffnen

# When an app was last updated.
store-age-updated = Zuletzt aktualisiert: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Zuletzt aktualisiert: { $date }. Seit mehr als sechs Monaten nicht aktualisiert, daher kann es unbehobene Sicherheitslücken haben.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Zuletzt aktualisiert: unbekannt

# When an app's project was made.
store-age-created = Erstellt am { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Erstellt: unbekannt

# A date the source does not give, on a line of Facts.
store-age-not-known = Unbekannt

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Bevor du eines hinzufügst

# Title of the card that lists the AppImages.
store-browse-list = AppImages

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] { $count } AppImage ist bei AppImageHub gelistet.
       *[other] { $count } AppImages sind bei AppImageHub gelistet.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Liste von AppImageHub wird geholt…

# The same when the list could not be fetched.
store-browse-failed = Die Liste von AppImageHub konnte nicht geholt werden.

# The same with no internet and no copy kept.
store-browse-offline = Die Liste von AppImageHub kann nicht geholt werden, solange dieser Computer nicht mit dem Internet verbunden ist.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub listet keines dieser Art.

# Under the rows: how many are shown of how many.
store-browse-showing = { $shown } von { $n } werden angezeigt.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Zu den Apps von Trindle hinzufügen

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Details und Dateien ansehen



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = die Suche nach Updates ist aus

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT kommt zurück, sobald OpenAI seinen Download signiert.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } zur Auswahl bei Flathub und AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } zur Auswahl bei AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Beide

# The same: only Flatpaks.
store-show-flatpaks = Flatpaks

# The same: only AppImages.
store-show-appimages = AppImages

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImages werden nicht geprüft. Niemand kontrolliert oder scannt sie.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle führt jedes AppImage in einer Box mit einem eigenen privaten Home-Ordner aus: Anfangs darf es das Internet und den Ton nutzen und sieht keine Ihrer Dateien.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Nicht mehr anzeigen

# Button on that card that goes on with the install.
store-appimage-warn-ok = OK

# Button on that card that gives the install up.
store-appimage-warn-cancel = Abbrechen

