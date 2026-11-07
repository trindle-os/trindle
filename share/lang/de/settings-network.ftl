
# machine translated (German), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } ist über einen Schalter oder eine Taste am Computer selbst ausgeschaltet. Schalte das wieder ein und versuche es dann noch einmal.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } wird vom System blockiert, und die Sperre konnte nicht aufgehoben werden.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Dieses Passwort wurde nicht akzeptiert. Versuche es noch einmal.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Das für dieses Netzwerk gespeicherte Passwort wurde nicht akzeptiert. Vergiss das Netzwerk mit dem Papierkorb und verbinde dich dann erneut.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Dieses Netzwerk wurde nicht gefunden. Prüfe den Namen und ob es in Reichweite ist.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Das Netzwerk hat diesen Computer hineingelassen, ihm aber keine Adresse gegeben. Versuche es noch einmal oder starte den Router neu.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Das Netzwerk hat diesen Computer nicht hineingelassen. Gehe näher heran und versuche es dann noch einmal.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Dieses Passwort kann ein Wi-Fi-Netzwerk nicht haben. Prüfe es und versuche es dann noch einmal.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Mit dem Netzwerk konnte keine Verbindung hergestellt werden.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = mit deinem Passwort zu öffnen

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = mit deinem Passwort zu öffnen und zu ändern

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = jeder zu Hause kann öffnen

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = jeder zu Hause kann ändern

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
        [strong] Verbunden mit { $name }, starkes Signal.
        [good] Verbunden mit { $name }, gutes Signal.
        [fair] Verbunden mit { $name }, mittleres Signal.
       *[other] Verbunden mit { $name }, schwaches Signal.
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
settings-network-top-gone = Netzwerke können nicht angezeigt werden: NetworkManager läuft nicht.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Der Systembus

# Name of the page in the list of settings pages and its heading.
settings-network-title = Netzwerk

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wlan drahtlos internet ethernet kabel passwort verbinden firewall freigabe teilen drucker ordner

# Help text at the top of the page.
settings-network-help = Schalte Wi-Fi ein oder aus, verbinde dich mit einem Netzwerk, vergiss eines, das du nicht mehr willst, oder lass andere Computer zu.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Starkes Signal von { $name }.
        [good] Gutes Signal von { $name }.
        [fair] Mittleres Signal von { $name }.
       *[other] Schwaches Signal von { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Sein Chip braucht ein neueres Linux.

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
settings-network-wifi-about = Wi-Fi verbindet diesen Computer ohne Kabel mit einem Netzwerk. Ausgeschaltet sucht es keine Netzwerke mehr und spart etwas Akku. Ein Netzwerk, das seinen Namen nicht aussendet, erscheint nie in der Liste der Netzwerke in Reichweite: Verbinde dich hier, indem du den Namen genau eintippst, auch Groß- und Kleinschreibung. Wenn der Wi-Fi-Chip dieses Computers einer ist, den noch kein Treiber in seinem Linux unterstützt, braucht er ein neueres Linux als das, auf dem Trindle läuft. Ein Netzwerkkabel hat eine eigene Zeile mit einer Taste zum Verbinden oder Trennen.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Andere Computer zulassen

# Small line under that switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch-sub = Um zu Hause Ordner und Drucker freizugeben.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Freigegebene Ordner werden gesucht …

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Freigegebene Ordner konnten nicht gelesen werden

# Line in the Sharing card when folders are shared but the switch is off.
# SAFETY: needs a human review before it ships
settings-network-not-reachable = Noch nicht erreichbar

# Small line under it.
# SAFETY: needs a human review before it ships
settings-network-not-reachable-sub = Andere können diese nicht öffnen, solange es aus ist.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Es sind keine Ordner freigegeben

# Small line under it.
settings-network-no-shared-sub = Klicke in Ordner mit der rechten Maustaste auf einen, um ihn freizugeben.

# Heading of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing = Freigabe

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Solange „Andere Computer zulassen“ aus ist, kann nichts im Netzwerk diesen Computer erreichen, außer er hat zuerst gefragt, sodass niemand im selben Wi-Fi sein Glück versuchen kann. Schalte es ein, um einen Ordner oder Drucker mit anderen Computern zu Hause zu teilen. Um einen Ordner freizugeben, klicke in Ordner mit der rechten Maustaste darauf und wähle „Im Netzwerk freigeben“. Jeder sagt, wer was tun darf. Mit deinem Passwort: Andere können die Dateien mit deinem Freigabepasswort öffnen oder öffnen und ändern. Jeder zu Hause: Jeder im Netzwerk kann die Dateien ohne Passwort öffnen oder öffnen und ändern. Andere Computer zu Hause können keinen davon öffnen, solange der Schalter aus ist.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Wird beendet …

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Warten auf das Passwort ...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Freigabe erlaubt

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Ausgesperrt

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi konnte nicht eingeschaltet werden.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi konnte nicht ausgeschaltet werden.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Trennen

# Tip of the button that connects a network or a cable.
settings-network-connect = Verbinden

# Shown under a network or cable while it is being connected.
settings-network-connecting = Verbindung wird hergestellt ...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Keine Netzwerke in Reichweite

# Small line under it.
settings-network-none-in-range-sub = Wird jedes Mal neu gesucht, wenn diese Seite geöffnet wird.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Netzwerke in Reichweite

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Jedes Wi-Fi-Netzwerk, das dieser Computer hören kann. Beim Öffnen der Seite steht das benutzte zuerst, dann die hier gespeicherten, dann die übrigen nach Signalstärke. Eine Zeile bleibt an ihrem Platz, solange die Seite geöffnet ist. Drücke die Pfeile, um dich mit einem zu verbinden. Ein gesichertes Netzwerk fragt beim ersten Mal nach seinem Passwort. Der Papierkorb vergisst ein gespeichertes Netzwerk und sein Passwort. Ein offenes Netzwerk ist nicht verschlüsselt, daher könnten andere in der Nähe sehen, was darüber gesendet wird.

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
settings-network-forget-failed = Das Netzwerk konnte nicht vergessen werden.

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
settings-network-joining = Verbindung wird hergestellt ...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi-Passwörter haben mindestens 8 Zeichen.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Mit einem versteckten Netzwerk verbinden

# Small line under it.
settings-network-hidden-sub = Tippe den Namen genau ein, auch Groß- und Kleinschreibung.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Netzwerkname

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Passwort, falls es eines hat

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Der Name des Netzwerks wird benötigt.

# Shown under a network after it was joined.
settings-network-joined = Verbunden

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Es wurde trotzdem gespeichert: Vergiss es mit dem Papierkorb, bevor du dich erneut verbindest.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Die Verbindung ließ sich nicht trennen.
