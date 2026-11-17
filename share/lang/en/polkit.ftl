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
polkit-not-accepted = That password wasn't right. Try again.
# Name a screen reader speaks for the password question window.
polkit-spoken = Your password
# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Another request is waiting.
       *[other] { $n } more requests are waiting.
    }

## Why Drives asks (src/polkit/mod.rs)

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Drives needs an administrator's password to take a snapshot before making room.
# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Drives needs an administrator's password to switch the swap file off while it makes room.
# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Drives needs an administrator's password to switch the swap file back on.
