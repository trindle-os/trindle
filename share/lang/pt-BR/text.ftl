
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Texto

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Sem título

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Abrir

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Novo (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Abrir (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Arquivos recentes

# Button in the menu that saves the file as it is.
text-save = Salvar

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Salvar (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Salvar como

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Salvar como (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Desfazer (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Refazer (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Buscar (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Buscar e substituir (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Ir para a linha (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Visual final (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Exportar como página da web

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Exportar como PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Quebrar linhas longas

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Números das linhas

# Button in the menu that opens the Text settings.
text-settings = Configurações

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Fechar { $name }

# Tip of the button that closes the open bar.
text-close-bar = Fechar a barra

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Visual final

# Menu item that copies everything in the text.
text-copy-all = Copiar todo o texto

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Recortar

text-copy = Copiar

text-paste = Colar

text-select-all = Selecionar tudo

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Verificar ortografia

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ não pode ser gravado em { $encoding }. Escolha outra codificação na barra de status.

# The words on the status bar: the line and column of the caret.
text-line-column = Linha { $line }, coluna { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } palavra
        [many] { $words } palavras
       *[other] { $words } palavras
    }, { $letters ->
        [one] { $letters } letra
        [many] { $letters } letras
       *[other] { $letters } letras
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } palavra
        [many] { $words } palavras
       *[other] { $words } palavras
    }, { $letters ->
        [one] { $letters } letra
        [many] { $letters } letras
       *[other] { $letters } letras
    } selecionadas

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Fins de linha do Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Codificação: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Este texto ainda não tem arquivo.

# Words on the find bar: the find box's label and placeholder.
text-find = Buscar

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Diferenciar maiúsculas

# Words on the find bar: a switch to find only whole words.
text-whole-words = Palavras inteiras

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Anterior (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Próximo (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Substituir por

# SAFETY: needs a human review before it ships
# Button on the replace bar that replaces the word found.
text-replace = Substituir

# SAFETY: needs a human review before it ships
# The words a screen reader gives the Replace button.
text-replace-spoken = Substituir a palavra encontrada

# SAFETY: needs a human review before it ships
# Button on the replace bar that replaces every word found.
text-replace-all = Substituir tudo

# SAFETY: needs a human review before it ships
# The words a screen reader gives the Replace all button.
text-replace-all-spoken = Substituir todas as palavras encontradas

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Nenhuma encontrada

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } de { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } encontradas

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 substituída
        [many] { $n } substituídas
       *[other] { $n } substituídas
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Digite um número de linha

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 a { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = de { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Ir

# Words on the go to line bar: the box's label.
text-go-to-line = Ir para a linha

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Voltar ao texto

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = agora é uma das suas palavras

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = não está no dicionário, que não tem nada parecido

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = não está no dicionário. Você quis dizer uma destas?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Usar esta grafia

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Adicionar às minhas palavras

# Heading over the list of suggested spellings.
text-suggestions = Grafias sugeridas

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Os arquivos que você abrir ou salvar aparecem aqui.

# Button that clears the list of recent files.
text-clear-list = Limpar a lista

# Heading of the Text settings page.
text-settings-title = Configurações de Texto

# Heading of the group on the settings page for when Text starts.
text-starting = Início

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Quando o Texto abrir

# First choice: Text opens the files it had open last time.
text-reopen = Reabrir os arquivos da última vez

# Second choice: Text opens with an empty page.
text-start-blank = Começar em branco

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = O trabalho não salvo em um arquivo é sempre guardado e volta de qualquer jeito.

# Heading of the group on the settings page for how the text looks.
text-showing = Exibição do texto

# Label of the size slider: how big the text is.
text-size = Tamanho

# Name of the size slider, read out by a screen reader.
text-text-size = Tamanho do texto

# Heading of the group on the settings page for spelling.
text-spelling = Ortografia

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Nenhum dicionário Hunspell está instalado.

# Label of the list of spelling languages.
text-language = Idioma

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Idioma da ortografia

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Automático ({ $lang })

# Heading of the font list on the settings page.
text-font = Fonte

# The first font in the list: the font Trindle ships with.
text-own-font = A própria do Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Ler o arquivo como

# Heading over the encodings a file is saved in.
text-save-in = Salvar em

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = O próximo salvamento grava o arquivo nesta codificação.

# SAFETY: needs a human review before it ships
# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } tem alterações não salvas no arquivo.

# SAFETY: needs a human review before it ships
# Words saying the text has changes and no file yet.
text-not-saved = Este texto não foi salvo em um arquivo.

# SAFETY: needs a human review before it ships
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = Salvar { $name }?

# SAFETY: needs a human review before it ships
# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = Mais { $left } depois deste.

# SAFETY: needs a human review before it ships
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = Fechar sem salvar

# SAFETY: needs a human review before it ships
# Button in the question before closing that keeps the tab open.
text-keep-open = Manter aberto

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } é uma pasta.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } é grande demais para abrir aqui.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = Não foi possível abrir { $name }: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = Não foi possível gravar { $name }: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = Não foi possível salvar { $name }: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = Não foi possível ler { $name }: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Não foi possível guardar o trabalho em { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Não foi possível salvar as configurações: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = Não foi possível adicionar { $word } às suas palavras: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Não foi possível limpar a lista: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Não foi possível abrir o seletor de arquivos.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Não é possível escolher arquivos sem um barramento de sessão.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Não existe um dicionário chamado { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Não foi possível ler o dicionário { $lang }: { $reason }

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. O que ele tinha antes foi guardado em { $path }.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } O que ele tinha antes desse salvamento foi guardado em { $path }.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } estava sendo salvo quando o Texto foi interrompido, e o que foi guardado do salvamento não pode ser lido.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } estava sendo salvo quando o Texto foi interrompido e foi alterado desde então, por isso foi deixado como está.

# SAFETY: needs a human review before it ships
# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } estava sendo salvo quando o Texto foi interrompido e não pôde ser devolvido como estava: { $reason }. O que ele tinha antes desse salvamento foi guardado em { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Pronto

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Sem título.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Alterado em outro lugar desde que você o abriu.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Salvar mesmo assim

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Recarregar

