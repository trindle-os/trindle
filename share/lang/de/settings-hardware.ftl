# machine translated (German), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Hardware

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = hardware geräte treiber firmware pci usb grafik ton netzwerk bluetooth speicher kamera tastatur maus fehlt problem chip

# Text behind the info button of the Hardware page.
settings-hardware-help = Alle Teile dieses Computers, die Trindle sehen kann, nach Aufgabe geordnet. Ein Teil ohne Treiber oder mit einem Treiber, dem eine Firmware-Datei fehlt, steht ganz oben. Treiber gehören zum System, du musst hier also nichts herunterladen.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] Dein { $n } Gerät hat einen funktionierenden Treiber.
       *[other] Alle { $n } deiner Geräte haben funktionierende Treiber.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } Gerät braucht Aufmerksamkeit.
       *[other] { $n } Geräte brauchen Aufmerksamkeit.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Grafik

settings-hardware-kind-sound = Ton

settings-hardware-kind-network = Netzwerk

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Speicher

settings-hardware-kind-input = Tastaturen und Mäuse

settings-hardware-kind-camera = Kameras

settings-hardware-kind-other = Sonstige

# Under a device that works; name is the driver's name.
settings-hardware-driver = Funktioniert. Treiber: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Funktioniert

# Under a device that has no driver.
settings-hardware-no-driver = Kein Treiber

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Firmware fehlt

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Kein Treiber in Benutzung

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Unbekanntes Gerät { $id }

# Card name.
settings-hardware-chip = Sicherheitschip

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = Chip entsperrte nicht, daher das Passwort. Ein BIOS-Update kann es sein.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Chip wieder verwenden

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Festplattenpasswort

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Abbrechen

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Gib zuerst das Festplattenpasswort ein.

# Beside the line while it works.
settings-hardware-chip-waiting = Chip wird eingerichtet

# Once it worked.
settings-hardware-chip-done = Der Chip entsperrt die Festplatte ab dem nächsten Start.
