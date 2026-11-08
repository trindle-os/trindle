
# machine translated (Spanish), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Abrir sus ajustes

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = No se hizo: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = Esta actualización necesita un permiso nuevo. Está esperando a que lo decidas.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Ahora la versión { $mine }, la { $new } está lista

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = Ahora la versión { $mine }, la { $new } está lista. Pide más que antes: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Los botones de aquí vuelven cuando haya terminado.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Buscando complementos…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = No hay complementos instalados, y todavía no se ofrece ninguno.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Aún no hay complementos instalados; 1 está listo para instalar.
        [many] Aún no hay complementos instalados; { $n } están listos para instalar.
       *[other] Aún no hay complementos instalados; { $n } están listos para instalar.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] 1 complemento instalado
        [many] { $n } complementos instalados
       *[other] { $n } complementos instalados
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 tiene una actualización
        [many] { $n } tienen una actualización
       *[other] { $n } tienen una actualización
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] 1 actualización retenida
        [many] { $n } actualizaciones retenidas
       *[other] { $n } actualizaciones retenidas
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, todos al día.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, y { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, y { $have }; { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = No pide nada.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = Lo que pide: { $list }.

# Title of the Plugins page.
store-plugins-title = Complementos

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Versión { $mine } instalada; la { $new } está lista.

# On a plugin's card: its installed version.
store-plugins-version-installed = Versión { $version }, instalada.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Versión { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
store-plugins-install = Instalar { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = Dejar de retener las actualizaciones de { $name }

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = Actualizar a { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = De dónde vienen los complementos

# Information on the card that says where plugins come from.
store-plugins-come-from = Los complementos vienen solo de la lista propia de Trindle, leída de trindle.org cada vez que se abre esta página. Para ofrecer uno, envíalo: cada uno se prueba antes de publicarse.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Está instalado, pero su parte en segundo plano no se está ejecutando: { $why }

# Red words under a plugin that could not be removed; the reason follows.
# SAFETY: needs a human review before it ships
store-plugins-not-removed = No se pudo eliminar: { $why }

