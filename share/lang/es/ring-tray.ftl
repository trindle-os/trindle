
# machine translated (Spanish), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] 1 actualización de complemento necesita revisión en Aplicaciones
        [many] { $n } actualizaciones de complementos necesitan revisión en Aplicaciones
       *[other] { $n } actualizaciones de complementos necesitan revisión en Aplicaciones
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Todo está al día

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] { $n } en espera: el botón de actualizaciones junto a Aplicaciones la instala
        [many] { $n } en espera: el botón de actualizaciones junto a Aplicaciones las instala
       *[other] { $n } en espera: el botón de actualizaciones junto a Aplicaciones las instala
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = { $n } en espera: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = { $n } en espera: el botón de actualizaciones junto a Aplicaciones instala { $put }, y { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Ver la actualización de firmware
        [many] Ver { $n } actualizaciones de firmware
       *[other] Ver { $n } actualizaciones de firmware
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] Ver { $n } actualización, con firmware entre ellas
        [many] Ver { $n } actualizaciones, con firmware entre ellas
       *[other] Ver { $n } actualizaciones, con firmware entre ellas
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] Instalar { $n } actualización
        [many] Instalar { $n } actualizaciones
       *[other] Instalar { $n } actualizaciones
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Ver la actualización del complemento, que pide más permisos que antes
        [many] Ver { $n } actualizaciones de complementos, que piden más permisos que antes
       *[other] Ver { $n } actualizaciones de complementos, que piden más permisos que antes
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] Instalar { $put } actualización; { $look }
        [many] Instalar { $put } actualizaciones; { $look }
       *[other] Instalar { $put } actualizaciones; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Al día

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Teclado

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Brillo de pantalla

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Mantener activo

# Name of the sound button when the sound is muted.
ring-tray-muted = Silenciado

# Name of the sound button with the volume level.
ring-tray-volume = Volumen { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Batería { $pct }%, cargando
        [onbattery] Batería { $pct }%, con batería
        [full] Batería { $pct }%, llena
        [pluggedin] Batería { $pct }%, enchufado
       *[other] Batería { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = La red está desactivada

# Name of the network button when nothing is connected.
ring-tray-not-connected = Sin conexión

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Conectado mediante una VPN

# Name of the network button when connected by cable.
ring-tray-wired = Por cable

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Por cable, conectividad limitada

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, señal { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, señal { $strength }%, conectividad limitada

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth está desactivado

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Conectado a { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth está activado

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Icono de la bandeja

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Todos los iconos de bandeja

# Words on the panel of every tray icon when there are none.
ring-tray-none = No hay iconos de bandeja

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Mostrar en el anillo

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Quitar del anillo

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Ya hay cuatro en el anillo. Quita uno primero.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = La comprobación está desactivada

