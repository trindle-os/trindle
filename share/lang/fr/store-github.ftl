
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Téléchargé de façon sécurisée, mais personne n'a vérifié ce fichier : son créateur ne publie pas de signature.

# What a project with nothing to install here is told as.
store-gh-no-files = Les versions de ce projet n'ont pas de fichier AppImage ou Flatpak pour cet ordinateur.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub n'a aucune version pour ce projet.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Recherche d'une clé dans le trousseau

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Une clé est enregistrée

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Aucune clé n'est enregistrée

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Il n'y a pas de trousseau sur cet ordinateur, donc aucune clé ne peut être gardée

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = Votre clé GitHub est encore dans un fichier simple sur cet ordinateur, et elle est utilisée à partir de là : elle n'a pas pu être déplacée dans le trousseau. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } GitHub est donc interrogé sans clé, qu'il y en ait une d'enregistrée ou non.

# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = Une clé GitHub est enregistrée, mais elle ne peut pas être envoyée : elle contient une espace ou un saut de ligne, comme une clé collée en deux morceaux. GitHub est donc interrogé sans clé. Supprimez-la et collez de nouveau la clé.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Le fichier n'a pas pu être supprimé.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Le trousseau ne l'a pas prise.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Une clé tient sur une seule ligne, sans espaces, et celle-ci contient un saut de ligne ou une espace. Collez la clé seule.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub a cessé de répondre pour le moment : { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub a cessé de répondre pour le moment : { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Réessayez plus tard.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Il répondra de nouveau dans environ une minute.
       *[other] Il répondra de nouveau dans environ { $n } minutes.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = il répond à 30 recherches par minute

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = il répond à 10 recherches par minute

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = il répond à 5 000 questions par heure avec votre clé, et elles sont épuisées

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = il répond à 60 questions par heure depuis une même adresse sans clé

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Une clé GitHub porte cette limite à 5 000 : appuyez sur la clé en haut à droite de Rechercher sur GitHub.

# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub n'a pas accepté la clé. Faites-en une nouvelle, ou supprimez-la : appuyez sur la clé en haut à droite de Rechercher sur GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub a répondu { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub a refusé, bien qu'il n'ait pas épuisé ses réponses.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub a refusé : { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] 1 projet trouvé n'a pas pu être examiné, il n'est donc pas listé
       *[other] { $n } projets trouvés n'ont pas pu être examinés, ils ne sont donc pas listés
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which } : { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = GitHub n'a pas pu être joint.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Seuls les projets avec une AppImage ont été recherchés : GitHub n'a pas répondu à la recherche de Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Téléchargement

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Installation

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Le téléchargement ne s'est pas terminé. { $why }

# The size of a download that is under a megabyte.
store-size-under = moins de 1 Mo

