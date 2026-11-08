# Trindle's administrator password question, the box that asks for the
# password when a step needs the administrator (polkit's agent in src/polkit).
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The password question (src/polkit/mod.rs)

# Heading of the card asking for the administrator's password.
polkit-title = Administrator's password needed
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = Password for { $user }
# Button that lets the step go ahead once the password is typed.
polkit-allow = Allow
# Button that stops the step and closes the question.
polkit-cancel = Cancel
# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = That password was not accepted. Try again.
# Name a screen reader speaks for the password question window.
polkit-spoken = Authentication
# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Another request is waiting for this one.
       *[other] { $n } more requests are waiting for this one.
    }
