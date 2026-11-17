# machine translated (Spanish), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
mcp-title-ask = Petición de un asistente

# Heading over the words of what the assistant wants to do.
mcp-ask-heading = Un asistente conectado a Trindle quiere:

# SAFETY: needs a human review before it ships
# Button that refuses the change the assistant asked for.
mcp-no = No

# SAFETY: needs a human review before it ships
# Button that lets the change the assistant asked for go ahead.
mcp-yes = Sí, adelante

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = traer { $name } al frente

# SAFETY: needs a human review before it ships
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = cambiar el ajuste { $key } de { $from } a { $to }

# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = usar el tema { $look }

# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = usar el tema { $look } con { $accent } como color de énfasis

# SAFETY: needs a human review before it ships
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = instalar { $id } desde Flathub

# SAFETY: needs a human review before it ships
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = quitar { $id } y conservar sus archivos

# SAFETY: needs a human review before it ships
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = conectarse a la red Wi-Fi { $name }

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its address comes in.
mcp-ask-pair = emparejarse con el dispositivo Bluetooth { $address }
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = emparejarse con el dispositivo Bluetooth { $name } ({ $address })
# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = cerrar { $name }
# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = abrir { $name }
# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = dejar que { $id } use { $what }
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = impedir que { $id } use { $what }

# SAFETY: needs a human review before it ships
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = restaurar los ajustes y los permisos de las aplicaciones como estaban antes: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = devolver los ajustes, los permisos y los archivos de { $id } a como estaban antes: { $note }. { $id } se cierra antes si está abierta

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = cerrar { $id } y abrirla otra vez. Lo que no esté guardado en ella se pierde

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = cambiar { $key } en { $file } de { $id } de { $from } a { $to }. { $id } se cierra antes si está abierta

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = añadir { $key } como { $to } en { $file } de { $id }. { $id } se cierra antes si está abierta

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = vaciar la caché de { $id }. { $id } se cierra antes si está abierta, y se guarda una copia de la caché para poder devolverla
