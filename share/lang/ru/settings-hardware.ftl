# machine translated (Russian), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Оборудование

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = оборудование устройства драйверы драйвер прошивка pci usb графика звук сеть bluetooth накопитель камера клавиатура мышь нет проблема чип

# Text behind the info button of the Hardware page.
settings-hardware-help = Все части этого компьютера, которые видит Trindle, по роду работы. Сначала идут части без драйвера или те, которым для драйвера не хватает файла прошивки. Драйверы входят в систему, поэтому скачивать здесь ничего не нужно.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] У вашего устройства ({ $n }) работает драйвер.
        [few] У всех ваших устройств ({ $n }) работают драйверы.
        [many] У всех ваших устройств ({ $n }) работают драйверы.
       *[other] У всех ваших устройств ({ $n }) работают драйверы.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } устройство требует внимания.
        [few] { $n } устройства требуют внимания.
        [many] { $n } устройств требуют внимания.
       *[other] { $n } устройства требуют внимания.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Графика

settings-hardware-kind-sound = Звук

settings-hardware-kind-network = Сеть

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Накопители

settings-hardware-kind-input = Клавиатуры и мыши

settings-hardware-kind-camera = Камеры

settings-hardware-kind-other = Другое

# Under a device that works; name is the driver's name.
settings-hardware-driver = Работает. Драйвер: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Работает

# Under a device that has no driver.
settings-hardware-no-driver = Нет драйвера

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Не хватает прошивки

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Драйвер не используется

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Неизвестное устройство { $id }

# Card name.
settings-hardware-chip = Чип безопасности

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = Чип не открыл диск, пришлось ввести пароль. Возможно, обновили BIOS.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Снова использовать чип

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Пароль диска

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Отмена

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Сначала введите пароль диска.

# Beside the line while it works.
settings-hardware-chip-waiting = Настраиваем чип

# Once it worked.
settings-hardware-chip-done = Со следующего запуска диск будет разблокировать чип.
