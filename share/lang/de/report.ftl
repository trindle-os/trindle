
# machine translated (German), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Ein Problem melden

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Etwas ist schiefgegangen. Einen Bericht senden?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = Dabei werden der Fehler, eine Zusammenfassung deines Computers und die letzten Zeilen des Protokolls gesendet. Namen, E-Mail-Adressen, Netzwerkadressen und WLAN-Namen werden entfernt. Dateinamen und Webadressen können in den Protokollzeilen noch vorkommen. Deine Dateien und Passwörter werden nicht gesendet.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = Was ist passiert?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Welche App?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Weiß ich nicht

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Ein anderes Programm

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = Die Arbeitsumgebung

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
# SAFETY: needs a human review before it ships
report-send = Senden

# Button at the bottom of the window when the person opened it themselves.
# SAFETY: needs a human review before it ships
report-cancel = Abbrechen

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
# SAFETY: needs a human review before it ships
report-dont-send = Nicht senden

# Button that opens the exact text of the report.
report-show-details = Details anzeigen

# Button that closes the exact text of the report.
report-hide-details = Details ausblenden

# Button under the shown report text that copies it.
report-copy = Kopieren

# Button under the shown report text that saves it as a file in Downloads.
report-save = Speichern

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Berichte können noch nicht gesendet werden.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Wird geprüft.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Zu viele Berichte auf einmal.

# Written beside the Send button once the report has been sent.
report-already-sent = Schon gesendet.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Schon gespeichert.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Warte auf eine Verbindung.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Warte auf eine Verbindung. { $why }

# Under the typing box after the report was sent.
report-sent = Gesendet. Danke.

# Under the typing box while the report is being posted.
report-sending = Wird gesendet.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Gespeichert als Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Gib deinen Buildmorphic-Benutzernamen ein, keine E-Mail-Adresse. Verwende kein @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Ein Benutzername muss 2 bis 31 Zeichen lang sein und darf Kleinbuchstaben, Zahlen und Bindestriche enthalten.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Ein GitHub-Name darf nur Buchstaben, Zahlen und Bindestriche enthalten.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle konnte den Bericht nicht für später speichern: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Es konnte nicht gespeichert werden: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = keine Berechtigung

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = das Laufwerk ist voll

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = der Ordner fehlt

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = ein Laufwerksfehler

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl fehlt.

# Why a report could not be posted: the post could not be started.
report-not-posted = Es konnte nicht gesendet werden.

# Why a report could not be posted: the computer is offline.
report-no-connection = Keine Verbindung.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Die Adresse hat mit { $code } geantwortet.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Schreibe noch ein paar Worte dazu, was passiert ist.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Das Protokoll wird gelesen.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Es gibt kein Protokoll, das mit dem Bericht gesendet werden kann, deshalb lässt er sich nicht senden. Speichere ihn stattdessen.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Optionales Bild. Nur der Projektinhaber sieht es.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = PNG-Bild auswählen

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Bildschirmfoto aufnehmen

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Bild entfernen

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Dein Bild, { $width } mal { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = PNG-Bild auswählen

report-picture-filter = PNG-Bilder

# Under the picture while it is being read.
report-picture-reading = Wird gelesen.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Kein Bildschirmfoto möglich.

report-picture-no-chooser = Trindle konnte die Dateiauswahl nicht öffnen.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Das ist kein PNG-Bild.

report-picture-too-big = Es ist über { $megabytes } MB groß.

report-picture-too-large = Das Bild ist zu groß. Es darf höchstens { $side } Pixel breit und { $side } Pixel hoch sein.

report-picture-damaged = Das Bild ist beschädigt.

report-picture-unreadable = Es ließ sich nicht lesen.



report-sent-as-bug = Gesendet. Die Nummer deines Fehlerberichts ist { $number }. Danke.


# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle befindet sich in der Beta. Berichte sind anonym: Namen, E-Mail-Adressen und Home-Ordner werden entfernt. Ein Benutzername ist optional.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic-Benutzername (optional)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle konnte den Bericht nicht senden. { $why } Drücke auf Senden, um es erneut zu versuchen.
