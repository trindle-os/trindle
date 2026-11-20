# machine translated (Turkish), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Donanım

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = donanım aygıtlar aygıt sürücüler sürücü bellenim firmware pci usb grafik ses ağ bluetooth depolama kamera klavye fare eksik sorun yonga

# Text behind the info button of the Hardware page.
settings-hardware-help = Trindle'ın görebildiği bu bilgisayarın tüm parçaları, işlevlerine göre gruplanmıştır. Sürücüsü olmayan ya da sürücüsünün bellenim dosyası eksik olan parça en başta listelenir. Sürücüler sistemle gelir, bu yüzden burada indirilecek bir şey yoktur.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] { $n } aygıtınızın sürücüsü çalışıyor.
       *[other] { $n } aygıtınızın hepsinin sürücüsü çalışıyor.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } aygıt ilgi bekliyor.
       *[other] { $n } aygıt ilgi bekliyor.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Grafik

settings-hardware-kind-sound = Ses

settings-hardware-kind-network = Ağ

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Depolama

settings-hardware-kind-input = Klavyeler ve fareler

settings-hardware-kind-camera = Kameralar

settings-hardware-kind-other = Diğer

# Under a device that works; name is the driver's name.
settings-hardware-driver = Çalışıyor. Sürücü: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Çalışıyor

# Under a device that has no driver.
settings-hardware-no-driver = Sürücü yok

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Bellenim eksik

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Sürücü kullanılmıyor

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Bilinmeyen aygıt { $id }

# Card name.
settings-hardware-chip = Güvenlik yongası

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = Yonga diski açmadı, parola istendi. BIOS güncellemesi neden olabilir.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Yongayı yeniden kullan

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Disk parolası

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = İptal

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Önce disk parolasını yazın.

# Beside the line while it works.
settings-hardware-chip-waiting = Yonga ayarlanıyor

# Once it worked.
settings-hardware-chip-done = Yonga, sonraki açılıştan itibaren diski açacak.
