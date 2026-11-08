# The ring's quick pages: sound, Bluetooth, Wi-Fi, brightness, clipboard, notifications and battery, opened from the round buttons in the ring. Part of the ring's own words (see ring.ftl).

## Quick pages (src/ring/quick/)

# Tip of the pin button on a clipboard or notification item that is not pinned: keeps it for good.
ring-quick-pin = Pin: keep it for good
# Tip of the pin button on a clipboard or notification item that is pinned.
ring-quick-unpin = Unpin
# Tip of the bin button that forgets one clipboard item or notification; it asks twice.
ring-quick-forget-one = Forget this one
# Tip of the bin button that forgets everything on the page except what is pinned; it asks twice.
ring-quick-forget-all = Forget everything but the pinned ones
# Name of the volume slider for the speakers on the sound page.
ring-quick-speakers = Speakers
# Name of the volume slider for the microphone on the sound page.
ring-quick-microphone = Microphone
# Tip of the battery page button for the power profile that saves the battery.
ring-quick-profile-saver = Battery saver
# Tip of the battery page button for the balanced power profile.
ring-quick-profile-balanced = Balanced
# Tip of the battery page button for the power profile that favours speed.
ring-quick-profile-performance = Performance
# Heading of the sound page.
ring-quick-title-sound = Sound
# Heading of the Bluetooth page.
ring-quick-title-bluetooth = Bluetooth
# Heading of the Wi-Fi page.
ring-quick-title-wifi = Wi-Fi
# Heading of the brightness page.
ring-quick-title-brightness = Brightness
# Heading of the clipboard page.
ring-quick-title-clipboard = Clipboard
# Heading of the notifications page.
ring-quick-title-notifications = Notifications
# Title of the panel in the ring's middle that lists every tray icon.
ring-quick-title-tray = Tray icons
# Heading of the shortcut list page, opened with Super and K.
ring-quick-title-shortcuts = Shortcuts
# Heading of the battery page.
ring-quick-title-battery = Battery
# Tip of the round button at the top of a quick page that opens the matching page in Settings.
ring-quick-more-settings = More in Settings
# Screen reader name of the brightness slider.
ring-quick-brightness-slider = Screen brightness
# Tip of the button that turns a muted sound back on.
ring-quick-sound-on = Turn the sound back on
# Button on the Sound page that plays a short tune through the speakers, and its label.
ring-quick-test-speakers = Test the speakers
# What the microphone's level bar shows, for the screen reader.
ring-quick-mic-level = Microphone level
# Tip of the button that mutes the sound.
ring-quick-mute = Mute
# Name of the Bluetooth switch, and the label beside it, at the top of the Bluetooth page.
ring-quick-bt-switch = Bluetooth
# Line on the Bluetooth page when Bluetooth is switched off.
ring-quick-bt-off = Bluetooth is off.
# Line on the Bluetooth page while it looks for devices and none is found yet.
ring-quick-bt-searching = Searching... put the device into pairing mode.
# Tip of the button beside a connected Bluetooth device that disconnects it.
ring-quick-bt-disconnect = Disconnect
# Tip of the button beside a paired Bluetooth device that connects it.
ring-quick-bt-connect = Connect
# Tip of the button beside a new Bluetooth device that pairs and connects it.
ring-quick-bt-pair = Pair and connect
# Name used for a Bluetooth device that has no name of its own.
ring-quick-bt-device = Bluetooth device
# Line under the Bluetooth page while pairing with a device.
ring-quick-bt-pairing = Pairing with { $name }...
# Line under the Bluetooth page while connecting a device.
ring-quick-bt-connecting = Connecting { $name }...
# Line under the Bluetooth page while disconnecting a device.
ring-quick-bt-disconnecting = Disconnecting { $name }...
# Line under the Bluetooth page when connecting or disconnecting did not work.
ring-quick-bt-failed = That did not work. Is it switched on and near by?
# Line under the Bluetooth page when pairing failed because the codes were not accepted.
ring-quick-bt-codes = Not paired: the codes were not accepted.
# Line under the Bluetooth page when pairing failed.
ring-quick-bt-not-paired = Not paired. Is it in pairing mode and near by?
# Line under the Bluetooth page when pairing worked but connecting did not.
ring-quick-bt-paired-not-connected = Paired, but it did not connect. Try again in a moment.
# Question in the ring when a Bluetooth device wants a PIN typed.
ring-quick-bt-ask-pin = Type the PIN for this device. It is often in its instructions, or 0000.
# Typing box for the PIN of a Bluetooth device, shown quietly while empty.
ring-quick-bt-pin-box = PIN
# Button in the ring that sends the PIN and pairs, or agrees to pair.
ring-quick-bt-ask-pair = Pair
# Button in the ring that gives up a question about pairing or joining.
ring-quick-cancel = Cancel
# Instruction in the ring when a Bluetooth device shows a code to be typed on the device itself.
ring-quick-bt-type-on-device = Type this on the device, then press Enter on it.
# Question in the ring when a Bluetooth device shows a number to compare.
ring-quick-bt-same-number = Does the device show this number too?
# Button in the ring that agrees the two numbers match.
ring-quick-bt-matches = Yes, it matches
# Button in the ring that says no to a question about pairing.
ring-quick-no = No
# Question in the ring when a Bluetooth device asks to pair.
ring-quick-bt-pair-this = Pair with this device?
# Name of the Wi-Fi switch, and the label beside it, at the top of the Wi-Fi page.
ring-quick-wifi-switch = Wi-Fi
# Line on the Wi-Fi page when Wi-Fi is switched off.
ring-quick-wifi-off = Wi-Fi is off.
# Line on the Wi-Fi page while it looks for networks.
ring-quick-wifi-looking = Looking for networks...
# Tip of the button beside the Wi-Fi network in use that leaves it.
ring-quick-wifi-disconnect = Disconnect
# Tip of the button beside a Wi-Fi network that joins it.
ring-quick-wifi-connect = Connect
# Typing box for the password of a Wi-Fi network, shown quietly while empty.
ring-quick-wifi-password-box = Password
# Button in the ring that joins a Wi-Fi network with the typed password.
ring-quick-wifi-join = Join
# Line under the Wi-Fi page while joining a network.
ring-quick-wifi-joining = Joining { $ssid }...
# Question text in the ring when the Wi-Fi password was not accepted.
ring-quick-wifi-wrong-password = That password did not work.
# Line under the Wi-Fi page when joining failed and the system gave no reason.
ring-quick-wifi-failed = Could not join.
# Line on the battery page when the computer has no battery.
ring-quick-battery-none = No battery in this computer.
# Line under a quick page when the screen has no brightness control.
ring-quick-brightness-fixed = This screen's brightness can't be changed from here
# Line on the clipboard page when nothing has been copied yet.
ring-quick-clip-empty = Nothing copied yet. What you copy is kept here.
# Line on the clipboard page for a copied picture.
ring-quick-clip-picture = A picture
# Tip of the button that copies a clipboard item again.
ring-quick-clip-again = Copy this again
# Typing box at the top of the clipboard page that filters the items, shown quietly while empty.
ring-quick-clip-filter = Filter
# Line on the notifications page when nothing has been said yet.
ring-quick-notes-empty = Nothing has been said yet.
# Tip of the button on one notification's page that goes back to the list.
ring-quick-notes-back = Back to all notifications
# Tip of the copy button for a moment after it was pressed.
ring-quick-copied = Copied
# Tip of the button on a crash notification that copies its details for an assistant.
ring-quick-copy-assistant = Copy it for your assistant
# Line on the battery page while charging, with the time until full.
ring-quick-charging-full = Charging: full in about { $time }
# Line on the battery page while charging, when the time is not known.
ring-quick-charging = Charging
# Line on the battery page on battery, with the time left and the power in watts.
ring-quick-on-battery-left = On battery: about { $time } left, using { $rate } W
# Line on the battery page on battery, when the time is not known.
ring-quick-on-battery = On battery
# Line on the battery page when the battery is full.
ring-quick-full = Fully charged
# Line on the battery page when plugged in and not charging.
ring-quick-plugged = Plugged in, not charging
# Line on the battery page about how much charge the battery still holds. The health is good, worn, or something else for worn out.
ring-quick-health =
    { $how ->
        [good] Health: good. It holds { $capacity }% of what it did when new.
        [worn] Health: worn. It holds { $capacity }% of what it did when new.
       *[old] Health: worn out: time for a new one. It holds { $capacity }% of what it did when new.
    }
# Line on the battery page about how many times the battery has been charged.
ring-quick-cycles =
    { $cycles ->
        [one] Charged { $cycles } time.
       *[other] Charged { $cycles } times.
    }
# Tip of the button that turns notification circles off.
ring-quick-notes-on = Notifications are on: turn them off
# Tip of the button that turns notification circles back on.
ring-quick-notes-off = Notifications are off: turn them on
# A time of under an hour on the battery page.
ring-quick-minutes = { $m } min
# A time of whole hours on the battery page.
ring-quick-hours = { $h } h
# A time of hours and minutes on the battery page.
ring-quick-hours-minutes = { $h } h { $m } min
