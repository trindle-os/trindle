# machine translated (French), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Matériel

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = matériel appareils périphériques pilotes pilote micrologiciel pci usb graphique audio son réseau bluetooth stockage caméra clavier souris manquant problème puce

# Text behind the info button of the Hardware page.
settings-hardware-help = Chaque élément de cet ordinateur que Trindle peut voir, classé selon son rôle. Un élément sans pilote, ou dont le pilote n'a pas son micrologiciel, est placé en premier. Les pilotes viennent avec le système, il n'y a donc rien à télécharger ici.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] Votre { $n } appareil a un pilote qui fonctionne.
       *[other] Vos { $n } appareils ont tous un pilote qui fonctionne.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } appareil demande votre attention.
       *[other] { $n } appareils demandent votre attention.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Graphique

settings-hardware-kind-sound = Son

settings-hardware-kind-network = Réseau

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Stockage

settings-hardware-kind-input = Claviers et souris

settings-hardware-kind-camera = Caméras

settings-hardware-kind-other = Autres

# Under a device that works; name is the driver's name.
settings-hardware-driver = Fonctionne. Pilote : { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Fonctionne

# Under a device that has no driver.
settings-hardware-no-driver = Pas de pilote

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Micrologiciel manquant

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Aucun pilote utilisé

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Appareil inconnu { $id }

# Card name.
settings-hardware-chip = Puce de sécurité

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = La puce n'a pas ouvert le disque. Un BIOS mis à jour peut l'expliquer.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Réutiliser la puce

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Mot de passe du disque

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Annuler

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Tapez d'abord le mot de passe du disque.

# Beside the line while it works.
settings-hardware-chip-waiting = Configuration de la puce

# Once it worked.
settings-hardware-chip-done = La puce déverrouillera le disque dès le prochain démarrage.
