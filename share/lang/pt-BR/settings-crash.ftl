
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Relatórios de falhas

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = relatórios falhas travamentos erros bugs defeitos enviar enviados privacidade telemetria buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = Quando um dos programas do Trindle trava, o módulo de relatórios de falhas guarda uma nota curta neste computador. Ele não envia nada até você executar o comando de envio e dizer sim a cada relatório, que mostra primeiro o relatório inteiro. Seu nome e seu nome no GitHub só entram se você os digitou aqui. Nunca uma mensagem, um nome de arquivo, onde você está nem nada sobre outros programas. Remover o módulo interrompe tudo.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Travamentos

# Under the Crashes switch.
settings-crash-crashes-short = Onde, no código do Trindle, ocorreu o travamento.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Travamentos envia o programa, sua versão, o tipo de travamento e onde ele ocorreu no código do Trindle.

# Heading and label of the switch for sending errors.
settings-crash-errors = Erros

# Under the Errors switch.
settings-crash-errors-short = O nome fixo do erro e onde ocorreu.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Erros envia o programa, sua versão, o nome fixo que o Trindle dá ao erro e onde ele ocorreu no código.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = Sobre este computador

# Under the About this computer switch.
settings-crash-computer-short = Linux, processador, memória, gráficos, telas.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = Sobre este computador acrescenta a cada relatório: o Linux em uso, o tipo de processador, a memória arredondada para baixo, Wayland ou X11, o fabricante dos gráficos e quantas telas há.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = O que é enviado

# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Os relatórios não levam nome, a menos que você digite seu nome de usuário do Buildmorphic na caixa abaixo, o que só lhe dá crédito.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Nada foi alterado, porque { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Nada foi alterado, porque o arquivo que guarda isso não está como o Trindle o escreveu.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Não há pasta de configurações.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Não foi possível criar a pasta de configurações.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Não foi possível gravar a configuração.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Travamento ({ $signal }) em { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Erro { $code } em { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Recebido

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Não enviado: rastreador fechado

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Não enviado: grande demais

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Não enviado: recusado

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Não enviado: muitos de uma vez

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Não enviado

# Said beside the link line once it is saved.
settings-crash-saved = Salvo

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Limpar a lista

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Não foi possível limpar a lista.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Removido

# Heading of the line that holds the clear button.
settings-crash-kept-here = Guardado somente neste computador

# Under the Kept on this computer only heading.
settings-crash-newest-first = Os mais recentes primeiro, até cinquenta.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Nada ainda

# Under the Nothing yet heading.
settings-crash-listed-here = Cada relatório aparece aqui, exatamente como foi enviado.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Mostrar exatamente o que foi enviado

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }, { $n } vezes: { $answer }
       *[other] { $when }, { $n } vezes: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Enviados até agora

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Cada relatório aparece exatamente como foi enviado, o mais recente primeiro, e fica guardado somente neste computador. Recebido significa que o Buildmorphic o tem. Não enviado: rastreador fechado significa que o rastreador de bugs do Trindle não está aceitando relatórios; recusado significa que foi recusado por estar mal formado; muitos de uma vez significa que chegaram relatórios demais ao mesmo tempo.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Após uma falha, você é consultado, com o relatório inteiro à vista, sobre enviá-lo ou não.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Nada é enviado, e você não é consultado após uma falha.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Nada é enviado: travamentos e erros estão ambos desativados.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Os erros dos programas da Trindle são relatados de forma anônima.
        [both] As falhas e os erros dos programas da Trindle são relatados de forma anônima.
       *[crashes] As falhas dos programas da Trindle são relatadas de forma anônima.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Perguntar sobre problemas e compartilhar relatórios

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Desligado oculta tudo e apaga os relatórios em espera.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Desligado. Nenhum relatório estava em espera.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [one] Desligado. { $n } relatório em espera apagado.
        [many] Desligado. { $n } de relatórios em espera apagados.
       *[other] Desligado. { $n } relatórios em espera apagados.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Sobre a comunidade

# Heading of the line with the community button.
settings-crash-go-to-bm = Os relatórios vão para o Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = A comunidade que ajuda a construir o Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = nome de usuário

# Heading of the line with the username box.
settings-crash-your-username = Seu nome de usuário no Buildmorphic

# Under the username heading.
settings-crash-username-sub = Opcional. Nunca um endereço de e-mail.

# Heading of a line about joining with an email address.
settings-crash-join = Participe só com um endereço de e-mail

# Under the Join heading.
settings-crash-join-sub = Ganhe créditos e níveis pelos problemas que você relatar primeiro.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Um relatório contém só uma lista fixa

# Under the fixed list heading.
settings-crash-fixed-sub = Você o vê antes, e nada sai sem o seu clique.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Relatórios que você envia

# Help text of the Reports you send card (the information button).
settings-crash-reports-about = Quando um programa do Trindle trava, pode abrir uma janela perguntando se você quer enviar um relatório. Ela mostra antes o texto exato: suas palavras, o modelo do computador e dos gráficos, o erro e o registro de logo antes. Arquivos, senhas e nomes de redes Wi-Fi nunca são lidos, e nomes e endereços são apagados. Você também pode abrir a mesma janela a qualquer momento pelo botão Relatar um problema do anel.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = nome no github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Seu nome no GitHub

# Small words under that heading.
settings-crash-github-about = Opcional. Aparece como Relatado por.

