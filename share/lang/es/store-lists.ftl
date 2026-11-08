
# machine translated (Spanish), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Un archivo Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Un archivo .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Complemento

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = candidata a versión final

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's tile while it is being removed.
store-list-removing = Eliminando { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Instalando { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Abrir { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Instalar { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Instalada

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Mostrar

# How long is left when it is a minute or less.
store-left-minute = falta aproximadamente un minuto

# How long is left, in minutes.
store-left-minutes = faltan aproximadamente { $n } minutos

# How long is left, in hours.
store-left-hours =
    { $n ->
        [one] falta aproximadamente 1 hora
        [many] faltan aproximadamente { $n } horas
       *[other] faltan aproximadamente { $n } horas
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Comprobado con la lista Threat Intelligence de HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Descargada de { $site }, a la que se llegó desde { $referrer }.

# Where a download came from: the site.
store-origin-site = Descargada de { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Se llegó desde { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle no puede saber de dónde viene esto.

# SAFETY: needs a human review before it ships
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name } está en la lista Threat Intelligence de HaGeZi de sitios conocidos por descargas dañinas y estafas. Instala esto solo si estás seguro de dónde viene.

# SAFETY: needs a human review before it ships
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } El sitio no está en la lista Threat Intelligence de HaGeZi, pero la lista no es completa, así que esto no demuestra que sea seguro.

# SAFETY: needs a human review before it ships
# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } No se pudo comprobar el sitio, porque no se pudo obtener la lista Threat Intelligence de HaGeZi.

# Name shown for a review whose writer gave none.
store-review-someone = Alguien

# Red words when the keyring could not be reached.
store-key-unreached = No se pudo acceder al llavero: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = La clave guardada no es texto.

# SAFETY: needs a human review before it ships
# Red words when a file that held the key could not be removed.
store-key-not-removed = No se pudo eliminar { $path }: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = No se pudo leer { $path }: { $why }

