
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Permisos

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = permissions allow privacy flatpak flathub sandbox internet network files sound microphone camera bluetooth notifications plugins deb x11 xwayland older programs steam wine java screen terminal computer thunderbolt usb4 dock devices permisos permitir privacidad red archivos sonido micrófono cámara notificaciones complementos programas antiguos pantalla dispositivos

# SAFETY: needs a human review before it ships
# Help line at the foot of the Permissions page.
settings-permissions-help = Aquí se ve lo que puede usar cada aplicación. Las aplicaciones de Flathub, los programas instalados en su propia carpeta y los complementos se ejecutan cada uno en su propia caja, con límites por defecto. Un cambio en una aplicación de Flathub surte efecto la próxima vez que se inicie. Los dispositivos Thunderbolt y USB4 que has permitido siempre se conectan sin preguntar. Olvida un dispositivo si quieres que se te pregunte de nuevo.

# SAFETY: needs a human review before it ships
# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = No se pudo cambiar.

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Guardado, pero Trindle no pudo volver a iniciar el complemento. El cambio surte efecto la próxima vez que se inicie.

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Guardado

# SAFETY: needs a human review before it ships
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = Cada AppImage se ejecuta en su propia caja, con una carpeta personal privada. Un cambio surte efecto la próxima vez que se inicie la AppImage.

# SAFETY: needs a human review before it ships
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Cada programa que inicias desde Folders se ejecuta en su propia caja, con una carpeta personal privada. Un cambio surte efecto la próxima vez que se inicie el programa.

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }

# Under the Allow older programs switch.
settings-permissions-older-why = Cada uno se ejecuta por separado. Si esto está desactivado, no se abren.

# Heading of the card with the switch for older programs.
settings-permissions-older = Programas antiguos

# SAFETY: needs a human review before it ships
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = Permitir programas antiguos

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = Estos puertos son solo para pantallas, así que ningún dispositivo puede acceder a la memoria de este equipo. No hay nada que preguntar.

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = Estos puertos son solo para USB y pantallas, así que ningún dispositivo puede acceder a la memoria de este equipo. No hay nada que preguntar.

# SAFETY: needs a human review before it ships
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = Estos puertos dejan que se conecte cualquier dispositivo, así que no hay nada que preguntar. Esto se establece en el firmware del equipo.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Solo las pantallas usan estos puertos.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Solo USB y pantallas usan estos puertos.

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = El firmware deja que todos los dispositivos se conecten.

# Under a package's name: its version number is { $v }.
settings-permissions-version = Versión { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Versión { $v }. No pide nada.

# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = Versión { $v }: { $notes }.

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }

# SAFETY: needs a human review before it ships
# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = No se pudo olvidar.

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt y USB4

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = Los dispositivos Thunderbolt y USB4 que has permitido siempre se conectan sin preguntar. Olvida un dispositivo si quieres que se te pregunte de nuevo. { $about }

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Puertos

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Comprobando los puertos…

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Dispositivos permitidos

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Cargando…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Protección de la memoria

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = No hay puertos Thunderbolt ni USB4

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = Así que no hay nada que preguntar.

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = Ningún dispositivo se conecta de esta forma.

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Nada que preguntar

# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = Todos. Lo deciden los puertos.

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = Este equipo también impide que los dispositivos accedan a memoria que no es suya, así que el riesgo es menor. Si olvidas un dispositivo, sigue conectado hasta que lo desenchufes.

# SAFETY: needs a human review before it ships
# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = Si olvidas un dispositivo, sigue conectado hasta que lo desenchufes. La próxima vez que lo conectes, se te preguntará de nuevo.

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = Se te pregunta por los dispositivos nuevos

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Trindle no pudo leer la lista de dispositivos permitidos: { $why }

# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = Aún no se permite que ningún dispositivo se conecte sin preguntar.

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
settings-permissions-reading-apps = Comprobando lo que pueden usar las aplicaciones…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Comprobando lo que puede usar { $n } aplicación…
       *[other] Comprobando lo que pueden usar { $n } aplicaciones…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = No has instalado ninguna aplicación ni complemento que se ejecute en su propia caja.

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = Una aplicación se ejecuta en su propia caja y solo puede usar lo que permites aquí.

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } aplicación o complemento se ejecuta en su propia caja y solo puede usar lo que permites aquí.
       *[other] { $n } aplicaciones y complementos se ejecutan cada uno en su propia caja y solo pueden usar lo que permites aquí.
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
settings-permissions-none-run = Ninguno todavía.

# SAFETY: needs a human review before it ships
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Cada aplicación de Flathub se ejecuta en su propia caja y solo puede usar lo que se permite aquí. Un cambio surte efecto la próxima vez que se inicie la aplicación.

# SAFETY: needs a human review before it ships
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = Los programas que vinieron con Trindle, como la Terminal, y los programas que instalaste desde archivos .deb. Cada uno se ejecuta en su propia carpeta, con límites por defecto.

# SAFETY: needs a human review before it ships
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = Los complementos empiezan con límites, y solo puedes permitir lo que un complemento pide. Un complemento que se ejecuta en segundo plano se reinicia enseguida cuando cambias algo.

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Códigos de escritura

# SAFETY: needs a human review before it ships
settings-permissions-codes-about = Códigos cortos que Trindle cambia por palabras mientras escribes. Cada uno muestra el complemento del que procede.

settings-permissions-codes-none = No hay códigos activos

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
settings-permissions-codes-unread-line = No se pudo leer la carpeta de complementos

# Red words under that line.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread = Trindle no pudo leer la carpeta de complementos, así que pueden faltar algunos códigos.

settings-permissions-not-added = Se abre desde Folders, no está en Apps

settings-permissions-kind-tip = Tipo de aplicación



# Heading of the card for programs that are not in a box.
settings-outside-heading = Programas fuera de una caja

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle comprueba cada pocos segundos y te avisa de lo que encuentra. No puede detener estos programas, y puede que se le escape uno que solo se ejecuta un momento. Los programas en una caja solo pueden usar lo que su caja permite.

# Label of the switch for the notice about the network.
settings-outside-network = Avisarme cuando los programas usen la red

# Label of the switch for the notice about the person's files.
settings-outside-files = Avisarme cuando los programas abran mis archivos

# Under those two switches.
settings-outside-once = Una vez por cada programa.

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Dejar de ignorar { $name }

# Under a program marked fine for the network.
settings-outside-fine-network = Usar la red está bien.

# Under a program marked fine for the person's files.
settings-outside-fine-files = Tener tus archivos abiertos está bien.

# The line in the card when no program is marked fine.
settings-outside-no-fine = Ningún programa está marcado como correcto.

# Heading of the card for downloaded files.
settings-downloads-heading = Archivos descargados

# Help text of that card (the information button).
settings-downloads-about = Los archivos de Descargas, y los archivos que tu navegador ha marcado como descargados, se abren en una caja. El programa que abre uno recibe solo ese archivo. Solo puede leerlo, y no puede usar internet ni tus carpetas. Esto no cubre el botón Abrir propio de un navegador, que inicia el programa por su cuenta.

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Abrir los archivos descargados en una caja

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Abrir los archivos { $kind } de forma normal

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = Archivos { $kind }

# Under a file type's switch.
settings-downloads-normal-sub = Se abren de forma normal, con internet.

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = Aún no hay tipos de archivo. Un tipo aparece aquí cuando un archivo descargado se abre en una caja.
