
# machine translated (French), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
mcp-title-ask = Demande d'un assistant

# Heading over the words of what the assistant wants to do.
mcp-ask-heading = Un assistant connecté à Trindle souhaite :

# SAFETY: needs a human review before it ships
# Button that refuses the change the assistant asked for.
mcp-no = Non

# SAFETY: needs a human review before it ships
# Button that lets the change the assistant asked for go ahead.
mcp-yes = Oui, continuer

# SAFETY: needs a human review before it ships
# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = mettre { $name } au premier plan

# SAFETY: needs a human review before it ships
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = modifier le réglage { $key } de { $from } à { $to }

# SAFETY: needs a human review before it ships
# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = utiliser le thème { $look }

# SAFETY: needs a human review before it ships
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = utiliser le thème { $look } avec { $accent } comme couleur d'accent

# SAFETY: needs a human review before it ships
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = installer { $id } depuis Flathub

# SAFETY: needs a human review before it ships
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = supprimer { $id } et conserver ses fichiers

# SAFETY: needs a human review before it ships
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = se connecter au réseau Wi-Fi { $name }

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its address comes in.
mcp-ask-pair = s'appairer avec l'appareil Bluetooth { $address }

# SAFETY: needs a human review before it ships
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = remettre les réglages et les autorisations des applications comme avant : { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = remettre les réglages, les permissions et les fichiers de { $id } comme avant : { $note }. { $id } est d'abord fermée si elle est ouverte

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = fermer { $id } et la relancer. Ce qui n'y est pas enregistré est perdu

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = changer { $key } dans { $file } de { $id } de { $from } à { $to }. { $id } est d'abord fermée si elle est ouverte

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = ajouter { $key } avec { $to } dans { $file } de { $id }. { $id } est d'abord fermée si elle est ouverte

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = vider le cache de { $id }. { $id } est d'abord fermée si elle est ouverte, et une copie du cache est gardée pour pouvoir le remettre

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = s'appairer avec l'appareil Bluetooth { $name } ({ $address })
# SAFETY: needs a human review before it ships
# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = fermer { $name }
# SAFETY: needs a human review before it ships
# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = ouvrir { $name }
# SAFETY: needs a human review before it ships
# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = autoriser { $id } à utiliser { $what }
# SAFETY: needs a human review before it ships
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = empêcher { $id } d'utiliser { $what }
