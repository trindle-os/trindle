# machine translated (Arabic), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = الموافقة على مفتاح برنامج التشغيل عند إعادة التشغيل

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen التمهيد الآمن مفتاح الشاشة الزرقاء كلمة المرور برنامج التشغيل كاميرا الشاشة السوداء

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = يُبنى برنامج تشغيل NVIDIA على هذا الحاسوب، لذلك لن يحمّله Secure Boot حتى توافق على مفتاحه. افعل هذا قبل إعادة التشغيل، وإلا قد تبقى الشاشة سوداء.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = يُبنى برنامج التشغيل الإضافي على هذا الحاسوب، لذلك لن يحمّله Secure Boot حتى توافق على مفتاحه. افعل هذا قبل إعادة التشغيل.

# Step 1: the password, made before the restart.
key-enrol-step1 = اختر كلمة مرور من حروف وأرقام فقط. الشاشة الزرقاء تستخدم لوحة مفاتيح أمريكية، لذا قد تكون الرموز في مفاتيح أخرى. دوّنها؛ ستُستخدم مرة واحدة.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = أعد التشغيل. ستظهر شاشة زرقاء بعنوان Enroll MOK قبل أن يبدأ Trindle. تنتهي مهلتها بعد نحو 10 ثوانٍ، فكن مستعدًا لضغط مفتاح.

# Step 3: the first two screens.
key-enrol-step3 = اختر Enroll MOK ثم Continue.

# Step 4: the third screen and the password.
key-enrol-step4 = اختر Yes ثم اكتب كلمة المرور التي اخترتها.

# Step 5: the last screen.
key-enrol-step5 = اختر Reboot. يُحمَّل برنامج التشغيل عندما يعود الحاسوب.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = إذا فاتتك الشاشة الزرقاء يبدأ الحاسوب بدون برنامج التشغيل كما كان. لم يتعطل شيء؛ شغّل sudo mokutil --import /var/lib/dkms/mok.pub ثم أعد التشغيل ليُطلب منك مرة أخرى.

