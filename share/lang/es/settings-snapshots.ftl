
# machine translated (Spanish), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Instantáneas

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = instantáneas instantánea snapshots snapshot restaurar deshacer revertir volver atrás copia de seguridad actualización se rompió btrfs snapper menú de arranque grub versiones anteriores versión antigua archivo eliminado recuperar inicio copias

# SAFETY: needs a human review before it ships
# Help line at the foot of the Snapshots page.
settings-snap-help = El sistema tal como estaba antes y después de cada cambio de software, y siempre que tomes una. Restaurar una devuelve todo el sistema a ese estado desde el siguiente arranque; tus propios archivos se dejan como están cuando la carpeta personal tiene una parte propia del disco, como la crea el instalador de Trindle. Si el equipo no arranca, elige las instantáneas de Trindle en el menú de arranque. Versiones anteriores guarda cada día una copia de los archivos de cada persona, aparte de las instantáneas del sistema.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = No se hizo ninguna copia: el disco está casi lleno.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = hoy a las { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = ayer a las { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = el { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = El equipo no se reinició. Reinícialo desde el anillo.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Haciendo una instantánea

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Liberando...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Liberado

# SAFETY: needs a human review before it ships
# Said beside a snapshot while it is restored.
settings-snap-restoring = Restaurando

# SAFETY: needs a human review before it ships
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = El sistema es la instantánea { $n } desde el siguiente arranque. Todo lo cambiado desde entonces se aparta, no se pierde.

# SAFETY: needs a human review before it ships
# Said beside a snapshot while it is deleted.
settings-snap-deleting = Eliminando

# SAFETY: needs a human review before it ships
# Said beside a snapshot once it is deleted.
settings-snap-deleted = Eliminada

# SAFETY: needs a human review before it ships
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = Esperando la contraseña...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Se hace una copia cada día

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = No se hacen más

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Conservada

# SAFETY: needs a human review before it ships
# Said beside the earlier copies while they are cleared.
settings-snap-clearing = Vaciando

# SAFETY: needs a human review before it ships
# Said beside the earlier copies once they are cleared.
settings-snap-cleared = Vaciado

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Aún no hay copias
        [one] Una copia
       *[other] { $n } copias
       [many] { $n } copias
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = La primera se hace en menos de un día.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Ocupan { $size }, medido { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Su espacio se mide una vez al día.

# SAFETY: needs a human review before it ships
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = Vaciarlas todas ahora

# Heading of the card about earlier versions of files.
settings-snap-earlier = Versiones anteriores

# SAFETY: needs a human review before it ships
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle puede guardar una copia de los archivos de todos tal como estaban cada día, para que una versión más antigua de un archivo, o uno eliminado por error, se pueda abrir o recuperar desde Carpetas. Cada persona solo puede leer en una copia lo que podía leer ese día, así que una carpeta personal privada sigue siendo privada; un archivo que estaba abierto a otros seguirá abierto para ellos en esa copia hasta que desaparezca. Las copias son aparte de las instantáneas del sistema: restaurar el sistema nunca cambia tus archivos, y estas nunca cambian el sistema. El coste es el espacio: un archivo que eliminas o cambias sigue ocupando su espacio hasta que desaparece la última copia que lo contiene, así que eliminar cosas no libera el disco de inmediato. Vaciar las copias lo libera ahora, para todos los usuarios de este equipo, y no se puede deshacer. No se hace ninguna copia mientras el disco está casi lleno. Cambiar cualquiera de esto requiere la contraseña de un administrador.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Conservar versiones anteriores de tus archivos

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Los archivos eliminados se conservan durante { $time } antes de liberar su espacio.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Conservar la copia de cada día durante

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Cuánto tiempo se conserva la copia de cada día

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Mirando las instantáneas…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = No se pudieron leer las instantáneas.

# SAFETY: needs a human review before it ships
# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Se arrancó desde la instantánea { $n }, así que nada de lo que se haga ahora se conserva.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] { $n } instantánea a la que volver, la más reciente de { $when }.
       *[other] { $n } instantáneas a las que volver, la más reciente de { $when }.
       [many] { $n } instantáneas a las que volver, la más reciente de { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Aún no hay instantáneas. Se hace una antes de cada cambio de software.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, conservada hasta que se elimine

# SAFETY: needs a human review before it ships
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = Restaurar el sistema a esta

# SAFETY: needs a human review before it ships
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = Eliminar esta instantánea

# SAFETY: needs a human review before it ships
# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Reiniciar ahora

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Aún no están configuradas.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Reinstala el paquete trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Las instantáneas no están configuradas. Reinstalar el paquete trindle las configura: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = otro tipo

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Necesitan Btrfs; este equipo usa { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Elige Btrfs al instalar Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Las instantáneas necesitan el sistema en Btrfs, y este equipo usa { $fs }. Elige Btrfs para el sistema al instalar Debian para tenerlas.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = No hay instantáneas en este equipo.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Sin instantáneas

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Sin instantáneas en este equipo

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Para tenerlas

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Buscando…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = No se pudieron leer las instantáneas

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Aún ninguna

# Under the None yet heading.
settings-snap-none-line-sub = Se hace una antes de cada cambio de software.

# SAFETY: needs a human review before it ships
# Help text of the Snapshots card (the information button).
settings-snap-list-about = Cada instantánea puede restaurar todo el sistema a como estaba entonces, desde el siguiente arranque, o eliminarse. Ambas acciones piden pulsar dos veces y necesitan la contraseña de un administrador. Tras una restauración, junto a ella aparece un botón para reiniciar.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Hacer una instantánea ahora

# SAFETY: needs a human review before it ships
# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = No mientras se arranca desde una instantánea.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Se conserva hasta que la elimines.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Un sistema apartado por restaurar
       *[other] { $n } sistemas apartados por restaurar
       [many] { $n } sistemas apartados por restaurar
    }

# Under the set aside heading.
settings-snap-free-space = Libera el espacio cuando todo vaya bien.

# SAFETY: needs a human review before it ships
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = Liberar el espacio que usan

# Heading of the card for taking a snapshot.
settings-snap-take-one = Hacer una

# Help text of the Take one card (the information button).
settings-snap-take-about = Haz una instantánea antes de probar algo que quizá quieras deshacer; la que hagas se conserva hasta que la elimines. Mientras el equipo está arrancado desde una instantánea, nada de lo que se haga se conservaría, así que reinicia normalmente primero. Cada restauración aparta lo que era el sistema justo antes, así que no se pierde nada; libera el espacio que usan cuando todo vaya bien.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Se arrancó desde la instantánea { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Nada de lo que se haga ahora se conserva.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Tus archivos también están en las instantáneas

# SAFETY: needs a human review before it ships
# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Restaurar también devuelve tus archivos.

# SAFETY: needs a human review before it ships
# Heading of the card about what to know before restoring.
settings-snap-before = Antes de restaurar

# SAFETY: needs a human review before it ships
# Help text of the Before restoring card (the information button).
settings-snap-before-about = Al arrancar desde una instantánea, nada de lo que se haga ahora se conserva: si el sistema debe estar así, pulsa restaurar en ella en la lista y luego reinicia. En este equipo la carpeta personal puede ser parte del sistema, y entonces restaurar una instantánea también devuelve tus archivos a como estaban entonces, así que copia antes a un lugar seguro todo lo nuevo.



# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Archivos del sistema bloqueados

# Under the System files locked switch.
settings-snap-locked-sub = Solo las instalaciones y las actualizaciones los desbloquean.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Bloqueados

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Desbloqueados

