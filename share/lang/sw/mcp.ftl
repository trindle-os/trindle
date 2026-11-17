
# machine translated (Swahili), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
# SAFETY: needs a human review before it ships
mcp-title-ask = Ombi la msaidizi

# Heading over the words of what the assistant wants to do.
# SAFETY: needs a human review before it ships
mcp-ask-heading = Msaidizi aliyeunganishwa na Trindle anataka:

# Button that refuses the change the assistant asked for.
# SAFETY: needs a human review before it ships
mcp-no = Hapana

# Button that lets the change the assistant asked for go ahead.
# SAFETY: needs a human review before it ships
mcp-yes = Ndiyo, endelea

# The question for bringing a window to the front; the app's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-focus = kuleta { $name } mbele

# The question for changing a Trindle setting; the setting's name and its old and new values come in.
# SAFETY: needs a human review before it ships
mcp-ask-setting = kubadilisha mpangilio { $key } kutoka { $from } kuwa { $to }

# The question for changing the look; the look (light or dark) comes in.
# SAFETY: needs a human review before it ships
mcp-ask-theme = kutumia mandhari ya { $look }

# The question for changing the look and the accent colour; both come in.
# SAFETY: needs a human review before it ships
mcp-ask-theme-accent = kutumia mandhari ya { $look } na { $accent } kama rangi ya msisitizo

# The question for installing an app from Flathub; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-install = kusakinisha { $id } kutoka Flathub

# The question for removing an app; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-remove = kuondoa { $id } na kubaki na faili zake

# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-network = kujiunga na mtandao wa Wi-Fi { $name }

# The question for pairing a Bluetooth device; its address comes in.
# SAFETY: needs a human review before it ships
mcp-ask-pair = kuoanisha na kifaa cha Bluetooth { $address }

# The question for putting settings back; the name of the saved point comes in.
# SAFETY: needs a human review before it ships
mcp-ask-undo = kurudisha mipangilio na ruhusa za programu kama zilivyokuwa kabla ya: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = kurudisha mipangilio, ruhusa za programu na faili za { $id } kama zilivyokuwa kabla ya: { $note }. { $id } hufungwa kwanza ikiwa iko wazi

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = kufunga { $id } na kuianzisha tena. Kisichohifadhiwa ndani yake kitapotea

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = kubadilisha { $key } katika { $file } ya { $id } kutoka { $from } kuwa { $to }. { $id } hufungwa kwanza ikiwa iko wazi

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = kuongeza { $key } kama { $to } katika { $file } ya { $id }. { $id } hufungwa kwanza ikiwa iko wazi

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = kufuta akiba ya { $id }. { $id } hufungwa kwanza ikiwa iko wazi, na nakala ya akiba huhifadhiwa ili irudishwe

# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
# SAFETY: needs a human review before it ships
mcp-ask-pair-named = kuoanisha na kifaa cha Bluetooth { $name } ({ $address })

# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = kufunga { $name }

# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = kufungua { $name }

# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
# SAFETY: needs a human review before it ships
mcp-ask-allow = kuruhusu { $id } kutumia { $what }

# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-deny = kuzuia { $id } kutumia { $what }
