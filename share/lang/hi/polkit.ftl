# SAFETY: needs a human review before it ships
# machine translated (Hindi), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
polkit-title = एडमिनिस्ट्रेटर का पासवर्ड चाहिए

# SAFETY: needs a human review before it ships
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = { $user } का पासवर्ड

# SAFETY: needs a human review before it ships
# Button that lets the step go ahead once the password is typed.
polkit-allow = अनुमति दें

# SAFETY: needs a human review before it ships
# Button that stops the step and closes the question.
polkit-cancel = रद्द करें

# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = वह पासवर्ड स्वीकार नहीं हुआ। फिर कोशिश करें।

# Name a screen reader speaks for the password question window.
polkit-spoken = प्रमाणीकरण

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] एक और अनुरोध इसके पूरा होने की प्रतीक्षा में है।
       *[other] { $n } और अनुरोध इसके पूरा होने की प्रतीक्षा में हैं।
    }
