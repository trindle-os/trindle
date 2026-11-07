# machine translated (Russian), not yet reviewed by a person.

# Window title when the keyring is started for the first time, with no keyring yet.
keyring-title-start = Запуск связки ключей

# Window title when the keyring is there and needs its password to open.
# SAFETY: needs a human review before it ships
keyring-title-unlock = Разблокировка связки ключей

# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
keyring-password = Пароль

# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
keyring-unlock = Разблокировать

# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
keyring-cancel = Отмена

# Shown under the password box when Unlock is pressed with the box empty.
keyring-type-first = Сначала введите пароль

# Shown under the password box while the password is being checked.
keyring-checking = Проверка…

# Under the title when no keyring exists yet and a program wants to keep a password.
# SAFETY: needs a human review before it ships
keyring-why-none = Программа хочет надёжно сохранить пароль. Введите пароль входа, чтобы запустить связку ключей; дальше она будет открываться сама при входе.

# Under the title when the keyring was made with a different password from the login one.
# SAFETY: needs a human review before it ships
keyring-why-old = Ваш пароль входа не тот, с которым создана эта связка ключей. Один раз введите старый пароль; после этого она будет открываться сама при входе.

# Under the title when no password was typed at log in, for example with a fingerprint or automatic log in.
# SAFETY: needs a human review before it ships
keyring-why-no-login = При входе пароль не вводился (отпечаток пальца или автоматический вход), поэтому связка ключей не открылась сама. Введите пароль входа.

# Under the title when a program asks for a saved password and the keyring is shut.
# SAFETY: needs a human review before it ships
keyring-why-wanted = Программа запрашивает сохранённый пароль. Введите пароль входа, чтобы разблокировать связку ключей.

# Shown under the box when the keyring has stopped before it could check the password.
keyring-stopped = Связка ключей остановилась

# Shown under the box when the password does not open the keyring.
# SAFETY: needs a human review before it ships
keyring-not-the-keyring-password = Это не пароль от связки ключей

# Shown under the box when the keyring file is damaged and has been left as it is.
keyring-damaged-left = Файл связки ключей повреждён, поэтому его не трогали

# Shown under the box when the password is not the person's login password.
# SAFETY: needs a human review before it ships
keyring-not-login-password = Это не ваш пароль входа

# Shown in Settings when the password typed there is not the person's password.
# SAFETY: needs a human review before it ships
keyring-not-your-password = Это не ваш пароль.

# Shown in Settings when the old keyring is still running after its time to stop has run out.
keyring-would-not-stop = Старая связка ключей не остановилась.

# Said when the keyring is locked and cannot be used.
# SAFETY: needs a human review before it ships
keyring-error-locked = Связка ключей заблокирована

# Said when the password given does not open the keyring.
# SAFETY: needs a human review before it ships
keyring-error-wrong-password = Этот пароль не открывает связку ключей

# Said when the collection or item asked for is not there.
keyring-error-no-such = Нет такой коллекции или записи

# Said when the keyring file cannot be read as a keyring.
keyring-error-damaged = Файл связки ключей повреждён


# Under the reason, naming the program that asked, when the keyring can find its name.
keyring-asked-by = Запрошено программой { $program }

# Told to a program that asked while the keyring was taking over from the one before and many calls already waited.
keyring-starting = Связка ключей запускается. Повторите запрос.

# The program in "Asked for by", for a program on the bus: $name is the name it gives itself, which anything can claim.
keyring-named-itself = программа, которая называет себя { $name }

