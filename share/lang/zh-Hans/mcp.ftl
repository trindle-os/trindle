# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Window title of the question an assistant's request raises.
mcp-title-ask = 有助手提出请求

# SAFETY: needs a human review before it ships
# Heading over the words of what the assistant wants to do.
mcp-ask-heading = 连接到 Trindle 的助手想要：

# SAFETY: needs a human review before it ships
# Button that refuses the change the assistant asked for.
mcp-no = 否

# SAFETY: needs a human review before it ships
# Button that lets the change the assistant asked for go ahead.
mcp-yes = 是，继续

# SAFETY: needs a human review before it ships
# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = 把 { $name } 带到最前面

# SAFETY: needs a human review before it ships
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = 把设置 { $key } 从 { $from } 改为 { $to }

# SAFETY: needs a human review before it ships
# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = 切换为{ $look }外观

# SAFETY: needs a human review before it ships
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = 切换为{ $look }外观，强调色为 { $accent }

# SAFETY: needs a human review before it ships
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = 从 Flathub 安装 { $id }

# SAFETY: needs a human review before it ships
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = 移除 { $id }（其自身的文件会保留）

# SAFETY: needs a human review before it ships
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = 加入 Wi-Fi 网络 { $name }

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its address comes in.
mcp-ask-pair = 与蓝牙设备 { $address } 配对

# SAFETY: needs a human review before it ships
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = 把设置和应用权限恢复到之前的样子：{ $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = 把 { $id } 的设置、应用权限和文件还原到之前的状态：{ $note }。如果 { $id } 已打开，会先关闭

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = 关闭 { $id } 并重新启动。其中未保存的内容会丢失

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = 把 { $id } 的 { $file } 中的 { $key } 从 { $from } 改为 { $to }。如果 { $id } 已打开，会先关闭

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = 在 { $id } 的 { $file } 中添加 { $key }，值为 { $to }。如果 { $id } 已打开，会先关闭

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = 清除 { $id } 的缓存。如果 { $id } 已打开，会先关闭它，并保留一份缓存副本，以便恢复

# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = 与蓝牙设备 { $name }（{ $address }）配对

# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = 关闭 { $name }

# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = 打开 { $name }

# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = 允许 { $id } 使用 { $what }

# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = 不再允许 { $id } 使用 { $what }
