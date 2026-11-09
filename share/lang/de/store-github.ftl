
# machine translated (German), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = Sicher heruntergeladen, aber niemand hat diese Datei geprüft: Ihr Hersteller veröffentlicht keine Signatur.

# What a project with nothing to install here is told as.
store-gh-no-files = Die Veröffentlichungen dieses Projekts haben keine AppImage- oder Flatpak-Datei für diesen Computer.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub hat keine Veröffentlichungen für dieses Projekt.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Im Schlüsselbund wird nach einem Schlüssel gesucht

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Ein Schlüssel ist gespeichert

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Es ist kein Schlüssel gespeichert

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Auf diesem Computer gibt es keinen Schlüsselbund, deshalb kann kein Schlüssel aufbewahrt werden

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = Dein GitHub-Schlüssel liegt noch in einer einfachen Datei auf diesem Computer und wird von dort benutzt: Er konnte nicht in den Schlüsselbund verschoben werden. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Deshalb wird GitHub ohne Schlüssel gefragt, ob nun einer gespeichert ist oder nicht.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Ein GitHub-Schlüssel ist gespeichert, kann aber nicht gesendet werden: Er hat ein Leerzeichen oder einen Zeilenumbruch darin, wie ein Schlüssel, der in zwei Teilen eingefügt wurde. Deshalb wird GitHub ohne Schlüssel gefragt. Entferne ihn und füge den Schlüssel noch einmal ein.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Die Datei konnte nicht entfernt werden.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Der Schlüsselbund hat ihn nicht angenommen.

# Red words when a pasted key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-key-spaces = Ein Schlüssel ist eine Zeile ohne Leerzeichen, und dieser hat einen Zeilenumbruch oder ein Leerzeichen darin. Füge nur den Schlüssel ein.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub antwortet vorerst nicht mehr: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub antwortet vorerst nicht mehr: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Versuche es später noch einmal.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Es antwortet in etwa einer Minute wieder.
       *[other] Es antwortet in etwa { $n } Minuten wieder.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = es beantwortet 30 Suchen pro Minute

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = es beantwortet 10 Suchen pro Minute

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = es beantwortet 5.000 Fragen pro Stunde mit deinem Schlüssel, und sie sind aufgebraucht

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = es beantwortet 60 Fragen pro Stunde von einer Adresse ohne Schlüssel

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Ein GitHub-Schlüssel erhöht das auf 5.000: Drücke oben rechts bei „GitHub durchsuchen“ auf den Schlüssel.

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub hat den Schlüssel nicht akzeptiert. Erstelle einen neuen oder entferne ihn: Drücke oben rechts bei „GitHub durchsuchen“ auf den Schlüssel.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub hat mit { $code } geantwortet.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub hat das abgelehnt, obwohl die Antworten nicht aufgebraucht sind.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub hat das abgelehnt: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] 1 gefundenes Projekt konnte nicht angesehen werden und ist deshalb nicht aufgelistet
       *[other] { $n } gefundene Projekte konnten nicht angesehen werden und sind deshalb nicht aufgelistet
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub war nicht erreichbar.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Es wurde nur nach Projekten mit einem AppImage gesucht: GitHub hat die Suche nach Flatpaks nicht beantwortet. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Wird heruntergeladen

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Wird installiert

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Der Download wurde nicht fertig. { $why }

# The size of a download that is under a megabyte.
store-size-under = unter 1 MB

