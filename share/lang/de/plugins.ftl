
# machine translated (German), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Im obersten Ordner gibt es keine lesbare { $file } ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } hat keinen Abschnitt [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } kennt dieses Trindle nicht: Es kennt Interface={ $known }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] braucht Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] braucht eine Id aus Buchstaben, Ziffern, Punkten und Bindestrichen, zum Beispiel org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] braucht einen Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] braucht eine Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary besteht aus einer Zeile.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } besteht aus einfachen Wörtern in einer Zeile, höchstens { $max } Zeichen.
       *[other] [{ $group }] { $key } besteht aus einfachen Wörtern in einer Zeile, höchstens { $max } Zeichen.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = Die Berechtigung { $name } gibt es nicht: Es gibt { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] braucht ein Icon: eine SVG- oder PNG-Datei im Ordner des Plugins.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon nennt { $icon }, das nicht im Ordner des Plugins liegt.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] braucht ein Label, das erscheint, wenn der Zeiger darauf ruht.

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] braucht Run, das, was ein Klick startet.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] braucht Fields, die Namen seiner Felder.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Der Feldname { $name } darf nur Buchstaben, Ziffern, Bindestriche und Unterstriche enthalten.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Das Feld { $name } hat keinen Abschnitt [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] braucht ein Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices sind höchstens { $most }, jede aus einfachen Wörtern mit höchstens { $chars } Zeichen.
       *[other] [{ $group }] Choices sind höchstens { $most }, jede aus einfachen Wörtern mit höchstens { $chars } Zeichen.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom ist ein Dateiname im Home-Ordner des Plugins: Buchstaben, Ziffern, Punkte, Bindestriche und Unterstriche, kein Punkt am Anfang, keine Ordner.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] ist eine Auswahl und braucht daher Choices mit zwei oder mehr Einträgen oder ChoicesFrom mit einer Datei.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default ist keine seiner Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] ist ein Hinweis und braucht daher TextFrom, einen Dateinamen im Home-Ordner des Plugins: Buchstaben, Ziffern, Punkte, Bindestriche und Unterstriche, kein Punkt am Anfang, keine Ordner.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] ist ein Hinweis, der keine Einstellung enthält, und hat daher kein Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind ist { $kind }: Es muss switch, text, choice oder note sein.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] ist ein Schalter, daher ist sein Default true oder false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] braucht die Berechtigung background in [Plugin] Permissions.

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Das Plugin hängt sich an nichts: Gib ihm [Ring], [Settings], [Background], [Shortcut] oder die Berechtigung für Benachrichtigungen.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } verweist aus dem Ordner des Plugins hinaus.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run lässt sich nicht als Befehl lesen.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run muss ein Programm im Ordner des Plugins nennen, nicht { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run nennt { $program }, das kein Programm des Plugins ist (fehlt oder ist nicht ausführbar).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu hat { $n } Eintrag: Höchstens { $max } sind erlaubt.
       *[other] [Ring] Menu hat { $n } Einträge: Höchstens { $max } sind erlaubt.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Die Menü-ID { $id } darf nur Buchstaben, Ziffern, Bindestriche und Unterstriche enthalten.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu nennt { $id } zweimal.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu nennt { $id }, das keinen Abschnitt [{ $group }] hat.

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label ist eine kurze Zeile mit höchstens { $max } Zeichen.
       *[other] [{ $group }] Label ist eine kurze Zeile mit höchstens { $max } Zeichen.
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] braucht Run, das, was die Auswahl startet.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] gibt es nur für Flow aus dem Katalog von Trindle, das die Berechtigung control anfordert.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] braucht Run, den Befehl, der die PIN erhält.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen hat höchstens { $max } Zeichen.
       *[other] [{ $group }] ShowWhen hat höchstens { $max } Zeichen.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen wird als field=value geschrieben, zum Beispiel ShowWhen=mode=home: ein anderes Feld und der eine Wert, den es haben muss.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen wird als field=value geschrieben, und der Name des Feldes darf nur Buchstaben, Ziffern, Bindestriche und Unterstriche enthalten.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen braucht einen Wert aus einfachen Wörtern in einer Zeile, höchstens { $max } Zeichen.
       *[other] [{ $group }] ShowWhen braucht einen Wert aus einfachen Wörtern in einer Zeile, höchstens { $max } Zeichen.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen nennt sein eigenes Feld: Es muss ein anderes, früheres Feld nennen.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen nennt das Feld { $other }, das nicht in den Fields dieses Plugins steht.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen nennt das Feld { $other }, das in Fields danach kommt: Es muss ein früheres Feld sein.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen nennt das Feld { $other }, das selbst ein ShowWhen hat: Ein Feld darf nur von einem abhängen, das keines hat.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen nennt den Schalter { $other }, daher ist sein Wert true oder false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen hat den Wert { $value }, der keine der Choices von { $other } ist.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen nennt den Hinweis { $other }, der keine Einstellung enthält: Ein Feld darf nur von einem Schalter, einer Auswahl oder einem Text abhängen.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Netzwerk] Lauschende Ports: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen braucht die Berechtigung network in [Plugin] Permissions.

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen braucht die Berechtigung listen in [Plugin] Permissions.

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = Die Berechtigung listen braucht [Network] Listen, die Ports, die hereingelassen werden.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Seine plugin.ini ist nicht für eine Schnittstelle, die dieses Trindle kennt.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Es nennt keine Ports, auf denen es lauschen soll.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Seine plugin.ini konnte nicht gelesen werden.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Es darf nicht lauschen.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = Die Ports, die es nennt, sind nicht die erlaubten. Sie müssen erneut erlaubt werden.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name } braucht deine Erlaubnis, um zu arbeiten: 0 von { $n } erlaubt.

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } hat nur einen Teil der angeforderten Erlaubnis: { $a } von { $n } erlaubt.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Es fordert nichts an.
        [one] Das eine, was es anfordert, ist erlaubt.
       *[other] Alle { $n } angeforderten Dinge sind erlaubt.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = Erlauben

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = auf anderen Ports als bisher erreichbar sein

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Nicht installiert.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = ein anderes Plugin

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Dieses Plugin ist nicht installiert.

# Red words when a permission could not be kept. { $why } is the reason.
# SAFETY: needs a human review before it ships
plugins-not-saved = Es konnte nicht gespeichert werden: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Die Dateien des Plugins konnten nicht kopiert werden.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Sein Hintergrundteil läuft nicht: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = trindle-plugin wurde nicht gefunden, um es zu starten.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] es wurde nicht innerhalb von { $n } Sekunde gestartet.
       *[other] es wurde nicht innerhalb von { $n } Sekunden gestartet.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = Es läuft keine Desktop-Sitzung, der es beitreten könnte, daher wurde es nicht gestartet. Melde dich zuerst an.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = es kann nicht erkennen, welcher Desktop-Sitzung es beitreten soll ({ $sessions }), daher wurde es nicht gestartet. Führe es erneut aus, mit WAYLAND_DISPLAY auf die richtige gesetzt.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = dort, wo die Datei sein sollte, liegt ein Ordner

# Error text when a plugin's id is not an id.
plugins-not-an-id = keine Plugin-ID

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts hat { $n } Tastenkürzel: Höchstens { $max } sind erlaubt.
       *[other] [Plugin] Shortcuts hat { $n } Tastenkürzel: Höchstens { $max } sind erlaubt.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Die Tastenkürzel-ID { $id } darf nur Buchstaben, Ziffern, Bindestriche und Unterstriche enthalten.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts nennt { $id } zweimal.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts nennt { $id }, das keinen Abschnitt [{ $group }] hat.

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } ist bereits ein Tastenkürzel von Trindle; wähle andere Tasten.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } wird von einem anderen Tastenkürzel des Plugins verwendet.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] braucht Keys, zum Beispiel Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] braucht Run, das, was die Tasten starten.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Die eingegebenen Tasten sind kein Tastenkürzel. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } ist bereits ein Tastenkürzel von Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } ist bereits eines deiner eigenen Tastenkürzel.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } wird bereits von { $holder } verwendet.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Das ist kein Tastenkürzel eines installierten Plugins.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Es konnte nicht in Gebrauch genommen werden ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Es konnte nicht gespeichert werden: Nichts wurde geändert, weil { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Es konnte nicht gespeichert werden ({ $why }).


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Die Berechtigung für Barrierefreiheit gilt nur für den Trindle-Screenreader aus dem Katalog.

