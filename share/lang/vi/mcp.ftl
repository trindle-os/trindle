
# machine translated (Vietnamese), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
# SAFETY: needs a human review before it ships
mcp-title-ask = Yêu cầu từ trợ lý

# Heading over the words of what the assistant wants to do.
# SAFETY: needs a human review before it ships
mcp-ask-heading = Một trợ lý được kết nối với Trindle muốn:

# Button that refuses the change the assistant asked for.
# SAFETY: needs a human review before it ships
mcp-no = Không

# Button that lets the change the assistant asked for go ahead.
# SAFETY: needs a human review before it ships
mcp-yes = Có, tiếp tục

# The question for bringing a window to the front; the app's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-focus = đưa { $name } lên phía trước

# The question for changing a Trindle setting; the setting's name and its old and new values come in.
# SAFETY: needs a human review before it ships
mcp-ask-setting = đổi cài đặt { $key } từ { $from } sang { $to }

# The question for changing the look; the look (light or dark) comes in.
# SAFETY: needs a human review before it ships
mcp-ask-theme = dùng giao diện { $look }

# The question for changing the look and the accent colour; both come in.
# SAFETY: needs a human review before it ships
mcp-ask-theme-accent = dùng giao diện { $look } với { $accent } làm màu nhấn

# The question for installing an app from Flathub; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-install = cài đặt { $id } từ Flathub

# The question for removing an app; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-remove = gỡ { $id } và giữ lại các tệp của nó

# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-network = tham gia mạng Wi-Fi { $name }

# The question for pairing a Bluetooth device; its address comes in.
# SAFETY: needs a human review before it ships
mcp-ask-pair = ghép đôi với thiết bị Bluetooth { $address }

# The question for putting settings back; the name of the saved point comes in.
# SAFETY: needs a human review before it ships
mcp-ask-undo = khôi phục cài đặt và quyền của ứng dụng về như trước: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = khôi phục cài đặt, quyền và tệp của { $id } về như trước: { $note }. { $id } sẽ được đóng trước nếu đang mở

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = đóng { $id } rồi mở lại. Mọi thứ chưa lưu trong đó sẽ mất

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = đổi { $key } trong { $file } của { $id } từ { $from } sang { $to }. { $id } sẽ được đóng trước nếu đang mở

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = thêm { $key } là { $to } vào { $file } của { $id }. { $id } sẽ được đóng trước nếu đang mở

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = xoá bộ nhớ đệm của { $id }. { $id } sẽ được đóng trước nếu đang mở, và một bản sao bộ nhớ đệm được giữ lại để có thể khôi phục

# The question for pairing a named Bluetooth device; its name and address come in.
# SAFETY: needs a human review before it ships
mcp-ask-pair-named = ghép đôi với thiết bị Bluetooth { $name } ({ $address })

# The question for closing an app; its name comes in.
# SAFETY: needs a human review before it ships
mcp-ask-close = đóng { $name }

# The question for opening an app; its name comes in.
# SAFETY: needs a human review before it ships
mcp-ask-open = mở { $name }

# The question for allowing an app a permission; the app and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-allow = cho { $id } dùng { $what }

# The question for taking a permission away from an app; the app and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-deny = không cho { $id } dùng { $what } nữa
