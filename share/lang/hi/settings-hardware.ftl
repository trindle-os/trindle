# machine translated (Hindi), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = हार्डवेयर

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = हार्डवेयर उपकरण ड्राइवर फ़र्मवेयर ग्राफ़िक्स आवाज़ नेटवर्क ब्लूटूथ स्टोरेज कैमरा कीबोर्ड माउस गायब समस्या चिप hardware devices drivers driver firmware pci usb graphics sound network bluetooth storage camera keyboard mouse missing problem chip

# Text behind the info button of the Hardware page.
settings-hardware-help = इस कंप्यूटर के वे सभी हिस्से जो Trindle देख सकता है, काम के हिसाब से समूहों में। जिस हिस्से का ड्राइवर नहीं है, या जिसके ड्राइवर की फ़र्मवेयर फ़ाइल गायब है, वह सबसे पहले दिखता है। ड्राइवर सिस्टम के साथ आते हैं, इसलिए यहाँ कुछ डाउनलोड नहीं करना है।

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [one] आपके { $n } उपकरण का ड्राइवर ठीक से काम कर रहा है।
       *[other] आपके सभी { $n } उपकरणों के ड्राइवर ठीक से काम कर रहे हैं।
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [one] { $n } उपकरण पर ध्यान देना है।
       *[other] { $n } उपकरणों पर ध्यान देना है।
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = ग्राफ़िक्स

settings-hardware-kind-sound = आवाज़

settings-hardware-kind-network = नेटवर्क

settings-hardware-kind-bluetooth = ब्लूटूथ

settings-hardware-kind-storage = स्टोरेज

settings-hardware-kind-input = कीबोर्ड और माउस

settings-hardware-kind-camera = कैमरे

settings-hardware-kind-other = अन्य

# Under a device that works; name is the driver's name.
settings-hardware-driver = चालू है। ड्राइवर: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = चालू है

# Under a device that has no driver.
settings-hardware-no-driver = ड्राइवर नहीं है

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = फ़र्मवेयर गायब है

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = कोई ड्राइवर इस्तेमाल में नहीं

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = अज्ञात उपकरण { $id }

# Card name.
settings-hardware-chip = सुरक्षा चिप

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = शुरू होते समय चिप ने डिस्क नहीं खोली, इसलिए पासवर्ड माँगा गया। BIOS अपडेट से ऐसा हो सकता है।

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = चिप फिर इस्तेमाल करें

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = डिस्क का पासवर्ड

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = रद्द करें

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = पहले डिस्क का पासवर्ड लिखें।

# Beside the line while it works.
settings-hardware-chip-waiting = चिप तैयार की जा रही है

# Once it worked.
settings-hardware-chip-done = अगली बार से चिप डिस्क खोलेगी।
