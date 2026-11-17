# machine translated (Russian), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
# SAFETY: needs a human review before it ships
mcp-title-ask = Запрос помощника

# Heading over the words of what the assistant wants to do.
# SAFETY: needs a human review before it ships
mcp-ask-heading = Помощник, подключённый к Trindle, хочет:

# Button that refuses the change the assistant asked for.
# SAFETY: needs a human review before it ships
mcp-no = Нет

# Button that lets the change the assistant asked for go ahead.
# SAFETY: needs a human review before it ships
mcp-yes = Да, продолжить

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = вывести { $name } на передний план

# The question for changing a Trindle setting; the setting's name and its old and new values come in.
# SAFETY: needs a human review before it ships
mcp-ask-setting = изменить настройку { $key } с { $from } на { $to }

# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = переключить оформление: { $look }

# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = переключить оформление ({ $look }) и сменить цвет акцента на { $accent }

# The question for installing an app from Flathub; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-install = установить { $id } из Flathub

# The question for removing an app; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-remove = удалить { $id } (собственные файлы приложения сохранятся)

# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-network = подключиться к сети Wi-Fi { $name }

# The question for pairing a Bluetooth device; its address comes in.
# SAFETY: needs a human review before it ships
mcp-ask-pair = выполнить сопряжение с устройством Bluetooth { $address }

# The question for putting settings back; the name of the saved point comes in.
# SAFETY: needs a human review before it ships
mcp-ask-undo = вернуть настройки и разрешения приложений в состояние до: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = вернуть настройки, разрешения и файлы { $id } как было до: { $note }. Если { $id } открыто, оно сначала закроется

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = закрыть { $id } и запустить снова. Несохранённое в нём пропадёт

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = изменить { $key } в { $file } у { $id } с { $from } на { $to }. Если { $id } открыто, оно сначала закроется

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = добавить { $key } со значением { $to } в { $file } у { $id }. Если { $id } открыто, оно сначала закроется

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = очистить кэш { $id }. Если { $id } открыто, оно сначала закроется, а копия кэша сохранится, чтобы его можно было вернуть

# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
# SAFETY: needs a human review before it ships
mcp-ask-pair-named = выполнить сопряжение с устройством Bluetooth { $name } ({ $address })

# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = закрыть { $name }

# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = открыть { $name }

# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
# SAFETY: needs a human review before it ships
mcp-ask-allow = разрешить { $id } использовать: { $what }

# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-deny = запретить { $id } использовать: { $what }
