# machine translated (Russian), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } выключен переключателем или клавишей на самом компьютере. Включите его и повторите попытку.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = Система блокирует { $radio }, и Trindle не смог снять блокировку.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Пароль не принят. Попробуйте ещё раз.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Сохранённый пароль для этой сети не принят. Забудьте сеть кнопкой с корзиной и подключитесь заново.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle не нашёл эту сеть. Проверьте её имя и убедитесь, что она в зоне досягаемости.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Компьютер подключился к сети, но не получил адрес. Повторите попытку или перезапустите маршрутизатор.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Компьютеру не удалось подключиться к сети. Подойдите ближе к маршрутизатору и повторите попытку.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Это не может быть паролем Wi-Fi. Проверьте его и повторите попытку.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Не удалось подключиться к сети.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = открывается по вашему паролю

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = открывается и изменяется по вашему паролю

# Small line of a shared folder: anyone on the local network may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = любой рядом может открыть

# Small line of a shared folder: anyone on the local network may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = любой рядом может изменять

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = Закрыть общий доступ к «{ $name }»

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Не управляется

# State of a network device that cannot be used.
settings-network-state-unavailable = Недоступно

# State of a network device that is not connected.
settings-network-state-disconnected = Отключено

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Подготовка

# State of a network device that is being set up.
settings-network-state-configuring = Настройка

# State of a network device that needs a password.
settings-network-state-needs-password = Нужен пароль

# State of a network device that is getting an address.
settings-network-state-address = Получение адреса

# State of a network device whose connection is being checked.
settings-network-state-checking = Проверка

# State of a network device that is waiting.
settings-network-state-waiting = Ожидание

# State of a network device that is connected.
settings-network-state-connected = Подключено

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Отключение

# State of a network device that failed.
settings-network-state-failed = Сбой

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Подключено по кабелю и к сети { $name } по Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Подключено к сети { $name }. Сигнал сильный.
        [good] Подключено к сети { $name }. Сигнал хороший.
        [fair] Подключено к сети { $name }. Сигнал средний.
       *[other] Подключено к сети { $name }. Сигнал слабый.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Подключено по кабелю.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi выключен, поэтому компьютер не подключён.

# Headline of the page when not connected.
settings-network-top-none = Нет подключения. Выберите сеть ниже.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Поиск сетей…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Trindle не может показать сети, потому что NetworkManager не запущен.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Системная шина

# Name of the page in the list of settings pages and its heading.
settings-network-title = Сеть

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi беспроводная интернет ethernet проводная пароль подключение брандмауэр файрвол общий доступ принтер папка

# Help text at the top of the page.
settings-network-help = Включайте и выключайте Wi-Fi, подключайтесь к сети, забывайте ненужные сети или разрешайте другим компьютерам подключаться.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Сильный сигнал от сети { $name }.
        [good] Хороший сигнал от сети { $name }.
        [fair] Средний сигнал от сети { $name }.
       *[other] Слабый сигнал от сети { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Для Wi-Fi нужен более новый Linux.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = На этом компьютере нет работающего Wi-Fi.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Выключено. Включите, чтобы подключиться к сети.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Включено, к сети не подключено.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Использовать Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi и кабель

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi подключает этот компьютер к сети без кабеля. Если его выключить, поиск сетей прекращается, и это немного экономит заряд батареи. Сеть, которая скрывает своё имя, не появляется в списке сетей в зоне досягаемости. Чтобы подключиться к ней, введите её имя точно, включая заглавные буквы. Для некоторых чипов Wi-Fi нужен драйвер, которого пока нет в версии Linux, на которой работает Trindle. В таком случае нужна более новая версия Linux. У сетевого кабеля есть отдельная строка с кнопкой подключения и отключения.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Разрешить другим компьютерам подключаться

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Поиск общих папок…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Не удалось прочитать общие папки

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Пока недоступно

# Small line under it.
settings-network-not-reachable-sub = Пока общий доступ выключен, другие компьютеры не могут их открыть.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Нет общих папок

# Heading of the Sharing card.
settings-network-sharing = Общий доступ

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Пока выключено «Разрешить другим компьютерам подключаться», другие компьютеры в сети могут обратиться к этому, только если он сам обратился к ним первым. Это не даёт никому в той же сети Wi-Fi попытаться проникнуть в него. Включите, чтобы делиться папкой или принтером с другими компьютерами дома. Чтобы открыть доступ к папке, щёлкните её правой кнопкой в Folders и выберите «Поделиться в сети». У каждой общей папки указано, кто и что может с ней делать. По вашему паролю: другие могут открывать файлы или открывать и изменять их, если у них есть ваш пароль общего доступа. Любой в домашней сети: любой в сети может открывать файлы или открывать и изменять их без пароля. Пока переключатель выключен, другие компьютеры дома не могут открыть ни одну из папок.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Остановка…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Ожидание вашего пароля…

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Общий доступ включён

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Заблокировано

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Не удалось включить Wi-Fi.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Не удалось выключить Wi-Fi.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Отключить

# Tip of the button that connects a network or a cable.
settings-network-connect = Подключить

# Shown under a network or cable while it is being connected.
settings-network-connecting = Подключение…

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Нет сетей в зоне досягаемости

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Сети в зоне досягаемости

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Все сети Wi-Fi, которые этот компьютер может поймать. При открытии страницы первой идёт сеть, которой вы пользуетесь, затем сохранённые сети, затем остальные по силе сигнала. Порядок не меняется, пока страница открыта. Нажмите стрелку, чтобы подключиться к сети. Защищённая сеть при первом подключении запросит пароль. Кнопка с корзиной забывает сохранённую сеть и её пароль. Открытая сеть не зашифрована, поэтому люди поблизости могут видеть, что вы по ней отправляете.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Включите Wi-Fi, чтобы увидеть сети в зоне досягаемости.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Сохранена

# Part of a network's small line: it needs a password.
settings-network-secured = Защищена

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = Открытая, не зашифрована

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Мбит/с

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Забыть эту сеть

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Не удалось забыть сеть.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Пароль

# Tip of the button that joins the network.
settings-network-join = Подключиться

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Отмена

# Shown under a network while it is being joined.
settings-network-joining = Подключение…

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Пароль Wi-Fi должен содержать не менее 8 символов.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Подключиться к скрытой сети

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Имя сети

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Пароль, если он есть

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Введите имя сети.

# Shown under a network after it was joined.
settings-network-joined = Подключено

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Сеть всё равно сохранена. Забудьте её кнопкой с корзиной, прежде чем подключаться снова.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Не удалось отключиться.

# Heading of the card that lists the ports opened by hand.
settings-network-ports = Открытые порты

# Help text (i button) of the Open ports card.
settings-network-ports-about = Все порты закрыты, если этот компьютер не обратился к другому первым. До программы, которая ждёт подключений от других, например игрового сервера или средства синхронизации, не достучаться, пока вы не откроете её порт здесь. Открытый порт пускает любой компьютер той же локальной сети, что и этот, но не весь интернет. В сети кафе, отеля или офиса это значит, что и всех остальных, кто там есть. Чтобы открыть или закрыть порт, нужен пароль администратора.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Нет открытых портов

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = Закрыть { $port }

# Heading of the line that opens a port.
settings-network-port-add = Открыть порт

# Placeholder in the box where the port number is typed.
settings-network-port-number = Номер порта

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Протокол

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Оба

# Button that opens the typed port.
settings-network-port-open = Открыть

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Введите число от 1 до 65535.

# Red words when the port is already open.
settings-network-port-already = Уже открыт.

# Said under the line once the port is open.
settings-network-port-opened = Открыт

# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = Нужно имя пользователя

# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = Этой сети нужны имя пользователя и пароль. Trindle пока не умеет подключаться к таким сетям.
