# The plugins code: what a plugin's author or the person is told when a plugin's plugin.ini is refused, when its permissions or ports cannot be put to use, and the words of its shortcuts. Names of parts and keys of plugin.ini, such as [Ring] and Label, stay as they are in every language.

## The check of a plugin.ini (src/plugins/mod.rs)

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = There is no { $file } that can be read at the top of the folder ({ $why }).
# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } has no [Plugin] part.
# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } is not one this Trindle knows: it knows Interface={ $known }.
# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] needs Interface={ $known }.
# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] needs an Id of letters, digits, dots and dashes, such as org.example.Weather.
# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] needs a Name.
# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] needs a Version.
# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary is one line.
# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } is plain words on one line, at most { $max } character.
       *[other] [{ $group }] { $key } is plain words on one line, at most { $max } characters.
    }
# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = Permission { $name } is not one there is: they are { $list }.
# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] needs an Icon: an SVG or PNG file in the plugin's folder.
# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon names { $icon }, which is not in the plugin's folder.
# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] needs a Label, said when the pointer rests on it.
# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] needs Run, what a click starts.
# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] needs Fields, the names of its fields.
# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Field name { $name } may only have letters, digits, dashes and underscores.
# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Field { $name } has no [{ $group }] part.
# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] needs a Label.
# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices are at most { $most }, each plain words of at most { $chars } character.
       *[other] [{ $group }] Choices are at most { $most }, each plain words of at most { $chars } characters.
    }
# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom is a file name in the plugin's home: letters, digits, dots, dashes and underscores, not starting with a dot, no folders.
# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] is a choice, so needs Choices with two or more, or ChoicesFrom a file.
# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default is not one of its Choices.
# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] is a note, so needs TextFrom, a file name in the plugin's home: letters, digits, dots, dashes and underscores, not starting with a dot, no folders.
# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] is a note, which holds no setting, so has no Default.
# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind is { $kind }: it must be switch, text, choice or note.
# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] is a switch, so its Default is true or false.
# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] needs the background permission in [Plugin] Permissions.
# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = The plugin hooks into nothing: give it [Ring], [Settings], [Background], [Shortcut] or the notifications permission.
# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } links outside the plugin's folder.

## A Run line (src/plugins/mod.rs)

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run cannot be read as a command.
# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run must name a program inside the plugin's folder, not { $program }.
# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run names { $program }, which is not a program the plugin carries (missing, or not executable).

## Ring menu items (src/plugins/mod.rs)

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu has { $n } item: at most { $max } are allowed.
       *[other] [Ring] Menu has { $n } items: at most { $max } are allowed.
    }
# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = Menu id { $id } may only have letters, digits, dashes and underscores.
# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu names { $id } twice.
# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu names { $id }, which has no [{ $group }] part.
# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label is one short line of at most { $max } character.
       *[other] [{ $group }] Label is one short line of at most { $max } characters.
    }
# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] needs Run, what choosing it starts.

## The PIN part (src/plugins/mod.rs)

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] is only for Flow from Trindle's catalogue, which asks for the control permission.
# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = The accessibility permission is only for Trindle's screen reader from the catalogue.
# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] needs Run, the command that is given the PIN.

## Settings fields shown only when another field holds a value (src/plugins/mod.rs)

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen is at most { $max } character.
       *[other] [{ $group }] ShowWhen is at most { $max } characters.
    }
# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen is written field=value, such as ShowWhen=mode=home: one other field and the one value it must hold.
# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen is written field=value, and the field's name may only have letters, digits, dashes and underscores.
# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen needs a value of plain words on one line, at most { $max } character.
       *[other] [{ $group }] ShowWhen needs a value of plain words on one line, at most { $max } characters.
    }
# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen names its own field: it must name another, earlier field.
# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen names the field { $other }, which is not in this plugin's Fields.
# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen names the field { $other }, which comes after it in Fields: it must be an earlier field.
# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen names the field { $other }, which has a ShowWhen of its own: a field may only depend on one that has none.
# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen names the switch { $other }, so its value is true or false.
# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen has the value { $value }, which is not one of the Choices of { $other }.
# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen names the note { $other }, which holds no setting: a field may only depend on a switch, a choice or a text.

## Ports a plugin listens on (src/plugins/mod.rs)

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }
# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen needs the network permission in [Plugin] Permissions.
# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen needs the listen permission in [Plugin] Permissions.
# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = The listen permission needs [Network] Listen, the ports to let in.
# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Its plugin.ini is not for an interface this Trindle knows.
# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = It names no ports to listen on.
# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Its plugin.ini could not be read.
# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = It has not been allowed to listen.
# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = The ports it names are not the ones that were allowed. They have to be allowed again.

## Permissions (src/plugins/mod.rs)

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none = { $name } needs your permission to work: 0 of { $n } allowed.
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = { $name } has only some of the permission it asks for: { $a } of { $n } allowed.
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] It asks for nothing.
        [one] The one thing it asks for is allowed.
       *[other] All { $n } things it asks for are allowed.
    }
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = Allow
# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = be reached on other ports than before
# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Not installed.
# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = another plugin
# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = That plugin is not installed.
# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = It could not be saved: { $why }
# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = The plugin's files could not be copied.
# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Its background part is not running: { $not }
# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = trindle-plugin could not be found to start it.
# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] it did not start within { $n } second.
       *[other] it did not start within { $n } seconds.
    }
# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = no desktop session is running for it to join, so it was not started. Log in first.
# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = it cannot tell which desktop session to join ({ $sessions }), so it was not started. Run it again with WAYLAND_DISPLAY set to the right one.
# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = a folder is where the file should be
# Error text when a plugin's id is not an id.
plugins-not-an-id = not a plugin's id

## Keyboard shortcuts of plugins (src/plugins/shortcuts.rs)

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts has { $n } shortcut: at most { $max } are allowed.
       *[other] [Plugin] Shortcuts has { $n } shortcuts: at most { $max } are allowed.
    }
# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = Shortcut id { $id } may only have letters, digits, dashes and underscores.
# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts names { $id } twice.
# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts names { $id }, which has no [{ $group }] part.
# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } is already a shortcut of Trindle's; choose other keys.
# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } is used by another shortcut of the plugin's.
# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] needs Keys, such as Ctrl+Alt+S. { $why }
# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] needs Run, what the keys start.
# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = The keys typed for it are not a shortcut. { $why }
# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } is already a shortcut of Trindle's.
# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } is already one of your own shortcuts.
# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } is already used by { $holder }.
# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = That is not a shortcut of an installed plugin.
# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = It could not be put to use ({ $why }).
# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = It could not be saved: nothing was changed, because { $why }.
# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = It could not be saved ({ $why }).
