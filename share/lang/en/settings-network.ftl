# Settings, Network page and the radio words shared with Bluetooth.

## Radio switches (src/common/rfkill.rs)

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } is switched off by a switch or key on the computer itself. Switch it on, then try again.
# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = The system is blocking { $radio }, and Trindle couldn't remove the block.

## Network page (src/settings/panels/network.rs)

# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = That password wasn't accepted. Try it again.
# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = The saved password for this network wasn't accepted. Use the bin button to forget the network, then join it again.
# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle couldn't find that network. Check its name, and check it is in range.
# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = This computer joined the network but didn't get an address. Try again, or restart the router.
# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = This computer couldn't join the network. Move closer to the router, then try again.
# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = That can't be a Wi-Fi password. Check it, then try again.
# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Couldn't join the network.
# Small line of a shared folder: who may open it with a password.
settings-network-access-open = open with your password
# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = open and change with your password
# Small line of a shared folder: anyone on the local network may open it.
settings-network-access-guests-open = anyone nearby can open
# Small line of a shared folder: anyone on the local network may change it.
settings-network-access-guests-change = anyone nearby can change
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
settings-network-state-configuring = Setting up
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
        [strong] Connected to { $name }. The signal is strong.
        [good] Connected to { $name }. The signal is good.
        [fair] Connected to { $name }. The signal is fair.
       *[other] Connected to { $name }. The signal is weak.
    }
# Headline of the page when connected by cable only.
settings-network-top-cable = Connected by cable.
# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi is off, so this computer isn't connected.
# Headline of the page when not connected.
settings-network-top-none = Not connected. Choose a network below.
# Headline of the page while the networks are looked for.
settings-network-top-looking = Looking for networks…
# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Trindle can't show networks because NetworkManager isn't running.
# The word for the system bus in "The system bus is not available".
settings-network-the-bus = The system bus
# Name of the page in the list of settings pages and its heading.
settings-network-title = Network
# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wireless internet ethernet wired password connect firewall sharing share printer folder
# Help text at the top of the page.
settings-network-help = Switch Wi-Fi on or off, join a network, forget a network you no longer want, or let other computers connect.
# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Strong signal from { $name }.
        [good] Good signal from { $name }.
        [fair] Fair signal from { $name }.
       *[other] Weak signal from { $name }.
    }
# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi needs a newer Linux.
# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = This computer has no working Wi-Fi.
# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Off. Switch it on to join a network.
# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = On, but not on a network.
# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Use Wi-Fi
# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi
# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi and cable
# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi connects this computer to a network without a cable. If you switch it off, it stops looking for networks and saves a little battery. A network that hides its name never appears in the list of networks in range. To join one, type its name exactly, including capital letters. Some Wi-Fi chips need a driver that the version of Linux Trindle uses doesn't have yet. For those, you need a newer version of Linux. A network cable has its own line, with a button to connect or disconnect it.
# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = Let other computers connect
# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Looking for shared folders…
# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Couldn't read the shared folders
# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Not reachable yet
# Small line under it.
settings-network-not-reachable-sub = Other computers can't open these while sharing is off.
# Line in the Sharing card when no folder is shared.
settings-network-no-shared = No folders are shared
# Heading of the Sharing card.
settings-network-sharing = Sharing
# Help text (i button) of the Sharing card.
settings-network-sharing-about = While Let other computers connect is off, other computers on the network can only reach this one if this computer contacted them first. This stops anyone on the same Wi-Fi from trying to get in. Switch it on to share a folder or printer with other computers at home. To share a folder, right-click it in Folders and choose Share on the network. Each shared folder shows who can do what. With your password: others can open the files, or open and change them, if they have your sharing password. Anyone at home: anyone on the network can open the files, or open and change them, without a password. While the switch is off, other computers at home can't open any of them.
# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Stopping…
# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = Waiting for your password…
# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = Sharing is on
# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = Blocked
# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Couldn't switch Wi-Fi on.
# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Couldn't switch Wi-Fi off.
# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet
# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Disconnect
# Tip of the button that connects a network or a cable.
settings-network-connect = Connect
# Shown under a network or cable while it is being connected.
settings-network-connecting = Connecting…
# Line in the Networks in range card when there are none.
settings-network-none-in-range = No networks in range
# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Networks in range
# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Every Wi-Fi network this computer can pick up. When you open the page, the network you are using comes first, then saved networks, then the rest by signal strength. The order doesn't change while the page is open. Press the arrow to join a network. A secured network asks for its password the first time. The bin button forgets a saved network and its password. An open network isn't encrypted, so people nearby could see what you send over it.
# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Switch on Wi-Fi to see the networks in range.
# Part of a network's small line: it is saved on this computer.
settings-network-saved = Saved
# Part of a network's small line: it needs a password.
settings-network-secured = Secured
# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = Needs a username
# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = This network needs a username as well as a password. Trindle can't join those yet.
# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = Open, not encrypted
# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s
# Tip of the bin button that forgets a saved network.
settings-network-forget = Forget this network
# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Couldn't forget the network.
# Placeholder of the box where a network's password is typed.
settings-network-password = Password
# Tip of the button that joins the network.
settings-network-join = Join
# Tip of the button that closes the box without joining.
settings-network-cancel = Cancel
# Shown under a network while it is being joined.
settings-network-joining = Joining…
# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Wi-Fi passwords have at least 8 characters.
# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Join a hidden network
# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Network name
# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = Password, if it has one
# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Type the network's name.
# Shown under a network after it was joined.
settings-network-joined = Joined
# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } The network was saved anyway. Forget it with the bin button before joining again.
# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Couldn't disconnect.

# Heading of the card that lists the ports opened by hand.
settings-network-ports = Open ports

# Help text (i button) of the Open ports card.
settings-network-ports-about = Every port is closed unless this computer contacted the other one first. A program that waits for others to connect, such as a game server or a sync tool, can't be reached until you open its port here. An open port lets in any computer on the same local network as this one, but never the wider internet. On a café, hotel or office network, that includes everyone else there. To open or close a port, you need the administrator's password.

# Line in the Open ports card when no port is open.
settings-network-ports-none = No ports opened



# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = Close { $port }

# Heading of the line that opens a port.
settings-network-port-add = Open a port


# Placeholder in the box where the port number is typed.
settings-network-port-number = Port number

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Protocol

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Both

# Button that opens the typed port.
settings-network-port-open = Open

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Type a number from 1 to 65535.

# Red words when the port is already open.
settings-network-port-already = Already open.

# Said under the line once the port is open.
settings-network-port-opened = Opened
