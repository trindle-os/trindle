
# machine translated (Spanish), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Texto

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Sin título

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Abrir

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Nuevo (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Abrir (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Archivos recientes

# Button in the menu that saves the file as it is.
text-save = Guardar

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Guardar (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Guardar como

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Guardar como (Ctrl+Shift+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Deshacer (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Rehacer (Ctrl+Shift+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Buscar (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Buscar y reemplazar (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Ir a la línea (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Vista final (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Exportar como página web

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Exportar como PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Ajustar líneas largas

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Números de línea

# Button in the menu that opens the Text settings.
text-settings = Ajustes

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Cerrar { $name }

# Tip of the button that closes the open bar.
text-close-bar = Cerrar la barra

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Vista final

# Menu item that copies everything in the text.
text-copy-all = Copiar todo el texto

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Cortar

text-copy = Copiar

text-paste = Pegar

text-select-all = Seleccionar todo

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Revisar ortografía

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’ no se puede escribir en { $encoding }. Elige otra codificación en la barra de estado.

# The words on the status bar: the line and column of the caret.
text-line-column = Línea { $line }, columna { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } palabra
       *[other] { $words } palabras
    }, { $letters ->
        [one] { $letters } letra
       *[other] { $letters } letras
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } palabra
       *[other] { $words } palabras
    }, { $letters ->
        [one] { $letters } letra
       *[other] { $letters } letras
    } seleccionadas

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Finales de línea de Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Codificación: { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Este texto aún no tiene archivo.

# Words on the find bar: the find box's label and placeholder.
text-find = Buscar

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Distinguir mayúsculas

# Words on the find bar: a switch to find only whole words.
text-whole-words = Palabras completas

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Anterior (Shift+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Siguiente (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Reemplazar con

# Button on the replace bar that replaces the word found.
text-replace = Reemplazar

# The words a screen reader gives the Replace button.
text-replace-spoken = Reemplazar la palabra encontrada

# Button on the replace bar that replaces every word found.
text-replace-all = Reemplazar todo

# The words a screen reader gives the Replace all button.
text-replace-all-spoken = Reemplazar todas las palabras encontradas

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Sin resultados

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } de { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } encontrados

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 reemplazado
       *[other] { $n } reemplazados
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Escribe un número de línea

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 a { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = de { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Ir

# Words on the go to line bar: the box's label.
text-go-to-line = Ir a la línea

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Volver al texto

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = ya es una de tus palabras

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = no está en el diccionario, que no tiene nada parecido

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = no está en el diccionario. ¿Querías decir alguna de estas?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Usar esta ortografía

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Añadir a mis palabras

# Heading over the list of suggested spellings.
text-suggestions = Ortografías sugeridas

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Los archivos que abras o guardes aparecen aquí.

# Button that clears the list of recent files.
text-clear-list = Vaciar la lista

# Heading of the Text settings page.
text-settings-title = Ajustes de Texto

# Heading of the group on the settings page for when Text starts.
text-starting = Inicio

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Al abrir Texto

# First choice: Text opens the files it had open last time.
text-reopen = Reabrir los archivos de la última vez

# Second choice: Text opens with an empty page.
text-start-blank = Empezar en blanco

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = El trabajo no guardado en un archivo siempre se conserva y vuelve en ambos casos.

# Heading of the group on the settings page for how the text looks.
text-showing = Cómo se ve el texto

# Label of the size slider: how big the text is.
text-size = Tamaño

# Name of the size slider, read out by a screen reader.
text-text-size = Tamaño del texto

# Heading of the group on the settings page for spelling.
text-spelling = Ortografía

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = No hay diccionarios Hunspell instalados.

# Label of the list of spelling languages.
text-language = Idioma

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Idioma de la ortografía

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Automático ({ $lang })

# Heading of the font list on the settings page.
text-font = Tipo de letra

# The first font in the list: the font Trindle ships with.
text-own-font = La propia de Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Leer el archivo como

# Heading over the encodings a file is saved in.
text-save-in = Guardar en

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = El próximo guardado escribirá el archivo con esta codificación.

# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } tiene cambios sin guardar en su archivo.

# Words saying the text has changes and no file yet.
text-not-saved = Este texto no se ha guardado en un archivo.

# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = ¿Guardar { $name }?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after = { $left } más después de este.

# Button in the question before closing that closes the tab without saving it.
# SAFETY: needs a human review before it ships
text-close-no-save = Cerrar sin guardar

# Button in the question before closing that keeps the tab open.
text-keep-open = Mantener abierto

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } es una carpeta.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } es demasiado grande para abrirlo aquí.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = No se pudo abrir { $name }: { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = No se pudo escribir { $name }: { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = No se pudo guardar { $name }: { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = No se pudo leer { $name }: { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = No se pudo conservar el trabajo en { $folder }: { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = No se pudieron guardar los ajustes: { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = No se pudo añadir { $word } a tus palabras: { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = No se pudo vaciar la lista: { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = No se pudo abrir el selector de archivos.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = No se pueden elegir archivos sin un bus de sesión.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = No hay ningún diccionario llamado { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = No se pudo leer el diccionario { $lang }: { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. Lo que contenía antes se conserva en { $path }.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } Lo que contenía antes de ese guardado se conserva en { $path }.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name } se estaba guardando cuando Texto se detuvo, y no se puede leer lo que se conservó del guardado.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name } se estaba guardando cuando Texto se detuvo, y ha cambiado desde entonces, así que se deja como está.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name } se estaba guardando cuando Texto se detuvo, y no se pudo devolver a como estaba: { $reason }. Lo que contenía antes de ese guardado se conserva en { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Hecho

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Sin título.txt



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Ha cambiado en otro lugar desde que lo abriste.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Guardar de todos modos

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Volver a cargar

