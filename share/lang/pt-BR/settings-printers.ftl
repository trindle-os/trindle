
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Ainda não há impressoras. As que o Trindle encontrar na sua rede aparecem abaixo.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } está em pausa, por isso nada será impresso nela por enquanto.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } está imprimindo.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Pronto para imprimir em { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } está { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } está configurada, mas você não escolheu uma impressora padrão.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } impressoras estão configuradas, mas você não escolheu uma impressora padrão.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = Esse trabalho não existe mais.

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Cancelado

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = A verificação falhou.

# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = O serviço de impressão não está em execução ou não pode ser acessado.

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
settings-printers-adding = Adicionando…

# Shown beside a printer or the address line after it was added.
settings-printers-added = Adicionada

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Procurando o driver…

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
settings-printers-jobs-about = Todos os trabalhos esperando para imprimir, em qualquer impressora. O Trindle atualiza a lista a cada poucos segundos enquanto esta página está aberta. Páginas que já foram impressas não podem ser canceladas.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = O CUPS não está respondendo

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Não foi possível ler a fila.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Verificando…

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Nada esperando

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Em pausa

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Ociosa

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Imprimindo

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Padrão, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Tornar esta a padrão

# Tip of the button that prints a test page.
settings-printers-test-page = Imprimir uma página de teste

# SAFETY: needs a human review before it ships
# Tip of the bin button that removes a printer.
settings-printers-remove = Remover esta impressora

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = A busca precisa do cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Para instalar, execute: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Adicionar esta impressora

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Nada novo encontrado

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Adicionar

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Adicionar pelo endereço

# Heading of the card for adding printers.
settings-printers-add-card = Adicionar uma impressora

# Help text (i button) of the Add a printer card.
settings-printers-add-about = O Trindle procura primeiro as impressoras que se anunciam e depois verifica cada dispositivo da rede em sequência. A impressora precisa estar ligada e na mesma rede. Algumas impressoras não se anunciam, e algumas redes não repassam o anúncio. Nesse caso, adicione a impressora pelo endereço, como 192.168.1.50. Você encontra o endereço na tela da própria impressora, nas configurações de rede.

# Name of the page in the list of settings pages.
settings-printers-title = Impressoras

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = impressora imprimir impressão scanner

# Help text at the top of the page.
settings-printers-help = O Trindle encontra as impressoras da sua rede sozinho. Para usar uma, pressione Adicionar esta impressora ao lado dela.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = A impressão não está instalada neste computador.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Verificando…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = O Trindle não consegue acessar o serviço de impressão, por isso nada pode ser impresso agora.

# Heading of the card shown when printing is not installed.
settings-printers-printing = Impressão

# Help text (i button) of that card.
settings-printers-printing-about = A impressão precisa do CUPS, que não está neste computador. Depois de instalado, o Trindle encontra as impressoras da sua rede sozinho.

# Line in that card.
settings-printers-unavailable = A impressão não está disponível

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Instale com: sudo apt install cups

# Line in that card.
settings-printers-then-found = Encontrando impressoras

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Serviço de impressão

# Line in the Printers card when there are none.
settings-printers-none = Ainda não há impressoras

# Heading of the card that lists the printers.
settings-printers-card = Impressoras

# Help text (i button) of the Printers card.
settings-printers-card-about = Todas as impressoras configuradas neste computador. Sua conta precisa estar no grupo lpadmin para fazer mudanças.

# Red words under a page range box that cannot be used.
printers-bad-range = Por exemplo: 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = De 1 a 99 cópias.

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
printers-not-allowed = Você não tem permissão para fazer isso. Sua conta precisa estar no grupo lpadmin. Depois disso, saia e entre de novo.

# Red words when CUPS refuses and gives no reason.
printers-refused = O CUPS recusou.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = Esse nome de impressora não é permitido.

# Red words when printing is not installed on the computer.
printers-not-installed = A impressão não está instalada neste computador.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } não está mais configurada.

# Red words when the printer chosen is paused.
printers-paused = { $name } está em pausa.

# Red words when sending the document to CUPS took too long.
printers-too-slow = O documento demorou demais para ser enviado.

# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = O Trindle não conseguiu cancelar o trabalho.

# Red words when a printer could not be made the default.
settings-printers-not-default = O Trindle não conseguiu tornar esta a impressora padrão.

# Red words when the test page could not be sent.
settings-printers-not-sent = O Trindle não conseguiu enviar a página de teste.

# Red words when a printer could not be removed.
settings-printers-not-removed = O Trindle não conseguiu remover a impressora.

# Red words when a printer could not be added.
settings-printers-not-added = O Trindle não conseguiu adicionar a impressora.

# Red words when the list of printers could not be read.
settings-printers-not-read = O Trindle não conseguiu ler a lista de impressoras.
