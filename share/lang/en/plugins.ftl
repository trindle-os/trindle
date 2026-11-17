# The plugins code: what a plugin's author or the person is told when a plugin's plugin.ini is refused, when its permissions or ports cannot be put to use, and the words of its shortcuts. Names of parts and keys of plugin.ini, such as [Ring] and Label, stay as they are in every language.

## The check of a plugin.ini (src/plugins/mod.rs)

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle can't read a { $file } at the top of the folder ({ $why }).
# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } has no [Plugin] part.
# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = This version of Trindle supports Interface={ $known }, not Interface={ $n }.
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
        [one] [{ $group }] { $key } must be plain text on one line, with no more than { $max } character.
       *[other] [{ $group }] { $key } must be plain text on one line, with no more than { $max } characters.
    }
# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = Permission { $name } is named twice.
# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = Ask for { $home } or network, not both: network reaches the local network as well as the internet.
# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = Permission { $name } doesn't exist. The permissions are { $list }.
# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] needs an Icon: an SVG or PNG file in the plugin's folder.
# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon names { $icon }, but that isn't in the plugin's folder.
# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] needs a Label, shown when the pointer rests on it.
# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }] needs Run, the command to run.
# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] needs Fields, the names of its fields.
# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = The field name { $name } can only contain letters, numbers, hyphens and underscores.
# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Field { $name } has no [{ $group }] part.
# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] needs a Label.
# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] can have at most { $most } Choices. Each must be plain text with no more than { $chars } character.
       *[other] [{ $group }] can have at most { $most } Choices. Each must be plain text with no more than { $chars } characters.
    }
# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom must name a file in the plugin's folder. Use only letters, digits, dots, dashes and underscores. Don't start the name with a dot or include a folder path.
# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] is a choice, so it needs Choices with two or more options, or ChoicesFrom, a file.
# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default isn't one of its Choices.
# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] is a note, so it needs TextFrom to name a file in the plugin's folder. Use only letters, digits, dots, dashes and underscores. Don't start the name with a dot or include a folder path.
# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] is a note, so it doesn't store a setting and cannot have a Default.
# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind is { $kind }, but it must be switch, text, choice or note.
# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] is a switch, so its Default must be true or false.
# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] needs the background permission in [Plugin] Permissions.
# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = The plugin doesn't connect to anything. Give it [Ring], [Settings], [Background], [Shortcut] or the notifications permission.
# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } is a link that leads outside the plugin's folder.

## A Run line (src/plugins/mod.rs)

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run can't be read as a command.
# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run must name a program inside the plugin's folder, not { $program }.
# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run names { $program }, but the plugin doesn't contain that program (it is missing or not executable).

## Ring menu items (src/plugins/mod.rs)

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu has { $n } item. At most { $max } are allowed.
       *[other] [Ring] Menu has { $n } items. At most { $max } are allowed.
    }
# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = The menu ID { $id } can only contain letters, numbers, hyphens and underscores.
# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu names { $id } twice.
# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu names { $id }, which has no [{ $group }] part.
# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label must be one short line of at most { $max } character.
       *[other] [{ $group }] Label must be one short line of at most { $max } characters.
    }

## The PIN part (src/plugins/mod.rs)

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] is only for Flow from Trindle's catalogue, which asks for the control permission.
# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Only Trindle's screen reader from the catalogue can use the accessibility permission.
# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] needs Run, the command that is given the PIN.

## Settings fields shown only when another field holds a value (src/plugins/mod.rs)

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen can be at most { $max } character.
       *[other] [{ $group }] ShowWhen can be at most { $max } characters.
    }
# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen must use field=value, such as ShowWhen=mode=home. Specify another field and the value it must have.
# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen must be written field=value, and the field's name can only have letters, digits, dashes and underscores.
# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen needs a plain text value on one line, with no more than { $max } character.
       *[other] [{ $group }] ShowWhen needs a plain text value on one line, with no more than { $max } characters.
    }
# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen names its own field. It must name another, earlier field.
# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen names the field { $other }, which is not in this plugin's Fields.
# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen names the field { $other }, which comes after it in Fields. It must be an earlier field.
# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen names the field { $other }, which has a ShowWhen of its own. A field can only depend on one that has none.
# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen names the switch { $other }, so its value must be true or false.
# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen has the value { $value }, which is not one of the Choices of { $other }.
# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen refers to the note { $other }, which doesn't store a setting. Use a switch, choice or text field instead.

## Ports a plugin listens on (src/plugins/mod.rs)

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }
# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen needs the network permission in [Plugin] Permissions.
# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen needs the listen permission in [Plugin] Permissions.
# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = The listen permission needs [Network] Listen to list the ports the plugin can receive connections on.
# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Its plugin.ini uses an interface that this version of Trindle doesn't support.
# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = It lists no ports to listen on.
# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Its plugin.ini can't be read.
# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = It hasn't been allowed to listen.
# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = The listed ports have changed. You need to allow connections on them again.

## Permissions (src/plugins/mod.rs)

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none =
    { $n ->
        [one] { $name } needs your permission before it can work. You haven't allowed the one permission it asks for.
       *[other] { $name } needs your permission before it can work. You've allowed none of the { $n } permissions it asks for.
    }
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = You've allowed only { $a } of the { $n } permissions { $name } asks for.
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] It asks for nothing.
        [one] The permission it needs is allowed.
       *[other] All { $n } permissions it asks for are allowed.
    }
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = Allow
# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = being reached on different ports
# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Not installed.
# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = another plugin
# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = That plugin is not installed.
# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = Couldn't save it: { $why }
# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Couldn't copy the plugin's files.
# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } The plugin isn't running in the background: { $not }
# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } The old version couldn't be put back either: { $back } Both are kept in the plugin's folder.
# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle couldn't find trindle-plugin to start it.
# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] it did not start within { $n } second.
       *[other] it did not start within { $n } seconds.
    }
# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = no desktop session is running for it to join, so it wasn't started. Log in first.
# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = it can't tell which desktop session to join ({ $sessions }), so it wasn't started. Run it again with WAYLAND_DISPLAY set to the right one.
# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = a folder is where the file should be
# Error text when a plugin's id is not an id.
plugins-not-an-id = not a valid ID

## Keyboard shortcuts of plugins (src/plugins/shortcuts.rs)

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts has { $n } shortcut. At most { $max } are allowed.
       *[other] [Plugin] Shortcuts has { $n } shortcuts. At most { $max } are allowed.
    }
# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = The shortcut ID { $id } can only contain letters, numbers, hyphens and underscores.
# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts names { $id } twice.
# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts names { $id }, which has no [{ $group }] part.
# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } is already one of Trindle's shortcuts. Choose other keys.
# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } is already used by another shortcut in this plugin.
# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] needs Keys, such as Ctrl+Alt+S. { $why }
# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = That key combination isn't a valid shortcut. { $why }
# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } is already one of Trindle's shortcuts.
# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } is already one of your own shortcuts.
# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } is already used by { $holder }.
# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = That is not a shortcut of an installed plugin.
# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle couldn't use it ({ $why }).
# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle couldn't save it because { $why }. Nothing has changed.
# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Couldn't save it: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = Its ports couldn't be closed: { $why }.
# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle couldn't open its ports, so listening is switched off again: { $why }.
# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = Its ports weren't opened ({ $why }), and listening couldn't be switched off again either ({ $kept }). Nothing is open. Switch it off in Settings, Permissions.
