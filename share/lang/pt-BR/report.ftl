# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Relatar um problema

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Algo deu errado. Enviar um relatório?

# Under the title: what a report holds, and what it never holds.
report-summary = Isto envia o erro, um resumo do computador e as últimas linhas do registro, com nomes, endereços de e-mail, endereços de rede e nomes de Wi-Fi removidos. Nomes de arquivos e endereços da web ainda podem aparecer nas linhas do registro. Não envia seus arquivos nem suas senhas.

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = O que você estava fazendo? (opcional)

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Enviar

# Button at the bottom of the window when the person opened it themselves.
# SAFETY: needs a human review before it ships
report-cancel = Cancelar

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
# SAFETY: needs a human review before it ships
report-dont-send = Não enviar

# Button that opens the exact text of the report.
report-show-details = Mostrar detalhes

# Button that closes the exact text of the report.
report-hide-details = Ocultar detalhes

# Button under the shown report text that copies it.
report-copy = Copiar

# Button under the shown report text that saves it as a file in Downloads.
report-save = Salvar

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Os relatórios ainda não estão sendo aceitos.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Verificando.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Relatórios demais ao mesmo tempo.

# Written beside the Send button once the report has been sent.
report-already-sent = Já enviado.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Já salvo.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Aguardando uma conexão.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Aguardando uma conexão. { $why }

# Under the typing box after the report was sent.
report-sent = Enviado. Obrigado.

# Under the typing box while the report is being posted.
report-sending = Enviando.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Salvo como Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Use seu nome de usuário do Buildmorphic, não um endereço de e-mail, então sem @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Um nome de usuário tem de 2 a 31 letras minúsculas, números ou hifens.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Um nome do GitHub só pode ter letras, números e hifens.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Não foi possível guardar para depois: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Não foi possível salvar: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = sem permissão

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = a unidade está cheia

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = a pasta não existe

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = a unidade recusou

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = o curl não está instalado.

# Why a report could not be posted: the post could not be started.
report-not-posted = Não foi possível enviar.

# Why a report could not be posted: the computer is offline.
report-no-connection = Sem conexão.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = O endereço respondeu { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Escreva mais algumas palavras para dizer o que aconteceu.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Lendo o registro.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Não há registro para enviar com ele, então não pode ser enviado. Salve-o em vez disso.



report-sent-as-bug = Enviado. Registrado como o erro { $number }. Obrigado.


# Under the title, beta only: why a report matters and what it holds.
report-beta = O Trindle está em beta, e cada relatório nos ajuda a fazê-lo funcionar com perfeição. Um relatório é anônimo e não contém nada privado nem pessoal. Não é preciso e-mail e o nome de usuário é opcional. Nada é enviado até você pressionar Enviar.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Nome de usuário do Buildmorphic (opcional)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Não enviado. { $why } Pressione Enviar para tentar de novo.

