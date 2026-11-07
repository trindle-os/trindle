
# machine translated (German), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = Benutzer

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = konten benutzer personen passwort bild foto administrator

# Help line at the foot of the Users page.
# SAFETY: needs a human review before it ships
settings-users-help = Zum Hinzufügen von Personen, Ändern von Passwörtern und Löschen von Konten wird das Passwort eines Administrators benötigt.

# Beside a change that is waiting for the administrator's password to be typed.
# SAFETY: needs a human review before it ships
settings-users-waiting = Warte auf das Passwort...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = Niemand hier ist Administrator, daher kann nichts für alle installiert oder geändert werden.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Nur du nutzt diesen Computer, und du bist sein Administrator.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Nur eine Person nutzt diesen Computer.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] { $n } Person nutzt diesen Computer, und du bist der Administrator.
       *[other] { $n } Personen nutzen diesen Computer, und du bist der Administrator.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] { $n } Person nutzt diesen Computer, und sie ist der Administrator.
       *[other] { $n } Personen nutzen diesen Computer, und eine davon ist der Administrator.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] { $n } Person nutzt diesen Computer, und { $a } davon sind Administratoren.
       *[other] { $n } Personen nutzen diesen Computer, und { $a } davon sind Administratoren.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = Das bist du.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Ihr Anmeldename.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Ein Bild auswählen

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Bilder

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = Das Passwort ändern

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = Löschen, samt ihren Dateien

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = Gelöscht

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = Administrator

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = Kann für alle Dinge installieren und ändern.

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
# SAFETY: needs a human review before it ships
settings-users-cannot-take-away = Du kannst dir das nicht selbst wegnehmen.

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = Jetzt Administrator

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = Kein Administrator mehr

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = Bild, Passwort und, für alle außer dir, das Löschen samt Dateien sind die runden Schaltflächen neben dem Namen. Ein Administrator kann Programme installieren und Dinge für alle ändern; du kannst dir das nicht selbst wegnehmen. Jede Änderung verlangt das Passwort eines Administrators.

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = Aktuelles Passwort

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = Neues Passwort

# Typing box in the change password form where the new password is typed again.
# SAFETY: needs a human review before it ships
settings-users-again = Noch einmal

# Button in the change password form that saves the new password.
settings-users-save = Speichern

# Spoken name of the Save button in the change password form.
# SAFETY: needs a human review before it ships
settings-users-save-new = Das neue Passwort speichern

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = Abbrechen

# Red words in the change password form when your current password was left empty.
# SAFETY: needs a human review before it ships
settings-users-need-current = Bitte auch dein aktuelles Passwort, damit deine gespeicherten Anmeldungen weiter funktionieren.

# Red words in the change password form when the new password is too short.
# SAFETY: needs a human review before it ships
settings-users-at-least-8 = Bitte mindestens 8 Zeichen.

# Red words in the change password form when the two new passwords differ.
# SAFETY: needs a human review before it ships
settings-users-no-match = Die beiden stimmen nicht überein.

# Said beside a person once their password is changed.
# SAFETY: needs a human review before it ships
settings-users-password-changed = Passwort geändert

# Red words after a password change when the saved logins could not take the new password.
# SAFETY: needs a human review before it ships
settings-users-not-rekeyed = Passwort geändert, aber gespeicherte Anmeldungen lassen sich noch mit dem alten öffnen.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = Bitte PNG, JPG oder WebP.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = Das Bild konnte nicht geändert werden.

# Said beside a person once their picture is saved.
settings-users-saved = Gespeichert

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = Das Bild erscheint bei der Anmeldung, konnte aber deinen anderen Programmen nicht zur Verfügung gestellt werden.

# Heading of the form for adding a person.
settings-users-add-someone = Jemanden hinzufügen

# Help text for the Add someone form (the information button).
# SAFETY: needs a human review before it ships
settings-users-add-about = Ein neues Konto für jemand anderen: der vollständige Name, ein Anmeldename aus Kleinbuchstaben und Ziffern, beginnend mit einem Buchstaben, und ein Passwort mit mindestens 8 Zeichen. Die Person kann sich sofort anmelden. Ein Administrator kann Programme installieren und Dinge für alle ändern.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = Neues Konto

# Under the New account heading.
settings-users-new-account-sub = Die Person kann sich sofort mit diesem Passwort anmelden.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Vollständiger Name

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Anmeldename, Kleinbuchstaben

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = Passwort

# Tip of the button that adds the new person.
settings-users-add-this = Diese Person hinzufügen

# Red words when the login name typed is not allowed.
settings-users-login-rules = Anmeldenamen bestehen aus Kleinbuchstaben und Ziffern und beginnen mit einem Buchstaben.

# Red words when the new person's password is too short.
# SAFETY: needs a human review before it ships
settings-users-password-8 = Das Passwort braucht mindestens 8 Zeichen.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } gibt es bereits.

# Said beside the Add someone form once the person is added.
settings-users-added = Hinzugefügt

# Heading of the Saved logins card.
# SAFETY: needs a human review before it ships
settings-users-saved-logins = Gespeicherte Anmeldungen

# Help text for the Saved logins card (the information button).
# SAFETY: needs a human review before it ships
settings-users-keyring-about = Die Passwörter, die Programme für dich aufbewahren, etwa für E-Mail oder WLAN, liegen in einem Schlüsselbund, der sich mit deiner Anmeldung öffnet. Alle wechseln bei der Anmeldung von selbst zum Schlüsselbund von Trindle. Falls deiner nicht verschoben werden konnte, kopiert das Verschieben jede gespeicherte Anmeldung, fragt nach dem Passwort jedes Schlüsselbunds, den deiner nicht öffnet, und die Dateien von GNOME bleiben unverändert.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Verwahrt vom Schlüsselbund von Trindle

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = Verwahrt vom Schlüsselbund von GNOME

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = Öffnet sich mit deiner Anmeldung.

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = Deine Anmeldung konnte sie nicht herüberbringen.

# Tip of the button that moves saved logins to Trindle's keyring.
settings-users-move-keyring = Zum Schlüsselbund von Trindle verschieben

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = Dein Passwort

# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = Los

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = Passwort für { $name }

# Red words when the move of saved logins was started without a password.
# SAFETY: needs a human review before it ships
settings-users-your-password-please = Bitte dein Passwort.

# Said beside the Saved logins line while they are copied.
settings-users-copying = Deine gespeicherten Anmeldungen werden kopiert...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } kopiert

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
# SAFETY: needs a human review before it ships
settings-users-nothing-moved =
    { $n ->
        [one] Noch nichts verschoben: { $names } braucht ein eigenes Passwort.
       *[other] Noch nichts verschoben: { $names } brauchen eigene Passwörter.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = Es wurde auf halbem Weg abgebrochen.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Konten werden angesehen…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = Personen

# Under the Administrator line of the placeholder card.
settings-users-looking = Wird gesucht…
