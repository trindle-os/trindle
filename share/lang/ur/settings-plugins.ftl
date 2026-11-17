
# machine translated (Urdu), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = آپ کے پلگ ان

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = پلگ ان ایڈ آن ایکسٹینشن انسٹال شدہ ہٹائیں ترتیبات plugins add-ons extensions installed remove settings

# Help line at the foot of the Your plugins page.
settings-plugins-help = پلگ ان Trindle میں اضافہ کرتے ہیں۔ پلگ ان حلقہ میں آئیکن، یہاں ترتیبات، اطلاعات، یا لاگ ان پر شروع ہونے والی کوئی چیز شامل کر سکتا ہے۔ ہر پلگ ان اپنے الگ ماحول میں چلتا ہے اور جب تک آپ اجازتوں میں اجازت نہ دیں کچھ استعمال نہیں کر سکتا۔ اجازتیں صفحے کے عنوان کے ساتھ والا بٹن کھولتا ہے۔ کسی پلگ ان کو ہٹانے کے لیے اس کے کارڈ پر ردی کا ڈبا دو بار دبائیں۔

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Trindle وہ ونڈو نہیں کھول سکا جو پوچھتی ہے کہ پلگ ان کیا استعمال کر سکتا ہے۔

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = ورژن { $v }۔

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = ورژن { $v }۔ { $settled }

# SAFETY: needs a human review before it ships
# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } کچھ نہیں مانگتا، اس لیے یہ اپنے الگ ماحول میں چلتا ہے اور اسے کچھ بھی اجازت نہیں ہے۔

# SAFETY: needs a human review before it ships
# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } { $list } مانگتا ہے۔ آپ نے { $asks } میں سے { $allowed } کی اجازت دی ہے۔ یہ اپنے الگ ماحول میں چلتا ہے، صرف اسی کے ساتھ جس کی آپ اجازت دیں۔
       *[other] { $name } { $list } مانگتا ہے۔ آپ نے { $asks } میں سے { $allowed } کی اجازت دی ہے۔ یہ اپنے الگ ماحول میں چلتا ہے، صرف اسی کے ساتھ جس کی آپ اجازت دیں۔
    }

# Heading of the line about a plugin on its card.
settings-plugins-about-it = بارے میں

# SAFETY: needs a human review before it ships
# Tip of the button that changes what a plugin may use.
settings-plugins-change = بدلیں کہ یہ کیا استعمال کر سکتا ہے

# SAFETY: needs a human review before it ships
# Tip of the button that removes a plugin.
settings-plugins-remove = یہ پلگ ان ہٹائیں

# SAFETY: needs a human review before it ships
# Red words when a plugin could not be removed.
settings-plugins-not-removed = یہ ہٹایا نہیں جا سکا۔

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = یہاں لکھیں

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } کی اپنی کوئی ترتیبات نہیں ہیں۔

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = اس وقت { $name } کی کوئی ترتیب دکھانے کو نہیں ہے۔

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] کوئی پلگ ان انسٹال نہیں ہے۔
        [one] ایک پلگ ان انسٹال ہے: { $list }۔
       *[other] { $n } پلگ ان انسٹال ہیں: { $list }۔
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = پلگ ان

# Heading of the line shown when no plugin is installed.
settings-plugins-none = کوئی انسٹال نہیں

# SAFETY: needs a human review before it ships
# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = اجازتیں کھولیں

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name } سے پوچھا جا رہا ہے…

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } جواب نہیں دے رہا۔ ہو سکتا ہے اس کی سروس رک گئی ہو۔

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = محفوظ ہو گیا۔

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } نے اسے قبول نہیں کیا۔

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } یہ نہیں کر سکا۔

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } یہ نہیں کر سکا: { $why }۔
