
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = Usuários

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = contas pessoas senha foto imagem administrador

# Help line at the foot of the Users page.
settings-users-help = Adicionar pessoas, trocar senhas e excluir contas exigem a senha de um administrador.

# Beside a change that is waiting for the administrator's password to be typed.
settings-users-waiting = Aguardando a senha...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = Ninguém aqui é administrador, então nada pode ser instalado ou alterado para todos.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Só você usa este computador, e você é o administrador.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Só uma pessoa usa este computador.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] { $n } pessoa usa este computador, e você é o administrador.
        [many] { $n } de pessoas usam este computador, e você é o administrador.
       *[other] { $n } pessoas usam este computador, e você é o administrador.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] { $n } pessoa usa este computador, e ela é a administradora.
        [many] { $n } de pessoas usam este computador, e uma delas é a administradora.
       *[other] { $n } pessoas usam este computador, e uma delas é a administradora.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] { $n } pessoa usa este computador, e { $a } delas são administradoras.
        [many] { $n } de pessoas usam este computador, e { $a } delas são administradoras.
       *[other] { $n } pessoas usam este computador, e { $a } delas são administradoras.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = Este é você.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Nome de login.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Escolher uma foto

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Imagens

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = Trocar a senha

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = Excluir, com os arquivos da pessoa

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = Excluído

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = Administrador

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = Pode instalar e alterar coisas para todos.

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
# SAFETY: needs a human review before it ships
settings-users-cannot-take-away = Você não pode tirar isso de si mesmo.

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = Agora é administrador

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = Não é mais administrador

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = A foto, a senha e, para qualquer pessoa que não seja você, a exclusão com os arquivos são os botões redondos ao lado do nome. Um administrador pode instalar programas e alterar coisas para todos; você não pode tirar isso de si mesmo. Cada mudança pede a senha de um administrador.

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = Senha atual

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = Nova senha

# Typing box in the change password form where the new password is typed again.
settings-users-again = Repetir

# Button in the change password form that saves the new password.
settings-users-save = Salvar

# Spoken name of the Save button in the change password form.
settings-users-save-new = Salvar a nova senha

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = Cancelar

# Red words in the change password form when your current password was left empty.
settings-users-need-current = Digite também a sua senha atual, para que seus logins salvos continuem funcionando.

# Red words in the change password form when the new password is too short.
settings-users-at-least-8 = Use pelo menos 8 caracteres.

# Red words in the change password form when the two new passwords differ.
settings-users-no-match = As duas senhas não são iguais.

# Said beside a person once their password is changed.
settings-users-password-changed = Senha trocada

# Red words after a password change when the saved logins could not take the new password.
settings-users-not-rekeyed = Senha trocada, mas os logins salvos ainda abrem com a antiga.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = Use PNG, JPG ou WebP.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = Não foi possível trocar a foto.

# Said beside a person once their picture is saved.
settings-users-saved = Salvo

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = A foto aparece no login, mas não pôde ser passada para os seus outros programas.

# Heading of the form for adding a person.
settings-users-add-someone = Adicionar alguém

# Help text for the Add someone form (the information button).
# SAFETY: needs a human review before it ships
settings-users-add-about = Uma conta nova para outra pessoa: o nome completo, um nome de login com letras minúsculas e números, começando por uma letra, e uma senha de pelo menos 8 caracteres. A pessoa pode entrar logo em seguida. Um administrador pode instalar programas e alterar coisas para todos.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = Conta nova

# Under the New account heading.
settings-users-new-account-sub = A pessoa pode entrar logo com esta senha.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Nome completo

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Nome de login, em minúsculas

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = Senha

# Tip of the button that adds the new person.
settings-users-add-this = Adicionar esta pessoa

# Red words when the login name typed is not allowed.
settings-users-login-rules = Nomes de login têm letras minúsculas e números, começando por uma letra.

# Red words when the new person's password is too short.
settings-users-password-8 = A senha precisa ter pelo menos 8 caracteres.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } já existe.

# Said beside the Add someone form once the person is added.
settings-users-added = Adicionado

# Heading of the Saved logins card.
settings-users-saved-logins = Logins salvos

# Help text for the Saved logins card (the information button).
settings-users-keyring-about = As senhas que os programas guardam para você, como as do e-mail ou do Wi-Fi, ficam num chaveiro que abre com o seu login. Todos passam para o chaveiro do Trindle sozinhos ao entrar. Se o seu não pôde ser passado, a mudança copia todos os logins salvos, pedindo a senha de qualquer chaveiro que o seu não abra, e os arquivos do GNOME ficam como estão.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Guardados pelo chaveiro do Trindle

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = Guardados pelo chaveiro do GNOME

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = Abre com o seu login.

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = O seu login não conseguiu passá-los.

# Tip of the button that moves saved logins to Trindle's keyring.
settings-users-move-keyring = Passar para o chaveiro do Trindle

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = Sua senha

# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = Continuar

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = Senha de { $name }

# Red words when the move of saved logins was started without a password.
settings-users-your-password-please = Digite a sua senha.

# Said beside the Saved logins line while they are copied.
settings-users-copying = Copiando os seus logins salvos...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } copiados

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
settings-users-nothing-moved =
    { $n ->
        [one] Nada foi passado ainda: { $names } precisa da própria senha.
        [many] Nada foi passado ainda: { $names } precisam das próprias senhas.
       *[other] Nada foi passado ainda: { $names } precisam das próprias senhas.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = Parou no meio do caminho.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Verificando as contas…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = Pessoas

# Under the Administrator line of the placeholder card.
settings-users-looking = Verificando…
