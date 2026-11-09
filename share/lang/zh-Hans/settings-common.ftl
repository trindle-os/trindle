
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = 已保存

# Said beside a button that worked, when nothing more is known.
settings-common-done = 完成

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = 未完成

# Tip of the information button at the top of every Settings page.
settings-common-about-page = 关于此页面

# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = 关于{ $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } 已存在

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
       *[other] 对其余的也这样处理（还有 { $n } 个）
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = 对其余的也这样处理

# Tip of the button that keeps the one already there.
# SAFETY: needs a human review before it ships
settings-common-skip = 跳过：保留已有的

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = 替换

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = 用新带入的替换已有的

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = 新带入的

# Heading of the line that shows the one already there.
settings-common-already-line = 已有的

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = 要替换已有的吗？

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
       *[other] 将在 { $left } 秒后还原。
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
       *[other] 将在 { $left } 秒后还原。{ $more }
    }

# Button at the foot of the window that undoes the change on trial.
# SAFETY: needs a human review before it ships
settings-common-go-back-now = 立即还原

# Button at the foot of the window that keeps the change on trial.
# SAFETY: needs a human review before it ships
settings-common-keep = 保留

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } 不可用

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = 请用以下命令安装：sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = 无法打开网页：无法启动任何网络浏览器。

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = 无法打开 { $name }。

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = 无法保存。

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = 未完成：无法连接系统的消息总线。请重启电脑，然后重试。

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = 未完成：无法连接本次登录的消息总线。请注销后重新登录，然后重试。

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = 无法打开选择窗口。请注销后重新登录，然后重试。

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = 磁盘已满。

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = 磁盘目前无法写入。

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = 负责此操作的系统部分没有运行。

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = 响应时间过长。请重试。

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = 没有输入密码。

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = 此账户无权执行该操作。

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = 这台电脑缺少所需的东西。

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not }{ $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest }和{ $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { "、" }

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = 点击屏幕边缘，圆环就会打开。

# Word drawn in the middle of that picture.
settings-common-ring-name = 圆环

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = 无法连接笔记本电脑的固件：{ $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = 笔记本电脑的固件不接受充电上限。

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = 这台电脑不允许提前停止充电。

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = 无法设置 { $path }：{ $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = 在虚拟机中试用 Trindle

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle 正在 { $machine } 中运行。它在那里运行良好，但虚拟机永远不如真实电脑那样快速流畅，所以请在真实硬件上评判它的速度。

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle 正在没有 3D 图形的 { $machine } 中运行，因此处理器必须绘制通常由图形芯片完成的一切。窗口、视频和网页会比在真实电脑上慢，Trindle 也会省去动画来帮忙。如果运行它的电脑支持，在虚拟机的显示设置中开启 3D 加速会有很大改善。安装在真实电脑上则不存在这些问题。

