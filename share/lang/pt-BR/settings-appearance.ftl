
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Settings page about appearance, in the list and as its heading.
settings-appearance-title = Aparência

# Words a person might type into the Settings search to find the appearance page; separate them with spaces.
settings-appearance-keywords = cor cores destaque tema temas visual aparência vidro escuro claro modo sol brilho ícones fonte letra cursor ponteiro leitor de tela cego ler em voz alta acessibilidade rótulos dicas nomes apontar

# Introduction under the heading of the appearance page, and the words behind the page i.
settings-appearance-help = Claro ou escuro, cores, ícones, fontes, o ponteiro e a leitura da tela em voz alta. Alguns programas só percebem a mudança depois de reabertos.

# Question along the bottom of the page while a theme is on trial.
settings-appearance-foot-keep = Manter este visual?

# Small words beside the question along the bottom while a theme is on trial.
settings-appearance-foot-escape = Esc volta agora.

# Name of the dark look, on a theme's button and in the top sentence.
settings-appearance-look-dark = Escuro

# Name of the light look, on a theme's button and in the top sentence.
settings-appearance-look-light = Claro

# The dark look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-dark-lower = escuro

# The light look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-light-lower = claro

# Spoken name of a theme's Dark or Light button; { $name } is the theme and { $look } the look.
settings-appearance-theme-spoken = { $name }, { $look }

# Spoken description of the small picture of a theme; { $name } is the theme and { $look } its look, such as dark.
settings-appearance-sample-spoken = Como fica o { $name }, { $look }

# Said aloud after Dark or Light was chosen; { $look } is dark or light.
settings-appearance-aloud-look = O visual { $look }.

# Sentence at the top of the page when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-top-unusable = O visual próprio do Trindle, porque o tema { $theme } não pode ser usado. { $why }

# Sentence at the top of the page for a theme that has dark and light looks; { $theme } is the theme, { $look } dark or light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme-look = O tema { $theme }, { $look }, com o destaque { $accent } e ícones { $shape }.

# Sentence at the top of the page for a theme with one look; { $theme } is the theme, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme = O tema { $theme }, com o destaque { $accent } e ícones { $shape }.

# Sentence at the top of the page when no theme is chosen; { $look } is Dark or Light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-look = { $look }, com o destaque { $accent } e ícones { $shape }.

# Shape of the icons in the top sentence.
settings-appearance-shape-round = redondos

# Name of Trindle's own highlight colour, first in the list of colours.
settings-appearance-trindle-yellow = Amarelo Trindle

# Start of the words behind the i of the reading the screen card.
settings-appearance-reading-says = Diz o que está na tela e o que você digita. Super, Alt e S juntas também a ligam e desligam.

# Red words under the icon shape line when the icons could not be remade.
settings-appearance-no-icons = Não foi possível refazer os ícones, então alguns mantêm a forma antiga. Escolha o conjunto de novo para tentar mais uma vez.

# Small words on a theme's tile when Trindle made some of its colours clearer.
settings-appearance-corrected = Algumas cores foram ajustadas para ficar mais legíveis.

# Said aloud when a theme is tried; { $theme } is its name and { $seconds } how long until the old look comes back.
settings-appearance-aloud-trying =
    { $seconds ->
        [one] Testando o tema { $theme }. O visual anterior volta em { $seconds } segundo, a menos que você pressione Manter. Esc volta agora.
        [many] Testando o tema { $theme }. O visual anterior volta em { $seconds } de segundos, a menos que você pressione Manter. Esc volta agora.
       *[other] Testando o tema { $theme }. O visual anterior volta em { $seconds } segundos, a menos que você pressione Manter. Esc volta agora.
    }

# Name of Trindle's own theme when none is named.
settings-appearance-trindle = Trindle

# Said aloud when a theme was kept; { $theme } is its name.
settings-appearance-aloud-kept = O tema { $theme } foi mantido.

# Red words under the Theme line when the old look could not be written back.
settings-appearance-back-failed = O visual anterior volta quando o tempo acabar.

# Said aloud when the look from before is back.
settings-appearance-aloud-back = O visual anterior voltou.

# Title of the window for choosing a theme file.
settings-appearance-chooser-title = Escolher um tema

# Name of the filter in that window that shows only themes.
settings-appearance-chooser-filter = Temas

# Green words under the Add a theme line after a theme was added; { $name } is its name.
settings-appearance-added = { $name } está na lista acima. Pressione-o para usá-lo.

# Red words under the Theme line when a theme could not be removed.
# SAFETY: needs a human review before it ships
settings-appearance-not-removed = Não foi possível remover o tema.

# Red words under the theme tiles when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-cannot-use = O tema { $theme } não pode ser usado. { $why }

# Heading of the line saying a theme is tried first.
settings-appearance-tried-first = Um tema é testado primeiro

# Small words under that line; { $seconds } is how long Keep is asked for.
settings-appearance-tried-first-about =
    { $seconds ->
        [one] Manter é perguntado no pé da janela por { $seconds } segundo.
        [many] Manter é perguntado no pé da janela por { $seconds } de segundos.
       *[other] Manter é perguntado no pé da janela por { $seconds } segundos.
    }

# Name of the bin button that removes one of the person's own themes.
# SAFETY: needs a human review before it ships
settings-appearance-remove-theme = Remover este tema

# Name of the button that chooses a theme; { $name } is the theme.
settings-appearance-use-theme = Usar { $name }

# Small words on a theme's tile naming its author; { $author } is the name.
settings-appearance-by = Por { $author }

# Name of the button that adds a theme, and heading of its line.
settings-appearance-add-theme = Adicionar um tema

# Spoken name of the turning ring shown while themes are being found.
settings-appearance-looking = Procurando temas

# Title of the Theme card.
settings-appearance-theme-card = Tema

# Words behind the i of the Theme card.
settings-appearance-theme-about = Um tema é um visual completo em um só arquivo: cores, cantos, tamanhos e espaço. Pressione um para testá-lo: todas as janelas do Trindle mudam de uma vez, e o visual anterior volta sozinho em 30 segundos, a menos que você pressione Manter, então um tema que não pode ser lido nunca fica. Um tema com visual escuro e claro tem Escuro e Claro ao lado da imagem; o Claro é mais fácil de ver sob sol forte, e os aplicativos de Aplicativos o adotam na próxima vez que abrirem. Um tema com um só visual não tem esses botões. Sua cor de destaque continua sendo sua em todos os temas. Um tema que você adiciona só é lido, nunca executado, e as palavras que ficariam difíceis de ler são clareadas. Em um terminal, trindle-settings --theme default sempre traz de volta o visual do próprio Trindle.

# Heading of the line holding the theme tiles.
settings-appearance-theme-line = Tema

# Small words under that line.
settings-appearance-theme-line-about = O visual completo. Escuro ou Claro fica ao lado de um tema que tem os dois.

# Small words under the Add a theme line.
settings-appearance-add-theme-about = Um arquivo .css, como um que alguém compartilhou.

# Words of the link that opens help about what a theme file holds.
settings-appearance-theme-file-help = O que um arquivo de tema contém

# Red words under the colour box when what was typed is not a colour.
settings-appearance-hex-bad = Seis caracteres de 0-9 e a-f, como #fbc711.

# Spoken description of the small dot that shows the colour typed.
settings-appearance-dot = A cor digitada

# Placeholder of the box for typing a colour code.
settings-appearance-hex-code = Código hex

# Spoken name of the box for typing a colour as a hex code; the example is a code.
settings-appearance-hex-spoken = Qualquer cor, como código hex, por exemplo #fbc711

# Heading of the line for typing any colour.
settings-appearance-any-colour = Qualquer cor

# Small words under that line.
settings-appearance-any-colour-about = Seis ou oito dígitos, como #fbc711.

# Title of the Colour card.
settings-appearance-colour-title = Cor

# Words behind the i of the Colour card.
settings-appearance-colour-about = O destaque marca botões, interruptores, o anel de preenchimento e onde você está. As janelas do próprio Trindle mudam na hora; o anel, na próxima abertura. Uma cor escura demais para ser vista em um cartão é clareada até ficar legível, e, em um visual claro, uma cor pálida demais é escurecida. Escuro ou Claro se escolhe no cartão do próprio tema, em Tema.

# Heading of the line with the highlight colour swatches.
settings-appearance-highlight = Destaque

# Small words under that line.
settings-appearance-highlight-about = Botões, interruptores e onde você está.

# Small words under the read the screen aloud switch when a screen reader is installed.
settings-appearance-reader-on = Super, Alt e S também a ligam e desligam.

# Heading of the line, and name of its switch, for reading the screen aloud.
settings-appearance-reader = Ler a tela em voz alta

# Heading of the line, and name of its switch, for keeping labels while pointing.
settings-appearance-labels = Manter rótulos ao apontar

# Small words under that line.
settings-appearance-labels-about = O nome de um botão fica visível enquanto você aponta para ele.

# Title of the Reading the screen card.
settings-appearance-reading-title = Leitura da tela

# Words behind the i of that card; { $says } is the first sentences, from the key reading-says.
settings-appearance-reading-about = { $says } O nome de um botão costuma sumir depois de um segundo e meio; se mantido, fica enquanto o ponteiro estiver sobre ele. Enquanto a tela é lida em voz alta, todo botão mostra seu nome embaixo.

# Name of the set of round icons.
settings-appearance-icons-circles = Círculos

# Words beside the turning ring while the icons are remade.
settings-appearance-icons-working = Mudando os ícones

# Green words under the shape line after the icons were remade.
settings-appearance-icons-changed = Alterado

# Name of the button that chooses an icon set; { $name } is the set, such as Circles.
settings-appearance-icons-set = Ícones: { $name }

# Heading of the line for the shape of the icons.
settings-appearance-shape = Forma

# Small words under that line.
settings-appearance-shape-about = Os Ícones Trindle, redondos ou quadrados.

# Title of the Icons card.
settings-appearance-icons-title = Ícones

# Words behind the i of the Icons card.
settings-appearance-icons-about = Os Ícones Trindle vêm redondos ou quadrados. Um aplicativo que eles não desenham ganha um ícone feito para combinar, e o anel e o painel são redesenhados com a nova forma.

# Name of the list of font families, and heading of the font line.
settings-appearance-font = Fonte

# Name of the font line's choice before the font in use has been read, so a screen reader has a word to say.
settings-appearance-font-default = Padrão

# Name of the list of font sizes.
settings-appearance-font-size = Tamanho da fonte

# Small words under the font line.
settings-appearance-font-about = As palavras nas janelas dos programas.

# Name of the button that adds fonts, and heading of its line.
settings-appearance-add-fonts = Adicionar fontes

# Small words under the Add fonts line.
settings-appearance-add-fonts-about = Um arquivo .ttf ou .otf, ou um zip deles.

# Words of the link that opens help about what a font file must contain.
settings-appearance-fonts-help = O que o arquivo deve conter

# Title of the Text card.
settings-appearance-text-title = Texto

# Words behind the i of the Text card.
settings-appearance-text-about = A fonte que os programas usam para suas palavras, e seu tamanho. As fontes adicionadas aqui, um arquivo .ttf ou .otf ou um zip deles, entram na lista de fontes só para você; alguns programas só percebem depois de reabertos.

# Title of the window for choosing font files.
settings-appearance-chooser-fonts = Escolher fontes

# Red words when a font could not be replaced; { $name } is the file name.
settings-appearance-font-not-replaced = Não foi possível substituir { $name }.

# Green words after one font took the place of the one there; { $name } is the file name.
settings-appearance-font-replaced-one = { $name } substituída. Os programas iniciados a partir de agora a usam.

# Green words after several fonts took the place of the ones there; { $n } of { $total } were done.
settings-appearance-font-replaced-some = { $n } de { $total } substituídas. Os programas iniciados a partir de agora as usam.

# Green words after fonts were added; { $n } is how many.
settings-appearance-fonts-added =
    { $n ->
        [one] { $n } fonte está na lista acima. Escolha-a acima.
        [many] { $n } de fontes estão na lista acima. Escolha-as acima.
       *[other] { $n } fontes estão na lista acima. Escolha-as acima.
    }

# Green words after added fonts were already in the list.
settings-appearance-fonts-in-list = Essas fontes já estão na lista.

# Choice in the list of cursor sizes: the normal size.
settings-appearance-cursor-normal = Normal

# Choice in the list of cursor sizes: a large size.
settings-appearance-cursor-large = Grande

# Choice in the list of cursor sizes: the largest size.
settings-appearance-cursor-larger = Maior

# Spoken name of a picture of a cursor size; { $size } is its name, such as Large.
settings-appearance-pointer-spoken = Ponteiro: { $size }

# Name of the standard cursor theme (the one every system has).
settings-appearance-cursor-standard = Padrão

# Name of the list of cursor themes, and heading of its line.
settings-appearance-cursor = Cursor

# Name of the list of cursor sizes, and heading of its line.
settings-appearance-cursor-size = Tamanho do cursor

# Small words under the cursor line.
settings-appearance-cursor-about = Como o ponteiro é desenhado.

# Small words under the cursor size line.
settings-appearance-cursor-size-about = Maior é mais fácil de achar.

# Title of the Pointer card.
settings-appearance-pointer-title = Ponteiro

