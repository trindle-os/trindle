# machine translated (Russian), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } выключен переключателем или клавишей на самом компьютере. Включите его и повторите попытку.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } заблокирован системой, и блокировку не удалось снять.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Пароль не принят. Попробуйте ещё раз.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Сохранённый пароль для этой сети не принят. Забудьте сеть кнопкой с корзиной и подключитесь заново.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Эта сеть не найдена. Проверьте её имя и то, что она в зоне досягаемости.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Сеть пустила этот компьютер, но не выдала ему адрес. Повторите попытку или перезапустите маршрутизатор.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Сеть не пустила этот компьютер. Подойдите ближе и повторите попытку.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Такого пароля у сети Wi-Fi быть не может. Проверьте его и повторите попытку.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Не удалось подключиться к сети.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = открывается по вашему паролю

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = открывается и изменяется по вашему паролю

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = любой в домашней сети может открыть

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = любой в домашней сети может изменять

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
        [strong] Подключено к сети { $name }, сильный сигнал.
        [good] Подключено к сети { $name }, хороший сигнал.
        [fair] Подключено к сети { $name }, средний сигнал.
       *[other] Подключено к сети { $name }, слабый сигнал.
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
settings-network-top-gone = Сети недоступны для просмотра: NetworkManager не запущен.

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
settings-network-chip = Его чипу нужен более новый Linux.

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
settings-network-wifi-about = Wi-Fi подключает этот компьютер к сети без кабеля. Когда он выключен, поиск сетей прекращается, и это немного экономит заряд батареи. Сеть, которая не показывает своё имя, не появляется в списке сетей в зоне досягаемости: подключитесь к ней здесь, точно набрав её имя, включая заглавные буквы. Если чип Wi-Fi этого компьютера ещё не поддерживается ни одним драйвером в Linux, на котором работает Trindle, понадобится более новый Linux. У сетевого кабеля есть отдельная строка с кнопкой подключения и отключения.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Разрешить другим компьютерам подключаться

# Small line under that switch.
settings-network-share-switch-sub = Чтобы делиться папками и принтерами дома.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Поиск общих папок…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Не удалось прочитать общие папки

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Пока недоступно

# Small line under it.
settings-network-not-reachable-sub = Пока переключатель выключен, другие не могут их открыть.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Нет общих папок

# Small line under it.
settings-network-no-shared-sub = Щёлкните папку правой кнопкой в «Папках», чтобы открыть к ней доступ.

# Heading of the Sharing card.
settings-network-sharing = Общий доступ

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Пока выключено «Разрешить другим компьютерам подключаться», ничто в сети не может обратиться к этому компьютеру, если он сам не начал обмен, поэтому никто в той же сети Wi-Fi не сможет попытать удачу. Включите, чтобы делиться папкой или принтером с другими компьютерами дома. Чтобы открыть доступ к папке, щёлкните её правой кнопкой в «Папках» и выберите «Поделиться в сети». У каждой папки указано, кто и что может с ней делать. По вашему паролю: другие могут открывать файлы или открывать и изменять их, используя ваш пароль общего доступа. Любой в домашней сети: любой в сети может открывать файлы или открывать и изменять их без пароля. Пока переключатель выключен, другие компьютеры дома не могут открыть ни одну из папок.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Остановка…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Ожидание пароля...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Общий доступ разрешён

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Доступ закрыт

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
settings-network-connecting = Подключение...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Нет сетей в зоне досягаемости

# Small line under it.
settings-network-none-in-range-sub = Поиск повторяется при каждом открытии этой страницы.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Сети в зоне досягаемости

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Все сети Wi-Fi, которые слышит этот компьютер. При открытии страницы первой идёт используемая сеть, затем сохранённые здесь, затем остальные по силе сигнала; пока страница открыта, строки остаются на своих местах. Нажмите стрелки, чтобы подключиться; защищённая сеть при первом подключении запросит пароль. Корзина забывает сохранённую сеть и её пароль. Открытая сеть не зашифрована, поэтому другие поблизости могут видеть, что по ней отправляется.

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
settings-network-joining = Подключение...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Пароль Wi-Fi должен содержать не менее 8 символов.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Подключиться к скрытой сети

# Small line under it.
settings-network-hidden-sub = Введите имя точно, включая заглавные буквы.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Имя сети

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Пароль, если он есть

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Нужно имя сети.

# Shown under a network after it was joined.
settings-network-joined = Подключено

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Сеть всё же сохранена: забудьте её кнопкой с корзиной, прежде чем подключаться снова.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Не удалось отключиться.
