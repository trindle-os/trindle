
# machine translated (Spanish), not yet reviewed by a person.

# Name of the AI programs page, in the page list, as its heading and on the card listing the programs.
settings-ai-title = Programas de IA

# Words people may search for to find the AI programs page, separated by spaces.
settings-ai-keywords = ia programas de ia claude codex cursor gemini aider agente valla sandbox aislamiento carpetas privadas compartir claves ssh token sudo

# Help line at the foot of the AI programs page.
settings-ai-help = Los programas de IA para programar se ejecutan dentro de una valla que oculta tus archivos personales. Comparte solo las carpetas que necesiten.

# Headline of the AI programs page.
# SAFETY: needs a human review before it ships
settings-ai-note = En el uso diario, esto mantiene a los programas de IA lejos de tus archivos personales. Por ejemplo, impide que un programa al que han engañado reúna tus extractos bancarios y los envíe fuera. La protección tiene límites y no es una garantía. Algo podría pasar igualmente si un programa se empeña, si el software del equipo tiene un fallo, o si apruebas una petición o compartes una carpeta.

# Note under the headline of the AI programs page.
settings-ai-inside = Todo lo que inicia un programa de IA, como programas instalados y scripts, se queda dentro de la valla con él.

# Red words when a change to the fence could not be made.
settings-ai-not-changed = No se pudo cambiar.

# Said beside a setting on the AI programs page once it is saved.
settings-ai-saved = Guardado

# Title of the window for choosing a folder to share with AI programs.
settings-ai-choose-folder = Elige una carpeta para compartir con los programas de IA

# Red words when the key for AI programs to reach computers by ssh could not be made.
settings-ai-key-not-made = No se pudo crear la clave.

# Label of the switch that limits the network of AI programs, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-limit-net = Permitir solo los sitios de abajo

# Tip of the button that removes a site from the list AI programs may reach.
# SAFETY: needs a human review before it ships
settings-ai-stop-site = Impedir que los programas de IA lleguen a este sitio

# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = api.anthropic.com

# Tip of the button that adds a site AI programs may reach.
# SAFETY: needs a human review before it ships
settings-ai-let-site = Permitir que los programas de IA lleguen a este sitio

# Heading of the line for adding a site.
settings-ai-add-site = Añadir un sitio

# Under the Add a site heading.
settings-ai-add-site-sub = Escribe su nombre, o *.example.com para todos los sitios que cuelgan de él.

# Heading of the card that limits the network of AI programs.
settings-ai-network = Red

# Help text of the Network card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-network-about = Si esto está desactivado, los programas de IA usan la red igual que tú. Si está activado, solo pueden llegar a los sitios de abajo mediante conexiones seguras (https), y la valla no tiene red propia. Las herramientas de un programa deben usar los ajustes de proxy que se les den. Añade el sitio del servicio de IA y el de cualquier origen de paquetes que necesite una instalación.

# Label of the switch that keeps AI programs inside the fence, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-keep-inside = Mantener los programas de IA dentro de la valla

# Under the fence switch when it is off.
# SAFETY: needs a human review before it ships
settings-ai-fence-off = La valla está desactivada. Ahora los programas de IA pueden ver todo lo que ves tú.

# Heading of the card with the fence switch.
settings-ai-the-fence = La valla

# Help text of The fence card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-fence-about = Los programas de IA siguen pudiendo ver y usar el sistema. En lugar de tu carpeta personal, tienen una privada que empieza vacía y se conserva entre ejecuciones. Solo aparecen en ella las carpetas que compartas. No pueden ver tus otros programas, tu sesión, tus claves ni tus contraseñas. Un programa que inicies de otra forma, y no con un nombre de esta lista, no queda dentro del cerco.

# Label of the switch that fences one AI program: its name is { $name }.
# SAFETY: needs a human review before it ships
settings-ai-fence-name = Poner { $name } dentro de la valla

# Tip of the button that takes one program off the list: its name is { $name }.
# SAFETY: needs a human review before it ships
settings-ai-take-off = Quitar { $name } de la lista

# Label of the switch for using a program's existing settings folder: its name is { $name }.
settings-ai-own-for = Usar mi carpeta de ajustes existente para { $name }

# Heading of the line with the switch for using a program's existing settings folder.
settings-ai-own = Usar mi carpeta de ajustes existente

# Under the existing settings folder heading.
settings-ai-own-sub = Esta carpeta tiene las conversaciones anteriores del programa.

# Example in the box for typing the name of an AI program.
settings-ai-eg-program = claude

# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = Añadir un programa

# Help text of the AI programs card (the information button).
settings-ai-programs-about = Estos programas se inician dentro de la valla. Si Usar mi carpeta de ajustes existente está desactivado, el programa empieza con una carpeta privada nueva. Cada programa inicia sesión por su cuenta, dentro de la valla. Tu propio inicio de sesión no se comparte.

# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = No se comparte nada

# Label of the switch that lets AI programs only read a shared folder.
# SAFETY: needs a human review before it ships
settings-ai-only-read = Permitir solo leer esta carpeta

# Tip of the button that stops sharing a folder with AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-private = Volver a hacer privada esta carpeta

# Under a shared folder's path, beside the switch that makes it read only.
settings-ai-only-read-short = Solo lectura

# Tip of the button that shares a folder, and heading of its line.
# SAFETY: needs a human review before it ships
settings-ai-share-folder = Compartir una carpeta

# Under the Share a folder heading.
settings-ai-share-folder-sub = Se muestra en el mismo lugar. Los programas de IA pueden leerla y escribir en ella.

# Heading of the card listing folders shared with AI programs.
settings-ai-shared = Compartido con programas de IA

# Help text of the shared folders card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-shared-about = Todas las demás carpetas de tu carpeta personal siguen siendo privadas para los programas de IA. Nunca puedes compartir carpetas ocultas ni las que guardan claves, contraseñas o correo. Una carpeta que compartes aquí estará disponible la próxima vez que se inicie el programa.

# Tip of the button that removes a computer from the list AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-stop-computer = Impedir que los programas de IA lleguen a este equipo

# Example in the box for typing a computer AI programs may reach by ssh.
settings-ai-eg-computer = nas.local

# Tip of the button that adds a computer AI programs may reach by ssh.
# SAFETY: needs a human review before it ships
settings-ai-let-computer = Permitir que los programas de IA lleguen a este equipo

# Heading of the line for adding a computer.
settings-ai-add-computer = Añadir un equipo

# Note showing the public key to put on each computer: the key is { $key }.
# SAFETY: needs a human review before it ships
settings-ai-put-key = Pon esta clave en cada equipo: { $key }

# Tip of the button that makes a key for AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-key = Crear la clave

# Heading of the line for making a key for AI programs.
# SAFETY: needs a human review before it ships
settings-ai-make-key-for = Crear una clave para los programas de IA

# Under the Make a key heading.
settings-ai-make-key-sub = Una clave aparte, guardada fuera de las carpetas que los programas de IA pueden ver.

# Heading of the card listing computers AI programs may reach by ssh.
settings-ai-ssh = Equipos a los que pueden llegar por ssh

# Help text of the ssh card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-ssh-about = Dentro de la valla, ssh se conecta a través de Trindle. Trindle usa una clave propia, y solo para los equipos de esta lista. Los programas de IA nunca ven tus propias claves.

# Tip of the button that stops giving AI programs a key.
# SAFETY: needs a human review before it ships
settings-ai-stop-key = Dejar de dar esta clave a los programas de IA

# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = NOMBRE=valor

# Tip of the button that gives AI programs a key.
# SAFETY: needs a human review before it ships
settings-ai-give-key = Dar esta clave a los programas de IA

# Heading of the line for adding a key.
settings-ai-add-key = Añadir una clave

# Heading of the card listing keys given to AI programs.
settings-ai-keys = Claves dadas a los programas de IA

# Help text of the keys card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-keys-about = Los programas de IA nunca reciben las claves y los tokens de tu sesión. Solo reciben una clave que escribas aquí.
# Under the AI programs card, greyed, when none of the listed AI programs is on this computer.
settings-ai-none-installed = No hay programas de IA en este equipo.
# Under the box, when the program typed in to add is not on this computer.
settings-ai-not-installed = Ese programa no está en este equipo.
