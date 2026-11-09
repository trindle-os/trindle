
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = 用户

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = 账户 人员 密码 头像 照片 管理员

# Help line at the foot of the Users page.
# SAFETY: needs a human review before it ships
settings-users-help = 添加用户、更改密码和删除账户都需要管理员密码。

# Beside a change that is waiting for the administrator's password to be typed.
# SAFETY: needs a human review before it ships
settings-users-waiting = 正在等待输入密码...

# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = 这里没有人是管理员，所以无法为所有人安装或更改任何内容。

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = 只有你使用这台电脑，并且你是它的管理员。

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = 只有一个人使用这台电脑。

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
       *[other] 有 { $n } 人使用这台电脑，你是管理员。
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
       *[other] 有 { $n } 人使用这台电脑，其中一人是管理员。
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
       *[other] 有 { $n } 人使用这台电脑，其中 { $a } 人是管理员。
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = 这是你。

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = 他们的登录名。

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = 选择头像

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = 图片

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = 更改密码

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = 删除，连同其文件

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = 已删除

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = 管理员

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = 可以为所有人安装和更改内容。

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
# SAFETY: needs a human review before it ships
settings-users-cannot-take-away = 你不能取消自己的管理员权限。

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = 现为管理员

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = 不再是管理员

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = 头像、密码，以及（对你以外的人）连同文件一起删除，都是其名字旁边的圆形按钮。管理员可以安装程序并为所有人更改内容；你不能取消自己的管理员权限。每项更改都需要管理员密码。

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = 当前密码

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = 新密码

# Typing box in the change password form where the new password is typed again.
# SAFETY: needs a human review before it ships
settings-users-again = 再次输入

# Button in the change password form that saves the new password.
settings-users-save = 保存

# Spoken name of the Save button in the change password form.
# SAFETY: needs a human review before it ships
settings-users-save-new = 保存新密码

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = 取消

# Red words in the change password form when your current password was left empty.
# SAFETY: needs a human review before it ships
settings-users-need-current = 也请输入你当前的密码，这样你保存的登录信息才能继续使用。

# Red words in the change password form when the new password is too short.
# SAFETY: needs a human review before it ships
settings-users-at-least-8 = 请至少输入 8 个字符。

# Red words in the change password form when the two new passwords differ.
# SAFETY: needs a human review before it ships
settings-users-no-match = 两次输入不一致。

# Said beside a person once their password is changed.
# SAFETY: needs a human review before it ships
settings-users-password-changed = 密码已更改

# Red words after a password change when the saved logins could not take the new password.
# SAFETY: needs a human review before it ships
settings-users-not-rekeyed = 密码已更改，但保存的登录信息仍用旧密码打开。

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = 请选择 PNG、JPG 或 WebP。

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = 无法更改头像。

# Said beside a person once their picture is saved.
settings-users-saved = 已保存

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = 头像会在登录时显示，但无法提供给你的其他程序。

# Heading of the form for adding a person.
settings-users-add-someone = 添加用户

# Help text for the Add someone form (the information button).
# SAFETY: needs a human review before it ships
settings-users-add-about = 为他人创建新账户：需要其全名、由小写字母和数字组成且以字母开头的登录名，以及至少 8 个字符的密码。他们可以立即登录。管理员可以安装程序并为所有人更改内容。

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = 新账户

# Under the New account heading.
# SAFETY: needs a human review before it ships
settings-users-new-account-sub = 他们可以立即用这个密码登录。

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = 全名

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = 登录名，小写

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = 密码

# Tip of the button that adds the new person.
settings-users-add-this = 添加此用户

# Red words when the login name typed is not allowed.
settings-users-login-rules = 登录名由小写字母和数字组成，且以字母开头。

# Red words when the new person's password is too short.
# SAFETY: needs a human review before it ships
settings-users-password-8 = 密码至少需要 8 个字符。

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } 已存在。

# Said beside the Add someone form once the person is added.
settings-users-added = 已添加

# Heading of the Saved logins card.
# SAFETY: needs a human review before it ships
settings-users-saved-logins = 保存的登录信息

# Help text for the Saved logins card (the information button).
# SAFETY: needs a human review before it ships
settings-users-keyring-about = 程序为你保存的密码（例如邮件或 Wi-Fi 的密码）存放在一个随你登录而打开的密钥环中。每个人在登录时都会自动迁移到 Trindle 的密钥环。如果你的未能迁移，迁移会复制每一条保存的登录信息，并会询问你的登录无法打开的任何密钥环的密码，GNOME 的文件则保持原样。

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-kept-trindle = 由 Trindle 的密钥环保管

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
# SAFETY: needs a human review before it ships
settings-users-kept-gnome = 由 GNOME 的密钥环保管

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = 随你的登录打开。

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = 你的登录未能把它们迁移过去。

# Tip of the button that moves saved logins to Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-move-keyring = 迁移到 Trindle 的密钥环

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = 你的密码

# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = 继续

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = { $name } 的密码

# Red words when the move of saved logins was started without a password.
# SAFETY: needs a human review before it ships
settings-users-your-password-please = 请输入你的密码。

# Said beside the Saved logins line while they are copied.
# SAFETY: needs a human review before it ships
settings-users-copying = 正在复制你保存的登录信息...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = 已复制 { $n } 条

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
# SAFETY: needs a human review before it ships
settings-users-nothing-moved =
    { $n ->
       *[other] 尚未迁移任何内容：{ $names } 需要各自的密码。
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = 中途停止了。

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = 正在查看账户…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = 用户

# Under the Administrator line of the placeholder card.
settings-users-looking = 查看中…

