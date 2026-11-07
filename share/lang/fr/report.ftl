
# machine translated (French), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Signaler un problème

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Un problème est survenu. Envoyer un rapport ?

# Under the title: what a report holds, and what it never holds.
report-summary = Ceci envoie l'erreur, un résumé de l'ordinateur et les dernières lignes du journal, dont les noms, adresses e-mail, adresses réseau et noms de Wi-Fi ont été retirés. Des noms de fichiers et des adresses web peuvent encore figurer dans les lignes du journal. Cela n'envoie ni vos fichiers ni vos mots de passe.

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = Que faisiez-vous ? (facultatif)

# SAFETY: needs a human review before it ships
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Envoyer

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when the person opened it themselves.
report-cancel = Annuler

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = Ne pas envoyer

# Button that opens the exact text of the report.
report-show-details = Afficher les détails

# Button that closes the exact text of the report.
report-hide-details = Masquer les détails

# Button under the shown report text that copies it.
report-copy = Copier

# Button under the shown report text that saves it as a file in Downloads.
report-save = Enregistrer

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Les rapports ne sont pas encore acceptés.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Vérification.


# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Trop de rapports à la fois.

# Written beside the Send button once the report has been sent.
report-already-sent = Déjà envoyé.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Déjà enregistré.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = En attente d'une connexion.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = En attente d'une connexion. { $why }

# Under the typing box after the report was sent.
report-sent = Envoyé. Merci.

# Under the typing box while the report is being posted.
report-sending = Envoi en cours.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Enregistré sous Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Utilisez votre nom d'utilisateur Buildmorphic, pas une adresse e-mail : donc pas de @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Un nom d'utilisateur compte de 2 à 31 lettres minuscules, chiffres ou tirets.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Un nom GitHub ne contient que des lettres, des chiffres et des tirets.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Impossible de le conserver pour plus tard : { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Impossible de l'enregistrer : { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = aucune autorisation

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = le disque est plein

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = le dossier est introuvable

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = le disque a refusé

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl est absent.

# Why a report could not be posted: the post could not be started.
report-not-posted = L'envoi n'a pas pu être effectué.

# Why a report could not be posted: the computer is offline.
report-no-connection = Pas de connexion.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = L'adresse a répondu { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Écrivez quelques mots de plus pour dire ce qui s'est passé.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Lecture du journal.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Il n'y a pas de journal à envoyer avec, donc il ne peut pas être envoyé. Enregistrez-le plutôt.



report-sent-as-bug = Envoyé. Enregistré comme bogue { $number }. Merci.

