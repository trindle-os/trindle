# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = agora mesmo

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] há { $n } minuto
        [many] há { $n } de minutos
       *[other] há { $n } minutos
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] há { $n } hora
        [many] há { $n } de horas
       *[other] há { $n } horas
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] há { $n } dia
        [many] há { $n } de dias
       *[other] há { $n } dias
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Aguardando a senha...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Bloqueio ligado

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Bloqueio desligado

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Salvo

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Digite o nome de um site, como example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = O bloqueio está desligado. As consultas vão para os servidores da sua própria rede, como antes.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = O bloqueio está iniciando.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Anúncios e sites ruins estão bloqueados, e as consultas vão em privado para { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = O servidor privado não pode ser acessado nesta rede, então os servidores da sua própria rede são usados por enquanto. O bloqueio continua funcionando.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Anúncios e sites ruins estão bloqueados. As consultas vão para os servidores da sua própria rede, que podem vê-las.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Anúncios e sites ruins estão bloqueados. As consultas vão para os servidores da sua VPN, pela VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Anúncios e sites ruins estão bloqueados.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Iniciando

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Desligado

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = o bloqueio está iniciando

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = o bloqueio está desligado

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Bloqueados hoje

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = consultas bloqueadas hoje

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Listas

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Baixando

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Nenhuma ainda

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = as listas de sites

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = sites listados, baixados { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Nada bloqueado ainda

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Os sites aparecem aqui, os mais recentes primeiro.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Sempre deixar este site passar

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Bloqueados recentemente

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Os sites aparecem aqui conforme são bloqueados, os mais recentes primeiro. O mais deixa um passar daqui em diante. Só os sites bloqueados são mostrados aqui. Os sites deixados passar nunca são guardados.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Ligue o bloqueio para ver o que ele bloqueia.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = por exemplo, example.com

# Heading of the line with the box for adding a site.
# SAFETY: needs a human review before it ships
settings-blocking-add = Adicionar um site

# Small line under "Add a site".
settings-blocking-add-sub = Ele e todos os seus subdomínios nunca são bloqueados.

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = Bloquear este site de novo

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Sempre liberados

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Os sites listados aqui, e todos os seus subdomínios, nunca são bloqueados, mesmo que estejam nas listas. O menos bloqueia um de novo.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Bloqueio

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = bloquear anúncios propaganda rastreadores malware golpes dns privado consultas quad9 mullvad cloudflare permitir

# Help text at the top of the page.
settings-blocking-help = Bloqueia anúncios, rastreadores e sites ruins conhecidos para todos os navegadores e aplicativos deste computador, e envia suas consultas de nomes por uma conexão criptografada, para que a rede em que você está não possa lê-las nem alterá-las.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Bloquear anúncios e sites ruins

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Para onde vão as consultas

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Usa as listas da HaGeZi de anúncios, rastreadores e sites ruins conhecidos, baixadas uma vez por dia. Enquanto estiver desligado, nada é executado. As consultas de nomes vão por uma conexão criptografada ao servidor privado escolhido aqui. A Quad9 não guarda registro de quem perguntou. "Sua rede" ainda bloqueia, mas não é privada: os servidores da sua própria rede veem cada consulta.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Listas da HaGeZi, baixadas uma vez por dia.

# Heading of the line with the private server dropdown.
settings-blocking-private = Servidor privado

# Small line under Private server.
settings-blocking-private-sub = A Quad9 não guarda registro de quem perguntou.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Sua rede
