
# machine translated (Spanish), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
        [one] { $count } aplicación
        [many] { $count } aplicaciones
       *[other] { $count } aplicaciones
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
        [one] { $count } actualización en espera
        [many] { $count } actualizaciones en espera
       *[other] { $count } actualizaciones en espera
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } para elegir en Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = todo lo tuyo está al día

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Ninguna aplicación coincide con «{ $text }».

# Line under the title of an empty page of apps.
store-grid-none-here = Aún no hay aplicaciones aquí.

# Advice in the empty page of a search.
store-grid-try-shorter = Prueba una palabra más corta, o explora un tipo a la izquierda.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Busca por nombre arriba, o explora otro tipo a la izquierda.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Mostrando { $shown } de { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = Mostrar { $next } más

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Obteniendo la lista de aplicaciones de Flathub. La primera vez tarda un minuto.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Apps no pudo contactar con Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Este equipo no está conectado a internet. Apps lo intenta de nuevo por su cuenta en cuanto lo esté.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Las aplicaciones para elegir vienen de Flathub, por internet.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } y los ajustes de las aplicaciones, a la izquierda, funcionan sin ella.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } y los ajustes de las aplicaciones, a la izquierda, funcionan sin Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } y los ajustes de las aplicaciones, a la izquierda, funcionan sin internet.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Reintentar

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Resultados

# Red words under the search box when no web browser opens a link.
store-no-browser = No hay ningún navegador web para abrir { $url }. Consigue uno primero en Apps.

# Said in the card or under the title while the lists are being read.
store-looking = Buscando…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Ahora está en Instaladas, y en el anillo

# Said under an app's row when it was taken off, and the word on its dial.
# SAFETY: needs a human review before it ships
store-done-removed = Eliminado

# The word on an app's dial when installing or removing did not work.
store-done-not = No se hizo

# The word on an app's dial when it was installed.
store-done-installed = Instalado

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak lo rechazó.

# Tab of the Installed page that shows everything.
store-tab-all = Todas

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Archivos Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImages

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Archivos deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Complementos

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Todavía no hay nada instalado. Descubrir está lleno de cosas por probar.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [one] 1 complemento instalado
        [many] { $n } complementos instalados
       *[other] { $n } complementos instalados
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [one] 1 programa instalado
        [many] { $n } programas instalados
       *[other] { $n } programas instalados
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } elementos instalados

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [one] { $start }, de Flathub
        [many] { $start }, todos de Flathub
       *[other] { $start }, todos de Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [one] { $start }, de un archivo Flatpak
        [many] { $start }, todos de archivos Flatpak
       *[other] { $start }, todos de archivos Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [one] { $start }, una AppImage
        [many] { $start }, todas AppImages
       *[other] { $start }, todas AppImages
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [one] { $start }, de un archivo .deb
        [many] { $start }, todos de archivos .deb
       *[other] { $start }, todos de archivos .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } de Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [one] 1 de un archivo Flatpak
        [many] { $n } de archivos Flatpak
       *[other] { $n } de archivos Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [one] 1 AppImage
        [many] { $n } AppImages
       *[other] { $n } AppImages
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [one] 1 aplicación
        [many] { $n } aplicaciones
       *[other] { $n } aplicaciones
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [one] 1 de un archivo .deb
        [many] { $n } de archivos .deb
       *[other] { $n } de archivos .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [one] 1 complemento
        [many] { $n } complementos
       *[other] { $n } complementos
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Está al día.

# End of the Installed line when everything is up to date.
store-installed-state-all = Todos están al día.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [one] 1 tiene una actualización en espera.
        [many] { $n } tienen actualizaciones en espera.
       *[other] { $n } tienen actualizaciones en espera.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } de { $total } programas coinciden con «{ $find }».

# The same when plugins are among the things.
store-installed-match-things = { $shown } de { $total } elementos coinciden con «{ $find }».

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Añadir una AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Buscar entre lo instalado

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = No se pudo añadir la AppImage.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Aquí aparecen las aplicaciones que instalas desde Descubrir, desde GitHub o como AppImages, los programas de archivos .deb y los complementos.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Nada de lo instalado coincide con «{ $find }».

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = Eliminar pide confirmación dos veces: púlsalo de nuevo en menos de tres segundos.

# On an installed thing's line: its version.
store-installed-version = versión { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = retenido

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = retenido; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Actualizando { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Actualizar { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = Acerca de

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = Eliminando...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = No hay un bus de sesión al que preguntar qué archivo.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Elige una AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImages

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = No se pudo consultar al selector de archivos.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Añadiendo...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } ya está ahí: mira en Instaladas.

# Words once an AppImage is in the ring.
store-add-in-ring = Añadido. Ya está en tu lista.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Esa no es una referencia de Flatpak que este equipo pueda usar.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Instalando { $name } desde { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } está instalado.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak no quiso instalarlo.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = No se pudo instalar { $name }. { $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = Reemplazando...

# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } ya está ahí

# Small heading over the AppImage being brought.
store-replace-new = La que llega

# Small heading over the AppImage already there.
store-replace-old = La que ya está

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = Omitir: conservar la que ya está

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Reemplazar

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = No terminó, y no dio ningún motivo.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Mostrar dónde está el problema

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Buscar en GitHub: escribe y pulsa Intro

# Heading of the card of matches from AppImageHub.
store-gh-hub = En AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Pulsa Intro para buscar también en todo GitHub.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = No se consultó GitHub: no hay internet para hacerlo.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Consultando GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = No hay nada más en GitHub.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [one] Buscando archivos AppImage y Flatpak en { $n } proyecto...
        [many] Buscando archivos AppImage y Flatpak en { $n } proyectos...
       *[other] Buscando archivos AppImage y Flatpak en { $n } proyectos...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Ninguna de las coincidencias de GitHub tiene un archivo AppImage o Flatpak para este equipo.

# Heading of the card of matches from GitHub.
store-gh-on = En GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Buscar en GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Aplicaciones en GitHub como archivos AppImage o Flatpak. Nadie las revisa: instala solo de personas en las que confíes.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = No se puede buscar en GitHub mientras este equipo no esté conectado a internet.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak y AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Ver los archivos

# Button that installs a GitHub project or file.
store-gh-install = Instalar

# Link to a project's page on GitHub.
store-gh-open = Abrir en GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, versión { $tag }

# Name of the GitHub key's button and of its box.
store-gh-key = Clave de GitHub

# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = Clave de GitHub: hay una guardada

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Casi seguro que no necesitas una. Sin clave, GitHub responde 60 consultas por hora desde este equipo; con una, responde 5000. La clave solo sube ese límite. Se usa para ver archivos públicos y nada más: no da a Trindle ninguna entrada a tu cuenta, y no marcas ningún permiso al crearla. Se guarda en este equipo, en tu llavero, y se envía solo a GitHub.

# In the GitHub key's box: how to make a key.
store-gh-key-how = Crea una clave en GitHub, ponle cualquier nombre, deja todos los permisos como están, pulsa Generate token y pega aquí lo que muestre.

# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = No se pudo eliminar: { $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = No se pudo guardar: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = Déjalo vacío para conservarla

# Placeholder of the GitHub key's box.
store-gh-key-paste = Pega la clave

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Eliminar la clave

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = Crear una clave en GitHub

# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = Guardar la clave

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Nada de su versión más reciente se instala aquí.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Añadido

# Line under the title of the Move your apps page.
store-bk-line = Tus aplicaciones y ajustes, en un solo archivo para llevar a cualquier parte.

# Heading of the card that makes a backup.
store-bk-card = Copia de seguridad

# Tip of the round button while the backup is being made.
store-bk-making-tip = Creando la copia de seguridad

# Main button that makes the backup.
store-bk-make = Crear la copia de seguridad

# Tip of the round button that shows the backup in Folders.
store-bk-show = Mostrar en Folders

# Name of the switch for the list of apps in a backup.
store-bk-apps = Aplicaciones

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [one] La lista de tu 1 aplicación Flatpak.
        [many] La lista de tus { $n } aplicaciones Flatpak.
       *[other] La lista de tus { $n } aplicaciones Flatpak.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } El otro equipo las instala desde cero desde Flathub, así que el archivo es pequeño.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Datos propios de las aplicaciones

# Under the Apps' own data switch.
store-bk-data-sub = Los ajustes y el trabajo guardado de cada aplicación, sin sus cachés.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImages

# Under the AppImages switch.
store-bk-appimages-sub = Se guardan completas, porque son archivos únicos.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Ajustes de Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = El anillo, el fondo de pantalla elegido, los colores, el teclado, la energía, las aplicaciones predeterminadas y tu imagen.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Ahora { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Adónde va

# Under that row: the folder and the file's name.
store-bk-where-sub = Tu carpeta personal, como { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = Restaurar una copia de seguridad

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Elige un archivo de copia de seguridad

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Una copia de seguridad de Trindle, o una hecha por { $flat }.

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = Se instalan sus aplicaciones y se restauran sus datos y ajustes. Tus documentos e imágenes no están incluidos: cópialos tú mismo.

# Said while a backup is being made.
store-bk-making = Haciendo la copia... puede llevar unos minutos.

# Said when a backup is made: the file and its size.
store-bk-made = Es { $path } ({ $size }). Cópiala a una memoria USB o a almacenamiento en la nube para llevarla contigo.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = No se eligió ninguna copia de seguridad, porque Trindle no tiene conexión con el selector de archivos.

# Title of the window that asks which backup to use.
store-bk-choose-title = Elige una copia de seguridad

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Copias de seguridad de Trindle y Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = No se eligió ninguna copia de seguridad, porque el selector de archivos no respondió.

# Said while a backup file is read.
store-bk-reading = Leyendo la copia de seguridad...

# Said when a backup was put back.
store-bk-logout = Cierra la sesión y vuelve a iniciarla para ver todos los ajustes.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Cierra la sesión y vuelve a iniciarla para ver todos los ajustes.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [one] 1 aplicación, { $with } con sus ajustes. Empezando...
        [many] { $apps } aplicaciones, { $with } con sus ajustes. Empezando...
       *[other] { $apps } aplicaciones, { $with } con sus ajustes. Empezando...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Aún no se ha añadido ningún origen propio de autores.
        [one] 1 origen de autor añadido, al día con las actualizaciones de Debian.
       *[other] { $n } orígenes de autores añadidos, al día con las actualizaciones de Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Añadido y mantenido al día.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. El origen está añadido, pero el programa no está instalado.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Instalado de otra forma.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Eliminarlo junto con su origen

# Link to a maker's page, naming the program.
store-src-open = Abrir { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Iniciar sesión en Tailscale

# Tip of the round button that adds a maker's source, naming the program.
store-src-add = Añadir { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Listo para añadir

# Name of the field for the address of a source.
store-src-address = Dirección

# Under the address field.
store-src-address-sub = La dirección del repositorio, de las instrucciones de instalación de su autor.

# Name of the field for the suite of a source.
store-src-suite = Suite

# Under the suite field.
store-src-suite-sub = Normalmente trixie o stable.

# Name of the field for the components of a source.
store-src-components = Componentes

# Under the components field.
store-src-components-sub = Normalmente main.

# Name of the field for a source's signing key.
store-src-key = Clave de firma

# Under the signing key field.
store-src-key-sub = La dirección de su clave de firma.

# Name of the field for the program to install from a source.
store-src-program = Programa que instalar

# Under the program field.
store-src-program-sub = El nombre de su paquete, si quieres instalarlo ahora.

# Placeholder of the program field.
store-src-program-hint = opcional

# Tip of the round button that adds the source typed.
store-src-add-this = Añadir este origen

# Heading of the card that adds another source.
store-src-another = Añadir otro origen

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Un origen puede instalar actualizaciones para todo el equipo, así que añade solo los de autores en los que confíes. Cada uno pide la contraseña de un administrador.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Eliminar este origen

# Heading of the card of sources already on the computer.
store-src-already = Ya está en este equipo

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Las dos direcciones deben empezar por https://

# Red words when the suite or components are not plain words.
store-src-bad-words = La suite y los componentes deben ser palabras simples.

# Red words when the program is not a package name.
store-src-bad-package = Ese no es un nombre de paquete.

# Word on a source's dial while its job starts.
store-src-starting = Iniciando

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Abriendo la página de inicio de sesión de Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Inicia sesión en la página que se ha abierto; esto termina por sí solo.

# Said under Tailscale's row when it is connected.
store-src-connected = Conectado

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = El enlace a las aplicaciones no está cifrado.

# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = Sin cifrar

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Esta dirección no está cifrada. Alguien en el camino podría ver lo que se envía y podría cambiar la copia de seguridad por otra distinta.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Entiendo y acepto este riesgo

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Restaurar

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Cancelar

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = ¿Quitar { $name }?

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = Sus datos guardados ocupan { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = No tiene datos guardados.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Calculando sus datos guardados...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Los datos borrados no se pueden recuperar.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Quitar y conservar sus datos

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Quitar y borrar sus datos

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Borrar solo sus datos

# Why Delete its data only is greyed.
store-flatpak-data-only-none = No tiene datos guardados que borrar.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Cancelar: conservar ambos

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = Sus datos guardados se han borrado.

# Words while only the data is being deleted.
store-flatpak-data-going = Borrando sus datos guardados...

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = No se pudieron contar sus datos guardados.

store-hub-warning = AppImageHub solo enumera proyectos: nadie los revisa ni los analiza, y no tienen por qué estar firmados. Trindle ejecuta cada AppImage en una caja con un directorio personal privado: al principio puede usar internet y sonido, y no ve ninguno de tus archivos.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = De dónde viene un AppImage es una pista, no una comprobación.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = En GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = En GitHub: { $repo }, { $stars } estrellas

# Where an AppImage comes from: a website, by its name.
store-from-site = Fuera de GitHub: de un sitio web, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Origen desconocido

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Abrir su sitio web

# When an app was last updated.
store-age-updated = Última actualización: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Última actualización: { $date }. No se actualiza desde hace más de seis meses, así que puede tener fallos de seguridad sin corregir.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Última actualización: desconocida

# When an app's project was made.
store-age-created = Creado: { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Creado: desconocido

# A date the source does not give, on a line of Facts.
store-age-not-known = Desconocido

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Antes de añadir uno

# Title of the card that lists the AppImages.
store-browse-list = AppImages

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
        [one] { $count } AppImage está en la lista de AppImageHub.
        [many] { $count } AppImages están en la lista de AppImageHub.
       *[other] { $count } AppImages están en la lista de AppImageHub.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Obteniendo la lista de AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = No se pudo obtener la lista de AppImageHub.

# The same with no internet and no copy kept.
store-browse-offline = No se puede obtener la lista de AppImageHub mientras este equipo no está conectado a internet.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub no lista ninguno de este tipo.

# Under the rows: how many are shown of how many.
store-browse-showing = Mostrando { $shown } de { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Añadir a las aplicaciones de Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Ver detalles y archivos



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = la búsqueda de actualizaciones está desactivada

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT volverá cuando OpenAI firme su descarga.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = { $apps } para elegir en Flathub y AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = { $apps } para elegir en AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Ambos

# The same: only Flatpaks.
store-show-flatpaks = Flatpaks

# The same: only AppImages.
store-show-appimages = AppImages

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = Los AppImages no se comprueban. Nadie los revisa ni los analiza.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle ejecuta cada AppImage en una caja con una carpeta personal propia: al principio puede usar internet y el sonido, y no ve ninguno de tus archivos.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = No volver a mostrar

# Button on that card that goes on with the install.
store-appimage-warn-ok = Aceptar

# Button on that card that gives the install up.
store-appimage-warn-cancel = Cancelar

