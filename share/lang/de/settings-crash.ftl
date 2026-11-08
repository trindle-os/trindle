# machine translated (German), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Absturzberichte

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = Absturzberichte Abstürze Fehler Bugs Senden Gesendet Datenschutz Telemetrie buildmorphic crash reports errors privacy telemetry

# Help line at the foot of the Crash reports page.
# SAFETY: needs a human review before it ships
settings-crash-help = Wenn eines der Trindle-Programme abstürzt, speichert das Modul für Absturzberichte eine kurze Notiz auf diesem Computer. Es sendet nichts, bis du den Sendebefehl ausführst und jedem Bericht zustimmst; dabei siehst du zuerst den ganzen Bericht. Dein Name und dein GitHub-Name stehen nur darin, wenn du sie hier eingetragen hast. Nie eine Nachricht, ein Dateiname, dein Aufenthaltsort oder etwas über andere Programme. Das Entfernen des Moduls stoppt alles.

# Heading and label of the switch for sending crashes.
# SAFETY: needs a human review before it ships
settings-crash-crashes = Abstürze

# Under the Crashes switch.
settings-crash-crashes-short = Wo im Code von Trindle es abgestürzt ist.

# Part of the help text of the What is sent card: what Crashes sends.
# SAFETY: needs a human review before it ships
settings-crash-crashes-long = Abstürze sendet das Programm, seine Version, die Art des Absturzes und wo im Code von Trindle er geschah.

# Heading and label of the switch for sending errors.
# SAFETY: needs a human review before it ships
settings-crash-errors = Fehler

# Under the Errors switch.
settings-crash-errors-short = Der feste Name des Fehlers und wo.

# Part of the help text of the What is sent card: what Errors sends.
# SAFETY: needs a human review before it ships
settings-crash-errors-long = Fehler sendet das Programm, seine Version, den festen Namen, den Trindle dem Fehler gibt, und wo in seinem Code.

# Heading and label of the switch for sending facts about the computer.
# SAFETY: needs a human review before it ships
settings-crash-computer = Über diesen Computer

# Under the About this computer switch.
settings-crash-computer-short = Linux, Prozessor, Speicher, Grafik, Bildschirme.

# Part of the help text of the What is sent card: what About this computer adds.
# SAFETY: needs a human review before it ships
settings-crash-computer-long = Über diesen Computer fügt jedem Bericht hinzu: das laufende Linux, die Art des Prozessors, den abgerundeten Speicher, Wayland oder X11, wer die Grafik gebaut hat, und wie viele Bildschirme es gibt.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Was gesendet wird

# Help text of the What is sent card: { $switches } is what each switch sends.
# SAFETY: needs a human review before it ships
settings-crash-what-about = { $switches } Berichte tragen keinen Namen, außer du gibst unten deinen Buildmorphic-Benutzernamen ein, was dir nur Anerkennung bringt.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Nichts wurde geändert, weil { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Nichts wurde geändert, weil die Datei, die es speichert, nicht so ist, wie Trindle sie geschrieben hat.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Kein Einstellungsordner.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Der Einstellungsordner konnte nicht erstellt werden.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Die Einstellung konnte nicht geschrieben werden.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Absturz ({ $signal }) in { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Fehler { $code } in { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Angenommen

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Nicht gesendet: Tracker geschlossen

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Nicht gesendet: zu groß

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Nicht gesendet: abgelehnt

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Nicht gesendet: zu viele auf einmal

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Nicht gesendet

# Said beside the link line once it is saved.
settings-crash-saved = Gespeichert

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Liste leeren

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Die Liste konnte nicht geleert werden.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Entfernt

# Heading of the line that holds the clear button.
settings-crash-kept-here = Nur auf diesem Computer gespeichert

# Under the Kept on this computer only heading.
settings-crash-newest-first = Die neuesten zuerst, bis zu fünfzig.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Noch nichts

# Under the Nothing yet heading.
settings-crash-listed-here = Jeder Bericht wird hier aufgelistet, genau so, wie er gesendet wurde.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Genau zeigen, was gesendet wurde

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }, { $n } Mal: { $answer }
       *[other] { $when }, { $n } Mal: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Bisher gesendet

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Jeder Bericht wird genau so aufgelistet, wie er gesendet wurde, die neuesten zuerst, und nur auf diesem Computer gespeichert. „Angenommen“ heißt, Buildmorphic hat ihn. „Nicht gesendet: Tracker geschlossen“ heißt, der Bugtracker von Trindle nimmt keine Berichte an; „abgelehnt“ heißt, er wurde als falsch erstellt abgelehnt; „zu viele“ heißt, es kamen zu viele auf einmal.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
# SAFETY: needs a human review before it ships
settings-crash-sum-asked = Nach einem Absturz wirst du gefragt, ob der Bericht gesendet werden soll, und der ganze Bericht wird gezeigt.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Nichts wird gesendet, und du wirst nach einem Absturz nicht gefragt.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Nichts wird gesendet: Abstürze und Fehler sind beide ausgeschaltet.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Fehler der Trindle-Programme werden anonym gemeldet.
        [both] Abstürze und Fehler der Trindle-Programme werden anonym gemeldet.
       *[crashes] Abstürze der Trindle-Programme werden anonym gemeldet.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Nach Problemen fragen und Berichte teilen

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Aus blendet alles aus und löscht wartende Berichte.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Aus. Es warteten keine Berichte.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [one] Aus. { $n } wartender Bericht gelöscht.
       *[other] Aus. { $n } wartende Berichte gelöscht.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Über die Gemeinschaft

# Heading of the line with the community button.
settings-crash-go-to-bm = Berichte gehen an Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = Die Gemeinschaft, die beim Bau von Trindle hilft.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = Benutzername

# Heading of the line with the username box.
settings-crash-your-username = Dein Buildmorphic-Benutzername

# Under the username heading.
settings-crash-username-sub = Optional. Niemals eine E-Mail-Adresse.

# Heading of a line about joining with an email address.
settings-crash-join = Mit nur einer E-Mail-Adresse beitreten

# Under the Join heading.
settings-crash-join-sub = Sammle Anerkennung und Stufen für Probleme, die du zuerst meldest.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Ein Bericht enthält nur eine feste Liste

# Under the fixed list heading.
# SAFETY: needs a human review before it ships
settings-crash-fixed-sub = Du siehst ihn zuerst, und ohne deinen Tastendruck geht nichts raus.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Berichte, die du sendest

# Help text of the Reports you send card (the information button).
# SAFETY: needs a human review before it ships
settings-crash-reports-about = Wenn eines der Programme von Trindle abstürzt, kann sich ein Fenster öffnen und fragen, ob ein Bericht gesendet werden soll. Es zeigt zuerst den genauen Text: deine Worte, das Modell und die Grafik des Computers, den Fehler und das Protokoll von kurz davor. Dateien, Passwörter und WLAN-Namen werden nie gelesen, und Namen und Adressen werden geschwärzt. Du kannst dasselbe Fenster jederzeit auch über die Schaltfläche „Problem melden“ im Ring öffnen.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = GitHub-Name

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Dein GitHub-Name

# Small words under that heading.
settings-crash-github-about = Optional. Wird als „Gemeldet von“ angezeigt.

