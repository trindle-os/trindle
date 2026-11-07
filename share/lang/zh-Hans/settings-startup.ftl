
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = 登录时启动

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = 自启动 启动 开机 登录 自动 启动项 延迟 autostart startup login boot delay

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = 你登录时会自行打开的应用。延迟可以让应用先等待网络或另一个应用。

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = 登录时没有任何内容会自行启动。

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = 登录时 { $name } 会自行启动。

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
       *[other] 登录时有 { $n } 个应用会自行启动。
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
       *[other] 另外保留了 { $n } 个，已关闭。
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = 立即

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = 1 分钟后

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = 2 分钟后

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
       *[other] { $n } 秒后
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name } 的延迟

# Name of the switch on an app's line that keeps it from starting at login without removing it.
# SAFETY: needs a human review before it ships
settings-startup-switch = 登录时启动它

# Name of the bin button on an app's line that stops it starting at login.
# SAFETY: needs a human review before it ships
settings-startup-stop = 不再在登录时启动它

# Red words under an app's line when it could not be taken off the list.
# SAFETY: needs a human review before it ships
settings-startup-not-removed = 无法将它删除，因此它仍会在登录时启动。

# Red words under the add card when something could not be added.
settings-startup-not-added = 无法添加它。

# Placeholder of the search box above the list of installed apps.
settings-startup-find = 查找应用

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = 登录时启动这个

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = 无法添加：该应用已不再安装。

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = 例如 syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = 从磁盘中选择程序

# Heading of the line for adding a command.
settings-startup-command = 命令

# Small words under the command line.
settings-startup-command-about = 与在终端中输入的相同，或磁盘上的某个程序。

# Red words under the command line when nothing was typed.
settings-startup-command-empty = 请输入命令，或选择一个程序。

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = 选择登录时要启动的程序

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = 没有任何内容会自行启动

# Small words under that line.
settings-startup-none-about = 请在旁边添加一个应用。

# Title of the card listing what starts at login.
settings-startup-list-title = 登录时启动

# Words behind the i of the card listing what starts at login.
settings-startup-about = 每个应用都可以在你登录后等一会儿，让网络或另一个应用先准备好。它的开关只是把它留在这里而不启动它；垃圾桶则会让它完全不再在登录时启动。

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } 运行：{ $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = 从已安装的应用中选择

# Name of the button that opens the box for typing a command.
settings-startup-open-command = 输入命令，或选择程序

# Heading of the line for adding an installed app.
settings-startup-app-heading = 已安装的应用

# Small words under that line.
settings-startup-app-about = 从应用列表中选择一个。

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = 命令或程序

# Small words under that line.
settings-startup-cmd-about = 手动输入，或从磁盘中选择。

# Words behind the i of the Add an app card.
settings-startup-add-about = 从已安装的应用中选择，或给出要运行的命令（与在终端中输入的相同），或从磁盘中选择一个程序。它会在下次登录时立即启动；加入列表后，可以为它选择等待时间。

# Title of the card for adding an app.
settings-startup-add-title = 添加应用

