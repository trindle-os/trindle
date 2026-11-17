
# machine translated (Spanish), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = ahora mismo

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] hace { $n } minuto
       *[other] hace { $n } minutos
       [many] hace { $n } minutos
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] hace { $n } hora
       *[other] hace { $n } horas
       [many] hace { $n } horas
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] hace { $n } día
       *[other] hace { $n } días
       [many] hace { $n } días
    }

# SAFETY: needs a human review before it ships
# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
settings-blocking-waiting = Esperando tu contraseña…

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Bloqueo activado

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Bloqueo desactivado

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Guardado

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Escribe el nombre de un sitio, como example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = El bloqueo está desactivado. Los sitios web se consultan a través de los servidores de tu propia red, como antes.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = El bloqueo se está iniciando.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Se bloquean los anuncios y los sitios dañinos, y los sitios web se consultan en privado a través de { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle no puede llegar al servidor privado en esta red, así que por ahora usa los servidores de tu propia red. El bloqueo sigue funcionando.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Se bloquean los anuncios y los sitios dañinos. Los sitios web se consultan a través de los servidores de tu propia red, que pueden ver cada sitio que visitas.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Se bloquean los anuncios y los sitios dañinos. Los sitios web se consultan a través de los servidores de tu VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Se bloquean los anuncios y los sitios dañinos.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Iniciando

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Desactivado

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = el bloqueo se está iniciando

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = el bloqueo está desactivado

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Bloqueados hoy

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = consultas bloqueadas hoy

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Listas

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Actualizando

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Aún ninguna

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = las listas de sitios

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = sitios en la lista, actualizada { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Aún no se ha bloqueado nada

# SAFETY: needs a human review before it ships
# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = Dejar pasar siempre este sitio

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Bloqueados hace poco

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Los sitios aparecen aquí a medida que se bloquean, los más recientes primero. Pulsa el signo más para dejar pasar un sitio desde ahora. Trindle no guarda ningún registro de los sitios que deja pasar.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Activa el bloqueo para ver lo que bloquea.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Añadir un sitio

# SAFETY: needs a human review before it ships
# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Volver a bloquear este sitio

# SAFETY: needs a human review before it ships
# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = Dejar pasar siempre

# SAFETY: needs a human review before it ships
# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = Estos sitios, y todas las direcciones que cuelgan de ellos, nunca se bloquean, aunque estén en las listas. Pulsa el signo menos para volver a bloquear un sitio.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Bloqueo

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = bloquear anuncios publicidad rastreadores malware estafas dns consultas privadas quad9 mullvad cloudflare permitir

# Help text at the top of the page.
settings-blocking-help = Bloquea anuncios, rastreadores y sitios dañinos conocidos para todos los navegadores y programas de este equipo. También consulta los sitios web por una conexión cifrada, para que la red en la que estás no pueda ver ni cambiar qué sitios consultas.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Bloquear anuncios y sitios dañinos

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Adónde van las consultas

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Usa las listas de HaGeZi de anuncios, rastreadores y sitios dañinos conocidos, actualizadas una vez al día. Mientras el bloqueo esté desactivado, no se ejecuta nada. Los sitios web se consultan por una conexión cifrada a través del servidor privado que elijas aquí. Quad9 no guarda registro de quién preguntó. "Tu red" también bloquea, pero no es privada, porque los servidores de tu propia red ven cada sitio que consultas.

# Heading of the line with the private server dropdown.
settings-blocking-private = Servidor privado

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Tu red


# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = Cortafuegos

# Help text (i button) of the Firewall card.
settings-firewall-about = Cuando el cortafuegos está activado, los otros equipos no pueden conectarse a este a menos que este equipo haya iniciado la conexión. Así nadie en la misma Wi-Fi puede intentar colarse. Navegar, actualizar e imprimir siguen funcionando. Un programa que espera conexiones de otros equipos, como un servidor de juegos, necesita que abras su puerto más abajo. Cuando el cortafuegos está desactivado, cualquier equipo de la red puede llegar a todos los programas que esperan conexiones. Necesitas la contraseña de administrador para activarlo o desactivarlo.

# Name of the firewall's on and off switch.
settings-firewall-switch = Cortafuegos

# Small line under the switch while the firewall is off.
settings-firewall-off-sub = Los otros equipos pueden llegar a los programas que esperan conexiones.

# Said beside the switch once the firewall is on.
settings-firewall-done-on = Cortafuegos activado

# Said beside the switch once the firewall is off.
settings-firewall-done-off = Cortafuegos desactivado

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = Bloqueo



# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = El cortafuegos está activado pero no funciona, porque sus reglas no se cargaron al iniciar el equipo. Apágalo y vuelve a encenderlo.
