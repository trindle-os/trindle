
# machine translated (Arabic), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = لوحة المفاتيح والفأرة

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = تخطيط لغة كتابة caps lock لوحة مفاتيح على الشاشة زر حلقة فأرة لوحة اللمس سرعة المؤشر نقر تمرير عجلة أبطأ أسرع طبيعي أعسر إضافة اختصار مفاتيح إعادة تعيين تغيير تبديل ctrl escape super alt mac أزرار جانبية ضغطة خفيفة مطوّلة صنعتها برامج keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = كل ما هنا يتغير فورًا. يُبدَّل التخطيط الثاني بالضغط على Alt وShift معًا.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = الكتابة بتخطيط { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt وShift يبدّلان إلى { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock لا يفعل شيئًا.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock يعمل بوصفه { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = Ctrl آخر

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = قائمة تخطيطات لوحة المفاتيح غير موجودة على هذا الحاسوب.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = لوحة المفاتيح

# Heading of the line in that card.
settings-keyboard-no-list-heading = قائمة تخطيطات لوحة المفاتيح غير متاحة

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = ثبّتها بالأمر: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [zero] لم يتغير أي مفتاح.
        [one] تغيّر مفتاح واحد.
        [two] تغيّر مفتاحان.
        [few] تغيّرت { $n } مفاتيح وأزرار.
        [many] تغيّر { $n } مفتاحًا وزرًا.
       *[other] تغيّر { $n } مفتاح وزر.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = التخطيط

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = التخطيط

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = الصيغة

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = القياسية

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = بلا

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = التبديل إلى

# Small words under the second layout line.
settings-keyboard-switch-to-about = Alt وShift معًا يبدّلان بين الاثنين.

# Words behind the i of the layout card.
settings-keyboard-layout-about = ما يكتبه كل مفتاح: التخطيط، وصيغة منه مثل صيغة بلا مفاتيح ميتة. يُبدَّل التخطيط الثاني بالضغط على Alt وShift معًا، ويُعاد بالطريقة نفسها. كل هذا يتغير فورًا.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = قيد الاستخدام الآن

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Ctrl آخر

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = لا شيء

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = مفتاح Caps Lock يعمل بوصفه

# Small words under that line.
settings-keyboard-caps-acts-as-about = Ctrl آخر، أو Escape، وهكذا.

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = زر لوحة المفاتيح في الحلقة

# Small words under that line.
settings-keyboard-ring-button-about = يُظهر لوحة مفاتيح على الشاشة.

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = مفاتيح أخرى

# Words behind the i of that card.
settings-keyboard-other-about = يمكن أن يعمل Caps Lock بوصفه مفتاحًا آخر، مثل Ctrl آخر أو Escape. وزر لوحة المفاتيح هو لوحة مفاتيح في وسط الحلقة تُظهر لوحة مفاتيح على الشاشة، للكتابة بالفأرة أو بشاشة اللمس. يظهر في المرة التالية التي تُفتح فيها الحلقة.

# Words under a line when a press that was not the person's own changed nothing.
# SAFETY: needs a human review before it ships
settings-keyboard-only-yours = لا يستطيع تغيير هذا إلا ضغطك أنت.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = اضغط المفتاح أو زر الفأرة المراد تغييره.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = لم يُضغط شيء، لذلك لم يُضف شيء.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = اكتب ما سيصبح عليه، مثل Escape أو Ctrl+C أو Nothing.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = تعذّر تمييز هذه الضغطة. اختر المفتاح من القائمة بدلًا من ذلك.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = كل لوحة مفاتيح وفأرة

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = مغيّر أدناه أيضًا؛ ويُستخدم ذلك التغيير.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = لوحة المفاتيح أو الفأرة التي يُغيَّر عليها { $key }

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = ما سيصبح عليه { $key }

# Name of the bin button that takes a change away.
settings-keyboard-take-away = أزِل هذا التغيير

# Small words under a changed key's line when it changes on every keyboard and mouse.
settings-keyboard-every-device-about = على كل لوحة مفاتيح وفأرة.

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = السطر { $n } غير مستخدم: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = تعذّرت قراءة المفاتيح المغيّرة

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = كل مفتاح كما هو مطبوع إلى أن تمكن قراءتها: { $why }.

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = اختر المفتاح أو الزر المراد تغييره

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = اضغط المفتاح أو الزر المراد تغييره

# Small words under the change a key line.
settings-keyboard-keys-add-about = اضغطه، أو اخترْه من القائمة.

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = غيّر مفتاحًا أو زرًا

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } مغيّر بالفعل، أدناه.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = المفاتيح والأزرار

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = يمكن لأي مفتاح أو زر فأرة أن يفعل شيئًا آخر: مفتاحًا أو زرًا آخر، أو مفاتيح مضغوطة معًا مثل Ctrl+C، أو سلسلة مفاتيح الواحد بعد الآخر مثل H, I، أو لا شيء على الإطلاق، أو شيئًا عند النقر الخفيف وآخر عند الإبقاء، مثل Escape عند النقر الخفيف وCtrl عند الإبقاء. ويعمل في كل مكان بدءًا من شاشة تسجيل الدخول. تصل Ctrl وAlt ومفتاح F دائمًا إلى طرفية نصية، وكتابة trindle-settings --keys default هناك تعيد كل مفتاح إلى حاله.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = بدّل بين Ctrl وCaps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock هو Escape عند النقر الخفيف، وCtrl عند الإبقاء

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = بدّل بين Escape وCaps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = بدّل بين Alt وSuper، كما في لوحة مفاتيح Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = الأزرار الجانبية للفأرة تنسخ وتلصق

# Small words under a keyboard or mouse a program made, when it counts as the person.
# SAFETY: needs a human review before it ships
settings-keyboard-made-yes = يُحسب بوصفه أنت، كما تُحسب لوحة مفاتيح موصولة.

# Small words under a keyboard or mouse a program made, when it does not count as the person.
# SAFETY: needs a human review before it ships
settings-keyboard-made-no = يكتب ويلعب. لا يستطيع الضغط على الأزرار المحمية.

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = { $name } يُحسب بوصفه أنت

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = لا شيء حتى الآن

# Small words under that line.
settings-keyboard-made-none-about = ما يصنعه برنامج يظهر هنا، لتسمح له بأن يُحسب بوصفك.

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = لوحات المفاتيح والفئران التي تصنعها البرامج

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = يمكن لأدوات إعادة التعيين ومشغّلات الألعاب أن تصنع لوحة مفاتيح أو فأرة خاصة بها. تبقى مفاتيحها ونقراتها تكتب وتلعب، لكنها لا تستطيع الضغط على الأزرار المحمية، مثل إيقاف التشغيل أو السماح، ولا الكتابة في سؤال موجّه إليك. شغّل واحدًا هنا لتسمح له بأن يُحسب بوصفك.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = وحدة تحكم { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } على { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = ما تفعله { $title }

# Small words under a controller's changed button; the words Button and what it becomes stand for the two sides of the line typed.
settings-keyboard-pad-type = اكتبه هكذا: الزر = ما سيصبح عليه.

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = بدّل بين A وB

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = بدّل بين X وY

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = اقلب اتجاه العصا اليمنى للأعلى والأسفل

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = اختصارات الإضافات

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }. غير مستخدم: { $why }

# Small words under a plugin shortcut; { $plugin } is the plugin. The example is keys a person types.
settings-keyboard-plugin-about = { $plugin }. اكتب المفاتيح، مثل Ctrl+Alt+S، أو لا شيء لعدم وجود اختصار.

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = مفاتيح { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = تم الحفظ

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = ولوحة اللمس أيضًا.

# Title of the mouse card.
settings-keyboard-mouse-title = الفأرة

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = سرعة المؤشر هي المسافة التي يقطعها المؤشر مقابل حركة الفأرة، ولوحة اللمس كذلك. وسرعة التمرير هي المسافة التي تتحركها الصفحة مع كل دورة للعجلة: أبطأ أو عادية أو أسرع، أو في أي نقطة على الشريط. ومع التمرير الطبيعي تتحرك الصفحة في اتجاه دوران العجلة، كما في الهاتف. وخيار اليد اليسرى يبدّل الزرين الأيسر والأيمن.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = سرعة المؤشر

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = سرعة التمرير

# Small words under the wheel's scroll speed.
settings-keyboard-scroll-wheel-about = المسافة التي تتحركها الصفحة مع كل دورة للعجلة.

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = التمرير الطبيعي

# Small words under natural scrolling for the mouse wheel.
settings-keyboard-natural-wheel-about = تتبع الصفحة العجلة، كما في الهاتف.

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = اليد اليسرى

# Small words under that switch.
settings-keyboard-left-handed-about = يبدّل الزرين الأيسر والأيمن.

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = مساعد النقر

# Small words under the clicking helper line.
# SAFETY: needs a human review before it ships
settings-keyboard-clicker-about = نقرات أداة النقر بالتوقف أو بالمفتاح تُحسب نقراتك أنت.

# Title of the touchpad card.
settings-keyboard-touchpad-title = لوحة اللمس

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = سرعة التمرير هي المسافة التي تتحركها الصفحة مع إصبعين. والنقر بالطرق يجعل الطرقة الخفيفة تُحسب نقرة. ومع التمرير الطبيعي تتحرك الصفحة في اتجاه أصابعك، كما في الهاتف. والإيقاف أثناء الكتابة يمنع راحة اليد التي تلامس لوحة اللمس من تحريك المؤشر وأنت تكتب.

# Small words under the touchpad's scroll speed.
settings-keyboard-scroll-touchpad-about = المسافة التي تتحركها الصفحة مع إصبعين.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = النقر بالطرق

# Small words under that switch.
settings-keyboard-tap-about = الطرقة الخفيفة تُحسب نقرة.

# Small words under natural scrolling for the touchpad.
settings-keyboard-natural-touchpad-about = تتبع الصفحة أصابعك، كما في الهاتف.

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = إيقاف أثناء الكتابة

# Small words under that switch.
settings-keyboard-quiet-about = راحة اليد الملامسة تترك المؤشر ثابتًا.

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = أبطأ

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = عادية

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = أسرع

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = منافذ USB

# Words behind the i of that card.
settings-keyboard-usb-about = عندما يفشل منفذ USB في البدء، يطفئه Trindle ثم يشغّله مرة واحدة لإعادة المحاولة، ولا يزيد على مرة واحدة للمنفذ الواحد.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error }؛ وتعذّرت إزالة علامته من جديد: { $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = لقطة الشاشة وقائمة الاختصارات

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = التقاط صورة للشاشة

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = إظهار قائمة الاختصارات

# Small words under each of the two shortcuts.
settings-keyboard-shortcuts-about = اكتب المفاتيح، أو اتركه فارغًا لعدم وجود اختصار.

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = التقاط صورة للشاشة الآن

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = إظهار القائمة الآن

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = إعادة تشغيل سطح المكتب

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = إعادة تشغيل الحلقة والأجزاء المساعدة

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = إعادة تشغيل كل شيء

# Small words under each of the two restart shortcuts. The example is keys a person types.
settings-keyboard-restart-about = اكتب المفاتيح، مثل Ctrl+Alt+Escape، أو اتركه فارغًا لعدم وجود اختصار.

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = مفاتيح { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = إعادة التشغيل الآن. تبقى النوافذ المفتوحة كما هي.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = إعادة تشغيل كل شيء. يسأل أولًا.

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = أُعيد التشغيل

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } مستخدم بالفعل في اختصار آخر.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = لم يُحفظ: { $why }

