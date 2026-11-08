# Settings, Bluetooth page: the adapter, pairing, connecting and forgetting devices.

## Bluetooth page (src/settings/panels/bluetooth.rs)

# Said in red under the Bluetooth switch when switching it on failed for no known reason.
settings-bluetooth-on-failed = Bluetooth could not be switched on.
# Said in red under the Bluetooth switch when switching it off failed for no known reason.
settings-bluetooth-off-failed = Bluetooth could not be switched off.
# Said in red when Bluetooth refuses something because it is not switched on; the first sentence says what could not be done.
settings-bluetooth-cause-not-ready = { $could_not } Bluetooth is not switched on.
# Said in red when the PIN or number typed was wrong; the first sentence says what could not be done.
settings-bluetooth-cause-auth-failed = { $could_not } The PIN or the number was not accepted. Try again.
# Said in red when the pairing was turned down; the first sentence says what could not be done.
settings-bluetooth-cause-turned-down = { $could_not } It was turned down, here or on the other device.
# Said in red when the other device did not answer in time; the first sentence says what could not be done.
settings-bluetooth-cause-timeout = { $could_not } The other device did not answer in time. Put it in pairing mode, then try again.
# Said in red when the same thing is already being done; the first sentence says what could not be done.
settings-bluetooth-cause-in-progress = { $could_not } It is already being done. Wait a moment.
# Said in red when the device did not answer; the first sentence says what could not be done.
settings-bluetooth-cause-no-answer = { $could_not } The device did not answer. Check it is switched on and near, then try again.
# Said in red when the device is not ready to be used; the first sentence says what could not be done.
settings-bluetooth-cause-not-available = { $could_not } The device is not ready to be used. Put it in pairing mode, then try again.
# The word for the system bus in "The system bus is not available".
settings-bluetooth-the-bus = The system bus
# Headline of the page while Bluetooth is looked for.
settings-bluetooth-top-looking = Looking for Bluetooth…
# Headline of the page when the Bluetooth service is not running.
settings-bluetooth-top-gone = Bluetooth is not running on this computer.
# Headline of the page when Bluetooth is off.
settings-bluetooth-top-off = Bluetooth is off.
# Headline of the page when Bluetooth is off at a switch or key on the computer itself.
settings-bluetooth-top-hard = Bluetooth is switched off on the computer itself.
# Headline of the page when the computer has no Bluetooth.
settings-bluetooth-top-none = This computer has no Bluetooth.
# Headline of the page when Bluetooth is on and searching.
settings-bluetooth-top-searching = On, looking for devices nearby…
# Headline of the page when Bluetooth is on and nothing is connected.
settings-bluetooth-top-idle = On, with nothing connected just now.
# Headline of the page when Bluetooth is on and devices are connected; the names are already joined into a list.
settings-bluetooth-top-connected =
    { $n ->
        [one] On, and { $names } is connected.
       *[other] On, and { $names } are connected.
    }
# Help text (i button) of the Bluetooth card.
settings-bluetooth-card-about = The switch turns this computer's Bluetooth on or off; under it is the adapter's name. While Visible to other devices is on, nearby devices can find this computer.
# Heading of the Bluetooth card, of its switch's line, and the name of its switch.
settings-bluetooth-title = Bluetooth
# Small line under the Bluetooth switch when Bluetooth is blocked by the system.
settings-bluetooth-switched-off = Switched off.
# Heading of the line with the switch that makes this computer visible, and the name of that switch.
settings-bluetooth-visible = Visible to other devices
# Small line under the Visible to other devices switch.
settings-bluetooth-visible-sub = Nearby devices can find this computer.
# Heading of the card when Bluetooth is off at a switch on the computer itself.
settings-bluetooth-hard-line = Switched off on the computer itself
# Small line under it.
settings-bluetooth-hard-line-sub = By a switch or a key. Turn that on first.
# Heading of the card when the computer has no Bluetooth adapter.
settings-bluetooth-no-adapter = No Bluetooth adapter
# Help text (i button) of that card.
settings-bluetooth-no-adapter-about = No Bluetooth adapter was found in this computer. One that plugs into a USB socket shows here as soon as it is plugged in.
# Line in that card.
settings-bluetooth-none-found = None was found
# Small line under it.
settings-bluetooth-none-found-sub = A USB one shows here once plugged in.
# Shown beside the Bluetooth switch while the password is asked for.
settings-bluetooth-waiting = Waiting for the password...
# Said in red when this computer could not be made visible to other devices.
settings-bluetooth-visible-failed = This computer could not be made visible to other devices.
# Said in red when this computer could not be hidden from other devices.
settings-bluetooth-hidden-failed = This computer could not be hidden from other devices.
# Shown on the two device lists while Bluetooth is off.
settings-bluetooth-switch-on-first = Switch on Bluetooth to use these.
# Line in the My devices card when nothing is paired.
settings-bluetooth-none-paired = None paired yet
# Small line under it.
settings-bluetooth-none-paired-sub = Look for devices to pair one.
# Heading of the card that lists paired devices.
settings-bluetooth-mine = My devices
# Help text (i button) of the My devices card.
settings-bluetooth-mine-about = Devices paired with this computer, each with its battery where it says. Those connected when the page opens come first, and a line stays where it is while the page is open. The round buttons connect or disconnect one; the bin forgets it, so it has to be paired again.
# Line in the Nearby card while the search runs.
settings-bluetooth-looking = Looking for devices...
# Tip of the button that stops the search.
settings-bluetooth-stop-looking = Stop looking
# Heading of the line that starts the search, and tip of its button.
settings-bluetooth-look = Look for devices
# Small line under it.
settings-bluetooth-look-sub = Put the other one in pairing mode first.
# Heading of the card that lists devices nearby.
settings-bluetooth-nearby = Nearby
# Help text (i button) of the Nearby card.
settings-bluetooth-nearby-about = Press Look for devices, put the other device into pairing mode, then press Pair beside it. Devices nearby are listed strongest first as the page opens, and each new one found is added at the end. Some show a number to check against the one here, and some ask for a PIN, which is often in their instructions, or 0000.
# Said in red when the search could not be stopped.
settings-bluetooth-stop-failed = Looking for devices could not be stopped.
# Said in red when the search could not be started.
settings-bluetooth-start-failed = Devices could not be looked for.
# Small line of a device that is connected.
settings-bluetooth-connected = Connected
# Small line of a paired device that is not connected.
settings-bluetooth-not-connected = Not connected
# Part of a device's small line: its battery level in per cent.
settings-bluetooth-battery = battery { $n }%
# Tip of the button that pairs a device, and the button of the same name in the pairing question.
settings-bluetooth-pair = Pair
# Tip of the button that disconnects a device.
settings-bluetooth-disconnect = Disconnect
# Tip of the button that connects a device.
settings-bluetooth-connect = Connect
# Tip of the bin button that forgets a device.
settings-bluetooth-forget = Forget this device
# Said in red when a device could not be disconnected.
settings-bluetooth-disconnect-failed = It could not be disconnected.
# Said in red when a device could not be forgotten.
settings-bluetooth-forget-failed = It could not be forgotten.
# Shown under a device while it is being paired.
settings-bluetooth-pairing = Pairing...
# Said in red when a device could not be paired.
settings-bluetooth-pair-failed = It could not be paired.
# Shown under a device while it is being connected.
settings-bluetooth-connecting = Connecting...
# Said in red when a device could not be connected.
settings-bluetooth-connect-failed = It could not be connected.
# Said under a device that paired but could not be set to connect by itself.
settings-bluetooth-not-trusted = Connected, but it could not be set to connect by itself next time. Connect it here when you want it.
# Said under a device that shows a code to be typed on it; the code is already written.
settings-bluetooth-type-code = Type { $code } on the device, then press Enter on it.
# Question under a device that shows a number to compare with the one on this computer.
settings-bluetooth-compare = Does the other device show { $code }?
# Tip of the button that answers no to a pairing question.
settings-bluetooth-no = No
# Tip of the button that says the numbers match.
settings-bluetooth-yes-matches = Yes, it matches
# Question under a device that asks to be paired.
settings-bluetooth-pair-question = Pair with this device?
# Placeholder of the box where a device's PIN is typed.
settings-bluetooth-pin = PIN
# Tip of the button that refuses to type a PIN.
settings-bluetooth-cancel = Cancel
# Help line above the box where a device's PIN is typed.
settings-bluetooth-pin-help = Type the PIN for this device. It is often in its instructions, or 0000.
# Words the settings search looks for on this page (not shown).
settings-bluetooth-keywords = headphones mouse keyboard pair devices wireless
# Help text at the top of the page.
settings-bluetooth-help = Press Search, put the other device into pairing mode, then press Pair beside it.

# Said in red under the Bluetooth switch when restarting the sound service after Bluetooth came back on failed; $why is the system's own reason.
settings-bluetooth-sound-failed = Bluetooth sound could not be restarted, so headphones may use basic quality until you log in again. { $why }
# Tooltip of the help button beside the Bluetooth switch.
settings-bluetooth-sound-help = Why headphones sound basic after Bluetooth is switched off and on
# Said in red when a device is no longer in BlueZ's list; the first sentence says what could not be done.
settings-bluetooth-cause-gone = { $could_not } It is no longer known. Look for it again.
# Said in red when a paired device's saved pairing is refused; the first sentence says what could not be done.
settings-bluetooth-cause-old-pairing = { $could_not } The saved pairing is not accepted any more. Forget the device and pair it again.
# Said in red under a device that connected and then dropped at once.
settings-bluetooth-dropped = It connected, then dropped. The saved pairing may be out of date.
# Tooltip of the button that forgets a paired device and looks for it afresh.
settings-bluetooth-pair-again = Forget and pair again
# Said under Look for devices after a device was forgotten to be paired afresh.
settings-bluetooth-forgotten-pair = Forgotten. Put it in pairing mode, then press Pair when it shows below.
# Tooltip of the button that opens Help from a failed connection.
settings-bluetooth-more = More about this
