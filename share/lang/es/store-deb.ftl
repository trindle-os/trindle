
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
store-deb-trindle = Esto es Trindle mismo. Actualízalo desde la página de actualizaciones de Apps, no desde un archivo.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } es un controlador o un servicio en segundo plano. No puede funcionar desde su propia carpeta, así que no se puede instalar aquí.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Un paquete de fuera de Debian y de Trindle se instala en su propia carpeta. Allí queda restringido, pero no totalmente aislado. Instala solo paquetes de autores en los que confíes.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } está instalado en su propia carpeta.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Comprobando de dónde viene…

# Heading in the card that says where the file came from.
store-deb-where = De dónde viene

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Eliminarlo junto con sus datos

# Word beside the ring while the package goes in.
store-deb-installing = Instalando

# Main button that installs the package.
store-deb-install = Instalar

# Main button that installs the same version again.
store-deb-reinstall = Reinstalar

# Main button that brings the installed package up to this version.
store-deb-update = Actualizar

# Heading of the card for a package not installed yet.
store-deb-goes-in = Se instala en su propia carpeta restringida

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = No hace falta contraseña. No puede usar internet ni el sonido, ni ver tus archivos, hasta que lo permitas.

# Heading of the card for a package installed at this version.
store-deb-same-version = La versión { $version } está instalada en su propia carpeta

# Heading of the card for a package installed at another version.
store-deb-other-version = La versión { $version } está instalada en su propia carpeta

# Under that heading: the version this brings.
store-deb-other-version-note = Esto lo actualiza a la versión { $version } y conserva lo que haya guardado.

# Facts line: how big the package is once installed.
store-deb-fact-size = Tamaño una vez instalado

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Hecho para

# Text of the About card when the package has no description.
store-deb-no-description = Este paquete no tiene descripción.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Instalando en su propia carpeta. Descargar las partes que necesita puede llevar unos minutos.

# Words at the top of the window once the package is installed.
store-deb-done = Se ha instalado en su propia carpeta. Lo encontrarás en el anillo.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Se ha instalado en su propia carpeta, pero faltan estas partes, así que puede que no arranque: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Se ha eliminado de su propia carpeta, con todo lo que guardó.

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
store-loose-not-boxed = Ese programa no se ejecuta en una caja.

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
store-boxed-terminal-told = El terminal puede acceder a más partes de tu equipo que otras aplicaciones. Escribe solo comandos que entiendas, de personas en las que confíes.

# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = Sin red local

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } no tiene red: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = falta el programa de Trindle

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare no está instalado

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta no está instalado

# The same: the network room was not made.
store-boxed-home-why-not-made = no se configuró su red

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = su red ya existía

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle no pudo configurar un filtro de seguridad

# The same: the room's rules would not load.
store-boxed-home-why-rules = no se pudieron cargar sus reglas de red

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } no pudo arrancar: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta dijo: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta no pudo conectarla

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = La carpeta principal { $name } es un enlace, así que no se puede usar el paquete.

# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name } es un controlador o un servicio en segundo plano. Esos no pueden funcionar desde su propia carpeta, así que no se instaló.



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Abierto sin internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Como se descargó, el programa solo puede leer ese archivo.

# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = Los programas antiguos no pueden abrirse

# Body of that notice: what to do.
store-boxed-x-unasked-why = Cierra sesión, vuelve a iniciarla y prueba otra vez
