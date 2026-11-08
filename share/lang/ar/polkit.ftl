
# machine translated (Arabic), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card asking for the administrator's password.
polkit-title = كلمة مرور المسؤول مطلوبة

# SAFETY: needs a human review before it ships
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = كلمة مرور { $user }

# SAFETY: needs a human review before it ships
# Button that lets the step go ahead once the password is typed.
polkit-allow = سماح

# SAFETY: needs a human review before it ships
# Button that stops the step and closes the question.
polkit-cancel = إلغاء

# SAFETY: needs a human review before it ships
# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = كلمة المرور غير مقبولة. حاول مرة أخرى.

# Name a screen reader speaks for the password question window.
polkit-spoken = المصادقة

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [zero] لا طلبات أخرى تنتظر انتهاء هذا الطلب.
        [one] هناك طلب آخر ينتظر انتهاء هذا الطلب.
        [two] هناك طلبان آخران ينتظران انتهاء هذا الطلب.
        [few] هناك { $n } طلبات أخرى تنتظر انتهاء هذا الطلب.
        [many] هناك { $n } طلبًا آخر ينتظر انتهاء هذا الطلب.
       *[other] هناك { $n } طلب آخر ينتظر انتهاء هذا الطلب.
    }
