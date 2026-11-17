
# machine translated (German), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
# SAFETY: needs a human review before it ships
mcp-title-ask = Anfrage eines Assistenten

# Heading over the words of what the assistant wants to do.
# SAFETY: needs a human review before it ships
mcp-ask-heading = Ein mit Trindle verbundener Assistent möchte:

# Button that refuses the change the assistant asked for.
# SAFETY: needs a human review before it ships
mcp-no = Nein

# Button that lets the change the assistant asked for go ahead.
# SAFETY: needs a human review before it ships
mcp-yes = Ja, ausführen

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = { $name } in den Vordergrund holen

# The question for changing a Trindle setting; the setting's name and its old and new values come in.
# SAFETY: needs a human review before it ships
mcp-ask-setting = die Einstellung { $key } von { $from } auf { $to } ändern

# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = das Thema { $look } verwenden

# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = das Thema { $look } mit { $accent } als Akzentfarbe verwenden

# The question for installing an app from Flathub; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-install = { $id } von Flathub installieren

# The question for removing an app; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-remove = { $id } entfernen und seine Dateien behalten

# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-network = dem Wi-Fi-Netzwerk { $name } beitreten

# The question for pairing a Bluetooth device; its address comes in.
# SAFETY: needs a human review before it ships
mcp-ask-pair = mit dem Bluetooth-Gerät { $address } koppeln

# The question for putting settings back; the name of the saved point comes in.
# SAFETY: needs a human review before it ships
mcp-ask-undo = Einstellungen und App-Berechtigungen auf den Stand von vorher zurücksetzen: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = Einstellungen, App-Berechtigungen und Dateien von { $id } auf den Stand von vorher zurücksetzen: { $note }. { $id } wird zuerst geschlossen, falls offen

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = { $id } schließen und neu starten. Nicht Gespeichertes darin geht verloren

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = { $key } in { $file } von { $id } von { $from } auf { $to } ändern. { $id } wird zuerst geschlossen, falls offen

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = { $key } als { $to } in { $file } von { $id } hinzufügen. { $id } wird zuerst geschlossen, falls offen

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = den Cache von { $id } leeren. { $id } wird zuerst geschlossen, falls offen, und eine Kopie des Caches bleibt erhalten, damit er zurückkehren kann

# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
# SAFETY: needs a human review before it ships
mcp-ask-pair-named = mit dem Bluetooth-Gerät { $name } ({ $address }) koppeln

# The question for closing a window; the window's title or app name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-close = { $name } schließen

# The question for opening an app; the app's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-open = { $name } öffnen

# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
# SAFETY: needs a human review before it ships
mcp-ask-allow = { $id } erlauben, { $what } zu nutzen

# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-deny = { $id } verbieten, { $what } zu nutzen
