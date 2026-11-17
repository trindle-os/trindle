
# machine translated (German), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } ist über einen Schalter oder eine Taste am Computer selbst ausgeschaltet. Schalte das wieder ein und versuche es dann noch einmal.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = Das System blockiert { $radio }, und Trindle konnte die Sperre nicht aufheben.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Dieses Passwort wurde nicht akzeptiert. Versuche es noch einmal.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Das gespeicherte Passwort für dieses Netzwerk wurde nicht akzeptiert. Vergiss das Netzwerk mit der Papierkorb-Taste und verbinde dich dann erneut.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle konnte dieses Netzwerk nicht finden. Prüfe den Namen und ob es in Reichweite ist.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Dieser Computer hat sich mit dem Netzwerk verbunden, aber keine Adresse bekommen. Versuche es noch einmal oder starte den Router neu.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Dieser Computer konnte sich nicht mit dem Netzwerk verbinden. Gehe näher an den Router heran und versuche es dann noch einmal.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Das kann kein Wi-Fi-Passwort sein. Prüfe es und versuche es dann noch einmal.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Die Verbindung mit dem Netzwerk ließ sich nicht herstellen.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = mit deinem Passwort zu öffnen

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = mit deinem Passwort zu öffnen und zu ändern

# Small line of a shared folder: anyone on the local network may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = jeder im lokalen Netzwerk kann öffnen

# Small line of a shared folder: anyone on the local network may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = jeder im lokalen Netzwerk kann ändern

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = Freigabe von { $name } beenden

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Nicht verwaltet

# State of a network device that cannot be used.
settings-network-state-unavailable = Nicht verfügbar

# State of a network device that is not connected.
settings-network-state-disconnected = Getrennt

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Wird vorbereitet

# State of a network device that is being set up.
settings-network-state-configuring = Wird eingerichtet

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = Braucht ein Passwort

# State of a network device that is getting an address.
settings-network-state-address = Adresse wird bezogen

# State of a network device whose connection is being checked.
settings-network-state-checking = Wird geprüft

# State of a network device that is waiting.
settings-network-state-waiting = Wartet

# State of a network device that is connected.
settings-network-state-connected = Verbunden

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Wird getrennt

# State of a network device that failed.
settings-network-state-failed = Fehlgeschlagen

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Per Kabel verbunden und über Wi-Fi mit { $name }.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Verbunden mit { $name }. Das Signal ist stark.
        [good] Verbunden mit { $name }. Das Signal ist gut.
        [fair] Verbunden mit { $name }. Das Signal ist mittelstark.
       *[other] Verbunden mit { $name }. Das Signal ist schwach.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Per Kabel verbunden.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi ist aus, daher ist dieser Computer nicht verbunden.

# Headline of the page when not connected.
settings-network-top-none = Nicht verbunden. Wähle unten ein Netzwerk.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Netzwerke werden gesucht …

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Trindle kann keine Netzwerke anzeigen, weil NetworkManager nicht läuft.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Der Systembus

# Name of the page in the list of settings pages and its heading.
settings-network-title = Netzwerk

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wlan drahtlos internet ethernet kabel passwort verbinden firewall freigabe teilen drucker ordner

# Help text at the top of the page.
settings-network-help = Schalte Wi-Fi ein oder aus, verbinde dich mit einem Netzwerk, vergiss ein Netzwerk, das du nicht mehr willst, oder lass andere Computer zu.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Starkes Signal von { $name }.
        [good] Gutes Signal von { $name }.
        [fair] Mittleres Signal von { $name }.
       *[other] Schwaches Signal von { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi braucht ein neueres Linux.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Dieser Computer hat kein funktionierendes Wi-Fi.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Aus. Schalte es ein, um dich mit einem Netzwerk zu verbinden.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = An, aber mit keinem Netzwerk verbunden.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi verwenden

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi und Kabel

# Help text (i button) of the Wi-Fi card.
# SAFETY: needs a human review before it ships
settings-network-wifi-about = Wi-Fi verbindet diesen Computer ohne Kabel mit einem Netzwerk. Wenn du es ausschaltest, sucht es keine Netzwerke mehr und spart etwas Akku. Ein Netzwerk, das seinen Namen verbirgt, erscheint nie in der Liste der Netzwerke in Reichweite. Um dich damit zu verbinden, tippe den Namen genau ein, auch Groß- und Kleinschreibung. Manche Wi-Fi-Chips brauchen einen Treiber, den die Linux-Version von Trindle noch nicht hat. Dafür brauchst du eine neuere Linux-Version. Ein Netzwerkkabel hat eine eigene Zeile mit einer Taste zum Verbinden oder Trennen.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Andere Computer zulassen

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Freigegebene Ordner werden gesucht …

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Die freigegebenen Ordner konnten nicht gelesen werden

# Line in the Sharing card when folders are shared but the switch is off.
# SAFETY: needs a human review before it ships
settings-network-not-reachable = Noch nicht erreichbar

# Small line under it.
# SAFETY: needs a human review before it ships
settings-network-not-reachable-sub = Andere Computer können diese nicht öffnen, solange die Freigabe aus ist.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Es sind keine Ordner freigegeben

# Heading of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing = Freigabe

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Solange „Andere Computer zulassen“ aus ist, können andere Computer im Netzwerk diesen nur erreichen, wenn dieser Computer sie zuerst kontaktiert hat. So kann niemand im selben Wi-Fi versuchen, sich Zugang zu verschaffen. Schalte es ein, um einen Ordner oder Drucker mit anderen Computern zu Hause zu teilen. Um einen Ordner freizugeben, klicke in Folders mit der rechten Maustaste darauf und wähle „Im Netzwerk freigeben“. Jeder freigegebene Ordner zeigt, wer was tun darf. Mit deinem Passwort: Andere können die Dateien öffnen oder öffnen und ändern, wenn sie dein Freigabepasswort haben. Jeder zu Hause: Jeder im Netzwerk kann die Dateien ohne Passwort öffnen oder öffnen und ändern. Solange der Schalter aus ist, können andere Computer zu Hause keinen davon öffnen.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Wird beendet …

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Warte auf dein Passwort …

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Freigabe ist an

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Blockiert

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi ließ sich nicht einschalten.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi ließ sich nicht ausschalten.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Trennen

# Tip of the button that connects a network or a cable.
settings-network-connect = Verbinden

# Shown under a network or cable while it is being connected.
settings-network-connecting = Verbindung wird hergestellt …

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Keine Netzwerke in Reichweite

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Netzwerke in Reichweite

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Jedes Wi-Fi-Netzwerk, das dieser Computer empfangen kann. Beim Öffnen der Seite steht das Netzwerk, das du benutzt, zuerst, dann die gespeicherten, dann die übrigen nach Signalstärke. Die Reihenfolge ändert sich nicht, solange die Seite geöffnet ist. Drücke den Pfeil, um dich mit einem Netzwerk zu verbinden. Ein gesichertes Netzwerk fragt beim ersten Mal nach seinem Passwort. Die Papierkorb-Taste vergisst ein gespeichertes Netzwerk und sein Passwort. Ein offenes Netzwerk ist nicht verschlüsselt, daher könnten Leute in der Nähe sehen, was du darüber sendest.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Schalte Wi-Fi ein, um die Netzwerke in Reichweite zu sehen.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Gespeichert

# Part of a network's small line: it needs a password.
settings-network-secured = Gesichert

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = Offen, nicht verschlüsselt

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Dieses Netzwerk vergessen

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Das Netzwerk ließ sich nicht vergessen.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Passwort

# Tip of the button that joins the network.
# SAFETY: needs a human review before it ships
settings-network-join = Verbinden

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Abbrechen

# Shown under a network while it is being joined.
settings-network-joining = Verbindung wird hergestellt …

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi-Passwörter haben mindestens 8 Zeichen.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Mit einem versteckten Netzwerk verbinden

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Netzwerkname

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Passwort, falls es eines hat

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Tippe den Namen des Netzwerks ein.

# Shown under a network after it was joined.
settings-network-joined = Verbunden

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Das Netzwerk wurde trotzdem gespeichert. Vergiss es mit der Papierkorb-Taste, bevor du dich erneut verbindest.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Die Verbindung ließ sich nicht trennen.

# Heading of the card that lists the ports opened by hand.
settings-network-ports = Offene Ports

# Help text (i button) of the Open ports card.
settings-network-ports-about = Jeder Port ist geschlossen, außer dieser Computer hat zuerst den anderen kontaktiert. Ein Programm, das auf Verbindungen von anderen wartet, etwa ein Spieleserver oder ein Sync-Werkzeug, ist erst erreichbar, wenn du hier seinen Port öffnest. Ein offener Port lässt jeden Computer im selben lokalen Netzwerk wie dieser herein, nie das weite Internet. In einem Café-, Hotel- oder Büronetzwerk gehören dazu auch alle anderen dort. Zum Öffnen oder Schließen eines Ports brauchst du das Passwort des Administrators.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Keine Ports geöffnet

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = { $port } schließen

# Heading of the line that opens a port.
settings-network-port-add = Einen Port öffnen

# Placeholder in the box where the port number is typed.
settings-network-port-number = Portnummer

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Protokoll

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Beide

# Button that opens the typed port.
settings-network-port-open = Öffnen

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Gib eine Zahl von 1 bis 65535 ein.

# Red words when the port is already open.
settings-network-port-already = Schon offen.

# Said under the line once the port is open.
settings-network-port-opened = Geöffnet

settings-network-username = Braucht einen Benutzernamen

settings-network-username-unsupported = Dieses Netzwerk braucht außer dem Passwort auch einen Benutzernamen. Trindle kann sich noch nicht mit solchen Netzwerken verbinden.
