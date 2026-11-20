# machine translated (Indonesian), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Perangkat keras

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = perangkat keras perangkat driver firmware pci usb grafis suara jaringan bluetooth penyimpanan kamera papan ketik mouse hilang masalah chip

# Text behind the info button of the Hardware page.
settings-hardware-help = Semua bagian komputer ini yang dapat dilihat Trindle, dikelompokkan menurut fungsinya. Bagian yang tidak memiliki driver, atau drivernya kehilangan berkas firmware, ditampilkan paling atas. Driver sudah disertakan di sistem, jadi tidak ada yang perlu diunduh di sini.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
       *[other] Semua { $n } perangkat Anda memiliki driver yang berfungsi.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
       *[other] { $n } perangkat perlu diperhatikan.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Grafis

settings-hardware-kind-sound = Suara

settings-hardware-kind-network = Jaringan

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Penyimpanan

settings-hardware-kind-input = Papan ketik dan mouse

settings-hardware-kind-camera = Kamera

settings-hardware-kind-other = Lainnya

# Under a device that works; name is the driver's name.
settings-hardware-driver = Berfungsi. Driver: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Berfungsi

# Under a device that has no driver.
settings-hardware-no-driver = Tidak ada driver

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Firmware hilang

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Tidak ada driver yang digunakan

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Perangkat tidak dikenal { $id }

# Card name.
settings-hardware-chip = Chip keamanan

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = Chip tidak membuka disk saat mulai, jadi kata sandi diminta. Bisa karena pembaruan BIOS.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Gunakan chip lagi

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Kata sandi disk

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Batal

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Ketik kata sandi disk dulu.

# Beside the line while it works.
settings-hardware-chip-waiting = Menyiapkan chip

# Once it worked.
settings-hardware-chip-done = Chip akan membuka disk mulai dari awal berikutnya.
