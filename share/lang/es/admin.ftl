
# machine translated (Spanish), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Ese no es un nombre de usuario válido.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Ese nombre de usuario es demasiado largo.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = No hay ningún usuario llamado { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } es una cuenta del sistema y no se toca.

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = No se indicó ninguna contraseña.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
        [one] La contraseña necesita al menos { $n } carácter.
       *[other] La contraseña necesita al menos { $n } caracteres.
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = Una contraseña no puede contener un salto de línea.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } ya existe.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } es el único administrador, así que sigue siéndolo.

# SAFETY: needs a human review before it ships
# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = No puedes eliminar tu propia cuenta.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = Se eliminó a { $name }, pero su imagen sigue guardada para la pantalla de inicio de sesión: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Esa imagen es demasiado grande.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Esa no es una imagen creada por Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = No se pudo abrir la carpeta de imágenes para la pantalla de inicio de sesión: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Ese no es un nombre de idioma.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = No se pudo leer la lista de idiomas de Debian: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } no es un idioma que Debian conozca.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Un nombre de equipo lleva letras, números y guiones, hasta 63, sin empezar ni terminar con un guion.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = El límite está activado por ahora, pero se perderá cuando se apague el equipo: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = La carga es completa por ahora, pero el límite volverá en el próximo arranque: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Otros equipos aún pueden conectarse, porque no se pudo desactivar el interruptor: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Ese no es el identificador de un complemento.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = No se conoce a esa persona.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = No se conoce la carpeta de esa persona.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = No se pudo guardar la lista de complementos permitidos: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = No se encontró ese complemento.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = No se actualizó nada: no se pudo saber qué versión de Trindle tiene este equipo, así que no se pudo dejar fuera la actualización propia de Trindle.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Instalando las actualizaciones

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Ordenando

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Instalando Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Ese no es el nombre de un paquete.

# Red words when the package to remove is not installed.
admin-package-not-installed = No se hizo: no está instalado.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Ese no es un dispositivo que fwupd conozca.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = No se pudo saber quién pidió Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = El bloqueo no se inició a tiempo.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = El bloqueo no está del todo desactivado: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = No se pudo restaurar { $path }: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Ese no es un nombre de sitio.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = No existe ese servidor.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } se detuvo antes de recibirlo todo, así que esto no se puede dar por hecho: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } falló.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = No se pudo quitar { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = No se cambió nada, porque { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Dejar las cosas como estaban tampoco funcionó: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Ese no es un tiempo durante el que se puedan conservar las versiones anteriores.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = La carpeta de instantáneas no existe.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Hay algo que no es una carpeta donde van las instantáneas.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = No se pudo saber quién lo pidió.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = Esto solo se ejecuta mediante pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Ese no es el nombre de un equipo.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Ese no es el nombre de una carpeta compartida.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Esa no es la dirección de un equipo.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Esa no es la ruta de una carpeta compartida.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = Aquí solo se pueden cerrar tus propias carpetas compartidas.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = No hay ninguna carpeta compartida abierta ahí.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } no está como lo dejó Trindle, así que no se monta nada ahí.

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
admin-mounts-unreadable = No se pudo leer lo que está montado en este equipo: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Algún programa todavía tiene algo abierto en ella. Ciérralo primero.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Ese equipo no responde. Se cerrará al cerrar la sesión.

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
admin-sharing-still-shown = Este equipo aún se muestra a otros en la red: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Ese no es un número de instantánea.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Creada en Ajustes

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = No se pudo saber en qué disco está el sistema.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = El sistema no está en un subvolumen Btrfs propio, así que no se puede cambiar por una instantánea.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = No se pudo obtener la clave de firma de { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Lo que devolvió esa dirección no es una clave de firma.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = No se cambió nada, porque no se puede leer { $path }: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Obteniendo su clave de firma

# Second line of the progress while a source is added.
admin-step-adding-source = Añadiéndola y leyendo su lista

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = No se pudo usar la fuente. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = No se pudo usar la fuente, así que se quitó de nuevo. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } aún no puede activarlo ni desactivarlo: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = no se pudo iniciar para { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = No se pudo permitir a { $user } usar máquinas virtuales, que Cowork necesita: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = No se pudo permitir a { $user } usarlo sin contraseña: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } está instalado, pero su configuración no terminó: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Instalándolo

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Esa no es una de las fuentes preparadas.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } no está hecho para este tipo de equipo ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Configurándolo

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Las direcciones deben empezar por https:// y no contener nada más.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = La suite y los componentes deben ser palabras sencillas.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Ese no es un nombre de paquete.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Quitándolo

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Leyendo de nuevo las listas

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Aquí solo se quitan las fuentes que añadió Trindle.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = La fuente sigue ahí: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = La instantánea { $n } no está en el disco.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = No se cambió nada: no se pudo preparar la instantánea. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = No se pudo apartar el sistema actual: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = el sistema actual ahora se llama { $aside } y no { $sub }, y el equipo no arrancará hasta que vuelva a llamarse { $sub }: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = No se pudieron pasar las instantáneas: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = No se pudo colocar la instantánea: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = No se pudo actualizar la lista de lo que se apartó: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = No se pudo quitar: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Ya no hacían falta, así que se quitaron: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = No todas las actualizaciones se instalaron. Retenidos: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature }: está desactivada porque su controlador ({ $driver }, DKMS) todavía no se compila en el núcleo nuevo.

admin-driver-off = Un controlador ({ $driver }, DKMS) todavía no se compila en el núcleo nuevo, así que lo que ofrece está desactivado.

# Said once after the drivers that are off.
admin-driver-off-rest = El núcleo y el resto de la actualización se instalaron.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = la cámara virtual

admin-language-in-use = { $code } es el idioma con el que inicia este equipo, así que se queda.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Este equipo no tiene ninguna ralentización por bloqueo dividido que desactivar.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = No se encontró esa unidad, o contiene el sistema.

# The drive could not be closed where it was open.
admin-games-drive-busy = La unidad está en uso. Cierra lo que haya abierto en ella e inténtalo de nuevo: { $why }

# Another software change has the read-only system open.
admin-system-busy = Hay otro cambio de software en curso. Inténtalo de nuevo cuando termine.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = No se cambió nada: no se pudo desbloquear el sistema: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = No se pudo volver a bloquear el sistema. Se bloqueará en el próximo arranque: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = No se pudo mantener /usr/local con permiso de escritura: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Solo root puede ejecutar esto, y no mediante pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Necesita Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = La instantánea está en su sitio y el equipo arrancará desde ella. La limpieza del sistema antiguo no terminó y se volverá a intentar en el próximo arranque: { $why }

