
# machine translated (German), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Ein Problem melden

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Etwas ist schiefgegangen. Einen Bericht senden?

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = Dabei werden der Fehler, eine Zusammenfassung des Computers und die letzten Zeilen des Protokolls gesendet, wobei Namen, E-Mail-Adressen, Netzwerkadressen und Wi-Fi-Namen entfernt sind. Dateinamen und Webadressen können in den Protokollzeilen noch vorkommen. Deine Dateien und Passwörter werden nicht gesendet.

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = Was hast du gerade gemacht? (freiwillig)

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
report-not-taken = Berichte werden noch nicht angenommen.

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
report-no-at = Nimm deinen Buildmorphic-Benutzernamen, keine E-Mail-Adresse, also kein @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Ein Benutzername besteht aus 2 bis 31 Kleinbuchstaben, Zahlen oder Bindestrichen.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Ein GitHub-Name enthält nur Buchstaben, Zahlen und Bindestriche.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Es konnte nicht für später gespeichert werden: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Es konnte nicht gespeichert werden: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = keine Berechtigung

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = das Laufwerk ist voll

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = der Ordner fehlt

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = das Laufwerk hat es abgelehnt

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl fehlt.

# Why a report could not be posted: the post could not be started.
report-not-posted = Es konnte nicht gesendet werden.

# Why a report could not be posted: the computer is offline.
report-no-connection = Keine Verbindung.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Die Adresse hat mit { $code } geantwortet.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Schreibe noch ein paar Worte, was passiert ist.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Das Protokoll wird gelesen.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Es gibt kein Protokoll, das mitgeschickt werden kann, daher lässt es sich nicht senden. Speichere es stattdessen.



report-sent-as-bug = Gesendet. Als Fehler { $number } erfasst. Danke.

