
# machine translated (French), not yet reviewed by a person.

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = Saisissez votre code PIN

# Heading of the card when the password is the way in.
session-lock-locked = Verrouillé

# Under the heading of the password card: what to do.
session-lock-type-password = Saisissez votre mot de passe, puis appuyez sur Entrée

# Button under the PIN card that changes to typing the password.
session-lock-use-password = Utiliser le mot de passe

# Button under the password card that changes to typing a PIN.
session-lock-use-pin = Utiliser le code PIN

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Utiliser le téléphone

# Shown while the typed password or PIN is being checked.
session-lock-checking = Vérification…

# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = Le code PIN a expiré. Saisissez votre mot de passe.

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = Trop de codes PIN erronés. Utilisez votre mot de passe.

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] Code PIN erroné. { $n } essai restant.
        [many] Code PIN erroné. { $n } essais restants.
       *[other] Code PIN erroné. { $n } essais restants.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Demande envoyée à votre téléphone…

# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Trindle n'a pas pu déverrouiller avec votre téléphone. Utilisez votre code PIN.

# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Trindle n'a pas pu déverrouiller avec votre téléphone. Saisissez votre mot de passe.

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = Ce mot de passe n'est pas le bon.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Un instant, s'il vous plaît

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Cette image est plus grande que votre écran et utilise de la mémoire en plus.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Les fonds d'écran animés utilisent plus de mémoire et de batterie.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = plus grande que la 4K

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = plus grande que votre écran

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = plus de 20 Mo (200 Mo si animée)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = plus de 20 Mo

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = plus de 200 Mo

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } ou { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out =
    { $count ->
        [one] Trindle a ignoré { $count } image : { $why }.
       *[other] Trindle a ignoré { $count } images : { $why }.
    }

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Cela a pris trop de temps. Réessayez.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Redémarrer tout le bureau ?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Toutes les fenêtres ouvertes se fermeront. Tout ce que vous n'avez pas enregistré sera perdu.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = Annuler

# Main button that restarts the whole desktop.
session-restart-go = Redémarrer

# Under the lock screen when Enter is pressed while the last try, given up on, is still being checked.
session-lock-still-checking = Votre dernier essai est encore en cours de vérification. Réessayez dans un instant.

# Under the lock screen after several wrong tries, with how many seconds to wait (1 is one second).
session-lock-wait =
    { $n ->
        [one] Trop d'essais incorrects. Réessayez dans { $n } seconde.
       *[other] Trop d'essais incorrects. Réessayez dans { $n } secondes.
    }
