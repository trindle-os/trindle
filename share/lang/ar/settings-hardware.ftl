# machine translated (Arabic), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = الأجهزة

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = أجهزة عتاد برنامج تشغيل برامج تشغيل برنامج ثابت pci usb رسوميات صوت شبكة بلوتوث تخزين كاميرا لوحة مفاتيح فأرة ناقص مشكلة شريحة

# Text behind the info button of the Hardware page.
settings-hardware-help = كل جزء في هذا الحاسوب يراه Trindle، مجمّعًا حسب وظيفته. الجزء الذي لا يملك برنامج تشغيل، أو ينقص برنامج تشغيله ملف برنامج ثابت، يظهر أولًا. برامج التشغيل تأتي مع النظام، فلا شيء تنزّله هنا.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
        [zero] لا توجد أجهزة.
        [one] جهازك له برنامج تشغيل يعمل.
        [two] لكل جهازيك برنامج تشغيل يعمل.
        [few] لكل أجهزتك الـ { $n } برامج تشغيل تعمل.
        [many] لكل أجهزتك الـ { $n } برامج تشغيل تعمل.
       *[other] لكل أجهزتك الـ { $n } برامج تشغيل تعمل.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
        [zero] لا جهاز يحتاج إلى انتباه.
        [one] جهاز واحد يحتاج إلى انتباه.
        [two] جهازان يحتاجان إلى انتباه.
        [few] { $n } أجهزة تحتاج إلى انتباه.
        [many] { $n } جهازًا تحتاج إلى انتباه.
       *[other] { $n } جهاز تحتاج إلى انتباه.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = الرسوميات

settings-hardware-kind-sound = الصوت

settings-hardware-kind-network = الشبكة

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = التخزين

settings-hardware-kind-input = لوحات المفاتيح والفأرات

settings-hardware-kind-camera = الكاميرات

settings-hardware-kind-other = أخرى

# Under a device that works; name is the driver's name.
settings-hardware-driver = يعمل. برنامج التشغيل: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = يعمل

# Under a device that has no driver.
settings-hardware-no-driver = بلا برنامج تشغيل

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = البرنامج الثابت ناقص

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = لا برنامج تشغيل قيد الاستخدام

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = جهاز غير معروف { $id }

# Card name.
settings-hardware-chip = شريحة الأمان

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = لم تفتح الشريحة القرص عند التشغيل، فطُلبت كلمة المرور. قد يكون السبب تحديث BIOS.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = استخدم الشريحة مرة أخرى

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = كلمة مرور القرص

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = إلغاء

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = اكتب كلمة مرور القرص أولاً.

# Beside the line while it works.
settings-hardware-chip-waiting = جارٍ إعداد الشريحة

# Once it worked.
settings-hardware-chip-done = ستفتح الشريحة القرص من التشغيل التالي.
