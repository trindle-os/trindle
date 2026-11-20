# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Hardware

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = hardware dispositivos drivers driver firmware pci usb gráficos som rede bluetooth armazenamento câmera teclado mouse faltando problema chip

# Text behind the info button of the Hardware page.
settings-hardware-help = Todas as peças deste computador que o Trindle consegue ver, agrupadas pelo que fazem. Uma peça sem driver, ou cujo driver não tem um arquivo de firmware, aparece primeiro. Os drivers vêm com o sistema, então não há nada para baixar aqui.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] Seu { $n } dispositivo tem um driver funcionando.
       *[other] Todos os { $n } dispositivos têm drivers funcionando.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } dispositivo precisa de atenção.
       *[other] { $n } dispositivos precisam de atenção.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Gráficos

settings-hardware-kind-sound = Som

settings-hardware-kind-network = Rede

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Armazenamento

settings-hardware-kind-input = Teclados e mouses

settings-hardware-kind-camera = Câmeras

settings-hardware-kind-other = Outros

# Under a device that works; name is the driver's name.
settings-hardware-driver = Funcionando. Driver: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Funcionando

# Under a device that has no driver.
settings-hardware-no-driver = Sem driver

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Falta firmware

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Nenhum driver em uso

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Dispositivo desconhecido { $id }

# Card name.
settings-hardware-chip = Chip de segurança

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = O chip não destravou o disco ao iniciar, então a senha foi pedida. Uma atualização da BIOS pode causar isso.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Usar o chip de novo

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Senha do disco

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Cancelar

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Digite primeiro a senha do disco.

# Beside the line while it works.
settings-hardware-chip-waiting = Configurando o chip

# Once it worked.
settings-hardware-chip-done = O chip vai destravar o disco a partir da próxima inicialização.
