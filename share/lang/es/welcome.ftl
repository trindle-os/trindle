
# machine translated (Spanish), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Te damos la bienvenida a Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Abre el anillo

# Title of the third page, where the first apps are picked.
welcome-page-apps = Elige tus aplicaciones

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Más para elegir

# Title of the fifth page, about plugins.
welcome-page-plugins = Los complementos añaden más

# Title of the sixth page, about crash reports.
welcome-page-reports = Ayuda a mejorar Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Los puntos

# Title of the last page, with a few things worth knowing.
welcome-page-know = Conviene saber

# Sentence under the title of the first page.
welcome-sentence-hello = Todo lo que necesitas está en un círculo, el anillo.

# Sentence under the title of the second page.
welcome-sentence-ring = Haz doble clic en el borde mismo de la pantalla, o pulsa la tecla Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Casi nada está instalado todavía, así que elige las aplicaciones que quieras.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Elige más y luego instala todo lo elegido con un solo clic.

# Sentence under the title of the fifth page.
welcome-sentence-plugins = Los complementos son pequeños extras de Aplicaciones, y cada uno hace solo lo que le permitas.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Si quieres, envía un informe cuando algo salga mal.

# Sentence under the title of the dots page.
welcome-sentence-dots = Un punto significa que un programa está usando algo tuyo.

# Sentence under the title of the last page.
welcome-sentence-know = Algunas cosas que hacen distinto a Trindle.

# Top right of every page: where the person is in the tour.
welcome-step = Paso { $n } de { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Paso { $n } de { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = Ya se ha visto la bienvenida. Está en Todas las aplicaciones como Bienvenida; borra este archivo para verla otra vez en el próximo inicio de sesión.

# Name of the round button at the foot that closes the tour.
welcome-close = Cerrar

# Name of the round button at the foot that goes to the page before.
welcome-back = Atrás

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Siguiente

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Instalar las aplicaciones elegidas

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Empezar a usar Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Conéctate a internet para instalar aplicaciones.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Reintentar

# Foot while apps are going in: their names.
welcome-installing = Instalando { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Instalando { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } ya está en el anillo.
       *[other] { $names } ya están en el anillo.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = No se pudieron instalar las aplicaciones. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak lo rechazó.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Elegidas: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } y { $more } más

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = El anillo

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Qué contiene

# Name of the first thing the ring holds.
welcome-held-programs = Tus programas

# Line under it.
welcome-held-programs-line = Van por el borde exterior.

# Name of the second thing the ring holds.
welcome-held-time = La hora y la fecha

# Line under it.
welcome-held-time-line = Van en el centro.

# Name of the third thing the ring holds.
welcome-held-switches = Los interruptores

# Line under it.
welcome-held-switches-line = Sonido, pantalla, mantener activo y energía.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Con el puntero

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Con el teclado

# Note under the picture of the Super key.
welcome-ring-super-note = La tecla Super suele llevar el logotipo de Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Navegador web

# Head of a card of apps: email programs.
welcome-kind-email = Correo

# Head of a card of apps: chat programs.
welcome-kind-chat = Chat

# Head of a card of apps: office programs.
welcome-kind-office = Ofimática

# Head of a card of apps: music and video players.
welcome-kind-media = Música y vídeo

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Imágenes y dibujo

# Head of a card of apps: notes and writing.
welcome-kind-notes = Notas y escritura

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF y documentos

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Ninguna de estas está ahora en Flathub.

# Beside an app that has been installed.
welcome-app-installed = Instalada

# Beside an app that is going in now.
welcome-app-installing = Instalando

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, elegida

# Name of the round button on an app's row that picks it.
welcome-app-pick = Elegir { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = En el anillo

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Los complementos

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = En Aplicaciones, en Complementos.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Abrir los complementos en Aplicaciones

# Said in the plugins card while there are none to show.
welcome-plugins-none = Los complementos disponibles aparecen en Aplicaciones.

# Head of the card on the crash reports page.
welcome-reports-card = Informes de fallos

# The one question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-question = ¿Quieres ayudar a mejorar Trindle enviando un informe cuando algo salga mal? Siempre lo ves antes.

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = No se envía nada sin que lo pulses tú. Ves el informe completo antes, y no contiene archivos, contraseñas ni nombres de redes Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = Puedes cambiarlo cuando quieras en Ajustes, Informes de fallos.

# The answer that turns crash reports off.
# SAFETY: needs a human review before it ships
welcome-reports-no = No

# The answer that turns crash reports on.
# SAFETY: needs a human review before it ships
welcome-reports-yes = Sí, pregúntame

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Enviar un informe cuando algo salga mal

# Head of a card on the last page.
welcome-know-boxes = Las aplicaciones viven en sus propias cajas

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Una aplicación pide permiso antes de acceder a tus archivos, tu cámara o tu micrófono.

# Head of a card on the last page.
welcome-know-updates = Las actualizaciones de seguridad se instalan solas

# Line in that card.
welcome-know-updates-line = Cada día, sin preguntar. Las demás actualizaciones esperan a que tú decidas.

# Head of a card on the last page.
welcome-know-ring = Ajustes y Aplicaciones están en el anillo

# Line in that card.
welcome-know-ring-line = En su centro, encima de la hora.

# Head of a card on the last page.
welcome-know-help = La ayuda está en Ajustes

# Line in that card.
welcome-know-help-line = Esta guía está en Todas las aplicaciones como Bienvenida, para cuando quieras verla otra vez.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Una pantalla pequeña sin nada más que el anillo en el centro: la hora en medio y las aplicaciones favoritas alrededor.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = El anillo abierto en una pantalla pequeña, con los iconos de algunos complementos en fila en su centro, bajo la hora, donde va el icono de un complemento una vez instalado.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Dos dibujos pequeños de dónde aparecen los puntos: en la pestaña de una ventana en el anillo y en la parte superior central de la pantalla.

welcome-dots-where-tab = En la pestaña de una ventana

welcome-dots-where-corner = Parte superior central de la pantalla

welcome-picture-ring = Una pantalla pequeña. El puntero se mueve al borde superior y hace doble clic, y el anillo se abre en el centro.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Se pulsa la tecla Super, que abre el anillo.


# Title of the first page, which asks how much the person knows.
welcome-page-level = ¿Cuánto sabes?

# Sentence under that title.
welcome-sentence-level = Elige lo más parecido y la visita omitirá lo que ya sabes.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Nuevo en Linux

# Line in that answer's card.
welcome-level-new-line = Todas las páginas, incluido lo que hace cualquier escritorio Linux.

# Name of the answer for someone who has used Linux.
welcome-level-linux = He usado Linux

# Line in that answer's card.
welcome-level-linux-line = Solo lo propio de Trindle. Se omiten los conceptos básicos de Linux.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = He usado Trindle

# Line in that answer's card.
welcome-level-trindle-line = Cierra la visita ahora y no vuelve a mostrarla.

# Title of the page about files and folders.
welcome-page-files = Tus archivos

# Sentence under that title.
welcome-sentence-files = Todo lo tuyo está en tu carpeta personal.

# Title of the page about getting software.
welcome-page-software = Conseguir programas

# Sentence under that title.
welcome-sentence-software = Las aplicaciones vienen de Aplicaciones, no de instaladores que descargas.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = La contraseña de administrador

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Los cambios en todo el equipo piden antes una contraseña.

# Title of the page about snapshots.
welcome-page-snapshots = Instantáneas

# Sentence under that title.
welcome-sentence-snapshots = Trindle guarda copias del sistema, así que una actualización se puede deshacer.

# Title of the page about the person's own words.
welcome-page-words = Tus propias palabras

# Sentence under that title.
welcome-sentence-words = Cambia cualquier palabra que muestre Trindle o añade un idioma propio.

# Head of a card on the files page.
welcome-files-home = Una carpeta personal

# Line in that card.
welcome-files-home-line = Documentos, Descargas e Imágenes están dentro. Carpetas se abre ahí.

# Head of a card on the files page.
welcome-files-drives = Sin letras de unidad

# Line in that card.
welcome-files-drives-line = Una memoria USB u otro disco aparece en Carpetas al conectarlo.

# Head of a card on the files page.
welcome-files-hidden = Nombres ocultos

# Line in that card.
welcome-files-hidden-line = Un nombre que empieza por un punto, como .config, está oculto. Guarda los ajustes de una aplicación.

# Head of a card on the files page.
# SAFETY: needs a human review before it ships
welcome-files-bin = La papelera

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Un archivo eliminado espera en la papelera hasta que la vacíes.

# Head of a card on the software page.
welcome-software-apps = Un solo lugar para las aplicaciones

# Line in that card.
welcome-software-apps-line = Abre Aplicaciones desde el anillo para buscar, instalar y quitar programas.

# Head of a card on the software page.
welcome-software-exe = Sin instaladores

# Line in that card.
welcome-software-exe-line = Un instalador de Windows, que acaba en .exe, no se ejecuta aquí. Busca la aplicación en Aplicaciones.

# Head of a card on the software page.
welcome-software-updates = Actualizaciones en un solo lugar

# Line in that card.
welcome-software-updates-line = Aplicaciones y Ajustes lo mantienen todo al día, así que ninguna aplicación necesita su propio actualizador.

# Head of a card on the software page.
# SAFETY: needs a human review before it ships
welcome-software-remove = Quitar deja todo limpio

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Quitar una aplicación se lleva todo y no deja nada atrás.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Por qué la pide

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Instalar para todos o cambiar un disco afecta a todos los usuarios, así que el equipo pregunta primero.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = De quién es la contraseña

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = La de un administrador, normalmente con quien inicias sesión.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Cuándo no la pide

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Las aplicaciones que instalas solo para ti y tus propios archivos nunca la necesitan.

# Head of a card on the snapshots page.
welcome-snap-taken = Hechas por ti

# Line in that card.
welcome-snap-taken-line = En un sistema Btrfs se hace una antes y otra después de cada cambio de programas.

# Head of a card on the snapshots page.
welcome-snap-list = En Ajustes

# Line in that card.
welcome-snap-list-line = Ajustes, Instantáneas las muestra. La cámara hace una ahora.

# Head of a card on the snapshots page.
welcome-snap-boot = Útil si no arranca

# Line in that card.
welcome-snap-boot-line = Elige Trindle snapshots en el menú de arranque y luego una instantánea.

# Head of a card on the your own words page.
welcome-words-change = Cambiar una palabra

# Line in that card.
welcome-words-change-line = Busca cualquier palabra que muestre Trindle y escribe la tuya. «Ábrelo, brew» puede sustituir a «Haz clic aquí para abrir el archivo».

# Head of a card on the your own words page.
welcome-words-share = Compartir una hoja

# Line in that card.
welcome-words-share-line = Exporta tus palabras como un archivo o importa uno que te envió un amigo. Se comprueba antes de usarlo.

# Head of a card on the your own words page.
welcome-words-all = Todos los programas a la vez

# Line in that card.
welcome-words-all-line = Todo Trindle lee sus palabras de un solo lugar, así que un cambio se ve en todas partes.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Está en Ajustes, Región e idioma. La Ayuda lo explica paso a paso.

# Name of the round button that opens that Settings page.
welcome-words-open = Abrir Región e idioma
