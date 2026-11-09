# machine translated (Arabic), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = البطارية عند { $pct }%

# Line under the low battery warning's title.
ring-battery-plug-in = صِل الشاحن قريبًا.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = جارٍ الشحن

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = فُصل الشاحن

# Line under the title when the charger comes out.
ring-battery-running = يعمل على البطارية، عند { $pct }%

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 دقائق

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = ساعة واحدة

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 ساعات

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = صباح الغد

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = أجّل 10 دقائق

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = أجّل ساعة واحدة

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = أجّل 6 ساعات

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = أجّل إلى صباح الغد

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = العودة إلى الرسالة

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = ألغِ التثبيت

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = ثبّت

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = أجّل

# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = امسح الكل

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = تم النسخ

# Tip of the button on a notification circle that copies its message.
ring-act-copy = انسخ الرسالة

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = انقر لتسليمه إلى مساعدك.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = سطح مكتب Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = شاشة دخول Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = شاشة قفل Trindle

# Name of Settings in the crash notice.
ring-crash-settings = إعدادات Trindle

# Name of Apps in the crash notice.
ring-crash-apps = تطبيقات Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = { $name } من Trindle

# Title of the notice that says a program crashed.
ring-crash-stopped = توقف { $name } بشكل غير متوقع

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = هذا القرص لم يعد موجودًا

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = تعذّر فتح { $name }

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = السكون

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = السبات

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = قفل الشاشة

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = لا شيء

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = إيقاف التشغيل

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = إغلاق الغطاء يفعل ما تقوله الإعدادات، الطاقة

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = اضغط مطولًا في أي مكان للتكبير

# Screen reader name of the magnifier's glass.
ring-magnify-name = المكبّر

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = أوقف التكبير

# Tip of the switch in the ring that turns the microphone off for every program.
# SAFETY: needs a human review before it ships
ring-mic-off-tip = أطفئ الميكروفون لكل البرامج

# Tip of the switch in the ring that turns the microphone back on.
# SAFETY: needs a human review before it ships
ring-mic-on-tip = أعد تشغيل الميكروفون

# Line in the ring when something other than the person pressed the microphone switch on.
# SAFETY: needs a human review before it ships
ring-mic-own-press = ضغطتك أنت وحدها تستطيع إعادة تشغيل الميكروفون.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = تعذّر تغيير مفتاح الميكروفون: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = لم يُعثر على برنامج

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = أرسله برنامج آخر

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = تعذّر تشغيله

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = نُسخ لمساعدك

# Screen reader name of a notification circle.
ring-notes-name = إشعار

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = جارٍ النقل. اترك للإسقاط، وEscape للإعادة.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = نُقل

# Said when Escape put a held notification back.
ring-notes-put-back = أُعيد

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = ثبّته لنقله

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] ثانية واحدة
        [two] ثانيتان
        [few] { $n } ثوانٍ
        [many] { $n } ثانية
        [zero] { $n } ثانية
       *[other] { $n } ثانية
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = حتى تغلقه

# Time under a notification from yesterday.
ring-notify-yesterday = أمس { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = أسفل اليمين

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = أسفل اليسار

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = أعلى اليسار

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = أعلى اليمين

# Title of the notice when a plugin that asks for permissions has been given none.
# SAFETY: needs a human review before it ships
ring-plugin-needs = { $name } يحتاج إلى إذنك

# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
# SAFETY: needs a human review before it ships
ring-plugin-cannot-work = لا يستطيع العمل بعد: المسموح 0 من { $asks }.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = تم النسخ. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = وصل إلى الشبكة

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = ملفاتك مفتوحة لديه

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
        [zero] { $names }
        [one] { $names } وواحد آخر
        [two] { $names } واثنان آخران
        [few] { $names } و{ $more } آخرون
        [many] { $names } و{ $more } آخرًا
       *[other] { $names } و{ $more } آخر
    }

