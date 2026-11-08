
# machine translated (French), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Aucun { $file } lisible à la racine du dossier ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } n'a pas de partie [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } n'est pas connue de ce Trindle : il connaît Interface={ $known }.

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
plugins-check-permission-unknown = La permission { $name } n'existe pas : les permissions sont { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] a besoin d'une Icon : un fichier SVG ou PNG dans le dossier du plugin.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = L'Icon de [Ring] désigne { $icon }, qui n'est pas dans le dossier du plugin.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] a besoin d'un Label, lu quand le pointeur s'arrête dessus.

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] a besoin de Run, ce qu'un clic lance.

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
        [one] [{ $group }] Choices en compte { $most } au plus, chacun en texte simple de { $chars } caractère au plus.
       *[other] [{ $group }] Choices en compte { $most } au plus, chacun en texte simple de { $chars } caractères au plus.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom est un nom de fichier dans le dossier du plugin : lettres, chiffres, points, tirets et tirets bas, sans point au début, sans dossiers.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] est un choix, il a donc besoin de Choices avec deux entrées ou plus, ou de ChoicesFrom avec un fichier.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = Le Default de [{ $group }] ne figure pas parmi ses Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] est une note, elle a donc besoin de TextFrom, un nom de fichier dans le dossier du plugin : lettres, chiffres, points, tirets et tirets bas, sans point au début, sans dossiers.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] est une note, qui ne contient aucun réglage, donc elle n'a pas de Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = Le Kind de [{ $group }] est { $kind } : il doit être switch, text, choice ou note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] est un interrupteur, son Default est donc true ou false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] a besoin de la permission background dans Permissions de [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Le plugin ne se branche sur rien : donnez-lui [Ring], [Settings], [Background], [Shortcut] ou la permission des notifications.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } est un lien qui sort du dossier du plugin.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = Le Run de [{ $group }] ne peut pas être lu comme une commande.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = Le Run de [{ $group }] doit désigner un programme dans le dossier du plugin, pas { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = Le Run de [{ $group }] désigne { $program }, qui n'est pas un programme fourni par le plugin (absent, ou non exécutable).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] Le Menu de [Ring] compte { $n } élément : { $max } au plus sont permis.
       *[other] Le Menu de [Ring] compte { $n } éléments : { $max } au plus sont permis.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = L'id de menu { $id } ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Le Menu de [Ring] nomme { $id } deux fois.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Le Menu de [Ring] nomme { $id }, qui n'a pas de partie [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] Le Label de [{ $group }] est une ligne courte de { $max } caractère au plus.
       *[other] Le Label de [{ $group }] est une ligne courte de { $max } caractères au plus.
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] a besoin de Run, ce que son choix lance.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] est réservé à Flow du catalogue de Trindle, qui demande la permission control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] a besoin de Run, la commande à qui l'on donne le code PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] Le ShowWhen de [{ $group }] fait { $max } caractère au plus.
       *[other] Le ShowWhen de [{ $group }] fait { $max } caractères au plus.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = Le ShowWhen de [{ $group }] s'écrit champ=valeur, par exemple ShowWhen=mode=home : un autre champ et la seule valeur qu'il doit avoir.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = Le ShowWhen de [{ $group }] s'écrit champ=valeur, et le nom du champ ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] Le ShowWhen de [{ $group }] a besoin d'une valeur en texte simple sur une ligne, { $max } caractère au plus.
       *[other] Le ShowWhen de [{ $group }] a besoin d'une valeur en texte simple sur une ligne, { $max } caractères au plus.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = Le ShowWhen de [{ $group }] nomme son propre champ : il doit nommer un autre champ, placé avant.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = Le ShowWhen de [{ $group }] nomme le champ { $other }, qui ne figure pas dans les Fields de ce plugin.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = Le ShowWhen de [{ $group }] nomme le champ { $other }, qui vient après lui dans Fields : ce doit être un champ placé avant.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = Le ShowWhen de [{ $group }] nomme le champ { $other }, qui a lui-même un ShowWhen : un champ ne peut dépendre que d'un champ qui n'en a pas.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = Le ShowWhen de [{ $group }] nomme l'interrupteur { $other }, sa valeur est donc true ou false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = Le ShowWhen de [{ $group }] a la valeur { $value }, qui ne figure pas parmi les Choices de { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = Le ShowWhen de [{ $group }] nomme la note { $other }, qui ne contient aucun réglage : un champ ne peut dépendre que d'un interrupteur, d'un choix ou d'un texte.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Réseau] Écoute : { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen a besoin de la permission network dans Permissions de [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen a besoin de la permission listen dans Permissions de [Plugin].

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = La permission listen a besoin de [Network] Listen, les ports à laisser entrer.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Son plugin.ini n'est pas pour une interface connue de ce Trindle.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Il ne nomme aucun port à écouter.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Son plugin.ini n'a pas pu être lu.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = Il n'a pas reçu l'autorisation d'écouter.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = Les ports qu'il nomme ne sont pas ceux qui ont été autorisés. Il faut les autoriser de nouveau.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none = { $name } a besoin de votre permission pour fonctionner : 0 sur { $n } autorisée.

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = { $name } n'a reçu qu'une partie des permissions demandées : { $a } sur { $n } autorisées.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] Il ne demande rien.
        [one] La seule chose qu'il demande est autorisée.
       *[other] Les { $n } choses qu'il demande sont toutes autorisées.
    }

# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = Autoriser

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = être joignable sur d'autres ports qu'avant

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Non installé.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = un autre plugin

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Ce plugin n'est pas installé.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = Cela n'a pas pu être enregistré : { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Les fichiers du plugin n'ont pas pu être copiés.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Sa partie en arrière-plan ne tourne pas : { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = trindle-plugin est introuvable pour la démarrer.

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
plugins-not-an-id = ce n'est pas l'id d'un plugin

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] Shortcuts de [Plugin] compte { $n } raccourci : { $max } au plus sont permis.
       *[other] Shortcuts de [Plugin] compte { $n } raccourcis : { $max } au plus sont permis.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = L'id de raccourci { $id } ne peut contenir que des lettres, des chiffres, des tirets et des tirets bas.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts de [Plugin] nomme { $id } deux fois.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts de [Plugin] nomme { $id }, qui n'a pas de partie [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Les Keys { $keys } de [{ $group }] sont déjà un raccourci de Trindle ; choisissez d'autres touches.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Les Keys { $keys } de [{ $group }] sont utilisées par un autre raccourci du plugin.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] a besoin de Keys, comme Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] a besoin de Run, ce que les touches lancent.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Les touches saisies ne forment pas un raccourci. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } est déjà un raccourci de Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } est déjà l'un de vos propres raccourcis.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } est déjà utilisé par { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Ce n'est pas un raccourci d'un plugin installé.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Il n'a pas pu être mis en service ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Cela n'a pas pu être enregistré : rien n'a été modifié, car { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Cela n'a pas pu être enregistré ({ $why }).



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = L'autorisation d'accessibilité est réservée au lecteur d'écran de Trindle du catalogue.

