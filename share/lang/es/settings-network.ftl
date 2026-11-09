
# machine translated (Spanish), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } está apagado por un interruptor o una tecla del propio equipo. Enciéndelo e inténtalo de nuevo.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } está bloqueado por el sistema, y no se pudo quitar el bloqueo.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = No se aceptó esa contraseña. Inténtalo de nuevo.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = No se aceptó la contraseña guardada para esta red. Olvida la red con la papelera y vuelve a unirte a ella.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = No se encontró esa red. Comprueba su nombre y que esté al alcance.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = La red dejó entrar a este equipo pero no le dio ninguna dirección. Inténtalo de nuevo, o reinicia el router.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = La red no dejó unirse a este equipo. Acércate más a ella e inténtalo de nuevo.

# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = Esa contraseña no es válida para una red Wi-Fi. Compruébala e inténtalo de nuevo.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = No se pudo unir a la red.

# SAFETY: needs a human review before it ships
# Small line of a shared folder: who may open it with a password.
settings-network-access-open = abrir con tu contraseña

# SAFETY: needs a human review before it ships
# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = abrir y cambiar con tu contraseña

# SAFETY: needs a human review before it ships
# Small line of a shared folder: anyone at home may open it.
settings-network-access-guests-open = cualquiera en casa puede abrir

# SAFETY: needs a human review before it ships
# Small line of a shared folder: anyone at home may change it.
settings-network-access-guests-change = cualquiera en casa puede cambiar

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# SAFETY: needs a human review before it ships
# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = Dejar de compartir { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Sin gestionar

# State of a network device that cannot be used.
settings-network-state-unavailable = No disponible

# State of a network device that is not connected.
settings-network-state-disconnected = Desconectado

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Preparando

# State of a network device that is being set up.
settings-network-state-configuring = Configurando

# State of a network device that needs a password.
settings-network-state-needs-password = Necesita contraseña

# State of a network device that is getting an address.
settings-network-state-address = Obteniendo dirección

# State of a network device whose connection is being checked.
settings-network-state-checking = Comprobando

# State of a network device that is waiting.
settings-network-state-waiting = Esperando

# State of a network device that is connected.
settings-network-state-connected = Conectado

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Desconectando

# State of a network device that failed.
settings-network-state-failed = Falló

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Conectado por cable, y a { $name } por Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Conectado a { $name }, señal fuerte.
        [good] Conectado a { $name }, buena señal.
        [fair] Conectado a { $name }, señal regular.
       *[other] Conectado a { $name }, señal débil.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Conectado por cable.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = El Wi-Fi está desactivado, así que este equipo no está conectado.

# Headline of the page when not connected.
settings-network-top-none = Sin conexión. Elige una red de abajo.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Buscando redes…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = No se pueden ver las redes: NetworkManager no se está ejecutando.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = El bus del sistema

# Name of the page in the list of settings pages and its heading.
settings-network-title = Red

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi inalámbrico internet ethernet cable contraseña conectar cortafuegos compartir impresora carpeta red

# Help text at the top of the page.
settings-network-help = Activa o desactiva el Wi-Fi, únete a una red, olvida una que ya no quieras, o permite que otros equipos se conecten.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Señal fuerte de { $name }.
        [good] Buena señal de { $name }.
        [fair] Señal regular de { $name }.
       *[other] Señal débil de { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Su chip necesita un Linux más reciente.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Este equipo no tiene Wi-Fi que funcione.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Desactivado. Actívalo para unirte a una red.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Activado, sin unirse a ninguna red.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Usar Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi y cable

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = El Wi-Fi une este equipo a una red sin cable. Desactivado, deja de buscar redes y ahorra un poco de batería. Una red que no difunde su nombre nunca aparece en la lista de redes al alcance: únete a ella aquí escribiendo su nombre exactamente, incluidas las mayúsculas. Cuando el chip Wi-Fi de este equipo es de los que ningún controlador de su Linux ha incorporado todavía, necesita un Linux más reciente que el que usa Trindle. Un cable de red tiene su propia línea, con un botón para conectarlo o desconectarlo.

# SAFETY: needs a human review before it ships
# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = Permitir que otros equipos se conecten

# Small line under that switch.
settings-network-share-switch-sub = Para compartir carpetas e impresoras en casa.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Buscando carpetas compartidas…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = No se pudieron leer las carpetas compartidas

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Aún no se puede llegar a ellas

# Small line under it.
settings-network-not-reachable-sub = Los demás no pueden abrirlas mientras esté desactivado.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = No hay carpetas compartidas

# Small line under it.
settings-network-no-shared-sub = Haz clic derecho en una en Carpetas para compartirla.

# Heading of the Sharing card.
settings-network-sharing = Compartir

# Help text (i button) of the Sharing card.
settings-network-sharing-about = Mientras Permitir que otros equipos se conecten esté desactivado, nada de la red puede llegar a este equipo salvo que él lo pida primero, así que nadie en el mismo Wi-Fi puede probar suerte. Actívalo para compartir una carpeta o una impresora con otros equipos de casa. Para compartir una carpeta, haz clic derecho en ella en Carpetas y elige Compartir en la red. Cada una indica quién puede hacer qué. Con tu contraseña: los demás pueden abrir los archivos, o abrirlos y cambiarlos, con tu contraseña de uso compartido. Cualquiera en casa: cualquiera en la red puede abrir los archivos, o abrirlos y cambiarlos, sin contraseña. Los demás equipos de casa no pueden abrir ninguna mientras el interruptor esté desactivado.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Deteniendo…

# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = Esperando la contraseña...

# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = Compartir permitido

# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Mantenido fuera

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = No se pudo activar el Wi-Fi.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = No se pudo desactivar el Wi-Fi.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Desconectar

# Tip of the button that connects a network or a cable.
settings-network-connect = Conectar

# Shown under a network or cable while it is being connected.
settings-network-connecting = Conectando...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = No hay redes al alcance

# Small line under it.
settings-network-none-in-range-sub = Se vuelve a buscar cada vez que se abre esta página.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Redes al alcance

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Todas las redes Wi-Fi que este equipo puede oír. Al abrir la página, primero va la que está en uso, luego las guardadas aquí y después el resto por la fuerza de su señal; una línea no se mueve mientras la página esté abierta. Pulsa las flechas para unirte a una; una red protegida pide su contraseña la primera vez. La papelera olvida una red guardada y su contraseña. Una red abierta no está cifrada, así que otros cerca podrían ver lo que se envía por ella.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Activa el Wi-Fi para ver las redes al alcance.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Guardada

# Part of a network's small line: it needs a password.
settings-network-secured = Protegida

# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = Abierta, sin cifrar

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# SAFETY: needs a human review before it ships
# Tip of the bin button that forgets a saved network.
settings-network-forget = Olvidar esta red

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = No se pudo olvidar la red.

# Placeholder of the box where a network's password is typed.
settings-network-password = Contraseña

# Tip of the button that joins the network.
settings-network-join = Unirse

# SAFETY: needs a human review before it ships
# Tip of the button that closes the box without joining.
settings-network-cancel = Cancelar

# Shown under a network while it is being joined.
settings-network-joining = Uniéndose...

# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Las contraseñas Wi-Fi tienen al menos 8 caracteres.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Unirse a una red oculta

# Small line under it.
settings-network-hidden-sub = Escribe su nombre exactamente, con mayúsculas.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Nombre de la red

# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = Contraseña, si tiene

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Hace falta el nombre de la red.

# Shown under a network after it was joined.
settings-network-joined = Unido

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Se guardó de todos modos: olvídala con la papelera antes de volver a unirte.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = No se pudo desconectar.

