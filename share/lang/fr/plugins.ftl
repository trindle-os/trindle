
# machine translated (French), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle ne peut pas lire de { $file } à la racine du dossier ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } n'a pas de partie [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Cette version de Trindle prend en charge Interface={ $known }, pas Interface={ $n }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] a besoin de Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] a besoin d'un Id fait de lettres, de chiffres, de points et de tirets, comme org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] a besoin d'un Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] a besoin d'une Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = Le Summary de [Plugin] tient sur une seule ligne.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } doit être du texte simple sur une ligne, { $max } caractère au plus.
       *[other] [{ $group }] { $key } doit être du texte simple sur une ligne, { $max } caractères au plus.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = L'autorisation { $name } n'existe pas. Les autorisations sont { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] a besoin d'une Icon : un fichier SVG ou PNG dans le dossier du module.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = L'Icon de [Ring] désigne { $icon }, mais ce n'est pas dans le dossier du module.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] a besoin d'un Label, affiché quand le pointeur s'arrête dessus.

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] a besoin de Fields, les noms de ses champs.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Le nom de champ { $name } ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Le champ { $name } n'a pas de partie [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] a besoin d'un Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] peut compter { $most } Choices au plus. Chaque choix doit être du texte simple de { $chars } caractère au plus.
       *[other] [{ $group }] peut compter { $most } Choices au plus. Chaque choix doit être du texte simple de { $chars } caractères au plus.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom doit nommer un fichier du dossier du module. Utilisez uniquement des lettres, des chiffres, des points, des tirets et des tirets bas. Ne commencez pas le nom par un point et n'indiquez pas de chemin de dossier.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] est un choix, il a donc besoin de Choices avec deux entrées ou plus, ou de ChoicesFrom avec un fichier.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = Le Default de [{ $group }] ne figure pas parmi ses Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] est une note, elle a donc besoin que TextFrom nomme un fichier du dossier du module. Utilisez uniquement des lettres, des chiffres, des points, des tirets et des tirets bas. Ne commencez pas le nom par un point et n'indiquez pas de chemin de dossier.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] est une note, elle ne stocke donc aucun réglage et ne peut pas avoir de Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = Le Kind de [{ $group }] est { $kind }, mais il doit être switch, text, choice ou note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] est un interrupteur, son Default doit donc être true ou false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] a besoin de l'autorisation background dans Permissions de [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Le module ne se connecte à rien. Donnez-lui [Ring], [Settings], [Background], [Shortcut] ou l'autorisation des notifications.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } est un lien qui sort du dossier du module.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = Le Run de [{ $group }] ne peut pas être lu comme une commande.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = Le Run de [{ $group }] doit désigner un programme dans le dossier du module, pas { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = Le Run de [{ $group }] désigne { $program }, mais le module ne contient pas ce programme (il est absent ou non exécutable).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] Le Menu de [Ring] compte { $n } élément. Le maximum autorisé est { $max }.
       *[other] Le Menu de [Ring] compte { $n } éléments. Le maximum autorisé est { $max }.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = L'ID de menu { $id } ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Le Menu de [Ring] nomme { $id } deux fois.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Le Menu de [Ring] nomme { $id }, qui n'a pas de partie [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] Le Label de [{ $group }] doit être une ligne courte de { $max } caractère au plus.
       *[other] Le Label de [{ $group }] doit être une ligne courte de { $max } caractères au plus.
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] est réservé à Flow du catalogue de Trindle, qui demande l'autorisation control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] a besoin de Run, la commande à qui l'on donne le code PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] Le ShowWhen de [{ $group }] fait { $max } caractère au plus.
       *[other] Le ShowWhen de [{ $group }] fait { $max } caractères au plus.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = Le ShowWhen de [{ $group }] doit s'écrire champ=valeur, par exemple ShowWhen=mode=home. Indiquez un autre champ et la valeur qu'il doit avoir.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = Le ShowWhen de [{ $group }] doit s'écrire champ=valeur, et le nom du champ ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] Le ShowWhen de [{ $group }] a besoin d'une valeur en texte simple sur une ligne, { $max } caractère au plus.
       *[other] Le ShowWhen de [{ $group }] a besoin d'une valeur en texte simple sur une ligne, { $max } caractères au plus.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = Le ShowWhen de [{ $group }] nomme son propre champ. Il doit nommer un autre champ, placé avant.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = Le ShowWhen de [{ $group }] nomme le champ { $other }, qui ne figure pas dans les Fields de ce module.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = Le ShowWhen de [{ $group }] nomme le champ { $other }, qui vient après lui dans Fields. Il doit s'agir d'un champ placé avant.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = Le ShowWhen de [{ $group }] nomme le champ { $other }, qui a lui-même un ShowWhen. Un champ ne peut dépendre que d'un champ qui n'en a pas.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = Le ShowWhen de [{ $group }] nomme l'interrupteur { $other }, sa valeur doit donc être true ou false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = Le ShowWhen de [{ $group }] a la valeur { $value }, qui ne figure pas parmi les Choices de { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = Le ShowWhen de [{ $group }] fait référence à la note { $other }, qui ne stocke aucun réglage. Utilisez plutôt un interrupteur, un choix ou un champ de texte.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen a besoin de l'autorisation network dans Permissions de [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen a besoin de l'autorisation listen dans Permissions de [Plugin].

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = L'autorisation listen a besoin que [Network] Listen indique les ports sur lesquels le module peut recevoir des connexions.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Son plugin.ini utilise une interface que cette version de Trindle ne prend pas en charge.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Il n'indique aucun port à écouter.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Son plugin.ini ne peut pas être lu.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = Il n'a pas reçu l'autorisation d'écouter.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = Les ports indiqués ont changé. Vous devez autoriser de nouveau les connexions sur ces ports.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none =
    { $n ->
        [one] { $name } a besoin de votre autorisation pour fonctionner. Vous n'avez pas accordé la seule autorisation qu'il demande.
       *[other] { $name } a besoin de votre autorisation pour fonctionner. Vous n'avez accordé aucune des { $n } autorisations qu'il demande.
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = Vous n'avez accordé que { $a } des { $n } autorisations demandées par { $name }.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] Il ne demande rien.
        [one] L'autorisation dont il a besoin est accordée.
       *[other] Les { $n } autorisations qu'il demande sont toutes accordées.
    }

# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = Autoriser

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = être joignable sur des ports différents

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Non installé.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = un autre module

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Ce module n'est pas installé.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = Enregistrement impossible : { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Impossible de copier les fichiers du module.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Le module ne tourne pas en arrière-plan : { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle n'a pas trouvé trindle-plugin pour le démarrer.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] elle n'a pas démarré en { $n } seconde.
       *[other] elle n'a pas démarré en { $n } secondes.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = aucune session de bureau n'est ouverte à rejoindre, elle n'a donc pas été démarrée. Ouvrez d'abord une session.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = elle ne sait pas quelle session de bureau rejoindre ({ $sessions }), elle n'a donc pas été démarrée. Relancez-la avec WAYLAND_DISPLAY réglé sur la bonne.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = un dossier se trouve à la place du fichier

# Error text when a plugin's id is not an id.
plugins-not-an-id = ce n'est pas un id valide

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] Shortcuts de [Plugin] compte { $n } raccourci. Le maximum autorisé est { $max }.
       *[other] Shortcuts de [Plugin] compte { $n } raccourcis. Le maximum autorisé est { $max }.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = L'ID de raccourci { $id } ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts de [Plugin] nomme { $id } deux fois.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts de [Plugin] nomme { $id }, qui n'a pas de partie [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Les Keys { $keys } de [{ $group }] sont déjà l'un des raccourcis de Trindle. Choisissez d'autres touches.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Les Keys { $keys } de [{ $group }] sont déjà utilisées par un autre raccourci de ce module.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] a besoin de Keys, comme Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Cette combinaison de touches n'est pas un raccourci valide. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } est déjà l'un des raccourcis de Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } est déjà l'un de vos propres raccourcis.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } est déjà utilisé par { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Ce n'est pas un raccourci d'un module installé.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle n'a pas pu l'utiliser ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle n'a pas pu l'enregistrer, car { $why }. Rien n'a changé.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Enregistrement impossible : { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = Ses ports n'ont pas pu être fermés : { $why }.

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle n'a pas pu ouvrir ses ports, l'écoute est donc de nouveau désactivée : { $why }.

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = Ses ports n'ont pas été ouverts ({ $why }), et l'écoute n'a pas pu être désactivée non plus ({ $kept }). Rien n'est ouvert. Désactivez-la dans Réglages, Permissions.



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = L'autorisation d'accessibilité est réservée au lecteur d'écran de Trindle du catalogue.

# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = L'autorisation { $name } est nommée deux fois.

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = Demandez { $home } ou network, pas les deux : network donne accès au réseau local comme à Internet.

# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }] a besoin de Run, la commande à exécuter.

# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } L'ancienne version n'a pas pu être remise en place non plus : { $back } Les deux sont conservées dans le dossier du module.
