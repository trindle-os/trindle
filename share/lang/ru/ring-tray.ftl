# machine translated (Russian), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
        [one] { $n } обновление плагина требует проверки в «Приложениях»
        [few] { $n } обновления плагинов требуют проверки в «Приложениях»
        [many] { $n } обновлений плагинов требуют проверки в «Приложениях»
       *[other] { $n } обновления плагинов требуют проверки в «Приложениях»
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Всё обновлено

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
        [one] Ожидает { $n } обновление: кнопка обновлений рядом с «Приложениями» установит его
        [few] Ожидают { $n } обновления: кнопка обновлений рядом с «Приложениями» установит их
        [many] Ожидают { $n } обновлений: кнопка обновлений рядом с «Приложениями» установит их
       *[other] Ожидают { $n } обновления: кнопка обновлений рядом с «Приложениями» установит их
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = Ожидает обновлений: { $n }. { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = Ожидает обновлений: { $n }. Кнопка обновлений рядом с «Приложениями» установит { $put }, а { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
        [one] Открыть обновление прошивки
        [few] Открыть { $n } обновления прошивки
        [many] Открыть { $n } обновлений прошивки
       *[other] Открыть { $n } обновления прошивки
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
        [one] Открыть { $n } обновление, среди них прошивка
        [few] Открыть { $n } обновления, среди них прошивка
        [many] Открыть { $n } обновлений, среди них прошивка
       *[other] Открыть { $n } обновления, среди них прошивка
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
        [one] Установить { $n } обновление
        [few] Установить { $n } обновления
        [many] Установить { $n } обновлений
       *[other] Установить { $n } обновления
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
        [one] Открыть обновление плагина, которому нужно больше разрешений
        [few] Открыть { $n } обновления плагинов, которым нужно больше разрешений
        [many] Открыть { $n } обновлений плагинов, которым нужно больше разрешений
       *[other] Открыть { $n } обновления плагинов, которым нужно больше разрешений
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
        [one] Установить { $put } обновление; { $look }
        [few] Установить { $put } обновления; { $look }
        [many] Установить { $put } обновлений; { $look }
       *[other] Установить { $put } обновления; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Обновлено

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Клавиатура

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Яркость экрана

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Не засыпать

# Name of the sound button when the sound is muted.
ring-tray-muted = Звук выключен

# Name of the sound button with the volume level.
ring-tray-volume = Громкость { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Батарея { $pct }%, заряжается
        [onbattery] Батарея { $pct }%, работа от батареи
        [full] Батарея { $pct }%, заряжена
        [pluggedin] Батарея { $pct }%, подключено питание
       *[other] Батарея { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Сеть выключена

# Name of the network button when nothing is connected.
ring-tray-not-connected = Нет подключения

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Подключено через VPN

# Name of the network button when connected by cable.
ring-tray-wired = Проводное подключение

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Проводное подключение, ограниченный доступ

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, сигнал { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, сигнал { $strength }%, ограниченный доступ

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth выключен

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Подключено: { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth включён

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Значок в трее

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Все значки в трее

# Words on the panel of every tray icon when there are none.
ring-tray-none = Нет значков в трее

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Показать в кольце

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Убрать из кольца

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = В кольце уже четыре. Сначала уберите один.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Проверка выключена

