
# machine translated (Urdu), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = فولڈر کے اوپر کوئی پڑھنے کے قابل { $file } نہیں ہے ({ $why })۔

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } میں [Plugin] حصہ نہیں ہے۔

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } کو یہ Trindle نہیں جانتا: یہ Interface={ $known } جانتا ہے۔

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] کو Interface={ $known } چاہیے۔

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] کو حروف، ہندسوں، نقطوں اور ڈیشوں کی Id چاہیے، جیسے org.example.Weather۔

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] کو Name چاہیے۔

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] کو Version چاہیے۔

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary ایک سطر کی ہوتی ہے۔

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } ایک سطر کے سادہ الفاظ ہیں، زیادہ سے زیادہ { $max } حرف۔
       *[other] [{ $group }] { $key } ایک سطر کے سادہ الفاظ ہیں، زیادہ سے زیادہ { $max } حروف۔
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = اجازت { $name } موجود نہیں ہے: موجود اجازتیں یہ ہیں { $list }۔

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] کو Icon چاہیے: پلگ ان کے فولڈر میں SVG یا PNG فائل۔

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon میں { $icon } کا نام ہے، جو پلگ ان کے فولڈر میں نہیں ہے۔

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] کو Label چاہیے، جو پوائنٹر ٹھہرنے پر بتایا جاتا ہے۔

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] کو Run چاہیے، یعنی کلک سے کیا شروع ہو۔

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] کو Fields چاہییں، یعنی اس کے خانوں کے نام۔

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = خانے کے نام { $name } میں صرف حروف، ہندسے، ڈیش اور انڈر اسکور ہو سکتے ہیں۔

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = خانے { $name } کا [{ $group }] حصہ نہیں ہے۔

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] کو Label چاہیے۔

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] Choices زیادہ سے زیادہ { $most } ہیں، ہر ایک سادہ الفاظ کا، زیادہ سے زیادہ { $chars } حرف۔
       *[other] [{ $group }] Choices زیادہ سے زیادہ { $most } ہیں، ہر ایک سادہ الفاظ کا، زیادہ سے زیادہ { $chars } حروف۔
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom پلگ ان کے ہوم میں فائل کا نام ہے: حروف، ہندسے، نقطے، ڈیش اور انڈر اسکور، نقطے سے شروع نہیں، فولڈر نہیں۔

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] انتخاب ہے، اس لیے اسے دو یا زیادہ Choices چاہییں، یا ChoicesFrom میں فائل۔

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default اس کی Choices میں سے نہیں ہے۔

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] نوٹ ہے، اس لیے اسے TextFrom چاہیے، یعنی پلگ ان کے ہوم میں فائل کا نام: حروف، ہندسے، نقطے، ڈیش اور انڈر اسکور، نقطے سے شروع نہیں، فولڈر نہیں۔

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] نوٹ ہے، جس میں کوئی ترتیب نہیں ہوتی، اس لیے اس کا Default نہیں ہے۔

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind { $kind } ہے: اسے switch، text، choice یا note ہونا چاہیے۔

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] سوئچ ہے، اس لیے اس کا Default true یا false ہے۔

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] کو [Plugin] Permissions میں background اجازت چاہیے۔

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = پلگ ان کہیں نہیں جڑتا: اسے [Ring]، [Settings]، [Background]، [Shortcut] یا اطلاعات کی اجازت دیں۔

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } پلگ ان کے فولڈر سے باہر جاتا ہے۔

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run کو کمانڈ کے طور پر پڑھا نہیں جا سکتا۔

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run میں پلگ ان کے فولڈر کے اندر کا پروگرام ہونا چاہیے، { $program } نہیں۔

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run میں { $program } ہے، جو پلگ ان کا پروگرام نہیں (موجود نہیں، یا چل نہیں سکتا)۔

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] [Ring] Menu میں { $n } شے ہے: زیادہ سے زیادہ { $max } کی اجازت ہے۔
       *[other] [Ring] Menu میں { $n } اشیا ہیں: زیادہ سے زیادہ { $max } کی اجازت ہے۔
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = مینو id { $id } میں صرف حروف، ہندسے، ڈیش اور انڈر اسکور ہو سکتے ہیں۔

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu میں { $id } دو بار آیا ہے۔

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu میں { $id } ہے، جس کا [{ $group }] حصہ نہیں ہے۔

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] [{ $group }] Label ایک چھوٹی سطر ہے، زیادہ سے زیادہ { $max } حرف۔
       *[other] [{ $group }] Label ایک چھوٹی سطر ہے، زیادہ سے زیادہ { $max } حروف۔
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] کو Run چاہیے، یعنی اسے چننے سے کیا شروع ہو۔

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] صرف Trindle کی فہرست والے Flow کے لیے ہے، جو control اجازت مانگتا ہے۔

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] کو Run چاہیے، یعنی وہ کمانڈ جسے PIN دیا جاتا ہے۔

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] [{ $group }] ShowWhen زیادہ سے زیادہ { $max } حرف کا ہے۔
       *[other] [{ $group }] ShowWhen زیادہ سے زیادہ { $max } حروف کا ہے۔
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen کو field=value لکھا جاتا ہے، جیسے ShowWhen=mode=home: ایک اور خانہ اور وہ ایک قدر جو اس میں ہونی چاہیے۔

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen کو field=value لکھا جاتا ہے، اور خانے کے نام میں صرف حروف، ہندسے، ڈیش اور انڈر اسکور ہو سکتے ہیں۔

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] [{ $group }] ShowWhen کو ایک سطر کے سادہ الفاظ کی قدر چاہیے، زیادہ سے زیادہ { $max } حرف۔
       *[other] [{ $group }] ShowWhen کو ایک سطر کے سادہ الفاظ کی قدر چاہیے، زیادہ سے زیادہ { $max } حروف۔
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen اپنے ہی خانے کا نام لیتا ہے: اسے کسی دوسرے، پہلے والے خانے کا نام لینا چاہیے۔

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen خانہ { $other } کا نام لیتا ہے، جو اس پلگ ان کے Fields میں نہیں ہے۔

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen خانہ { $other } کا نام لیتا ہے، جو Fields میں اس کے بعد آتا ہے: وہ پہلے والا خانہ ہونا چاہیے۔

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen خانہ { $other } کا نام لیتا ہے، جس کا اپنا ShowWhen ہے: خانہ صرف ایسے خانے پر منحصر ہو سکتا ہے جس کا ShowWhen نہ ہو۔

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen سوئچ { $other } کا نام لیتا ہے، اس لیے اس کی قدر true یا false ہے۔

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen میں قدر { $value } ہے، جو { $other } کی Choices میں سے نہیں۔

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen نوٹ { $other } کا نام لیتا ہے، جس میں کوئی ترتیب نہیں: خانہ صرف سوئچ، انتخاب یا متن پر منحصر ہو سکتا ہے۔

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [نیٹ ورک] سننا: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen کو [Plugin] Permissions میں network اجازت چاہیے۔

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen کو [Plugin] Permissions میں listen اجازت چاہیے۔

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = listen اجازت کو [Network] Listen چاہیے، یعنی داخلے کے پورٹ۔

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = اس کا plugin.ini ایسے انٹرفیس کا نہیں جسے یہ Trindle جانتا ہو۔

# SAFETY: needs a human review before it ships
# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = یہ سننے کے لیے کسی پورٹ کا نام نہیں لیتا۔

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = اس کا plugin.ini پڑھا نہیں جا سکا۔

# SAFETY: needs a human review before it ships
# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = اسے سننے کی اجازت نہیں دی گئی۔

# SAFETY: needs a human review before it ships
# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = جن پورٹس کا یہ نام لیتا ہے وہ وہ نہیں جنہیں اجازت ملی تھی۔ انہیں دوبارہ اجازت دینی ہوگی۔

# SAFETY: needs a human review before it ships
# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none = { $name } کو کام کرنے کے لیے آپ کی اجازت چاہیے: { $n } میں سے 0 کی اجازت ہے۔

# SAFETY: needs a human review before it ships
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = { $name } کو اس کی مانگی ہوئی اجازت کا صرف کچھ حصہ ملا ہے: { $n } میں سے { $a } کی اجازت ہے۔

# SAFETY: needs a human review before it ships
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] یہ کچھ نہیں مانگتا۔
        [one] جو ایک چیز یہ مانگتا ہے اس کی اجازت ہے۔
       *[other] جو سب { $n } چیزیں یہ مانگتا ہے ان کی اجازت ہے۔
    }

# SAFETY: needs a human review before it ships
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = اجازت دیں

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = پہلے سے مختلف پورٹس پر رسائی

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = انسٹال نہیں ہے۔

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = کوئی اور پلگ ان

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = وہ پلگ ان انسٹال نہیں ہے۔

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = یہ محفوظ نہیں ہو سکا: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = پلگ ان کی فائلیں نقل نہیں ہو سکیں۔

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } اس کا پس منظر والا حصہ نہیں چل رہا: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = اسے شروع کرنے کے لیے trindle-plugin نہیں ملا۔

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] یہ { $n } سیکنڈ کے اندر شروع نہیں ہوا۔
       *[other] یہ { $n } سیکنڈ کے اندر شروع نہیں ہوا۔
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = کوئی ڈیسک ٹاپ سیشن نہیں چل رہا جس میں یہ شامل ہو، اس لیے یہ شروع نہیں ہوا۔ پہلے لاگ ان کریں۔

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = یہ نہیں جان سکتا کہ کس ڈیسک ٹاپ سیشن میں شامل ہو ({ $sessions })، اس لیے شروع نہیں ہوا۔ اسے WAYLAND_DISPLAY درست سیشن پر رکھ کر دوبارہ چلائیں۔

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = جہاں فائل ہونی چاہیے وہاں فولڈر ہے

# Error text when a plugin's id is not an id.
plugins-not-an-id = پلگ ان کی id نہیں ہے

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] [Plugin] Shortcuts میں { $n } شارٹ کٹ ہے: زیادہ سے زیادہ { $max } کی اجازت ہے۔
       *[other] [Plugin] Shortcuts میں { $n } شارٹ کٹ ہیں: زیادہ سے زیادہ { $max } کی اجازت ہے۔
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = شارٹ کٹ id { $id } میں صرف حروف، ہندسے، ڈیش اور انڈر اسکور ہو سکتے ہیں۔

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts میں { $id } دو بار آیا ہے۔

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts میں { $id } ہے، جس کا [{ $group }] حصہ نہیں ہے۔

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } پہلے ہی Trindle کا شارٹ کٹ ہے؛ دوسری کیز چنیں۔

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } پلگ ان کا دوسرا شارٹ کٹ استعمال کر رہا ہے۔

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] کو Keys چاہییں، جیسے Ctrl+Alt+S۔ { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] کو Run چاہیے، یعنی کیز سے کیا شروع ہو۔

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = جو کیز لکھی گئیں وہ شارٹ کٹ نہیں ہیں۔ { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } پہلے ہی Trindle کا شارٹ کٹ ہے۔

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } پہلے ہی آپ کے اپنے شارٹ کٹس میں سے ہے۔

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } پہلے ہی { $holder } استعمال کر رہا ہے۔

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = یہ کسی انسٹال شدہ پلگ ان کا شارٹ کٹ نہیں ہے۔

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = اسے استعمال میں نہیں لایا جا سکا ({ $why })۔

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = یہ محفوظ نہیں ہو سکا: کچھ نہیں بدلا، کیونکہ { $why }۔

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = یہ محفوظ نہیں ہو سکا ({ $why })۔


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = رسائی کی اجازت صرف فہرست میں موجود Trindle کے اسکرین ریڈر کے لیے ہے۔

