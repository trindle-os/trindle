# Settings, Hardware page: the computer's devices by kind and which ones have no working driver.

## The page (src/settings/panels/hardware.rs)

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Hardware
# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = hardware devices drivers driver firmware pci usb graphics sound network bluetooth storage camera keyboard mouse missing problem chip
# Text behind the info button of the Hardware page.
settings-hardware-help = Every part of this computer that Trindle can see, grouped by what it does. A part with no driver, or whose driver is missing a firmware file, is listed first. Drivers come with the system, so there is nothing to download here.
# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] Your { $n } device has a working driver.
       *[other] All { $n } of your devices have working drivers.
    }
# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } device needs attention.
       *[other] { $n } devices need attention.
    }
# Card names, one for each kind of device.
settings-hardware-kind-graphics = Graphics
settings-hardware-kind-sound = Sound
settings-hardware-kind-network = Network
settings-hardware-kind-bluetooth = Bluetooth
settings-hardware-kind-storage = Storage
settings-hardware-kind-input = Keyboards and mice
settings-hardware-kind-camera = Cameras
settings-hardware-kind-other = Other
# Under a device that works; name is the driver's name.
settings-hardware-driver = Working. Driver: { $name }
# Under a device that works when the driver's name is not known.
settings-hardware-working = Working
# Under a device that has no driver.
settings-hardware-no-driver = No driver
# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Missing firmware
# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = No driver in use
# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Unknown device { $id }

## The security chip card, shown only after the TPM chip did not unlock the encrypted disk at start

# Card name.
settings-hardware-chip = Security chip
# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = The chip didn't unlock the disk at start, so it asked for the password. A BIOS update can cause this.
# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Use the chip again
# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Disk password
# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Cancel
# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Type the disk password first.
# Beside the line while it works.
settings-hardware-chip-waiting = Setting up the chip
# Once it worked.
settings-hardware-chip-done = The chip will unlock the disk from the next start.
