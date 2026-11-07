
# machine translated (French), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = Utilisateurs

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = comptes personnes mot de passe image photo administrateur

# SAFETY: needs a human review before it ships
# Help line at the foot of the Users page.
settings-users-help = Ajouter des personnes, changer des mots de passe et supprimer des comptes demandent le mot de passe d'un administrateur.

# SAFETY: needs a human review before it ships
# Beside a change that is waiting for the administrator's password to be typed.
settings-users-waiting = En attente du mot de passe...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = Personne ici n'est administrateur, donc rien ne peut être installé ni modifié pour tout le monde.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Vous seul utilisez cet ordinateur, et vous en êtes l'administrateur.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Une seule personne utilise cet ordinateur.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] { $n } personne utilise cet ordinateur, et vous en êtes l'administrateur.
       *[other] { $n } personnes utilisent cet ordinateur, et vous en êtes l'administrateur.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] { $n } personne utilise cet ordinateur, et elle en est l'administrateur.
       *[other] { $n } personnes utilisent cet ordinateur, et l'une d'elles en est l'administrateur.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] { $n } personne utilise cet ordinateur, dont { $a } administrateurs.
       *[other] { $n } personnes utilisent cet ordinateur, dont { $a } administrateurs.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = C'est vous.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Son nom de connexion.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Choisir une image

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Images

# SAFETY: needs a human review before it ships
# Tip of the round button on a person's card that changes their password.
settings-users-change-password = Changer le mot de passe

# SAFETY: needs a human review before it ships
# Tip of the round button on a person's card that deletes them and their files.
settings-users-delete = Supprimer, avec ses fichiers

# SAFETY: needs a human review before it ships
# Said beside a person once they are deleted.
settings-users-deleted = Supprimé

# SAFETY: needs a human review before it ships
# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
settings-users-administrator = Administrateur

# SAFETY: needs a human review before it ships
# Under the Administrator line: what an administrator can do.
settings-users-admin-can = Peut installer et modifier des choses pour tout le monde.

# SAFETY: needs a human review before it ships
# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
settings-users-cannot-take-away = Vous ne pouvez pas vous retirer ce droit.

# SAFETY: needs a human review before it ships
# Said beside a person after they are made an administrator.
settings-users-admin-now = Maintenant administrateur

# SAFETY: needs a human review before it ships
# Said beside a person after they stop being an administrator.
settings-users-admin-no-longer = N'est plus administrateur

# SAFETY: needs a human review before it ships
# Help text for a person's card (the information button).
settings-users-person-about = Son image, son mot de passe et, pour toute autre personne que vous, sa suppression avec ses fichiers sont les boutons ronds à côté de son nom. Un administrateur peut installer des programmes et modifier des choses pour tout le monde ; vous ne pouvez pas vous retirer ce droit. Chaque changement demande le mot de passe d'un administrateur.

# SAFETY: needs a human review before it ships
# Typing box in the change password form, for your own current password.
settings-users-current-password = Mot de passe actuel

# SAFETY: needs a human review before it ships
# Typing box in the change password form for the new password.
settings-users-new-password = Nouveau mot de passe

# SAFETY: needs a human review before it ships
# Typing box in the change password form where the new password is typed again.
settings-users-again = Encore une fois

# Button in the change password form that saves the new password.
settings-users-save = Enregistrer

# SAFETY: needs a human review before it ships
# Spoken name of the Save button in the change password form.
settings-users-save-new = Enregistrer le nouveau mot de passe

# SAFETY: needs a human review before it ships
# Tip of the button that closes a form on the Users page without saving.
settings-users-cancel = Annuler

# SAFETY: needs a human review before it ships
# Red words in the change password form when your current password was left empty.
settings-users-need-current = Votre mot de passe actuel aussi, pour que vos identifiants enregistrés continuent de fonctionner.

# SAFETY: needs a human review before it ships
# Red words in the change password form when the new password is too short.
settings-users-at-least-8 = Au moins 8 caractères, s'il vous plaît.

# SAFETY: needs a human review before it ships
# Red words in the change password form when the two new passwords differ.
settings-users-no-match = Les deux ne correspondent pas.

# SAFETY: needs a human review before it ships
# Said beside a person once their password is changed.
settings-users-password-changed = Mot de passe changé

# SAFETY: needs a human review before it ships
# Red words after a password change when the saved logins could not take the new password.
settings-users-not-rekeyed = Mot de passe changé, mais les identifiants enregistrés s'ouvrent encore avec l'ancien.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = PNG, JPG ou WebP, s'il vous plaît.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = L'image n'a pas pu être changée.

# Said beside a person once their picture is saved.
settings-users-saved = Enregistré

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = L'image s'affiche à la connexion, mais n'a pas pu être donnée à vos autres programmes.

# Heading of the form for adding a person.
settings-users-add-someone = Ajouter quelqu'un

# SAFETY: needs a human review before it ships
# Help text for the Add someone form (the information button).
settings-users-add-about = Un nouveau compte pour quelqu'un d'autre : son nom complet, un nom de connexion en lettres minuscules et chiffres, commençant par une lettre, et un mot de passe d'au moins 8 caractères. Cette personne peut se connecter tout de suite. Un administrateur peut installer des programmes et modifier des choses pour tout le monde.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = Nouveau compte

# SAFETY: needs a human review before it ships
# Under the New account heading.
settings-users-new-account-sub = Cette personne peut se connecter tout de suite avec ce mot de passe.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Nom complet

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Nom de connexion, en minuscules

# SAFETY: needs a human review before it ships
# Typing box in the Add someone form for the first password.
settings-users-password = Mot de passe

# Tip of the button that adds the new person.
settings-users-add-this = Ajouter cette personne

# Red words when the login name typed is not allowed.
settings-users-login-rules = Les noms de connexion sont des lettres minuscules et des chiffres, et commencent par une lettre.

# SAFETY: needs a human review before it ships
# Red words when the new person's password is too short.
settings-users-password-8 = Le mot de passe doit avoir au moins 8 caractères.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } existe déjà.

# Said beside the Add someone form once the person is added.
settings-users-added = Ajouté

# Heading of the Saved logins card.
settings-users-saved-logins = Identifiants enregistrés

# SAFETY: needs a human review before it ships
# Help text for the Saved logins card (the information button).
settings-users-keyring-about = Les mots de passe que les programmes gardent pour vous, par exemple pour le courrier ou le Wi-Fi, sont rangés dans un trousseau qui s'ouvre avec votre connexion. Chacun passe de lui-même au trousseau de Trindle à la connexion. Si le vôtre n'a pas pu être déplacé, le déplacement copie tous les identifiants enregistrés, en demandant le mot de passe de tout trousseau que le vôtre n'ouvre pas, et les fichiers de GNOME restent tels quels.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Gardés par le trousseau de Trindle

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = Gardés par le trousseau de GNOME

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = S'ouvre avec votre connexion.

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = Votre connexion n'a pas pu les déplacer.

# Tip of the button that moves saved logins to Trindle's keyring.
settings-users-move-keyring = Déplacer vers le trousseau de Trindle

# SAFETY: needs a human review before it ships
# Typing box for your password in the form for moving saved logins.
settings-users-your-password = Votre mot de passe

# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = Continuer

# SAFETY: needs a human review before it ships
# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
settings-users-password-for = Mot de passe pour { $name }

# SAFETY: needs a human review before it ships
# Red words when the move of saved logins was started without a password.
settings-users-your-password-please = Votre mot de passe, s'il vous plaît.

# Said beside the Saved logins line while they are copied.
settings-users-copying = Copie de vos identifiants enregistrés...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } copiés

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
settings-users-nothing-moved =
    { $n ->
        [one] Rien n'a encore été déplacé : { $names } a besoin de son propre mot de passe.
       *[other] Rien n'a encore été déplacé : { $names } ont besoin de leurs propres mots de passe.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = Cela s'est arrêté en cours de route.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Examen des comptes…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = Personnes

# Under the Administrator line of the placeholder card.
settings-users-looking = Examen…

