
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Permisos

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = permissions allow privacy flatpak flathub sandbox internet network files sound microphone camera bluetooth notifications plugins deb x11 xwayland older programs steam wine java screen terminal computer thunderbolt usb4 dock devices permisos permitir privacidad red archivos sonido micrófono cámara notificaciones complementos programas antiguos pantalla dispositivos

# SAFETY: needs a human review before it ships
# Help line at the foot of the Permissions page.
settings-permissions-help = Lo que puede usar cada aplicación. Las aplicaciones de Flathub, los programas instalados en su propia carpeta y los complementos se instalan cada uno en su propia carpeta, con restricciones por defecto. Un icono lleno está permitido; haz clic en él para cambiarlo. Un cambio en una aplicación de Flathub surte efecto la próxima vez que se inicie. Los dispositivos Thunderbolt y USB4 permitidos siempre se conectan sin preguntar; olvida uno para que se te pregunte de nuevo.

# SAFETY: needs a human review before it ships
# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = No se pudo cambiar.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Guardado, pero no se pudo volver a iniciar el complemento. El cambio surte efecto la próxima vez que se inicie.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Guardado

# SAFETY: needs a human review before it ships
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = Cada AppImage se ejecuta en una caja propia, con un directorio personal privado. Un icono lleno está permitido; haz clic en él para cambiarlo. Un cambio surte efecto la próxima vez que se inicie la AppImage.

# SAFETY: needs a human review before it ships
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Los programas iniciados desde Folders se ejecutan cada uno en una caja propia, con un directorio personal privado. Un icono lleno está permitido; haz clic en él para cambiarlo. Un cambio surte efecto la próxima vez que se inicie el programa.

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }

# Under the Allow older programs switch.
settings-permissions-older-why = Cada uno se mantiene aislado de los demás. Si se desactivan, no se abren.

# Heading of the card with the switch for older programs.
settings-permissions-older = Programas antiguos

# SAFETY: needs a human review before it ships
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = Permitir programas antiguos

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = Estos puertos solo transportan pantallas, así que ningún dispositivo puede acceder a la memoria de este equipo y no hay nada que preguntar.

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Estos puertos solo transportan USB y pantallas, así que ningún dispositivo puede acceder a la memoria de este equipo y no hay nada que preguntar.

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = Estos puertos dejan que todos los dispositivos se conecten por sí solos, así que no hay nada que preguntar. Esto se establece en el firmware del equipo.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Estos puertos solo transportan pantallas.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Estos puertos solo transportan USB y pantallas.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = El firmware deja que todos los dispositivos se conecten.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Versión { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Versión { $v }. No pide nada.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# SAFETY: needs a human review before it ships
# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = No se pudo olvidar.

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt y USB4

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = Los dispositivos Thunderbolt y USB4 permitidos siempre se conectan sin preguntar; olvida uno para que se te pregunte de nuevo. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Los puertos

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Leyendo lo que dejan pasar…

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Dispositivos permitidos

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Leyendo…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Memoria aislada

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = No hay puertos Thunderbolt ni USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Así que no hay nada que preguntar.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Ninguno: nada se conecta de esta forma.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Nada que preguntar

# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = Todos, por los propios puertos.

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = Este equipo también mantiene a los dispositivos lejos de la memoria que no se les ha dado, así que el riesgo es menor. Olvidar un dispositivo lo deja conectado hasta que se desenchufe.

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = Olvidar un dispositivo lo deja conectado hasta que se desenchufe; la próxima vez se preguntará de nuevo por él.

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = Se pregunta antes de que se conecte un dispositivo nuevo

# Under the heading above.
settings-permissions-forget-one = Olvida uno para que se te pregunte de nuevo.

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = No se pudieron leer los dispositivos permitidos: { $why }

# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = Aún ningún dispositivo puede conectarse sin preguntar.

# SAFETY: needs a human review before it ships
# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
settings-permissions-forget-device = Olvidar este dispositivo

# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = Un dispositivo Thunderbolt

# SAFETY: needs a human review before it ships
# Under an allowed device's name when its maker is not known.
settings-permissions-may-always = Puede conectarse siempre

# SAFETY: needs a human review before it ships
# Under an allowed device's name: the maker is { $vendor }.
settings-permissions-made-by = Fabricado por { $vendor }. Puede conectarse siempre.

# Headline while the list of apps is being read.
settings-permissions-reading-apps = Leyendo lo que puede usar cada aplicación…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Leyendo lo que puede usar cada una de las { $n }…
        [many] Leyendo lo que puede usar cada una de las { $n }…
       *[other] Leyendo lo que puede usar cada una de las { $n }…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = No hay instalados aplicaciones ni complementos en una caja propia.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Una aplicación está en una caja propia y usa solo lo que se permite aquí.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } aplicación o complemento está en una caja propia y usa solo lo que se permite aquí.
        [many] { $n } aplicaciones y complementos están cada uno en una caja propia y usan solo lo que se permite aquí.
       *[other] { $n } aplicaciones y complementos están cada uno en una caja propia y usan solo lo que se permite aquí.
    }

# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Aplicaciones de Flathub

# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = En su propia carpeta

# Heading of the card for plugins.
settings-permissions-plugins = Complementos

# Heading of the card for AppImages.
settings-permissions-appimages = AppImages

# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Iniciados desde Folders

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = Ninguno instalado.

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = Ninguno ejecutado aún.

# SAFETY: needs a human review before it ships
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Las aplicaciones de Flathub se mantienen cada una en una caja propia y solo pueden usar lo que se permite aquí. Un icono lleno está permitido; haz clic en él para cambiarlo. Un cambio surte efecto la próxima vez que se inicie la aplicación.

# SAFETY: needs a human review before it ships
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = Los programas instalados en su propia carpeta tienen restricciones por defecto. Un icono lleno está permitido; haz clic en él para cambiarlo.

# SAFETY: needs a human review before it ships
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = Los complementos tienen restricciones por defecto y solo se les puede permitir lo que piden. Un complemento que se ejecuta en segundo plano vuelve a iniciarse enseguida con el cambio.

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Códigos de escritura

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Códigos cortos que Trindle cambia por palabras mientras escribes, cada uno con el complemento al que pertenece. Desactiva los códigos de un complemento con su icono redondo.

settings-permissions-codes-none = No hay códigos activos

# SAFETY: needs a human review before it ships
settings-permissions-codes-none-sub = Un complemento al que se permita expandir códigos los muestra aquí.

# Button on a plugin's row: stops that plugin's codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-off = Desactivar los códigos de { $name }

settings-permissions-codes-count =
    { $n ->
        [one] Un código, { $id }
       *[other] { $n } códigos, { $id }
    }

# A code's second line: what it types and who owns it.
settings-permissions-code-types = Escribe «{ $words }» para { $name }

settings-permissions-codes-more =
    { $n ->
        [one] Hay un código más activo
       *[other] Hay { $n } códigos más activos
    }

# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread-line = No se pudo leer la carpeta de plugins

# Red words under that line.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread = No se pudo leer la carpeta de plugins, así que pueden faltar algunos códigos.

settings-permissions-not-added = Se ejecuta desde Folders, no se añade a Apps

settings-permissions-kind-tip = Tipo de aplicación



# Heading of the card for programs that are not in a box.
settings-outside-heading = Programas fuera de una caja

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle mira cada pocos segundos y te avisa. No puede detener estos programas y se le puede escapar uno rápido. Un programa en una caja queda retenido por su caja.

# Label of the switch for the notice about the network.
settings-outside-network = Avisar cuando un programa llega a la red

# Label of the switch for the notice about the person's files.
settings-outside-files = Avisar cuando un programa tiene tus archivos abiertos

# Under those two switches.
settings-outside-once = Una vez por cada programa.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Avisar otra vez sobre { $name }

# Under a program marked fine for the network.
settings-outside-fine-network = Llegar a la red está bien.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Tener tus archivos abiertos está bien.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Ningún programa está marcado como correcto.

# Heading of the card for downloaded files.
settings-downloads-heading = Archivos descargados

# Help text of that card (the information button).
settings-downloads-about = Un archivo de internet es uno que está en Descargas, o uno que el navegador ha marcado. El programa que lo abre recibe solo ese archivo, en solo lectura, sin internet y sin ninguna de tus carpetas. Esto no cubre el botón Abrir propio de un navegador, que inicia el programa por su cuenta.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Abrir los archivos descargados en una caja

# Under that switch.
settings-downloads-box-sub = Sin internet y solo ese archivo, en solo lectura.

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Abrir los archivos { $kind } de forma normal

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Archivos { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Activado: se abre de forma normal, con internet.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Todavía no hay ningún tipo. Se mostrará uno cuando un archivo descargado se abra en una caja.

