# machine translated (Arabic), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = المستخدمون

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = accounts people password picture photo administrator حسابات أشخاص كلمة المرور صورة مسؤول

# Help line at the foot of the Users page.
# SAFETY: needs a human review before it ships
settings-users-help = إضافة أشخاص وتغيير كلمات المرور وحذف الحسابات تحتاج إلى كلمة مرور المسؤول.

# Beside a change that is waiting for the administrator's password to be typed.
# SAFETY: needs a human review before it ships
settings-users-waiting = بانتظار كلمة المرور...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = لا أحد هنا مسؤول، لذا لا يمكن تثبيت شيء أو تغييره للجميع.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = أنت وحدك تستخدم هذا الحاسوب، وأنت مسؤوله.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = شخص واحد فقط يستخدم هذا الحاسوب.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [zero] لا أحد يستخدم هذا الحاسوب، وأنت المسؤول.
        [one] شخص واحد يستخدم هذا الحاسوب، وأنت المسؤول.
        [two] شخصان يستخدمان هذا الحاسوب، وأنت المسؤول.
        [few] { $n } أشخاص يستخدمون هذا الحاسوب، وأنت المسؤول.
        [many] { $n } شخصًا يستخدمون هذا الحاسوب، وأنت المسؤول.
       *[other] { $n } شخص يستخدمون هذا الحاسوب، وأنت المسؤول.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [zero] لا أحد يستخدم هذا الحاسوب، وأحدهم مسؤول.
        [one] شخص واحد يستخدم هذا الحاسوب، وهو المسؤول.
        [two] شخصان يستخدمان هذا الحاسوب، وأحدهما المسؤول.
        [few] { $n } أشخاص يستخدمون هذا الحاسوب، وأحدهم المسؤول.
        [many] { $n } شخصًا يستخدمون هذا الحاسوب، وأحدهم المسؤول.
       *[other] { $n } شخص يستخدمون هذا الحاسوب، وأحدهم المسؤول.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [zero] لا أحد يستخدم هذا الحاسوب، والمسؤولون { $a }.
        [one] شخص واحد يستخدم هذا الحاسوب، والمسؤولون منهم { $a }.
        [two] شخصان يستخدمان هذا الحاسوب، والمسؤولون منهما { $a }.
        [few] { $n } أشخاص يستخدمون هذا الحاسوب، والمسؤولون منهم { $a }.
        [many] { $n } شخصًا يستخدمون هذا الحاسوب، والمسؤولون منهم { $a }.
       *[other] { $n } شخص يستخدمون هذا الحاسوب، والمسؤولون منهم { $a }.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = هذا أنت.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = اسم الدخول الخاص بهم.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = اختر صورة

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = الصور

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = غيّر كلمة المرور

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = احذف، مع ملفاتهم

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = تم الحذف

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = مسؤول

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = يستطيع تثبيت الأشياء وتغييرها للجميع.

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
# SAFETY: needs a human review before it ships
settings-users-cannot-take-away = لا يمكنك سحب هذا من نفسك.

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = أصبح مسؤولًا

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = لم يعد مسؤولًا

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = صورتهم وكلمة مرورهم، وحذفهم مع ملفاتهم لأي شخص غيرك، هي الأزرار المستديرة بجانب اسمهم. يستطيع المسؤول تثبيت البرامج وتغيير الأشياء للجميع؛ ولا يمكنك سحب هذا من نفسك. كل تغيير يطلب كلمة مرور المسؤول.

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = كلمة المرور الحالية

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = كلمة المرور الجديدة

# Typing box in the change password form where the new password is typed again.
settings-users-again = مرة أخرى

# Button in the change password form that saves the new password.
settings-users-save = حفظ

# Spoken name of the Save button in the change password form.
# SAFETY: needs a human review before it ships
settings-users-save-new = احفظ كلمة المرور الجديدة

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = إلغاء

# Red words in the change password form when your current password was left empty.
# SAFETY: needs a human review before it ships
settings-users-need-current = كلمة مرورك الحالية أيضًا، حتى تبقى تسجيلات دخولك المحفوظة تعمل.

# Red words in the change password form when the new password is too short.
# SAFETY: needs a human review before it ships
settings-users-at-least-8 = 8 أحرف على الأقل، من فضلك.

# Red words in the change password form when the two new passwords differ.
# SAFETY: needs a human review before it ships
settings-users-no-match = الاثنتان غير متطابقتين.

# Said beside a person once their password is changed.
# SAFETY: needs a human review before it ships
settings-users-password-changed = تم تغيير كلمة المرور

# Red words after a password change when the saved logins could not take the new password.
# SAFETY: needs a human review before it ships
settings-users-not-rekeyed = تم تغيير كلمة المرور، لكن تسجيلات الدخول المحفوظة ما زالت تُفتح بالقديمة.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = PNG أو JPG أو WebP، من فضلك.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = تعذّر تغيير الصورة.

# Said beside a person once their picture is saved.
settings-users-saved = تم الحفظ

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = تظهر الصورة عند تسجيل الدخول، لكن تعذّر إعطاؤها لبرامجك الأخرى.

# Heading of the form for adding a person.
settings-users-add-someone = أضف شخصًا

# Help text for the Add someone form (the information button).
# SAFETY: needs a human review before it ships
settings-users-add-about = حساب جديد لشخص آخر: اسمه الكامل، واسم دخول بأحرف إنجليزية صغيرة وأرقام يبدأ بحرف، وكلمة مرور من 8 أحرف على الأقل. يستطيع تسجيل الدخول فورًا. يستطيع المسؤول تثبيت البرامج وتغيير الأشياء للجميع.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = حساب جديد

# Under the New account heading.
# SAFETY: needs a human review before it ships
settings-users-new-account-sub = يستطيع تسجيل الدخول فورًا بكلمة المرور هذه.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = الاسم الكامل

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = اسم الدخول، بأحرف صغيرة

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = كلمة المرور

# Tip of the button that adds the new person.
settings-users-add-this = أضف هذا الشخص

# Red words when the login name typed is not allowed.
settings-users-login-rules = أسماء الدخول أحرف إنجليزية صغيرة وأرقام، وتبدأ بحرف.

# Red words when the new person's password is too short.
# SAFETY: needs a human review before it ships
settings-users-password-8 = تحتاج كلمة المرور إلى 8 أحرف على الأقل.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } موجود بالفعل.

# Said beside the Add someone form once the person is added.
settings-users-added = تمت الإضافة

# Heading of the Saved logins card.
# SAFETY: needs a human review before it ships
settings-users-saved-logins = تسجيلات الدخول المحفوظة

# Help text for the Saved logins card (the information button).
# SAFETY: needs a human review before it ships
settings-users-keyring-about = كلمات المرور التي تحفظها البرامج لك، مثل البريد أو Wi-Fi، تُحفظ في حلقة مفاتيح تُفتح بتسجيل دخولك. ينتقل الجميع إلى حلقة مفاتيح Trindle من تلقاء أنفسهم عند تسجيل الدخول. وإن تعذّر نقل حلقتك، فإن النقل ينسخ كل تسجيل دخول محفوظ، ويطلب كلمة مرور أي حلقة مفاتيح لا تفتحها حلقتك، وتُترك ملفات GNOME كما هي.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = محفوظة في حلقة مفاتيح Trindle

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = محفوظة في حلقة مفاتيح GNOME

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = تُفتح بتسجيل دخولك.

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = تعذّر على تسجيل دخولك نقلها.

# Tip of the button that moves saved logins to Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-move-keyring = انقل إلى حلقة مفاتيح Trindle

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = كلمة مرورك

# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = تابع

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = كلمة مرور { $name }

# Red words when the move of saved logins was started without a password.
# SAFETY: needs a human review before it ships
settings-users-your-password-please = كلمة مرورك، من فضلك.

# Said beside the Saved logins line while they are copied.
settings-users-copying = جارٍ نسخ تسجيلات دخولك المحفوظة...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = تم نسخ { $n }

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
# SAFETY: needs a human review before it ships
settings-users-nothing-moved =
    { $n ->
        [zero] لم يُنقل شيء بعد.
        [one] لم يُنقل شيء بعد: { $names } يحتاج إلى كلمة مروره الخاصة.
        [two] لم يُنقل شيء بعد: { $names } يحتاجان إلى كلمتي مرورهما الخاصتين.
        [few] لم يُنقل شيء بعد: { $names } تحتاج إلى كلمات مرورها الخاصة.
        [many] لم يُنقل شيء بعد: { $names } تحتاج إلى كلمات مرورها الخاصة.
       *[other] لم يُنقل شيء بعد: { $names } تحتاج إلى كلمات مرورها الخاصة.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = توقف في منتصف الطريق.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = جارٍ النظر في الحسابات…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = الأشخاص

# Under the Administrator line of the placeholder card.
# SAFETY: needs a human review before it ships
settings-users-looking = جارٍ النظر…

