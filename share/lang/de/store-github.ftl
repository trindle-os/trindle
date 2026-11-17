
# machine translated (German), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = Diese Datei wurde sicher heruntergeladen, aber niemand hat sie geprüft, weil ihr Hersteller keine Signatur veröffentlicht.

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
store-gh-keyis-nokeyring = Dieser Computer hat keinen Schlüsselbund und kann keinen Schlüssel speichern

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = Dein GitHub-Schlüssel liegt noch in einer alten einfachen Datei auf diesem Computer, die Trindle nicht mehr liest. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Deshalb nutzt Trindle GitHub ohne Schlüssel, auch wenn einer gespeichert ist.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Trindle kann deinen gespeicherten GitHub-Schlüssel nicht senden, weil er ein Leerzeichen oder einen Zeilenumbruch enthält. Deshalb nutzt Trindle GitHub ohne Schlüssel. Entferne den Schlüssel und füge ihn noch einmal ein.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Die Datei konnte nicht entfernt werden.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Der Schlüsselbund hat ihn nicht gespeichert. Deshalb benutzt Trindle GitHub ohne Schlüssel.

# Red words when a pasted key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-key-spaces = Ein Schlüssel ist eine Zeile ohne Leerzeichen, und dieser hat einen Zeilenumbruch oder ein Leerzeichen darin. Füge nur den Schlüssel ein.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub antwortet vorerst nicht mehr, weil { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub antwortet vorerst nicht mehr, weil { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Versuche es später noch einmal.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Es antwortet in etwa einer Minute wieder.
       *[other] Es antwortet in etwa { $n } Minuten wieder.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = es 30 Suchen pro Minute erlaubt

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = es 10 Suchen pro Minute erlaubt

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = du die 5.000 Anfragen pro Stunde aufgebraucht hast, die dein Schlüssel erlaubt

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = es von einer Adresse ohne Schlüssel 60 Anfragen pro Stunde erlaubt

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Mit einem GitHub-Schlüssel steigt das auf 5.000. Drücke oben rechts bei „GitHub durchsuchen“ auf die Schlüssel-Taste.

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub hat den Schlüssel nicht akzeptiert. Erstelle einen neuen oder entferne diesen mit der Schlüssel-Taste oben rechts bei „GitHub durchsuchen“.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub hat { $code } zurückgegeben.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub hat das blockiert, obwohl du sein Limit nicht erreicht hast.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub hat das blockiert: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Trindle konnte 1 gefundenes Projekt nicht ansehen, daher ist es nicht aufgelistet
       *[other] Trindle konnte { $n } gefundene Projekte nicht ansehen, daher sind sie nicht aufgelistet
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub war nicht erreichbar.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Trindle hat nur nach Projekten mit einem AppImage gesucht, weil GitHub die Suche nach Flatpaks nicht beantwortet hat. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Wird heruntergeladen

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Wird installiert

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Der Download wurde nicht fertig. { $why }

# The size of a download that is under a megabyte.
store-size-under = unter 1 MB
