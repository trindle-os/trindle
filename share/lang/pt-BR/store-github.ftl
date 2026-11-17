
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Este arquivo foi baixado com segurança, mas ninguém o verificou, porque quem o fez não publica assinatura.

# What a project with nothing to install here is told as.
store-gh-no-files = As versões deste projeto não têm arquivo AppImage ou Flatpak para este computador.

# What a project that has never made a release is told as.
store-gh-no-releases = O GitHub não tem versões publicadas deste projeto.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Procurando uma chave no chaveiro

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Há uma chave guardada

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Não há chave guardada

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Este computador não tem chaveiro, então não pode guardar uma chave

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = Sua chave do GitHub ainda está num arquivo simples antigo neste computador, que o Trindle não lê mais. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Por isso o Trindle usa o GitHub sem chave, mesmo que haja uma guardada.

# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = O Trindle não consegue enviar a chave do GitHub guardada, porque ela tem um espaço ou uma quebra de linha. Por isso o Trindle usa o GitHub sem chave. Remova a chave e cole de novo.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Não foi possível remover o arquivo.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = O chaveiro não guardou a chave, então o Trindle usa o GitHub sem chave.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Uma chave é uma só linha, sem espaços. Esta tem uma quebra de linha ou um espaço no meio. Cole só a chave.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = O GitHub parou de responder por enquanto, porque { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = O GitHub parou de responder por enquanto, porque { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Tente de novo mais tarde.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Ele volta a responder em cerca de um minuto.
        [many] Ele volta a responder em cerca de { $n } de minutos.
       *[other] Ele volta a responder em cerca de { $n } minutos.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = ele permite 30 buscas por minuto

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = ele permite 10 buscas por minuto

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = você usou as 5.000 solicitações por hora que a sua chave permite

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = ele permite 60 solicitações por hora de um mesmo endereço sem chave

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Uma chave do GitHub aumenta esse limite para 5.000. Clique no botão da chave no canto superior direito de Buscar.

# Red words when GitHub does not accept the key.
store-gh-key-refused = O GitHub não aceitou a chave. Crie uma nova ou remova esta, usando o botão da chave no canto superior direito de Buscar.

# Red words when GitHub answered with a code and no words.
store-gh-answered = O GitHub retornou { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = O GitHub bloqueou isto, mesmo que você não tenha atingido o limite.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = O GitHub bloqueou isto: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] O Trindle não conseguiu examinar 1 projeto que encontrou, por isso ele não está na lista
        [many] O Trindle não conseguiu examinar { $n } de projetos que encontrou, por isso eles não estão na lista
       *[other] O Trindle não conseguiu examinar { $n } projetos que encontrou, por isso eles não estão na lista
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = Não foi possível acessar o GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = O Trindle buscou só projetos com AppImage, porque o GitHub não respondeu à busca por Flatpaks. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Baixando

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Instalando

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = O download não terminou. { $why }

# The size of a download that is under a megabyte.
store-size-under = menos de 1 MB
