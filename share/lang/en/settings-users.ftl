# Settings: the Users page (src/settings/panels/users.rs).

## The Users page (src/settings/panels/users.rs)

# Name of the Users page, in the page list and as its heading.
settings-users-title = Users
# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = accounts people password picture photo administrator
# Help line at the foot of the Users page.
settings-users-help = Adding people, changing passwords and deleting accounts need an administrator's password.
# Beside a change that is waiting for the administrator's password to be typed.
settings-users-waiting = Waiting for the password...
# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = Nobody here is an administrator, so nothing can be installed or changed for everyone.
# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Only you use this computer, and you are its administrator.
# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Only one person uses this computer.
# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] { $n } person uses this computer, and you are the administrator.
       *[other] { $n } people use this computer, and you are the administrator.
    }
# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] { $n } person uses this computer, and one of them is the administrator.
       *[other] { $n } people use this computer, and one of them is the administrator.
    }
# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] { $n } person uses this computer, and { $a } of them are administrators.
       *[other] { $n } people use this computer, and { $a } of them are administrators.
    }
# Under a person's name on their card when the card is yours.
settings-users-this-is-you = This is you.
# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Their login name.
# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Choose a picture
# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Pictures
# Tip of the round button on a person's card that changes their password.
settings-users-change-password = Change the password
# Tip of the round button on a person's card that deletes them and their files.
settings-users-delete = Delete, with their files
# Said beside a person once they are deleted.
settings-users-deleted = Deleted
# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
settings-users-administrator = Administrator
# Under the Administrator line: what an administrator can do.
settings-users-admin-can = Can install and change things for everyone.
# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
settings-users-cannot-take-away = You cannot take this away from yourself.
# Said beside a person after they are made an administrator.
settings-users-admin-now = Administrator now
# Said beside a person after they stop being an administrator.
settings-users-admin-no-longer = No longer an administrator
# Help text for a person's card (the information button).
settings-users-person-about = Their picture, their password and, for anyone but you, deleting them with their files are the round buttons beside their name. An administrator can install programs and change things for everyone; you cannot take this away from yourself. Each change asks for an administrator's password.
# Typing box in the change password form, for your own current password.
settings-users-current-password = Current password
# Typing box in the change password form for the new password.
settings-users-new-password = New password
# Typing box in the change password form where the new password is typed again.
settings-users-again = Again
# Button in the change password form that saves the new password.
settings-users-save = Save
# Spoken name of the Save button in the change password form.
settings-users-save-new = Save the new password
# Tip of the button that closes a form on the Users page without saving.
settings-users-cancel = Cancel
# Red words in the change password form when your current password was left empty.
settings-users-need-current = Your current password too, so your saved logins keep working.
# Red words in the change password form when the new password is too short.
settings-users-at-least-8 = At least 8 characters, please.
# Red words in the change password form when the two new passwords differ.
settings-users-no-match = The two do not match.
# Said beside a person once their password is changed.
settings-users-password-changed = Password changed
# Red words after a password change when the saved logins could not take the new password.
settings-users-not-rekeyed = Password changed, but saved logins still open with the old one.
# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = PNG, JPG or WebP, please.
# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = The picture could not be changed.
# Said beside a person once their picture is saved.
settings-users-saved = Saved
# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = The picture shows at login, but could not be given to your other programs.
# Heading of the form for adding a person.
settings-users-add-someone = Add someone
# Help text for the Add someone form (the information button).
settings-users-add-about = A new account for someone else: their full name, a login name in lower case letters and numbers, starting with a letter, and a password of at least 8 characters. They can log in straight away. An administrator can install programs and change things for everyone.
# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = New account
# Under the New account heading.
settings-users-new-account-sub = They can log in at once with this password.
# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Full name
# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Login name, lower case
# Typing box in the Add someone form for the first password.
settings-users-password = Password
# Tip of the button that adds the new person.
settings-users-add-this = Add this person
# Red words when the login name typed is not allowed.
settings-users-login-rules = Login names are lower case letters and numbers, starting with a letter.
# Red words when the new person's password is too short.
settings-users-password-8 = The password needs at least 8 characters.
# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } already exists.
# Said beside the Add someone form once the person is added.
settings-users-added = Added
# Heading of the Saved logins card.
settings-users-saved-logins = Saved logins
# Help text for the Saved logins card (the information button).
settings-users-keyring-about = The passwords programs keep for you, such as for mail or Wi-Fi, are kept in a keyring that opens with your login. Everyone moves to Trindle's keyring by themselves at login. If yours could not be moved, moving copies every saved login, asking for the password of any keyring yours does not open, and GNOME's files are left as they are.
# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Kept by Trindle's keyring
# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = Kept by GNOME's keyring
# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = Opens with your login.
# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = Your login could not move them across.
# Tip of the button that moves saved logins to Trindle's keyring.
settings-users-move-keyring = Move to Trindle's keyring
# Typing box for your password in the form for moving saved logins.
settings-users-your-password = Your password
# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = Go ahead
# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
settings-users-password-for = Password for { $name }
# Red words when the move of saved logins was started without a password.
settings-users-your-password-please = Your password, please.
# Said beside the Saved logins line while they are copied.
settings-users-copying = Copying your saved logins...
# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } copied
# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
settings-users-nothing-moved =
    { $n ->
        [one] Nothing moved yet: { $names } needs its own password.
       *[other] Nothing moved yet: { $names } need their own passwords.
    }
# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = It stopped part way.
# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Looking at the accounts…
# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = People
# Under the Administrator line of the placeholder card.
settings-users-looking = Looking…
