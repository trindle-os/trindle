
# machine translated (Spanish), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.

# Under the app's name when only the maker is known.
store-app-line-by = De { $by }.

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, de { $by }.

# SAFETY: needs a human review before it ships
# The state beside the dot while the app is being taken off.
store-app-state-removing = Eliminando

# The state beside the dot while the app is being fetched.
store-app-state-downloading = Descargando

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = Instalándola

# The state beside the dot while the app's own update goes in.
store-app-state-updating = Actualizando

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = Instalada, retenida

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = Instalada, retenida en la versión { $version }

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = Instalada, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = Instalada, versión { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = Instalada, al día

# The same, with the installed version.
store-app-state-current-version = Instalada, versión { $version }, al día

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = No instalada

# The same, with the size of the download.
store-app-state-absent-size = No instalada, { $size } por descargar

# SAFETY: needs a human review before it ships
# Word beside the spinner while the app is being taken off.
store-app-busy-removing = Eliminando

# Word beside the spinner while the app is being put in.
store-app-busy-installing = Instalando

# Tip and spoken name of the round button that gives up a download.
store-app-stop = Detener

# SAFETY: needs a human review before it ships
# Tip and spoken name of the round button that takes the app off.
store-app-remove = Eliminar

# Round button beside Update or Release that opens the app.
store-app-open-round = Abrir

# Main button that releases an app whose updates are held back.
store-app-release = Versión

# Main button that updates the app.
store-app-update = Actualizar

# Main button that opens the app.
store-app-open = Abrir

# Main button that installs the app.
store-app-install = Instalar

# Button that opens the app's website.
store-app-website = Sitio web

# In the picture frame when the app has no pictures.
store-app-no-pictures = Esta aplicación no tiene imágenes.

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = No se pudo obtener esta imagen.

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = Obteniendo la imagen…

# In the picture frame when the computer has no internet.
store-app-picture-offline = No se pueden obtener imágenes mientras este equipo no esté conectado a internet.

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = Imagen { $at } de { $n }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = Imagen { $at } de { $n }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = Imagen anterior

# Tip of the round button that shows the picture after.
store-app-picture-later = Imagen siguiente

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = Propietario

# Name of the Facts card.
store-app-facts = Datos

# Facts line: the size of the download.
store-app-fact-download = Tamaño de descarga

# Facts line: where the app comes from.
store-app-fact-source = Origen

# Facts line: the version.
store-app-fact-version = Versión

# Facts line: the licence of an open source app.
store-app-fact-licence = Licencia

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = Licencia, no es de código abierto

# Facts line: who made the app.
store-app-fact-made-by = Creada por

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  de { $n ->
        [one] { $count } valoración
        [many] { $count } valoraciones
       *[other] { $count } valoraciones
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = Aún no hay valoraciones.

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = No hay valoraciones que mostrar.

# Under the stars while the ratings are being fetched.
store-app-rating-looking = Buscando valoraciones…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = Del servicio abierto de reseñas de GNOME.

# Last line of the Rating card when there is no internet.
store-app-rating-offline = Este equipo no está conectado a internet.

# Name of the Rating card.
store-app-rating = Valoración

# Text of the About card when the app says nothing of itself.
store-app-no-description = Esta aplicación no tiene descripción.

# Name of the About card.
store-app-about = Acerca de

# Heading of a review that has no title.
store-app-review-no-title = Sin título

# Under What people say when nobody has written.
store-app-people-none = Nadie ha escrito aún sobre esta aplicación.

# Under What people say when there is no internet.
store-app-people-offline = No se puede obtener lo que dice la gente mientras este equipo no esté conectado a internet.

# Under What people say while reviews are being fetched.
store-app-people-looking = Buscando lo que dice la gente…

# Name of the card of reviews.
store-app-people = Lo que dice la gente

# Facts line: when the app's project was made.
store-app-fact-created = Creado

