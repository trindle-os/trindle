# machine translated (Arabic), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = لا يوجد { $file } يمكن قراءته في أعلى المجلد ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = ليس في { $file } قسم [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } ليس مما يعرفه هذا Trindle: هو يعرف Interface={ $known }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] يحتاج إلى Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] يحتاج إلى Id من حروف وأرقام ونقاط وشرطات، مثل org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] يحتاج إلى Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] يحتاج إلى Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = Summary في [Plugin] سطر واحد.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } كلمات عادية في سطر واحد، بحرف واحد على الأكثر.
        [two] [{ $group }] { $key } كلمات عادية في سطر واحد، بحرفين على الأكثر.
        [few] [{ $group }] { $key } كلمات عادية في سطر واحد، بـ{ $max } أحرف على الأكثر.
        [many] [{ $group }] { $key } كلمات عادية في سطر واحد، بـ{ $max } حرفًا على الأكثر.
        [zero] [{ $group }] { $key } كلمات عادية في سطر واحد، بـ{ $max } حرف على الأكثر.
       *[other] [{ $group }] { $key } كلمات عادية في سطر واحد، بـ{ $max } حرف على الأكثر.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = الإذن { $name } ليس من الأذونات الموجودة: الموجودة هي { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] يحتاج إلى Icon: ملف SVG أو PNG في مجلد الإضافة.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = Icon في [Ring] يسمّي { $icon }، وهو ليس في مجلد الإضافة.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] يحتاج إلى Label، يُقال عند وقوف المؤشر عليه.

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] يحتاج إلى Run، ما تبدؤه النقرة.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] يحتاج إلى Fields، أسماء حقوله.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = اسم الحقل { $name } لا يجوز أن يحتوي إلا على حروف وأرقام وشرطات وشرطات سفلية.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = الحقل { $name } ليس له قسم [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] يحتاج إلى Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices على الأكثر { $most }، كل منها كلمات عادية بحرف واحد على الأكثر.
        [two] [{ $group }] Choices على الأكثر { $most }، كل منها كلمات عادية بحرفين على الأكثر.
        [few] [{ $group }] Choices على الأكثر { $most }، كل منها كلمات عادية بـ{ $chars } أحرف على الأكثر.
        [many] [{ $group }] Choices على الأكثر { $most }، كل منها كلمات عادية بـ{ $chars } حرفًا على الأكثر.
        [zero] [{ $group }] Choices على الأكثر { $most }، كل منها كلمات عادية بـ{ $chars } حرف على الأكثر.
       *[other] [{ $group }] Choices على الأكثر { $most }، كل منها كلمات عادية بـ{ $chars } حرف على الأكثر.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom اسم ملف في مجلد الإضافة الرئيسي: حروف وأرقام ونقاط وشرطات وشرطات سفلية، لا يبدأ بنقطة، ولا مجلدات.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] خيار متعدد، فيحتاج إلى Choices بخيارين أو أكثر، أو ChoicesFrom ملف.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = Default في [{ $group }] ليس من Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] ملاحظة، فتحتاج إلى TextFrom، اسم ملف في مجلد الإضافة الرئيسي: حروف وأرقام ونقاط وشرطات وشرطات سفلية، لا يبدأ بنقطة، ولا مجلدات.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] ملاحظة لا تحمل إعدادًا، فلا Default لها.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = Kind في [{ $group }] هو { $kind }: يجب أن يكون switch أو text أو choice أو note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] مفتاح تبديل، فـDefault فيه true أو false.

# Red words when the [Background] part is given without the background permission.
# SAFETY: needs a human review before it ships
plugins-background-needs-permission = [Background] يحتاج إلى إذن background في Permissions ضمن [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = الإضافة لا ترتبط بشيء: أعطها [Ring] أو [Settings] أو [Background] أو [Shortcut] أو إذن الإشعارات.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } يشير إلى خارج مجلد الإضافة.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = تعذّرت قراءة Run في [{ $group }] كأمر.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = Run في [{ $group }] يجب أن يسمّي برنامجًا داخل مجلد الإضافة، لا { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = Run في [{ $group }] يسمّي { $program }، وهو ليس برنامجًا تحمله الإضافة (مفقود، أو غير قابل للتنفيذ).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] Menu في [Ring] فيه عنصر واحد: يُسمح بـ{ $max } على الأكثر.
        [two] Menu في [Ring] فيه عنصران: يُسمح بـ{ $max } على الأكثر.
        [few] Menu في [Ring] فيه { $n } عناصر: يُسمح بـ{ $max } على الأكثر.
        [many] Menu في [Ring] فيه { $n } عنصرًا: يُسمح بـ{ $max } على الأكثر.
        [zero] Menu في [Ring] فيه { $n } عنصر: يُسمح بـ{ $max } على الأكثر.
       *[other] Menu في [Ring] فيه { $n } عنصر: يُسمح بـ{ $max } على الأكثر.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = معرّف القائمة { $id } لا يجوز أن يحتوي إلا على حروف وأرقام وشرطات وشرطات سفلية.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Menu في [Ring] يسمّي { $id } مرتين.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Menu في [Ring] يسمّي { $id }، وليس له قسم [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] Label في [{ $group }] سطر واحد قصير بحرف واحد على الأكثر.
        [two] Label في [{ $group }] سطر واحد قصير بحرفين على الأكثر.
        [few] Label في [{ $group }] سطر واحد قصير بـ{ $max } أحرف على الأكثر.
        [many] Label في [{ $group }] سطر واحد قصير بـ{ $max } حرفًا على الأكثر.
        [zero] Label في [{ $group }] سطر واحد قصير بـ{ $max } حرف على الأكثر.
       *[other] Label في [{ $group }] سطر واحد قصير بـ{ $max } حرف على الأكثر.
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] يحتاج إلى Run، ما يبدؤه اختياره.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-only-flow = [{ $group }] لـ Flow من فهرس Trindle فقط، الذي يطلب إذن التحكم.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] يحتاج إلى Run، الأمر الذي يُعطى الرقم السري PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] ShowWhen في [{ $group }] بحرف واحد على الأكثر.
        [two] ShowWhen في [{ $group }] بحرفين على الأكثر.
        [few] ShowWhen في [{ $group }] بـ{ $max } أحرف على الأكثر.
        [many] ShowWhen في [{ $group }] بـ{ $max } حرفًا على الأكثر.
        [zero] ShowWhen في [{ $group }] بـ{ $max } حرف على الأكثر.
       *[other] ShowWhen في [{ $group }] بـ{ $max } حرف على الأكثر.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = ShowWhen في [{ $group }] يُكتب field=value، مثل ShowWhen=mode=home: حقل آخر واحد والقيمة الواحدة التي يجب أن يحملها.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = ShowWhen في [{ $group }] يُكتب field=value، واسم الحقل لا يجوز أن يحتوي إلا على حروف وأرقام وشرطات وشرطات سفلية.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] ShowWhen في [{ $group }] يحتاج إلى قيمة من كلمات عادية في سطر واحد، بحرف واحد على الأكثر.
        [two] ShowWhen في [{ $group }] يحتاج إلى قيمة من كلمات عادية في سطر واحد، بحرفين على الأكثر.
        [few] ShowWhen في [{ $group }] يحتاج إلى قيمة من كلمات عادية في سطر واحد، بـ{ $max } أحرف على الأكثر.
        [many] ShowWhen في [{ $group }] يحتاج إلى قيمة من كلمات عادية في سطر واحد، بـ{ $max } حرفًا على الأكثر.
        [zero] ShowWhen في [{ $group }] يحتاج إلى قيمة من كلمات عادية في سطر واحد، بـ{ $max } حرف على الأكثر.
       *[other] ShowWhen في [{ $group }] يحتاج إلى قيمة من كلمات عادية في سطر واحد، بـ{ $max } حرف على الأكثر.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = ShowWhen في [{ $group }] يسمّي حقله هو: يجب أن يسمّي حقلًا آخر سابقًا.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = ShowWhen في [{ $group }] يسمّي الحقل { $other }، وهو ليس في Fields لهذه الإضافة.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = ShowWhen في [{ $group }] يسمّي الحقل { $other }، الذي يأتي بعده في Fields: يجب أن يكون حقلًا سابقًا.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = ShowWhen في [{ $group }] يسمّي الحقل { $other }، الذي له ShowWhen خاص به: لا يجوز أن يعتمد الحقل إلا على حقل بلا ShowWhen.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = ShowWhen في [{ $group }] يسمّي المفتاح { $other }، فقيمته true أو false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = ShowWhen في [{ $group }] قيمته { $value }، وهي ليست من Choices في { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = ShowWhen في [{ $group }] يسمّي الملاحظة { $other }، التي لا تحمل إعدادًا: لا يجوز أن يعتمد الحقل إلا على مفتاح أو خيار أو نص.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [الشبكة] الاستماع: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = Listen في [Network] يحتاج إلى إذن network في Permissions ضمن [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = Listen في [Network] يحتاج إلى إذن listen في Permissions ضمن [Plugin].

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = إذن listen يحتاج إلى Listen في [Network]، المنافذ التي يُسمح بدخولها.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = plugin.ini الخاص بها ليس لواجهة يعرفها هذا Trindle.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = لا تسمّي أي منافذ للاستماع.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = تعذّرت قراءة plugin.ini الخاص بها.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = لم يُسمح لها بالاستماع.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = المنافذ التي تسمّيها ليست التي سُمح بها. يجب السماح بها من جديد.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name } يحتاج إلى إذنك ليعمل: المسموح 0 من { $n }.

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } معه بعض الإذن الذي يطلبه فقط: المسموح { $a } من { $n }.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] لا تطلب شيئًا.
        [one] الشيء الواحد الذي تطلبه مسموح.
        [two] الشيئان اللذان تطلبهما مسموحان.
        [few] الأشياء الـ{ $n } التي تطلبها كلها مسموحة.
        [many] الأشياء الـ{ $n } التي تطلبها كلها مسموحة.
        [zero] الأشياء الـ{ $n } التي تطلبها كلها مسموحة.
       *[other] الأشياء الـ{ $n } التي تطلبها كلها مسموحة.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = اسمح

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
# SAFETY: needs a human review before it ships
plugins-other-ports = يُوصَل إليها على منافذ غير التي كانت من قبل

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = غير مثبّتة.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = إضافة أخرى

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = تلك الإضافة غير مثبّتة.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = تعذّر الحفظ: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = تعذّر نسخ ملفات الإضافة.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } جزؤها الذي يعمل في الخلفية لا يعمل: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = تعذّر العثور على trindle-plugin لتشغيلها.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] لم تبدأ خلال ثانية واحدة.
        [two] لم تبدأ خلال ثانيتين.
        [few] لم تبدأ خلال { $n } ثوانٍ.
        [many] لم تبدأ خلال { $n } ثانية.
        [zero] لم تبدأ خلال { $n } ثانية.
       *[other] لم تبدأ خلال { $n } ثانية.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = لا توجد جلسة سطح مكتب تعمل لتنضم إليها، فلم تبدأ. سجّل الدخول أولًا.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = لا تستطيع تحديد أي جلسة سطح مكتب تنضم إليها ({ $sessions })، فلم تبدأ. شغّلها مرة أخرى مع ضبط WAYLAND_DISPLAY على الصحيحة.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = يوجد مجلد حيث ينبغي أن يكون الملف

# Error text when a plugin's id is not an id.
plugins-not-an-id = ليس معرّف إضافة

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] Shortcuts في [Plugin] فيه اختصار واحد: يُسمح بـ{ $max } على الأكثر.
        [two] Shortcuts في [Plugin] فيه اختصاران: يُسمح بـ{ $max } على الأكثر.
        [few] Shortcuts في [Plugin] فيه { $n } اختصارات: يُسمح بـ{ $max } على الأكثر.
        [many] Shortcuts في [Plugin] فيه { $n } اختصارًا: يُسمح بـ{ $max } على الأكثر.
        [zero] Shortcuts في [Plugin] فيه { $n } اختصار: يُسمح بـ{ $max } على الأكثر.
       *[other] Shortcuts في [Plugin] فيه { $n } اختصار: يُسمح بـ{ $max } على الأكثر.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = معرّف الاختصار { $id } لا يجوز أن يحتوي إلا على حروف وأرقام وشرطات وشرطات سفلية.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts في [Plugin] يسمّي { $id } مرتين.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts في [Plugin] يسمّي { $id }، وليس له قسم [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Keys { $keys } في [{ $group }] اختصار من اختصارات Trindle بالفعل؛ اختر مفاتيح أخرى.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Keys { $keys } في [{ $group }] يستخدمها اختصار آخر من اختصارات الإضافة.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] يحتاج إلى Keys، مثل Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] يحتاج إلى Run، ما تبدؤه المفاتيح.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = المفاتيح المكتوبة له ليست اختصارًا. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } اختصار من اختصارات Trindle بالفعل.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } أحد اختصاراتك الخاصة بالفعل.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } تستخدمه { $holder } بالفعل.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = ذلك ليس اختصارًا من إضافة مثبّتة.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = تعذّر تفعيله ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = تعذّر الحفظ: لم يتغير شيء، لأن { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = تعذّر الحفظ ({ $why }).


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = إذن إمكانية الوصول مخصص فقط لقارئ الشاشة من Trindle في الفهرس.

