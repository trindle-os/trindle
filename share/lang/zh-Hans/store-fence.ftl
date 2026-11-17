
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = 这不是有效的程序名称。

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = 没有保存成功。

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = 请选择一个存在的文件夹。

# Red words when the whole home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-whole-home = 整个主目录保持私有。

# Red words when a folder that holds the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-holds-home = 这个文件夹包含你的主目录，所以保持私有。

# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = 只能共享你主目录内的文件夹。

# Red words when a hidden folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-hidden = 隐藏文件夹始终保持私有。

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = 这个文件夹存放密码或邮件，所以保持私有。

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = 该程序没有可使用的设置文件夹。

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = 请输入网站名称，例如 api.example.com，或用 *.example.com 表示该名称下的所有网站。

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = 请输入电脑名称，例如 nas.local。

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = 请将每一项写成 NAME=value。

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = 请将每一项写成 NAME=value，名称用大写字母。

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = 一个 AI 程序正在请求使用此文件夹。它目前看不到该文件夹。

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = 一个 AI 程序正在请求以管理员身份运行此内容。它会在围栏之外运行，并可能看到你能看到的一切。

# Small heading of the window that asks about an AI program.
store-fence-ai-program = AI 程序

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
# SAFETY: needs a human review before it ships
store-fence-read-only = 只允许它读取此文件夹

# Note under the question about a folder.
store-fence-sees-next = 该程序下次启动时才能看到此文件夹。

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = 拒绝

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = 仅在本次会话中共享

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = 始终共享

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = 不允许

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = 允许

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = 这条命令太长，无法完整显示，所以没有运行。

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = 另一个请求正在等你回答。

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = 你没有允许这个操作。

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } 在围栏内不可用。

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = 它无法在围栏之外启动。

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = 请给出文件夹的完整路径。

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = 你没有共享它。

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = 已共享。该程序下次启动时就能看到它。

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = 请写明要连接的电脑名称。

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = 围栏内的 AI 程序无法用 ssh 连接那台电脑。

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = 无法生成 AI 程序使用的密钥：{ $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = 请写明要运行的命令。

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } 在这里不可用。

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } 在围栏内不可用。请让它在围栏之外运行。

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }：围栏的请求通道不存在。

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }：请求没有发出。

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock 拒绝了：{ $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = 围栏没有启动：{ $why }。

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = 围栏没有启动：{ $path }：{ $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = AI 程序在这里保留自己的登录信息，与你的分开。需要时请在程序内登录。

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = 你的文件夹对 AI 程序是私有的。要让它在某个文件夹里工作，请在“设置”的“AI 程序”中共享该文件夹。

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = 围栏没有启动：无法创建系统调用过滤器。

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = 围栏没有启动：无法设置网络限制。

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = 围栏没有启动：bubblewrap 无法运行：{ $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }：在围栏之外也找不到
