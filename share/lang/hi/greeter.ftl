
# machine translated (Hindi), not yet reviewed by a person.

# Name of the login screen's window (what a screen reader and the window list call it).
greeter-window-title = लॉग इन करें

# Sentence at the top of the panel, for someone this computer knows.
greeter-welcome-back = फिर स्वागत है।

# Sentence at the top of the panel, when the name has not been typed yet.
greeter-welcome = स्वागत है।

# Under the boxes, while there is nothing else to say.
# SAFETY: needs a human review before it ships
greeter-ready = अपना पासवर्ड लिखें, फिर Enter दबाएँ।

# Under the boxes, when a paired phone can stand in for the password.
# SAFETY: needs a human review before it ships
greeter-ready-phone = अपना पासवर्ड लिखें, या अपने फ़ोन से लॉग इन करने के लिए Enter दबाएँ।

# Under the boxes, before a name is in.
# SAFETY: needs a human review before it ships
greeter-ready-name = अपना नाम और पासवर्ड लिखें, फिर Enter दबाएँ।

# Red line under the boxes after the phone did not log the person in.
# SAFETY: needs a human review before it ships
greeter-phone-failed = आपके फ़ोन ने आपको लॉग इन नहीं किया। अपना पासवर्ड लिखें।

# Under the password box for a few seconds, where a phone is paired.
greeter-phone-hint = फ़ोन वाला बटन आपको आपके फ़ोन से लॉग इन करता है।

# At the foot of the screen, for when nothing else works.
greeter-trouble = लॉग इन में परेशानी? टेक्स्ट लॉगिन के लिए Ctrl+Alt+F2 दबाएँ।

# Beside the light drawing switch, on a computer with no graphics chip.
greeter-no-chip = कोई ग्राफ़िक्स चिप नहीं मिली, इसलिए हल्की ड्राइंग चीज़ों को तेज़ रखती है।

# The main button's name (a round button with an arrow) that logs in.
greeter-sign-in = साइन इन करें

# Red line under the boxes when the name and password are wrong, or the account is unknown.
# SAFETY: needs a human review before it ships
greeter-no-match = वह नाम और पासवर्ड मेल नहीं खाते।

# Red line under the boxes after too many wrong passwords.
greeter-pam-max-tries = बहुत ज़्यादा कोशिशें। एक मिनट रुकें, फिर कोशिश करें।

# Red line under the boxes when the account has run out.
greeter-pam-account-expired = इस खाते की अवधि समाप्त हो गई है।

# Red line under the boxes when the password must be changed before logging in.
# SAFETY: needs a human review before it ships
greeter-pam-password-expired = इस पासवर्ड की अवधि समाप्त हो गई है। इसे बदलने के लिए टेक्स्ट कंसोल (Ctrl+Alt+F2) पर लॉग इन करें।

# Red line under the boxes, or the sentence at the top, when the login failed and the system gave no reason.
greeter-login-failed = लॉग इन नहीं हो सका।

# Red line under the name box when Enter is pressed with no name.
greeter-name-first = पहले अपना नाम लिखें।

# Sentence at the top while waiting for the phone to answer.
greeter-asking-phone = आपके फ़ोन से पूछा जा रहा है…

# Sentence at the top while the system checks the password.
greeter-logging-in = लॉग इन हो रहा है…

# Typing box for the person's name, shown quietly while empty.
greeter-name-box = नाम

# Typing box for the password, shown quietly while empty.
# SAFETY: needs a human review before it ships
greeter-password-box = पासवर्ड

# Typing box for an extra question the login asked (a code, for one), when the question itself is empty.
greeter-answer-box = उत्तर

# Name and tip of the round button that logs in with a paired phone.
greeter-use-phone = फ़ोन इस्तेमाल करें

# Name of the switch for light drawing on a computer with no graphics chip, and the word under it.
greeter-light-drawing = हल्की ड्राइंग

# Tip of the round button that shows or hides the on-screen keyboard.
greeter-keyboard-tip = स्क्रीन पर कीबोर्ड

# Word under the on-screen keyboard button.
greeter-keyboard = कीबोर्ड

# Tip and word under the round button that turns the screen reader on or off.
greeter-screen-reader = स्क्रीन रीडर

# Word under a power button after the first press, and its tip.
greeter-press-again = फिर दबाएँ

# Tip and word under the round button that restarts the computer.
# SAFETY: needs a human review before it ships
greeter-restart = रीस्टार्ट

# Tip of the restart button after the first press.
# SAFETY: needs a human review before it ships
greeter-restart-sure = रीस्टार्ट करने के लिए फिर दबाएँ

# Tip and word under the round button that turns the computer off.
# SAFETY: needs a human review before it ships
greeter-shut-down = शट डाउन

# Tip of the shut down button after the first press.
# SAFETY: needs a human review before it ships
greeter-shut-down-sure = शट डाउन करने के लिए फिर दबाएँ

# Red line under the boxes when a power button is pressed and the system has no bus to ask.
greeter-no-bus = सिस्टम बस उपलब्ध नहीं है।

# Red line under the boxes when the system refused to restart; the reason follows, as the system worded it.
greeter-restart-failed = रीस्टार्ट नहीं हो सका: { $why }

# Red line under the boxes when the system refused to turn off; the reason follows, as the system worded it.
greeter-shut-down-failed = शट डाउन नहीं हो सका: { $why }

# Red line under the boxes when the login service could not be spoken to.
greeter-service-no-talk = लॉगिन सेवा से बात नहीं हो सकी।

# Red line under the boxes when the login service was too slow.
greeter-service-timeout = लॉगिन सेवा ने समय पर जवाब नहीं दिया।

# Red line under the boxes when the login service hung up.
greeter-service-closed = greetd ने कनेक्शन बंद कर दिया।

# Red line under the boxes when the login service's answer was far too large.
greeter-service-too-big = लॉगिन सेवा ने पढ़ने के लिए बहुत बड़ा जवाब दिया।

# Red line under the boxes when the login screen was started by hand and not by the login service.
greeter-service-not-started = GREETD_SOCK सेट नहीं है: लॉगिन स्क्रीन greetd ने शुरू नहीं की।

# Red line under the boxes when there is no connection to the login service.
greeter-service-no-connection = कोई कनेक्शन नहीं

