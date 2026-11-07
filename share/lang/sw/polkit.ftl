# machine translated (Swahili), not yet reviewed by a person.

# Heading of the card asking for the administrator's password.
# SAFETY: needs a human review before it ships
polkit-title = Nenosiri la msimamizi linahitajika

# Under the heading: whose password the box is for; the person's name comes in.
# SAFETY: needs a human review before it ships
polkit-password-for = Nenosiri la { $user }

# Button that lets the step go ahead once the password is typed.
# SAFETY: needs a human review before it ships
polkit-allow = Ruhusu

# Button that stops the step and closes the question.
# SAFETY: needs a human review before it ships
polkit-cancel = Ghairi

# Said when the password typed is not accepted, and the question is asked again.
# SAFETY: needs a human review before it ships
polkit-not-accepted = Nenosiri hilo halikukubaliwa. Jaribu tena.

# Name a screen reader speaks for the password question window.
polkit-spoken = Uthibitishaji

# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
        [one] Ombi lingine linasubiri hili liishe.
       *[other] Maombi mengine { $n } yanasubiri hili liishe.
    }
