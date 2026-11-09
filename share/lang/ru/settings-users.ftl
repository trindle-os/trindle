# machine translated (Russian), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = Пользователи

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = учётные записи аккаунты люди пароль картинка фото аватар администратор

# Help line at the foot of the Users page.
# SAFETY: needs a human review before it ships
settings-users-help = Чтобы добавлять людей, менять пароли и удалять учётные записи, нужен пароль администратора.

# Beside a change that is waiting for the administrator's password to be typed.
# SAFETY: needs a human review before it ships
settings-users-waiting = Ждём пароль...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = Здесь нет ни одного администратора, поэтому для всех ничего нельзя установить или изменить.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Этим компьютером пользуетесь только вы, и вы его администратор.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Этим компьютером пользуется только один человек.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] Этим компьютером пользуется { $n } человек, и вы администратор.
        [few] Этим компьютером пользуются { $n } человека, и вы администратор.
        [many] Этим компьютером пользуются { $n } человек, и вы администратор.
       *[other] Этим компьютером пользуются { $n } человека, и вы администратор.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] Этим компьютером пользуется { $n } человек, и один из них администратор.
        [few] Этим компьютером пользуются { $n } человека, и один из них администратор.
        [many] Этим компьютером пользуются { $n } человек, и один из них администратор.
       *[other] Этим компьютером пользуются { $n } человека, и один из них администратор.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] Этим компьютером пользуется { $n } человек, администраторов среди них: { $a }.
        [few] Этим компьютером пользуются { $n } человека, администраторов среди них: { $a }.
        [many] Этим компьютером пользуются { $n } человек, администраторов среди них: { $a }.
       *[other] Этим компьютером пользуются { $n } человека, администраторов среди них: { $a }.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = Это вы.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Имя для входа.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Выбрать картинку

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Картинки

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = Сменить пароль

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = Удалить вместе с файлами

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = Удалён

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = Администратор

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = Может устанавливать и менять всё для всех.

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
# SAFETY: needs a human review before it ships
settings-users-cannot-take-away = Вы не можете лишить себя этого.

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = Теперь администратор

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = Больше не администратор

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = Картинка, пароль и (для всех, кроме вас) удаление вместе с файлами - это круглые кнопки рядом с именем. Администратор может устанавливать программы и менять всё для всех; вы не можете лишить себя этого. Для каждого изменения нужен пароль администратора.

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = Текущий пароль

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = Новый пароль

# Typing box in the change password form where the new password is typed again.
# SAFETY: needs a human review before it ships
settings-users-again = Ещё раз

# Button in the change password form that saves the new password.
settings-users-save = Сохранить

# Spoken name of the Save button in the change password form.
# SAFETY: needs a human review before it ships
settings-users-save-new = Сохранить новый пароль

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = Отмена

# Red words in the change password form when your current password was left empty.
# SAFETY: needs a human review before it ships
settings-users-need-current = Нужен и ваш текущий пароль, чтобы сохранённые пароли продолжали работать.

# Red words in the change password form when the new password is too short.
# SAFETY: needs a human review before it ships
settings-users-at-least-8 = Пожалуйста, не менее 8 символов.

# Red words in the change password form when the two new passwords differ.
# SAFETY: needs a human review before it ships
settings-users-no-match = Пароли не совпадают.

# Said beside a person once their password is changed.
# SAFETY: needs a human review before it ships
settings-users-password-changed = Пароль изменён

# Red words after a password change when the saved logins could not take the new password.
# SAFETY: needs a human review before it ships
settings-users-not-rekeyed = Пароль изменён, но сохранённые пароли всё ещё открываются старым.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = Пожалуйста, PNG, JPG или WebP.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = Не удалось изменить картинку.

# Said beside a person once their picture is saved.
settings-users-saved = Сохранено

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = Картинка видна при входе, но не удалось передать её вашим другим программам.

# Heading of the form for adding a person.
settings-users-add-someone = Добавить пользователя

# Help text for the Add someone form (the information button).
# SAFETY: needs a human review before it ships
settings-users-add-about = Новая учётная запись для другого человека: полное имя, имя для входа из строчных букв и цифр, начинающееся с буквы, и пароль не короче 8 символов. Войти можно сразу. Администратор может устанавливать программы и менять всё для всех.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = Новая учётная запись

# Under the New account heading.
settings-users-new-account-sub = С этим паролем можно войти сразу.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Полное имя

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Имя для входа, строчными буквами

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = Пароль

# Tip of the button that adds the new person.
settings-users-add-this = Добавить этого человека

# Red words when the login name typed is not allowed.
settings-users-login-rules = Имя для входа состоит из строчных букв и цифр и начинается с буквы.

# Red words when the new person's password is too short.
# SAFETY: needs a human review before it ships
settings-users-password-8 = Пароль должен быть не короче 8 символов.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = «{ $name }» уже существует.

# Said beside the Add someone form once the person is added.
settings-users-added = Добавлено

# Heading of the Saved logins card.
# SAFETY: needs a human review before it ships
settings-users-saved-logins = Сохранённые пароли

# Help text for the Saved logins card (the information button).
# SAFETY: needs a human review before it ships
settings-users-keyring-about = Пароли, которые программы хранят для вас, например от почты или Wi-Fi, лежат в связке ключей, которая открывается вашим входом. Каждый сам переходит на связку ключей Trindle при входе. Если вашу не удалось перенести, перенос копирует все сохранённые пароли и запрашивает пароль любой связки ключей, которую ваш вход не открывает; файлы GNOME остаются как есть.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-kept-trindle = Хранятся в связке ключей Trindle

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
# SAFETY: needs a human review before it ships
settings-users-kept-gnome = Хранятся в связке ключей GNOME

# Under the heading when the logins are kept by Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-opens-login = Открывается вашим входом.

# Under the heading when the logins are still kept by GNOME's keyring.
# SAFETY: needs a human review before it ships
settings-users-could-not-move = Ваш вход не смог их перенести.

# Tip of the button that moves saved logins to Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-move-keyring = Перенести в связку ключей Trindle

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = Ваш пароль

# Tip of the button that starts moving the saved logins.
# SAFETY: needs a human review before it ships
settings-users-go-ahead = Продолжить

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = Пароль для «{ $name }»

# Red words when the move of saved logins was started without a password.
# SAFETY: needs a human review before it ships
settings-users-your-password-please = Введите ваш пароль.

# Said beside the Saved logins line while they are copied.
# SAFETY: needs a human review before it ships
settings-users-copying = Копируем сохранённые пароли...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
# SAFETY: needs a human review before it ships
settings-users-copied = Скопировано: { $n }

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
# SAFETY: needs a human review before it ships
settings-users-nothing-moved =
    { $n ->
        [one] Пока ничего не перенесено: для «{ $names }» нужен собственный пароль.
        [few] Пока ничего не перенесено: для «{ $names }» нужны собственные пароли.
        [many] Пока ничего не перенесено: для «{ $names }» нужны собственные пароли.
       *[other] Пока ничего не перенесено: для «{ $names }» нужны собственные пароли.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = Остановилось на полпути.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Читаем учётные записи…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = Люди

# Under the Administrator line of the placeholder card.
settings-users-looking = Поиск…
