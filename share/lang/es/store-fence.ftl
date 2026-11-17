
# machine translated (Spanish), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Ese no es un nombre de programa válido.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Eso no se guardó.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Elige una carpeta que exista.

# Red words when the whole home folder is chosen to share.
store-fence-whole-home = La carpeta personal completa se mantiene privada.

# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = Esta carpeta contiene tu carpeta personal, así que se mantiene privada.

# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = Solo se pueden compartir carpetas dentro de tu carpeta personal.

# Red words when a hidden folder is chosen to share.
store-fence-hidden = Las carpetas ocultas siempre se mantienen privadas.

# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = Esta carpeta guarda contraseñas o correo, así que se mantiene privada.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Ese programa no tiene una carpeta de ajustes que usar.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Escribe el nombre de un sitio, como api.example.com, o *.example.com para todos los sitios que cuelgan de ese nombre.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Escribe el nombre de un equipo, como nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Escribe cada uno como NOMBRE=valor.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Escribe cada uno como NOMBRE=valor, en mayúsculas.

# SAFETY: needs a human review before it ships
# Words above the question when an AI program asks to use a folder.
store-fence-ask-share = Un programa de IA está pidiendo usar esta carpeta. Ahora mismo no puede verla.

# SAFETY: needs a human review before it ships
# Words above the question when an AI program asks to run something as the administrator.
store-fence-ask-admin = Un programa de IA está pidiendo ejecutar esto como administrador. Se ejecutaría fuera de la valla y podría ver todo lo que ves tú.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Programa de IA

# SAFETY: needs a human review before it ships
# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = Permitir solo leer esta carpeta

# Note under the question about a folder.
store-fence-sees-next = Verá la carpeta la próxima vez que se inicie el programa.

# SAFETY: needs a human review before it ships
# Tip of the round button that refuses a folder.
store-fence-no = No

# SAFETY: needs a human review before it ships
# Tip of the round button that shares a folder until the session ends.
store-fence-share-session = Compartir solo en esta sesión

# SAFETY: needs a human review before it ships
# Main button that shares a folder for good.
store-fence-share-always = Compartir siempre

# SAFETY: needs a human review before it ships
# Tip of the round button that refuses to run something as the administrator.
store-fence-dont-allow = No permitir

# SAFETY: needs a human review before it ships
# Main button that lets something run as the administrator.
store-fence-allow = Permitir

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Ese comando es demasiado largo para mostrarlo entero, así que no se ejecutó.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Hay otra petición esperando a que la respondas.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = No lo permitiste.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } no está disponible dentro de la valla.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = No se pudo iniciar fuera de la valla.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Indica la ruta completa de la carpeta.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = No la compartiste.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Compartida. Estará ahí la próxima vez que se inicie este programa.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Indica el nombre del equipo al que conectarte.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Los programas de IA del cerco no pueden llegar a ese equipo con ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = No se pudo crear la clave para programas de IA: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Indica el nombre de un comando que ejecutar.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } no está disponible aquí.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } no está disponible dentro de la valla. Pide que se ejecute fuera.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: falta la línea de peticiones de la valla.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: la petición no se envió.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock lo rechazó: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = La valla no se inició: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = La valla no se inició: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Los programas de IA guardan aquí sus propios inicios de sesión, aparte del tuyo. Inicia sesión dentro del programa cuando lo pida.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Tus carpetas son privadas para los programas de IA. Comparte una en Ajustes, Programas de IA, para trabajar en ella.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = La valla no se inició: no se pudo crear el filtro de llamadas al sistema.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = La valla no se inició: no se pudo configurar el límite de red.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = La valla no se inició: bubblewrap no quiso ejecutarse: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: no se encontró fuera de la valla
