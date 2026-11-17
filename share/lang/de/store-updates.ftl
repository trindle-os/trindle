
# machine translated (German), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian-Pakete

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Zurückgehalten bei Version { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Zurückgehalten; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Zurückgehalten bei Version { $have }

# On a thing held back with no update waiting.
store-upd-held = Zurückgehalten

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Zurückgehalten bei Version { $have }. Es gibt noch nichts Neueres.

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Zurückgehalten; noch nichts Neueres

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 Update
       *[other] { $n } Updates
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] 1 Update ist bereit
       *[other] { $n } Updates sind bereit
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, insgesamt { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 wird zurückgehalten
       *[other] { $n } werden zurückgehalten
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Starte neu, um das Firmware-Update abzuschließen.

# Headline while the check runs.
store-upd-checking = Es wird nach Updates gesucht.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = { $name } wird aktualisiert. Du kannst weiterarbeiten, während es installiert wird.

# Headline while all the updates go in.
store-upd-installing-all = Die Updates werden installiert. Du kannst weiterarbeiten, während sie installiert werden.

# Headline when the updates are in and the computer must start again.
store-upd-restart = Starte den Computer neu, um die Installation der Updates abzuschließen.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Starte neu, um die Installation abzuschließen.

# Added to the headline with no internet.
store-upd-no-more = Dieser Computer ist nicht mit dem Internet verbunden, deshalb kann Trindle nicht nach weiteren Updates suchen.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Keine Updates bereit

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Trindle kennt keine wartenden Updates. Es kann nicht nach weiteren suchen, solange dieser Computer nicht mit dem Internet verbunden ist.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = die Apps

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle und Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = die Firmware

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Trindle konnte nicht nach Updates für { $names } suchen.

# Headline when everything is up to date.
store-upd-current = Alles ist auf dem neuesten Stand.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Zuletzt geprüft um { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Ohne Internet nicht prüfbar

# Red words under a card when a check took too long.
store-upd-too-long = Die Prüfung hat zu lange gedauert.

# Tip of the round button that checks again.
store-upd-check-again = Erneut prüfen

# Main button while the check runs.
store-upd-checking-main = Wird geprüft

# Main button while the updates go in.
store-upd-installing-main = Wird installiert

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] 1 Update installieren
       *[other] { $n } Updates installieren
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = Jetzt neu starten

# Main button that checks for updates.
store-upd-check-for = Nach Updates suchen

# Tip of the round button that releases a thing held back.
store-upd-release = Updates für { $name } erlauben

# Tip of the round button that holds a thing's updates back.
store-upd-hold = { $name } zurückhalten

# A card's state while it is being checked.
store-upd-status-checking = Wird geprüft

# A card's state while its updates go in.
store-upd-status-installing = Wird installiert

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Prüfung nicht möglich. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } zurückgehalten

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } zurückgehalten

# A card's state when nothing waits.
store-upd-up-to-date = Aktuell

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Von { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Von { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Ein gemeinsamer Teil, von { $from }

# Note under the Apps card.
store-upd-yours-alone = Diese Apps gehören nur dir, daher werden sie ohne Passwort aktualisiert. Beim Update einer App werden auch die gemeinsamen Teile installiert, die sie braucht.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Ein zurückgehaltenes Update bleibt bei der aktuellen Version. Es wird übersprungen, wenn du alle Updates installierst, hier und am Ring, bis du Updates wieder erlaubst.

# Heading of the Apps card.
store-upd-card-apps = Apps

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Alle Plugins sind auf dem neuesten Stand.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] Das Update von 1 Plugin wird zurückgehalten.
       *[other] Die Updates von { $n } Plugins werden zurückgehalten.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 Plugin hat ein Update
       *[other] { $n } Plugins haben ein Update
    }

# Heading of the Plugins card.
store-upd-card-plugins = Plugins

# Held back card: nothing is held back.
store-upd-nothing-held = Nichts wird zurückgehalten.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Nicht zurückgehalten

# Note under the Held back card: what releasing does.
store-upd-release-why = Sobald du Updates wieder erlaubst, wird es mit den anderen installiert, hier und am Ring.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Die Firmware eines Geräts

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = die Firmware von Gerät { $id }

# Heading of the Held back card.
store-upd-card-held = Zurückgehalten

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } auf { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 Update, das für sich installiert wird
       *[other] { $n } Updates, die zusammen installiert werden
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Von Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Es werden { $shown } von { $total } Debian-Updates angezeigt.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Dieser Computer hat Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle und Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Debian-Pakete sind darauf ausgelegt, zusammenzuarbeiten. Wer nur eines installiert, zieht vielleicht andere nach oder bringt sie aus dem Takt. Darum werden alle zusammen installiert.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Die Updates von Trindle und Debian bringen Sicherheitskorrekturen für den ganzen Computer und können deshalb nicht zurückgehalten werden.

# Heading of the card about the daily security updates.
store-upd-card-security = Sicherheitsupdates

# On the security card: its state.
store-upd-security-on = An, jeden Tag

# On the security card: that holding back never stops these.
store-upd-never-held = Das Zurückhalten eines Updates stoppt Sicherheitsupdates nie.

# Firmware card: the computer has no updater.
store-upd-fw-none = Dieser Computer hat kein Firmware-Update-Programm.

# Firmware card: restart to finish.
store-upd-fw-restart = Zum Abschluss neu starten

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Version { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Neue Firmware wird beim Neustart des Computers installiert.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Installiere fwupd, um die Firmware aktuell zu halten.

# Heading of the Firmware card.
store-upd-card-firmware = Firmware

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd konnte die neueste Liste nicht abrufen.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub war nicht erreichbar.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Trindle konnte die Listen von Debian nicht abrufen.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Trindle konnte die Listen von Debian nicht abrufen. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Trindle konnte es nicht zurückhalten, weil es die Liste der zurückgehaltenen Updates nicht speichern konnte: { $why }.

# The same, for a release.
store-upd-not-released-list = Trindle konnte Updates nicht wieder erlauben, weil es die Liste der zurückgehaltenen Updates nicht speichern konnte: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Es ist in Apps zurückgehalten, aber Trindle konnte das in Flatpak nicht ändern. Ein anderes Programm, das Apps aktualisiert, könnte es trotzdem installieren.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Updates sind in Apps wieder erlaubt, aber Trindle konnte das in Flatpak nicht ändern, daher lässt Flatpak es vielleicht weiterhin aus. Halte es zurück und erlaube dann Updates erneut.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak meldete: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle

# Said when a security fix for a boxed app can't be downloaded. { $package } is the package, { $version } the fixed version.
store-patch-not-fetched = Trindle konnte { $package } { $version } nicht herunterladen.

# Said when a security fix for a boxed app was downloaded but isn't there to use.
store-patch-not-found = Trindle konnte das heruntergeladene Paket nicht finden.


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Die automatische Suche nach Updates ist ausgeschaltet.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Nicht geprüft

# Tip of the round button that ends an update that is running.
store-upd-stop = Anhalten

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Wird angehalten

# Under a row whose update was stopped.
store-upd-stopped = Angehalten. Drücke auf Aktualisieren, um weiterzumachen.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Angehalten. Installiere noch einmal, um weiterzumachen.

# Note under the Apps card.
store-upd-steam-restarts = Steam startet neu, sobald kein Spiel läuft.
