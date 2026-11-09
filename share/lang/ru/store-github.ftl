# machine translated (Russian), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
# SAFETY: needs a human review before it ships
store-gh-unchecked = Скачано безопасно, но этот файл никто не проверял: его автор не публикует подпись.

# What a project with nothing to install here is told as.
store-gh-no-files = В выпусках этого проекта нет файла AppImage или Flatpak для этого компьютера.

# What a project that has never made a release is told as.
store-gh-no-releases = У этого проекта на GitHub нет выпусков.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Поиск ключа в связке ключей

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Ключ сохранён

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Ключ не сохранён

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = На этом компьютере нет связки ключей, поэтому ключ сохранить нельзя

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = Ваш ключ GitHub всё ещё лежит в обычном файле на этом компьютере и используется оттуда: перенести его в связку ключей не удалось. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Поэтому GitHub опрашивается без ключа, сохранён он или нет.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Ключ GitHub сохранён, но отправить его нельзя: внутри него есть пробел или перевод строки, как бывает с ключом, вставленным в двух частях. Поэтому GitHub опрашивается без ключа. Удалите его и вставьте ключ заново.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Не удалось удалить файл.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Связка ключей его не приняла.

# Red words when a pasted key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-key-spaces = Ключ состоит из одной строки без пробелов, а здесь внутри есть перевод строки или пробел. Вставьте только сам ключ.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub пока перестал отвечать: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub пока перестал отвечать: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Попробуйте позже.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Ответит снова примерно через { $n } минуту.
        [few] Ответит снова примерно через { $n } минуты.
        [many] Ответит снова примерно через { $n } минут.
       *[other] Ответит снова примерно через { $n } минуты.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = он отвечает на 30 поисков в минуту

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = он отвечает на 10 поисков в минуту

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = он отвечает на 5000 вопросов в час с вашим ключом, и они исчерпаны

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = он отвечает на 60 вопросов в час с одного адреса без ключа

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Ключ GitHub поднимает это до 5000: нажмите на ключ в правом верхнем углу раздела «Поиск на GitHub».

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub не принял ключ. Создайте новый или удалите этот: нажмите на ключ в правом верхнем углу раздела «Поиск на GitHub».

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub ответил: { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub отказал, хотя ответы у него не закончились.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub отказал: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Не удалось просмотреть найденных проектов: { $n }, поэтому их нет в списке
        [few] Не удалось просмотреть найденных проектов: { $n }, поэтому их нет в списке
        [many] Не удалось просмотреть найденных проектов: { $n }, поэтому их нет в списке
       *[other] Не удалось просмотреть найденных проектов: { $n }, поэтому их нет в списке
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = Не удалось связаться с GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Искались только проекты с AppImage: GitHub не ответил на поиск Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Загрузка

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Установка

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Загрузка не завершилась. { $why }

# The size of a download that is under a megabyte.
store-size-under = меньше 1 МБ

