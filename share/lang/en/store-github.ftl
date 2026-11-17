# Apps: searching and installing from GitHub (src/store/github.rs).

## GitHub (src/store/github.rs)

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = This file was downloaded securely, but nobody has checked it, because its maker doesn't publish a signature.
# What a project with nothing to install here is told as.
store-gh-no-files = This project's releases have no AppImage or Flatpak file for this computer.
# What a project that has never made a release is told as.
store-gh-no-releases = GitHub has no releases for this project.
# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Looking in the keyring for a key
# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = A key is stored
# In the GitHub key's box: no key is stored.
store-gh-keyis-none = No key is stored
# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = This computer has no keyring, so it can't store a key
# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = Your GitHub key is still in an old plain file on this computer, which Trindle no longer reads. { $why }
# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } So Trindle uses GitHub without a key, even if one is saved.
# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = Trindle can't send your saved GitHub key, because it has a space or line break in it. So Trindle uses GitHub without a key. Remove the key and paste it again.
# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = The file couldn't be removed.
# Reason the keyring did not take the key.
store-gh-keyis-not-taken = The keyring didn't save it, so Trindle uses GitHub without a key.
# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = A key is one line with no spaces. This one has a line break or a space inside it. Paste the key on its own.
# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub has stopped answering for now, because { $which }. { $when }
# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub has stopped answering for now, because { $which }. { $when } { $hint }
# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Try again later.
# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] It will answer again in about a minute.
       *[other] It will answer again in about { $n } minutes.
    }
# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = it allows 30 searches a minute
# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = it allows 10 searches a minute
# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = you have used up the 5,000 requests an hour that your key allows
# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = it allows 60 requests an hour from one address without a key
# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = A GitHub key raises that to 5,000. Press the key button at the top right of Search.
# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub didn't accept the key. Make a new one, or remove this one, using the key button at the top right of Search.
# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub returned { $code }.
# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub blocked this, even though you haven't reached its limit.
# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub blocked this: { $why }.
# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Trindle couldn't look at 1 project it found, so it isn't listed
       *[other] Trindle couldn't look at { $n } projects it found, so they aren't listed
    }
# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.
# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.
# Red words when GitHub could not be reached.
store-gh-unreached = GitHub couldn't be reached.
# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Trindle only searched for projects with an AppImage, because GitHub didn't answer the search for Flatpaks. { $why }
# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Downloading
# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Installing
# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = The download didn't finish. { $why }
# The size of a download that is under a megabyte.
store-size-under = under 1 MB
