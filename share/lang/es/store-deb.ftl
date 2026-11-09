
# machine translated (Spanish), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Instalar un paquete

# Words at the top of the window while the package is read.
store-deb-reading = Leyendo el paquete…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Este archivo no es un paquete Debian que se pueda instalar. Puede que no haya terminado de descargarse.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } está listo para instalarse en su propia carpeta.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Este paquete es para equipos { $arch }, así que no se puede instalar en este.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Esto es Trindle mismo. Se actualiza desde la página de actualizaciones de Apps, no desde un archivo.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } es un controlador o un servicio en segundo plano, que no puede funcionar desde su propia carpeta, así que no se puede instalar aquí.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Un paquete de fuera de Debian y de Trindle se instala en su propia carpeta, donde queda restringido pero no totalmente aislado. Instala solo paquetes de autores en los que confíes.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } está instalado en su propia carpeta.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Comprobando de dónde viene…

# Heading in the card that says where the file came from.
store-deb-where = De dónde viene

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Eliminarlo y lo que guardó

# Word beside the ring while the package goes in.
store-deb-installing = Instalando

# Main button that installs the package.
store-deb-install = Instalar

# Main button that installs the same version again.
store-deb-reinstall = Reinstalar

# Main button that brings the installed package up to this version.
store-deb-update = Actualizar

# Heading of the card for a package not installed yet.
store-deb-goes-in = Se instala en su propia carpeta, restringido de forma predeterminada

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = No hace falta contraseña. No puede acceder a internet, al sonido ni a tus archivos hasta que lo permitas.

# Heading of the card for a package installed at this version.
store-deb-same-version = La versión { $version } está instalada en su propia carpeta

# Under that heading.
store-deb-same-version-note = Reinstálalo ahí si ha dejado de funcionar bien.

# Heading of the card for a package installed at another version.
store-deb-other-version = La versión { $version } está instalada en su propia carpeta

# Under that heading: the version this brings.
store-deb-other-version-note = Instalar esto ahí lo actualiza a la versión { $version }, conservando lo que haya guardado.

# Facts line: how big the package is once installed.
store-deb-fact-size = Tamaño una vez instalado

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Hecho para

# Text of the About card when the package has no description.
store-deb-no-description = Este paquete no tiene descripción.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Instalando en su propia carpeta. Descargar lo que necesita puede llevar unos minutos.

# Words at the top of the window once the package is installed.
store-deb-done = Instalado en su propia carpeta. Lo encontrarás en el anillo.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Instalado en su propia carpeta, pero no se encontraron estas partes, así que puede que no arranque: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Eliminado de su propia carpeta, con todo lo que guardó.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Este archivo no es un paquete Debian que se pueda instalar.

# Step shown while a package is unpacked.
store-boxed-unpacking = Desempaquetando { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = Añadiendo { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } no es un paquete Debian que se pueda instalar.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = No se pudo desempaquetar { $path }.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Ese programa no está guardado en una caja.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal

# Title of the notice when older programs are off.
# SAFETY: needs a human review before it ships
store-boxed-x-off = Los programas antiguos están desactivados

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Ajustes, Permisos

# Title of the notice when too many older programs are open.
store-boxed-x-many = Demasiados programas antiguos

# Body of that notice.
store-boxed-x-many-close = Cierra uno e inténtalo de nuevo

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = No se pudo abrir { $app }

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = El terminal puede acceder a más partes de tu equipo que otras aplicaciones, porque lo necesita para su trabajo. Escribe solo comandos que entiendas, de personas en las que confíes.

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = La red doméstica está desactivada

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } no tiene red: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = no encontró su propio programa

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare no está instalado

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta no está instalado

# The same: the network room was not made.
store-boxed-home-why-not-made = no se creó su sala de red

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = su sala de red no era nueva

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = no se pudo crear el filtro de llamadas al sistema

# The same: the room's rules would not load.
store-boxed-home-why-rules = no se cargaron las reglas de la sala

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } no arrancó: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta dijo: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta no pudo unirse a la sala



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Abierto sin internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Se descargó, así que el programa tiene solo ese archivo, en solo lectura.

