# machine translated (Spanish), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Los paquetes de Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Retenida en la versión { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Retenida; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Retenida en la versión { $have }

# On a thing held back with no update waiting.
store-upd-held = Retenida

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Retenida en la versión { $have }; todavía no hay nada más reciente

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Retenida; todavía no hay nada más reciente

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [one] 1 actualización
       *[other] { $n } actualizaciones
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [one] Hay 1 actualización lista
       *[other] Hay { $n } actualizaciones listas
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, { $size } en total

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [one] 1 está retenida
       *[other] { $n } están retenidas
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = El firmware necesita un reinicio para terminar.

# Headline while the check runs.
store-upd-checking = Buscando actualizaciones.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Actualizando { $name }. Puedes seguir trabajando mientras se instala.

# Headline while all the updates go in.
store-upd-installing-all = Instalando las actualizaciones. Puedes seguir trabajando mientras se instalan.

# Headline when the updates are in and the computer must start again.
store-upd-restart = Reinicia para terminar: las actualizaciones están instaladas y se pondrán en uso cuando el equipo vuelva a arrancar.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Lo que se ha instalado necesita un reinicio para terminar.

# Added to the headline with no internet.
store-upd-no-more = Este equipo no está conectado a internet, así que no se pueden buscar más.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = No hay actualizaciones listas

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = No hay actualizaciones pendientes que este equipo conozca. No puede buscar más mientras no esté conectado a internet.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = las aplicaciones

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle y Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = el firmware

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = No se pudieron buscar actualizaciones para { $names }.

# Headline when everything is up to date.
store-upd-current = Todo está actualizado.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Última comprobación a las { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = No se puede comprobar sin internet

# Red words under a card when a check took too long.
store-upd-too-long = Tardó demasiado en responder.

# Tip of the round button that checks again.
store-upd-check-again = Comprobar de nuevo

# Main button while the check runs.
store-upd-checking-main = Comprobando

# Main button while the updates go in.
store-upd-installing-main = Instalando

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
        [one] Instalar 1 actualización
       *[other] Instalar { $n } actualizaciones
    }

# SAFETY: needs a human review before it ships
# Main button that restarts the computer.
store-upd-restart-now = Reiniciar ahora

# Main button that checks for updates.
store-upd-check-for = Buscar actualizaciones

# Tip of the round button that releases a thing held back.
store-upd-release = Liberar { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = Retener { $name }

# A card's state while it is being checked.
store-upd-status-checking = Comprobando

# A card's state while its updates go in.
store-upd-status-installing = Instalando

# A card's state when its check failed; the reason follows.
store-upd-status-failed = No se pudo comprobar. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } retenidas

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } retenidas

# A card's state when nothing waits.
store-upd-up-to-date = Actualizado

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = De { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = De { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Una parte compartida, de { $from }

# Note under the Apps card.
store-upd-yours-alone = Son solo tuyas, así que se actualizan sin contraseña. Una aplicación trae las partes compartidas que necesita su nueva versión.

# Note under the Apps card: what holding back does.
store-upd-hold-why = Una actualización retenida se queda en la versión que tiene y queda fuera de Instalar todo, aquí y en el anillo, hasta que la liberes.

# Heading of the Apps card.
store-upd-card-apps = Aplicaciones

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Todos los complementos están actualizados.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [one] La actualización de 1 complemento está retenida.
       *[other] Las actualizaciones de { $n } complementos están retenidas.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [one] 1 complemento tiene una actualización
       *[other] { $n } complementos tienen una actualización
    }

# Heading of the Plugins card.
store-upd-card-plugins = Complementos

# Held back card: nothing is held back.
store-upd-nothing-held = No hay nada retenido.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Liberada

# Note under the Held back card: what releasing does.
store-upd-release-why = Lo que se libera vuelve a instalarse con el resto, desde Instalar todo, aquí y en el anillo.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = El firmware de un dispositivo

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = el firmware del dispositivo { $id }

# Heading of the Held back card.
store-upd-card-held = Retenidas

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } a { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [one] 1 actualización, que se instala por sí sola
       *[other] { $n } actualizaciones, que se instalan juntas
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = De Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Se muestran { $shown } de { $total } actualizaciones de Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Este equipo tiene Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle y Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Los paquetes de Debian están hechos para encajar entre sí, y uno instalado solo puede traer otros consigo o dejarlos desajustados. Por eso se instalan juntos, con un único botón.

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Las actualizaciones de Trindle y de Debian traen correcciones de seguridad para todo el equipo, así que no se pueden retener.

# Heading of the card about the daily security updates.
store-upd-card-security = Actualizaciones de seguridad

# On the security card: its state.
store-upd-security-on = Activadas, cada día

# On the security card.
store-upd-security-note = Las correcciones de Debian se instalan cada día sin preguntar.

# On the security card.
store-upd-security-part = Forman parte de Trindle; no se pueden desactivar.

# On the security card: that holding back never stops these.
store-upd-never-held = Retener una actualización nunca detiene estas.

# Firmware card: the computer has no updater.
store-upd-fw-none = Este equipo no tiene actualizador de firmware.

# Firmware card: restart to finish.
store-upd-fw-restart = Reinicia para terminar

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Versión { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = El firmware nuevo se instala cuando el equipo se reinicia.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Instala fwupd para mantener el firmware actualizado.

# Heading of the Firmware card.
store-upd-card-firmware = Firmware

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd no pudo obtener la lista más reciente.

# Red words when Flathub could not be reached.
store-upd-no-flathub = No se pudo acceder a Flathub.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = No se pudieron obtener las listas de Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = No se pudieron obtener las listas de Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = No se retuvo. No se pudo guardar la lista de lo retenido: { $why }.

# The same, for a release.
store-upd-not-released-list = No se liberó. No se pudo guardar la lista de lo retenido: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Retenida en Aplicaciones, pero no se pudo avisar al propio Flatpak, así que otra cosa que actualice aplicaciones aún podría instalarla.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Liberada en Aplicaciones, pero no se pudo avisar al propio Flatpak y puede que aún la deje fuera. Retenla y libérala de nuevo.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak dijo: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle



# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = La búsqueda automática de actualizaciones está desactivada. Pulsa Buscar actualizaciones para buscar.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Sin comprobar

# Tip of the round button that ends an update that is running.
store-upd-stop = Detener

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Deteniendo

# Under a row whose update was stopped.
store-upd-stopped = Detenido. Pulsa Actualizar para continuar.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Detenido. Pulsa Instalar para continuar.

# Note under the Apps card.
store-upd-steam-restarts = Steam se reinicia cuando no haya ningún juego en marcha.

