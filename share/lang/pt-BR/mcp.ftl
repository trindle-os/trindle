
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
mcp-title-ask = Pedido de um assistente

# Heading over the words of what the assistant wants to do.
mcp-ask-heading = Um assistente conectado ao Trindle quer:

# SAFETY: needs a human review before it ships
# Button that refuses the change the assistant asked for.
mcp-no = Não

# SAFETY: needs a human review before it ships
# Button that lets the change the assistant asked for go ahead.
mcp-yes = Sim, pode continuar

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = trazer { $name } para a frente

# SAFETY: needs a human review before it ships
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = mudar a configuração { $key } de { $from } para { $to }

# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = usar o tema { $look }

# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = usar o tema { $look } com { $accent } como cor de destaque

# SAFETY: needs a human review before it ships
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = instalar { $id } do Flathub

# SAFETY: needs a human review before it ships
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = remover { $id } e manter os arquivos dele

# SAFETY: needs a human review before it ships
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = entrar na rede Wi-Fi { $name }

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its address comes in.
mcp-ask-pair = parear com o dispositivo Bluetooth { $address }

# SAFETY: needs a human review before it ships
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = restaurar as configurações e as permissões dos aplicativos como estavam antes: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = restaurar configurações, permissões e arquivos de { $id } como estavam antes: { $note }. { $id } é fechado antes, se estiver aberto

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = fechar { $id } e abrir de novo. O que não foi salvo nele se perde

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = mudar { $key } em { $file } de { $id } de { $from } para { $to }. { $id } é fechado antes, se estiver aberto

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = adicionar { $key } como { $to } em { $file } de { $id }. { $id } é fechado antes, se estiver aberto

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = limpar o cache de { $id }. { $id } é fechado antes, se estiver aberto, e uma cópia do cache é guardada para poder voltar

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = parear com o dispositivo Bluetooth { $name } ({ $address })

# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = fechar { $name }

# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = abrir { $name }

# SAFETY: needs a human review before it ships
# The question for letting an app use something; the app's Flathub id and the permission come in.
mcp-ask-allow = deixar { $id } usar { $what }

# SAFETY: needs a human review before it ships
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = impedir { $id } de usar { $what }
