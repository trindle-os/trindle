
# machine translated (Spanish), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Unidades

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Acerca de esta página

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Todas las unidades dentro del equipo o conectadas a él, dibujadas como una barra de particiones. Cada partición es tan ancha como grande es y se rellena según lo que se usa, así que una partición llena se ve enseguida.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Bajo la imagen, cada unidad tiene una tarjeta. Abre una partición, móntala o desmóntala, o saca una unidad con seguridad o apágala. El lápiz de una partición despliega sus cambios: su nombre, una carpeta propia y si está lista al arrancar, su tamaño, comprobarla y repararla, formatearla o eliminarla. El signo más en el espacio libre crea una partición nueva.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = La unidad desde la que funciona el sistema se muestra, pero nunca se ofrece ningún cambio que borre o mueva cosas. Cada cambio en cualquier unidad pide una contraseña de administrador, cada vez. El ojo oculta una unidad o una partición de Carpetas y del anillo: sigue aquí, en gris, y no está bloqueada.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Cada partición de cada unidad, rellena según lo que se usa.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Tus unidades

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = No se pueden leer las unidades en este momento.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Volver a intentar

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Buscar de nuevo

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = No se encontró ninguna unidad. Conecta una y aparecerá aquí.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = El servicio de unidades

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = El bus del sistema

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } no está disponible. Instálalo con: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Mirando las unidades…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } está fallando. Copia ya lo que quieras conservar.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } necesita una revisión: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Unidades y el espacio que hay en ellas.
        [one] Unidades y el espacio que hay en ellas, y una unidad conectada.
       *[other] Unidades y el espacio que hay en ellas, y { $plugged } unidades conectadas.
       [many] Unidades y el espacio que hay en ellas, y { $plugged } unidades conectadas.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Casi lleno: solo quedan { $free } libres en este equipo. Elimina o mueve algunos archivos pronto.
        [one] Casi lleno: solo quedan { $free } libres en este equipo, y una unidad conectada. Elimina o mueve algunos archivos pronto.
       *[other] Casi lleno: solo quedan { $free } libres en este equipo, y { $plugged } unidades conectadas. Elimina o mueve algunos archivos pronto.
       [many] Casi lleno: solo quedan { $free } libres en este equipo, y { $plugged } unidades conectadas. Elimina o mueve algunos archivos pronto.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Se va llenando: { $free } libres en este equipo.
        [one] Se va llenando: { $free } libres en este equipo, y una unidad conectada.
       *[other] Se va llenando: { $free } libres en este equipo, y { $plugged } unidades conectadas.
       [many] Se va llenando: { $free } libres en este equipo, y { $plugged } unidades conectadas.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Hay espacio de sobra: { $free } libres en este equipo.
        [one] Hay espacio de sobra: { $free } libres en este equipo, y una unidad conectada.
       *[other] Hay espacio de sobra: { $free } libres en este equipo, y { $plugged } unidades conectadas.
       [many] Hay espacio de sobra: { $free } libres en este equipo, y { $plugged } unidades conectadas.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Este equipo

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Tus archivos

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] Aún quedan { $free } libres donde está el sistema.
       *[files] Aún quedan { $free } libres para tus propios archivos.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } llenos, { $free } libres de { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = de { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Sin lecturas, las unidades no indican cómo están

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Sana, según la propia unidad

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
        [one] Sanas, según indican las { $n } unidades
       *[other] Sanas, según indican las { $n } unidades
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
        [one] { $n } necesita una revisión, mira su tarjeta
       *[other] { $n } necesitan una revisión, mira sus tarjetas
       [many] { $n } necesitan una revisión, mira sus tarjetas
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
        [one] 1 unidad, { $total } en total. { $health }.
       *[other] { $n } unidades, { $total } en total. { $health }.
       [many] { $n } unidades, { $total } en total. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } libres

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Sin montar

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Espacio libre

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = No hay disco ni tarjeta dentro

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Aún no hay nada

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB o extraíble

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, dentro de este equipo

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB o extraíble, oculta

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, dentro de este equipo, oculta

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } llenos, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Partición

# Name given to a drive that gives no maker or model.
disks-drive-name = Unidad

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Intercambio

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Cifrada, desbloqueada

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Cifrada, bloqueada

# Kind of a part that holds other parts.
disks-kind-container = Contiene otras particiones

# Kind of a part that has no filesystem on it.
disks-kind-none = Sin sistema de archivos

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, la partición desde la que arranca un equipo
        [legacy] { $kind }, la partición desde la que arranca un equipo antiguo
       *[loader] { $kind }, la partición propia de un gestor de arranque
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = cualquier equipo

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = Solo Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = dispositivos antiguos

# Trouble a drive reports about its spare space.
disks-trouble-spare = se está quedando sin espacio de reserva

# Trouble a drive reports about its heat.
disks-trouble-temperature = demasiado caliente

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = se está desgastando

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = se ha puesto en solo lectura

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = ha fallado su copia de seguridad de memoria

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = parte de ella se ha puesto en solo lectura

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = desgastada más allá de lo que espera su fabricante

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
        [one] { $n } zona dañada
       *[other] { $n } zonas dañadas
       [many] { $n } zonas dañadas
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
        [one] usada durante { $hours } hora
       *[other] usada durante { $hours } horas
       [many] usada durante { $hours } horas
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = Fallando: copia ya lo que quieras conservar

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Necesita atención: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Sana

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Ninguna unidad responde

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = La carcasa responde, pero no la unidad de dentro. Desconéctala, espera diez segundos y conéctala a un puerto azul o USB-C con su propio cable; si sigue igual, puede que la unidad de dentro se haya soltado o fallado.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Quitar con seguridad

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Apagar

# SAFETY: needs a human review before it ships
# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
disks-erase-whole = Borrar toda la unidad

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Mostrar esta unidad
       *[part] Mostrar esta partición
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] Ocultar esta unidad. { $lock }
       *[part] Ocultar esta partición. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Oculta de Carpetas y del anillo. { $lock }

# Note on the card of the drive the system runs from.
disks-system-note = El sistema funciona desde ella: aquí no se puede borrar nada.

# Note on the card of a drive that has no partition table.
disks-no-table = Sin tabla de particiones: toda la unidad está libre.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = sin montar

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = se monta al arrancar en { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = aparece en { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = en { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Abrir

# Tip of the round button that mounts a part.
disks-mount = Montar

# Tip of the round button that unmounts a part.
disks-unmount = Desmontar

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Cambiar

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Nombre

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Nombre (opcional)

# Tip of the button that saves a part's new name.
disks-rename-tip = Cambiar el nombre

# Note under the Name row of a part.
disks-name-note = Cómo se llama aquí y en otros equipos.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Nombre de la carpeta

# Tip of the button that saves a part's folder.
disks-use-folder = Usar esta carpeta

# Label of the row where a part's folder is chosen.
disks-folder = Carpeta

# Note under the Folder row of a part.
disks-folder-note = Dónde aparecen sus archivos. Déjalo vacío para que el equipo elija.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Montar al arrancar

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Lista siempre que el equipo arranca. Si no está, el equipo arranca igualmente.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Tamaño en GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = De { $least } a { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = De { $least } a { $most }. Móntala para ver cuánto se puede reducir.

# Tip of the button that resizes a part.
disks-resize-tip = Cambiar el tamaño

# Label of the row where a part is resized.
disks-size = Tamaño

# SAFETY: needs a human review before it ships
# Said above the Format and Erase buttons at all times. { $words } names what is erased.
disks-format-erases = Se borrará todo lo que hay en { $words }. No se puede deshacer.

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning = Haz clic otra vez: { $name } en { $drive } pasa de { $from } a { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Hacer sitio para una partición nueva

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Sitio en GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Hacer sitio

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
disks-room-warning = Haz clic otra vez: { $name } en { $drive } pierde { $size }, tras una instantánea

# SAFETY: needs a human review before it ships
# Note under the row that makes room.
disks-room-note = Se reduce mientras está en uso, tras una instantánea, y deja espacio libre.

# Tip of the button that checks a part for faults.
disks-check-tip = Buscar errores

# SAFETY: needs a human review before it ships
# Tip of the button that repairs a part.
disks-repair-tip = Repararla

# Label of the row where a part is checked and repaired.
disks-faults = Errores

# Note under the Faults row.
disks-faults-note = Busca errores en cómo se guardan sus archivos y los corrige. Se desmonta mientras esto se ejecuta.

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
disks-repair-warning = Haz clic otra vez para reparar { $name }, en { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Tipo

# SAFETY: needs a human review before it ships
# Label of the row where a part is formatted.
disks-format = Formatear

# SAFETY: needs a human review before it ships
# Tip of the button that formats a part.
disks-format-tip = Formatearla

# SAFETY: needs a human review before it ships
# Note under the row that erases the whole drive.
disks-format-whole = La empieza de cero como una sola partición, que ocupa toda la unidad.

# SAFETY: needs a human review before it ships
# Note under the row that formats one part.
disks-format-part = Borra todo lo que hay solo en esta partición.

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the format or erase button. { $words } names what is erased.
disks-format-warning = Haz clic otra vez: se borra todo lo que hay en { $words }

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, en { $drive }

# SAFETY: needs a human review before it ships
# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-delete-name = Eliminar { $name }, { $size }, en { $drive }

# SAFETY: needs a human review before it ships
# Label of the row where a part is deleted.
disks-delete-label = Eliminar esta partición

# SAFETY: needs a human review before it ships
# Note under the row that deletes a part.
disks-delete-note = Su espacio queda libre, listo para una partición nueva o para que crezca la anterior.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Crear aquí una partición nueva

# Label of the row where a new part is made.
disks-new-part = Partición nueva

# Note under the row where a new part is made.
disks-new-part-note = Se crea en este espacio libre y se formatea para poder usarla.

# Tip of the button that makes a new part.
disks-make-tip = Crearla

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = Haz clic otra vez para crear una partición de { $size } en { $drive }

# SAFETY: needs a human review before it ships
# Title of the box for erasing a whole drive. { $name } is the drive.
disks-sheet-erase = Borrar { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, en { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Una partición nueva

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $size } libres, en { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Cerrar

# Words on a row while a part is being mounted.
disks-mounting = Montando

# Words on a row when a part has been mounted.
disks-mounted = Montada

# Words on a row while a part is being unmounted.
disks-unmounting = Desmontando

# Words on a row when a part has been unmounted.
disks-unmounted = Desmontada

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Cerrándola

# Words on a drive's row when it is safe to remove.
disks-safe-remove = Se puede quitar con seguridad

# Words on a drive's row while it is being powered off.
disks-powering-off = Apagando

# Words on a drive's row when it has been powered off.
disks-off-safe = Apagada: se puede desconectar

# Words on a row while a part is being renamed.
disks-renaming = Cambiando el nombre

# Words on a row when a part has been renamed.
disks-renamed = Nombre cambiado

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = El equipo vuelve a elegir su carpeta

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Aparece en { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Se monta al arrancar en { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Se deja como está al arrancar

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Guardando

# Words on a row while a part is being resized.
disks-resizing = Cambiando el tamaño

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Tamaño cambiado a { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = No se encontraron los archivos, así que no se puede hacer sitio.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Haciendo una instantánea

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = No se encontró la instantánea después.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } No se cambió el tamaño.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Algo cambió mientras se hacía la instantánea, así que no se cambió el tamaño.

# Words on a row while room is being made.
disks-making-room = Haciendo sitio

# Words on a row when room has been made. { $size } is how much.
disks-room-made = Se hizo sitio de { $size }

# Words on a row while a part is being repaired.
disks-repairing = Reparando

# Words on a row when a part has been repaired.
disks-repaired = Reparada

# Red words when some faults could not be put right.
disks-repair-failed = Algunos errores no se pudieron corregir. Copia lo que quieras conservar.

# Words on a row while a part is being checked.
disks-checking = Comprobando

# Words on a row when the check found no faults.
disks-no-faults = No se encontraron errores

# Red words when the check found faults.
disks-faults-found = Se encontraron errores: pulsa la llave para repararlos.

# Words on a row while a part or drive is being formatted.
disks-formatting = Formateando

# Words on a row when a part has been formatted.
disks-formatted = Formateada

# SAFETY: needs a human review before it ships
# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = borrando toda la unidad

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = No se encontró la partición nueva.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = No se pudo contactar con el servicio de unidades.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = No se pudo leer el identificador de la partición nueva.

# Words on a row while a part is being deleted.
disks-deleting = Eliminando

# Words on a row when a part has been deleted.
disks-deleted = Eliminada

# Words on a row while a new part is being made.
disks-making-it = Creándola

# Words on a row when a new part has been made.
disks-made = Creada

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = No se pudo guardar: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Ponle un nombre primero.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
        [one] El nombre es demasiado largo: { $max } letra como máximo.
       *[other] El nombre es demasiado largo: { $max } letras como máximo.
       [many] El nombre es demasiado largo: { $max } letras como máximo.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Usa solo letras, números, - y _ (sin espacios).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Empieza con una letra o un número.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } ya está en uso.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
        [one] El nombre es demasiado largo para este tipo: { $max } letra como máximo.
       *[other] El nombre es demasiado largo para este tipo: { $max } letras como máximo.
       [many] El nombre es demasiado largo para este tipo: { $max } letras como máximo.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Los nombres FAT no pueden contener * ? . , ; : / \ | + = < > [ ] ni ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Móntala primero: este tipo solo se puede ampliar mientras está montado.
       *[smaller] Móntala primero: este tipo solo se puede reducir mientras está montado.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Este tipo no se puede ampliar.
       *[smaller] Este tipo no se puede reducir.
    }

# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password. Said whole, as a sentence.
disks-no-password = No se dio ninguna contraseña, así que no cambió nada.

# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password, after some steps of a change were already done.
disks-no-password-part-way = No se dio ninguna contraseña, así que el trabajo se detuvo a medias.

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = Aún se está haciendo otro cambio. Vuelve a intentarlo cuando termine.

# SAFETY: needs a human review before it ships
# Red words when it could not be made sure that the password would be asked for.
disks-not-sure = No se pudo asegurar que se pidiera la contraseña, así que no cambió nada.

# SAFETY: needs a human review before it ships
# Red words after how a change ended, when the password just given could not be taken back.
disks-standing = No se pudo retirar la contraseña que se acaba de dar, así que durante unos minutos otro cambio en una unidad puede no pedirla. Se está intentando de nuevo.

# SAFETY: needs a human review before it ships
# Words after how a change ended, when the password just given has been taken back after all.
disks-taken-back = La contraseña dada ya se ha retirado: el siguiente cambio pedirá una.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = No se pudo contactar con el servicio de unidades, así que no cambió nada.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Se cerró Unidades mientras se hacía este cambio.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Unidades sigue abierto hasta que esto termine, para poder decir cómo acabó.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, así que no cambió nada.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, así que el trabajo se detuvo a medias.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = No se pudo consultar al servicio de unidades

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } El trabajo se detuvo a medias.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Hecho: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Falló: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = No se hizo: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Se desconoce: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } se desmontó para esto y ahora no está montada.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } se desmontó para esto y sigue desmontada: móntala desde su fila.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = Puede que { $dev } se haya desmontado: mira su fila.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } se desmontó para esto, y volver a montarla requiere una contraseña propia: móntala desde su fila.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } se desmontó para esto y no se pudo volver a montar ({ $why }): móntala desde su fila.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Se formateó, pero no se le volvió a dar su carpeta { $dir }. { $why } Elige la carpeta de nuevo desde su fila.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Algo la sigue usando. Cierra las ventanas o programas que muestren sus archivos y vuelve a intentarlo.

# SAFETY: needs a human review before it ships
# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = No hay sitio suficiente para lo que contiene con ese tamaño.

# Step of a change. { $dev } is the part.
disks-step-unmount = desmontando { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = volviendo a montar { $dev }

# Step of a change.
disks-step-eject = cerrando la unidad que se va a quitar

# Step of a change.
disks-step-power-off = apagando la unidad

# Step of a change. { $dev } is the part.
disks-step-format = formateando { $dev }

# Step of a change.
disks-step-create = creando la partición nueva

# Step of a change. { $dev } is the part.
disks-step-delete = eliminando { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = cambiando el tamaño de la partición { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = cambiando el tamaño de lo que hay en { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = reparando { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = comprobando { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = cambiando el nombre de { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = guardando la carpeta y la opción de arranque de { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = el paso { $method } en { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Esta partición { $kind } solo se puede reducir cuando no está en uso, así que aquí no se puede hacer sitio con ella.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Esta partición xfs no se puede reducir.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Esta partición { $kind } no se puede reducir mientras está en uso.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Solo se puede hacer una instantánea del disco propio del sistema, así que aquí no se puede hacer sitio.

# Red words under the Make room row.
disks-room-not-in-table = No es una partición de la tabla de una unidad, así que no se puede hacer sitio con ella.

# Red words under the Make room row.
disks-room-no-online-shrink = El servicio de unidades no puede reducir Btrfs mientras está en uso aquí, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-holds-parts = Contiene otras particiones, así que no se puede hacer sitio con ella.

# Red words under the Make room row.
disks-room-not-gpt = Su unidad no tiene una tabla GPT, así que no se puede hacer sitio con ella.

# Red words under the Make room row.
disks-room-mount-first = Móntala primero: Btrfs se reduce mientras está montado.

# Red words under the Make room row.
disks-room-mounts-not-one = No está montada en un solo lugar exacto, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-read-only = Está montada en solo lectura, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-read-only-unknown = No se pudo saber si está en solo lectura, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-many-drives = Los archivos se reparten por más de una unidad, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-drives-unknown = No se pudo saber por cuántas unidades se reparten los archivos, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-swap-file = Hay un archivo de intercambio en ella, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-swap-unknown = No se pudo saber si hay un archivo de intercambio en ella, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-used-unknown = No se pudo leer cuánto se usa de ella, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-area-unknown = El servicio de unidades no indica el tamaño del área de los archivos, así que no se pudo comprobar y no se puede hacer sitio.

# Red words under the Make room row.
disks-room-open-missing = No se encontró su copia abierta, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-encryption-sizes = Los tamaños dados para el cifrado no coinciden, así que no se puede hacer sitio.

# Red words under the Make room row.
disks-room-sizes = Los tamaños dados para ella no coinciden, así que no se puede hacer sitio.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Contiene { $used } y hay que conservar más que eso, así que se puede liberar muy poco.

# Red words under the Make room row.
disks-room-sizes-unread = No se pudieron leer sus tamaños.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Se puede hacer sitio de { $least } a { $most }.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, así que se detuvo el trabajo.

# What was found wrong while making room.
disks-room-check-unmounted = Los archivos ya no están montados

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = La partición es de { $is } y debería ser de { $should }

# What was found wrong while making room.
disks-room-check-open = La copia abierta del cifrado no tiene el tamaño que debería

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = El área de los archivos es de { $is } y debería ser de { $should }

# What was found wrong while making room.
disks-room-check-area-small = El área de los archivos es menor que lo que está en uso

# What was found wrong while making room.
disks-room-check-fit = Lo que contiene lo que ya no cabe

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] El servicio de unidades ya no lista la partición, así que se detuvo el trabajo.
        [copy] El servicio de unidades ya no lista la copia abierta, así que se detuvo el trabajo.
       *[files] El servicio de unidades ya no lista el sistema de archivos, así que se detuvo el trabajo.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = reduciendo el área de los archivos

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = reduciendo la copia abierta del cifrado

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = reduciendo la partición

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = comprobando los tamaños

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Antes de hacer sitio bajo { $dev } ({ $at })


# Red words when another change is pressed while one is still being made.
disks-change-going = Se está realizando un cambio.


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Haz clic otra vez: { $name } en { $drive } pasa de { $from } a { $to }. Haz antes una copia de seguridad de sus archivos, porque un corte de luz mientras se reduce puede hacer que se pierdan.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows en { $name } se hibernó o se dejó en inicio rápido, así que reducirla podría hacer que pierda sus archivos. Inicia Windows, elige Reiniciar y vuelve a intentarlo.

