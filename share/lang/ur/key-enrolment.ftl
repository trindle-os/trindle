
# machine translated (Urdu), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
# SAFETY: needs a human review before it ships
key-enrol-title = ری اسٹارٹ پر ڈرائیور کی کلید کی منظوری

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen کلید پاس ورڈ ڈرائیور نیلی اسکرین

# First paragraph before the steps when the driver is NVIDIA's own.
# SAFETY: needs a human review before it ships
key-enrol-why-nvidia = NVIDIA کا اپنا ڈرائیور اسی کمپیوٹر پر بنتا ہے، اس لیے Secure Boot اسے اس وقت تک لوڈ نہیں کرے گا جب تک آپ اس کی کلید کی منظوری نہ دیں۔ یہ ری اسٹارٹ سے پہلے کریں، ورنہ اسکرین کالی رہ سکتی ہے۔

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
# SAFETY: needs a human review before it ships
key-enrol-why-addon = ایک اضافی ڈرائیور اسی کمپیوٹر پر بنتا ہے، اس لیے Secure Boot اسے اس وقت تک لوڈ نہیں کرے گا جب تک آپ اس کی کلید کی منظوری نہ دیں۔ یہ ری اسٹارٹ سے پہلے کریں۔

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = صرف حروف اور ہندسوں پر مشتمل ایک پاس ورڈ بنائیں۔ نیلی اسکرین پر امریکی کی بورڈ ہوتا ہے، اس لیے علامتیں دوسری کلیدوں پر ہو سکتی ہیں۔ اسے لکھ لیں؛ یہ ایک ہی بار استعمال ہوتا ہے۔

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = ری اسٹارٹ کریں۔ Trindle شروع ہونے سے پہلے Enroll MOK کے عنوان والی نیلی اسکرین آتی ہے۔ یہ تقریباً 10 سیکنڈ بعد ہار مان لیتی ہے، اس لیے کوئی کلید دبانے کے لیے تیار رہیں۔

# Step 3: the first two screens.
key-enrol-step3 = Enroll MOK چنیں، پھر Continue۔

# Step 4: the third screen and the password.
key-enrol-step4 = Yes چنیں، پھر وہ پاس ورڈ لکھیں جو آپ نے بنایا تھا۔

# Step 5: the last screen.
key-enrol-step5 = Reboot چنیں۔ کمپیوٹر واپس آنے پر ڈرائیور لوڈ ہو جائے گا۔

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = نیلی اسکرین چھوٹ جائے تو کمپیوٹر پہلے کی طرح ڈرائیور کے بغیر شروع ہوتا ہے۔ کچھ خراب نہیں ہوتا؛ دوبارہ پوچھے جانے کے لیے sudo mokutil --import /var/lib/dkms/mok.pub چلائیں اور ری اسٹارٹ کریں۔
