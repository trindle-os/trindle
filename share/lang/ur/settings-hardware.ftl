# machine translated (Urdu), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = ہارڈ ویئر

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = ہارڈ ویئر آلات ڈرائیور ڈرائیورز فرم ویئر hardware devices drivers driver firmware pci usb گرافکس آواز نیٹ ورک bluetooth اسٹوریج کیمرا کی بورڈ ماؤس غائب مسئلہ چپ

# Text behind the info button of the Hardware page.
settings-hardware-help = اس کمپیوٹر کا ہر وہ حصہ جو Trindle دیکھ سکتا ہے، کام کے مطابق گروپ میں۔ جس حصے کا ڈرائیور نہیں ہے، یا جس کے ڈرائیور کی فرم ویئر فائل غائب ہے، وہ سب سے پہلے دکھایا جاتا ہے۔ ڈرائیور سسٹم کے ساتھ آتے ہیں، اس لیے یہاں کچھ ڈاؤن لوڈ کرنا نہیں پڑتا۔

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] آپ کے { $n } آلے کا ڈرائیور کام کر رہا ہے۔
       *[other] آپ کے تمام { $n } آلات کے ڈرائیور کام کر رہے ہیں۔
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } آلے پر توجہ چاہیے۔
       *[other] { $n } آلات پر توجہ چاہیے۔
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = گرافکس

settings-hardware-kind-sound = آواز

settings-hardware-kind-network = نیٹ ورک

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = اسٹوریج

settings-hardware-kind-input = کی بورڈ اور ماؤس

settings-hardware-kind-camera = کیمرے

settings-hardware-kind-other = دیگر

# Under a device that works; name is the driver's name.
settings-hardware-driver = کام کر رہا ہے۔ ڈرائیور: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = کام کر رہا ہے

# Under a device that has no driver.
settings-hardware-no-driver = ڈرائیور نہیں ہے

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = فرم ویئر غائب ہے

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = کوئی ڈرائیور استعمال میں نہیں

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = نامعلوم آلہ { $id }

# Card name.
settings-hardware-chip = سیکیورٹی چپ

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = چپ نے ڈسک نہیں کھولی، اس لیے پاس ورڈ مانگا گیا۔

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = چپ دوبارہ استعمال کریں

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = ڈسک کا پاس ورڈ

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = منسوخ کریں

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = پہلے ڈسک کا پاس ورڈ لکھیں۔

# Beside the line while it works.
settings-hardware-chip-waiting = چپ سیٹ ہو رہی ہے

# Once it worked.
settings-hardware-chip-done = اگلی بار شروع کرنے سے چپ ڈسک کھولے گی۔
