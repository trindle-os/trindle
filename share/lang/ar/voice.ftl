
# machine translated (Arabic), not yet reviewed by a person.

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = تم تشغيل { $name }

# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = تم إيقاف { $name }

# Said under the ring after "brighter" turns the screen up.
voice-brighter = أكثر سطوعًا

# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = أقل سطوعًا

# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = يمكن للشاشة أن تنام من جديد

# Said under the ring when the screen is kept awake.
voice-keep-awake-on = تبقى الشاشة مضاءة

# Said under the ring when the request is to look for updates.
voice-updates = جارٍ البحث عن تحديثات

# Said under the ring when nothing was heard at all.
voice-nothing-heard = لم يسمعك Trindle. اضغط وسط الحلقة وتكلّم.

# Said under the ring when the volume goes up.
voice-louder = أعلى صوتًا

# Said under the ring when the volume goes down.
voice-quieter = أخفض صوتًا

# Said under the ring when the sound is switched back on.
voice-sound-on = الصوت يعمل

# Said under the ring when the sound is muted.
voice-muted = الصوت مكتوم

# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = الخلفية التالية

# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = حُفظت لقطة الشاشة في الصور

# Said under the ring when the request is to show every app.
voice-all-apps = كل التطبيقات

# Said under the ring before shutting down, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-power-off = إيقاف التشغيل؟ انقر للتأكيد

# Said under the ring before restarting, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-reboot = إعادة التشغيل؟ انقر للتأكيد

# Said under the ring before logging out, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-logout = تسجيل الخروج؟ انقر للتأكيد

# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = جارٍ البحث عن { $what } في التطبيقات

# Said under the ring when the request opens Apps (the store).
voice-apps = التطبيقات

# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = الأقراص

# Said under the ring when the request opens Settings.
voice-settings = الإعدادات

# Said under the ring when the request closes the window in front.
voice-closing-window = جارٍ إغلاق النافذة

# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = جارٍ إغلاق { $name }

# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = لا يوجد تطبيق باسم { $name }

# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = جارٍ عرض { $name }

# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = جارٍ فتح { $name }

# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = لم يتعرّف Trindle على الأمر «{ $heard }»

# Said in the ring when voice has not been added yet.
voice-optional = الأوامر الصوتية إضافة اختيارية. أضفها من الإعدادات، في صفحة الحلقة.

# Said in the ring when voice has been added but is switched off.
voice-switched-off = الأوامر الصوتية متوقفة. شغّلها من الإعدادات، في صفحة الحلقة.

# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = تعذّر على Trindle الوصول إلى الميكروفون: { $why }

# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = تعذّر على Trindle الوصول إلى الميكروفون لأن برنامج التسجيل لم يبدأ

# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = لم يُسجَّل شيء.

# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = لم يسمعك Trindle. اضغط وسط الحلقة وتكلّم.

# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = فشل Whisper

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = جارٍ تثبيت أدوات البناء مؤقتًا: { $names }

# Progress line while whisper.cpp is built; its version comes in.
voice-building = جارٍ بناء whisper.cpp ‏{ $version } للأوامر الصوتية. يستغرق هذا دقيقة أو دقيقتين.

# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = جارٍ تنزيل نموذج الكلام (148 ميغابايت)

# Progress line while the build tools are taken away again.
voice-removing-tools = جارٍ إزالة أدوات البناء الآن

# Said once voice is added and ready to use.
voice-ready = التحكم الصوتي جاهز. افتح الحلقة، واضغط في وسطها، وتكلّم.

# Said once voice is removed.
voice-removed = أُزيل التحكم الصوتي.

# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = اضغط Enter للإغلاق.

# Shown when the command is run with neither install nor remove.
voice-usage = الاستخدام: trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = لم يعمل { $program }.

# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = تنزيل whisper.cpp ليس الذي توقعه Trindle، لذلك لم يُبنَ.

# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = تنزيل نموذج الكلام ليس الذي توقعه Trindle، لذلك لم يُثبَّت.

# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = لا يوجد مجلد خاص متاح.
