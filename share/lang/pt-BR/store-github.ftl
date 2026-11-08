
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Baixado com segurança, mas ninguém verificou este arquivo: quem o fez não publica assinatura.

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
store-gh-keyis-nokeyring = Este computador não tem chaveiro, então não é possível guardar uma chave

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = Sua chave do GitHub ainda está num arquivo simples neste computador e é usada de lá: não foi possível passá-la para o chaveiro. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Por isso o GitHub é consultado sem chave, haja ou não uma guardada.

# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = Há uma chave do GitHub guardada, mas ela não pode ser enviada: tem um espaço ou uma quebra de linha no meio, como acontece com uma chave colada em duas partes. Por isso o GitHub é consultado sem chave. Remova-a e cole a chave de novo.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Não foi possível remover o arquivo.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = O chaveiro não aceitou a chave.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Uma chave é uma só linha, sem espaços, e esta tem uma quebra de linha ou um espaço no meio. Cole só a chave.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = O GitHub parou de responder por enquanto: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = O GitHub parou de responder por enquanto: { $which }. { $when } { $hint }

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
store-gh-limit-search-key = ele responde a 30 buscas por minuto

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = ele responde a 10 buscas por minuto

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = ele responde a 5.000 consultas por hora com a sua chave, e elas acabaram

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = ele responde a 60 consultas por hora de um mesmo endereço sem chave

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Uma chave do GitHub sobe esse limite para 5.000: toque na chave no canto superior direito de Buscar no GitHub.

# Red words when GitHub does not accept the key.
store-gh-key-refused = O GitHub não aceitou a chave. Crie uma nova ou remova esta: toque na chave no canto superior direito de Buscar no GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = O GitHub respondeu { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = O GitHub recusou, embora ainda não tenha esgotado as respostas.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = O GitHub recusou: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] 1 projeto encontrado não pôde ser examinado, por isso não está na lista
        [many] { $n } de projetos encontrados não puderam ser examinados, por isso não estão na lista
       *[other] { $n } projetos encontrados não puderam ser examinados, por isso não estão na lista
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = Não foi possível acessar o GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Só foram buscados projetos com AppImage: o GitHub não respondeu à busca por Flatpaks. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Baixando

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Instalando

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = O download não terminou. { $why }

# The size of a download that is under a megabyte.
store-size-under = menos de 1 MB
