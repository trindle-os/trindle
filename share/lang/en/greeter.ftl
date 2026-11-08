## The login screen (src/greeter): shown by greetd before anyone is logged in

# Name of the login screen's window (what a screen reader and the window list call it).
greeter-window-title = Log in
# Sentence at the top of the panel, for someone this computer knows.
greeter-welcome-back = Welcome back.
# Sentence at the top of the panel, when the name has not been typed yet.
greeter-welcome = Welcome.
# Under the boxes, while there is nothing else to say.
greeter-ready = Type your password, then press Enter.
# Under the boxes, when a paired phone can stand in for the password.
greeter-ready-phone = Type your password, or press Enter to use your phone.
# Under the boxes, before a name is in.
greeter-ready-name = Type your name and password, then press Enter.
# Red line under the boxes after the phone did not log the person in.
greeter-phone-failed = Your phone did not log you in. Type your password.
# Under the password box for a few seconds, where a phone is paired.
greeter-phone-hint = The phone button logs you in with your phone.
# At the foot of the screen, for when nothing else works.
greeter-trouble = Trouble logging in? Press Ctrl+Alt+F2 for a text login.
# Beside the light drawing switch, on a computer with no graphics chip.
greeter-no-chip = No graphics chip was found, so light drawing keeps things quick.
# The main button's name (a round button with an arrow) that logs in.
greeter-sign-in = Sign in
# Red line under the boxes when the name and password are wrong, or the account is unknown.
greeter-no-match = That name and password do not match.
# Red line under the boxes after too many wrong passwords.
greeter-pam-max-tries = Too many tries. Wait a minute, then try again.
# Red line under the boxes when the account has run out.
greeter-pam-account-expired = This account has expired.
# Red line under the boxes when the password must be changed before logging in.
greeter-pam-password-expired = This password has expired. Log in on a text console (Ctrl+Alt+F2) to change it.
# Red line under the boxes, or the sentence at the top, when the login failed and the system gave no reason.
greeter-login-failed = Logging in did not work.
# Red line under the name box when Enter is pressed with no name.
greeter-name-first = Type your name first.
# Sentence at the top while waiting for the phone to answer.
greeter-asking-phone = Asking your phone…
# Sentence at the top while the system checks the password.
greeter-logging-in = Logging in…
# Typing box for the person's name, shown quietly while empty.
greeter-name-box = Name
# Typing box for the password, shown quietly while empty.
greeter-password-box = Password
# Typing box for an extra question the login asked (a code, for one), when the question itself is empty.
greeter-answer-box = Answer
# Name and tip of the round button that logs in with a paired phone.
greeter-use-phone = Use phone
# Name of the switch for light drawing on a computer with no graphics chip, and the word under it.
greeter-light-drawing = Light drawing
# Tip of the round button that shows or hides the on-screen keyboard.
greeter-keyboard-tip = On-screen keyboard
# Word under the on-screen keyboard button.
greeter-keyboard = Keyboard
# Tip and word under the round button that turns the screen reader on or off.
greeter-screen-reader = Screen reader
# Word under a power button after the first press, and its tip.
greeter-press-again = Press again
# Tip and word under the round button that restarts the computer.
greeter-restart = Restart
# Tip of the restart button after the first press.
greeter-restart-sure = Press again to restart
# Tip and word under the round button that turns the computer off.
greeter-shut-down = Shut down
# Tip of the shut down button after the first press.
greeter-shut-down-sure = Press again to shut down
# Red line under the boxes when a power button is pressed and the system has no bus to ask.
greeter-no-bus = The system bus is not available.
# Red line under the boxes when the system refused to restart; the reason follows, as the system worded it.
greeter-restart-failed = Could not restart: { $why }
# Red line under the boxes when the system refused to turn off; the reason follows, as the system worded it.
greeter-shut-down-failed = Could not shut down: { $why }
# Red line under the boxes when the login service could not be spoken to.
greeter-service-no-talk = Could not talk to the login service.
# Red line under the boxes when the login service was too slow.
greeter-service-timeout = The login service did not answer in time.
# Red line under the boxes when the login service hung up.
greeter-service-closed = greetd closed the connection.
# Red line under the boxes when the login service's answer was far too large.
greeter-service-too-big = The login service answered with something too big to read.
# Red line under the boxes when the login screen was started by hand and not by the login service.
greeter-service-not-started = GREETD_SOCK is not set: the login screen was not started by greetd.
# Red line under the boxes when there is no connection to the login service.
greeter-service-no-connection = no connection
