
# machine translated (French), not yet reviewed by a person.

# Name of the screenshot overlay that covers the screen (what a screen reader calls it).
screenshot-name = Capture d'écran

# Note on the overlay before any part is chosen.
screenshot-drag = Faites glisser sur la partie à garder

# Name of the round button that keeps the whole screen.
screenshot-whole = Tout l'écran

# Name of the round button that closes the overlay or a pinned picture.
screenshot-close = Fermer

# Name of the round button that gives the screenshot up.
screenshot-cancel = Annuler

# Name of the round button that takes back the last mark.
screenshot-undo = Annuler le dernier trait

# Name of the round button that puts back the last mark taken away.
screenshot-redo = Rétablir

# Name of the round button that copies the picture to the clipboard, and of the copy button of a colour.
screenshot-copy = Copier

# Name of a colour's button after it has been copied.
screenshot-copied = Copié

# Word on the button that saves the picture in the Screenshots folder.
screenshot-save = Enregistrer

# Word on the button that asks where to save the picture.
screenshot-save-as = Enregistrer sous

# Name of the round button that keeps the picture on the screen as a small window.
screenshot-pin = Épingler à l'écran

# Name of the drawing tool that draws freehand.
screenshot-tool-pen = Crayon

# Name of the drawing tool that draws a straight line.
screenshot-tool-line = Ligne

# Name of the drawing tool that draws an arrow.
screenshot-tool-arrow = Flèche

# Name of the drawing tool that draws a box.
screenshot-tool-box = Rectangle

# Name of the drawing tool that draws a circle.
screenshot-tool-circle = Cercle

# Name of the drawing tool that highlights.
screenshot-tool-highlighter = Surligneur

# Name of the drawing tool that writes words.
screenshot-tool-words = Texte

# Name of the drawing tool that puts numbered spots.
screenshot-tool-numbers = Numéros

# Name of the drawing tool that blurs a patch.
screenshot-tool-blur = Flou

# Name of the round button that draws in red.
screenshot-colour-red = Rouge

# Name of the round button that draws in blue.
screenshot-colour-blue = Bleu

# Name of the round button that draws in green.
screenshot-colour-green = Vert

# Name of the round button that draws in orange.
screenshot-colour-orange = Orange

# Name of the round button that draws in purple.
screenshot-colour-purple = Violet

# Name of the round button that draws in white.
screenshot-colour-white = Blanc

# Name of the round button that draws in black.
screenshot-colour-black = Noir

# Name of the round button that draws thin lines.
screenshot-thickness-thin = Fin

# Name of the round button that draws lines of middle thickness.
screenshot-thickness-medium = Moyen

# Name of the round button that draws thick lines.
screenshot-thickness-thick = Épais

# Name the screen reader gives to a screenshot pinned on the screen.
screenshot-pinned = Capture d'écran épinglée

# Title of the window that asks where to save the screenshot.
screenshot-save-title = Enregistrer la capture d'écran

# Title of the notice that says the screenshot could not be saved.
screenshot-not-saved = Capture d'écran non enregistrée

# Title of the notice that says the screenshot could not be copied.
screenshot-not-copied = Capture d'écran non copiée

# Words of that notice.
screenshot-clipboard-refused = Le presse-papiers n'a pas pu la prendre.

# Words of the notice that the screenshot could not be saved: the picture was not made.
screenshot-no-picture = l'image n'a pas pu être créée

# Words of the notice that the screenshot could not be saved: no bus to ask on.
screenshot-no-bus = il n'y a pas de bus de session pour demander où

# Words of the notice that the screenshot could not be saved: the file chooser could not be asked.
screenshot-no-chooser = le sélecteur de fichiers n'a pas pu être sollicité

# Words of the notice that the screenshot could not be saved: every file name was taken.
screenshot-no-free-name = aucun nom libre

# Words of the notice that the screenshot could not be saved: the path and why it failed.
screenshot-write-failed = { $path } : { $why }

# Name of the eye dropper's window, and its title.
screenshot-eye-title = Pipette

# Words that float beside the i of the eye dropper's window.
screenshot-eye-help = Appuyez sur la pipette, puis cliquez n'importe où sur l'écran pour prendre la couleur sous le pointeur. Échap abandonne. Chaque couleur est copiée aussitôt sous forme de code hexadécimal et gardée dans la liste ci-dessous, la plus récente en premier, pour pouvoir la copier de nouveau plus tard.

# Name of the i button beside the title.
screenshot-eye-about = À propos de la pipette

# Name of the main button that takes a colour.
screenshot-eye-take = Prendre une couleur

# Sentence under the title while no colour has been taken.
screenshot-eye-none = Prenez une couleur n'importe où sur l'écran.

# Sentence under the title once colours have been taken.
screenshot-eye-taken =
    { $n ->
        [one] Une couleur prise : { $newest }.
       *[other] { $n } couleurs prises ; la plus récente est { $newest }.
    }

# Shown in the big card while no colour has been taken.
screenshot-eye-latest-empty = La couleur que vous prenez s'affiche ici, en grand, prête à copier.

# Name of the card that holds the latest colour.
screenshot-eye-latest = Dernière couleur

# Name of the round button that clears the list of colours.
# SAFETY: needs a human review before it ships
screenshot-eye-clear = Vider la liste

# Head of the card with the list of colours taken.
screenshot-eye-list = Couleurs prises

# Shown in the list card while no colour has been taken.
screenshot-eye-list-empty = Les couleurs prises avec la pipette sont gardées ici, la plus récente en premier.

# Name of the round button that takes a colour out of the list.
screenshot-eye-remove = Retirer

# What the saved list of colours is called when it is found damaged and set aside.
screenshot-eye-damaged = Couleurs de la pipette

# Title of the window that asks what an app may share.
screenshot-pick-title = Partager votre écran avec { $app } ?

# Sentence under the title when several things can be picked.
screenshot-pick-many = Choisissez-en un ou plusieurs. Seul ce que vous choisissez est partagé, jusqu'à ce que l'application s'arrête ou se ferme.

# Sentence under the title when one thing can be picked.
screenshot-pick-one = Choisissez-en un. Seul ce que vous choisissez est partagé, jusqu'à ce que l'application s'arrête ou se ferme.

# Heading over the screens that can be shared.
screenshot-pick-screens = Écrans

# Heading over the windows that can be shared.
screenshot-pick-windows = Fenêtres

# Shown when there is nothing to share.
screenshot-pick-nothing = Il n'y a rien à partager.

# Name of the choice that shares every screen at once.
screenshot-pick-desktop = Tout le bureau

# Words under that choice.
screenshot-pick-desktop-detail = Tous les écrans, tels qu'ils sont disposés

# Words under a screen's choice: its size in pixels.
screenshot-pick-size = { $width } sur { $height }

# What the screen reader says for a choice that is picked.
screenshot-pick-chosen = { $name }, choisi

# Name of the round button that refuses to share.
screenshot-pick-refuse = Ne pas partager

# Name of the main button that shares what is picked.
# SAFETY: needs a human review before it ships
screenshot-pick-share = Partager

# Name of the button that says no in a question from a program, and of its answer to the program.
# SAFETY: needs a human review before it ships
screenshot-portal-deny = Refuser

# Name of the main button that says yes in a question from a program.
# SAFETY: needs a human review before it ships
screenshot-portal-allow = Autoriser

# Name of the round button that gives up a window from a program.
screenshot-portal-cancel = Annuler

# Name of the main button that opens, and the title of a window that opens a file.
screenshot-portal-open = Ouvrir

# Title of the window that saves a file, and the word of its main button.
screenshot-portal-save = Enregistrer

# Name of the box where a name is typed.
screenshot-portal-name = Nom

# Heading when an unknown program asks who the person is.
screenshot-account-asks-unknown = Un programme demande qui vous êtes

# Heading when a program asks who the person is.
screenshot-account-asks = { $who } demande qui vous êtes

# Name the screen reader gives to the person's picture.
screenshot-account-picture = Votre image

# Words under the person's name.
screenshot-account-signed-in = Connecté en tant que { $id }

# Name of the search box in the window that asks which app should open a file.
screenshot-apps-search = Rechercher des applications

# Shown when no app can open the file.
screenshot-apps-none = Aucune application ici ne peut l'ouvrir

# Name of the switch that remembers the choice.
screenshot-apps-always = Toujours utiliser cette application pour ce type de fichier

# Title of the window that asks which app should open a file.
screenshot-apps-open-with = Ouvrir { $name } avec

# Title of that window when the file's name is not known.
screenshot-apps-choose = Choisir une application

# Name of the screen sharing that other programs see, and the name Trindle gives itself to PipeWire.
screenshot-cast-name = Partage d'écran Trindle

# What other programs show for the shared screen.
screenshot-cast-node = Écran partagé avec une application

# Name used for a program that cannot be named, in the window that asks what to share.
screenshot-cast-an-app = Une application

# Title of the window that asks what to share.
screenshot-cast-window = Partager l'écran

# Title of the window that saves several files into a folder.
screenshot-files-save-many = Enregistrer dans un dossier

# Title of the window that chooses a folder.
screenshot-files-folder = Choisir un dossier

# What a program holding the computer awake is called when it gave no name.
screenshot-inhibit-who = Un programme

# Heading when a program wants to add an app to the person's apps and cannot be named.
screenshot-launcher-ask = Ajouter ceci à vos applications ?

# Heading when a named program wants to add an app to the person's apps.
screenshot-launcher-asks = { $who } veut l'ajouter à vos applications

# Name the screen reader gives to the app's icon.
screenshot-launcher-icon = Son icône

# Words under the name, when the app opens a web address.
screenshot-launcher-opens = Ouvre { $target }

# Title of the window that asks to add an app.
screenshot-launcher-title = Ajouter une application

# Name of the main button that adds the app.
screenshot-launcher-add = Ajouter

# Heading when a program that cannot be named offers a wallpaper.
screenshot-wallpaper-ask = Définir cette image comme fond d'écran ?

# Heading when a program offers a wallpaper.
screenshot-wallpaper-asks = { $who } propose un nouveau fond d'écran

# Name the screen reader gives to the picture offered.
screenshot-wallpaper-picture = Le nouveau fond d'écran

# Name of the main button that sets the wallpaper.
screenshot-wallpaper-set = Définir comme fond d'écran

# Red line under the printer box when no printer is set up.
screenshot-print-no-printer = Aucune imprimante configurée. Ajoutez-en une dans Réglages, Imprimantes.

# Red line under the printer box when the printing system does not answer.
screenshot-print-not-answering = L'impression ne répond pas.

# Red line under the printer box when printing is not installed.
screenshot-print-not-installed = L'impression n'est pas installée sur cet ordinateur.

# Red line under the printer box when the chosen printer is paused.
screenshot-print-paused = { $name } est en pause.

# Label and name of the box where the printer is chosen.
screenshot-print-printer = Imprimante

# Label and name of the choice of which pages are printed.
screenshot-print-pages = Pages

# The choice to print all the pages.
screenshot-print-all = Toutes

# The choice to print a range of pages.
screenshot-print-range = Plage

# Label and name of the box for the number of copies.
screenshot-print-copies = Copies

# Label and name of the choice of printing on both sides.
screenshot-print-sides = Recto verso

# Label and name of the choice of colour, and the choice for colour.
screenshot-print-colour = Couleur

# The choice to print in black and white.
screenshot-print-mono = Noir et blanc

# Label and name of the choice of paper size.
screenshot-print-paper = Papier

# Label and name of the choice of how the page stands.
screenshot-print-page = Page

# Name of the main button that prints, and the title when nothing else is known.
screenshot-print-go = Imprimer

# Title of the print window for a named document.
screenshot-print-doc = Imprimer { $doc }

# Title of the print window when the program has no document name.
screenshot-print-from = Imprimer depuis { $who }

# Title of the notice that says a print did not go.
screenshot-print-failed = Impression impossible

# Words of that notice when the choices made were not kept.
screenshot-print-lost = Ses choix n'ont pas été gardés. Imprimez de nouveau.

# Line in the menu on a file or folder: change its name.
screenshot-chooser-rename = Renommer

# Line in the menu on a file or folder: cut it.
screenshot-chooser-cut = Couper

# Line in the menu: paste what was cut or copied.
screenshot-chooser-paste = Coller

# Line in the menu and name of the round button: make a new folder.
screenshot-chooser-new-folder = Nouveau dossier

# Line in the menu: move to the Trash.
# SAFETY: needs a human review before it ships
screenshot-chooser-trash = Mettre à la corbeille

# Name of the first place in the side bar, and of the only part of the path when Recent is shown.
screenshot-chooser-recent = Récents

# Name of the person's home folder in the side bar.
screenshot-chooser-home = Dossier personnel

# Name the screen reader gives to the path along the top.
screenshot-chooser-folder = Dossier

# Shown when a folder has nothing in it.
screenshot-chooser-empty = Rien ici

# Name of the choice of which kind of file is shown.
screenshot-chooser-kind = Type de fichier

# Said where a place on another computer is not answering.
screenshot-chooser-not-answering = L'autre ordinateur ne répond pas.

# What the saved list of last folders is called when it is found damaged and set aside.
screenshot-chooser-damaged = Dossiers du sélecteur

# Words under the box when the folder a name points into is not there.
screenshot-chooser-no-folder = Ce dossier n'existe pas.

# Words under the box when saving would replace a file.
screenshot-chooser-replace-ask = { $name } existe déjà. Appuyez sur Remplacer, ou de nouveau sur Entrée, pour le mettre à sa place.

# Words under the box when a key is pressed on Recent, which is not a folder.
screenshot-chooser-recent-not-folder = Les récents ne sont pas un dossier. Ouvrez d'abord un dossier.

# Words under the box when a key is pressed with nothing picked.
screenshot-chooser-nothing-picked = Rien n'est choisi.

# Words under the box while a file by that name is already there.
screenshot-chooser-there = { $name } existe déjà.

# Name of the round button that hides the hidden files.
screenshot-chooser-hide-hidden = Masquer les fichiers cachés

# Name of the round button that shows the hidden files.
screenshot-chooser-show-hidden = Afficher les fichiers cachés

# Words of the main button when the program gave none, to choose a folder.
screenshot-chooser-choose-folder = Choisir le dossier

# Head of the question when a folder of that name is already there.
screenshot-chooser-clash-folder = Un dossier nommé { $name } existe déjà

# Head of the question when a file of that name is already there.
screenshot-chooser-clash-file = { $name } existe déjà

# Name of the round button that leaves a pasted thing out.
screenshot-chooser-skip = Ignorer

# Name of the round button that keeps both under different names.
screenshot-chooser-both = Garder les deux

# Word on the button that replaces what is there.
# SAFETY: needs a human review before it ships
screenshot-chooser-replace = Remplacer

# What the screen reader says for the button that replaces a folder.
# SAFETY: needs a human review before it ships
screenshot-chooser-replace-spoken = Remplacer : le dossier déjà présent va à la corbeille

# Name of the main button that merges two folders.
screenshot-chooser-merge = Fusionner

# Words in the question when folders are being pasted.
screenshot-chooser-rest-folders = La même réponse pour chaque dossier déjà présent.

# Words in the question when files are being pasted.
screenshot-chooser-rest-files = La même réponse pour chaque fichier déjà présent.

# Name of the switch that gives the same answer to the rest.
screenshot-chooser-same = Faire de même pour les suivants

# Words under the box after Cut.
screenshot-chooser-was-cut = { $what } coupé. Collez-le dans un dossier.

# Words under the box after Copy.
screenshot-chooser-was-copied = { $what } copié. Collez-le dans un dossier.

# Words under the box when Paste is pressed and there is nothing to paste.
screenshot-chooser-nothing-cut = Rien n'a été coupé ni copié.

# Words under the box when Paste is pressed while a paste is still going.
screenshot-chooser-paste-busy = Un collage est encore en cours.

# Red line after some items could not be moved.
screenshot-chooser-not-moved =
    { $n ->
        [one] Un élément n'a pas pu être déplacé : { $names }.
       *[other] { $n } éléments n'ont pas pu être déplacés : { $names }.
    }

# Red line after some items could not be copied.
screenshot-chooser-not-copied =
    { $n ->
        [one] Un élément n'a pas pu être copié : { $names }.
       *[other] { $n } éléments n'ont pas pu être copiés : { $names }.
    }

# Red line after some items could not be put in the Trash.
screenshot-chooser-not-trashed =
    { $n ->
        [one] Un élément n'a pas pu être mis à la corbeille : { $names }.
       *[other] { $n } éléments n'ont pas pu être mis à la corbeille : { $names }.
    }

# Name of the box where a file or folder is renamed.
screenshot-chooser-new-name = Nouveau nom pour { $old }

# What several things are called in a line such as Cut.
screenshot-chooser-items =
    { $n ->
        [one] { $n } élément
       *[other] { $n } éléments
    }

# Title of the notice when an email could not be started.
screenshot-mail-failed = Impossible de lancer le courriel

# Why: no mail program is chosen or installed.
screenshot-mail-no-app = Aucun programme de courriel n'est défini. Choisissez-en un dans Réglages, Applications par défaut.

# Why: a program asked to attach more files than allowed.
screenshot-mail-too-many = Trop de fichiers à joindre.

# Why: a file, or all the files together, are too big.
screenshot-mail-too-big = Les fichiers sont trop volumineux pour être joints.

# Why: a file could not be read as a plain file.
screenshot-mail-bad-file = Un fichier n'a pas pu être joint.

# Why: an address was not a plain address.
screenshot-mail-bad-address = Une adresse n'était pas valide.

# Why: the subject was too long or had odd characters.
screenshot-mail-bad-subject = L'objet n'était pas valide.

# Why: the words were too long or had odd characters.
screenshot-mail-bad-words = Le texte n'était pas valide.

# Why: no private place to keep the files.
screenshot-mail-no-place = Les fichiers n'ont pas pu être gardés en sécurité.

# Why: the mail program did not start.
screenshot-mail-not-started = Le programme de courriel ne s'est pas lancé.

# Line in the menu on a file or folder: delete it for good, after asking.
screenshot-chooser-delete = Supprimer définitivement

# Question before deleting for good.
screenshot-chooser-delete-ask = Supprimer définitivement ?

# Line in the menu: copy the place of the thing as text.
screenshot-chooser-copy-location = Copier l'emplacement

# Line in the menu on a folder: pin it to Folders' side bar.
screenshot-chooser-pin = Épingler à la barre latérale

# Line in the menu on a folder pinned already.
screenshot-chooser-unpin = Désépingler

# Red line after a pin could not be kept.
screenshot-chooser-pin-not-kept = L'épingle n'a pas pu être enregistrée.

# Red line after some items could not be deleted.
screenshot-chooser-not-deleted =
    { $n ->
        [one] { $n } élément n'a pas pu être supprimé : { $names }.
       *[other] { $n } éléments n'ont pas pu être supprimés : { $names }.
    }

