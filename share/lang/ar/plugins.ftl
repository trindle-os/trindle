# machine translated (Arabic), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = لا يستطيع Trindle قراءة { $file } في أعلى المجلد ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = ليس في { $file } قسم [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = يدعم هذا الإصدار من Trindle القيمة Interface={ $known }، وليس Interface={ $n }.

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
        [one] [{ $group }] يجب أن يكون { $key } نصًا عاديًا في سطر واحد، بما لا يزيد على حرف واحد.
        [two] [{ $group }] يجب أن يكون { $key } نصًا عاديًا في سطر واحد، بما لا يزيد على حرفين.
        [few] [{ $group }] يجب أن يكون { $key } نصًا عاديًا في سطر واحد، بما لا يزيد على { $max } أحرف.
        [many] [{ $group }] يجب أن يكون { $key } نصًا عاديًا في سطر واحد، بما لا يزيد على { $max } حرفًا.
        [zero] [{ $group }] يجب أن يكون { $key } نصًا عاديًا في سطر واحد، بما لا يزيد على { $max } حرف.
       *[other] [{ $group }] يجب أن يكون { $key } نصًا عاديًا في سطر واحد، بما لا يزيد على { $max } حرف.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
# SAFETY: needs a human review before it ships
plugins-check-permission-unknown = الإذن { $name } غير موجود. الأذونات الموجودة هي { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] يحتاج إلى Icon: ملف SVG أو PNG في مجلد الإضافة.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = Icon في [Ring] يسمّي { $icon }، لكنه ليس في مجلد الإضافة.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] يحتاج إلى Label، يظهر عند وقوف المؤشر عليه.

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] يحتاج إلى Fields، أسماء حقوله.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = يمكن أن يحتوي اسم الحقل { $name } على حروف وأرقام وشرطات وشرطات سفلية فقط.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = الحقل { $name } ليس له قسم [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] يحتاج إلى Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] يمكن أن يحتوي [{ $group }] على { $most } من Choices على الأكثر. يجب أن يكون كل واحد منها نصًا عاديًا لا يزيد على حرف واحد.
        [two] يمكن أن يحتوي [{ $group }] على { $most } من Choices على الأكثر. يجب أن يكون كل واحد منها نصًا عاديًا لا يزيد على حرفين.
        [few] يمكن أن يحتوي [{ $group }] على { $most } من Choices على الأكثر. يجب أن يكون كل واحد منها نصًا عاديًا لا يزيد على { $chars } أحرف.
        [many] يمكن أن يحتوي [{ $group }] على { $most } من Choices على الأكثر. يجب أن يكون كل واحد منها نصًا عاديًا لا يزيد على { $chars } حرفًا.
        [zero] يمكن أن يحتوي [{ $group }] على { $most } من Choices على الأكثر. يجب أن يكون كل واحد منها نصًا عاديًا لا يزيد على { $chars } حرف.
       *[other] يمكن أن يحتوي [{ $group }] على { $most } من Choices على الأكثر. يجب أن يكون كل واحد منها نصًا عاديًا لا يزيد على { $chars } حرف.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = ChoicesFrom في [{ $group }] يجب أن يسمّي ملفًا في مجلد الإضافة. استخدم الحروف والأرقام والنقاط والشرطات والشرطات السفلية فقط. لا تبدأ الاسم بنقطة ولا تضع فيه مسار مجلد.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] خيار متعدد، لذلك يحتاج إلى Choices بخيارين أو أكثر، أو إلى ChoicesFrom، وهو ملف.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = Default في [{ $group }] ليس من Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] ملاحظة، لذلك تحتاج إلى TextFrom ليسمّي ملفًا في مجلد الإضافة. استخدم الحروف والأرقام والنقاط والشرطات والشرطات السفلية فقط. لا تبدأ الاسم بنقطة ولا تضع فيه مسار مجلد.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] ملاحظة، لذلك لا تخزّن إعدادًا ولا يمكن أن يكون لها Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = Kind في [{ $group }] هو { $kind }، لكن يجب أن يكون switch أو text أو choice أو note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] مفتاح تبديل، لذلك يجب أن يكون Default فيه true أو false.

# Red words when the [Background] part is given without the background permission.
# SAFETY: needs a human review before it ships
plugins-background-needs-permission = [Background] يحتاج إلى إذن background في Permissions ضمن [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = الإضافة لا تتصل بأي شيء. أعطها [Ring] أو [Settings] أو [Background] أو [Shortcut] أو إذن notifications.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } رابط يقود إلى خارج مجلد الإضافة.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = لا يمكن قراءة Run في [{ $group }] كأمر.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = Run في [{ $group }] يجب أن يسمّي برنامجًا داخل مجلد الإضافة، لا { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = Run في [{ $group }] يسمّي { $program }، لكن الإضافة لا تحتوي على هذا البرنامج (فهو مفقود أو غير قابل للتنفيذ).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] Menu في [Ring] فيه عنصر واحد. يُسمح بـ{ $max } على الأكثر.
        [two] Menu في [Ring] فيه عنصران. يُسمح بـ{ $max } على الأكثر.
        [few] Menu في [Ring] فيه { $n } عناصر. يُسمح بـ{ $max } على الأكثر.
        [many] Menu في [Ring] فيه { $n } عنصرًا. يُسمح بـ{ $max } على الأكثر.
        [zero] Menu في [Ring] فيه { $n } عنصر. يُسمح بـ{ $max } على الأكثر.
       *[other] Menu في [Ring] فيه { $n } عنصر. يُسمح بـ{ $max } على الأكثر.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = يمكن أن يحتوي معرّف القائمة { $id } على حروف وأرقام وشرطات وشرطات سفلية فقط.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Menu في [Ring] يسمّي { $id } مرتين.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Menu في [Ring] يسمّي { $id }، وليس له قسم [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] يجب أن يكون Label في [{ $group }] سطرًا واحدًا قصيرًا لا يزيد على حرف واحد.
        [two] يجب أن يكون Label في [{ $group }] سطرًا واحدًا قصيرًا لا يزيد على حرفين.
        [few] يجب أن يكون Label في [{ $group }] سطرًا واحدًا قصيرًا لا يزيد على { $max } أحرف.
        [many] يجب أن يكون Label في [{ $group }] سطرًا واحدًا قصيرًا لا يزيد على { $max } حرفًا.
        [zero] يجب أن يكون Label في [{ $group }] سطرًا واحدًا قصيرًا لا يزيد على { $max } حرف.
       *[other] يجب أن يكون Label في [{ $group }] سطرًا واحدًا قصيرًا لا يزيد على { $max } حرف.
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
# SAFETY: needs a human review before it ships
plugins-pin-group-only-flow = [{ $group }] لـ Flow من فهرس Trindle فقط، الذي يطلب إذن control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] يحتاج إلى Run، الأمر الذي يُعطى الرقم السري PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] يمكن أن يصل ShowWhen في [{ $group }] إلى حرف واحد على الأكثر.
        [two] يمكن أن يصل ShowWhen في [{ $group }] إلى حرفين على الأكثر.
        [few] يمكن أن يصل ShowWhen في [{ $group }] إلى { $max } أحرف على الأكثر.
        [many] يمكن أن يصل ShowWhen في [{ $group }] إلى { $max } حرفًا على الأكثر.
        [zero] يمكن أن يصل ShowWhen في [{ $group }] إلى { $max } حرف على الأكثر.
       *[other] يمكن أن يصل ShowWhen في [{ $group }] إلى { $max } حرف على الأكثر.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = يجب أن يستخدم ShowWhen في [{ $group }] الصيغة field=value، مثل ShowWhen=mode=home. حدّد حقلًا آخر والقيمة التي يجب أن تكون له.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = يجب أن يُكتب ShowWhen في [{ $group }] بالصيغة field=value، ويمكن أن يحتوي اسم الحقل على حروف وأرقام وشرطات وشرطات سفلية فقط.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] يحتاج ShowWhen في [{ $group }] إلى قيمة نصية عادية في سطر واحد، لا تزيد على حرف واحد.
        [two] يحتاج ShowWhen في [{ $group }] إلى قيمة نصية عادية في سطر واحد، لا تزيد على حرفين.
        [few] يحتاج ShowWhen في [{ $group }] إلى قيمة نصية عادية في سطر واحد، لا تزيد على { $max } أحرف.
        [many] يحتاج ShowWhen في [{ $group }] إلى قيمة نصية عادية في سطر واحد، لا تزيد على { $max } حرفًا.
        [zero] يحتاج ShowWhen في [{ $group }] إلى قيمة نصية عادية في سطر واحد، لا تزيد على { $max } حرف.
       *[other] يحتاج ShowWhen في [{ $group }] إلى قيمة نصية عادية في سطر واحد، لا تزيد على { $max } حرف.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = ShowWhen في [{ $group }] يسمّي حقله هو. يجب أن يسمّي حقلًا آخر سابقًا.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = ShowWhen في [{ $group }] يسمّي الحقل { $other }، وهو ليس في Fields لهذه الإضافة.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = ShowWhen في [{ $group }] يسمّي الحقل { $other }، الذي يأتي بعده في Fields. يجب أن يكون حقلًا سابقًا.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = ShowWhen في [{ $group }] يسمّي الحقل { $other }، الذي له ShowWhen خاص به. لا يمكن أن يعتمد الحقل إلا على حقل بلا ShowWhen.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = ShowWhen في [{ $group }] يسمّي المفتاح { $other }، لذلك يجب أن تكون قيمته true أو false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = ShowWhen في [{ $group }] قيمته { $value }، وهي ليست من Choices في { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = ShowWhen في [{ $group }] يشير إلى الملاحظة { $other }، التي لا تخزّن إعدادًا. استخدم بدلًا منها مفتاحًا أو خيارًا أو حقل نص.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-network = Listen في [Network] يحتاج إلى إذن network في Permissions ضمن [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
# SAFETY: needs a human review before it ships
plugins-listen-needs-listen = Listen في [Network] يحتاج إلى إذن listen في Permissions ضمن [Plugin].

# Red words when a plugin has the listen permission and names no ports.
# SAFETY: needs a human review before it ships
plugins-listen-needs-ports = إذن listen يحتاج إلى Listen في [Network] لسرد المنافذ التي يمكن للإضافة استقبال الاتصالات عليها.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = يستخدم plugin.ini الخاص بها واجهة لا يدعمها هذا الإصدار من Trindle.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = لا تسرد أي منافذ للاستماع.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = لا يمكن قراءة plugin.ini الخاص بها.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = لم يُسمح لها بالاستماع.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = تغيّرت المنافذ المدرجة. عليك السماح بالاتصالات عليها من جديد.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none =
    { $n ->
        [one] { $name } يحتاج إلى إذنك قبل أن يعمل. لم تسمح بالإذن الوحيد الذي يطلبه.
        [two] { $name } يحتاج إلى إذنك قبل أن يعمل. لم تسمح بأي من الإذنين اللذين يطلبهما.
        [few] { $name } يحتاج إلى إذنك قبل أن يعمل. لم تسمح بأي من الأذونات الـ{ $n } التي يطلبها.
        [many] { $name } يحتاج إلى إذنك قبل أن يعمل. لم تسمح بأي من الأذونات الـ{ $n } التي يطلبها.
        [zero] { $name } يحتاج إلى إذنك قبل أن يعمل. لم تسمح بأي من الأذونات الـ{ $n } التي يطلبها.
       *[other] { $name } يحتاج إلى إذنك قبل أن يعمل. لم تسمح بأي من الأذونات الـ{ $n } التي يطلبها.
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = سمحت بـ{ $a } فقط من أصل { $n } من الأذونات التي يطلبها { $name }.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] لا تطلب شيئًا.
        [one] الإذن الذي تحتاجه مسموح.
        [two] الإذنان اللذان تطلبهما مسموحان.
        [few] كل الأذونات الـ{ $n } التي تطلبها مسموحة.
        [many] كل الأذونات الـ{ $n } التي تطلبها مسموحة.
        [zero] كل الأذونات الـ{ $n } التي تطلبها مسموحة.
       *[other] كل الأذونات الـ{ $n } التي تطلبها مسموحة.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = اسمح

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
# SAFETY: needs a human review before it ships
plugins-other-ports = يُوصَل إليها على منافذ مختلفة

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
plugins-swap-not-running = { $why } الإضافة لا تعمل في الخلفية: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = تعذّر على Trindle العثور على trindle-plugin لتشغيلها.

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
plugins-not-an-id = معرّف غير صالح

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] Shortcuts في [Plugin] فيه اختصار واحد. يُسمح بـ{ $max } على الأكثر.
        [two] Shortcuts في [Plugin] فيه اختصاران. يُسمح بـ{ $max } على الأكثر.
        [few] Shortcuts في [Plugin] فيه { $n } اختصارات. يُسمح بـ{ $max } على الأكثر.
        [many] Shortcuts في [Plugin] فيه { $n } اختصارًا. يُسمح بـ{ $max } على الأكثر.
        [zero] Shortcuts في [Plugin] فيه { $n } اختصار. يُسمح بـ{ $max } على الأكثر.
       *[other] Shortcuts في [Plugin] فيه { $n } اختصار. يُسمح بـ{ $max } على الأكثر.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = يمكن أن يحتوي معرّف الاختصار { $id } على حروف وأرقام وشرطات وشرطات سفلية فقط.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts في [Plugin] يسمّي { $id } مرتين.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts في [Plugin] يسمّي { $id }، وليس له قسم [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Keys { $keys } في [{ $group }] هو بالفعل أحد اختصارات Trindle. اختر مفاتيح أخرى.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Keys { $keys } في [{ $group }] مستخدم بالفعل في اختصار آخر في هذه الإضافة.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] يحتاج إلى Keys، مثل Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = هذه المجموعة من المفاتيح ليست اختصارًا صالحًا. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } هو بالفعل أحد اختصارات Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } أحد اختصاراتك الخاصة بالفعل.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } تستخدمه { $holder } بالفعل.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = ذلك ليس اختصارًا من إضافة مثبّتة.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = تعذّر على Trindle استخدامه ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = تعذّر على Trindle الحفظ لأن { $why }. لم يتغير شيء.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = تعذّر الحفظ: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = تعذّر إغلاق منافذها: { $why }.

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = تعذّر على Trindle فتح منافذها، لذلك أُوقف الاستماع مرة أخرى: { $why }.

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = لم تُفتح منافذها ({ $why })، وتعذّر أيضًا إيقاف الاستماع من جديد ({ $kept }). لا شيء مفتوح. أوقفه من الإعدادات، الأذونات.


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = قارئ الشاشة من Trindle في الفهرس وحده يمكنه استخدام إذن accessibility.

plugins-permission-twice = الإذن { $name } مذكور مرتين.

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = اطلب { $home } أو network، وليس الاثنين معًا: network يصل إلى الشبكة المحلية وإلى الإنترنت أيضًا.

plugins-needs-run = [{ $group }] يحتاج إلى Run، وهو الأمر الذي يُشغَّل.

plugins-swap-not-back = { $why } تعذّرت إعادة النسخة القديمة أيضًا: { $back } تبقى النسختان في مجلد الإضافة.
