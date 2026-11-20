# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Relatar um problema

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Algo deu errado. Enviar um relatório?

# Under the title: what a report holds, and what it never holds.
report-summary = Isto envia suas palavras, dados do seu computador, os erros desta inicialização e o registro do Trindle. Nomes e endereços (e-mail, rede, Wi-Fi) são removidos do que o Trindle reúne. Nomes de arquivos e endereços da web ainda podem aparecer nas linhas do registro. Não envia seus arquivos nem suas senhas.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = O que aconteceu?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Qual aplicativo?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Não tenho certeza

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Outro programa

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = A área de trabalho

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
report-not-taken = Ainda não é possível enviar relatórios.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Verificando.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Relatórios demais ao mesmo tempo.

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
report-no-at = Digite seu nome de usuário do Buildmorphic, não um endereço de e-mail. Não use @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = O nome de usuário deve ter de 2 a 31 caracteres, usando letras minúsculas, números ou hifens.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Um nome do GitHub só pode ter letras, números e hifens.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = O Trindle não conseguiu guardar o relatório para depois: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Não foi possível salvar: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = sem permissão

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = a unidade está cheia

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = a pasta não existe

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = um erro na unidade

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = o curl não está instalado.

# Why a report could not be posted: the post could not be started.
report-not-posted = Não foi possível enviar.

# Why a report could not be posted: the computer is offline.
report-no-connection = Sem conexão.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = O servidor respondeu { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Escreva mais algumas palavras sobre o que aconteceu.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Lendo o registro.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Não há registro para enviar com o relatório, então ele não pode ser enviado. Salve-o em vez disso.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Imagem opcional. Só o dono do projeto a vê.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Escolher uma imagem PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Fazer uma captura de tela

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Remover a imagem

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Sua imagem, { $width } por { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Escolher uma imagem PNG

report-picture-filter = Imagens PNG

# Under the picture while it is being read.
report-picture-reading = Lendo.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Não foi possível fazer a captura de tela.

report-picture-no-chooser = O Trindle não conseguiu abrir o seletor de arquivos.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Não é uma imagem PNG.

report-picture-too-big = Tem mais de { $megabytes } MB.

report-picture-too-large = A imagem é grande demais. Ela pode ter até { $side } pixels de largura e { $side } pixels de altura.

report-picture-damaged = A imagem está danificada.

report-picture-unreadable = Não foi possível ler.

report-sent-as-bug = Enviado. O número do seu relatório de erro é { $number }. Obrigado.

# Under the title, beta only: why a report matters and what it holds.
report-beta = O Trindle está em beta. Os relatórios são anônimos: nomes, endereços de e-mail e pastas pessoais são removidos. O nome de usuário é opcional.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Nome de usuário do Buildmorphic (opcional)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = O Trindle não conseguiu enviar o relatório. { $why } Pressione Enviar para tentar de novo.
