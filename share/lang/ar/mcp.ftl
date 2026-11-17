
# machine translated (Arabic), not yet reviewed by a person.

# Window title of the question an assistant's request raises.
mcp-title-ask = طلب من المساعد

# Heading over the words of what the assistant wants to do.
mcp-ask-heading = مساعد متصل بـ Trindle يريد أن:

# SAFETY: needs a human review before it ships
# Button that refuses the change the assistant asked for.
mcp-no = لا

# SAFETY: needs a human review before it ships
# Button that lets the change the assistant asked for go ahead.
mcp-yes = نعم، تابع

# SAFETY: needs a human review before it ships
# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = يُظهر { $name } في المقدمة

# SAFETY: needs a human review before it ships
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = يغيّر الإعداد { $key } من { $from } إلى { $to }

# SAFETY: needs a human review before it ships
# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = يستخدم المظهر { $look }

# SAFETY: needs a human review before it ships
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = يستخدم المظهر { $look } مع { $accent } لونًا للتمييز

# SAFETY: needs a human review before it ships
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = يثبّت { $id } من Flathub

# SAFETY: needs a human review before it ships
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = يزيل { $id } ويبقي ملفاته

# SAFETY: needs a human review before it ships
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = ينضم إلى شبكة Wi-Fi ‏{ $name }

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its address comes in.
mcp-ask-pair = يقترن بجهاز Bluetooth ‏{ $address }

# SAFETY: needs a human review before it ships
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = يعيد الإعدادات وأذونات التطبيقات كما كانت قبل: { $note }

# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = إعادة الإعدادات وأذونات التطبيق وملفات { $id } كما كانت قبل: { $note }. يُغلق { $id } أولًا إن كان مفتوحًا

# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = إغلاق { $id } وتشغيله مجددًا. يضيع كل ما لم يُحفظ فيه

# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = تغيير { $key } في { $file } الخاص بـ { $id } من { $from } إلى { $to }. يُغلق { $id } أولًا إن كان مفتوحًا

# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = إضافة { $key } بالقيمة { $to } في { $file } الخاص بـ { $id }. يُغلق { $id } أولًا إن كان مفتوحًا

# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = مسح ذاكرة التخزين المؤقت لـ { $id }. يُغلق { $id } أولًا إن كان مفتوحًا، وتُحفظ نسخة من الذاكرة ليمكن إعادتها

# SAFETY: needs a human review before it ships
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = يقترن بجهاز Bluetooth ‏{ $name } ({ $address })

# SAFETY: needs a human review before it ships
# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = يغلق { $name }

# SAFETY: needs a human review before it ships
# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = يفتح { $name }

# SAFETY: needs a human review before it ships
# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = يسمح لـ { $id } باستخدام { $what }

# SAFETY: needs a human review before it ships
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = يمنع { $id } من استخدام { $what }
