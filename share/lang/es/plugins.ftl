
# machine translated (Spanish), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle no puede leer un { $file } en la raíz de la carpeta ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } no tiene la sección [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Esta versión de Trindle admite Interface={ $known }, no Interface={ $n }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] necesita Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] necesita un Id de letras, dígitos, puntos y guiones, como org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] necesita un Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] necesita una Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = El Summary de [Plugin] es de una sola línea.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } debe ser texto sencillo en una línea, de { $max } carácter como máximo.
       *[other] [{ $group }] { $key } debe ser texto sencillo en una línea, de { $max } caracteres como máximo.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = El permiso { $name } no existe. Los permisos son { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] necesita un Icon: un archivo SVG o PNG en la carpeta del complemento.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = El Icon de [Ring] indica { $icon }, pero no está en la carpeta del complemento.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] necesita un Label, que se muestra al dejar el puntero encima.

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] necesita Fields, los nombres de sus campos.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = El nombre de campo { $name } solo puede contener letras, números, guiones y guiones bajos.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = El campo { $name } no tiene la sección [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] necesita un Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] puede tener como máximo { $most } Choices. Cada una debe ser texto sencillo de { $chars } carácter como máximo.
       *[other] [{ $group }] puede tener como máximo { $most } Choices. Cada una debe ser texto sencillo de { $chars } caracteres como máximo.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom debe indicar un archivo de la carpeta del complemento. Usa solo letras, dígitos, puntos, guiones y guiones bajos. No empieces el nombre con un punto ni incluyas una ruta de carpetas.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] es una opción, así que necesita Choices con dos o más opciones, o ChoicesFrom, un archivo.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = El Default de [{ $group }] no es una de sus Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] es una nota, así que necesita TextFrom para indicar un archivo de la carpeta del complemento. Usa solo letras, dígitos, puntos, guiones y guiones bajos. No empieces el nombre con un punto ni incluyas una ruta de carpetas.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] es una nota, así que no guarda ningún ajuste y no puede tener Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = El Kind de [{ $group }] es { $kind }, pero debe ser switch, text, choice o note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] es un interruptor, así que su Default debe ser true o false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] necesita el permiso background en Permissions de [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = El complemento no se conecta a nada. Dale [Ring], [Settings], [Background], [Shortcut] o el permiso de notificaciones.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } es un enlace que lleva fuera de la carpeta del complemento.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = El Run de [{ $group }] no se puede leer como un comando.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = El Run de [{ $group }] debe indicar un programa dentro de la carpeta del complemento, no { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = El Run de [{ $group }] indica { $program }, pero el complemento no incluye ese programa (falta o no es ejecutable).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] El Menu de [Ring] tiene { $n } elemento. Se permiten { $max } como máximo.
       *[other] El Menu de [Ring] tiene { $n } elementos. Se permiten { $max } como máximo.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = El ID de menú { $id } solo puede contener letras, números, guiones y guiones bajos.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = El Menu de [Ring] nombra { $id } dos veces.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = El Menu de [Ring] nombra { $id }, que no tiene la sección [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] El Label de [{ $group }] debe ser una línea corta de { $max } carácter como máximo.
       *[other] El Label de [{ $group }] debe ser una línea corta de { $max } caracteres como máximo.
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] es solo para Flow del catálogo de Trindle, que pide el permiso control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] necesita Run, el comando que recibe el PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] El ShowWhen de [{ $group }] puede tener { $max } carácter como máximo.
       *[other] El ShowWhen de [{ $group }] puede tener { $max } caracteres como máximo.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = El ShowWhen de [{ $group }] debe usar el formato campo=valor, como ShowWhen=mode=home. Indica otro campo y el valor que debe tener.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = El ShowWhen de [{ $group }] debe escribirse como campo=valor, y el nombre del campo solo puede tener letras, dígitos, guiones y guiones bajos.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] El ShowWhen de [{ $group }] necesita un valor de texto sencillo en una línea, de { $max } carácter como máximo.
       *[other] El ShowWhen de [{ $group }] necesita un valor de texto sencillo en una línea, de { $max } caracteres como máximo.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = El ShowWhen de [{ $group }] nombra su propio campo. Debe nombrar otro campo anterior.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = El ShowWhen de [{ $group }] nombra el campo { $other }, que no está en los Fields de este complemento.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = El ShowWhen de [{ $group }] nombra el campo { $other }, que viene después en Fields. Debe ser un campo anterior.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = El ShowWhen de [{ $group }] nombra el campo { $other }, que tiene su propio ShowWhen. Un campo solo puede depender de uno que no tenga.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = El ShowWhen de [{ $group }] nombra el interruptor { $other }, así que su valor debe ser true o false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = El ShowWhen de [{ $group }] tiene el valor { $value }, que no es una de las Choices de { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = El ShowWhen de [{ $group }] se refiere a la nota { $other }, que no guarda ningún ajuste. Usa en su lugar un interruptor, una opción o un campo de texto.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen necesita el permiso network en Permissions de [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen necesita el permiso listen en Permissions de [Plugin].

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = El permiso listen necesita que [Network] Listen indique los puertos en los que el complemento puede recibir conexiones.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Su plugin.ini usa una interfaz que esta versión de Trindle no admite.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = No indica ningún puerto en el que escuchar.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = No se puede leer su plugin.ini.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = No se le ha permitido escuchar.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = Los puertos indicados han cambiado. Tienes que permitir las conexiones en ellos de nuevo.

# SAFETY: needs a human review before it ships
# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none =
    { $n ->
        [one] { $name } necesita tu permiso antes de poder funcionar. No has permitido el único permiso que pide.
       *[other] { $name } necesita tu permiso antes de poder funcionar. No has permitido ninguno de los { $n } permisos que pide.
    }

# SAFETY: needs a human review before it ships
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = Has permitido solo { $a } de los { $n } permisos que pide { $name }.

# SAFETY: needs a human review before it ships
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] No pide nada.
        [one] El permiso que necesita está permitido.
       *[other] Los { $n } permisos que pide están permitidos.
    }

# SAFETY: needs a human review before it ships
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = Permitir

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = que se pueda acceder a él por puertos distintos

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = No instalado.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = otro complemento

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Ese complemento no está instalado.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = No se pudo guardar: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = No se pudieron copiar los archivos del complemento.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } El complemento no se está ejecutando en segundo plano: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle no encontró trindle-plugin para iniciarla.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] no se inició en { $n } segundo.
       *[other] no se inició en { $n } segundos.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = no hay ninguna sesión de escritorio en ejecución a la que unirse, así que no se inició. Inicia sesión primero.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = no puede saber a qué sesión de escritorio unirse ({ $sessions }), así que no se inició. Ejecútala de nuevo con WAYLAND_DISPLAY puesto en la correcta.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = hay una carpeta donde debería estar el archivo

# Error text when a plugin's id is not an id.
plugins-not-an-id = no es un id válido

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] Shortcuts de [Plugin] tiene { $n } atajo. Se permiten { $max } como máximo.
       *[other] Shortcuts de [Plugin] tiene { $n } atajos. Se permiten { $max } como máximo.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = El ID de atajo { $id } solo puede contener letras, números, guiones y guiones bajos.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts de [Plugin] nombra { $id } dos veces.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts de [Plugin] nombra { $id }, que no tiene la sección [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Las Keys { $keys } de [{ $group }] ya son un atajo de Trindle. Elige otras teclas.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Las Keys { $keys } de [{ $group }] ya las usa otro atajo de este complemento.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] necesita Keys, como Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Esa combinación de teclas no es un atajo válido. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } ya es uno de los atajos de Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } ya es uno de tus propios atajos.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } ya lo usa { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Ese no es un atajo de un complemento instalado.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle no pudo usarlo ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = No se pudo guardar: no se cambió nada, porque { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = No se pudo guardar: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = No se pudieron cerrar sus puertos: { $why }.

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle no pudo abrir sus puertos, así que la escucha vuelve a estar desactivada: { $why }.

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = Sus puertos no se abrieron ({ $why }), y tampoco se pudo desactivar la escucha ({ $kept }). No hay nada abierto. Desactívala en Ajustes, Permisos.



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Solo el lector de pantalla de Trindle del catálogo puede usar el permiso de accesibilidad.

# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = El permiso { $name } aparece dos veces.

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = Pide { $home } o network, no ambos: network llega a la red local además de a internet.

# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }] necesita Run, el comando que se ejecuta.

# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } Tampoco se pudo volver a poner la versión anterior: { $back } Las dos se conservan en la carpeta del complemento.
