# machine translated (French), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Clavier et souris

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = disposition langue saisie verrouillage majuscules clavier à l'écran bouton anneau souris pavé tactile pointeur vitesse toucher clic défilement molette plus lent plus rapide naturel gaucher module raccourci touches remapper changer échanger ctrl échap super alt mac boutons latéraux appui court maintenu créés par des programmes keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Tout ce qui est ici change immédiatement. Vous pouvez passer à une deuxième disposition en appuyant sur Alt et Maj ensemble.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Saisie en { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt et Maj basculent vers { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock ne fait rien.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock agit comme { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = une autre touche Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = Cet ordinateur n'a pas la liste des dispositions de clavier.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Clavier

# Heading of the line in that card.
settings-keyboard-no-list-heading = La liste des dispositions de clavier n'est pas disponible

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Pour l'installer, lancez : sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Vous avez modifié une touche ou un bouton.
       *[other] Vous avez modifié { $n } touches et boutons.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Disposition

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Disposition

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Variante

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Standard

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Aucune

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Basculer vers

# Words behind the i of the layout card.
settings-keyboard-layout-about = Ceci règle ce que chaque touche écrit. Vous pouvez choisir une disposition et une variante de celle-ci. Appuyez sur Alt et Maj ensemble pour passer à une deuxième disposition, et de nouveau pour revenir. Les changements sont immédiats.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = Utilisée maintenant

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Une autre touche Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Rien

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Utiliser Caps Lock comme

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Bouton de clavier dans l'anneau

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Autres touches

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock peut fonctionner comme une autre touche, par exemple une deuxième touche Ctrl ou Escape. Le bouton de clavier se trouve au milieu de l'anneau et ouvre un clavier à l'écran. Vous pouvez l'utiliser pour écrire à la souris ou sur un écran tactile. Il apparaît à la prochaine ouverture de l'anneau.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Un programme ne peut pas changer ceci.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Appuyez sur la touche ou le bouton de souris à changer.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Vous n'avez appuyé sur aucune touche, donc rien n'a été ajouté.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Écrivez la touche à utiliser à la place, par exemple Escape ou Ctrl+C, ou écrivez Nothing.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle n'a pas pu savoir de quelle touche il s'agissait. Choisissez plutôt la touche dans la liste.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Tous les claviers et souris

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Vous avez aussi modifié cette touche plus bas, et cette modification est utilisée.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Modifier { $key } sur

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Changer { $key } en

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Retirer cette modification

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = La ligne { $n } a été ignorée : { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Impossible de lire les touches modifiées

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Vos touches fonctionnent normalement tant que Trindle ne peut pas lire les modifications. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Choisir la touche ou le bouton à modifier

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Appuyer sur la touche ou le bouton à modifier

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Modifier une touche ou un bouton

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = Vous avez déjà modifié { $key }.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Touches et boutons

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Toute touche ou tout bouton de souris peut faire autre chose. Il peut devenir une autre touche ou un autre bouton, ou des touches appuyées ensemble comme Ctrl+C, ou des touches appuyées l'une après l'autre comme H, I. Il peut aussi ne rien faire, ou faire une chose en appui court et une autre en appui maintenu, par exemple Escape en appui court et Ctrl en appui maintenu. Cela marche partout, dès l'écran de connexion. Ctrl, Alt et une touche F ouvrent toujours une console texte, et en y tapant trindle-settings --keys default, toutes les touches reviennent comme avant.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Échanger Ctrl et Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock : appui court sur Escape, appui maintenu sur Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Échanger Escape et Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Échanger Alt et Super, comme sur un clavier Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Les boutons latéraux de la souris copient et collent

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = Faire confiance à { $name }

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Aucun pour le moment

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Claviers et souris créés par des programmes

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Les outils de remappage et les lanceurs de jeux peuvent créer leur propre clavier ou souris. Leurs touches et clics peuvent toujours écrire et jouer, mais ne peuvent pas appuyer sur les boutons protégés, comme Éteindre ou Autoriser, ni répondre à une question qui vous est destinée. Activez-en un ici pour le laisser compter comme vous.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Manette { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } sur { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = Ce que fait { $title }

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Échanger A et B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Échanger X et Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Inverser le haut et le bas du stick droit

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Raccourcis des modules

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } n'est pas utilisé : { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Touches pour { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Enregistré

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = Pour le pavé tactile aussi.

# Title of the mouse card.
settings-keyboard-mouse-title = Souris

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = La vitesse du pointeur est la distance que parcourt le pointeur quand vous déplacez la souris. Elle s'applique aussi au pavé tactile. La vitesse de défilement est la distance que parcourt une page à chaque tour de molette. Choisissez Plus lent, Normal ou Plus rapide, ou réglez-la n'importe où sur le curseur. Avec le défilement naturel, la page bouge dans le sens où tourne la molette, comme sur un téléphone. Gaucher échange les boutons gauche et droit.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Vitesse du pointeur

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Vitesse de défilement

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Défilement naturel

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Gaucher

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Aide au clic

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Les clics d'un cliqueur à maintien ou à contacteur comptent comme les vôtres.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Pavé tactile

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = La vitesse de défilement est la distance que parcourt une page quand vous faites défiler avec deux doigts. Toucher pour cliquer fait compter un léger toucher comme un clic. Avec le défilement naturel, la page bouge dans le sens de vos doigts, comme sur un téléphone. Désactivé pendant la saisie empêche votre paume de déplacer le pointeur pendant que vous écrivez.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Toucher pour cliquer

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Désactivé pendant la saisie

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Plus lent

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Plus rapide

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = Ports USB

# Words behind the i of that card.
settings-keyboard-usb-about = Si un port USB ne démarre pas, Trindle l'éteint et le rallume une fois pour réessayer. Il ne le fait jamais plus d'une fois par port.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle n'a pas non plus pu annuler la modification : { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Captures d'écran, rapports et raccourcis

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Faire une capture d'écran

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Afficher la liste des raccourcis

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Faire une capture d'écran maintenant

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Afficher la liste maintenant

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Redémarrer le bureau

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Redémarrer l'anneau et les éléments d'aide

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Tout redémarrer

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Touches pour { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Redémarrer maintenant. Les fenêtres ouvertes restent telles quelles.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Tout redémarrer. Demande d'abord confirmation.

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Redémarré

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = Un autre raccourci utilise déjà { $keys }.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Non enregistré. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Signaler un problème

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Signaler un problème maintenant

# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = Non enregistré. Le fichier des touches modifiées n'a pas pu être lu, donc il a été laissé tel quel.
