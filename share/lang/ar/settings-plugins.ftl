# machine translated (Arabic), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = إضافاتك

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = plugins add-ons extensions installed remove settings إضافات ملحقات امتدادات مثبتة إزالة إعدادات

# Help line at the foot of the Your plugins page.
settings-plugins-help = تضيف الإضافات إلى Trindle. يمكن للإضافة أن تضيف أيقونة إلى الحلقة، أو إعدادات هنا، أو إشعارات، أو شيئًا يبدأ عند تسجيل الدخول. تعمل كل إضافة في صندوقها الخاص ولا تستطيع استخدام أي شيء حتى تسمح لها في الأذونات. الزر بجانب عنوان الصفحة يفتح الأذونات. لإزالة إضافة، اضغط على سلة المهملات في بطاقتها مرتين.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = تعذّر على Trindle فتح النافذة التي تسأل عمّا يجوز للإضافة استخدامه.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = الإصدار { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = الإصدار { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
# SAFETY: needs a human review before it ships
settings-plugins-asks-nothing = { $name } لا يطلب شيئًا، لذا يعمل في صندوقه الخاص دون أي إذن.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [zero] { $name } يطلب { $list }. سمحت بـ { $allowed } من { $asks }. يعمل في صندوقه الخاص بما تسمح به فقط.
        [one] { $name } يطلب { $list }. سمحت بـ { $allowed } من { $asks }. يعمل في صندوقه الخاص بما تسمح به فقط.
        [two] { $name } يطلب { $list }. سمحت بـ { $allowed } من { $asks }. يعمل في صندوقه الخاص بما تسمح به فقط.
        [few] { $name } يطلب { $list }. سمحت بـ { $allowed } من { $asks }. يعمل في صندوقه الخاص بما تسمح به فقط.
        [many] { $name } يطلب { $list }. سمحت بـ { $allowed } من { $asks }. يعمل في صندوقه الخاص بما تسمح به فقط.
       *[other] { $name } يطلب { $list }. سمحت بـ { $allowed } من { $asks }. يعمل في صندوقه الخاص بما تسمح به فقط.
    }

# Heading of the line about a plugin on its card.
settings-plugins-about-it = نبذة

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = غيّر ما يجوز لها استخدامه

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = أزل هذه الإضافة

# Red words when a plugin could not be removed.
settings-plugins-not-removed = تعذّرت إزالتها.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = اكتب هنا

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } ليس له إعدادات خاصة.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } ليس له إعدادات لعرضها حاليًا.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] لا توجد إضافات مثبتة.
        [one] إضافة واحدة مثبتة: { $list }.
        [two] إضافتان مثبتتان: { $list }.
        [few] { $n } إضافات مثبتة: { $list }.
        [many] { $n } إضافة مثبتة: { $list }.
        [zero] { $n } إضافة مثبتة: { $list }.
       *[other] { $n } إضافة مثبتة: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = الإضافات

# Heading of the line shown when no plugin is installed.
settings-plugins-none = لا شيء مثبت

# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = افتح الأذونات

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = جارٍ سؤال { $name }…

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } لا يستجيب. ربما توقفت خدمته.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = حُفظ.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = لم يقبل { $name } ذلك.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = تعذّر على { $name } فعل ذلك.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = تعذّر على { $name } فعل ذلك: { $why }.
