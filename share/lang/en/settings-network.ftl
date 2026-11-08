# Settings, Network page and the radio words shared with Bluetooth.

## Radio switches (src/common/rfkill.rs)

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } is switched off by a switch or a key on the computer itself. Turn that on, then try again.
# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } is blocked by the system, and the block could not be taken off.

## Network page (src/settings/panels/network.rs)

# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = That password was not accepted. Try it again.
# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = The password kept for this network was not accepted. Forget the network with the bin, then join it again.
# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = That network could not be found. Check its name, and that it is in range.
# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = The network let this computer in but gave it no address. Try again, or restart the router.
# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = The network did not let this computer join. Move closer to it, then try again.
# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = That password is not one a Wi-Fi network can have. Check it, then try again.
# Said in red when joining a network failed for no known reason.
settings-network-join-refused = The network could not be joined.
# Small line of a shared folder: who may open it with a password.
settings-network-access-open = open with your password
# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = open and change with your password
# Small line of a shared folder: anyone at home may open it.
settings-network-access-guests-open = anyone at home can open
# Small line of a shared folder: anyone at home may change it.
settings-network-access-guests-change = anyone at home can change
# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }
# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = Stop sharing { $name }
# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Unmanaged
# State of a network device that cannot be used.
settings-network-state-unavailable = Unavailable
# State of a network device that is not connected.
settings-network-state-disconnected = Disconnected
# State of a network device that is getting ready to connect.
settings-network-state-preparing = Preparing
# State of a network device that is being set up.
settings-network-state-configuring = Configuring
# State of a network device that needs a password.
settings-network-state-needs-password = Needs a password
# State of a network device that is getting an address.
settings-network-state-address = Getting an address
# State of a network device whose connection is being checked.
settings-network-state-checking = Checking
# State of a network device that is waiting.
settings-network-state-waiting = Waiting
# State of a network device that is connected.
settings-network-state-connected = Connected
# State of a network device that is disconnecting.
settings-network-state-disconnecting = Disconnecting
# State of a network device that failed.
settings-network-state-failed = Failed
# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Connected by cable, and to { $name } over Wi-Fi.
# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Connected to { $name }, strong signal.
        [good] Connected to { $name }, good signal.
        [fair] Connected to { $name }, fair signal.
       *[other] Connected to { $name }, weak signal.
    }
# Headline of the page when connected by cable only.
settings-network-top-cable = Connected by cable.
# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi is off, so this computer is not connected.
# Headline of the page when not connected.
settings-network-top-none = Not connected. Choose a network below.
# Headline of the page while the networks are looked for.
settings-network-top-looking = Looking for networks…
# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Networks cannot be looked at: NetworkManager is not running.
# The word for the system bus in "The system bus is not available".
settings-network-the-bus = The system bus
# Name of the page in the list of settings pages and its heading.
settings-network-title = Network
# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wireless internet ethernet wired password connect firewall sharing share printer folder
# Help text at the top of the page.
settings-network-help = Turn Wi-Fi on or off, join a network, forget one you no longer want, or let other computers connect.
# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Strong signal from { $name }.
        [good] Good signal from { $name }.
        [fair] Fair signal from { $name }.
       *[other] Weak signal from { $name }.
    }
# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Its chip needs a newer Linux.
# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = This computer has no working Wi-Fi.
# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Off. Switch it on to join a network.
# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = On, and not joined to a network.
# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Use Wi-Fi
# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi
# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi and cable
# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi joins this computer to a network without a cable. Turned off, it stops looking for networks and saves a little battery. A network that does not broadcast its name never appears in the list of networks in range: join it here by typing its name exactly, including capital letters. When this computer's Wi-Fi chip is one no driver in its Linux has taken yet, it needs a newer Linux than the one Trindle runs on. A network cable has a line of its own, with a button to connect or disconnect it.
# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = Let other computers connect
# Small line under that switch.
settings-network-share-switch-sub = To share folders and printers at home.
# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Looking for shared folders…
# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Shared folders could not be read
# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Not reachable yet
# Small line under it.
settings-network-not-reachable-sub = Others cannot open these while it is off.
# Line in the Sharing card when no folder is shared.
settings-network-no-shared = No folders are shared
# Small line under it.
settings-network-no-shared-sub = Right-click one in Folders to share it.
# Heading of the Sharing card.
settings-network-sharing = Sharing
# Help text (i button) of the Sharing card.
settings-network-sharing-about = While Let other computers connect is off, nothing on the network can reach this computer unless it asked first, so nobody on the same Wi-Fi can try their luck. Turn it on to share a folder or a printer with other computers at home. To share a folder, right-click it in Folders and choose Share on the network. Each says who may do what. With your password: others can open the files, or open and change them, with your sharing password. Anyone at home: anyone on the network can open the files, or open and change them, without a password. Other computers at home cannot open any of them while the switch is off.
# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Stopping…
# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = Waiting for the password...
# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = Sharing allowed
# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Kept out
# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi could not be switched on.
# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi could not be switched off.
# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet
# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Disconnect
# Tip of the button that connects a network or a cable.
settings-network-connect = Connect
# Shown under a network or cable while it is being connected.
settings-network-connecting = Connecting...
# Line in the Networks in range card when there are none.
settings-network-none-in-range = No networks in range
# Small line under it.
settings-network-none-in-range-sub = Looked for again each time this page opens.
# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Networks in range
# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Every Wi-Fi network this computer can hear. As the page opens, the one in use is first, then those saved here, then the rest by the strength of their signal; a line stays where it is while the page is open. Press the arrows to join one; a secured network asks for its password the first time. The bin forgets a saved network and its password. An open network is not encrypted, so others nearby could see what is sent over it.
# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Switch on Wi-Fi to see the networks in range.
# Part of a network's small line: it is saved on this computer.
settings-network-saved = Saved
# Part of a network's small line: it needs a password.
settings-network-secured = Secured
# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = Open, not encrypted
# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s
# Tip of the bin button that forgets a saved network.
settings-network-forget = Forget this network
# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = The network could not be forgotten.
# Placeholder of the box where a network's password is typed.
settings-network-password = Password
# Tip of the button that joins the network.
settings-network-join = Join
# Tip of the button that closes the box without joining.
settings-network-cancel = Cancel
# Shown under a network while it is being joined.
settings-network-joining = Joining...
# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Wi-Fi passwords are at least 8 characters.
# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Join a hidden network
# Small line under it.
settings-network-hidden-sub = Type its name exactly, capitals too.
# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Network name
# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = Password, if it has one
# Said in red when no name was typed for a hidden network.
settings-network-name-needed = The network's name is needed.
# Shown under a network after it was joined.
settings-network-joined = Joined
# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } It was saved all the same: forget it with the bin before joining again.
# Said in red when a network would not disconnect.
settings-network-disconnect-failed = It would not disconnect.
