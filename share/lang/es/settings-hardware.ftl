# machine translated (Spanish), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Hardware

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = hardware dispositivos controladores controlador firmware pci usb gráficos sonido red bluetooth almacenamiento cámara teclado ratón falta problema chip

# Text behind the info button of the Hardware page.
settings-hardware-help = Todas las piezas de este ordenador que Trindle puede ver, agrupadas por lo que hacen. Las piezas sin controlador, o cuyo controlador no tiene un archivo de firmware, aparecen primero. Los controladores vienen con el sistema, así que aquí no hay nada que descargar.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] Tu dispositivo tiene un controlador que funciona.
       *[other] Los { $n } dispositivos tienen controladores que funcionan.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } dispositivo necesita atención.
       *[other] { $n } dispositivos necesitan atención.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Gráficos

settings-hardware-kind-sound = Sonido

settings-hardware-kind-network = Red

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Almacenamiento

settings-hardware-kind-input = Teclados y ratones

settings-hardware-kind-camera = Cámaras

settings-hardware-kind-other = Otros

# Under a device that works; name is the driver's name.
settings-hardware-driver = Funciona. Controlador: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Funciona

# Under a device that has no driver.
settings-hardware-no-driver = Sin controlador

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Falta el firmware

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Sin controlador en uso

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Dispositivo desconocido { $id }

# Card name.
settings-hardware-chip = Chip de seguridad

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = El chip no desbloqueó el disco y se pidió la contraseña. Pudo ser la BIOS.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Usar el chip otra vez

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Contraseña del disco

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Cancelar

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Escribe primero la contraseña del disco.

# Beside the line while it works.
settings-hardware-chip-waiting = Configurando el chip

# Once it worked.
settings-hardware-chip-done = El chip desbloqueará el disco desde el próximo arranque.
