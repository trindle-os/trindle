
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Tela

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = tela monitor resolução tamanho escala zoom maior menor girar rotação atualização luz noturna quente luz azul noite olhos

# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = As mudanças valem na hora. Se a tela ficar estranha, não faça nada: ela volta sozinha após 15 segundos, a menos que você pressione Manter. A luz noturna deixa a tela mais quente à noite.

# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = Manter esta disposição?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Verificando as telas…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Nenhuma tela foi encontrada.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Nenhuma tela em uso

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Uma tela em uso

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] { $n } tela em uso
        [many] { $n } telas em uso
       *[other] { $n } telas em uso
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = a luz noturna está desligada

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = a luz noturna esquenta a tela das { $from } às { $to }

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = a luz noturna esquenta a tela do pôr do sol ao nascer do sol

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normal

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Girada para a direita

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Girada para a esquerda

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = De cabeça para baixo

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = À direita de

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = À esquerda de

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Acima de

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Abaixo de

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } a { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (recomendada)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (padrão)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Resolução de { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Tamanho em { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Rotação de { $display }

# Heading of the resolution line.
settings-display-resolution = Resolução

# Small words under that line.
settings-display-resolution-about = Quantos pontos mostra e com que frequência.

# Heading of the size line.
settings-display-size = Tamanho

# Small words under that line.
settings-display-size-about = Quão grande tudo parece nesta tela.

# Heading of the rotation line.
settings-display-rotation = Rotação

# Small words under that line.
settings-display-rotation-about = Para uma tela posta de lado.

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Usar esta tela

# Small words under that line for the only display in use.
settings-display-use-only = A única tela em uso não pode ser desligada.

# Small words under that line otherwise.
settings-display-use-off = Desligada, nada é mostrado nela.

# Words behind the i of a display's card.
settings-display-card-about = As mudanças valem na hora. Se a tela ficar estranha, não faça nada: ela volta sozinha após 15 segundos, a menos que você pressione Manter. A resolução recomendada é a própria da tela; o tamanho padrão é o que o Trindle escolheu para esta tela.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Posição de { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Posicione-a

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } em relação a { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Disposição

# Words behind the i of that card.
settings-display-arrangement-about = Onde as telas ficam umas ao lado das outras, para o ponteiro passar de uma para a próxima pela borda que elas dividem.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Use duas telas para colocar uma ao lado da outra.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = A área de trabalho recusou.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] As telas mudaram. Elas voltam em { $seconds } segundo, a menos que você pressione Manter.
        [many] As telas mudaram. Elas voltam em { $seconds } segundos, a menos que você pressione Manter.
       *[other] As telas mudaram. Elas voltam em { $seconds } segundos, a menos que você pressione Manter.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Não foi possível desfazer a mudança. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = A disposição está em uso agora, mas não pôde ser guardada para o próximo login.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Nada foi mudado

# Small words under that line.
settings-display-refused-change = A área de trabalho recusou a mudança.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = As mudanças valem na hora

# Small words under that line; { $seconds } is how long Keep is asked for.
settings-display-keep-asked =
    { $seconds ->
        [one] A opção Manter aparece no pé da janela por { $seconds } segundo.
        [many] A opção Manter aparece no pé da janela por { $seconds } segundos.
       *[other] A opção Manter aparece no pé da janela por { $seconds } segundos.
    }

# Title of the night light card.
settings-display-night-title = Luz noturna

# Words behind the i of that card.
settings-display-night-about = A tela fica mais quente à noite, o que cansa menos os olhos. Segue o pôr e o nascer do sol, calculados pelo fuso horário, ou horários definidos por você; quando o fuso horário não diz onde fica este computador, os horários definidos são usados.

# Heading of the night light line, and name of its switch.
settings-display-night = Luz noturna

# Small words under that line.
settings-display-night-about-line = Cores mais quentes à noite.

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Pôr e nascer do sol em { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = O fuso horário não indica um lugar: horários definidos.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Do pôr ao nascer do sol

# Choice in the When list: between times the person sets.
settings-display-when-times = Horários definidos

# Heading of the When line, and name of its list.
settings-display-when = Quando

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = De

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = Até

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Calor

# Small words under the warmth line.
settings-display-warmth-about = Quão quente a tela fica à noite.

# Small word under the left end of the warmth slider.
settings-display-little = Um pouco

# Small word under the right end of the warmth slider.
settings-display-very = Bem quente

# Spoken description of the strip of colours night light runs through.
settings-display-strip = De um pouco quente a bem quente

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Nenhuma tela em uso

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } tela, lado a lado como estão dispostas
        [many] { $n } telas, lado a lado como estão dispostas
       *[other] { $n } telas, lado a lado como estão dispostas
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Nenhuma tela

# Name of the large picture of the displays.
settings-display-your-displays = Suas telas

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = A área de trabalho não listou nenhuma tela.

# Title of the card shown when no display was found.
settings-display-none-title = Nenhuma tela encontrada

# Heading of the line in that card.
settings-display-none-heading = Nada onde mostrar

# Title of the card about the graphics card.
settings-display-graphics-title = Gráficos

# Words behind the i of that card.
settings-display-graphics-about = Este computador tem uma placa NVIDIA, usando o driver aberto. Tudo funciona. Em uma placa de 2022 em diante, ele vai parecer lento, porque o driver aberto não consegue aumentar a velocidade da placa. O driver próprio da NVIDIA pode ser instalado.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Uma placa NVIDIA, com o driver aberto

# Small words under that line.
settings-display-nvidia-about = Funciona; uma placa de 2022 em diante parece lenta.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = O que fazer a respeito

