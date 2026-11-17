
# machine translated (French), not yet reviewed by a person.

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Text

# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = Sans titre

# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = Ouvrir

# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = Nouveau (Ctrl+N)

# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = Ouvrir (Ctrl+O)

# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = Fichiers récents

# Button in the menu that saves the file as it is.
text-save = Enregistrer

# The words a screen reader gives the Save button, with its shortcut.
text-save-key = Enregistrer (Ctrl+S)

# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = Enregistrer sous

# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = Enregistrer sous (Ctrl+Maj+S)

# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = Annuler (Ctrl+Z)

# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = Rétablir (Ctrl+Maj+Z)

# Button in the menu that opens the find bar, with its shortcut.
text-find-key = Rechercher (Ctrl+F)

# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = Rechercher et remplacer (Ctrl+H)

# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = Aller à la ligne (Ctrl+G)

# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = Aperçu (Ctrl+E)

# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = Exporter en page web

# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = Exporter en PDF

# Button in the menu that turns on wrapping of long lines.
text-wrap = Renvoyer les longues lignes

# Button in the menu that shows or hides the line numbers.
text-line-numbers = Numéros de ligne

# Button in the menu that opens the Text settings.
text-settings = Réglages

# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = Fermer { $name }

# Tip of the button that closes the open bar.
text-close-bar = Fermer la barre

# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = Aperçu

# Menu item that copies everything in the text.
text-copy-all = Copier tout le texte

# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = Couper

text-copy = Copier

text-paste = Coller

text-select-all = Tout sélectionner

# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = Vérifier l'orthographe

# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = « { $c } » ne peut pas être écrit en { $encoding }. Choisissez un autre encodage dans la barre d'état.

# The words on the status bar: the line and column of the caret.
text-line-column = Ligne { $line }, colonne { $col }

# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
        [one] { $words } mot
       *[other] { $words } mots
    }, { $letters ->
        [one] { $letters } lettre
       *[other] { $letters } lettres
    }

# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
        [one] { $words } mot
       *[other] { $words } mots
    }, { $letters ->
        [one] { $letters } lettre
       *[other] { $letters } lettres
    } sélectionnés

# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Fins de ligne Windows

# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = Encodage : { $name }

# Words in place of the file's name when the text has never been saved to a file.
text-no-file = Ce texte n'a pas encore de fichier.

# Words on the find bar: the find box's label and placeholder.
text-find = Rechercher

# Words on the find bar: a switch to find only the same capitals.
text-match-case = Respecter la casse

# Words on the find bar: a switch to find only whole words.
text-whole-words = Mots entiers

# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = Précédent (Maj+F3)

# Tip of the button that goes to the next find, with its shortcut.
text-next-key = Suivant (F3)

# Words on the replace bar: the replace box's label.
text-replace-with = Remplacer par

# Button on the replace bar that replaces the word found.
text-replace = Remplacer

# The words a screen reader gives the Replace button.
text-replace-spoken = Remplacer le mot trouvé

# SAFETY: needs a human review before it ships
# Button on the replace bar that replaces every word found.
text-replace-all = Tout remplacer

# SAFETY: needs a human review before it ships
# The words a screen reader gives the Replace all button.
text-replace-all-spoken = Remplacer chaque mot trouvé

# Words on the find bar when a find has been asked for and has no match.
text-none-found = Aucun résultat

# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } sur { $n }

# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n } trouvés

# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
        [one] 1 remplacé
       *[other] { $n } remplacés
    }

# Words on the go to line bar when the number typed is not a line number.
text-type-line = Tapez un numéro de ligne

# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 à { $lines }

# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = sur { $lines }

# Button on the go to line bar that goes to the line typed.
text-go = Aller

# Words on the go to line bar: the box's label.
text-go-to-line = Aller à la ligne

# Button that goes back from a bar, a settings page or a list to the text.
text-back = Retour au texte

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = a été ajouté à vos mots

# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = n'est pas dans le dictionnaire, et il n'y a aucune suggestion

# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = n'est pas dans le dictionnaire. Vouliez-vous dire l'un de ceux-ci ?

# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = Utiliser cette orthographe

# Button on the spelling bar that adds the word to the person's own words.
text-add-word = Ajouter à mes mots

# Heading over the list of suggested spellings.
text-suggestions = Orthographes proposées

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = Les fichiers que vous ouvrez ou enregistrez apparaissent ici.

# SAFETY: needs a human review before it ships
# Button that clears the list of recent files.
text-clear-list = Vider la liste

# Heading of the Text settings page.
text-settings-title = Réglages de Text

# Heading of the group on the settings page for when Text starts.
text-starting = Démarrage

# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = À l'ouverture de Text

# First choice: Text opens the files it had open last time.
text-reopen = Rouvrir les fichiers précédents

# Second choice: Text opens with an empty page.
text-start-blank = Repartir de zéro

# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = Votre travail non enregistré est conservé et restauré, quelle que soit l'option choisie.

# Heading of the group on the settings page for how the text looks.
text-showing = Apparence du texte

# Label of the size slider: how big the text is.
text-size = Taille

# Name of the size slider, read out by a screen reader.
text-text-size = Taille du texte

# Heading of the group on the settings page for spelling.
text-spelling = Orthographe

# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = Aucun dictionnaire Hunspell n'est installé.

# Label of the list of spelling languages.
text-language = Langue

# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = Langue d'orthographe

# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = Automatique ({ $lang })

# Heading of the font list on the settings page.
text-font = Police

# The first font in the list: the font Trindle ships with.
text-own-font = Celle de Trindle

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = Lire le fichier comme

# Heading over the encodings a file is saved in.
text-save-in = Enregistrer en

# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = Le fichier utilisera cet encodage au prochain enregistrement.

# SAFETY: needs a human review before it ships
# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name } a des modifications non enregistrées.

# SAFETY: needs a human review before it ships
# Words saying the text has changes and no file yet.
text-not-saved = Ce texte n'est pas enregistré dans un fichier.

# SAFETY: needs a human review before it ships
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = Enregistrer { $name } ?

# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after =
    { $left ->
        [one] Encore { $left } onglet à enregistrer.
       *[other] Encore { $left } onglets à enregistrer.
    }

# SAFETY: needs a human review before it ships
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = Fermer sans enregistrer

# SAFETY: needs a human review before it ships
# Button in the question before closing that keeps the tab open.
text-keep-open = Le garder ouvert

# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }

# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name } est un dossier.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name } est trop gros pour être ouvert ici.

# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = Trindle n'a pas pu ouvrir { $name } : { $reason }

# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = Trindle n'a pas pu écrire { $name } : { $reason }

# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = Trindle n'a pas pu enregistrer { $name } : { $reason }

# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = Impossible de lire { $name } : { $reason }

# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Trindle n'a pas pu enregistrer votre travail dans { $folder } : { $reason }

# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = Impossible d'enregistrer les réglages : { $reason }

# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = Trindle n'a pas pu ajouter { $word } à votre dictionnaire : { $reason }

# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Trindle n'a pas pu vider la liste : { $reason }

# Shown when the file chooser could not be opened.
text-chooser-failed = Impossible d'ouvrir le sélecteur de fichiers.

# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Trindle ne peut pas ouvrir le sélecteur de fichiers pour le moment.

# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = Il n'y a pas de dictionnaire nommé { $lang }.

# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Trindle n'a pas pu lire le dictionnaire { $lang } : { $reason }

# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. L'ancien texte est enregistré dans { $path }.

# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } Le texte d'avant cet enregistrement se trouve dans { $path }.

# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = Text s'est fermé pendant l'enregistrement de { $name }, et Trindle ne peut pas lire la partie qui avait été enregistrée.

# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = Text s'est fermé pendant l'enregistrement de { $name }. Le fichier a changé depuis, Trindle l'a donc laissé tel quel.

# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = Text s'est fermé pendant l'enregistrement de { $name }, et Trindle n'a pas pu remettre le fichier comme avant : { $reason }. Le texte d'avant cet enregistrement se trouve dans { $path }.

# Quiet word beside what was just saved, exported or changed.
text-done = Terminé

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = Sans titre.txt

# The misspelt word, shown in quotation marks on the spelling bar. { $word } is the word.
text-spell-word = « { $word } »



# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = Ce fichier a changé depuis son ouverture.

# Button beside that line: save the words here over the changed file.
text-save-anyway = Enregistrer quand même

# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = Recharger

# Shown when a link in the finished look is pressed and is not a web address, or no browser started.
text-link-not-opened = Seules les adresses web s'ouvrent d'ici.
