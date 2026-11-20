
# machine translated (Spanish), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Ese no es un nombre de usuario válido.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Ese nombre de usuario es demasiado largo.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = No hay ningún usuario { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } es una cuenta del sistema, así que Trindle no la ha cambiado.

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = No se indicó ninguna contraseña.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
        [one] La contraseña necesita al menos { $n } carácter.
       *[other] La contraseña necesita al menos { $n } caracteres.
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = Una contraseña no puede tener saltos de línea.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } ya existe.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } es el único administrador, así que debe seguir siéndolo.

# SAFETY: needs a human review before it ships
# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = No puedes eliminar tu propia cuenta.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = Trindle eliminó la cuenta de { $name }, pero no pudo quitar su imagen de la pantalla de inicio de sesión: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Esa imagen es demasiado grande.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Esa no es una imagen creada por Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle no pudo dar acceso a la carpeta de imágenes a la pantalla de inicio de sesión: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Ese no es un nombre de idioma.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle no pudo leer la lista de idiomas de Debian: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian no reconoce el idioma { $code }.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Un nombre de equipo solo puede usar letras, números y guiones. Puede tener hasta 63 caracteres y no puede empezar ni terminar con un guion.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = El límite de carga está activado por ahora, pero se perderá cuando se apague el equipo: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = La carga completa está activada por ahora, pero el límite volverá la próxima vez que arranque el equipo: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Otros equipos aún pueden conectarse, porque Trindle no pudo desactivar el uso compartido: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Ese no es un ID de complemento.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = No existe esa persona.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Esa persona no tiene carpeta.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle no pudo guardar la lista de complementos permitidos: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = No existe ese complemento.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = No se actualizó nada. Trindle no pudo saber qué versión suya está instalada, así que no pudo dejar fuera su propia actualización.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Instalando las actualizaciones

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Ordenando

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Instalando Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Ese no es el nombre de un paquete.

# Red words when the package to remove is not installed.
admin-package-not-installed = Ese paquete no está instalado.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Ese no es un dispositivo que fwupd conozca.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle no sabe quién quiere Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = El bloqueo no se inició a tiempo.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = El bloqueo no está del todo desactivado: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle no pudo restaurar { $path }: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Ese no es un nombre de sitio.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = No existe ese servidor.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } se detuvo antes de tener todo lo que necesitaba, así que el trabajo no ha terminado: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } falló.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = No se pudo quitar { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = No se cambió nada, porque Trindle no pudo leer { $path }: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle tampoco pudo dejar las cosas como estaban: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Esa no es una duración válida para conservar las versiones anteriores.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = No se encuentran las versiones anteriores.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Hay un archivo donde debería estar la carpeta de las versiones anteriores.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = No se sabe quién lo pidió.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = Esto solo funciona con pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Ese no es el nombre de un equipo.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Ese no es el nombre de una carpeta compartida.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Esa no es la dirección de un equipo.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Esa no es la ruta de una carpeta compartida.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Aquí solo puedes cerrar tus propias carpetas compartidas.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = No hay ninguna carpeta compartida abierta ahí.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Algo ha modificado { $path } desde que Trindle lo preparó, así que no hay nada conectado ahí.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } no está como lo dejó Trindle.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Hay demasiadas carpetas compartidas abiertas.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } no está instalado. Viene en el paquete { $package } de Debian.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = No se pudo leer el inicio de sesión.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle no pudo leer lo que está montado en este equipo: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Un programa todavía está usando algo de esta carpeta. Cierra el programa y vuelve a intentarlo.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Ese equipo no responde. La carpeta compartida se cerrará cuando cierres la sesión.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Ese no es un identificador de usuario.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = No se pudo crear el grupo sambashare.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Obteniendo Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Configurando el uso compartido

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = El uso compartido aún no está configurado. Comparte primero una carpeta.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Este equipo aún es visible para otros en la red: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Ese no es un número de instantánea.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Creada en Ajustes

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle no pudo saber en qué disco está el sistema.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = El sistema no está en un subvolumen Btrfs propio, así que Trindle no puede cambiarlo por una instantánea.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle no pudo obtener la clave de firma de { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Esa dirección no devolvió una clave de firma.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = No se cambió nada, porque Trindle no pudo leer { $path }: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Obteniendo su clave de firma

# Second line of the progress while a source is added.
admin-step-adding-source = Añadiéndola y leyendo su lista

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = No se pudo usar la fuente. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle no pudo usar la fuente, así que la quitó de nuevo. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } aún no puede activarlo ni desactivarlo: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle no pudo iniciarlo para { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Trindle no pudo permitir a { $user } usar máquinas virtuales, que Cowork necesita: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = Trindle no pudo permitir a { $user } usarlo sin contraseña: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } está instalado, pero Trindle no pudo terminar de configurarlo: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Instalándolo

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Esa no es una de las fuentes preparadas.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } no está hecho para este tipo de equipo ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Configurándolo

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Cada dirección debe ser una dirección web sencilla que empiece por https://.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = La suite y los componentes deben ser palabras sencillas.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Ese no es un nombre de paquete.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Quitándolo

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Leyendo de nuevo las listas

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Aquí solo se pueden quitar las fuentes que añadió Trindle.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = La fuente sigue ahí: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = La instantánea { $n } no está en el disco.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = No se cambió nada, porque Trindle no pudo preparar la instantánea. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle no pudo apartar el sistema actual: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = El sistema actual ahora se llama { $aside } en lugar de { $sub }, y el equipo no arrancará hasta que vuelva a llamarse { $sub }: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle no pudo pasar las instantáneas: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle no pudo colocar la instantánea: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle no pudo actualizar la lista de sistemas que se apartaron: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = No se pudo quitar { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Trindle quitó estos elementos porque ya no hacían falta: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = No se instalaron todas las actualizaciones. Estas quedaron retenidas: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } está desactivada porque su controlador ({ $driver }, DKMS) todavía no se puede compilar para el nuevo núcleo de Linux.

admin-driver-off = El controlador ({ $driver }, DKMS) todavía no se puede compilar para el nuevo núcleo de Linux. Las funciones que lo necesitan están desactivadas.

# Said once after the drivers that are off.
admin-driver-off-rest = El núcleo y el resto de la actualización se instalaron.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = la cámara virtual

admin-language-in-use = { $code } es el idioma con el que inicia este equipo, así que no se puede quitar.

# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Este equipo no tiene ninguna ralentización por bloqueo dividido que desactivar.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle no encontró esa unidad, o es la unidad del sistema.

# The drive could not be closed where it was open.
admin-games-drive-busy = La unidad está en uso. Cierra los archivos o programas que haya abiertos en ella e inténtalo de nuevo: { $why }

# Another software change has the read-only system open.
admin-system-busy = Hay otro cambio de software en curso. Inténtalo de nuevo cuando termine.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = No se cambió nada, porque Trindle no pudo desbloquear el sistema: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle no pudo volver a bloquear el sistema. Se bloqueará en el próximo arranque: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle no pudo mantener /usr/local con permiso de escritura: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Solo root puede ejecutar esto, y no mediante pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Necesita Btrfs.

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = La instantánea está en su sitio y el equipo arrancará desde ella. Trindle no pudo terminar de limpiar el sistema antiguo. Lo intentará de nuevo la próxima vez que arranque el equipo: { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = Puedes abrir como máximo { $most } puertos.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle no pudo guardar la lista de puertos abiertos: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = El cortafuegos sigue activado: { $why }

# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = Este complemento ahora pide otros puertos de red distintos de los que permitiste. Vuelve a permitirlo para abrirlos.

# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = Un programa solo se puede añadir para la persona que lo pide.

# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = Otra restauración usó el mismo nombre hace un momento. Espera unos segundos e inténtalo de nuevo.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = No se pudo desconectar { $path }: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 carpeta compartida no respondió, así que sigue conectada. Se desconectará cuando el equipo se apague.
       *[other] { $n } carpetas compartidas no respondieron, así que siguen conectadas. Se desconectarán cuando el equipo se apague.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = No se pudo desactivar el archivo de intercambio { $path }: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = No se pudo reactivar el archivo de intercambio { $path }: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } no es un archivo de intercambio del sistema.

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = Trindle no usará el archivo de intercambio { $path }, porque otros podrían cambiar o leer { $why }.

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = No se encontró dónde está ahora el archivo de intercambio, así que quizá no funcione salir de la hibernación.

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = La hibernación volverá a funcionar después de reiniciar.

# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = El chip ya se está configurando. Espera a que termine.

# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = Esa no es la contraseña del disco.

# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = Demasiadas contraseñas incorrectas. Espera un minuto y vuelve a probar.

# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = El chip desbloqueó el disco al arrancar. No hay nada que hacer.

# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = El disco cambió desde el arranque. Reinicia y prueba otra vez.

# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = No se encontró el chip de seguridad. Actívalo en la BIOS.

# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = Falta clevis, la herramienta que configura el chip.

# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = El disco no tiene espacio para otra clave.

# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = No se configuró el chip. El disco se abre con su contraseña.

# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = El chip funciona, pero no se pudo quitar su clave anterior.
