# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 atualização de plugin precisa de uma olhada em Aplicativos
        [many] { $n } de atualizações de plugins precisam de uma olhada em Aplicativos
       *[other] { $n } atualizações de plugins precisam de uma olhada em Aplicativos
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Tudo está atualizado

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } em espera: o botão de atualizações ao lado de Aplicativos a instala
        [many] { $n } de atualizações em espera: o botão de atualizações ao lado de Aplicativos as instala
       *[other] { $n } em espera: o botão de atualizações ao lado de Aplicativos as instala
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } em espera: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } em espera: o botão de atualizações ao lado de Aplicativos instala { $put }, e { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Ver a atualização de firmware
        [many] Ver { $n } de atualizações de firmware
       *[other] Ver { $n } atualizações de firmware
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] Ver { $n } atualização, com firmware entre elas
        [many] Ver { $n } de atualizações, com firmware entre elas
       *[other] Ver { $n } atualizações, com firmware entre elas
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] Instalar { $n } atualização
        [many] Instalar { $n } de atualizações
       *[other] Instalar { $n } atualizações
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Ver a atualização de plugin, que pede mais do que antes
        [many] Ver { $n } de atualizações de plugins, que pedem mais do que antes
       *[other] Ver { $n } atualizações de plugins, que pedem mais do que antes
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] Instalar { $put } atualização; { $look }
        [many] Instalar { $put } de atualizações; { $look }
       *[other] Instalar { $put } atualizações; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Em dia

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Teclado

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Brilho da tela

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Manter ativo

# Name of the sound button when the sound is muted.
ring-tray-muted = Silenciado

# Name of the sound button with the volume level.
ring-tray-volume = Volume { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Bateria { $pct }%, carregando
        [onbattery] Bateria { $pct }%, na bateria
        [full] Bateria { $pct }%, cheia
        [pluggedin] Bateria { $pct }%, na tomada
       *[other] Bateria { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = A rede está desligada

# Name of the network button when nothing is connected.
ring-tray-not-connected = Não conectado

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Conectado por uma VPN

# Name of the network button when connected by cable.
ring-tray-wired = Cabeada

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Cabeada, conectividade limitada

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, sinal { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, sinal { $strength }%, conectividade limitada

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = O Bluetooth está desligado

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Conectado a { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = O Bluetooth está ligado

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Ícone da bandeja

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Todos os ícones da bandeja

# Words on the panel of every tray icon when there are none.
ring-tray-none = Nenhum ícone da bandeja

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Mostrar no anel

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Tirar do anel

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Já há quatro no anel. Tire um primeiro.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = A verificação está desligada

