# machine translated (Bengali), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
# SAFETY: needs a human review before it ships
mcp-title-ask = সহকারীর অনুরোধ

# Heading over the words of what the assistant wants to do.
# SAFETY: needs a human review before it ships
mcp-ask-heading = Trindle-এর সাথে যুক্ত একজন সহকারী এটি করতে চায়:

# Button that refuses the change the assistant asked for.
# SAFETY: needs a human review before it ships
mcp-no = না

# Button that lets the change the assistant asked for go ahead.
# SAFETY: needs a human review before it ships
mcp-yes = হ্যাঁ, করুন

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = { $name } সামনে আনুন

# The question for changing a Trindle setting; the setting's name and its old and new values come in.
# SAFETY: needs a human review before it ships
mcp-ask-setting = { $key } সেটিং { $from } থেকে { $to } করুন

# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = { $look } থিম ব্যবহার করুন

# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = { $look } থিম ব্যবহার করুন, জোরালো রং হিসেবে { $accent } দিয়ে

# The question for installing an app from Flathub; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-install = Flathub থেকে { $id } ইনস্টল করুন

# The question for removing an app; its Flathub id comes in.
# SAFETY: needs a human review before it ships
mcp-ask-remove = { $id } সরিয়ে ফেলুন এবং এর ফাইল রেখে দিন

# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
# SAFETY: needs a human review before it ships
mcp-ask-network = Wi-Fi নেটওয়ার্ক { $name }-এ যুক্ত হোন

# The question for pairing a Bluetooth device; its address comes in.
# SAFETY: needs a human review before it ships
mcp-ask-pair = Bluetooth ডিভাইস { $address }-এর সাথে পেয়ার করুন

# The question for putting settings back; the name of the saved point comes in.
# SAFETY: needs a human review before it ships
mcp-ask-undo = সেটিং ও অ্যাপের অনুমতি আগের অবস্থায় ফিরিয়ে দিন: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = { $id }-এর সেটিংস, অ্যাপের অনুমতি ও ফাইল আগের অবস্থায় ফিরিয়ে দিন: { $note }। খোলা থাকলে আগে { $id } বন্ধ হবে

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = { $id } বন্ধ করে আবার চালু করুন। এতে সংরক্ষণ না করা কাজ হারাবে

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = { $id }-এর { $file }-এ { $key } বদলে { $from } থেকে { $to } করুন। খোলা থাকলে আগে { $id } বন্ধ হবে

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = { $id }-এর { $file }-এ { $key } যোগ করে { $to } দিন। খোলা থাকলে আগে { $id } বন্ধ হবে

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = { $id }-এর ক্যাশ খালি করুন। খোলা থাকলে আগে { $id } বন্ধ হবে, আর ক্যাশের একটি কপি রাখা হবে যাতে ফেরানো যায়

# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
# SAFETY: needs a human review before it ships
mcp-ask-pair-named = Bluetooth ডিভাইস { $name } ({ $address })-এর সাথে পেয়ার করুন

# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = { $name } বন্ধ করুন

# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = { $name } খুলুন

# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
# SAFETY: needs a human review before it ships
mcp-ask-allow = { $id }-কে { $what } ব্যবহার করতে দিন

# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
# SAFETY: needs a human review before it ships
mcp-ask-deny = { $id }-এর { $what } ব্যবহার বন্ধ করুন
