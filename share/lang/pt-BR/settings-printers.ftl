
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Ainda não há impressoras. As que forem encontradas na sua rede aparecem abaixo.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } está em pausa, por isso nada será impresso nela por enquanto.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } está imprimindo.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Pronto para imprimir em { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } está { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } está configurada; nenhuma foi escolhida para imprimir primeiro.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } impressoras estão configuradas; nenhuma foi escolhida para imprimir primeiro.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Esse não é um trabalho que o CUPS conhece.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Cancelado

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = o lpstat falhou

# Words in the ring while the network is searched for printers.
settings-printers-searching = Procurando

# Words in the ring when the search is finished.
settings-printers-search-done = Pronto

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Agora é a padrão

# Shown beside a printer after a test page was sent.
settings-printers-sent = Enviada

# SAFETY: needs a human review before it ships
# Shown beside a printer after it was removed.
settings-printers-removed = Removida

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Digite primeiro o endereço da impressora.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Adicionando...

# Shown beside a printer or the address line after it was added.
settings-printers-added = Adicionada

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Procurando o driver...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Trabalho { $number } em { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# SAFETY: needs a human review before it ships
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = Cancelar este trabalho

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Trabalhos de impressão

# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Todos os trabalhos esperando para imprimir, em qualquer impressora, como estão agora. A lista é lida de novo a cada poucos segundos enquanto esta página está aberta. A cruz cancela um trabalho; o papel já impresso continua impresso.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = O CUPS não está respondendo

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Não foi possível ler a fila.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Verificando…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = A fila.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Nada esperando

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = Os trabalhos aparecem aqui enquanto imprimem.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Em pausa

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Ociosa

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Imprimindo

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Padrão, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Imprimir primeiro nesta

# Tip of the button that prints a test page.
settings-printers-test-page = Imprimir uma página de teste

# SAFETY: needs a human review before it ships
# Tip of the bin button that removes a printer.
settings-printers-remove = Remover esta impressora

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = A busca precisa do cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Instale com: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Adicionar esta impressora

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Nada novo encontrado

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = Ela está ligada e na mesma rede?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = Primeiro as que se anunciam, depois cada máquina em sequência.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Adicionar

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Pelo endereço

# Small line under "By its address".
settings-printers-by-address-sub = Como 192.168.1.50, na tela da própria impressora.

# Heading of the card for adding printers.
settings-printers-add-card = Adicionar uma impressora

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Primeiro são procuradas as impressoras que se anunciam; depois cada máquina da rede é consultada em sequência. A impressora precisa estar ligada e na mesma rede. Algumas impressoras não se anunciam, e algumas redes não repassam o anúncio: adicione-a pelo endereço, como 192.168.1.50. Você o encontra na tela da própria impressora, nas configurações de rede.

# Name of the page in the list of settings pages.
settings-printers-title = Impressoras

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = impressora imprimir impressão scanner

# Help text at the top of the page.
settings-printers-help = As impressoras da sua rede são encontradas sozinhas. Pressione Adicionar ao lado de uma para usá-la.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = A impressão não está instalada neste computador.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Verificando…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = O serviço de impressão não está respondendo, por isso nada pode ser impresso agora.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Impressão

# Help text (i button) of that card.
settings-printers-printing-about = A impressão precisa do CUPS, que não está neste computador. Depois de instalado, as impressoras da sua rede são encontradas sozinhas.

# Line in that card.
settings-printers-unavailable = A impressão não está disponível

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Instale com: sudo apt install cups

# Line in that card.
settings-printers-then-found = Depois as impressoras são encontradas

# Small line under it.
settings-printers-then-found-sub = As da sua rede, sozinhas.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = O serviço de impressão.

# Line in the Printers card when there are none.
settings-printers-none = Ainda não há impressoras

# Small line under "No printers yet".
settings-printers-none-sub = Adicione uma ao lado.

# Heading of the card that lists the printers.
settings-printers-card = Impressoras

# Help text (i button) of the Printers card.
settings-printers-card-about = Todas as impressoras configuradas neste computador. A estrela faz de uma a impressora usada primeiro; o ícone de página imprime uma página de teste nela; a lixeira a remove, perguntando duas vezes. As mudanças exigem que a sua conta esteja no grupo lpadmin.

# Red words under a page range box that cannot be used.
printers-bad-range = Páginas como 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = Cópias: de 1 a 99

# Name of the choice to print on one side of the paper.
printers-sides-one = Um lado

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Dois, borda longa

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Dois, borda curta

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Carta

# Name of the US Legal paper size.
printers-paper-legal = Ofício

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Em pé

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Deitada

# SAFETY: needs a human review before it ships
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = Não permitido. Sua conta precisa estar no grupo lpadmin; saia e entre de novo quando estiver.

# Red words when CUPS refuses and gives no reason.
printers-refused = O CUPS recusou.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = O nome dessa impressora não é aceito pelo CUPS.

# Red words when printing is not installed on the computer.
printers-not-installed = A impressão não está instalada neste computador.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } não está mais configurada.

# Red words when the printer chosen is paused.
printers-paused = { $name } está em pausa.

# Red words when sending the document to CUPS took too long.
printers-too-slow = O documento demorou demais para ser enviado.

