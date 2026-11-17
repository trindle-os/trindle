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
polkit-not-accepted = वह पासवर्ड सही नहीं था। फिर कोशिश करें।

# Name a screen reader speaks for the password question window.
polkit-spoken = आपका पासवर्ड

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] एक और अनुरोध इंतज़ार में है।
       *[other] { $n } और अनुरोध इंतज़ार में हैं।
    }

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = जगह बनाने से पहले स्नैपशॉट लेने के लिए ड्राइव को एडमिनिस्ट्रेटर का पासवर्ड चाहिए।

# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = जगह बनाते समय स्वैप फ़ाइल बंद करने के लिए ड्राइव को एडमिनिस्ट्रेटर का पासवर्ड चाहिए।

# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = स्वैप फ़ाइल को फिर से चालू करने के लिए ड्राइव को एडमिनिस्ट्रेटर का पासवर्ड चाहिए।
