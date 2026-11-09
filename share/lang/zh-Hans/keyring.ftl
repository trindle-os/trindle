# machine translated (Simplified Chinese), not yet reviewed by a person.

# Window title when the keyring is started for the first time, with no keyring yet.
keyring-title-start = 启动密钥环

# Window title when the keyring is there and needs its password to open.
keyring-title-unlock = 解锁密钥环

# SAFETY: needs a human review before it ships
# Heading over the password box, and the tip shown in the box while it is empty.
keyring-password = 密码

# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
keyring-unlock = 解锁

# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
keyring-cancel = 取消

# Shown under the password box when Unlock is pressed with the box empty.
keyring-type-first = 请先输入密码

# Shown under the password box while the password is being checked.
keyring-checking = 正在检查…

# SAFETY: needs a human review before it ships
# Under the title when no keyring exists yet and a program wants to keep a password.
keyring-why-none = 有程序想要安全地保存一个密码。请输入您的登录密码来启动密钥环；之后每次登录时它会自动打开。

# SAFETY: needs a human review before it ships
# Under the title when the keyring was made with a different password from the login one.
keyring-why-old = 您的登录密码与创建这个密钥环时用的密码不同。请输入一次旧密码；之后它会在登录时自动打开。

# SAFETY: needs a human review before it ships
# Under the title when no password was typed at log in, for example with a fingerprint or automatic log in.
keyring-why-no-login = 登录时没有输入密码（使用了指纹或自动登录），所以密钥环无法自动打开。请输入您的登录密码。

# SAFETY: needs a human review before it ships
# Under the title when a program asks for a saved password and the keyring is shut.
keyring-why-wanted = 有程序想要使用已保存的密码。请输入您的登录密码来解锁密钥环。

# Shown under the box when the keyring has stopped before it could check the password.
keyring-stopped = 密钥环已停止

# SAFETY: needs a human review before it ships
# Shown under the box when the password does not open the keyring.
keyring-not-the-keyring-password = 这不是密钥环的密码

# Shown under the box when the keyring file is damaged and has been left as it is.
keyring-damaged-left = 密钥环文件已损坏，因此保持原样

# SAFETY: needs a human review before it ships
# Shown under the box when the password is not the person's login password.
keyring-not-login-password = 这不是您的登录密码

# SAFETY: needs a human review before it ships
# Shown in Settings when the password typed there is not the person's password.
keyring-not-your-password = 这不是您的密码。

# Shown in Settings when the old keyring is still running after its time to stop has run out.
keyring-would-not-stop = 旧的密钥环无法停止。

# Said when the keyring is locked and cannot be used.
keyring-error-locked = 密钥环已锁定

# SAFETY: needs a human review before it ships
# Said when the password given does not open the keyring.
keyring-error-wrong-password = 这个密码打不开密钥环

# Said when the collection or item asked for is not there.
keyring-error-no-such = 没有这个集合或项目

# Said when the keyring file cannot be read as a keyring.
keyring-error-damaged = 密钥环文件已损坏


# Under the reason, naming the program that asked, when the keyring can find its name.
keyring-asked-by = 由 { $program } 请求

# Told to a program that asked while the keyring was taking over from the one before and many calls already waited.
keyring-starting = 密钥环正在启动。请再请求一次。

# The program in "Asked for by", for a program on the bus: $name is the name it gives itself, which anything can claim.
keyring-named-itself = 一个自称 { $name } 的程序

