# machine translated (Arabic), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = تحديثات الإضافات متوقفة: قائمة الإضافات تالفة.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = ابدأ من جديد

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = تعذّر إزالة السجل: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = احتوى اسم على محرف nul.

# Error text when there is no record name to take away.
plugins-start-no-note = لا توجد ملاحظة لإزالتها.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = لا يملك هذا Trindle مفتاحًا للتحقق من فهرس إضافاته، لذا لا يقبل أي فهرس.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = فهرس إضافات Trindle بلا توقيع، فلم يُستخدم.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = توقيع فهرس إضافات Trindle ليس بحجم التوقيع، فلم يُستخدم الفهرس.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = فهرس إضافات Trindle غير موقّع بمفتاح Trindle، فلم يُستخدم.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
# SAFETY: needs a human review before it ships
plugins-refused-no-number = فهرس إضافات Trindle لا يذكر رقمه، فلم يُستخدم.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = فهرس إضافات Trindle أقدم من الذي قُبل قبله (الرقم { $is }، بعد { $after })، فلم يُستخدم.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
# SAFETY: needs a human review before it ships
plugins-refused-unreadable = فهرس إضافات Trindle موقّع لكن لا يمكن قراءته، فلم يُستخدم: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } يُعرض آخر فهرس سليم.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = ليس نصًا.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = الفهرس أكبر من أن يكون فهرسًا.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = ليس فهرس إضافات: { $file } لا يحتوي على قسم [Catalogue] فيه Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = الفهرس من نوع أحدث مما يقرؤه هذا Trindle (Format={ $format }).

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } مستبعَد: { $why ->
        [id] معرّفه ليس حروفًا وأرقامًا ونقاطًا وشرطات
        [needs] يحتاج إلى Name وVersion
        [archive] Archive فيه ليس اسم ملف .tar.zst بجانب الفهرس
        [hash] SHA256 فيه ليس 64 رقمًا سداسيًا عشريًا
        [permission] يطلب إذنًا غير موجود
       *[icon] Icon فيه ليس اسم ملف SVG أو PNG بجانب الفهرس
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = تعذّر تشغيل tar: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = تعذّر فك الأرشيف: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = رفضه tar

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = ليس في الأرشيف { $file } في أعلاه، ولا في المجلد الوحيد في أعلاه.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = تعذّر صنع مجلد مؤقت: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = تعذّر تشغيل curl: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = تعذّر تنزيله: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = ملفه ليس الملف المذكور في الفهرس (SHA-256 مختلف)، فلم يُثبَّت شيء.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = لا يجتاز فحص الإضافات في Trindle، فلم يُثبَّت شيء: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
# SAFETY: needs a human review before it ships
plugins-install-other-id = يحتوي الأرشيف على { $found } وليس { $wanted }، فلم يُثبَّت شيء.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = ثُبّتت الإضافة، لكن تعذّرت كتابة الملاحظة بأنها جاءت من فهرس Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = ليست مثبّتة، فلا شيء لتحديثه.
