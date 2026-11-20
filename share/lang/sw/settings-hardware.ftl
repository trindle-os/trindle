# machine translated (Swahili), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Vifaa

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = vifaa kiendeshi viendeshi programu dhibiti hardware devices drivers driver firmware pci usb picha sauti mtandao bluetooth hifadhi kamera kibodi kipanya kinakosekana tatizo chipu

# Text behind the info button of the Hardware page.
settings-hardware-help = Kila sehemu ya kompyuta hii ambayo Trindle inaweza kuona, imepangwa kwa kazi yake. Sehemu isiyo na kiendeshi, au yenye kiendeshi kinachokosa faili ya programu dhibiti, inaorodheshwa kwanza. Viendeshi huja na mfumo, kwa hiyo hakuna cha kupakua hapa.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] Kifaa chako { $n } kina kiendeshi kinachofanya kazi.
       *[other] Vifaa vyako vyote { $n } vina viendeshi vinavyofanya kazi.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] Kifaa { $n } kinahitaji uangalizi.
       *[other] Vifaa { $n } vinahitaji uangalizi.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Picha

settings-hardware-kind-sound = Sauti

settings-hardware-kind-network = Mtandao

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Hifadhi

settings-hardware-kind-input = Kibodi na kipanya

settings-hardware-kind-camera = Kamera

settings-hardware-kind-other = Vingine

# Under a device that works; name is the driver's name.
settings-hardware-driver = Inafanya kazi. Kiendeshi: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Inafanya kazi

# Under a device that has no driver.
settings-hardware-no-driver = Hakuna kiendeshi

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Programu dhibiti inakosekana

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Hakuna kiendeshi kinachotumika

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Kifaa kisichojulikana { $id }

# Card name.
settings-hardware-chip = Chipu ya usalama

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = Chipu haikufungua diski, kwa hiyo nenosiri liliombwa. BIOS inaweza kuwa sababu.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Tumia chipu tena

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Nenosiri la diski

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Ghairi

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Andika nenosiri la diski kwanza.

# Beside the line while it works.
settings-hardware-chip-waiting = Inaweka chipu

# Once it worked.
settings-hardware-chip-done = Chipu itafungua diski kuanzia kuwasha kunakofuata.
