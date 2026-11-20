# machine translated (Arabic), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = تحديثات الإضافات متوقفة لأن قائمة الإضافات تالفة.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = ابدأ من جديد

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = لم يتمكن Trindle من إزالة السجل: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = يحتوي اسم على بايت فارغ (null).

# Error text when there is no record name to take away.
plugins-start-no-note = لا يوجد سجل لإزالته.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = لا يستطيع Trindle التحقق من فهرس الإضافات لأن مفتاح التحقق مفقود.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = فهرس إضافات Trindle بلا توقيع، لذلك لم يستخدمه Trindle.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = توقيع فهرس إضافات Trindle بحجم خاطئ، لذلك لم يستخدم Trindle الفهرس.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = فهرس إضافات Trindle غير موقّع بمفتاح Trindle، لذلك لم يستخدمه Trindle.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
# SAFETY: needs a human review before it ships
plugins-refused-no-number = فهرس إضافات Trindle بلا رقم إصدار، لذلك لم يستخدمه Trindle.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = لم يستخدم Trindle فهرس الإضافات هذا لأنه أقدم من فهرس استخدمه من قبل.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
# SAFETY: needs a human review before it ships
plugins-refused-unreadable = فهرس إضافات Trindle موقّع لكن لا يمكن قراءته، لذلك لم يستخدمه Trindle: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } يُعرض آخر فهرس سليم.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = ليس نصًا.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = الفهرس كبير جداً.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = ليس فهرس إضافات، لأن { $file } لا يحتوي على قسم [Catalogue] فيه Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = يستخدم هذا الفهرس صيغة أحدث مما يستطيع هذا الإصدار من Trindle قراءته. يدعم Trindle الصيغة Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } مستبعَد: { $why ->
        [id] معرّفه ليس مكوّناً من حروف وأرقام ونقاط وشرطات
        [needs] يحتاج إلى Name وVersion
        [archive] Archive فيه ليس اسم ملف .tar.zst بجانب الفهرس
        [hash] SHA256 فيه ليس 64 رقمًا سداسيًا عشريًا
        [permission] يطلب إذنًا غير موجود
       *[icon] Icon فيه ليس اسم ملف SVG أو PNG بجانب الفهرس
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = لم يبدأ tar: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = لم يتمكن Trindle من فك الأرشيف: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = السبب غير معروف

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = ليس في الأرشيف { $file } في المستوى الأعلى، ولا في المجلد الوحيد في المستوى الأعلى.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = لم يتمكن Trindle من إنشاء مجلد مؤقت: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = لم يبدأ curl: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = تعذّر تنزيله: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = الملف الذي نُزّل لا يطابق المجموع الاختباري (SHA-256) الوارد في الفهرس، لذلك لم يثبّت Trindle شيئاً.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = لم يثبّت Trindle شيئاً لأن الإضافة لم تجتز الفحوص: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
# SAFETY: needs a human review before it ships
plugins-install-other-id = يحتوي الأرشيف على { $found } وليس { $wanted }، لذلك لم يثبّت Trindle شيئاً.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = ثبّت Trindle الإضافة لكنه لم يتمكن من تسجيل أنها جاءت من فهرس Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = ليست مثبّتة، فلا شيء لتحديثه.
